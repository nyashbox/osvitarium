use axum::http::StatusCode;
use entity::sea_orm_active_enums::UserRole;
use osvitarium_backend::{
    repositories::{course::CourseRepository, user::UserRepository},
    routes::build_routes,
};

use crate::{
    test_builder::TestBuilder,
    utils::{build_app_state, empty_database},
};

use std::sync::Arc;

#[rstest::rstest]
#[case::success("attendee", StatusCode::OK)]
#[case::insufficient_permissions("student", StatusCode::FORBIDDEN)]
#[tokio::test]
pub async fn meeting_course_get_test(#[case] username: &str, #[case] expected: StatusCode) {
    let state = Arc::new(build_app_state("secret").await);
    let router = build_routes(state.clone());

    empty_database(&state.db).await;

    let course = CourseRepository::create_course(&state.db, "title")
        .await
        .unwrap();

    let attendee = UserRepository::create(&state.db, "attendee", "password", UserRole::Student)
        .await
        .unwrap();

    let _student = UserRepository::create(&state.db, "student", "password", UserRole::Student)
        .await
        .unwrap();

    CourseRepository::add_attendee(&state.db, course.model.course_id, &attendee)
        .await
        .unwrap();

    let request = TestBuilder::new(router)
        .authenticate_as(UserRole::Student)
        .with_credentials(username, "password")
        .route(
            "GET",
            &format!("/courses/{}/meeting", course.model.course_id),
        );

    let res = request.run().await;

    assert_eq!(res.status(), expected);
    empty_database(&state.db).await;
}

#[rstest::rstest]
#[case::success("instructor", StatusCode::OK)]
#[case::insufficient_permissions("teacher", StatusCode::FORBIDDEN)]
#[tokio::test]
pub async fn meeting_course_post_test(#[case] username: &str, #[case] expected: StatusCode) {
    let state = Arc::new(build_app_state("secret").await);
    let router = build_routes(state.clone());

    empty_database(&state.db).await;

    let course = CourseRepository::create_course(&state.db, "title")
        .await
        .unwrap();

    let instructor = UserRepository::create(&state.db, "instructor", "password", UserRole::Teacher)
        .await
        .unwrap();

    CourseRepository::add_instructor(&state.db, course.model.course_id, &instructor)
        .await
        .unwrap();

    let _teacher = UserRepository::create(&state.db, "teacher", "password", UserRole::Teacher)
        .await
        .unwrap();

    let request = TestBuilder::new(router)
        .authenticate_as(UserRole::Student)
        .with_credentials(username, "password")
        .route(
            "POST",
            &format!("/courses/{}/meeting", course.model.course_id),
        );

    let res = request.run().await;

    assert_eq!(res.status(), expected);
    empty_database(&state.db).await;
}
