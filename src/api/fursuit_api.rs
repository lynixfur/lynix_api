use actix_web::{get, web, web::Data, App, HttpServer, Responder, http::StatusCode, HttpResponse};
use crate::models::{apimeta_model::ApiMeta};
use crate::models::error_model::{ApiError, ApiErrorType};
use chrono::{SecondsFormat, Utc};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/fursuit/get_data")]
pub async fn index() -> Result<impl Responder, ApiErrorType> {
    // Requires Auth
    // Allows an event to be added by UI / CLI
    let mut validation_sub_errs = vec![]; // Empty SubError
    Ok(HttpResponse::build(StatusCode::FORBIDDEN).json(ApiError {
    status: 403,
    time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
    message: "You don't have access to this endpoint.".to_string(),
    debug_message: Some("This resource is only available lynix at the current moment, this feature may come to other fursuiters in the future! Stay tuned for announcements for when the FurSystem releases!".to_string()),
    sub_errors: validation_sub_errs,
    }))
}