//! DNS lookup utilities

use crate::utils::cache::DnsCache;
use std::net::IpAddr;
use std::sync::Arc;

/// Perform reverse DNS lookup for an IP address
///
/// Returns the hostname if lookup succeeds, None otherwise.
/// This operation is performed in a blocking task to avoid blocking
/// the async runtime.
pub async fn reverse_lookup(ip_str: &str) -> Option<String> {
    let ip: IpAddr = ip_str.parse().ok()?;

    tokio::task::spawn_blocking(move || dns_lookup::lookup_addr(&ip).ok())
        .await
        .ok()
        .flatten()
}

/// Perform reverse DNS lookup with caching
///
/// Checks cache first, performs lookup if not cached, and stores result
pub async fn reverse_lookup_cached(ip_str: &str, cache: Arc<DnsCache>) -> Option<String> {
    // Check cache first
    if let Some(cached) = cache.get(ip_str).await {
        return cached;
    }

    // Perform lookup
    let result = reverse_lookup(ip_str).await;

    // Store in cache
    cache.insert(ip_str.to_string(), result.clone()).await;

    result
}
