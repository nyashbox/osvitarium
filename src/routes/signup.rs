use crate::models::user::CreateUserDTO;
use crate::routes::prelude::*;

use crate::repositories::user::UserRepository;

/// Create new user profile
#[utoipa::path(
    post,
    tag = "Authentication",
    path = "/signup",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Already Exists")
    )
)]
pub async fn signup_post_handler<S>(
    State(state): State<Arc<AppState<S>>>,
    Json(request_body): Json<CreateUserDTO>,
) -> Result<Status, Status>
where
    S: UserRepository,
{
    Ok(
        if let Err(status) = state.db.find_by_username(&request_body.username).await {
            match status {
                Status::NotFound(_) => {
                    UserRepository::create_from_dto(&state.db, request_body).await?;

                    Status::Ok("Operation successful!".into())
                }
                _ => Status::Internal(None),
            }
        } else {
            Status::AlreadyExists(None)
        },
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rstest::rstest;

    use axum::{
        Router,
        body::Body,
        http::{Request as AxumRequest, StatusCode},
        routing::{self},
    };
    use tower::ServiceExt;

    use crate::{app::state::AppState, routes::signup::signup_post_handler};
    use crate::{app::status::AppStatus as Status, models::user::CreateUserDTO};

    use crate::models::user::User;
    use crate::repositories::user::MockUserRepository;

    use entity::sea_orm_active_enums::UserRole;

    #[rstest]
    #[case::success("success", UserRole::Student, StatusCode::OK)]
    #[case::alredy_exists("exists", UserRole::Student, StatusCode::CONFLICT)]
    #[tokio::test]
    pub async fn signup_post_handler_test(
        #[case] username: &str,
        #[case] role: UserRole,
        #[case] expected: StatusCode,
    ) {
        let mut mock = MockUserRepository::new();

        mock.expect_find_by_username().return_once(|usr| {
            let usr = usr.to_owned();

            Box::pin(async move {
                match usr.as_str() {
                    "exists" => Ok(User::mock_user(UserRole::Student)),
                    _ => Err(Status::NotFound(None)),
                }
            })
        });

        mock.expect_create_from_dto()
            .return_once(|_| Box::pin(async move { Ok(User::mock_user(UserRole::Student)) }));

        let router = Router::new()
            .route("/signup", routing::post(signup_post_handler))
            .with_state(Arc::new(AppState {
                db: mock,
                secret: "secret".into(),
                jitsi_app_id: "app_state".into(),
                jitsi_secret: "secret".into(),
                jitsi_kid: "secret".into(),
            }));

        let request = AxumRequest::builder()
            .uri("/signup")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&CreateUserDTO {
                    username: username.into(),
                    fullname: None,
                    description: None,
                    password: "password".into(),
                    role: role,
                })
                .unwrap(),
            ))
            .unwrap();

        let res = router.oneshot(request).await.unwrap();

        assert_eq!(res.status(), expected);
    }
}
