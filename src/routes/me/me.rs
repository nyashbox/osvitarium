use crate::routes::prelude::*;

use crate::models::user::UserDTO;

use crate::repositories::user::UserRepository;

/// Returns the authenticated user's information
#[utoipa::path(
    get, 
    tag = "Profile",
    path = "/me", 
    responses(
        (status = 200, description = "Success", body = UserDTO),
        (status = 401, description = "Unauthenticated")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn get_profile_information(
    Extension(user): Extension<User>,
) -> Result<Json<UserDTO>, Status> {
    Ok(Json(user.into()))
}

/// Delete user's own profile
#[utoipa::path(
    delete,
    tag = "Profile",
    path = "/me",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthenticated")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn delete_my_profile<S>(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState<S>>>,
) -> Result<Status, Status>
where
    S: UserRepository,
{
    UserRepository::delete_user(&state.db, &user).await?;

    Ok(Status::Ok("Profile deleted successfully!".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod me_get_handler {
        use super::*;

        use tower::ServiceExt;

        use axum::{body::Body, http::StatusCode, middleware, routing};

        use entity::sea_orm_active_enums::UserRole;

        use crate::{
            app::state::AppState, middleware::auth::auth_middleware, models::user::User,
            repositories::user::MockUserRepository,
        };

        use axum::http::Request;

        #[tokio::test]
        pub async fn success() {
            let mut mock = MockUserRepository::new();
            let auth_header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdXhfc3ViIjoxLCJzdWIiOjEsImlhdCI6MTExMTExMTExMTEsImV4cCI6OTk5OTk5OTk5OTksInJvbGUiOiJzdHVkZW50In0.zOIV8xbN1eIM_n7AciKbuTkpgKbCHK6Kf1vFMgv3SKY";

            mock.expect_find_by_id().returning(move |_| {
                Box::pin(async move { Ok(User::mock_user(UserRole::Student)) })
            });

            let router = Router::new().route(
                "/me",
                routing::get(get_profile_information).layer(middleware::from_fn_with_state(
                    Arc::new({
                        AppState {
                            db: mock,
                            secret: "secret".into(),
                            jitsi_app_id: "app_id".into(),
                            jitsi_secret: "secret".into(),
                            jitsi_kid: "secret".into(),
                        }
                    }),
                    auth_middleware,
                )),
            );

            let request = Request::builder()
                .uri("/me")
                .method("GET")
                .header("content-type", "application/json")
                .header("Authorization", auth_header)
                .body(Body::empty())
                .unwrap();

            let res = router.oneshot(request).await.unwrap();

            assert_eq!(
                res.status(),
                StatusCode::OK,
                "When user uses CORRECT JWT TOKEN, 'Ok' (200) MUST BE returned"
            );
        }
    }
}
