//! Integration tests for /auth/logout endpoint
//!
//! TDD: RED phase - Tests for /auth/logout endpoint
//! Based on OpenAPI spec: docs/specs/openapi.yaml
//!
//! OpenAPI spec:
//!   POST /auth/logout
//!   Summary: Logout current session
//!   Response: 204 No Content (logged out)
//!   Security: BearerAuth required

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::post,
};
use serde_json::json;
use tower::util::ServiceExt;

use diskcortex::daemon::auth::{login, logout, AppState};
use diskcortex::daemon::db;

// --- Test helper ---

async fn create_test_app() -> Router {
    let pool = db::init_test_db().await.expect("Failed to create test database");
    
    let state = AppState::new(
        pool,
        "test-secret-key-for-logout-tests".to_string(),
        3600,
    );
    
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .with_state(state)
}

/// Helper to get a valid access token
async fn get_access_token(app: &Router) -> String {
    let response = app
        .clone()
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
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    json.get("access_token").unwrap().as_str().unwrap().to_string()
}

// --- Tests (RED phase) ---
// These tests SHOULD FAIL initially - endpoint returns NOT_IMPLEMENTED

#[tokio::test]
async fn logout_with_valid_token_returns_204() {
    // 🔴 RED: This test SHOULD FAIL - endpoint not implemented
    
    let app = create_test_app().await;
    let token = get_access_token(&app).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/logout")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Expected: 204 NO_CONTENT
    // Current: 501 NOT_IMPLEMENTED (this test will FAIL - RED phase)
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn logout_without_token_returns_401() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/logout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Without Authorization header, should return 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_with_invalid_token_returns_401() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/logout")
                .header("Authorization", "Bearer invalid-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Invalid token should return 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_with_expired_token_returns_401() {
    let app = create_test_app().await;
    
    // Create an expired token (exp in the past)
    // For RED phase, we use an obviously invalid/expired format
    let expired_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0LXVzZXItaWQiLCJ1c2VybmFtZSI6ImFkbWluIiwicm9sZSI6ImFkbWluIiwiZXhwIjoxMDAwMDAwMDAwLCJpYXQiOjEwMDAwMDAwMDAsImp0aSI6InRlc3QtaWQifQ.invalid";
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/logout")
                .header("Authorization", format!("Bearer {}", expired_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Expired token should return 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
