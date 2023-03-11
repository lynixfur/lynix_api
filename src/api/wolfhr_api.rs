use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, web, get, post};
use chrono::{SecondsFormat, Utc};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;
use crate::models::error_model::{ApiError, ApiErrorType};
use crate::models::wolfhr_model::WolfHR;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_wolfhr);
    cfg.service(post_wolfhr);
}

#[derive(Debug, Deserialize)]
struct WolfHrRequest {
    datatype: Option<String>
}

#[get("/wolfhr/{id}")]
pub async fn get_wolfhr(path: actix_web::web::Path<String>, info: web::Query<WolfHrRequest>, data: web::Data<AppState>) -> Result<impl Responder, ApiErrorType> {

    let user_data_result = sqlx::query_as::<_, WolfHR>(
        "SELECT * FROM wolf_hr WHERE str_id = $1",
    ).bind(path.into_inner()).fetch_optional(&data.db).await?;

    let user_data = match user_data_result {
        Some(data) => data,
        None => return Err(ApiErrorType::UserNotFound),
    };

    match &info.datatype {
        None => { Ok(HttpResponse::Ok().json(json!(user_data))) }
        Some(data) => {
            if data == "neos" {
                Ok(HttpResponse::Ok().body( format!("{},{}", user_data.wolf_hr, user_data.wolf_battery_percent)))
            } else {
                Ok(HttpResponse::Ok().json(json!(user_data)))
            }
        }
    }
}

#[post("/wolfhr/send_data")]
pub async fn post_wolfhr(data: web::Data<AppState>) -> Result<impl Responder, ApiErrorType> {
    // Check if the current user exists
    let user_data_result = sqlx::query_as::<_, WolfHR>(
        "SELECT * FROM wolf_hr WHERE str_id = $1",
    ).bind("test").fetch_optional(&data.db).await?;

    let user_data = match user_data_result {
        Some(_) => {
            // Update Row
            sqlx::query!(
                "UPDATE wolf_hr SET wolf_hr = $1, wolf_battery_percent = $2, last_updated = $3 WHERE str_id = $4",
                100,
                100,
                None,
                "test"
            ).execute(&data.db).await?;

            Ok(HttpResponse::Ok().body("Updated"))
        },
        None => {
            let new_user = WolfHR {
                id: uuid::Uuid::new_v4(),
                str_id: "test".to_string(),
                wolf_hr: 0,
                wolf_battery_percent: 0,
                last_updated: Some(Utc::now()),
            };

            sqlx::query!(
                "INSERT INTO wolf_hr (id, str_id, wolf_hr, wolf_battery_percent, last_updated) VALUES ($1, $2, $3, $4, $5)",
                new_user.id,
                new_user.str_id,
                new_user.wolf_hr,
                new_user.wolf_battery_percent,
                new_user.last_updated
            ).execute(&data.db).await?;

            Ok(HttpResponse::Ok().json(json!(new_user)))
        },
    };
}

