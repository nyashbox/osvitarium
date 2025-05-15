use crate::app::status::AppStatus as Status;

use crate::models::{Course, User};

use entity::course::{ActiveModel as ActiveCourseModel, Entity as CourseEntity};

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, SqlErr};

use log::error;

#[mockall::automock]
pub trait CourseRepository {
    /// Create new course
    ///
    /// # Arguments
    ///
    /// * 'title' - Course title
    ///
    /// # Returns
    ///
    /// On success: Created course
    /// On failure: Application status
    fn create_course(
        &self,
        title: &str,
    ) -> impl std::future::Future<Output = Result<Course, Status>> + Send;

    /// Find all courses
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// On success: Vector containing all courses
    /// On failure: Application status
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Course>, Status>> + Send;

    /// Find course by ID
    ///
    /// # Arguments
    ///
    /// * 'id' - course identifier (ID)
    ///
    /// # Returns
    ///
    /// On success: Course with specified ID
    /// On failure: Application status
    fn find_by_id(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<Course, Status>> + Send;

    /// Add course instructor
    ///
    /// # Arguments
    ///
    /// * 'coruse_id' - Course identifier
    /// * 'course_instructor' - Instructor
    ///
    /// # Returns
    ///
    /// On success: Nothing
    /// On failure: Application status
    fn add_instructor(
        &self,
        course_id: i32,
        instructor: &User,
    ) -> impl std::future::Future<Output = Result<(), Status>> + Send;

    /// Add course attendee (student)
    ///
    /// # Arguments
    ///
    /// * 'course_id' - Course identifier
    /// * 'attendee' - Attendee
    ///
    /// # Returns
    ///
    /// On success: Nothing
    /// On failure: Application status
    fn add_attendee(
        &self,
        course_id: i32,
        attendee: &User,
    ) -> impl std::future::Future<Output = Result<(), Status>> + Send;
}

impl CourseRepository for sea_orm::DatabaseConnection {
    async fn create_course(&self, title: &str) -> Result<Course, Status> {
        let course_model = ActiveCourseModel {
            title: sea_orm::ActiveValue::Set(title.into()),
            ..Default::default()
        }
        .insert(self)
        .await
        .map_err(|e| {
            if let Some(sql_error) = e.sql_err() {
                match sql_error {
                    SqlErr::UniqueConstraintViolation(_) => Status::AlreadyExists(None),
                    _ => Status::Internal(None),
                }
            } else {
                error!("Failed to create new course: {e}");

                Status::Internal(None)
            }
        })?;

        Ok(Course {
            model: course_model,
        })
    }

    async fn find_all(&self) -> Result<Vec<Course>, Status> {
        let courses = CourseEntity::find().all(self).await.map_err(|e| {
            error!("Failed to get all courses from the database: {e}");

            Status::Internal(None)
        })?;

        Ok(courses.into_iter().map(|model| Course { model }).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Course, Status> {
        let course = CourseEntity::find_by_id(id).one(self).await.map_err(|e| {
            error!("Failed to find course by ID: {e}");

            Status::Internal(None)
        })?;

        match course {
            Some(model) => Ok(Course { model }),
            None => Err(Status::NotFound(None)),
        }
    }

    async fn add_instructor(&self, course_id: i32, instructor: &User) -> Result<(), Status> {
        let _ = entity::course_instructor::ActiveModel {
            course_id: Set(course_id),
            instructor_id: Set(instructor.role_id()),
        }
        .insert(self)
        .await
        .map_err(|e| {
            error!("Failed to add instructor to the course: {e}");

            Status::Internal(None)
        })?;

        Ok(())
    }

    async fn add_attendee(&self, course_id: i32, attendee: &User) -> Result<(), Status> {
        let _ = entity::course_student::ActiveModel {
            course_id: Set(course_id),
            student_id: Set(attendee.role_id()),
        }
        .insert(self)
        .await
        .map_err(|e| {
            error!("Failed to add course attendee: {e}");

            Status::Internal(None)
        })?;

        Ok(())
    }
}
