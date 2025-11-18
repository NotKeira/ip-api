//! DNS lookup utilities

use std::net::IpAddr;

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
