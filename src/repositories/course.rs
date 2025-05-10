use crate::app::status::AppStatus as Status;

use entity::course::{
    ActiveModel as ActiveCourseModel, Entity as CourseEntity, Model as CourseModel,
};

use sea_orm::{ActiveModelTrait, EntityTrait};

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
}

/// Represents course
pub struct Course {
    /// Course model
    pub model: CourseModel,
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
            error!("Failed to create new course: {e}");

            Status::Internal(None)
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
}
