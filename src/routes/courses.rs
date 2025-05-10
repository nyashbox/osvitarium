use std::sync::Arc;

use crate::app::state::AppState;
use crate::models::user::User;
use crate::{app::status::AppStatus as Status, repositories::course::CourseRepository};

use crate::models::course::CourseRepresentation as CourseGetResponse;

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
) -> Result<Json<Vec<CourseGetResponse>>, Status>
where
    S: CourseRepository,
{
    let courses = state.db.find_all().await?;

    let response: Vec<CourseGetResponse> =
        courses.into_iter().map(|course| course.into()).collect();

    Ok(Json(response))
}
