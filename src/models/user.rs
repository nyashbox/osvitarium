use entity::{
    principal::Model as PrincipalModel, student::Model as StudentModel,
    teacher::Model as TeacherModel, user::Model as UserModel,
};

use serde::{Deserialize, Serialize};

/// Represents user
#[derive(Clone)]
pub enum User {
    /// Student
    Student(UserModel, StudentModel),

    /// Teacher
    Teacher(UserModel, TeacherModel),

    /// Principal
    Principal(UserModel, PrincipalModel),
}

/// JSON-serializable representation that can be safely returned from the app
#[derive(Serialize, Deserialize)]
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
            User::Student(_, role) => role.student_id,
            User::Teacher(_, role) => role.teacher_id,
            User::Principal(_, role) => role.principal_id,
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
            User::Student(model, _) => model,
            User::Teacher(model, _) => model,
            User::Principal(model, _) => model,
        }
    }

    pub fn role(&self) -> &str {
        match self {
            User::Student(_, _) => "Student",
            User::Teacher(_, _) => "Teacher",
            User::Principal(_, _) => "Principal",
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
        matches!(self, User::Student(_, _))
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
        matches!(self, User::Teacher(_, _))
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
        matches!(self, User::Principal(_, _))
    }
}
