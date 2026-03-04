//! DiskCortex Daemon - Local HTTP API server
//!
//! Runs on localhost:7331 and provides REST API for the GUI/CLI.

mod db;
mod health;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// Initialize tracing subscriber for structured logging
fn init_tracing() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .pretty()
        .init();
    
    info!("Tracing initialized");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging
    init_tracing();
    
    // Initialize database
    let db = db::init_db().await?;
    info!(database = "sqlite", "Database initialized");
    
    // Build router
    let app = Router::new()
        .route("/health", get(health::health))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db);
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 7331));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(address = %addr, "DiskCortex daemon started");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
