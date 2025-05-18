use crate::models::course::{CourseCreateDTO, CourseDTO};

use crate::routes::prelude::*;

/// Get all courses
#[utoipa::path(
    get,
    tag = "Courses",
    path = "/courses",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated"),
        (status = 403, description = "Forbidden")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_all_courses<S>(
    Extension(_user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
) -> Result<Json<Vec<CourseDTO>>, Status>
where
    S: CourseRepository,
{
    let courses = state.db.find_all().await?;

    let response: Vec<CourseDTO> = courses.into_iter().map(|course| course.into()).collect();

    Ok(Json(response))
}

/// Get course by course identifier (ID)
#[utoipa::path(
    get,
    tag = "Courses",
    path = "/courses/{id}",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated"),
        (status = 403, description = "Forbidden")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_course_by_id<S>(
    Extension(_user): Extension<User>,
    Path(course_id): Path<i32>,
    State(state): State<Arc<AppState<S>>>,
) -> Result<Json<CourseDTO>, Status>
where
    S: CourseRepository,
{
    Ok(Json(state.db.find_by_id(course_id).await?.into()))
}

/// Create new course
#[utoipa::path(
    post,
    tag = "Courses",
    path = "/courses",
    responses(
        (status = 200, description = "Success")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_new_course<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
    Json(request_body): Json<CourseCreateDTO>,
) -> Result<Status, Status>
where
    S: CourseRepository,
{
    match user {
        User::Teacher(_) => {
            let course = CourseRepository::create_from_dto(&state.db, request_body).await?;
            CourseRepository::add_instructor(&state.db, course.model.course_id, &user).await?;

            Ok(Status::Ok("Course created successfully!".into()))
        }
        _ => Err(Status::PermissionDenied(Some(
            "Only teachers are allowed to create new courses!".into(),
        ))),
    }
}

/// Delete course with specified course identifier (ID)
#[utoipa::path(
    delete,
    path = "/courses/{id}",
    tag = "Courses",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated"),
        (status = 403, description = "Permission denied"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn delete_course_by_id<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
    Path(course_id): Path<i32>,
) -> Result<Status, Status>
where
    S: CourseRepository,
{
    match user {
        User::Teacher(teacher) => {
            if teacher.is_instructing_id(course_id) {
                CourseRepository::delete_course_by_id(&state.db, course_id).await?;

                Ok(Status::Ok("Course was deleted successfully".into()))
            } else {
                Err(Status::PermissionDenied(Some(
                    "Only course instructors and principals can delete this course".into(),
                )))
            }
        }
        User::Principal(_) => {
            CourseRepository::delete_course_by_id(&state.db, course_id).await?;

            Ok(Status::Ok("Course was deleted successfully".into()))
        }
        _ => Err(Status::PermissionDenied(Some(
            "Only course instructors and principals can delete this course".into(),
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{
        Router,
        body::Body,
        http::{Request as AxumRequest, StatusCode},
        routing,
    };
    use sea_orm::sqlx::types::chrono;
    use tower::ServiceExt;

    use crate::{models::Course, repositories::course::MockCourseRepository};

    #[rstest::rstest]
    #[case::success("success", StatusCode::OK)]
    #[case::exists("exists", StatusCode::CONFLICT)]
    #[tokio::test]
    async fn courses_post_handler_test(#[case] title: &str, #[case] expected: StatusCode) {
        let mut mock = MockCourseRepository::new();
        mock.expect_create_from_dto().return_once(|course_dto| {
            Box::pin(async move {
                match course_dto.title.as_str() {
                    "success" => Ok(Course {
                        model: entity::course::Model {
                            course_id: 1,
                            title: "success".into(),
                            description: None,
                            is_active: true,
                            created_at: chrono::Utc::now().naive_utc(),
                            is_running_meeting: false,
                        },
                    }),
                    _ => Err(Status::AlreadyExists(None)),
                }
            })
        });

        mock.expect_add_instructor()
            .return_once(|_, _| Box::pin(async move { Ok(()) }));

        let router: Router = Router::new()
            .route("/courses", routing::post(create_new_course))
            .layer(Extension(User::mock_user(
                entity::sea_orm_active_enums::UserRole::Teacher,
            )))
            .with_state(Arc::new(AppState {
                db: mock,
                secret: "secret".into(),
                jitsi_app_id: "app_id".into(),
                jitsi_secret: "secret".into(),
                jitsi_kid: "secret".into(),
            }));

        let request = AxumRequest::builder()
            .uri("/courses")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&CourseCreateDTO {
                    title: title.into(),
                    description: None,
                    is_active: true,
                })
                .unwrap(),
            ))
            .unwrap();

        let res = router.oneshot(request).await.unwrap();

        assert_eq!(res.status(), expected);
    }
}
