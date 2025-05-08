use crate::{
    app::{state::AppState, status::AppStatus as Status},
    repositories::user::User,
    services::user::JWTClaims,
};

use crate::repositories::user::UserRepository;

use axum::{Json, extract::State};

use sea_orm::prelude::Json as SeaJson;

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Serialize;

use std::sync::Arc;

#[derive(Serialize)]
pub struct Response {
    user_id: i32,
    username: String,
    fullname: String,
    description: String,
    role: String,
    role_id: i32,
    metadata: SeaJson,
}

pub async fn me_get_handler<S>(
    State(state): State<Arc<AppState<S>>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Response>, Status>
where
    S: UserRepository,
{
    let token = decode::<JWTClaims>(
        bearer.token(),
        &DecodingKey::from_secret(state.secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| {
        log::error!("Error while decoding JWT token: {e}");

        Status::Internal(None)
    })?
    .claims;

    let user = state.db.find_by_id(token.sub).await?;
    let user = match user {
        User::Student(model, _) => model,
        User::Teacher(model, _) => model,
        User::Principal(model, _) => model,
    };

    Ok(Json(Response {
        user_id: user.user_id,
        username: user.username,
        fullname: user.fullname,
        description: user.description,
        role: token.role,
        role_id: token.aux_sub,
        metadata: user.metadata,
    }))
}

#[cfg(test)]
mod tests {
    mod me_get_handler {
        use std::sync::Arc;

        use tower::ServiceExt;

        use axum::{Router, body::Body, http::StatusCode, routing::get};
        use entity::{
            sea_orm_active_enums::UserRole, student::Model as StudentModel,
            user::Model as UserModel,
        };

        use crate::{
            app::state::AppState,
            repositories::user::{MockUserRepository, User},
            routes::me::me_get_handler,
        };

        use axum::http::Request;

        #[tokio::test]
        pub async fn success() {
            let mut mock = MockUserRepository::new();
            let auth_header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdXhfc3ViIjoxLCJzdWIiOjEsImlhdCI6MTExMTExMTExMTEsImV4cCI6OTk5OTk5OTk5OTksInJvbGUiOiJzdHVkZW50In0.zOIV8xbN1eIM_n7AciKbuTkpgKbCHK6Kf1vFMgv3SKY";

            mock.expect_find_by_id().returning(move |_| {
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
            });

            let router: Router =
                Router::new()
                    .route("/me", get(me_get_handler))
                    .with_state(Arc::new(AppState {
                        db: mock,
                        secret: "secret".into(),
                    }));

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
