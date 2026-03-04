//! Configuration validator

use super::settings::{AppConfig, RiskLevel};
use std::path::Path;

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate the entire configuration
    pub fn validate(config: &AppConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate daemon settings
        Self::validate_daemon(config, &mut errors);

        // Validate scanning settings
        Self::validate_scanning(config, &mut errors);

        // Validate cleanup settings
        Self::validate_cleanup(config, &mut errors);

        // Validate safety settings
        Self::validate_safety(config, &mut errors);

        // Validate UI settings
        Self::validate_ui(config, &mut errors);

        // Validate scheduling settings
        Self::validate_scheduling(config, &mut errors);

        // Validate logging settings
        Self::validate_logging(config, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_daemon(config: &AppConfig, errors: &mut Vec<String>) {
        // Port must be valid
        if config.daemon.port == 0 {
            errors.push("daemon.port cannot be 0".to_string());
        }

        // Bind address must be valid
        if config.daemon.bind_address.is_empty() {
            errors.push("daemon.bind_address cannot be empty".to_string());
        }

        // HTTPS requires cert and key
        if config.daemon.enable_https {
            if config.daemon.tls_cert_path.is_none() {
                errors.push("daemon.tls_cert_path is required when HTTPS is enabled".to_string());
            }
            if config.daemon.tls_key_path.is_none() {
                errors.push("daemon.tls_key_path is required when HTTPS is enabled".to_string());
            }
        }

        // Auth requires API key
        if config.daemon.auth_enabled && config.daemon.api_key.is_none() {
            errors.push("daemon.api_key is required when auth is enabled".to_string());
        }

        // Validate max concurrent requests
        if config.daemon.max_concurrent_requests == 0 {
            errors.push("daemon.max_concurrent_requests must be at least 1".to_string());
        }

        // Validate timeout
        if config.daemon.request_timeout_secs == 0 {
            errors.push("daemon.request_timeout_secs must be at least 1".to_string());
        }
    }

    fn validate_scanning(config: &AppConfig, errors: &mut Vec<String>) {
        // Parallel threads must be reasonable
        if config.scanning.parallel_threads == 0 {
            errors.push("scanning.parallel_threads must be at least 1".to_string());
        }
        if config.scanning.parallel_threads > 32 {
            errors.push("scanning.parallel_threads should not exceed 32".to_string());
        }

        // Timeout must be reasonable
        if config.scanning.scan_timeout_secs == 0 {
            errors.push("scanning.scan_timeout_secs must be at least 1".to_string());
        }

        // Max depth must be reasonable
        if config.scanning.max_depth == 0 {
            errors.push("scanning.max_depth must be at least 1".to_string());
        }
        if config.scanning.max_depth > 50 {
            errors.push("scanning.max_depth should not exceed 50".to_string());
        }

        // Validate additional paths exist
        for path in &config.scanning.additional_paths {
            if !path.exists() {
                errors.push(format!(
                    "scanning.additional_paths: {:?} does not exist",
                    path
                ));
            }
        }
    }

    fn validate_cleanup(config: &AppConfig, errors: &mut Vec<String>) {
        // Parallel ops must be reasonable
        if config.cleanup.max_parallel_ops == 0 {
            errors.push("cleanup.max_parallel_ops must be at least 1".to_string());
        }
        if config.cleanup.max_parallel_ops > 10 {
            errors.push("cleanup.max_parallel_ops should not exceed 10".to_string());
        }

        // Backup retention must be reasonable
        if config.cleanup.backup_retention_days > 365 {
            errors.push("cleanup.backup_retention_days should not exceed 365".to_string());
        }
    }

    fn validate_safety(config: &AppConfig, errors: &mut Vec<String>) {
        // Max unconfirmed size must be positive
        if config.safety.max_unconfirmed_size_gb <= 0.0 {
            errors.push("safety.max_unconfirmed_size_gb must be positive".to_string());
        }

        // Validate protected paths exist
        for path in &config.safety.protected_paths {
            if !path.exists() {
                errors.push(format!("safety.protected_paths: {:?} does not exist", path));
            }
        }

        // Audit log retention must be reasonable
        if config.safety.audit_log_retention_days > 365 {
            errors.push("safety.audit_log_retention_days should not exceed 365".to_string());
        }
    }

    fn validate_ui(config: &AppConfig, errors: &mut Vec<String>) {
        // Theme must be valid
        let valid_themes = ["dark", "light", "system"];
        if !valid_themes.contains(&config.ui.theme.as_str()) {
            errors.push(format!("ui.theme must be one of: {:?}", valid_themes));
        }

        // Font scale must be reasonable
        if config.ui.font_scale < 0.5 || config.ui.font_scale > 3.0 {
            errors.push("ui.font_scale must be between 0.5 and 3.0".to_string());
        }

        // Animation speed must be reasonable
        if config.ui.animation_speed < 0.1 || config.ui.animation_speed > 5.0 {
            errors.push("ui.animation_speed must be between 0.1 and 5.0".to_string());
        }

        // Window size must be reasonable
        if config.ui.window_size.0 < 800 || config.ui.window_size.1 < 600 {
            errors.push("ui.window_size must be at least 800x600".to_string());
        }

        // Default view must be valid
        let valid_views = ["overview", "tools", "cleanup", "settings"];
        if !valid_views.contains(&config.ui.default_view.as_str()) {
            errors.push(format!("ui.default_view must be one of: {:?}", valid_views));
        }
    }

    fn validate_scheduling(config: &AppConfig, errors: &mut Vec<String>) {
        if config.scheduling.enabled {
            // Validate cron expression (basic check)
            let parts: Vec<&str> = config
                .scheduling
                .schedule_expression
                .split_whitespace()
                .collect();
            if parts.len() != 5 {
                errors.push(
                    "scheduling.schedule_expression must be a valid cron expression (5 parts)"
                        .to_string(),
                );
            }

            // Validate notify time
            if config.scheduling.notify_before_minutes > 1440 {
                errors.push(
                    "scheduling.notify_before_minutes should not exceed 1440 (24 hours)"
                        .to_string(),
                );
            }
        }
    }

    fn validate_logging(config: &AppConfig, errors: &mut Vec<String>) {
        // Max log size must be reasonable
        if config.logging.max_log_size_mb == 0 {
            errors.push("logging.max_log_size_mb must be at least 1".to_string());
        }
        if config.logging.max_log_size_mb > 1000 {
            errors.push("logging.max_log_size_mb should not exceed 1000".to_string());
        }

        // Log retention must be reasonable
        if config.logging.log_retention_days > 365 {
            errors.push("logging.log_retention_days should not exceed 365".to_string());
        }

        // Log format must be valid
        let valid_formats = ["json", "text"];
        if !valid_formats.contains(&config.logging.log_format.as_str()) {
            errors.push(format!(
                "logging.log_format must be one of: {:?}",
                valid_formats
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default_config() {
        let config = AppConfig::default();
        assert!(ConfigValidator::validate(&config).is_ok());
    }

    #[test]
    fn test_validate_invalid_port() {
        let mut config = AppConfig::default();
        config.daemon.port = 0;
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("port")));
    }
}
