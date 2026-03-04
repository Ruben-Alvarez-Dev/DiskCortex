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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize database
    let db = db::init_db().await?;
    
    // Build router
    let app = Router::new()
        .route("/health", get(health::health))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db);
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 7331));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("DiskCortex daemon listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
