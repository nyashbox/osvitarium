use axum::http::StatusCode;
use osvitarium_backend::{repositories::user::UserRepository, routes::build_routes};

use crate::{
    test_builder::TestBuilder,
    utils::{build_app_state, empty_database},
};

use entity::sea_orm_active_enums::UserRole::Student;

use std::sync::Arc;

#[rstest::rstest]
#[tokio::test]
pub async fn courses_get_test() {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let _ =
        UserRepository::create(&state.db, "courses_get_test_username", "password", Student).await;

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials("courses_get_test_username", "password")
        .route("GET", "/courses");

    let response = request.run().await;

    assert_eq!(response.status(), StatusCode::OK);
    empty_database(&state.db).await;
}
