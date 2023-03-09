use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub event_name: String,
    pub event_description: String,
    pub event_location: String,
    pub event_start: DateTime<Utc>,
    pub event_end: DateTime<Utc>,
    pub event_type: String,
    pub event_url: String,
    pub event_image: String,
    pub event_image_alt: String,
    pub event_status: String, // This must be checked to make sure the statuses are the valid value keys: active, ended, live, upcoming, confirmed, canceled etc...
    pub event_active: bool,
    pub event_live: bool,
    pub created_ts: DateTime<Utc>,
    pub updated_ts: DateTime<Utc>,
}

