//! Command line argument parsing

use std::env;

/// Parse the --port argument from command line
///
/// Returns an error if --port is not provided or invalid
pub fn parse_port() -> Result<u16, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    args.iter()
        .position(|arg| arg == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|p| p.parse::<u16>().ok())
        .ok_or_else(|| "Usage: ip-api --port <PORT>".into())
}
