//! IP API Server
//!
//! A lightweight HTTP API service that returns client IP information including
//! IP address, reverse DNS, user agent, and timestamp data. Supports both IPv4
//! and IPv6 through separate port configurations.

mod config;
mod handlers;
mod middleware;
mod models;
mod utils;

use axum::{middleware as axum_middleware, routing::get, Router};
use config::Config;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use utils::{cache::DnsCache, metrics::Metrics};

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    metrics: Arc<Metrics>,
    dns_cache: Arc<DnsCache>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    utils::logging::init_logging();

    tracing::info!("Starting IP API Server");

    // Load configuration
    let config = Config::from_env()?;

    tracing::info!(
        port = config.port,
        rate_limit = config.rate_limit_requests,
        "Configuration loaded"
    );

    // Determine bind address based on port
    let bind_addr = utils::network::get_bind_address(config.port);

    // Create rate limiter
    let rate_limiter = Arc::new(middleware::rate_limit::RateLimiter::new(
        config.rate_limit_requests,
        config.rate_limit_window(),
    ));

    // Create DNS cache
    let dns_cache = Arc::new(DnsCache::new(config.dns_cache_ttl()));

    // Create metrics collector
    let metrics = Arc::new(Metrics::new());

    // Create app state
    let app_state = AppState {
        metrics: metrics.clone(),
        dns_cache: dns_cache.clone(),
    };

    // Spawn cleanup task for rate limiter
    {
        let cleanup_limiter = rate_limiter.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
            loop {
                interval.tick().await;
                cleanup_limiter.cleanup().await;
                tracing::debug!("Rate limiter cleanup completed");
            }
        });
    }

    // Spawn cleanup task for DNS cache
    {
        let cleanup_cache = dns_cache.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
            loop {
                interval.tick().await;
                cleanup_cache.cleanup().await;
                let size = cleanup_cache.size().await;
                tracing::debug!(cache_size = size, "DNS cache cleanup completed");
            }
        });
    }

    // Build router with all routes and middleware
    let app = Router::new()
        .route("/", get(handlers::ip::get_ip_info))
        .route("/health", get(handlers::health::health_check))
        .route("/metrics", get(handlers::metrics::get_metrics))
        .route("/headers", get(handlers::headers::get_headers))
        .route("/version", get(handlers::version::get_version))
        .route("/lookup", get(handlers::lookup::lookup_ip))
        .with_state(app_state)
        .layer(axum_middleware::from_fn_with_state(
            metrics.clone(),
            middleware::metrics::metrics_middleware,
        ))
        .layer(axum_middleware::from_fn(middleware::logging::log_request))
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

    tracing::info!(
        bind_addr = %bind_addr,
        "Server listening"
    );

    println!("Listening on {}", bind_addr);
    println!("Endpoints:");
    println!("  GET /           - Client IP information");
    println!("  GET /health     - Health check");
    println!("  GET /metrics    - Usage statistics");
    println!("  GET /headers    - Request headers");
    println!("  GET /version    - API version");
    println!("  GET /lookup?ip= - Lookup any IP address");

    axum::serve(listener, app).await?;

    Ok(())
}
