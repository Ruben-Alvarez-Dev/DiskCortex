//! Dry-run analyzer

use super::commands::get_cleanup_commands;
use std::path::PathBuf;

/// Dry-run analyzer for previewing cleanup
pub struct DryRunAnalyzer;

impl DryRunAnalyzer {
    /// Analyze what would be cleaned
    pub fn analyze(tool_ids: &[String]) -> Vec<DryRunResult> {
        let commands = get_cleanup_commands();
        let mut results = Vec::new();

        for tool_id in tool_ids {
            if let Some(cmd) = commands.get(tool_id) {
                let size = Self::calculate_paths_size(&cmd.paths);
                results.push(DryRunResult {
                    tool_id: tool_id.clone(),
                    tool_name: cmd.tool_name.clone(),
                    would_free: size,
                    paths: cmd.paths.clone(),
                    command: cmd.command.join(" "),
                    risk_level: cmd.risk_level.clone(),
                    description: cmd.description.clone(),
                });
            }
        }

        results
    }

    /// Calculate total size of paths
    fn calculate_paths_size(paths: &[PathBuf]) -> u64 {
        let mut total = 0u64;
        for path in paths {
            if path.exists() {
                total += Self::calculate_dir_size(path);
            }
        }
        total
    }

    /// Recursively calculate directory size
    fn calculate_dir_size(path: &PathBuf) -> u64 {
        let mut total = 0u64;

        if path.is_file() {
            if let Ok(metadata) = std::fs::metadata(path) {
                total += metadata.len();
            }
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    total += Self::calculate_dir_size(&entry.path());
                }
            }
        }

        total
    }
}

/// Result of a dry-run analysis
#[derive(Debug, Clone)]
pub struct DryRunResult {
    pub tool_id: String,
    pub tool_name: String,
    pub would_free: u64,
    pub paths: Vec<PathBuf>,
    pub command: String,
    pub risk_level: String,
    pub description: String,
}

impl DryRunResult {
    /// Format bytes as human readable
    pub fn format_size(&self) -> String {
        let bytes = self.would_free;
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
}
