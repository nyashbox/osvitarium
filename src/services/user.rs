use crate::app::error::AppStatus as Status;

use jsonwebtoken::{EncodingKey, Header, encode};

use entity::{
    principal::{Entity as PrincipalEntity, Model as PrincipalModel},
    student::{Entity as StudentEntity, Model as StudentModel},
    teacher::{Entity as TeacherEntity, Model as TeacherModel},
    user,
    user::{ActiveModel as ActiveUserModel, Model as UserModel},
};

use std::time::{SystemTime, UNIX_EPOCH};

use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use serde::{Deserialize, Serialize};

use log::error;

/// Represents JWT token claims
#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    /// Auxiliary Identifier
    pub aux_sub: i32,
    /// User Identifier (ID)
    pub sub: i32,
    /// Issued at
    pub iat: u64,
    /// Expires at
    pub exp: u64,
    /// User role
    pub role: String,
}

#[mockall::automock]
#[allow(async_fn_in_trait)]
pub trait UserService {
    /// Authenticate user with 'username:password' pair
    ///
    /// # Arguments
    ///
    /// * 'username' - username
    /// * 'password' - plain-text password
    ///
    /// # Returns
    ///
    /// On success: authenticated user's model
    /// On failure: database error/empty option
    async fn authenticate(&self, username: &str, password: &str) -> Result<UserModel, Status>;

    /// Create new user
    ///
    /// # Arguments
    ///
    /// * 'username' - username
    /// * 'password' - plain-text password
    ///
    /// # Returns
    ///
    /// On success: user model
    /// On failure: database error
    async fn create(&self, username: &str, password: &str) -> Result<UserModel, Status>;

    /// Check if user is a principal
    ///
    /// # Arguments
    ///
    /// * 'user' - user model
    ///
    /// # Returns
    ///
    /// On success: principal model
    /// On failure: database error
    async fn is_principal(&self, user: &UserModel) -> Result<Option<PrincipalModel>, Status>;

    /// Check if user is a teacher
    ///
    /// # Arguments
    ///
    /// * 'user' - user model
    ///
    /// # Returns
    ///
    /// On success: teacher model
    /// On failure: database error
    async fn is_teacher(&self, user: &UserModel) -> Result<Option<TeacherModel>, Status>;

    /// Check if user is a student
    ///
    /// # Arguments
    ///
    /// * 'user' - user model
    ///
    /// # Returns
    ///
    /// On success: student model
    /// On failure: database error
    async fn is_student(&self, user: &UserModel) -> Result<Option<StudentModel>, Status>;

    /// Convert user model into JWT
    ///
    /// # Arguments
    ///
    /// * 'user' - User model
    /// * 'secret' - JWT secret
    /// * 'ttl' - Session TTL
    ///
    /// # Returns
    ///
    /// On success: encoded JWT token
    /// On failure: application error
    async fn into_jwt(&self, user: &UserModel, secret: &str, ttl: u64) -> Result<String, Status>;

    /// Retrieve user from the database
    ///
    /// # Arguments
    ///
    /// * 'user_id' - user ID
    ///
    /// # Returns
    ///
    /// On success: user model
    /// On failure: application status
    async fn find_one(&self, user_id: i32) -> Result<Option<UserModel>, Status>;
}

mod utils {
    use log::error;

    use argon2::{
        Argon2,
        password_hash::{
            PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
        },
    };

    use crate::app::error::AppStatus as Status;

    /// Hash password
    ///
    /// # Arguments
    ///
    /// * 'password' - plain-text password
    ///
    /// # Returns
    ///
    /// Password hash
    pub fn hash_password(password: &str) -> Result<String, Status> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                error!("Failed to create password hash: {e}");

                Status::Internal(None)
            })?;

        Ok(password_hash.to_string())
    }

    /// Verify password against password hash
    ///
    /// # Arguments
    ///
    /// * 'password' - plain-text password
    /// * 'hash' - password hash
    ///
    /// # Return
    ///
    /// `true` if password matches received hash, 'false' if not
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, Status> {
        let hash = PasswordHash::new(hash).map_err(|e| {
            error!("Error while parsing password hash: {e}");

            Status::Internal(None)
        })?;
        let argon2 = Argon2::default();

        Ok(argon2.verify_password(password.as_bytes(), &hash).is_ok())
    }
}

