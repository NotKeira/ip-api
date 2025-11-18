//! IP API Server
//!
//! A lightweight HTTP API service that returns client IP information including
//! IP address, reverse DNS, user agent, and timestamp data. Supports both IPv4
//! and IPv6 through separate port configurations.

mod handlers;
mod models;
mod utils;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let port = utils::cli::parse_port()?;

    // Determine bind address based on port
    let bind_addr = utils::network::get_bind_address(port);

    // Build router with all routes
    let app = Router::new()
        .route("/", get(handlers::ip::get_ip_info))
        .into_make_service_with_connect_info::<SocketAddr>();

    // Start server
    let listener = TcpListener::bind(&bind_addr).await?;
    println!("Listening on {}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
