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
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

/// Create the daemon router with database state
pub fn create_app_with_db(db: SqlitePool) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(login))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db)
}
