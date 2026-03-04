//! Authentication module
//! JWT + session token authentication
//!
//! GREEN Phase implementation: Real authentication with argon2 + JWT

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

/// Login request payload (OpenAPI: LoginRequest)
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
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

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub username: String,   // Username
    pub role: String,       // User role
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub jti: String,        // JWT ID (unique identifier)
}

/// User database model
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub active: i32,
}

/// Application state combining database and JWT config
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
    pub jwt_expires_in: i64,
}

impl AppState {
    pub fn new(db: SqlitePool, jwt_secret: String, jwt_expires_in: i64) -> Self {
        Self {
            db,
            jwt_secret,
            jwt_expires_in,
        }
    }
}

/// POST /auth/login
///
/// Authenticates a user with username and password.
/// Returns JWT access and refresh tokens on success.
#[instrument(skip(state), fields(username = %payload.username))]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthError>)> {
    info!("Login attempt for user: {}", payload.username);
    
    // Validate input
    if payload.username.is_empty() || payload.password.is_empty() {
        debug!("Empty username or password");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthError {
                code: "INVALID_REQUEST".to_string(),
                message: "Username and password are required".to_string(),
            }),
        ));
    }

    // Query user from database
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, role, active FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await;

    match user {
        Ok(Some(user)) => {
            // Check if user is active
            if user.active == 0 {
                debug!("User {} is inactive", payload.username);
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(AuthError {
                        code: "USER_INACTIVE".to_string(),
                        message: "User account is disabled".to_string(),
                    }),
                ));
            }

            // Verify password with argon2
            let parsed_hash = match PasswordHash::new(&user.password_hash) {
                Ok(h) => h,
                Err(e) => {
                    error!("Invalid password hash format: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(AuthError {
                            code: "INTERNAL_ERROR".to_string(),
                            message: "Authentication error".to_string(),
                        }),
                    ));
                }
            };

            let argon2 = Argon2::default();
            
            if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
                debug!("Invalid password for user {}", payload.username);
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(AuthError {
                        code: "INVALID_CREDENTIALS".to_string(),
                        message: "Invalid username or password".to_string(),
                    }),
                ));
            }

            // Generate tokens
            let now = Utc::now();
            let expires_in = state.jwt_expires_in;
            
            // Access token (short-lived)
            let access_claims = Claims {
                sub: user.id.clone(),
                username: user.username.clone(),
                role: user.role.clone(),
                exp: (now + Duration::seconds(expires_in)).timestamp(),
                iat: now.timestamp(),
                jti: Uuid::new_v4().to_string(),
            };

            let access_token = match encode(
                &Header::default(),
                &access_claims,
                &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
            ) {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to generate access token: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(AuthError {
                            code: "TOKEN_GENERATION_FAILED".to_string(),
                            message: "Failed to generate authentication token".to_string(),
                        }),
                    ));
                }
            };

            // Refresh token (long-lived - 7 days)
            let refresh_claims = Claims {
                sub: user.id.clone(),
                username: user.username.clone(),
                role: user.role.clone(),
                exp: (now + Duration::days(7)).timestamp(),
                iat: now.timestamp(),
                jti: Uuid::new_v4().to_string(),
            };

            let refresh_token = match encode(
                &Header::default(),
                &refresh_claims,
                &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
            ) {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to generate refresh token: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(AuthError {
                            code: "TOKEN_GENERATION_FAILED".to_string(),
                            message: "Failed to generate authentication token".to_string(),
                        }),
                    ));
                }
            };

            info!("User {} authenticated successfully", payload.username);
            
            Ok(Json(AuthResponse {
                access_token,
                refresh_token,
                expires_in,
                token_type: "Bearer".to_string(),
            }))
        }
        Ok(None) => {
            debug!("User {} not found", payload.username);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    code: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid username or password".to_string(),
                }),
            ))
        }
        Err(e) => {
            error!("Database error during login: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthError {
                    code: "DATABASE_ERROR".to_string(),
                    message: "An error occurred while processing the request".to_string(),
                }),
            ))
        }
    }
}

/// Hash a password using argon2
/// Used during user creation/password reset
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash password: {}", e))?
        .to_string();
    Ok(password_hash)
}

/// POST /auth/logout
///
/// Logs out the current user session.
/// Invalidates the access token (delete from blacklist if implemented)
/// Returns 204 No Content on success.
#[instrument(skip(state))]
pub async fn logout(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<(), (StatusCode, Json<AuthError>)> {
    info!("Logout request");
    
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    code: "MISSING_AUTH_HEADER".to_string(),
                    message: "Authorization header is required".to_string(),
                }),
            )
        })?
        .to_str()
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    code: "INVALID_AUTH_HEADER".to_string(),
                    message: "Authorization header must be valid UTF-8".to_string(),
                }),
            )
        })?;
    
    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                code: "INVALID_AUTH_HEADER".to_string(),
                message: "Authorization header must start with 'Bearer '".to_string(),
            }),
        ));
    }
    
    let token = auth_header[7..].trim();
    if token.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                code: "EMPTY_TOKEN".to_string(),
                message: "Token cannot be empty".to_string(),
            }),
        ));
    }
    
    // Decode and validate JWT token
    let claims: Claims = match decode_token(token, &state.jwt_secret) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::warn!("Invalid token during logout: {}", e);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    code: "INVALID_TOKEN".to_string(),
                    message: "Invalid or expired token".to_string(),
                }),
            ));
        }
    };
    
    // Check if token is expired
    let now = Utc::now().timestamp();
    if claims.exp < now {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                code: "TOKEN_EXPIRED".to_string(),
                message: "Token has expired".to_string(),
            }),
        ));
    }
    
    // TODO: Add token to blacklist in database/redis
    // For now, just validate and return success
    info!("User {} logged out successfully", claims.username);
    
    Ok(())
}



/// Decode and validate JWT token
fn decode_token(token: &str, secret: &str) -> Result<Claims, String> {
    use jsonwebtoken::DecodingKey;
    
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = false; // We'll manually handle expiration in logout
    
    match jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
        Ok(claims) => Ok(claims.claims),
        Err(e) => Err(format!("Token validation failed: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password_123";
        let hash = hash_password(password).expect("Failed to hash password");
        
        // Verify the hash can be parsed
        let parsed = PasswordHash::new(&hash).expect("Failed to parse hash");
        assert!(parsed.hash.is_some());
    }
    
    #[test]
    fn test_hash_password_uniqueness() {
        let password = "same_password";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }
}
