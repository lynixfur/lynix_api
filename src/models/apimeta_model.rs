use serde::Serialize;

#[derive(Serialize)]
pub struct ApiMeta {
    pub api_version: String,
}