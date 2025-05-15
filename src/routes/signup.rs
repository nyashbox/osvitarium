use crate::{
    app::{state::AppState, status::AppStatus as Status},
    repositories::user::UserRepository,
};

use axum::{Json, extract::State};
use entity::sea_orm_active_enums::UserRole;
use sea_orm::ActiveEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::sync::Arc;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Request {
    pub username: String,
    pub password: String,
    pub role: String,
}

/// Create new user profile
#[utoipa::path(
    post,
    tag = "Authentication",
    path = "/signup",
    request_body = Request,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Already Exists")
    )
)]
pub async fn signup_post_handler<S>(
    State(state): State<Arc<AppState<S>>>,
    Json(request_body): Json<Request>,
) -> Result<Status, Status>
where
    S: UserRepository,
{
    let Request {
        username,
        password,
        role,
    } = request_body;

    let role = UserRole::try_from_value(&role)
        .map_err(|_| Status::InvalidArgument(Some("Invalid user role!".into())))?;

    Ok(
        if let Err(status) = state.db.find_by_username(&username).await {
            match status {
                Status::NotFound(_) => {
                    state.db.create(&username, &password, role).await?;

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

    use super::Request;

    use rstest::rstest;

    use axum::{
        Router,
        body::Body,
        http::{Request as AxumRequest, StatusCode},
        routing::{self},
    };
    use tower::ServiceExt;

    use crate::app::status::AppStatus as Status;
    use crate::{app::state::AppState, routes::signup::signup_post_handler};

    use crate::models::user::User;
    use crate::repositories::user::MockUserRepository;

    use entity::sea_orm_active_enums::UserRole;

    #[rstest]
    #[case::success("success", "Student", StatusCode::OK)]
    #[case::alredy_exists("exists", "Student", StatusCode::CONFLICT)]
    #[case::wrong_role("success", "Prince", StatusCode::BAD_REQUEST)]
    #[tokio::test]
    pub async fn signup_post_handler_test(
        #[case] username: &str,
        #[case] role: &str,
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

        mock.expect_create()
            .return_once(|_, _, _| Box::pin(async move { Ok(User::mock_user(UserRole::Student)) }));

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
                serde_json::to_string(&Request {
                    username: username.into(),
                    password: "password".into(),
                    role: role.into(),
                })
                .unwrap(),
            ))
            .unwrap();

        let res = router.oneshot(request).await.unwrap();

        assert_eq!(res.status(), expected);
    }
}
