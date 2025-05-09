use std::sync::Arc;

use crate::utils::*;

use entity::sea_orm_active_enums::UserRole;
use osvitarium_backend::repositories::user::UserRepository;
use osvitarium_backend::routes::build_routes;
use osvitarium_backend::routes::signup::Request;

use axum::{
    body::Body,
    http::{Request as AxumRequest, StatusCode},
};

use tower::ServiceExt;

use rstest::rstest;

#[rstest]
#[case::success("username", "Student", StatusCode::OK)]
#[case::exists("exists", "Student", StatusCode::CONFLICT)]
#[tokio::test]
async fn singup_post_handler(
    #[case] username: &str,
    #[case] role: &str,
    #[case] expected: StatusCode,
) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    UserRepository::create(&state.db, "exists", "password", UserRole::Student)
        .await
        .expect("Student must be inserted without any issues!");

    let router = build_routes(state.clone());

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

    empty_database(&state.db).await;
}
