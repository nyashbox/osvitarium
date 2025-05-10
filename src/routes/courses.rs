use std::sync::Arc;

use crate::app::state::AppState;
use crate::repositories::user::User;
use crate::{app::status::AppStatus as Status, repositories::course::CourseRepository};

use axum::extract::State;
use axum::{Extension, response::Json};

use serde::Serialize;

#[derive(Serialize)]
pub struct CourseJson {
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

pub async fn courses_get_handler<S>(
    Extension(_user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
) -> Result<Json<Vec<CourseJson>>, Status>
where
    S: CourseRepository,
{
    let courses = state.db.find_all().await?;

    let response: Vec<CourseJson> = courses
        .into_iter()
        .map(|course| CourseJson {
            course_id: course.model.course_id,
            title: course.model.title,
            description: course.model.description,
            is_active: course.model.is_active,
            created_at: course.model.created_at.to_string(),
        })
        .collect();

    Ok(Json(response))
}
