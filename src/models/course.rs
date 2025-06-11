use entity::course::Model as CourseModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use sea_orm::prelude::DateTime;

/// Represents course
pub struct Course {
    /// Course model
    pub model: CourseModel,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Course", description = "Course Object")]
pub struct CourseDTO {
    /// Course Identifier (ID)
    pub course_id: i32,

    /// Title
    pub title: String,

    /// Description
    pub description: Option<String>,

    /// Is course active?
    pub is_active: bool,

    /// Course creation date
    #[schema(value_type = String)]
    pub created_at: DateTime,

    /// Is course running a meeting?
    pub is_running_meeting: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "CourseCreate", description = "Course creation Object")]
pub struct CourseCreateDTO {
    /// title
    pub title: String,

    /// Description
    pub description: Option<String>,

    /// Is course active?
    pub is_active: bool,
}

impl Into<CourseDTO> for Course {
    fn into(self) -> CourseDTO {
        CourseDTO {
            course_id: self.model.course_id,
            title: self.model.title,
            description: self.model.description,
            is_active: self.model.is_active,
            created_at: self.model.created_at,
            is_running_meeting: self.model.is_running_meeting,
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
