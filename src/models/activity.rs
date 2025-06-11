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
    /// Activity identifier
    #[schema(example = 1)]
    pub activity_id: i32,

    /// Activity title
    #[schema(example = "Modular test #1")]
    pub title: String,

    /// Description
    #[schema(example = "Read the theoretical notes of laboratory work №...")]
    pub description: Option<String>,

    /// Activity type
    #[schema(value_type = String, example = "Material")]
    pub r#type: ActivityType,

    /// Course to which activity belongs
    #[schema(example = 1)]
    pub course_id: i32,

    /// When activity was created/published
    #[schema(value_type = String, example = "2025-05-14T13:45:30Z")]
    pub published_at: DateTime,

    /// Optional deadline for the activity
    #[schema(value_type = Option<String>, example = "2025-05-14T13:45:30Z", nullable)]
    pub deadline: Option<DateTime>,

    /// Maximum grade for the work performed
    #[schema(example = 10, nullable)]
    pub points: Option<i32>,

    /// Is this activity hidden?
    #[schema(example = false)]
    pub is_hidden: bool,

    /// Activity author
    #[schema(example = 1)]
    pub author_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(title = "CreateActivity", description = "Create activity Object")]
pub struct CreateActivityDTO {
    /// Activity title
    #[schema(example = "Modular test #1")]
    pub title: String,

    /// Description
    #[schema(example = "Read the theoretical notes of laboratory work №...")]
    pub description: Option<String>,

    /// Activity type
    #[schema(value_type = String, example = "Material")]
    pub r#type: ActivityType,

    /// Optional deadline for the task
    #[schema(value_type = Option<String>, example = "2025-05-14T13:45:30Z", nullable)]
    pub deadline: Option<DateTime>,

    /// Maximum grade for the work performed
    #[schema(example = 10, nullable)]
    pub points: Option<i32>,

    /// Is this activity hidden?
    #[schema(example = false)]
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
