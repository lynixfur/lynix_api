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
    // Removed as this caused issues in the API
    let mut validation_sub_errs = vec![]; // Empty SubError
    Ok(HttpResponse::build(StatusCode::FORBIDDEN).json(ApiError {
        status: 403,
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
        message: "You don't have access to this endpoint.".to_string(),
        debug_message: Some("Ask Lynix on NeosVR for more information why this is occuring, mostlikely there was problems with the new version of the HR System.".to_string()),
        sub_errors: validation_sub_errs,
    }))
}

