
use actix_web::{get, post, web::{Data, Json, ServiceConfig}, Responder, HttpResponse};
use actix_web::http::StatusCode;
use chrono::{SecondsFormat, Utc};
use mongodb::options::{FindOptions, FindOneOptions};
use mongodb::{bson::doc, Client, Collection};
use bson::{from_bson, to_bson, Bson};
use futures::TryStreamExt;
use bson::oid::ObjectId;

use crate::{
    models::{event_model::Event},
};
use crate::models::error_model::{ApiError, ApiErrorType};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_all_events);
    cfg.service(add_event);
    cfg.service(get_event);
    cfg.service(delete_event);
}

#[get("/events")]
pub async fn get_all_events(client: Data<Client>) -> Result<impl Responder, ApiErrorType> {
    let collection: Collection<Event> = client.database("lynix").collection("events");
    let filter = doc! { "event_name": "Furnal Equinox 2023" };
    let find_options = FindOptions::default();
    let mut cursors = collection.find(None, find_options).await?;

    let mut events: Vec<Event> = Vec::new();

    let count = collection.count_documents(None, None).await?;
    println!("🔨 [DEBUG] Events Found ({})", count);



    while let Some(event) = cursors.try_next().await? {
        println!("🔨 [DEBUG] Loading Cursor for Event ({})", event.event_name);
        events.push(event);
    }


    println!("✅ Loaded Events Successfully!");
    Ok(Json(events))
}

#[get("/events/{id}")]
pub async fn get_event(client: Data<Client>, path: actix_web::web::Path<String>) -> Result<impl Responder, ApiErrorType> {
    // This is not functional as (cursor) returns None / null for json
    let collection: Collection<Event> = client.database("lynix").collection("events");
    let find_options = FindOneOptions::default();
    let obj_id = String::from(path.into_inner());
    let filter = doc! {"_id": ""};
    let mut cursor = collection.find_one(None,None).await?;
    println!("✅ Loaded Event Successfully!");
    Ok(Json(cursor))
}

#[post("/event/add")]
pub async fn add_event(client: Data<Client>) -> Result<impl Responder, ApiErrorType> {
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
pub async fn delete_event(client: Data<Client>) -> Result<impl Responder, ApiErrorType> {
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