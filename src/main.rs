use serde::Serialize;
use actix_web::{get, web, web::Data, App, HttpServer, Responder};
use log::{info, warn};
use dotenv::dotenv;
use crate::config::db;
use mongodb::Client;
use std::env;

mod models;
mod config;

#[derive(Serialize)]
struct ApiObject {
    api_version: String,
    api_build_number: String,
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
        api_build_number: "".to_string(),
        api_status: "Operational".to_string()
    };

    return web::Json(obj);
}

#[get("/events")]
async fn events(client: Data<Client>) -> impl Responder {
    let obj = ErrorStatus {
        status: "error".to_string(),
        msg: "Can't find any events.".to_string()
    };

    return web::Json(obj);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("⚙️ LynixAPI is Initializing...");

    // Load Env File
    dotenv().ok();

    // Initialize MongoDB Connection
    let client = db::init().await;

    // Get Host Settings
    let server_host = match env::var("SERVER_HOST") {
        Ok(v) => v.to_string(),
        Err(_) => "127.0.0.1".to_string(),
    };

    let server_port: u16 = match env::var("SERVER_PORT") {
        Ok(v) => v.parse().unwrap_or(8080),
        Err(_) => 8080,
    };

    println!("✨ LynixAPI Started {}:{}", server_host, server_port);
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(events) // Events Service System
            .app_data(Data::new(client.clone()))
    })
        .bind(("127.0.0.1", 8557))?
        .run()
        .await
}