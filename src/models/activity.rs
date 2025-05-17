use entity::activity::Model as ActivityModel;
use sea_orm::ActiveEnum;
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
    pub r#type: String,
    pub course_id: i32,
    pub published_at: String,
    pub deadline: Option<String>,
    pub points: Option<i32>,
    pub is_hidden: bool,
    pub author_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "ActivityCreate",
    description = "DTO for creating new activity"
)]
pub struct CreateActivityDTO {
    pub title: String,
    pub description: Option<String>,
    pub r#type: String,
    pub deadline: Option<String>,
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
            r#type: model.r#type.into_value(),
            course_id: model.course_id,
            published_at: model.published_at.to_string(),
            deadline: model.deadline.map(|v| v.to_string()),
            points: model.points,
            is_hidden: model.is_hidden,
            author_id: model.author_id,
        }
    }
}

impl Activity {}
