use crate::routes::prelude::*;

use crate::repositories::user::UserRepository;
use crate::services::user::UserService;

use utoipa::ToSchema;

/// Authentication request
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "AuthRequest", description = "Authentication request")]
pub struct Request {
    /// Username
    pub username: String,

    /// Plain-text password
    pub password: String,
}

/// Authentication response
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "AuthToken", description = "Authentication token (JWT)")]
pub struct Response {
    /// Authentication token
    pub access_token: String,

    /// Token type
    pub token_type: String,

    /// Token TTL (in seconds)
    pub expires_in: u64,
}

/// Perform authentication with username/password pair
#[utoipa::path(
    post,
    tag = "Authentication",
    path = "/login",
    responses(
        (status = 200, description = "Success")
    )
)]
pub async fn login_post_handler<S>(
    State(state): State<Arc<AppState<S>>>,
    Json(request_body): Json<Request>,
) -> Result<Json<Response>, Status>
where
    S: UserRepository,
{
    let Request { username, password } = request_body;

    // TODO: should we move it somewhere else?
    let expires_in: u64 = 3600;
    let token_type: String = "Bearer".into();

    let user_model = state
        .db
        .find_by_username(username.as_str())
        .await
        .map_err(|e| match e {
            Status::Unauthenticated(_) | Status::NotFound(_) => {
                Status::Unauthenticated(Some("Incorrect username or password!".into()))
            }
            _ => Status::Internal(None),
        })?;

    user_model.authenticate(password.as_str())?;

    let access_token = user_model.into_jwt(state.secret.as_str(), expires_in)?;

    Ok(Json(Response {
        access_token,
        token_type,
        expires_in,
    }))
}

#[cfg(test)]
mod tests {
    mod login_post_handler {
        use axum::{
            Router,
            body::Body,
            http::{Request as AxumRequest, StatusCode},
            routing::{self},
        };

        use crate::{
            app::state::AppState,
            models::user::User,
            repositories::user::MockUserRepository,
            routes::login::{Request, login_post_handler},
        };

        use entity::sea_orm_active_enums::UserRole;

        use std::sync::Arc;

        use tower::ServiceExt;

        #[tokio::test]
        async fn success() {
            let mut mock = MockUserRepository::new();

            mock.expect_find_by_username()
                .return_once(|_| Box::pin(async move { Ok(User::mock_user(UserRole::Student)) }));

            let router = Router::new()
                .route("/login", routing::post(login_post_handler))
                .with_state(Arc::new(AppState {
                    db: mock,
                    secret: "secret".into(),
                    jitsi_app_id: "app".into(),
                    jitsi_secret: "secret".into(),
                    jitsi_kid: "secret".into(),
                }));

            let request = AxumRequest::builder()
                .uri("/login")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&Request {
                        username: "username".to_string(),
                        password: "password".to_string(),
                    })
                    .unwrap(),
                ))
                .unwrap();

            let res = router.oneshot(request).await.unwrap();

            assert_eq!(
                res.status(),
                StatusCode::OK,
                "When user sends CORRECT CREDENTIALS, 'OK' (200) MUST be returned!"
            );
        }

        #[tokio::test]
        async fn wrong_credentials() {
            let mut mock = MockUserRepository::new();

            mock.expect_find_by_username().return_once(move |_| {
                Box::pin(async move { Ok(User::mock_user(UserRole::Student)) })
            });

            let router = Router::new()
                .route("/login", routing::post(login_post_handler))
                .with_state(Arc::new(AppState {
                    db: mock,
                    secret: "secret".into(),
                    jitsi_app_id: "app_id".into(),
                    jitsi_secret: "secret".into(),
                    jitsi_kid: "secret".into(),
                }));

            let request = AxumRequest::builder()
                .uri("/login")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&Request {
                        username: "username".to_string(),
                        password: "wrong_password".to_string(),
                    })
                    .unwrap(),
                ))
                .unwrap();

            let res = router.oneshot(request).await.unwrap();

            assert_eq!(
                res.status(),
                StatusCode::UNAUTHORIZED,
                "When user sends INCORRECT CREDENTIALS, 'Unauthorized' (401) MUST be returned!"
            );
        }
    }
}
