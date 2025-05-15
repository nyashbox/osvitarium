use entity::{student::Model as StudentModel, user::Model as UserModel};

use super::Course;

/// Represents student model
#[derive(Debug, Clone)]
pub struct Student {
    /// Database model of the user data
    pub user_model: UserModel,

    /// Database model of the student
    pub student_model: StudentModel,

    /// Courses, that are attended by the student
    pub attended_courses: Option<Vec<i32>>,
}

impl Student {
    /// Check if student is attending course
    ///
    /// # Arguments
    ///
    /// * 'course' - Target course
    ///
    /// # Returns
    ///
    /// `true` - Student is attending a course
    /// `false` - Student doesn't attend course
    pub fn is_attending(&self, course: &Course) -> bool {
        match &self.attended_courses {
            Some(courses) => courses.contains(&course.model.course_id),
            None => false,
        }
    }
}
