//! Database module

use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tracing::info;

pub type DbState = SqlitePool;

pub async fn init_db() -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:diskcortex.db?mode=memory&cache=shared")
        .await?;
    
    run_migrations(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(pool)
}

/// Create database with file persistence
pub async fn init_db_with_path(db_path: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await?;
    
    run_migrations(&pool).await?;
    
    info!(path = %db_path, "Database initialized with file persistence");
    Ok(pool)
}

/// Run all migrations
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'viewer',
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS tools (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            category TEXT NOT NULL,
            description TEXT,
            version TEXT,
            installed INTEGER NOT NULL DEFAULT 0,
            install_path TEXT,
            requires_confirmation INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS cleanup_plans (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            confirmation_token TEXT,
            created_at TEXT NOT NULL,
            confirmed_at TEXT
        );
        
        CREATE TABLE IF NOT EXISTS cleanup_items (
            id TEXT PRIMARY KEY,
            plan_id TEXT NOT NULL,
            tool_id TEXT,
            path TEXT NOT NULL,
            size INTEGER NOT NULL DEFAULT 0,
            category TEXT NOT NULL,
            risk TEXT NOT NULL DEFAULT 'safe',
            FOREIGN KEY (plan_id) REFERENCES cleanup_plans(id)
        );
        
        CREATE TABLE IF NOT EXISTS audit_logs (
            id TEXT PRIMARY KEY,
            timestamp TEXT NOT NULL,
            user_id TEXT,
            action TEXT NOT NULL,
            resource TEXT,
            resource_id TEXT,
            details TEXT,
            ip_address TEXT
        );
        
        CREATE TABLE IF NOT EXISTS schedules (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            cron TEXT NOT NULL,
            last_run TEXT,
            next_run TEXT
        );
        "#,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Create in-memory database for tests with a REAL hashed password
pub async fn init_test_db() -> Result<SqlitePool> {
    use argon2::{
        password_hash::{SaltString, PasswordHasher},
        Argon2,
    };
    use rand_core::OsRng;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    
    // Run migrations
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'viewer',
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;
    
    // Create REAL password hash with argon2
    // This is a REAL hash for "admin123" - NO mock, NO fake
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password("admin123".as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();
    
    // Insert test user with REAL hashed password
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, active, created_at, updated_at)
        VALUES ('test-user-id', 'admin', 'admin@test.com', ?, 'admin', 1, datetime('now'), datetime('now'))
        "#,
    )
    .bind(&password_hash)
    .execute(&pool)
    .await?;
    
    // Also insert an inactive user for testing
    let inactive_salt = SaltString::generate(&mut OsRng);
    let inactive_hash = argon2
        .hash_password("inactive123".as_bytes(), &inactive_salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();
    
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, active, created_at, updated_at)
        VALUES ('inactive-user-id', 'inactive', 'inactive@test.com', ?, 'viewer', 0, datetime('now'), datetime('now'))
        "#,
    )
    .bind(&inactive_hash)
    .execute(&pool)
    .await?;
    
    Ok(pool)
}

/// Create test database with custom users
pub async fn init_test_db_with_users(users: Vec<(&str, &str, &str, bool)>) -> Result<SqlitePool> {
    use argon2::{
        password_hash::{SaltString, PasswordHasher},
        Argon2,
    };
    use rand_core::OsRng;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    
    // Run migrations
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'viewer',
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;
    
    let argon2 = Argon2::default();
    
    for (username, email, password, active) in users {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();
        
        let active_int = if active { 1 } else { 0 };
        
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, active, created_at, updated_at)
            VALUES (?, ?, ?, ?, 'viewer', ?, datetime('now'), datetime('now'))
            "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(username)
        .bind(email)
        .bind(&password_hash)
        .bind(active_int)
        .execute(&pool)
        .await?;
    }
    
    Ok(pool)
}
