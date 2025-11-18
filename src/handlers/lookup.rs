use crate::models::IpResponse;
use crate::utils::{dns, security, time};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;

/// Query parameters for IP lookup
#[derive(Deserialize)]
pub struct LookupQuery {
    ip: String,
}

/// Handler for GET /lookup endpoint
pub async fn lookup_ip(
    State(state): State<crate::AppState>,
    Query(query): Query<LookupQuery>,
) -> Result<Json<IpResponse>, StatusCode> {
    let ip = security::sanitize_ip(&query.ip).ok_or(StatusCode::BAD_REQUEST)?;
    let rdns = dns::reverse_lookup_cached(&ip, state.dns_cache).await;
    let (unix_timestamp, utc_time, local_time) = time::get_timestamps()?;

    Ok(Json(IpResponse {
        ip,
        rdns,
        user_agent: None,
        unix_timestamp,
        utc_time,
        local_time,
    }))
}
