use axum::http::StatusCode;
use entity::sea_orm_active_enums::ActivityType;
use osvitarium_backend::models::activity::CreateActivityDTO;
use osvitarium_backend::repositories::course::CourseRepository;
use osvitarium_backend::{repositories::user::UserRepository, routes::build_routes};

use crate::{
    test_builder::TestBuilder,
    utils::{build_app_state, empty_database},
};

use entity::sea_orm_active_enums::UserRole::Student;

use std::sync::Arc;

#[rstest::rstest]
#[case::success(
    "attendee",
    StatusCode::OK,
    "When request is made by a course attendee, course activities MUST be returned!"
)]
#[case::insufficient_permissions(
    "student",
    StatusCode::FORBIDDEN,
    "When request is made by a foreign student, '403' (Forbidden) MUST be returned!"
)]
#[tokio::test]
pub async fn get_course_activities_test(
    #[case] username: &str,
    #[case] expected: StatusCode,
    #[case] _description: &str,
) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let course = CourseRepository::create_course(&state.db, "title")
        .await
        .unwrap();

    let _student = UserRepository::create(&state.db, "student", "password", Student)
        .await
        .unwrap();

    let attendee = UserRepository::create(&state.db, "attendee", "password", Student)
        .await
        .unwrap();

    CourseRepository::add_attendee(&state.db, course.model.course_id, &attendee)
        .await
        .unwrap();

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials(username, "password")
        .route(
            "GET",
            &format!("/courses/{}/activities", course.model.course_id),
        );

    let response = request.run().await;

    assert_eq!(response.status(), expected);
    empty_database(&state.db).await;
}

#[rstest::rstest]
#[case::success("instructor", StatusCode::OK)]
#[case::insufficient_permissions("student", StatusCode::FORBIDDEN)]
#[tokio::test]
pub async fn create_course_activity_test(#[case] username: &str, #[case] expected: StatusCode) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let course = CourseRepository::create_course(&state.db, "title")
        .await
        .unwrap();

    let _student = UserRepository::create(&state.db, "student", "password", Student)
        .await
        .unwrap();

    let instructor = UserRepository::create(
        &state.db,
        "instructor",
        "password",
        entity::sea_orm_active_enums::UserRole::Teacher,
    )
    .await
    .unwrap();

    CourseRepository::add_instructor(&state.db, course.model.course_id, &instructor)
        .await
        .unwrap();

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials(username, "password")
        .with_body(&CreateActivityDTO {
            title: "test_activity".into(),
            description: None,
            deadline: None,
            r#type: ActivityType::Material,
            is_hidden: false,
            points: None,
        })
        .route(
            "POST",
            &format!("/courses/{}/activities", course.model.course_id),
        );

    let response = request.run().await;

    assert_eq!(response.status(), expected);
    empty_database(&state.db).await;
}
