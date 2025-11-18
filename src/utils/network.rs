//! Network configuration utilities

/// Determine bind address based on port number
///
/// Port 7112 binds to `[::]` for IPv6 dual-stack support.
/// All other ports bind to 0.0.0.0 for IPv4 only.
pub fn get_bind_address(port: u16) -> String {
    if port == 7112 {
        format!("[::]:{}", port)
    } else {
        format!("0.0.0.0:{}", port)
    }
}
