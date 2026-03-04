//! Cleanup command executor

use super::commands::{get_command, CleanupCommand};
use super::report::{CleanupReport, CleanupResult, CleanupStatus};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

/// Cleanup executor
pub struct CleanupExecutor {
    dry_run: bool,
    use_trash: bool,
}

impl CleanupExecutor {
    /// Create a new executor
    pub fn new(dry_run: bool, use_trash: bool) -> Self {
        Self { dry_run, use_trash }
    }

    /// Execute cleanup for a single tool
    pub fn cleanup_tool(&self, tool_id: &str) -> CleanupResult {
        let start = Instant::now();

        let command = match get_command(tool_id) {
            Some(cmd) => cmd,
            None => {
                return CleanupResult {
                    tool_id: tool_id.to_string(),
                    tool_name: tool_id.to_string(),
                    status: CleanupStatus::Skipped,
                    bytes_freed: 0,
                    duration_ms: start.elapsed().as_millis() as u64,
                    message: Some("Unknown tool".to_string()),
                    error: None,
                };
            }
        };

        // Check if dry-run is supported
        if self.dry_run && !command.supports_dry_run {
            return self.dry_run_fallback(&command, start);
        }

        // Build command
        let mut cmd_parts = command.command.clone();
        if self.dry_run {
            if let Some(flag) = &command.dry_run_flag {
                cmd_parts.push(flag.clone());
            }
        }

        // Expand ~ in paths
        let cmd_parts = self.expand_paths(cmd_parts);

        // Execute
        self.execute_command(&command, cmd_parts, start)
    }

    /// Execute multiple tool cleanups
    pub fn cleanup_tools(&self, tool_ids: &[String]) -> CleanupReport {
        let start = Instant::now();
        let mut results = Vec::new();
        let mut total_freed = 0u64;

        for tool_id in tool_ids {
            let result = self.cleanup_tool(tool_id);
            total_freed += result.bytes_freed;
            results.push(result);
        }

        CleanupReport {
            results,
            total_bytes_freed: total_freed,
            total_duration_ms: start.elapsed().as_millis() as u64,
            dry_run: self.dry_run,
        }
    }

    /// Execute a command
    fn execute_command(
        &self,
        command: &CleanupCommand,
        cmd_parts: Vec<String>,
        start: Instant,
    ) -> CleanupResult {
        if cmd_parts.is_empty() {
            return CleanupResult {
                tool_id: command.tool_id.clone(),
                tool_name: command.tool_name.clone(),
                status: CleanupStatus::Skipped,
                bytes_freed: 0,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some("No command to execute".to_string()),
                error: None,
            };
        }

        let program = &cmd_parts[0];
        let args = &cmd_parts[1..];

        // Calculate size before cleanup
        let size_before = self.calculate_paths_size(&command.paths);

        // Execute command
        match Command::new(program).args(args).output() {
            Ok(output) => {
                let duration_ms = start.elapsed().as_millis() as u64;

                if output.status.success() {
                    // Calculate size after
                    let size_after = self.calculate_paths_size(&command.paths);
                    let bytes_freed = size_before.saturating_sub(size_after);

                    CleanupResult {
                        tool_id: command.tool_id.clone(),
                        tool_name: command.tool_name.clone(),
                        status: if self.dry_run {
                            CleanupStatus::DryRun
                        } else {
                            CleanupStatus::Success
                        },
                        bytes_freed,
                        duration_ms,
                        message: Some(String::from_utf8_lossy(&output.stdout).to_string()),
                        error: None,
                    }
                } else {
                    // Try alternatives
                    self.try_alternatives(command, start)
                        .unwrap_or_else(|| CleanupResult {
                            tool_id: command.tool_id.clone(),
                            tool_name: command.tool_name.clone(),
                            status: CleanupStatus::Failed,
                            bytes_freed: 0,
                            duration_ms,
                            message: None,
                            error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                        })
                }
            }
            Err(e) => {
                // Command not found - try alternatives
                self.try_alternatives(command, start)
                    .unwrap_or_else(|| CleanupResult {
                        tool_id: command.tool_id.clone(),
                        tool_name: command.tool_name.clone(),
                        status: CleanupStatus::Failed,
                        bytes_freed: 0,
                        duration_ms: start.elapsed().as_millis() as u64,
                        message: None,
                        error: Some(format!("Failed to execute: {}", e)),
                    })
            }
        }
    }

    /// Try alternative commands
    fn try_alternatives(&self, command: &CleanupCommand, start: Instant) -> Option<CleanupResult> {
        for alt in &command.alternatives {
            let alt_parts = self.expand_paths(alt.clone());
            if alt_parts.is_empty() {
                continue;
            }

            let program = &alt_parts[0];
            let args = &alt_parts[1..];

            if let Ok(output) = Command::new(program).args(args).output() {
                if output.status.success() {
                    let size_before = self.calculate_paths_size(&command.paths);
                    let size_after = self.calculate_paths_size(&command.paths);
                    let bytes_freed = size_before.saturating_sub(size_after);

                    return Some(CleanupResult {
                        tool_id: command.tool_id.clone(),
                        tool_name: command.tool_name.clone(),
                        status: if self.dry_run {
                            CleanupStatus::DryRun
                        } else {
                            CleanupStatus::Success
                        },
                        bytes_freed,
                        duration_ms: start.elapsed().as_millis() as u64,
                        message: Some(String::from_utf8_lossy(&output.stdout).to_string()),
                        error: None,
                    });
                }
            }
        }
        None
    }

    /// Dry-run fallback (manual path size calculation)
    fn dry_run_fallback(&self, command: &CleanupCommand, start: Instant) -> CleanupResult {
        let size = self.calculate_paths_size(&command.paths);

        CleanupResult {
            tool_id: command.tool_id.clone(),
            tool_name: command.tool_name.clone(),
            status: CleanupStatus::DryRun,
            bytes_freed: size,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!(
                "Would clean: {}",
                command
                    .paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            error: None,
        }
    }

    /// Expand ~ in paths
    fn expand_paths(&self, parts: Vec<String>) -> Vec<String> {
        parts
            .iter()
            .map(|part| {
                if part.starts_with('~') {
                    if let Some(home) = dirs::home_dir() {
                        part.replacen('~', &home.display().to_string(), 1)
                    } else {
                        part.clone()
                    }
                } else {
                    part.clone()
                }
            })
            .collect()
    }

    /// Calculate total size of paths
    fn calculate_paths_size(&self, paths: &[PathBuf]) -> u64 {
        let mut total = 0u64;
        for path in paths {
            if path.exists() {
                total += self.calculate_dir_size(path);
            }
        }
        total
    }

    /// Recursively calculate directory size
    fn calculate_dir_size(&self, path: &PathBuf) -> u64 {
        let mut total = 0u64;

        if path.is_file() {
            if let Ok(metadata) = std::fs::metadata(path) {
                total += metadata.len();
            }
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    total += self.calculate_dir_size(&entry.path());
                }
            }
        }

        total
    }
}
