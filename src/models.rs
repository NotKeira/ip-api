//! Data models for API responses

use serde::Serialize;

/// Response structure containing client IP information
#[derive(Serialize, Debug)]
pub struct IpResponse {
    /// Client IP address (IPv4 or IPv6)
    #[serde(rename = "IP")]
    pub ip: String,

    /// Reverse DNS hostname (null if lookup fails)
    #[serde(rename = "rDNS")]
    pub rdns: Option<String>,

    /// User agent string from HTTP headers
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,

    /// Unix timestamp in seconds
    #[serde(rename = "Unix-Timestamp")]
    pub unix_timestamp: u64,

    /// UTC time formatted as YYYY-MM-DD HH:MM:SS UTC
    #[serde(rename = "UTC-Time")]
    pub utc_time: String,

    /// Local server time with timezone
    #[serde(rename = "Local-Time")]
    pub local_time: String,
}
