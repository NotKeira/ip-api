//! Logging configuration

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize structured logging
pub fn init_logging() {
    // Default to INFO level, allow override with RUST_LOG env var
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Check if JSON logging is requested
    let use_json = std::env::var("LOG_FORMAT")
        .map(|v| v.to_lowercase() == "json")
        .unwrap_or(false);

    if use_json {
        // JSON formatted logs
        tracing_subscriber::registry().with(env_filter).with(tracing_subscriber::fmt::layer().json()).init();
    } else {
        // Human-readable logs
        tracing_subscriber::registry().with(env_filter).with(tracing_subscriber::fmt::layer().pretty()).init();
    }
}