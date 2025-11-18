//! Health check endpoint handler

use axum::{http::StatusCode, response::Json};
use serde::Serialize;
use std::time::SystemTime;

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: u64,
    uptime_seconds: u64,
}

lazy_static::lazy_static! {
    static ref START_TIME: SystemTime = SystemTime::now();
}

/// Handler for GET /health endpoint
///
/// Returns basic health status and uptime information
pub async fn health_check() -> Result<Json<HealthResponse>, StatusCode> {
    let now = SystemTime::now();
    let uptime = now
        .duration_since(*START_TIME)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs();

    let timestamp = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs();

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp,
        uptime_seconds: uptime,
    }))
}