impl UserService for sea_orm::DatabaseConnection {
    async fn find_one(&self, user_id: i32) -> Result<Option<UserModel>, Status> {
        let user = user::Entity::find_by_id(user_id)
            .one(self)
            .await
            .map_err(|e| {
                error!("Failed to find user: {e}");

                Status::Internal(None)
            })?;

        Ok(user)
    }

    async fn authenticate(&self, username: &str, password: &str) -> Result<UserModel, Status> {
        let user = user::Entity::find()
            .filter(user::Column::Username.contains(username))
            .one(self)
            .await
            .map_err(|e| {
                error!("Error while authenticating user: {e}");

                Status::Internal(None)
            })?;

        match user {
            Some(model) => {
                if utils::verify_password(password, &model.password)? {
                    Ok(model)
                } else {
                    Err(Status::Unauthenticated(Some(
                        "Invalid username or password!".to_string(),
                    )))
                }
            }
            None => Err(Status::Unauthenticated(Some(
                "Invalid username or password!".to_string(),
            ))),
        }
    }

    async fn create(&self, username: &str, password: &str) -> Result<UserModel, Status> {
        let hashed_password = utils::hash_password(&password)?;

        let user = ActiveUserModel {
            username: ActiveValue::Set(username.into()),
            password: ActiveValue::Set(hashed_password),
            ..Default::default()
        };

        user.insert(self).await.map_err(|e| {
            error!("Error while creating new user: {e}");

            Status::Internal(None)
        })
    }

    async fn is_principal(&self, user: &UserModel) -> Result<Option<PrincipalModel>, Status> {
        let principal = user
            .find_related(PrincipalEntity)
            .one(self)
            .await
            .map_err(|e| {
                error!("Error while verifying if user is a principal: {e}");

                Status::Internal(None)
            })?;

        Ok(principal)
    }

    async fn is_student(&self, user: &UserModel) -> Result<Option<StudentModel>, Status> {
        let student = user
            .find_related(StudentEntity)
            .one(self)
            .await
            .map_err(|e| {
                error!("Error while verifying if user is a student: {e}");

                Status::Internal(None)
            })?;

        Ok(student)
    }

    async fn is_teacher(&self, user: &UserModel) -> Result<Option<TeacherModel>, Status> {
        let teacher = user
            .find_related(TeacherEntity)
            .one(self)
            .await
            .map_err(|e| {
                error!("Error while verifying if user is a teacher: {e}");

                Status::Internal(None)
            })?;

        Ok(teacher)
    }

