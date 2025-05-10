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

#[rstest::rstest]
#[tokio::test]
async fn find_all_test() {
    let state = build_app_state("secret".into()).await;
    empty_database(&state.db).await;

    // 0 courses
    {
        let courses = CourseRepository::find_all(&state.db).await.unwrap();
        assert_eq!(courses.len(), 0);
    }

    // 1 course
    {
        let _ = CourseRepository::create_course(&state.db, "find_all_test 1")
            .await
            .unwrap();

        let courses = CourseRepository::find_all(&state.db).await.unwrap();
        assert_eq!(courses.len(), 1);
    }

    // 2 courses
    {
        let _ = CourseRepository::create_course(&state.db, "find_all_test 2")
            .await
            .unwrap();

        let courses = CourseRepository::find_all(&state.db).await.unwrap();
        assert_eq!(courses.len(), 2);
    }

    empty_database(&state.db).await;
}
