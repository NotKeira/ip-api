//! IP API Server
//!
//! A lightweight HTTP API service that returns client IP information including
//! IP address, reverse DNS, user agent, and timestamp data. Supports both IPv4
//! and IPv6 through separate port configurations.

mod handlers;
mod middleware;
mod models;
mod utils;
mod config;

use axum::{middleware as axum_middleware, routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let port = utils::cli::parse_port()?;

    // Determine bind address based on port
    let bind_addr = utils::network::get_bind_address(port);

    // Create rate limiter: 60 requests per minute per IP
    let rate_limiter = Arc::new(middleware::rate_limit::RateLimiter::new(
        60,
        Duration::from_secs(60),
    ));

    // Spawn cleanup task for rate limiter
    let cleanup_limiter = rate_limiter.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            cleanup_limiter.cleanup().await;
        }
    });

    // Build router with all routes and middleware
    let app = Router::new()
        .route("/", get(handlers::ip::get_ip_info))
        .layer(axum_middleware::from_fn(
            middleware::security_headers::add_security_headers,
        ))
        .layer(axum_middleware::from_fn(move |req, next| {
            let limiter = rate_limiter.clone();
            middleware::rate_limit::rate_limit_middleware(limiter, req, next)
        }))
        .into_make_service_with_connect_info::<SocketAddr>();

    // Start server
    let listener = TcpListener::bind(&bind_addr).await?;
    println!("Listening on {}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
