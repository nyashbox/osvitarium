use axum::http::StatusCode;
use osvitarium_backend::{repositories::user::UserRepository, routes::build_routes};

use crate::{
    test_builder::TestBuilder,
    utils::{build_app_state, empty_database},
};

use entity::sea_orm_active_enums::UserRole::Student;

use std::sync::Arc;

#[rstest::rstest]
#[case::success(StatusCode::OK)]
#[tokio::test]
pub async fn delete_my_profile_test(#[case] expected: StatusCode) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let student = UserRepository::create(&state.db, "username", "password", Student)
        .await
        .unwrap();

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials("username", "password")
        .route("DELETE", "/me");

    let response = request.run().await;

    assert_eq!(response.status(), expected);
    assert!(
        UserRepository::find_by_id(&state.db, student.user_id())
            .await
            .is_err()
    );
    empty_database(&state.db).await;
}
