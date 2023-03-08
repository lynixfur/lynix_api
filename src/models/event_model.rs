use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id")]
    pub id: String,
    pub event_name: String,
    pub event_description: String,
    pub event_location: String,
    pub event_start: DateTime<Utc>,
    pub event_end: DateTime<Utc>,
    pub event_type: String,
    pub event_url: String,
    pub event_image: String,
    pub event_image_alt: String,
    pub event_active: bool,
    pub event_live: bool,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_ts: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_ts: DateTime<Utc>,
}

