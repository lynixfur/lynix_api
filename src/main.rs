use actix_web::{get, web, web::Data, App, HttpServer, Responder};
use log::{info, warn};
use dotenv::dotenv;
use std::env;
use serde::Serialize;
use pretty_env_logger;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use actix_web::middleware::Logger;
use pretty_env_logger::env_logger;
use pretty_env_logger::env_logger::Env;
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};

mod api;
mod models;
mod config;
mod middleware;

pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize)]
struct ErrorStatus {
    status: String,
    msg: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("⚙️ LynixAPI is Initializing...");

    // Load Env File
    dotenv().ok();

    // Get Host Settings
    let server_host = match env::var("SERVER_HOST") {
        Ok(v) => v.to_string(),
        Err(_) => "127.0.0.1".to_string(),
    };

    let server_port: u16 = match env::var("SERVER_PORT") {
        Ok(v) => v.parse().unwrap_or(8080),
        Err(_) => 8080,
    };

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("✨ LynixAPI Started {}:{}", server_host, server_port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(api::index_api::init)
            .configure(api::event_api::init)
            .configure(api::fursuit_api::init)
            .configure(api::auth_api::init)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .default_service(web::to(|| api::index_api::notfound()))
    })
        .bind((server_host, server_port))?
        .run()
        .await
}