//! Simple rate limiting middleware

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limit state
#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<HashMap<IpAddr, RateLimitEntry>>>,
    max_requests: usize,
    window: Duration,
}

struct RateLimitEntry {
    count: usize,
    window_start: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `max_requests` - Maximum requests per window
    /// * `window` - Time window duration
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    /// Check if request is allowed
    async fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        let entry = state.entry(ip).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) > self.window {
            entry.count = 0;
            entry.window_start = now;
        }

        entry.count += 1;

        entry.count <= self.max_requests
    }

    /// Cleanup old entries periodically
    pub async fn cleanup(&self) {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        state.retain(|_, entry| now.duration_since(entry.window_start) <= self.window);
    }
}

/// Middleware function for rate limiting
pub async fn rate_limit_middleware(
    limiter: Arc<RateLimiter>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Extract IP from connection or X-Forwarded-For
    let ip = extract_ip(&request);

    if let Some(ip_addr) = ip {
        if !limiter.check_rate_limit(ip_addr).await {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded. Please try again later.",
            )
                .into_response();
        }
    }

    next.run(request).await
}

/// Extract IP address from request
fn extract_ip(request: &Request<Body>) -> Option<IpAddr> {
    // Try X-Forwarded-For first
    if let Some(forwarded) = request.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse() {
                    return Some(ip);
                }
            }
        }
    }

    // Fall back to connection info if available
    None
}
