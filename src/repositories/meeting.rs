use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::models::{Course, meeting::Meeting, meeting::MeetingType};

use crate::app::status::AppStatus as Status;

use sea_orm::{ActiveModelTrait, IntoActiveModel};

use std::future::Future;

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiClaims {
    pub aud: String,
    pub iss: String,
    pub iat: usize,
    pub exp: usize,
    pub nbf: usize,
    pub sub: String,
    pub context: JitsiContext,
    pub room: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiContext {
    pub features: JitsiFeatures,
    pub user: JitsiUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiFeatures {
    pub livestreaming: bool,
    #[serde(rename = "outbound-call")]
    pub outbound_call: bool,
    #[serde(rename = "sip-outbound-call")]
    pub sip_outbound_call: bool,
    pub transcription: bool,
    pub recording: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiUser {
    #[serde(rename = "hidden-from-recorder")]
    pub hidden_from_recorder: bool,
    pub moderator: bool,
    pub name: String,
    pub id: String,
    pub avatar: String,
    pub email: String,
}

pub trait MeetingRepository {
    /// Create new meeting for the course
    ///
    /// # Arguments
    ///
    /// * 'course' - Course for which meeting should be started
    ///
    /// # Returns
    ///
    /// On success: Meeting model
    /// On failure: Application status
    fn create_course_meeting(
        &self,
        course: &Course,
    ) -> impl Future<Output = Result<Meeting, Status>> + Send;

    /// Terminate (stop) course meeting
    ///
    /// # Arguments
    ///
    /// * 'meeting' - Meeting that should be terminated
    /// * 'course' - Course for which meeting should be terminated
    ///
    /// # Returns
    ///
    /// On success: Nothing
    /// On failure: Application status
    fn terminate_course_meeting(
        &self,
        meeting: Meeting,
        course: &Course,
    ) -> impl Future<Output = Result<(), Status>> + Send;
}

impl MeetingRepository for sea_orm::DatabaseConnection {
    async fn create_course_meeting(&self, course: &Course) -> Result<Meeting, Status> {
        if course.is_running_meeting() {
            return Err(Status::AlreadyExists(Some(
                "This course is already running a meeting!".into(),
            )));
        }

        let mut course = course.model.clone().into_active_model();
        course.is_running_meeting = Set(true);

        let course = course.update(self).await.map_err(|e| {
            log::error!("Failed to start course meeting: {e}");

            Status::Internal(None)
        })?;

        Ok(Meeting::new(
            format!("course-{}", course.course_id).as_str(),
            7200,
            MeetingType::Course,
        ))
    }

    async fn terminate_course_meeting(
        &self,
        _meeting: Meeting,
        course: &Course,
    ) -> Result<(), Status> {
        if !course.is_running_meeting() {
            return Err(Status::AlreadyExists(Some(
                "Meeting is not running!".into(),
            )));
        }

        let mut course = course.model.clone().into_active_model();
        course.is_running_meeting = Set(false);

        course.update(self).await.map_err(|e| {
            log::error!("Failed to terminate course meeting: {e}");

            Status::Internal(None)
        })?;

        Ok(())
    }
}
