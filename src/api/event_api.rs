use actix_web::{get, web::{Data, Json, Path, ServiceConfig}, Responder};
use crate::models::{apimeta_model::ApiMeta, event_model::Event};
use chrono::{Utc};
use mongodb::Client;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_all_events);
}

#[get("/events")]
pub async fn get_all_events(client: Data<Client>) -> impl Responder {
    let mut event_names = vec!["The Big Adventure","Furnal Equniox 2023", "HackTheBox CTF", "Furality Sylva"];
    let mut events: Vec<Event> = Vec::new();

    for event in event_names.iter() {
        let e = Event {
            id: "".to_string(),
            event_name: event.to_string(),
            event_description: "".to_string(),
            event_location: "Toronto - ON, Canada".to_string(),
            event_start: Utc::now(),
            event_end: Utc::now(),
            event_type: "".to_string(),
            event_url: "".to_string(),
            event_image: "".to_string(),
            event_image_alt: "".to_string(),
            event_active: true,
            event_live: true,
            created_ts: Utc::now(),
            updated_ts: Utc::now(),
        };
        events.push(e)
    }

    // Return the vector as JSON
    return Json(events);
}