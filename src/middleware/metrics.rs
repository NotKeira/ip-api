//! Metrics collection middleware

use crate::utils::metrics::Metrics;
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Middleware to track request metrics
pub async fn metrics_middleware(
    State(metrics): State<Arc<Metrics>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    metrics.increment_total();

    let response = next.run(request).await;

    if response.status().is_success() {
        metrics.increment_success();
    } else {
        metrics.increment_failure();
    }

    response
}
