use crate::{app::error::AppStatus as Status, repositories};

use jsonwebtoken::{EncodingKey, Header, encode};

use repositories::user::User;

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
pub trait UserService {
    /// Authenticate user
    ///
    /// # Arguments
    ///
    /// * 'password' - plain-text password
    ///
    /// # Returns
    ///
    /// On success: authenticated user's model
    /// On failure: database error/empty option
    fn authenticate(&self, password: &str) -> Result<(), Status>;

    /// Convert user into JWT
    ///
    /// # Arguments
    ///
    /// * 'secret' - JWT secret
    /// * 'ttl' - Session TTL
    ///
    /// # Returns
    ///
    /// On success: encoded JWT token
    /// On failure: application error
    fn into_jwt(&self, secret: &str, ttl: u64) -> Result<String, Status>;
}

pub mod utils {
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

impl UserService for User {
    fn authenticate(&self, password: &str) -> Result<(), Status> {
        let user_model = match self {
            User::Student(model, _) => model,
            User::Teacher(model, _) => model,
            User::Principal(model, _) => model,
        };

        if utils::verify_password(password, &user_model.password)? {
            Ok(())
        } else {
            Err(Status::Unauthenticated(None))
        }
    }

    fn into_jwt(&self, secret: &str, ttl: u64) -> Result<String, Status> {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                error!("Error while getting system time: {e}");

                Status::Internal(None)
            })?
            .as_secs();

        let claims = match self {
            User::Student(user, student) => JWTClaims {
                exp: iat + ttl,
                iat,
                aux_sub: student.student_id,
                sub: user.user_id,
                role: "student".into(),
            },
            User::Teacher(user, teacher) => JWTClaims {
                exp: iat + ttl,
                iat,
                aux_sub: teacher.teacher_id,
                sub: user.user_id,
                role: "teacher".into(),
            },
            User::Principal(user, principal) => JWTClaims {
                exp: iat + ttl,
                iat,
                aux_sub: principal.principal_id,
                sub: user.user_id,
                role: "principal".into(),
            },
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| {
            error!("Error while encoding JWT token: {e}");

            Status::Internal(None)
        })?)
    }
}

#[cfg(test)]
mod tests {
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
