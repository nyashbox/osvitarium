use std::sync::Arc;

use crate::utils::*;

use entity::sea_orm_active_enums::UserRole;
use osvitarium_backend::repositories::user::UserRepository;
use osvitarium_backend::routes::build_routes;
use osvitarium_backend::routes::login::Request;

use axum::http::StatusCode;

use rstest::rstest;

#[rstest]
#[case::success("username", "password", StatusCode::OK)]
#[case::wrong_username("wrong_username", "password", StatusCode::UNAUTHORIZED)]
#[case::wrong_password("username", "wrong_password", StatusCode::UNAUTHORIZED)]
#[tokio::test]
async fn login_post_test(
    #[case] username: &str,
    #[case] password: &str,
    #[case] expected: StatusCode,
) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    UserRepository::create(&state.db, "username", "password", UserRole::Student)
        .await
        .expect("Student must be inserted without any issues!");

    let router = build_routes(state.clone());

    let response = request_post(router, "/login", &Request {
        username: username.to_string(),
        password: password.to_string(),
    })
    .await;

    assert_eq!(response.status(), expected);
    empty_database(&state.db).await;
}
