//! Daemon modules

pub mod auth;
pub mod db;
pub mod health;

pub use auth::*;
pub use db::*;
pub use health::*;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

/// Create the daemon router with application state
pub fn create_app(state: auth::AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(auth::login))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(state)
}
