use crate::{
    models::activity::{ActivityDTO, CreateActivityDTO},
    repositories::activity::ActivityRepository,
    routes::prelude::*,
};

/// Get all course activities
#[utoipa::path(
    get,
    tag = "Courses",
    path = "/courses/{id}/activities",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated"),
        (status = 403, description = "Forbidden")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_course_activities<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
    Path(course_id): Path<i32>,
) -> Result<Json<Vec<ActivityDTO>>, Status>
where
    S: CourseRepository,
{
    let course = CourseRepository::find_by_id(&state.db, course_id).await?;

    match user {
        User::Student(student) => {
            if student.is_attending(&course) {
                let activities = CourseRepository::get_activities(&state.db, &course).await?;

                let activity: Vec<ActivityDTO> = activities
                    .into_iter()
                    .map(|activity| activity.into())
                    .collect();
                Ok(Json(activity.into()))
            } else {
                Err(Status::PermissionDenied(Some(
                    "Only course attendees can get course activities!".into(),
                )))
            }
        }
        User::Teacher(teacher) => {
            if teacher.is_instructing(&course) {
                let activities = CourseRepository::get_activities(&state.db, &course).await?;

                let activity: Vec<ActivityDTO> = activities
                    .into_iter()
                    .map(|activity| activity.into())
                    .collect();

                Ok(Json(activity.into()))
            } else {
                Err(Status::PermissionDenied(Some(
                    "Only course instructors can get course activities!".into(),
                )))
            }
        }
        User::Principal(_) => {
            let activities = CourseRepository::get_activities(&state.db, &course).await?;

            let activity: Vec<ActivityDTO> = activities
                .into_iter()
                .map(|activity| activity.into())
                .collect();

            Ok(Json(activity.into()))
        }
    }
}

/// Create new course acitvity
#[utoipa::path(
    post,
    tag = "Courses",
    path = "/courses/{id}/activities",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated"),
        (status = 403, description = "Forbidden")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_course_activity<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
    Path(course_id): Path<i32>,
    Json(activity): Json<CreateActivityDTO>,
) -> Result<Status, Status>
where
    S: ActivityRepository,
{
    match &user {
        User::Teacher(teacher) => {
            if teacher.is_instructing_id(course_id) {
                ActivityRepository::create_from_dto(&state.db, user.user_id(), course_id, activity)
                    .await?;

                Ok(Status::Ok("Course created successfully!".into()))
            } else {
                Err(Status::PermissionDenied(None))
            }
        }
        _ => Err(Status::PermissionDenied(None)),
    }
}
