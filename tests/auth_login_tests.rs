//! Integration tests for daemon auth endpoints
//!
//! TDD: RED phase - Tests for /auth/login endpoint
//! Based on OpenAPI spec: docs/specs/openapi.yaml

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::post,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower::util::ServiceExt;

// --- Types matching OpenAPI spec ---

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    token_type: String,
}

#[derive(Debug, Serialize)]
struct AuthError {
    code: String,
    message: String,
}

// --- Handler for tests ---

async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthError>)> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthError {
                code: "INVALID_REQUEST".into(),
                message: "Username and password are required".into(),
            }),
        ));
    }

    // 🔴 RED: Return NOT_IMPLEMENTED until we implement authentication
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(AuthError {
            code: "NOT_IMPLEMENTED".into(),
            message: "Authentication not yet implemented".into(),
        }),
    ))
}

// --- Test helper ---

fn create_test_app() -> Router {
    Router::new().route("/auth/login", post(login))
}

// --- Tests (RED phase) ---
// These tests SHOULD FAIL because the endpoint is not implemented

#[tokio::test]
async fn login_valid_credentials_returns_200() {
    // 🔴 RED: This test SHOULD FAIL - endpoint returns NOT_IMPLEMENTED
    
    let app = create_test_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "username": "admin",
                    "password": "admin123"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Expected: 200 OK
    // Current: 501 NOT_IMPLEMENTED (this test will FAIL - RED phase)
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    // Verify AuthResponse structure (OpenAPI: AuthResponse)
    assert!(json.get("accessToken").is_some());
    assert!(json.get("refreshToken").is_some());
    assert!(json.get("expiresIn").is_some());
    assert_eq!(json.get("tokenType").unwrap(), "Bearer");
}

#[tokio::test]
async fn login_invalid_credentials_returns_401() {
    let app = create_test_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "username": "admin",
                    "password": "wrongpassword"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Expected: 401 UNAUTHORIZED
    // Current: 501 NOT_IMPLEMENTED (this test will FAIL - RED phase)
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_missing_password_returns_400() {
    let app = create_test_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "username": "admin"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // This test SHOULD PASS - handler returns BAD_REQUEST for missing fields
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn login_empty_body_returns_400() {
    let app = create_test_app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    // This test SHOULD PASS - handler returns BAD_REQUEST for empty fields
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
