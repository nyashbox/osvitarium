use axum::http::StatusCode;
use osvitarium_backend::repositories::course::CourseRepository;
use osvitarium_backend::{repositories::user::UserRepository, routes::build_routes};

use osvitarium_backend::routes::courses::post;

use crate::{
    test_builder::TestBuilder,
    utils::{build_app_state, empty_database},
};

use entity::sea_orm_active_enums::UserRole::{self, Student};

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

#[rstest::rstest]
#[case::success("success", UserRole::Teacher, StatusCode::OK)]
#[case::wrong_role("success", UserRole::Student, StatusCode::UNAUTHORIZED)]
#[case::exists("exists", UserRole::Teacher, StatusCode::CONFLICT)]
#[tokio::test]
pub async fn courses_post_test(
    #[case] title: &str,
    #[case] role: UserRole,
    #[case] expected: StatusCode,
) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let _ = UserRepository::create(&state.db, "username", "password", role).await;

    let _ = CourseRepository::create_course(&state.db, "exists").await;

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials("username", "password")
        .route("POST", "/courses")
        .with_body(&post::Request {
            title: title.into(),
        });

    let res = request.run().await;

    assert_eq!(res.status(), expected);
    empty_database(&state.db).await;
}
