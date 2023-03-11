
use actix_web::{get, post, web::{Data, Json, ServiceConfig}, Responder, HttpResponse, web};
use actix_web::http::StatusCode;
use chrono::{SecondsFormat, Utc};
use bson::{from_bson, to_bson, Bson};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::{AppState, models::{event_model::Event}};
use crate::models::error_model::{ApiError, ApiErrorType};
use crate::models::user_model::{AuthUser, SafeUser, User};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthUserBody {
    username: String,
    password: String,
}

#[post("/auth/login")]
pub async fn login(data: web::Data<AppState>, body: Json<AuthUserBody>) -> Result<impl Responder, ApiErrorType> {
    let user: AuthUserBody = body.into_inner();
    let password = user.password.as_bytes();

    let db_user = sqlx::query_as::<_, AuthUser>(
        "SELECT id, username, password FROM users WHERE username = $1",

    ).bind(user.username).fetch_one(&data.db).await?;

    let db_password = PasswordHash::new(&db_user.password)?;

    if Argon2::default().verify_password(password, &db_password).is_ok() == true {
        Ok(HttpResponse::Ok().body("authenticated!"))
    } else {
        Err(ApiErrorType::InvalidAuthAttempt)
    }
}

#[post("/auth/register")]
pub async fn register(data: web::Data<AppState>, body: Json<CreateUser>) -> Result<impl Responder, ApiErrorType> {

    let user: CreateUser = body.into_inner();

    // Argon2 Hashing
    let password = user.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();


    sqlx::query_as::<_, SafeUser>(
        "INSERT INTO users (id, username, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username",
    )
        .bind(uuid::Uuid::new_v4())
        .bind(user.username)
        .bind(user.email)
        .bind(password_hash)
        .fetch_one(&data.db)
        .await;

    Ok(HttpResponse::Ok().body("{\"status\":\"ok\"}"))
}