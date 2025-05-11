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
            services::utils::hash_password,
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
                            },
                        }),
                        _ => Err(Status::AlreadyExists(None)),
                    }
                })
            });

            let router: Router = Router::new()
                .route("/courses", routing::post(courses_post_handler))
                .layer(Extension(User::Teacher(
                    entity::user::Model {
                        user_id: 1,
                        username: "username".into(),
                        fullname: "John Doe".into(),
                        password: hash_password("wrong").unwrap(),
                        description: " ".into(),
                        metadata: "{}".into(),
                        role: entity::sea_orm_active_enums::UserRole::Teacher,
                    },
                    entity::teacher::Model {
                        user_id: 1,
                        teacher_id: 1,
                    },
                )))
                .with_state(Arc::new(AppState {
                    db: mock,
                    secret: "secret".into(),
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
