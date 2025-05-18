use crate::routes::prelude::*;

use crate::meetings::jitsi::build_jitsi_session;
use crate::models::meeting::MeetingDTO;

use crate::repositories::meeting::MeetingRepository;

/// Get course meeting credentials
#[utoipa::path(
    get, 
    tag = "Courses",
    path = "/courses/{id}/meeting", 
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_course_meeting_credentials<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
    Path(course_id): Path<i32>,
) -> Result<Json<MeetingDTO>, Status>
where
    S: MeetingRepository + CourseRepository,
{
    let course = CourseRepository::find_by_id(&state.db, course_id).await?;

    match &user {
        User::Student(student) => {
            if student.is_attending(&course) {
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
                    "Only participants can join course meetings!".into(),
                )))
            }
        }
        User::Teacher(teacher) => {
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
                    "Only participants can join course meetings!".into(),
                )))
            }
        }
        _ => Err(Status::PermissionDenied(Some(
            "Only students and teachers can join course meetings!".into(),
        ))),
    }
}

/// Create new course meeting
#[utoipa::path(
    post, 
    tag = "Courses",
    path = "/courses/{id}/meeting", 
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_new_course_meeting<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
    Path(course_id): Path<i32>,
) -> Result<Json<MeetingDTO>, Status>
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
