use std::sync::Arc;

use crate::utils::*;

use entity::sea_orm_active_enums::UserRole;
use osvitarium_backend::repositories::user::UserRepository;
use osvitarium_backend::routes::build_routes;
use osvitarium_backend::routes::signup::Request;

use axum::http::StatusCode;

use rstest::rstest;

#[rstest]
#[case::success("success", "Student", StatusCode::OK)]
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

    let response = request_post(router, "/signup", &Request {
        username: username.into(),
        password: "password".into(),
        role: role.into(),
    })
    .await;

    assert_eq!(response.status(), expected);

    empty_database(&state.db).await;
}
