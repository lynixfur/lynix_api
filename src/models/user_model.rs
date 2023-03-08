use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub active: bool,
    pub suspended: bool,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_ts: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_ts: DateTime<Utc>,
}
