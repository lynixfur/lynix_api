use actix_web::{get, web, web::Data, App, HttpServer, Responder};
use crate::models::{apimeta_model::ApiMeta};

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
