use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn init_db() -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:diskcortex.db?mode=memory&cache=shared")
        .await?;
    
    sqlx::query!(
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
    .execute(&pool)
    .await?;
    
    Ok(pool)
}
