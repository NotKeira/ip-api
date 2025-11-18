//! IP information endpoint handler

use crate::models::IpResponse;
use crate::utils::{dns, security, time};
use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    response::Json,
};
use std::net::SocketAddr;

/// Handler for GET / endpoint
///
/// Extracts client IP from connection or X-Forwarded-For header,
/// performs reverse DNS lookup, and returns comprehensive client information.
pub async fn get_ip_info(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Json<IpResponse>, StatusCode> {
    // Extract client IP from X-Forwarded-For or direct connection
    let client_ip = extract_client_ip(&headers, addr);

    // Validate and sanitize IP address
    let client_ip = security::sanitize_ip(&client_ip).ok_or(StatusCode::BAD_REQUEST)?;

    // Get user agent from headers
    let user_agent = extract_user_agent(&headers);

    // Validate user agent if present
    if let Some(ref ua) = user_agent {
        if !security::is_valid_user_agent(ua) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Perform reverse DNS lookup (non-blocking)
    let rdns = dns::reverse_lookup(&client_ip).await;

    // Get current timestamps in various formats
    let (unix_timestamp, utc_time, local_time) = time::get_timestamps()?;

    Ok(Json(IpResponse {
        ip: client_ip,
        rdns,
        user_agent,
        unix_timestamp,
        utc_time,
        local_time,
    }))
}

/// Extract client IP from X-Forwarded-For header or direct connection
fn extract_client_ip(headers: &HeaderMap, addr: SocketAddr) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

/// Extract user agent from headers
fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}
