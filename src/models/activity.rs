use entity::{activity::Model as ActivityModel, sea_orm_active_enums::ActivityType};
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Activity model
#[derive(Debug)]
pub struct Activity {
    /// Database model of the activity
    pub activity_model: ActivityModel,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(title = "Activity", description = "Activity object")]
pub struct ActivityDTO {
    pub activity_id: i32,
    pub title: String,
    pub description: Option<String>,
    #[schema(value_type = String)]
    pub r#type: ActivityType,
    pub course_id: i32,
    #[schema(value_type = String)]
    pub published_at: DateTime,
    #[schema(value_type = Option<String>)]
    pub deadline: Option<DateTime>,
    pub points: Option<i32>,
    pub is_hidden: bool,
    pub author_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(title = "CreateActivity", description = "Create activity Object")]
pub struct CreateActivityDTO {
    pub title: String,
    pub description: Option<String>,
    #[schema(value_type = String)]
    pub r#type: ActivityType,
    #[schema(value_type = Option<String>)]
    pub deadline: Option<DateTime>,
    pub points: Option<i32>,
    pub is_hidden: bool,
}

impl From<Activity> for ActivityDTO {
    fn from(value: Activity) -> Self {
        let model = value.activity_model;
        // let type = model.type;
        ActivityDTO {
            activity_id: model.acitvity_id,
            title: model.title,
            description: model.description,
            r#type: model.r#type,
            course_id: model.course_id,
            published_at: model.published_at,
            deadline: model.deadline,
            points: model.points,
            is_hidden: model.is_hidden,
            author_id: model.author_id,
        }
    }
}

impl Activity {}
