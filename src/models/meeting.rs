use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::time::{SystemTime, UNIX_EPOCH};

/// JSON-serializable representation that can be safely returned from the app routes
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(title = "Meeting", description = "Meeting object")]
pub struct MeetingRepresentation {
    /// Meeting Identifier
    pub meeting_id: String,
    /// User-specific URL that can be used to join meeting
    pub join_url: String,
    /// Authentication token (if required to join)
    pub auth_token: Option<String>,
    /// Meeting start time
    pub starts_at: String,
    /// Meeting end time
    pub expires_at: String,
}

/// Meeting type
pub enum MeetingType {
    Course,
}

/// Meeting model
pub struct Meeting {
    /// Meeting type
    pub meeting_type: MeetingType,

    /// Meeting identifier
    pub id: String,

    /// Meeting start time (UNIX)
    pub starts_at: u64,

    /// Meeting end time (UNIX)
    pub expires_at: u64,
}

impl Meeting {
    /// Create new meeting model
    ///
    /// # Arguments
    ///
    /// * 'id' - Meeting identifier
    /// * 'duration' - Meeting duration
    /// * 'meeting_type' - Meeting type
    ///
    /// # Returns
    ///
    /// Meeting model
    pub fn new(id: &str, duration: u64, meeting_type: MeetingType) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self::new_scheduled(id, now, duration, meeting_type)
    }

    /// Create new scheduled meeting
    ///
    /// # Arguments
    ///
    /// * 'id' - Meeting identifier
    /// * 'starts_at' - When meeting will start
    /// * 'duration' - Meeting duration
    /// * 'meeting_type' - Meeting type
    ///
    /// # Returns
    ///
    /// New shceduled meeting
    pub fn new_scheduled(
        id: &str,
        starts_at: u64,
        duration: u64,
        meeting_type: MeetingType,
    ) -> Self {
        Meeting {
            meeting_type,
            id: id.into(),
            starts_at,
            expires_at: starts_at + duration,
        }
    }
}
