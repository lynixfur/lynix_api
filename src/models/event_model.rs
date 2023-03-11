use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub event_name: String,
    pub event_description: String,
    pub event_image: Option<String>,
    pub event_location: Option<String>,
    pub event_started: bool,
    pub event_ended: bool,
    pub event_canceled: bool,
    pub event_live: bool,
    pub event_status: String,
    pub event_start: DateTime<Utc>,
    pub event_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>
}

