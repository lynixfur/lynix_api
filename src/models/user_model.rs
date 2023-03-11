use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use crate::models::fursuit_model::Fursuit;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SafeUser {
    pub id: uuid::Uuid,
    pub username: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String
}

