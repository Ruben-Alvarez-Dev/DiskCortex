//! Configuration settings structures

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// General application settings
    pub general: GeneralConfig,
    /// Daemon/service settings
    pub daemon: DaemonConfig,
    /// Scanning settings
    pub scanning: ScanningConfig,
    /// Cleanup settings
    pub cleanup: CleanupConfig,
    /// Safety settings
    pub safety: SafetyConfig,
    /// UI settings
    pub ui: UiConfig,
    /// Scheduling settings
    pub scheduling: SchedulingConfig,
    /// Logging settings
    pub logging: LoggingConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            daemon: DaemonConfig::default(),
            scanning: ScanningConfig::default(),
            cleanup: CleanupConfig::default(),
            safety: SafetyConfig::default(),
            ui: UiConfig::default(),
            scheduling: SchedulingConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Application language
    pub language: String,
    /// Auto-start with system
    pub auto_start: bool,
    /// Check for updates automatically
    pub auto_update_check: bool,
    /// Minimum disk space warning threshold (GB)
    pub min_disk_space_warning: f64,
    /// Enable analytics (anonymous usage data)
    pub enable_analytics: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            auto_start: false,
            auto_update_check: true,
            min_disk_space_warning: 5.0,
            enable_analytics: false,
        }
    }
}

/// Daemon/service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DaemonConfig {
    /// Port for the HTTP API
    pub port: u16,
    /// Bind address
    pub bind_address: String,
    /// Enable HTTPS
    pub enable_https: bool,
    /// TLS certificate path (if HTTPS enabled)
    pub tls_cert_path: Option<PathBuf>,
    /// TLS key path (if HTTPS enabled)
    pub tls_key_path: Option<PathBuf>,
    /// API authentication enabled
    pub auth_enabled: bool,
    /// API key (if auth enabled)
    pub api_key: Option<String>,
    /// Max concurrent requests
    pub max_concurrent_requests: usize,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Enable CORS
    pub enable_cors: bool,
    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            port: 7331,
            bind_address: "127.0.0.1".to_string(),
            enable_https: false,
            tls_cert_path: None,
            tls_key_path: None,
            auth_enabled: false,
            api_key: None,
            max_concurrent_requests: 100,
            request_timeout_secs: 30,
            enable_cors: true,
            cors_origins: vec!["http://localhost:*".to_string()],
        }
    }
}

/// Scanning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScanningConfig {
    /// Auto-scan on startup
    pub auto_scan_on_startup: bool,
    /// Scan interval in minutes (0 = disabled)
    pub scan_interval_minutes: u32,
    /// Parallel scan threads
    pub parallel_threads: usize,
    /// Scan timeout per tool in seconds
    pub scan_timeout_secs: u64,
    /// Include hidden files/folders
    pub include_hidden: bool,
    /// Follow symbolic links
    pub follow_symlinks: bool,
    /// Maximum directory depth
    pub max_depth: usize,
    /// Additional paths to scan
    pub additional_paths: Vec<PathBuf>,
    /// Paths to exclude from scanning
    pub exclude_paths: Vec<PathBuf>,
    /// File patterns to exclude (glob)
    pub exclude_patterns: Vec<String>,
    /// Minimum file size to consider (bytes)
    pub min_file_size: u64,
}

impl Default for ScanningConfig {
    fn default() -> Self {
        Self {
            auto_scan_on_startup: true,
            scan_interval_minutes: 0,
            parallel_threads: 4,
            scan_timeout_secs: 60,
            include_hidden: false,
            follow_symlinks: false,
            max_depth: 10,
            additional_paths: vec![],
            exclude_paths: vec![],
            exclude_patterns: vec!["*.log".to_string(), "*.tmp".to_string()],
            min_file_size: 1024, // 1KB
        }
    }
}

/// Cleanup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CleanupConfig {
    /// Default to dry-run mode
    pub default_dry_run: bool,
    /// Use trash instead of permanent delete
    pub use_trash: bool,
    /// Age filter: only clean files older than X days (0 = no filter)
    pub default_age_filter_days: u32,
    /// Confirm before cleanup
    pub confirm_before_cleanup: bool,
    /// Show detailed cleanup report
    pub detailed_report: bool,
    /// Parallel cleanup operations
    pub parallel_cleanup: bool,
    /// Max parallel cleanup operations
    pub max_parallel_ops: usize,
    /// Create backup before cleanup
    pub create_backup: bool,
    /// Backup retention days
    pub backup_retention_days: u32,
    /// Tools to always skip
    pub skip_tools: HashSet<String>,
    /// Categories to always skip
    pub skip_categories: HashSet<String>,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            default_dry_run: true,
            use_trash: false,
            default_age_filter_days: 0,
            confirm_before_cleanup: true,
            detailed_report: true,
            parallel_cleanup: true,
            max_parallel_ops: 3,
            create_backup: false,
            backup_retention_days: 7,
            skip_tools: HashSet::new(),
            skip_categories: HashSet::new(),
        }
    }
}

