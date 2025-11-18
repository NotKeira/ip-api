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

impl IpResponse {
    /// Convert to plain text format
    pub fn to_plain_text(&self) -> String {
        let rdns = self.rdns.as_deref().unwrap_or("null");
        let user_agent = self.user_agent.as_deref().unwrap_or("null");

        format!(
            "IP: {}\nrDNS: {}\nUser-Agent: {}\nUnix-Timestamp: {}\nUTC-Time: {}\nLocal-Time: {}",
            self.ip, rdns, user_agent, self.unix_timestamp, self.utc_time, self.local_time
        )
    }
}

/// Response format enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseFormat {
    Json,
    PlainText,
}
