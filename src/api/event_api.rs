
use actix_web::{get, post, web::{Data, Json, ServiceConfig}, Responder, HttpResponse, web};
use actix_web::http::StatusCode;
use chrono::{SecondsFormat, Utc};
use bson::{from_bson, to_bson, Bson};
use futures::TryStreamExt;
use bson::oid::ObjectId;
use serde_json::json;

use crate::{AppState, models::{event_model::Event}};
use crate::models::error_model::{ApiError, ApiErrorType};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_all_events);
    cfg.service(add_event);
    cfg.service(get_event);
    cfg.service(delete_event);
}

#[get("/events")]
pub async fn get_all_events(data: web::Data<AppState>) -> impl Responder {

    let query_result = sqlx::query_as::<_, Event>("SELECT * FROM events").fetch_all(&data.db).await;

    if query_result.is_err() {
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(ApiError {
            status: 500,
            time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
            message: "Critical Database Error".to_string(),
            debug_message: Some("Try again later.".to_string()),
            sub_errors: vec![],
        })
    }

    let events = query_result.unwrap();
    HttpResponse::Ok().json(json!(events))
}

#[get("/events/{id}")]
pub async fn get_event(path: actix_web::web::Path<String>) -> Result<impl Responder, ApiErrorType> {
    let mut validation_sub_errs = vec![]; // Empty SubError
    Ok(HttpResponse::build(StatusCode::FORBIDDEN).json(ApiError {
        status: 403,
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
        message: "You don't have access to this endpoint.".to_string(),
        debug_message: Some("You're possibly missing your API Token, Not Authenticated or this resource is blocked.".to_string()),
        sub_errors: validation_sub_errs,
    }))
}

#[post("/event/add")]
pub async fn add_event() -> Result<impl Responder, ApiErrorType> {
    // Requires Auth
    // Allows an event to be added by UI / CLI
    let mut validation_sub_errs = vec![]; // Empty SubError
    Ok(HttpResponse::build(StatusCode::FORBIDDEN).json(ApiError {
        status: 403,
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
        message: "You don't have access to this endpoint.".to_string(),
        debug_message: Some("You're possibly missing your API Token, Not Authenticated or this resource is blocked.".to_string()),
        sub_errors: validation_sub_errs,
    }))
}

#[post("/event/delete")]
pub async fn delete_event() -> Result<impl Responder, ApiErrorType> {
    // Requires Auth
    // Allows an event to be added by UI / CLI
    let mut validation_sub_errs = vec![]; // Empty SubError
    Ok(HttpResponse::build(StatusCode::FORBIDDEN).json(ApiError {
        status: 403,
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
        message: "You don't have access to this endpoint.".to_string(),
        debug_message: Some("You're possibly missing your API Token, Not Authenticated or this resource is blocked.".to_string()),
        sub_errors: validation_sub_errs,
    }))
}