//! Security headers middleware

use crate::utils::security::SecurityHeaders;
use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::IntoResponse,
};

/// Middleware to add security headers to all responses
pub async fn add_security_headers(request: Request<Body>, next: Next) -> impl IntoResponse {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    headers.insert(
        "x-content-type-options",
        SecurityHeaders::x_content_type_options(),
    );
    headers.insert("x-frame-options", SecurityHeaders::x_frame_options());
    headers.insert("x-xss-protection", SecurityHeaders::x_xss_protection());
    headers.insert("referrer-policy", SecurityHeaders::referrer_policy());
    headers.insert(
        "content-security-policy",
        SecurityHeaders::content_security_policy(),
    );

    response
}
