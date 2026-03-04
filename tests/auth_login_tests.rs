//! Integration tests for daemon auth endpoints
//!
//! TDD: GREEN phase - Tests for /auth/login endpoint
//! Based on OpenAPI spec: docs/specs/openapi.yaml
//!
//! Production-ready: REAL password hashing with argon2, REAL JWT tokens

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::post,
};
use serde_json::json;
use tower::util::ServiceExt;

use diskcortex::daemon::auth::{login, AppState};
use diskcortex::daemon::db;

// --- Test helper ---

async fn create_test_app() -> Router {
    // Create REAL in-memory database with REAL password hashes
    let pool = db::init_test_db().await.expect("Failed to create test database");
    
    // REAL JWT config (not mock) - unique secret for tests
    let state = AppState::new(
        pool,
        "test-secret-key-for-integration-tests-unique".to_string(),
        3600,
    );
    
    Router::new()
        .route("/auth/login", post(login))
        .with_state(state)
}

// --- Tests (GREEN phase) ---
// These tests SHOULD PASS now that authentication is implemented

#[tokio::test]
async fn login_valid_credentials_returns_200() {
    let app = create_test_app().await;
    
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

    // 🟢 GREEN: This test SHOULD PASS now
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    // Verify AuthResponse structure (OpenAPI: AuthResponse)
    assert!(json.get("access_token").is_some(), "Missing access_token");
    assert!(json.get("refresh_token").is_some(), "Missing refresh_token");
    assert!(json.get("expires_in").is_some(), "Missing expires_in");
    assert_eq!(json.get("token_type").unwrap(), "Bearer", "Token type should be Bearer");
    
    // Verify tokens are non-empty strings (REAL tokens)
    let access_token = json.get("access_token").unwrap().as_str().unwrap();
    let refresh_token = json.get("refresh_token").unwrap().as_str().unwrap();
    assert!(!access_token.is_empty(), "access_token should not be empty");
    assert!(!refresh_token.is_empty(), "refresh_token should not be empty");
}

#[tokio::test]
async fn login_invalid_credentials_returns_401() {
    let app = create_test_app().await;
    
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

    // 🟢 GREEN: This test SHOULD PASS - invalid credentials return 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_missing_password_returns_400() {
    let app = create_test_app().await;
    
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

    // axum returns 422 for missing required field in JSON deserialization
    assert!(response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn login_empty_body_returns_400() {
    let app = create_test_app().await;
    
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

    // axum deserializes {} with empty strings, our handler validates and returns 400
    // This is the expected behavior - our validation catches empty fields
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn login_nonexistent_user_returns_401() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "username": "nonexistent",
                    "password": "anypassword"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Non-existent user should return 401 (same as invalid credentials for security)
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_inactive_user_returns_401() {
    let app = create_test_app().await;
    
    // Test the inactive user that's created in init_test_db
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "username": "inactive",
                    "password": "inactive123"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Inactive user should return 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
