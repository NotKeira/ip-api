//! Request timeout middleware

use axum::{body::Body, http::Request, middleware::Next, response::Response};
use std::time::Duration;
use tokio::time::timeout;

/// Middleware to enforce request timeouts
pub async fn timeout_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // 30 second timeout for requests
    match timeout(Duration::from_secs(30), next.run(request)).await {
        Ok(response) => Ok(response),
        Err(_) => Err(axum::http::StatusCode::REQUEST_TIMEOUT),
    }
}
