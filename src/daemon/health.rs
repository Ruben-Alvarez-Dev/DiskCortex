use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub time: String,
    pub version: String,
}

pub async fn health(
    State(_pool): State<SqlitePool>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        time: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
