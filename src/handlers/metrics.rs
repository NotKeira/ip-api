use axum::{extract::State, http::StatusCode, response::Json};
use serde::Serialize;
use std::time::SystemTime;

/// Metrics response
#[derive(Serialize)]
pub struct MetricsResponse {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    uptime_seconds: u64,
    timestamp: u64,
}

lazy_static::lazy_static! {
    static ref START_TIME: SystemTime = SystemTime::now();
}

/// Handler for GET /metrics endpoint
pub async fn get_metrics(
    State(state): State<crate::AppState>,
) -> Result<Json<MetricsResponse>, StatusCode> {
    let now = SystemTime::now();
    let uptime = now
        .duration_since(*START_TIME)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs();

    let timestamp = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs();

    Ok(Json(MetricsResponse {
        total_requests: state.metrics.total(),
        successful_requests: state.metrics.success(),
        failed_requests: state.metrics.failure(),
        uptime_seconds: uptime,
        timestamp,
    }))
}
