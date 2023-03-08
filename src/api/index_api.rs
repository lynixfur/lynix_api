use actix_web::{get, web, web::Data, App, HttpServer, Responder};
use crate::models::{apimeta_model::ApiMeta};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
pub async fn index() -> impl Responder {
    let obj = ApiMeta {
        api_version: "v2.0.20230308_RUST".to_string(),
        api_build_number: "".to_string(),
        api_status: "Operational".to_string()
    };

    return web::Json(obj);
}
