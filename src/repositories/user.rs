use crate::{
    app::status::AppStatus as Status,
    models::{Principal, Student, Teacher},
    services::utils,
};

use entity::{
    principal::{ActiveModel as ActivePrincipalModel, Entity as PrincipalEntity},
    sea_orm_active_enums::UserRole,
    student::{ActiveModel as ActiveStudentModel, Entity as StudentEntity},
    teacher::{ActiveModel as ActiveTeacherModel, Entity as TeacherEntity},
    user::{self, ActiveModel as ActiveUserModel, Entity as UserEntity},
};

use crate::models::user::User;

use log::error;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect, Set,
};

#[mockall::automock]
pub trait UserRepository {
    /// Create new user in the storage
    ///
    /// # Arguments
    ///
    /// * 'username' - Username
    /// * 'password' - Plain-text password
    /// * 'user' - User role
    ///
    /// # Returns
    ///
    /// On success: User
    /// On failure: Application status
    fn create(
        &self,
        username: &str,
        password: &str,
        role: UserRole,
    ) -> impl std::future::Future<Output = Result<User, Status>> + Send;

    /// Find user by ID
    ///
    /// # Arguments
    ///
    /// * 'id' - user identifier (ID)
    ///
    /// # Returns
    ///
    /// On success: User
    /// On failure: Application status
    fn find_by_id(&self, id: i32)
    -> impl std::future::Future<Output = Result<User, Status>> + Send;

    /// Find user by username
    ///
    /// # Arguments
    ///
    /// * 'username' - user's username
    ///
    /// # Returns
    ///
    /// On success: User
    /// On failure: Application status
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl std::future::Future<Output = Result<User, Status>> + Send;

    /// Delete user profile
    ///
    /// # Arguments
    ///
    /// * 'user' - User that should be deleted
    ///
    /// # Returns
    ///
    /// On success: Nothing
    /// On failure: Application status
    fn delete_user(
        &self,
        user: &User,
    ) -> impl std::future::Future<Output = Result<(), Status>> + Send;
}

