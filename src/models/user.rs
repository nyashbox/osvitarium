use crate::models::{Principal, Student, Teacher};

use entity::user::Model as UserModel;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents user
#[derive(Clone)]
pub enum User {
    /// Student
    Student(Student),

    /// Teacher
    Teacher(Teacher),

    /// Principal
    Principal(Principal),
}

/// JSON-serializable representation that can be safely returned from the app
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserRepresentation {
    user_id: i32,
    username: String,
    fullname: String,
    description: String,
    role: String,
    role_id: i32,
}

impl Into<UserRepresentation> for User {
    fn into(self) -> UserRepresentation {
        let role = self.role().to_string();
        let role_id = self.role_id();

        let user = self.base_model();

        UserRepresentation {
            user_id: user.user_id,
            username: user.username,
            fullname: user.fullname,
            description: user.description,
            role: role,
            role_id: role_id,
        }
    }
}

impl User {
    /// Return role unique identifier (ID)
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// Role ID
    pub fn role_id(&self) -> i32 {
        match self {
            User::Student(model) => model.student_model.student_id,
            User::Teacher(model) => model.teacher_model.teacher_id,
            User::Principal(model) => model.principal_model.principal_id,
        }
    }

    /// Extract base model from the variant
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// Reference to the base user model
    pub fn base_model(self) -> UserModel {
        match self {
            User::Student(model) => model.user_model,
            User::Teacher(model) => model.user_model,
            User::Principal(model) => model.user_model,
        }
    }

    pub fn base_model_ref(&self) -> &UserModel {
        match self {
            User::Student(model) => &model.user_model,
            User::Teacher(model) => &model.user_model,
            User::Principal(model) => &model.user_model,
        }
    }

    pub fn role(&self) -> &str {
        match self {
            User::Student(_) => "Student",
            User::Teacher(_) => "Teacher",
            User::Principal(_) => "Principal",
        }
    }

    /// Check whether user is a student
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// `true` - User is a student
    /// `false` - User is NOT a student
    #[inline(always)]
    pub fn is_student(&self) -> bool {
        matches!(self, User::Student(_))
    }

    /// Check whether user is a teacher
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// `true` - User is a teacher
    /// `false` - User is NOT a teacher
    #[inline(always)]
    pub fn is_teacher(&self) -> bool {
        matches!(self, User::Teacher(_))
    }

    /// Check whether user is a principal
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// `true` - User is a principal
    /// `false` - User is NOT a principal
    #[inline(always)]
    pub fn is_principal(&self) -> bool {
        matches!(self, User::Principal(_))
    }

    /// Get username
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// Reference to the username
    #[inline(always)]
    pub fn username(&self) -> &String {
        &self.base_model_ref().username
    }

    /// Get fullname
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// Reference to the fullname
    #[inline(always)]
    pub fn fullname(&self) -> &String {
        &self.base_model_ref().fullname
    }

    /// Get password hash
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// Reference to the password hash
    #[inline(always)]
    pub fn password(&self) -> &String {
        &self.base_model_ref().password
    }

    /// Get profile description
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// Reference to the description
    #[inline(always)]
    pub fn description(&self) -> &String {
        &self.base_model_ref().description
    }

    /// Get user identifier (ID)
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// User identifier
    pub fn user_id(&self) -> i32 {
        self.base_model_ref().user_id
    }
}

#[cfg(test)]
impl User {
    pub fn mock_user(role: entity::sea_orm_active_enums::UserRole) -> User {
        use crate::services::utils;
        use entity::sea_orm_active_enums::UserRole as Role;

        let base_user_model = UserModel {
            user_id: 1,
            username: "johndoe@localhost.localdomain".into(),
            fullname: "John Doe".into(),
            password: utils::hash_password("password").unwrap(),
            description: "Description".into(),
            role: role.clone(),
            metadata: "{}".into(),
        };

        match role {
            Role::Student => User::Student(Student {
                user_model: base_user_model,
                student_model: entity::student::Model {
                    student_id: 1,
                    user_id: 1,
                },
                attended_courses: None,
            }),
            Role::Teacher => User::Teacher(Teacher {
                user_model: base_user_model,
                teacher_model: entity::teacher::Model {
                    teacher_id: 1,
                    user_id: 1,
                },
                instructed_courses: None,
            }),
            Role::Principal => User::Principal(Principal {
                user_model: base_user_model,
                principal_model: entity::principal::Model {
                    principal_id: 1,
                    user_id: 1,
                },
            }),
        }
    }
}
