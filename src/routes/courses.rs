pub mod get {
    use std::sync::Arc;

    use crate::app::state::AppState;
    use crate::models::user::User;
    use crate::{app::status::AppStatus as Status, repositories::course::CourseRepository};

    use crate::models::course::CourseRepresentation as CourseGetResponse;

    use axum::extract::{Path, State};
    use axum::{Extension, response::Json};

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

    pub async fn courses_get_one_handler<S>(
        Extension(_user): Extension<User>,
        Path(course_id): Path<i32>,
        State(state): State<Arc<AppState<S>>>,
    ) -> Result<Json<CourseGetResponse>, Status>
    where
        S: CourseRepository,
    {
        Ok(Json(state.db.find_by_id(course_id).await?.into()))
    }
}

pub mod post {
    use axum::{Extension, Json, extract::State};
    use serde::{Deserialize, Serialize};

    use crate::app::state::AppState;
    use crate::app::status::AppStatus as Status;

    use crate::models::User;
    use crate::repositories::course::CourseRepository;

    use std::sync::Arc;

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub title: String,
    }

    pub async fn courses_post_handler<S>(
        Extension(user): Extension<User>,
        State(state): State<Arc<AppState<S>>>,
        Json(request_body): Json<Request>,
    ) -> Result<Status, Status>
    where
        S: CourseRepository,
    {
        // Only teachers are allowed to create courses
        if !user.is_teacher() {
            return Err(Status::Unauthenticated(None));
        }

        CourseRepository::create_course(&state.db, &request_body.title).await?;

        Ok(Status::Ok("Course created successfully!".into()))
    }

    #[cfg(test)]
    mod tests {
        use axum::{
            Router,
            body::Body,
            http::{Request as AxumRequest, StatusCode},
            routing,
        };
        use sea_orm::sqlx::types::chrono;
        use tower::ServiceExt;

        use crate::{
            models::Course, repositories::course::MockCourseRepository,
        };

        use super::*;

        #[rstest::rstest]
        #[case::success("success", StatusCode::OK)]
        #[case::exists("exists", StatusCode::CONFLICT)]
        #[tokio::test]
        async fn courses_post_handler_test(#[case] title: &str, #[case] expected: StatusCode) {
            let mut mock = MockCourseRepository::new();
            mock.expect_create_course().return_once(|title| {
                let title = title.to_owned();

                Box::pin(async move {
                    match title.as_str() {
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

            let router: Router = Router::new()
                .route("/courses", routing::post(courses_post_handler))
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
                    serde_json::to_string(&Request {
                        title: title.into(),
                    })
                    .unwrap(),
                ))
                .unwrap();

            let res = router.oneshot(request).await.unwrap();

            assert_eq!(res.status(), expected);
        }
    }
}
