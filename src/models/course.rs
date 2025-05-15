use entity::course::Model as CourseModel;
use serde::{Deserialize, Serialize};

/// Represents course
pub struct Course {
    /// Course model
    pub model: CourseModel,
}

#[derive(Serialize, Deserialize)]
pub struct CourseRepresentation {
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

impl Into<CourseRepresentation> for Course {
    fn into(self) -> CourseRepresentation {
        CourseRepresentation {
            course_id: self.model.course_id,
            title: self.model.title,
            description: self.model.description,
            is_active: self.model.is_active,
            created_at: self.model.created_at.to_string(),
        }
    }
}

impl Course {
    /// Check if meeting is running in the course
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// `true` - Meeting is running
    /// `false` - Meeting is NOT running
    #[inline(always)]
    pub fn is_running_meeting(&self) -> bool {
        self.model.is_running_meeting
    }
}
