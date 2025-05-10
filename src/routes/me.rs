use crate::{app::status::AppStatus as Status, models::user::User};

use crate::models::user::UserRepresentation as Response;

use axum::{Json, extract::Extension};

pub async fn me_get_handler(Extension(user): Extension<User>) -> Result<Json<Response>, Status> {
    Ok(Json(user.into()))
}

#[cfg(test)]
mod tests {
    mod me_get_handler {
        use std::sync::Arc;

        use tower::ServiceExt;

        use axum::{Router, body::Body, http::StatusCode, middleware, routing};
        use entity::{
            sea_orm_active_enums::UserRole, student::Model as StudentModel,
            user::Model as UserModel,
        };

        use crate::{
            app::state::AppState, middleware::auth::auth_middleware, models::user::User,
            repositories::user::MockUserRepository, routes::me::me_get_handler,
        };

        use axum::http::Request;

        #[tokio::test]
        pub async fn success() {
            let mut mock = MockUserRepository::new();
            let auth_header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdXhfc3ViIjoxLCJzdWIiOjEsImlhdCI6MTExMTExMTExMTEsImV4cCI6OTk5OTk5OTk5OTksInJvbGUiOiJzdHVkZW50In0.zOIV8xbN1eIM_n7AciKbuTkpgKbCHK6Kf1vFMgv3SKY";

            mock.expect_find_by_id().returning(move |_| {
                Box::pin(async move {
                    Ok(User::Student(
                        UserModel {
                            user_id: 1,
                            username: "johndoe".into(),
                            fullname: "John Doe".into(),
                            password: "password".into(),
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

            let router = Router::new().route(
                "/me",
                routing::get(me_get_handler).layer(middleware::from_fn_with_state(
                    Arc::new({
                        AppState {
                            db: mock,
                            secret: "secret".into(),
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
