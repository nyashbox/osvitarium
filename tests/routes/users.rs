use std::sync::Arc;

use crate::test_builder::TestBuilder;
use crate::utils::*;

use entity::sea_orm_active_enums::UserRole;
use osvitarium_backend::repositories::user::UserRepository;
use osvitarium_backend::routes::build_routes;

use axum::http::StatusCode;

#[rstest::rstest]
#[tokio::test]
async fn get_user_description_test() {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let user = UserRepository::create(&state.db, "username", "password", UserRole::Student)
        .await
        .unwrap();

    let user_id = user.user_id();

    let request = TestBuilder::new(router)
        .authenticate_as(UserRole::Student)
        .with_credentials("username", "password")
        .route("GET", &format!("/users/{user_id}"));

    let response = request.run().await;

    assert_eq!(response.status(), StatusCode::OK);
    empty_database(&state.db).await;
}
