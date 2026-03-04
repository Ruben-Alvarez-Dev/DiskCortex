//! Cleanup report structures

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete cleanup report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupReport {
    /// Individual tool results
    pub results: Vec<CleanupResult>,
    /// Total bytes freed
    pub total_bytes_freed: u64,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
    /// Whether this was a dry-run
    pub dry_run: bool,
}

impl CleanupReport {
    /// Create a new empty report
    pub fn new(dry_run: bool) -> Self {
        Self {
            results: Vec::new(),
            total_bytes_freed: 0,
            total_duration_ms: 0,
            dry_run,
        }
    }

    /// Add a result to the report
    pub fn add_result(&mut self, result: CleanupResult) {
        self.total_bytes_freed += result.bytes_freed;
        self.results.push(result);
    }

    /// Format total size as human readable
    pub fn format_total_size(&self) -> String {
        format_bytes(self.total_bytes_freed)
    }

    /// Format duration as human readable
    pub fn format_duration(&self) -> String {
        let secs = self.total_duration_ms / 1000;
        let ms = self.total_duration_ms % 1000;
        if secs > 0 {
            format!("{}.{:03}s", secs, ms)
        } else {
            format!("{}ms", ms)
        }
    }

    /// Count successful cleanups
    pub fn successful_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == CleanupStatus::Success)
            .count()
    }

    /// Count failed cleanups
    pub fn failed_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == CleanupStatus::Failed)
            .count()
    }

    /// Count skipped cleanups
    pub fn skipped_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == CleanupStatus::Skipped)
            .count()
    }
}

/// Result for a single tool cleanup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupResult {
    /// Tool identifier
    pub tool_id: String,
    /// Tool display name
    pub tool_name: String,
    /// Cleanup status
    pub status: CleanupStatus,
    /// Bytes freed (estimated)
    pub bytes_freed: u64,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Output message
    pub message: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

impl CleanupResult {
    /// Format bytes as human readable
    pub fn format_size(&self) -> String {
        format_bytes(self.bytes_freed)
    }
}

/// Status of a cleanup operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CleanupStatus {
    /// Successfully cleaned
    Success,
    /// Dry-run only (preview)
    DryRun,
    /// Skipped (not installed, etc.)
    Skipped,
    /// Failed with error
    Failed,
}

impl std::fmt::Display for CleanupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanupStatus::Success => write!(f, "Success"),
            CleanupStatus::DryRun => write!(f, "Dry Run"),
            CleanupStatus::Skipped => write!(f, "Skipped"),
            CleanupStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Format bytes as human readable
pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
        assert_eq!(format_bytes(1099511627776), "1.00 TB");
    }

    #[test]
    fn test_cleanup_report() {
        let mut report = CleanupReport::new(true);
        assert!(report.dry_run);
        assert_eq!(report.results.len(), 0);

        report.add_result(CleanupResult {
            tool_id: "npm".to_string(),
            tool_name: "npm".to_string(),
            status: CleanupStatus::DryRun,
            bytes_freed: 1024 * 1024 * 100, // 100 MB
            duration_ms: 50,
            message: None,
            error: None,
        });

        assert_eq!(report.results.len(), 1);
        assert_eq!(report.total_bytes_freed, 104857600);
    }
}
