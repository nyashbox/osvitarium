use std::sync::Arc;

use crate::{
    app::{state::AppState, status::AppStatus as Status},
    repositories::user::UserRepository,
    services::user::JWTClaims,
};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub async fn auth_middleware<S>(
    State(state): State<Arc<AppState<S>>>,
    mut req: Request,
    next: Next,
) -> Result<Response, Status>
where
    S: UserRepository,
{
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(Status::Unauthenticated(None))?;

    let token = token.token();
    let claims = decode::<JWTClaims>(
        token,
        &DecodingKey::from_secret(state.secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|_| Status::Unauthenticated(None))?
    .claims;

    let user = state
        .db
        .find_by_id(claims.sub)
        .await
        .map_err(|_| Status::Unauthenticated(None))?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::repositories::user::{MockUserRepository, User};

    use jsonwebtoken::{EncodingKey, Header};
    use rstest::rstest;

    use axum::{Router, body::Body, http::StatusCode, routing::get};
    use std::time::{SystemTime, UNIX_EPOCH};

    use tower::ServiceExt;

    use entity::sea_orm_active_enums::UserRole;

    const MOCK_SECRET: &str = "secret";

    #[rstest]
    #[case::success(success_token(), StatusCode::OK)]
    #[case::expired(expired_token(), StatusCode::UNAUTHORIZED)]
    #[case::missing(missing_token(), StatusCode::UNAUTHORIZED)]
    #[tokio::test]
    pub async fn auth_middleware_test(#[case] token: String, #[case] expected: StatusCode) {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id().return_once(|id| {
            Box::pin(async move {
                match id {
                    1 => Ok(User::Student(
                        entity::user::Model {
                            user_id: 1,
                            username: "johndoe".into(),
                            fullname: "John Doe".into(),
                            password: "".into(),
                            description: " ".into(),
                            metadata: "{}".into(),
                            role: Some(UserRole::Student),
                        },
                        entity::student::Model {
                            user_id: 1,
                            student_id: 1,
                        },
                    )),
                    _ => Err(Status::NotFound(None)),
                }
            })
        });

        let router: Router = Router::new().route("/", get(|| async { "OK" })).layer(
            axum::middleware::from_fn_with_state(
                Arc::new(AppState {
                    db: mock,
                    secret: MOCK_SECRET.into(),
                }),
                auth_middleware,
            ),
        );

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap();

        let res = router.oneshot(request).await.unwrap();

        assert_eq!(res.status(), expected);
    }

    // Returns expired JWT token
    fn expired_token() -> String {
        jsonwebtoken::encode(
            &Header::default(),
            &JWTClaims {
                aux_sub: 1,
                sub: 1,
                role: "student".into(),
                exp: 1,
                iat: 1,
            },
            &EncodingKey::from_secret("secret".as_bytes()),
        )
        .unwrap()
    }

    // Returns correct JWT token
    fn success_token() -> String {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        jsonwebtoken::encode(
            &Header::default(),
            &JWTClaims {
                aux_sub: 1,
                sub: 1,
                role: "student".into(),
                exp: iat + 3600,
                iat,
            },
            &EncodingKey::from_secret("secret".as_bytes()),
        )
        .unwrap()
    }

    // Returns correct JWT token, but user will be missing in the DB
    fn missing_token() -> String {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        jsonwebtoken::encode(
            &Header::default(),
            &JWTClaims {
                aux_sub: 2,
                sub: 2,
                role: "student".into(),
                exp: iat + 3600,
                iat,
            },
            &EncodingKey::from_secret("secret".as_bytes()),
        )
        .unwrap()
    }
}
