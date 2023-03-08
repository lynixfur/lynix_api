use log::error;
use mongodb::Client;
use std::borrow::ToOwned;
use std::env;

// MongoDB initialize function.
// Get DB connection url from environment file and connect.
pub async fn init() -> Client {
    println!("⚙️ Connecting to Database...");

    let uri = match env::var("MONGOURI") {
        Ok(uri) => uri,
        Err(_) => {
            println!("❌ Error loading env info for MongoDB connection");
            "Error loading env variables to connect to MongoDB".to_owned()
        }
    };

    // panic if not able to connect to DB.
    let client = Client::with_uri_str(uri)
        .await
        .expect("Error connecting to backend database");
    client
}