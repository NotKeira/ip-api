//! Request timeout middleware

use axum::{body::Body, http::Request, middleware::Next, response::Response};
use std::time::Duration;
use tokio::time::timeout;

/// Middleware to enforce request timeouts
///
/// # Arguments
/// * `request` - The incoming request
/// * `next` - The next middleware/handler
/// * `duration` - Timeout duration
pub async fn timeout_middleware(
    request: Request<Body>,
    next: Next,
    duration: Duration,
) -> Result<Response, axum::http::StatusCode> {
    match timeout(duration, next.run(request)).await {
        Ok(response) => Ok(response),
        Err(_) => {
            tracing::warn!("Request timed out after {:?}", duration);
            Err(axum::http::StatusCode::REQUEST_TIMEOUT)
        }
    }
}