/// Safety configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SafetyConfig {
    /// Minimum risk level to require confirmation
    pub confirmation_risk_level: RiskLevel,
    /// Auto-skip high-risk items
    pub skip_high_risk: bool,
    /// Protect active projects (detected by git/vcs)
    pub protect_active_projects: bool,
    /// Protected paths (never clean)
    pub protected_paths: Vec<PathBuf>,
    /// Maximum cleanup size without confirmation (GB)
    pub max_unconfirmed_size_gb: f64,
    /// Enable undo/restore functionality
    pub enable_restore: bool,
    /// Audit log enabled
    pub audit_log_enabled: bool,
    /// Audit log path
    pub audit_log_path: Option<PathBuf>,
    /// Audit log retention days
    pub audit_log_retention_days: u32,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            confirmation_risk_level: RiskLevel::Medium,
            skip_high_risk: false,
            protect_active_projects: true,
            protected_paths: vec![],
            max_unconfirmed_size_gb: 1.0,
            enable_restore: true,
            audit_log_enabled: true,
            audit_log_path: None,
            audit_log_retention_days: 90,
        }
    }
}

/// Risk levels for cleanup items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Low,
    #[default]
    Medium,
    High,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Safe => write!(f, "safe"),
            RiskLevel::Low => write!(f, "low"),
            RiskLevel::Medium => write!(f, "medium"),
            RiskLevel::High => write!(f, "high"),
        }
    }
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    /// Theme (dark, light, system)
    pub theme: String,
    /// Accent color (hex)
    pub accent_color: String,
    /// Font size multiplier
    pub font_scale: f32,
    /// Enable animations
    pub animations_enabled: bool,
    /// Animation speed (0.5 = half, 1.0 = normal, 2.0 = double)
    pub animation_speed: f32,
    /// Show tooltips
    pub show_tooltips: bool,
    /// Tooltip delay in ms
    pub tooltip_delay_ms: u32,
    /// Compact mode
    pub compact_mode: bool,
    /// Show size in bytes
    pub show_bytes: bool,
    /// Date format
    pub date_format: String,
    /// Number format (comma or dot separator)
    pub number_format: String,
    /// Default view on startup
    pub default_view: String,
    /// Sidebar collapsed by default
    pub sidebar_collapsed: bool,
    /// Window size (width, height)
    pub window_size: (u32, u32),
    /// Window position (x, y)
    pub window_position: Option<(i32, i32)>,
    /// Maximize window on startup
    pub maximize_on_startup: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            accent_color: "#0ea5e9".to_string(),
            font_scale: 1.0,
            animations_enabled: true,
            animation_speed: 1.0,
            show_tooltips: true,
            tooltip_delay_ms: 500,
            compact_mode: false,
            show_bytes: false,
            date_format: "%Y-%m-%d %H:%M".to_string(),
            number_format: "1,234.56".to_string(),
            default_view: "overview".to_string(),
            sidebar_collapsed: false,
            window_size: (1400, 900),
            window_position: None,
            maximize_on_startup: false,
        }
    }
}

/// Scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SchedulingConfig {
    /// Enable scheduled cleanups
    pub enabled: bool,
    /// Schedule expression (cron format)
    pub schedule_expression: String,
    /// Scheduled cleanup mode
    pub cleanup_mode: ScheduledCleanupMode,
    /// Tools to include in scheduled cleanup
    pub scheduled_tools: HashSet<String>,
    /// Categories to include in scheduled cleanup
    pub scheduled_categories: HashSet<String>,
    /// Notify before scheduled cleanup (minutes)
    pub notify_before_minutes: u32,
    /// Auto-skip if low disk space not needed
    pub skip_if_not_needed: bool,
    /// Minimum free space to maintain (GB)
    pub min_free_space_gb: f64,
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            schedule_expression: "0 3 * * 0".to_string(), // Weekly on Sunday at 3 AM
            cleanup_mode: ScheduledCleanupMode::DryRun,
            scheduled_tools: HashSet::new(),
            scheduled_categories: HashSet::new(),
            notify_before_minutes: 30,
            skip_if_not_needed: true,
            min_free_space_gb: 10.0,
        }
    }
}

/// Scheduled cleanup modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ScheduledCleanupMode {
    #[default]
    DryRun,
    SafeOnly,
    AllSelected,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Log to file
    pub log_to_file: bool,
    /// Log file path
    pub log_file_path: Option<PathBuf>,
    /// Log rotation (daily, weekly, size-based)
    pub log_rotation: LogRotation,
    /// Max log file size (MB, for size-based rotation)
    pub max_log_size_mb: u64,
    /// Log retention days
    pub log_retention_days: u32,
    /// Log format (json, text)
    pub log_format: String,
    /// Include timestamps
    pub include_timestamp: bool,
    /// Include source location
    pub include_source: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_to_file: true,
            log_file_path: None,
            log_rotation: LogRotation::Daily,
            max_log_size_mb: 10,
            log_retention_days: 30,
            log_format: "text".to_string(),
            include_timestamp: true,
            include_source: false,
        }
    }
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

/// Log rotation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogRotation {
    #[default]
    Daily,
    Weekly,
    SizeBased,
    Never,
}
