use entity::{teacher::Model as TeacherModel, user::Model as UserModel};

use crate::models::Course;

/// Represents "Teacher" model
#[derive(Debug, Clone)]
pub struct Teacher {
    /// Database model of the user data
    pub user_model: UserModel,

    /// Database model of the teacher
    pub teacher_model: TeacherModel,

    /// Courses that are instructed by the teacher
    pub instructed_courses: Option<Vec<i32>>,
}

impl Teacher {
    /// Check if teacher is instructing a course
    ///
    /// # Arguments
    ///
    /// * 'course' - Target course
    ///
    /// # Returns
    ///
    /// `true` - Course is instructed by the teacher
    /// `false` - Course is NOT instructed by the teacher
    pub fn is_instructing(&self, course: &Course) -> bool {
        match &self.instructed_courses {
            Some(courses) => courses.contains(&course.model.course_id),
            None => false,
        }
    }
}
