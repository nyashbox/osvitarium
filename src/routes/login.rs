use crate::{
    app::{state::AppState, status::AppStatus as Status},
    repositories::user::UserRepository,
    services::user::UserService,
};

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use std::sync::Arc;

/// Authentication request
#[derive(Serialize, Deserialize)]
pub struct Request {
    /// Username
    pub username: String,

    /// Plain-text password
    pub password: String,
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
            body::Body,
            http::{Request as AxumRequest, StatusCode},
        };

        use crate::{
            app::state::AppState,
            repositories::user::{MockUserRepository, User},
            routes::{build_routes, login::Request},
            services::utils,
        };

        use entity::student::Model as StudentModel;

        use entity::{sea_orm_active_enums::UserRole, user::Model as UserModel};

        use std::sync::Arc;

        use tower::ServiceExt;

        #[tokio::test]
        async fn success() {
            let mut mock = MockUserRepository::new();

            mock.expect_find_by_username().return_once(|_| {
                Box::pin(async move {
                    Ok(User::Student(
                        UserModel {
                            user_id: 1,
                            username: "johndoe".into(),
                            fullname: "John Doe".into(),
                            password: utils::hash_password("password").unwrap(),
                            description: " ".into(),
                            metadata: "{}".into(),
                            role: Some(UserRole::Student),
                        },
                        StudentModel {
                            user_id: 1,
                            student_id: 1,
                        },
                    ))
                })
            });

            let router = build_routes(Arc::new(AppState {
                db: mock,
                secret: "secret".into(),
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
                Box::pin(async move {
                    Ok(User::Student(
                        UserModel {
                            user_id: 1,
                            username: "username".into(),
                            fullname: "John Doe".into(),
                            password: utils::hash_password("wrong").unwrap(),
                            description: " ".into(),
                            metadata: "{}".into(),
                            role: Some(UserRole::Student),
                        },
                        StudentModel {
                            user_id: 1,
                            student_id: 1,
                        },
                    ))
                })
            });

            let router = build_routes(Arc::new(AppState {
                db: mock,
                secret: "secret".into(),
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
                StatusCode::UNAUTHORIZED,
                "When user sends INCORRECT CREDENTIALS, 'Unauthorized' (401) MUST be returned!"
            );
        }
    }
}
