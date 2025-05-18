use axum::http::StatusCode;
use osvitarium_backend::models::course::CourseCreateDTO;
use osvitarium_backend::repositories::course::CourseRepository;
use osvitarium_backend::{repositories::user::UserRepository, routes::build_routes};


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
#[case::wrong_role("success", UserRole::Student, StatusCode::FORBIDDEN)]
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
        .with_body(&CourseCreateDTO {
            title: title.into(),
            description: None,
            is_active: true,
        });

    let res = request.run().await;

    assert_eq!(res.status(), expected);
    empty_database(&state.db).await;
}

#[rstest::rstest]
#[case::success(1, StatusCode::OK)]
#[case::bad_id(0, StatusCode::NOT_FOUND)]
#[tokio::test]
pub async fn courses_get_one_test(#[case] mut id: i32, #[case] expected: StatusCode) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let _ = UserRepository::create(&state.db, "username", "password", Student).await;

    let course = CourseRepository::create_course(&state.db, "exists")
        .await
        .unwrap();

    if id > 0 {
        id = course.model.course_id;
    }

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials("username", "password")
        .route("GET", &format!("/courses/{id}"));

    let res = request.run().await;

    assert_eq!(res.status(), expected);
    empty_database(&state.db).await;
}

#[rstest::rstest]
#[case::success("instructor", StatusCode::OK)]
#[case::insufficient_permissions("student", StatusCode::FORBIDDEN)]
#[tokio::test]
pub async fn delete_course_by_id(#[case] username: &str, #[case] expected: StatusCode) {
    let state = Arc::new(build_app_state("secret").await);
    empty_database(&state.db).await;

    let router = build_routes(state.clone());

    let course = CourseRepository::create_course(&state.db, "Mathematics 101")
        .await
        .unwrap();

    let instructor = UserRepository::create(&state.db, "instructor", "password", UserRole::Teacher)
        .await
        .unwrap();

    CourseRepository::add_instructor(&state.db, course.model.course_id, &instructor)
        .await
        .unwrap();

    let _ = UserRepository::create(&state.db, "student", "password", Student).await;

    let id = course.model.course_id;

    let request = TestBuilder::new(router)
        .authenticate_as(Student)
        .with_credentials(username, "password")
        .route("DELETE", &format!("/courses/{id}"));

    let res = request.run().await;

    assert_eq!(res.status(), expected);
    empty_database(&state.db).await;
}
