use serde::Serialize;

#[derive(Serialize)]
pub struct ApiMeta {
    pub api_version: String,
    pub api_build_number: String,
    pub api_status: String
}