use std::sync::Arc;

use crate::utils::*;

use entity::sea_orm_active_enums::UserRole;
use osvitarium_backend::repositories::user::UserRepository;
use osvitarium_backend::routes::login::{Request, login_post_handler};

use axum::{
    Router,
    body::Body,
    http::{Request as AxumRequest, StatusCode},
    routing::post,
};
use tower::ServiceExt;

#[tokio::test]
pub async fn success() {
    let state = build_app_state("secret").await;
    empty_database(&state.db).await;

    UserRepository::create(&state.db, "username", "password", UserRole::Student)
        .await
        .expect("Student must be inserted without any issues!");

    let router: Router = Router::new()
        .route("/login", post(login_post_handler))
        .with_state(Arc::new(state));

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
    )
}

#[tokio::test]
pub async fn not_found() {
    let state = build_app_state("secret").await;
    empty_database(&state.db).await;

    let router: Router = Router::new()
        .route("/login", post(login_post_handler))
        .with_state(Arc::new(state));

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
    )
}