impl UserRepository for sea_orm::DatabaseConnection {
    async fn create(&self, username: &str, password: &str, role: UserRole) -> Result<User, Status> {
        let password = utils::hash_password(password)?;

        // Create base user
        let user_model = ActiveUserModel {
            username: Set(username.to_owned()),
            fullname: Set("".to_owned()),
            password: Set(password),
            description: Set("".to_owned()),
            metadata: Set("{}".into()),
            role: Set(role.clone()),

            ..Default::default()
        }
        .insert(self)
        .await
        .map_err(|e| {
            error!("Failed to create 'user' record: {e}");

            Status::Internal(None)
        })?;

        let user_id = user_model.user_id;
        Ok(match role {
            UserRole::Student => {
                let student_model = ActiveStudentModel {
                    user_id: Set(user_id),

                    ..Default::default()
                }
                .insert(self)
                .await
                .map_err(|e| {
                    error!("Failed to create 'student' profile: {e}");

                    Status::Internal(None)
                })?;

                let student = Student {
                    user_model,
                    student_model,
                    attended_courses: None,
                };

                User::Student(student)
            }
            UserRole::Teacher => {
                let teacher_model = ActiveTeacherModel {
                    user_id: Set(user_id),

                    ..Default::default()
                }
                .insert(self)
                .await
                .map_err(|e| {
                    error!("Failed to create 'teacher' profile: {e}");

                    Status::Internal(None)
                })?;

                let teacher = Teacher {
                    user_model,
                    teacher_model,
                    instructed_courses: None,
                };

                User::Teacher(teacher)
            }
            UserRole::Principal => {
                let principal_model = ActivePrincipalModel {
                    user_id: Set(user_id),

                    ..Default::default()
                }
                .insert(self)
                .await
                .map_err(|e| {
                    error!("Failed to create 'principal' profile: {e}");

                    Status::Internal(None)
                })?;

                let principal = Principal {
                    user_model,
                    principal_model,
                };

                User::Principal(principal)
            }
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<User, Status> {
        let user = UserEntity::find_by_id(id)
            .one(self)
            .await
            .map_err(|e| {
                error!("Failed to find user by ID: {e}");

                Status::Internal(None)
            })?
            .ok_or(Status::NotFound(Some(
                "User with id '{id}' was not found!".to_string(),
            )))?;

        match &user.role {
            UserRole::Student => {
                let model = user
                    .find_related(StudentEntity)
                    .one(self)
                    .await
                    .map_err(|e| {
                        error!("Failed to find by ID: {e}");

                        Status::Internal(None)
                    })?;

                if let Some(student_model) = model {
                    let attended_courses: Vec<i32> = entity::course_student::Entity::find()
                        .select_only()
                        .column(entity::course_student::Column::CourseId)
                        .filter(
                            entity::course_student::Column::StudentId.eq(student_model.student_id),
                        )
                        .into_values::<i32, entity::course_student::Column>()
                        .all(self)
                        .await
                        .map_err(|e| {
                            error!("Failed to fetch courses attended by the student: {e}");

                            Status::Internal(None)
                        })?;

                    return Ok(User::Student(Student {
                        user_model: user,
                        student_model,
                        attended_courses: (!attended_courses.is_empty())
                            .then_some(attended_courses),
                    }));
                } else {
                    return Err(Status::Internal(None));
                }
            }
            UserRole::Teacher => {
                let model = user
                    .find_related(TeacherEntity)
                    .one(self)
                    .await
                    .map_err(|e| {
                        error!("Failed to find by ID: {e}");

                        Status::Internal(None)
                    })?;

                if let Some(teacher_model) = model {
                    let instructed_courses = entity::course_instructor::Entity::find()
                        .select_only()
                        .column(entity::course_instructor::Column::CourseId)
                        .filter(
                            entity::course_instructor::Column::InstructorId
                                .eq(teacher_model.teacher_id),
                        )
                        .into_values::<i32, entity::course_student::Column>()
                        .all(self)
                        .await
                        .map_err(|e| {
                            error!("Failed to fetch courses instructed by the teacher: {e}");

                            Status::Internal(None)
                        })?;

                    return Ok(User::Teacher(Teacher {
                        user_model: user,
                        teacher_model,
                        instructed_courses: (!instructed_courses.is_empty())
                            .then_some(instructed_courses),
                    }));
                } else {
                    return Err(Status::Internal(None));
                }
            }
            UserRole::Principal => {
                let model = user
                    .find_related(PrincipalEntity)
                    .one(self)
                    .await
                    .map_err(|e| {
                        error!("Failed to find by ID: {e}");

                        Status::Internal(None)
                    })?;

                if let Some(principal_model) = model {
                    return Ok(User::Principal(Principal {
                        user_model: user,
                        principal_model,
                    }));
                } else {
                    return Err(Status::Internal(None));
                }
            }
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<User, Status> {
        let user = UserEntity::find()
            .filter(user::Column::Username.eq(username))
            .one(self)
            .await
            .map_err(|e| {
                error!("Failed to find user by username: {e}");

                Status::Internal(None)
            })?;

        match user {
            Some(model) => Ok(self.find_by_id(model.user_id).await?),
            None => Err(Status::NotFound(None)),
        }
    }

    async fn delete_user(&self, user: &User) -> Result<(), Status> {
        let delete_result = UserEntity::delete_by_id(user.user_id())
            .exec(self)
            .await
            .map_err(|e| {
                error!("Failed to delete user: {e}");
                Status::Internal(None)
            })?;

        if delete_result.rows_affected == 0 {
            Err(Status::NotFound(Some(
                "User with specified ID was not found!".into(),
            )))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    mod create {
        use sea_orm::{DbErr, MockDatabase};

        use entity::{
            sea_orm_active_enums::UserRole, student::Model as StudentModel,
            user::Model as UserModel,
        };

        use crate::repositories::user::UserRepository;

        #[tokio::test]
        pub async fn success() {
            let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
            let db = db
                .append_query_results([
                    // First request - returns base user model on success
                    vec![UserModel {
                        user_id: 1,
                        username: "".into(),
                        fullname: "".into(),
                        password: "".into(),
                        description: "".into(),
                        metadata: "{}".into(),
                        role: UserRole::Student,
                    }],
                ])
                .append_query_results([
                    // Second request - returns student model on success
                    vec![StudentModel {
                        student_id: 1,
                        user_id: 1,
                    }],
                ])
                .into_connection();

            let res = UserRepository::create(&db, "username", "password", UserRole::Student).await;

            assert!(res.is_ok());
        }

        #[tokio::test]
        pub async fn db_err() {
            let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
            let db = db
                .append_query_errors(vec![DbErr::RecordNotInserted])
                .into_connection();

            let res = UserRepository::create(&db, "username", "password", UserRole::Student).await;

            assert!(res.is_err());
        }
    }
}
