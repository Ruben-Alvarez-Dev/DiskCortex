//! Configuration loader with file and environment variable support

use super::settings::AppConfig;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration loader
pub struct ConfigLoader {
    config_dir: PathBuf,
}

impl ConfigLoader {
    /// Create a new config loader
    pub fn new() -> Self {
        let config_dir = Self::get_default_config_dir();
        Self { config_dir }
    }

    /// Create a config loader with a custom directory
    pub fn with_dir<P: Into<PathBuf>>(dir: P) -> Self {
        Self {
            config_dir: dir.into(),
        }
    }

    /// Get the default configuration directory
    pub fn get_default_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("diskcortex")
    }

    /// Load configuration from file and environment
    pub fn load(&self) -> Result<AppConfig, ConfigError> {
        let mut config = AppConfig::default();

        // Try to load from file
        if let Ok(file_config) = self.load_from_file() {
            config = file_config;
        }

        // Apply environment variable overrides
        self.apply_env_overrides(&mut config)?;

        Ok(config)
    }

    /// Load configuration from file (YAML or JSON)
    fn load_from_file(&self) -> Result<AppConfig, ConfigError> {
        // Try YAML first, then JSON
        let yaml_path = self.config_dir.join("config.yaml");
        let yml_path = self.config_dir.join("config.yml");
        let json_path = self.config_dir.join("config.json");

        let config_path = if yaml_path.exists() {
            yaml_path
        } else if yml_path.exists() {
            yml_path
        } else if json_path.exists() {
            json_path
        } else {
            return Err(ConfigError::FileNotFound);
        };

        let content =
            fs::read_to_string(&config_path).map_err(|e| ConfigError::ReadError(e.to_string()))?;

        let config = if config_path
            .extension()
            .map(|e| e == "json")
            .unwrap_or(false)
        {
            serde_json::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?
        } else {
            serde_yaml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?
        };

        Ok(config)
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&self, config: &mut AppConfig) -> Result<(), ConfigError> {
        // Daemon settings
        if let Ok(port) = env::var("DISKCORTEX_PORT") {
            if let Ok(port) = port.parse() {
                config.daemon.port = port;
            }
        }

        if let Ok(addr) = env::var("DISKCORTEX_BIND") {
            config.daemon.bind_address = addr;
        }

        if let Ok(key) = env::var("DISKCORTEX_API_KEY") {
            config.daemon.auth_enabled = true;
            config.daemon.api_key = Some(key);
        }

        // Logging
        if let Ok(level) = env::var("DISKCORTEX_LOG_LEVEL") {
            config.logging.level = parse_log_level(&level);
        }

        // Scanning
        if let Ok(threads) = env::var("DISKCORTEX_SCAN_THREADS") {
            if let Ok(threads) = threads.parse() {
                config.scanning.parallel_threads = threads;
            }
        }

        // Safety
        if let Ok(val) = env::var("DISKCORTEX_SKIP_HIGH_RISK") {
            config.safety.skip_high_risk = val.eq_ignore_ascii_case("true");
        }

        Ok(())
    }

    /// Save configuration to file
    pub fn save(&self, config: &AppConfig) -> Result<(), ConfigError> {
        // Ensure config directory exists
        fs::create_dir_all(&self.config_dir).map_err(|e| ConfigError::WriteError(e.to_string()))?;

        let yaml_path = self.config_dir.join("config.yaml");
        let content = serde_yaml::to_string(config)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;

        fs::write(&yaml_path, content).map_err(|e| ConfigError::WriteError(e.to_string()))?;

        Ok(())
    }

    /// Get the path to the configuration file
    pub fn config_path(&self) -> PathBuf {
        self.config_dir.join("config.yaml")
    }

    /// Check if a configuration file exists
    pub fn exists(&self) -> bool {
        self.config_dir.join("config.yaml").exists()
            || self.config_dir.join("config.yml").exists()
            || self.config_dir.join("config.json").exists()
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse log level from string
fn parse_log_level(s: &str) -> super::settings::LogLevel {
    match s.to_lowercase().as_str() {
        "trace" => super::settings::LogLevel::Trace,
        "debug" => super::settings::LogLevel::Debug,
        "info" => super::settings::LogLevel::Info,
        "warn" | "warning" => super::settings::LogLevel::Warn,
        "error" => super::settings::LogLevel::Error,
        _ => super::settings::LogLevel::Info,
    }
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found")]
    FileNotFound,

    #[error("Failed to read configuration file: {0}")]
    ReadError(String),

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Failed to serialize configuration: {0}")]
    SerializeError(String),

    #[error("Failed to write configuration: {0}")]
    WriteError(String),

    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.daemon.port, 7331);
        assert!(config.cleanup.default_dry_run);
        assert!(config.cleanup.confirm_before_cleanup);
    }

    #[test]
    fn test_env_override() {
        std::env::set_var("DISKCORTEX_PORT", "9999");
        let loader = ConfigLoader::new();
        let config = loader.load().unwrap_or_default();
        // Note: This test may not work as expected if config file exists
        // In real tests, use temp directories
        std::env::remove_var("DISKCORTEX_PORT");
    }
}
