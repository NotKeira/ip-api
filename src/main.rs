use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;

#[derive(Serialize)]
struct IpResponse {
    #[serde(rename = "IP")]
    ip: String,
    #[serde(rename = "rDNS")]
    rdns: Option<String>,
    #[serde(rename = "User-Agent")]
    user_agent: Option<String>,
    #[serde(rename = "Unix-Timestamp")]
    unix_timestamp: u64,
    #[serde(rename = "UTC-Time")]
    utc_time: String,
    #[serde(rename = "Local-Time")]
    local_time: String,
}

async fn get_ip_info(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Json<IpResponse>, StatusCode> {
    // Extract client IP from X-Forwarded-For or connection
    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    // Get User-Agent
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Reverse DNS lookup (non-blocking)
    let rdns = if let Ok(ip) = client_ip.parse::<IpAddr>() {
        tokio::task::spawn_blocking(move || {
            dns_lookup::lookup_addr(&ip).ok()
        })
        .await
        .ok()
        .flatten()
    } else {
        None
    };

    // Get current time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let unix_timestamp = now.as_secs();
    
    // Format times (using chrono for proper formatting)
    let datetime = chrono::DateTime::from_timestamp(unix_timestamp as i64, 0)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let utc_time = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let local_time = datetime.with_timezone(&chrono::Local)
        .format("%Y-%m-%d %H:%M:%S%Z")
        .to_string();

    Ok(Json(IpResponse {
        ip: client_ip,
        rdns,
        user_agent,
        unix_timestamp,
        utc_time,
        local_time,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let port = args
        .iter()
        .position(|arg| arg == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|p| p.parse::<u16>().ok())
        .expect("--port <PORT> required");

    // Bind address: IPv6 dual-stack for port 7112, IPv4 otherwise
    let bind_addr = if port == 7112 {
        format!("[::]:{}", port)
    } else {
        format!("0.0.0.0:{}", port)
    };

    // Build router
    let app = Router::new()
        .route("/", get(get_ip_info))
        .into_make_service_with_connect_info::<SocketAddr>();

    // Start server
    let listener = TcpListener::bind(&bind_addr).await?;
    println!("Listening on {}", bind_addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}
