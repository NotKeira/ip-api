//! Request headers endpoint handler

use axum::{http::HeaderMap, response::Json};
use serde::Serialize;
use std::collections::HashMap;

/// Headers response
#[derive(Serialize)]
pub struct HeadersResponse {
    headers: HashMap<String, String>,
}

/// Handler for GET /headers endpoint
///
/// Returns all request headers for debugging purposes
pub async fn get_headers(headers: HeaderMap) -> Json<HeadersResponse> {
    let mut headers_map = HashMap::new();

    for (name, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            headers_map.insert(name.to_string(), value_str.to_string());
        }
    }

    Json(HeadersResponse {
        headers: headers_map,
    })
}
