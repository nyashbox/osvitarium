use crate::{
    app::{state::AppState, status::AppStatus as Status},
    models::{User, meeting::MeetingRepresentation as Response},
    repositories::{course::CourseRepository, meeting::MeetingRepository},
};

use axum::extract::{Extension, Path, State};

use std::sync::Arc;

use axum::Json;

pub mod get {
    use crate::meetings::jitsi::build_jitsi_session;

    use super::*;

    pub async fn meeting_get<S>(
        Extension(user): Extension<User>,
        State(state): State<Arc<AppState<S>>>,
        Path(course_id): Path<i32>,
    ) -> Result<Json<Response>, Status>
    where
        S: MeetingRepository + CourseRepository,
    {
        let course = CourseRepository::find_by_id(&state.db, course_id).await?;

        match &user {
            User::Student(student) => {
                if student.is_attending(&course) {
                    let meeting =
                        MeetingRepository::create_course_meeting(&state.db, &course).await?;

                    Ok(Json(build_jitsi_session(
                        &meeting,
                        &user,
                        &state.jitsi_app_id,
                        &state.jitsi_secret,
                        &state.jitsi_kid,
                    )))
                } else {
                    Err(Status::PermissionDenied(Some(
                        "Only participants can join course meetings!".into(),
                    )))
                }
            }
            User::Teacher(teacher) => {
                if teacher.is_instructing(&course) {
                    let meeting =
                        MeetingRepository::create_course_meeting(&state.db, &course).await?;

                    Ok(Json(build_jitsi_session(
                        &meeting,
                        &user,
                        &state.jitsi_app_id,
                        &state.jitsi_secret,
                        &state.jitsi_kid,
                    )))
                } else {
                    Err(Status::PermissionDenied(Some(
                        "Only participants can join course meetings!".into(),
                    )))
                }
            }
            _ => Err(Status::PermissionDenied(Some(
                "Only students and teachers can join course meetings!".into(),
            ))),
        }
    }
}

pub mod post {
    use crate::meetings::jitsi::build_jitsi_session;

    use super::*;

    pub async fn meeting_post_create<S>(
        Extension(user): Extension<User>,
        State(state): State<Arc<AppState<S>>>,
        Path(course_id): Path<i32>,
    ) -> Result<Json<Response>, Status>
    where
        S: MeetingRepository + CourseRepository,
    {
        if let User::Teacher(teacher) = &user {
            let course = CourseRepository::find_by_id(&state.db, course_id).await?;
            if teacher.is_instructing(&course) {
                let meeting = MeetingRepository::create_course_meeting(&state.db, &course).await?;

                Ok(Json(build_jitsi_session(
                    &meeting,
                    &user,
                    &state.jitsi_app_id,
                    &state.jitsi_secret,
                    &state.jitsi_kid,
                )))
            } else {
                Err(Status::PermissionDenied(Some(
                    "Only course instructors are allowed to create course meetings!".into(),
                )))
            }
        } else {
            Err(Status::PermissionDenied(Some(
                "Only course instructors are allowed to create course meetings!".into(),
            )))
        }
    }
}
