use actix_web::web::Data;
use mongodb::{
    bson::doc,
    error::Error,
    results::{DeleteResult, UpdateResult},
    Client, Collection,
    options::FindOptions
};
use crate::models::event_model::Event;
use crate::models::user_model::User;

// Fetch all events from the database
pub async fn get_all_events(
    client: &Data<Client>,
) -> Result<Vec<Event>, Error> {
    let collection: Collection<User> = client
        .database("lynix")
        .collection("events");
    let options = FindOptions::default();
    let mut cursors = collection.find(None, options).await?;
    let mut users: Vec<Event> = Vec::new();
    Ok(users)
}