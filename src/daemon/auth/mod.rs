//! Authentication module
//! JWT + session token authentication

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Login request payload (OpenAPI: LoginRequest)
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Authentication response (OpenAPI: AuthResponse)
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct AuthError {
    pub code: String,
    pub message: String,
}

/// POST /auth/login
pub async fn login(
    State(_db): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthError>)> {
    // Validate credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthError {
                code: "INVALID_REQUEST".to_string(),
                message: "Username and password are required".to_string(),
            }),
        ));
    }

    // TODO: Implement actual credential verification
    // For now, return 501 Not Implemented
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(AuthError {
            code: "NOT_IMPLEMENTED".to_string(),
            message: "Authentication not yet implemented".to_string(),
        }),
    ))
}
