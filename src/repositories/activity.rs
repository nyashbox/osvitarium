use std::future::Future;

use crate::{app::AppStatus as Status, models::activity::CreateActivityDTO};

use crate::models::Activity;

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, SqlErr};

use log::error;

pub trait ActivityRepository {
    /// Create new activity
    ///
    /// # Arguments
    ///
    /// * 'activity' - Activity model
    ///
    /// # Returns
    ///
    /// On success: Inserted activity
    /// On failure: Application status
    fn create(&self, activity: &Activity) -> impl Future<Output = Result<Activity, Status>> + Send;

    /// Create new activity from DTO
    ///
    /// # Arguments
    ///
    /// * 'author_id' - Author ID
    /// * 'course_id' - Course ID
    /// * 'activity' - Activity DTO
    ///
    /// # Returns
    ///
    /// This function returns nothing
    fn create_from_dto(
        &self,
        author_id: i32,
        course_id: i32,
        activity: CreateActivityDTO,
    ) -> impl Future<Output = Result<Activity, Status>> + Send;

    /// Find acitvity by ID
    ///
    /// # Arguments
    ///
    /// * 'activity_id' - Activity ID
    ///
    /// # Returns
    ///
    /// On success: Found activity
    /// On failure: Application status
    fn find_by_id(&self, activity_id: i32)
    -> impl Future<Output = Result<Activity, Status>> + Send;

    /// Delete activity
    ///
    /// # Arguments
    ///
    /// * 'activity' - Activity model
    ///
    /// # Returns
    ///
    /// On success: Nothing
    /// On failure: Application status
    fn delete(&self, activity: &Activity) -> impl Future<Output = Result<(), Status>> + Send;

    /// Delete activity by ID
    ///
    /// # Arguments
    ///
    /// * 'activity_id' - Activity ID
    ///
    /// # Returns
    ///
    /// On success: Nothing
    /// On failure: Application status
    fn delete_by_id(&self, activity_id: i32) -> impl Future<Output = Result<(), Status>> + Send;
}

impl ActivityRepository for sea_orm::DatabaseConnection {
    async fn create(&self, activity: &Activity) -> Result<Activity, Status> {
        let model = activity.activity_model.clone().into_active_model();

        let activity_model = model.insert(self).await.map_err(|e| {
            if let Some(sql_error) = e.sql_err() {
                match sql_error {
                    SqlErr::UniqueConstraintViolation(_) => Status::AlreadyExists(None),
                    _ => Status::Internal(None),
                }
            } else {
                error!("Failed to create new activity: {e}");

                Status::Internal(None)
            }
        })?;

        Ok(Activity { activity_model })
    }

    async fn create_from_dto(
        &self,
        author_id: i32,
        course_id: i32,
        activity: CreateActivityDTO,
    ) -> Result<Activity, Status> {
        let activity_model = entity::activity::ActiveModel {
            title: Set(activity.title),
            description: Set(activity.description),
            r#type: Set(activity.r#type),
            deadline: Set(activity.deadline),
            points: Set(activity.points),
            is_hidden: Set(activity.is_hidden),
            author_id: Set(author_id),
            course_id: Set(course_id),
            ..Default::default()
        }
        .insert(self)
        .await
        .map_err(|e| {
            error!("Failed to create new course: {e}");

            Status::Internal(None)
        })?;

        Ok(Activity { activity_model })
    }

    async fn find_by_id(&self, activity_id: i32) -> Result<Activity, Status> {
        let activity = entity::activity::Entity::find_by_id(activity_id)
            .one(self)
            .await
            .map_err(|e| {
                error!("Failed to get activity by ID: {e}");

                Status::Internal(None)
            })?;

        match activity {
            Some(activity_model) => Ok(Activity { activity_model }),
            None => Err(Status::NotFound(None)),
        }
    }

    async fn delete(&self, activity: &Activity) -> Result<(), Status> {
        self.delete_by_id(activity.activity_model.acitvity_id).await
    }

    async fn delete_by_id(&self, activity_id: i32) -> Result<(), Status> {
        let res = entity::course::Entity::delete_by_id(activity_id)
            .exec(self)
            .await
            .map_err(|e| {
                error!("Failed to delete activity: {e}");

                Status::Internal(None)
            })?;

        if res.rows_affected == 0 {
            Err(Status::NotFound(None))
        } else {
            Ok(())
        }
    }
}
