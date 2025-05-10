use osvitarium_backend::repositories::course::CourseRepository;

use crate::utils::{build_app_state, empty_database};

#[rstest::rstest]
#[case::success("success")]
#[should_panic]
#[case::exists("exists")]
#[tokio::test]
async fn create_course_test(#[case] title: &str) {
    let state = build_app_state("secret".into()).await;
    empty_database(&state.db).await;

    let _course = CourseRepository::create_course(&state.db, "exists").await;
    let course = CourseRepository::create_course(&state.db, title).await;

    empty_database(&state.db).await;
    assert!(course.is_ok());
}
