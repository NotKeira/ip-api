//! Configuration management

use std::time::Duration;

/// Application configuration
#[derive(Clone, Debug)]
pub struct Config {
    /// Port to bind to
    pub port: u16,

    /// Rate limit: max requests per window
    pub rate_limit_requests: usize,

    /// Rate limit: time window in seconds
    pub rate_limit_window_secs: u64,

    /// DNS cache TTL in seconds
    pub dns_cache_ttl_secs: u64,

    /// Request timeout in seconds
    pub request_timeout_secs: u64,
}

impl Config {
    /// Load configuration from environment variables and command line args
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Port from command line (required)
        let port = crate::utils::cli::parse_port()?;

        // Rate limiting configuration
        let rate_limit_requests = std::env::var("RATE_LIMIT_REQUESTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        let rate_limit_window_secs = std::env::var("RATE_LIMIT_WINDOW_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        // DNS cache TTL
        let dns_cache_ttl_secs = std::env::var("DNS_CACHE_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300); // 5 minutes default

        // Request timeout
        let request_timeout_secs = std::env::var("REQUEST_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Ok(Config {
            port,
            rate_limit_requests,
            rate_limit_window_secs,
            dns_cache_ttl_secs,
            request_timeout_secs,
        })
    }

    /// Get rate limit window as Duration
    pub fn rate_limit_window(&self) -> Duration {
        Duration::from_secs(self.rate_limit_window_secs)
    }

    /// Get DNS cache TTL as Duration
    pub fn dns_cache_ttl(&self) -> Duration {
        Duration::from_secs(self.dns_cache_ttl_secs)
    }

    /// Get request timeout as Duration
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_secs)
    }
}
