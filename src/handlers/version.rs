//! Version information endpoint handler

use axum::response::Json;
use serde::Serialize;

/// Version information response
#[derive(Serialize)]
pub struct VersionResponse {
    version: String,
    name: String,
    authors: Vec<String>,
    repository: String,
    rust_edition: String,
}

/// Handler for GET /version endpoint
///
/// Returns API version and build information
pub async fn get_version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        authors: vec!["xFanexx".to_string(), "NotKeira".to_string()],
        repository: "https://github.com/xFanexx/ip-api".to_string(),
        rust_edition: "2024".to_string(),
    })
}