    async fn into_jwt(&self, user: &UserModel, secret: &str, ttl: u64) -> Result<String, Status> {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                error!("Error while getting system time: {e}");

                Status::Internal(None)
            })?
            .as_secs();

        let aux_sub: i32;
        let role: String;

        if let Some(model) = self.is_student(user).await? {
            aux_sub = model.student_id;
            role = "student".to_string();
        } else if let Some(model) = self.is_teacher(user).await? {
            aux_sub = model.teacher_id;
            role = "teacher".to_string();
        } else if let Some(model) = self.is_principal(user).await? {
            aux_sub = model.principal_id;
            role = "principal".to_string();
        } else {
            return Err(Status::Internal(None));
        }

        let claims = JWTClaims {
            exp: iat + ttl,
            iat,
            aux_sub,
            sub: user.user_id,
            role,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| {
            error!("Error while encoding JWT token: {e}");

            Status::Internal(None)
        })?;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    mod authenticate {
        use crate::services::user::{UserModel, UserService};

        use sea_orm::{DatabaseBackend, MockDatabase};

        #[tokio::test]
        pub async fn success() {
            let db = MockDatabase::new(DatabaseBackend::Postgres);
            let db = db.append_query_results([vec![UserModel {
                user_id: 1,
                username: "username".to_string(),
                fullname: "John Doe".to_string(),
                password: "$argon2id$v=19$m=16,t=2,p=1$cGFzc3dvcmQ$8vDS3rsezOjrur01dF12EA"
                    .to_string(),
                description: "".to_string(),
                metadata: "{}".into(),
            }]]);

            let response = UserService::authenticate(
                &db.into_connection(),
                &"username".to_string(),
                &"password".to_string(),
            )
            .await;

            assert!(
                response.is_ok(),
                "If CORRECT 'username:password' pair is specified, user model MUST be returned"
            );
        }

        #[tokio::test]
        pub async fn bad_password() {
            let db = MockDatabase::new(DatabaseBackend::Postgres);
            let db = db.append_query_results([vec![UserModel {
                user_id: 1,
                username: "username".to_string(),
                fullname: "John Doe".to_string(),
                password: "$argon2id$v=19$m=16,t=2,p=1$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
                    .to_string(),
                description: "".to_string(),
                metadata: "{}".into(),
            }]]);

            let response = UserService::authenticate(
                &db.into_connection(),
                &"username".to_string(),
                &"password".to_string(),
            )
            .await;

            assert!(
                response.is_err(),
                "If INCORRECT 'username:password' pair is specified, error MUST be returned!"
            );
        }
    }

    mod into_jwt {
        use crate::services::user::{JWTClaims, UserService};

        use entity::{
            principal::Model as PrincipalModel, student::Model as StudentModel,
            teacher::Model as TeacherModel, user::Model as UserModel,
        };

        use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase};

        #[tokio::test]
        pub async fn success() {
            let user = UserModel {
                user_id: 1,
                username: "johndoe".into(),
                fullname: "John Doe".into(),
                password: "password".into(),
                description: " ".into(),
                metadata: "{}".into(),
            };

            assert_eq!(
                {
                    let student = StudentModel {
                        student_id: 1,
                        user_id: 1,
                    };

                    let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
                    let db = db.append_query_results([
                        // First query ('student' table) - returns STUDENT MODEL
                        vec![student],
                    ]);

                    let token = UserService::into_jwt(&db.into_connection(), &user, "secret", 100)
                        .await
                        .unwrap();

                    decode::<JWTClaims>(
                        token.as_str(),
                        &DecodingKey::from_secret("secret".as_bytes()),
                        &Validation::new(Algorithm::HS256),
                    )
                    .unwrap()
                    .claims
                    .role
                },
                "student",
                "When user is a STUDENT, JWT token with role 'student' MUST be returned"
            );

            assert_eq!(
                {
                    let teacher = TeacherModel {
                        teacher_id: 1,
                        user_id: 1,
                    };

                    let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
                    let db = db.append_query_results([
                        // First query ('student' table) - returns NOTHING
                        vec![],
                        // Second query ('teacher' table) - returns TEACHER MODEL
                        vec![teacher],
                    ]);

                    let token = UserService::into_jwt(&db.into_connection(), &user, "secret", 100)
                        .await
                        .unwrap();

                    decode::<JWTClaims>(
                        token.as_str(),
                        &DecodingKey::from_secret("secret".as_bytes()),
                        &Validation::new(Algorithm::HS256),
                    )
                    .unwrap()
                    .claims
                    .role
                },
                "teacher",
                "When user is a TEACHER, JWT token with role 'teacher' MUST be returned"
            );

            assert_eq!(
                {
                    let principal = PrincipalModel {
                        principal_id: 1,
                        user_id: 1,
                    };

                    let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
                    let db = db.append_query_results([
                        // First query (student table) returns NOTHING
                        vec![],
                        // Second query (teacher table) returns NOTHING
                        vec![],
                        // Third query (principal table) return PRINCIPAL MODEL
                        vec![principal],
                    ]);

                    let token = UserService::into_jwt(&db.into_connection(), &user, "secret", 100)
                        .await
                        .unwrap();

                    decode::<JWTClaims>(
                        token.as_str(),
                        &DecodingKey::from_secret("secret".as_bytes()),
                        &Validation::new(Algorithm::HS256),
                    )
                    .unwrap()
                    .claims
                    .role
                },
                "principal",
                "When user is a PRINCIPAL, JWT token with role 'principal' MUST be returned"
            );
        }

        #[tokio::test]
        pub async fn db_error() {
            let user = UserModel {
                user_id: 1,
                username: "johndoe".into(),
                fullname: "John Doe".into(),
                password: "password".into(),
                description: " ".into(),
                metadata: "{}".into(),
            };

            let db = MockDatabase::new(DatabaseBackend::Postgres);
            let db = db.append_query_errors([DbErr::RecordNotFound("".into())]);

            let token = UserService::into_jwt(&db.into_connection(), &user, "secret", 100).await;

            assert!(
                token.is_err(),
                "When database returns ERROR, received error MUST be propagated"
            )
        }

        #[tokio::test]
        pub async fn corrupted_data() {
            let user = UserModel {
                user_id: 1,
                username: "johndoe".into(),
                fullname: "John Doe".into(),
                password: "password".into(),
                description: " ".into(),
                metadata: "{}".into(),
            };

            let db = MockDatabase::new(DatabaseBackend::Postgres);
            let db = db.append_query_results([
                Vec::<UserModel>::new(),
                Vec::<UserModel>::new(),
                Vec::<UserModel>::new(),
            ]);

            let token = UserService::into_jwt(&db.into_connection(), &user, "secret", 100).await;

            assert!(
                token.is_err(),
                "When user model is CORRUPTED (e.g. user exists without specialization), error MUST be returned"
            )
        }
    }

    mod create {
        use crate::services::user::{UserService, utils::*};
        use sea_orm::{DatabaseBackend, MockDatabase};

        use entity::user::Model as UserModel;

        #[tokio::test]
        pub async fn success() {
            let username = "johndoe".to_string();
            let fullname = "John Doe".to_string();
            let password = "password".to_string();
            let password_hash = hash_password(&password).unwrap();
            let description = "Account Description".to_string();

            let db = MockDatabase::new(DatabaseBackend::Postgres);
            let db = db.append_query_results([vec![UserModel {
                username: username.clone(),
                fullname: fullname.clone(),
                password: password_hash.clone(),
                description: description.clone(),
                user_id: 1,
                metadata: "{}".into(),
            }]]);

            let user =
                UserService::create(&db.into_connection(), username.as_str(), password.as_str())
                    .await;

            assert!(
                user.is_ok(),
                "When 'create' request is successful, user model MUST be returned from the database"
            );

            assert!(
                verify_password(&"password".to_string(), &user.unwrap().password).unwrap(),
                "Model in the database MUST store argon2 hash of the password."
            );
        }
    }

    mod utils {
        mod verify_password {
            use crate::services::user::utils::*;

            #[test]
            pub fn success() {
                let test_password = "password".to_string();
                let password_hash =
                    "$argon2id$v=19$m=16,t=2,p=1$cGFzc3dvcmQ$8vDS3rsezOjrur01dF12EA".to_string();

                assert!(
                    verify_password(&test_password, &password_hash).unwrap(),
                    "When password hash and hash of the plain-text password MATCH, `true` MUST be returned!"
                );
            }

            #[test]
            pub fn mismatch() {
                let test_password = "password".to_string();
                let password_hash =
                    "$argon2id$v=19$m=16,t=2,p=1$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string();

                assert!(
                    !verify_password(&test_password, &password_hash).unwrap(),
                    "When password hash and hash of the plain-text password DO NOT MATCH, `false` MUST be returned!"
                );
            }
        }

        mod hash_password {
            use crate::services::user::utils::*;

            #[test]
            pub fn success() {
                let test_password = String::from("password");
                let password_hash = hash_password(&test_password).unwrap();

                assert!(
                    verify_password(&test_password, &password_hash).unwrap(),
                    "Password hash MUST pass verification"
                );
            }
        }
    }
}
