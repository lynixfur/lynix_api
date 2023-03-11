use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WolfHR {
    pub id: uuid::Uuid,
    pub str_id: String,
    pub wolf_hr: i32,
    pub wolf_battery_percent: i32,
    pub last_updated: Option<DateTime<Utc>>
}