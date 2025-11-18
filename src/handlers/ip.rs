//! IP information endpoint handler

use crate::models::{IpResponse, ResponseFormat};
use crate::utils::{dns, security, time};
use axum::{
    extract::{ConnectInfo, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::net::SocketAddr;

/// Query parameters for IP endpoint
#[derive(Deserialize)]
pub struct IpQuery {
    format: Option<String>,
}

/// Handler for GET / endpoint
///
/// Extracts client IP from connection or X-Forwarded-For header,
/// performs reverse DNS lookup, and returns comprehensive client information.
pub async fn get_ip_info(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<crate::AppState>,
    headers: HeaderMap,
    Query(query): Query<IpQuery>,
) -> Result<Response, StatusCode> {
    // Determine response format from query param or Accept header
    let format = determine_format(&query, &headers);

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

    // Perform reverse DNS lookup (non-blocking, with cache)
    let rdns = dns::reverse_lookup_cached(&client_ip, state.dns_cache.clone()).await;

    // Get current timestamps in various formats
    let (unix_timestamp, utc_time, local_time) = time::get_timestamps()?;

    let response = IpResponse {
        ip: client_ip,
        rdns,
        user_agent,
        unix_timestamp,
        utc_time,
        local_time,
    };

    // Return response in requested format
    Ok(match format {
        ResponseFormat::Json => axum::Json(response).into_response(),
        ResponseFormat::PlainText => (
            [(
                axum::http::header::CONTENT_TYPE,
                "text/plain; charset=utf-8",
            )],
            response.to_plain_text(),
        )
            .into_response(),
    })
}

/// Determine response format from query parameter or Accept header
fn determine_format(query: &IpQuery, headers: &HeaderMap) -> ResponseFormat {
    // Check query parameter first
    if let Some(ref fmt) = query.format {
        return match fmt.to_lowercase().as_str() {
            "text" | "plain" | "txt" => ResponseFormat::PlainText,
            _ => ResponseFormat::Json,
        };
    }

    // Check Accept header
    if let Some(accept) = headers.get("accept") {
        if let Ok(accept_str) = accept.to_str() {
            if accept_str.contains("text/plain") {
                return ResponseFormat::PlainText;
            }
        }
    }

    // Default to JSON
    ResponseFormat::Json
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
