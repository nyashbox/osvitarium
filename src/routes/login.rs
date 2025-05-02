use crate::{
    app::{error::AppStatus, state::AppState},
    services::user::UserService,
};

use axum::{Json, extract::State};
use serde::Serialize;

/// Authentication request
pub struct Request {
    /// Username
    username: String,

    /// Plain-text password
    password: String,
}

/// Authentication response
#[derive(Serialize)]
pub struct Response {
    /// Authentication token
    access_token: String,

    /// Token type
    token_type: String,

    /// Token TTL (in seconds)
    expires_in: u64,
}

pub async fn login_post_handler<S>(
    State(state): State<AppState<S>>,
    Json(request_body): Json<Request>,
) -> Result<Response, AppStatus>
where
    S: UserService,
{
    let Request { username, password } = request_body;

    // TODO: should we move it somewhere else?
    let expires_in: u64 = 3600;
    let token_type: String = "Bearer".into();

    let user_model = UserService::authenticate(&state.db, &username, &password).await?;

    let access_token =
        UserService::into_jwt(&state.db, &user_model, &state.secret, expires_in).await?;

    Ok(Response {
        access_token,
        token_type,
        expires_in,
    })
}

#[cfg(test)]
mod tests {
    mod login_post_handler {
        use axum::{Json, extract::State};

        use crate::{
            app::error::AppStatus, app::state::AppState, routes::login::Request,
            routes::login::login_post_handler, services::user::MockUserService,
        };

        use entity::user::Model as UserModel;

        #[tokio::test]
        async fn success() {
            let mut mock = MockUserService::new();

            mock.expect_authenticate().return_once(move |_, _| {
                Ok(UserModel {
                    user_id: 1,
                    username: "username".to_string(),
                    fullname: "John Doe".to_string(),
                    password: "$argon2id$v=19$m=16,t=2,p=1$cGFzc3dvcmQ$8vDS3rsezOjrur01dF12EA"
                        .to_string(),
                    description: "".to_string(),
                    metadata: "{}".into(),
                })
            });

            mock.expect_into_jwt()
                .return_once(move |_, _, _| Ok("mock_token".to_string()));

            let req = Request {
                username: "username".to_string(),
                password: "password".to_string(),
            };

            let response = login_post_handler(
                State(AppState {
                    db: mock,
                    secret: "secret".into(),
                }),
                Json(req),
            )
            .await;

            assert!(
                response.is_ok(),
                "When user sends CORRECT CREDENTIALS, 'OK' (200) MUST be returned!"
            );
        }

        #[tokio::test]
        async fn wrong_credentials() {
            let mut mock = MockUserService::new();

            mock.expect_authenticate()
                .return_once(move |_, _| Err(AppStatus::Unauthenticated(None)));

            mock.expect_into_jwt()
                .return_once(move |_, _, _| Ok("mock_token".to_string()));

            let req = Request {
                username: "username".to_string(),
                password: "password".to_string(),
            };

            let response = login_post_handler(
                State(AppState {
                    db: mock,
                    secret: "secret".into(),
                }),
                Json(req),
            )
            .await;

            assert!(
                matches!(response, Err(AppStatus::Unauthenticated(_))),
                "When user sends INCORRECT CREDENTIALS, 'Unauthorized' (401) MUST be returned!"
            );
        }
    }
}
