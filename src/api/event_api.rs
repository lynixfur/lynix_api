use actix_web::{get, web::{Data, Json, Path, ServiceConfig}, Responder, HttpResponse};
use chrono::{Utc};
use mongodb::options::FindOptions;
use mongodb::{bson::doc, Client, Collection};
use bson::{from_bson, to_bson, Bson};
use futures::TryStreamExt;

use crate::{
    models::{event_model::Event},
};
use crate::models::error_model::ApiErrorType;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_all_events);
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