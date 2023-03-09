use actix_web::{get, web, web::Data, App, HttpServer, Responder, http::StatusCode, HttpResponse};
use crate::models::{apimeta_model::ApiMeta};
use crate::models::error_model::{ApiError, ApiErrorType};
use chrono::{SecondsFormat, Utc};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
pub async fn index() -> impl Responder {

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let obj = ApiMeta {
        api_version: VERSION.to_string(),
    };

    return web::Json(obj);
}

pub async fn notfound() -> Result<HttpResponse, ApiErrorType> {
    let mut validation_sub_errs = vec![]; // Empty SubError

    Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(ApiError {
        status: 404,
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
        message: "The endpoint was not found by the API.".to_string(),
        debug_message: Some("This endpoint was possibly removed or moved, check API documentation!".to_string()),
        sub_errors: validation_sub_errs,
    }))
}
