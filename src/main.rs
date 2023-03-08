use serde::Serialize;
use actix_web::{get, web, App, HttpServer, Responder};

#[derive(Serialize)]
struct ApiObject {
    api_version: String,
    api_status: String
}

#[derive(Serialize)]
struct ErrorStatus {
    status: String,
    msg: String
}

#[get("/")]
async fn index() -> impl Responder {
    let obj = ApiObject {
        api_version: "v2.0.20230308_RUST".to_string(),
        api_status: "Operational".to_string()
    };

    return web::Json(obj);
}

#[get("/db_test")]
async fn db_test() -> impl Responder {
    let obj = ErrorStatus {
        status: "error".to_string(),
        msg: "Database connection failed".to_string()
    };

    return web::Json(obj);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("✨ LynixAPI Started on 127.0.0.1:{}", 8557);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(db_test)
    })
        .bind(("127.0.0.1", 8557))?
        .run()
        .await
}