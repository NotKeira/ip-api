//! Security utilities and middleware

use axum::http::HeaderValue;

/// Security headers to add to all responses
pub struct SecurityHeaders;

impl SecurityHeaders {
    /// Get X-Content-Type-Options header
    pub fn x_content_type_options() -> HeaderValue {
        HeaderValue::from_static("nosniff")
    }

    /// Get X-Frame-Options header
    pub fn x_frame_options() -> HeaderValue {
        HeaderValue::from_static("DENY")
    }

    /// Get X-XSS-Protection header
    pub fn x_xss_protection() -> HeaderValue {
        HeaderValue::from_static("1; mode=block")
    }

    /// Get Referrer-Policy header
    pub fn referrer_policy() -> HeaderValue {
        HeaderValue::from_static("strict-origin-when-cross-origin")
    }

    /// Get Content-Security-Policy header
    pub fn content_security_policy() -> HeaderValue {
        HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'")
    }

    /// Get Strict-Transport-Security header
    pub fn strict_transport_security() -> HeaderValue {
        HeaderValue::from_static("max-age=31536000; includeSubDomains")
    }
}

/// Validate IP address format
///
/// Returns true if the IP address is valid IPv4 or IPv6
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}

/// Sanitize IP address string
///
/// Removes any potentially dangerous characters and validates format
pub fn sanitize_ip(ip: &str) -> Option<String> {
    let trimmed = ip.trim();

    if is_valid_ip(trimmed) {
        Some(trimmed.to_string())
    } else {
        None
    }
}

/// Validate user agent string
///
/// Checks if user agent is within reasonable length and doesn't contain
/// potentially dangerous characters
pub fn is_valid_user_agent(user_agent: &str) -> bool {
    const MAX_USER_AGENT_LENGTH: usize = 512;

    if user_agent.len() > MAX_USER_AGENT_LENGTH {
        return false;
    }

    // Check for null bytes or other control characters
    !user_agent.chars().any(|c| c.is_control() && c != '\t')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ipv4() {
        assert!(is_valid_ip("192.168.1.1"));
        assert!(is_valid_ip("8.8.8.8"));
    }

    #[test]
    fn test_valid_ipv6() {
        assert!(is_valid_ip("::1"));
        assert!(is_valid_ip("2001:4860:4860::8888"));
    }

    #[test]
    fn test_invalid_ip() {
        assert!(!is_valid_ip("not.an.ip.address"));
        assert!(!is_valid_ip("256.256.256.256"));
        assert!(!is_valid_ip(""));
    }

    #[test]
    fn test_sanitize_ip() {
        assert_eq!(
            sanitize_ip("  192.168.1.1  "),
            Some("192.168.1.1".to_string())
        );
        assert_eq!(sanitize_ip("invalid"), None);
    }

    #[test]
    fn test_user_agent_validation() {
        assert!(is_valid_user_agent("Mozilla/5.0"));
        assert!(!is_valid_user_agent("Bad\0Agent"));
        assert!(!is_valid_user_agent(&"a".repeat(600)));
    }
}
