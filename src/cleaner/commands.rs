//! Native cleanup commands for each tool
//!
//! Maps each tool to its native cleanup command.

use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a cleanup command for a specific tool
#[derive(Debug, Clone)]
pub struct CleanupCommand {
    /// Tool identifier
    pub tool_id: String,
    /// Display name
    pub tool_name: String,
    /// Command to execute
    pub command: Vec<String>,
    /// Alternative commands (fallbacks)
    pub alternatives: Vec<Vec<String>>,
    /// Paths to clean (for manual cleanup if command fails)
    pub paths: Vec<PathBuf>,
    /// Whether this tool requires elevated privileges
    pub requires_sudo: bool,
    /// Whether dry-run is supported
    pub supports_dry_run: bool,
    /// Dry-run flag to append
    pub dry_run_flag: Option<String>,
    /// Estimated risk level
    pub risk_level: String,
    /// Description of what gets cleaned
    pub description: String,
}

/// Get all known cleanup commands
pub fn get_cleanup_commands() -> HashMap<String, CleanupCommand> {
    let mut commands = HashMap::new();

    // Node.js / JavaScript
    commands.insert(
        "npm".to_string(),
        CleanupCommand {
            tool_id: "npm".to_string(),
            tool_name: "npm".to_string(),
            command: vec![
                "npm".to_string(),
                "cache".to_string(),
                "clean".to_string(),
                "--force".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir().map(|h| h.join(".npm")).unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "npm package cache".to_string(),
        },
    );

    commands.insert(
        "yarn".to_string(),
        CleanupCommand {
            tool_id: "yarn".to_string(),
            tool_name: "Yarn".to_string(),
            command: vec!["yarn".to_string(), "cache".to_string(), "clean".to_string()],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".yarn/cache"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "Yarn package cache".to_string(),
        },
    );

    commands.insert(
        "pnpm".to_string(),
        CleanupCommand {
            tool_id: "pnpm".to_string(),
            tool_name: "pnpm".to_string(),
            command: vec!["pnpm".to_string(), "store".to_string(), "prune".to_string()],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".pnpm-store"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "pnpm package store".to_string(),
        },
    );

    // Python
    commands.insert(
        "pip".to_string(),
        CleanupCommand {
            tool_id: "pip".to_string(),
            tool_name: "pip".to_string(),
            command: vec!["pip".to_string(), "cache".to_string(), "purge".to_string()],
            alternatives: vec![
                vec!["pip3".to_string(), "cache".to_string(), "purge".to_string()],
                vec![
                    "python".to_string(),
                    "-m".to_string(),
                    "pip".to_string(),
                    "cache".to_string(),
                    "purge".to_string(),
                ],
            ],
            paths: vec![
                dirs::cache_dir().map(|c| c.join("pip")).unwrap_or_default(),
                dirs::home_dir()
                    .map(|h| h.join(".cache/pip"))
                    .unwrap_or_default(),
            ],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "pip package cache".to_string(),
        },
    );

    commands.insert(
        "conda".to_string(),
        CleanupCommand {
            tool_id: "conda".to_string(),
            tool_name: "Conda".to_string(),
            command: vec![
                "conda".to_string(),
                "clean".to_string(),
                "--all".to_string(),
                "-y".to_string(),
            ],
            alternatives: vec![],
            paths: vec![],
            requires_sudo: false,
            supports_dry_run: true,
            dry_run_flag: Some("--dry-run".to_string()),
            risk_level: "low".to_string(),
            description: "Conda packages and caches".to_string(),
        },
    );

    // Rust
    commands.insert(
        "cargo".to_string(),
        CleanupCommand {
            tool_id: "cargo".to_string(),
            tool_name: "Cargo".to_string(),
            command: vec!["cargo".to_string(), "cache".to_string(), "-a".to_string()],
            alternatives: vec![],
            paths: vec![
                dirs::home_dir()
                    .map(|h| h.join(".cargo/registry"))
                    .unwrap_or_default(),
                dirs::home_dir()
                    .map(|h| h.join(".cargo/git"))
                    .unwrap_or_default(),
            ],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "Cargo registry and git cache".to_string(),
        },
    );

    // Go
    commands.insert(
        "go".to_string(),
        CleanupCommand {
            tool_id: "go".to_string(),
            tool_name: "Go".to_string(),
            command: vec![
                "go".to_string(),
                "clean".to_string(),
                "-modcache".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("go/pkg/mod"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "low".to_string(),
            description: "Go module cache".to_string(),
        },
    );

    // Java / Maven / Gradle
    commands.insert(
        "maven".to_string(),
        CleanupCommand {
            tool_id: "maven".to_string(),
            tool_name: "Maven".to_string(),
            command: vec![
                "mvn".to_string(),
                "dependency:purge-local-repository".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".m2/repository"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "medium".to_string(),
            description: "Maven local repository (may require re-download)".to_string(),
        },
    );

    commands.insert(
        "gradle".to_string(),
        CleanupCommand {
            tool_id: "gradle".to_string(),
            tool_name: "Gradle".to_string(),
            command: vec!["gradle".to_string(), "--stop".to_string()],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".gradle/caches"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "low".to_string(),
            description: "Gradle caches (stops daemon, manual cache cleanup)".to_string(),
        },
    );

    // Docker
    commands.insert(
        "docker".to_string(),
        CleanupCommand {
            tool_id: "docker".to_string(),
            tool_name: "Docker".to_string(),
            command: vec![
                "docker".to_string(),
                "system".to_string(),
                "prune".to_string(),
                "-f".to_string(),
            ],
            alternatives: vec![vec![
                "docker".to_string(),
                "system".to_string(),
                "prune".to_string(),
                "-f".to_string(),
                "-a".to_string(),
            ]],
            paths: vec![],
            requires_sudo: false,
            supports_dry_run: true,
            dry_run_flag: Some("--dry-run".to_string()),
            risk_level: "medium".to_string(),
            description: "Docker images, containers, and build cache".to_string(),
        },
    );

    commands.insert(
        "docker_images".to_string(),
        CleanupCommand {
            tool_id: "docker_images".to_string(),
            tool_name: "Docker Images".to_string(),
            command: vec![
                "docker".to_string(),
                "image".to_string(),
                "prune".to_string(),
                "-f".to_string(),
                "-a".to_string(),
            ],
            alternatives: vec![],
            paths: vec![],
            requires_sudo: false,
            supports_dry_run: true,
            dry_run_flag: Some("--dry-run".to_string()),
            risk_level: "medium".to_string(),
            description: "Unused Docker images".to_string(),
        },
    );

    commands.insert(
        "docker_volumes".to_string(),
        CleanupCommand {
            tool_id: "docker_volumes".to_string(),
            tool_name: "Docker Volumes".to_string(),
            command: vec![
                "docker".to_string(),
                "volume".to_string(),
                "prune".to_string(),
                "-f".to_string(),
            ],
            alternatives: vec![],
            paths: vec![],
            requires_sudo: false,
            supports_dry_run: true,
            dry_run_flag: Some("--dry-run".to_string()),
            risk_level: "high".to_string(),
            description: "Unused Docker volumes (may contain data)".to_string(),
        },
    );

    commands.insert(
        "docker_buildx".to_string(),
        CleanupCommand {
            tool_id: "docker_buildx".to_string(),
            tool_name: "Docker Buildx".to_string(),
            command: vec![
                "docker".to_string(),
                "buildx".to_string(),
                "prune".to_string(),
                "-f".to_string(),
            ],
            alternatives: vec![],
            paths: vec![],
            requires_sudo: false,
            supports_dry_run: true,
            dry_run_flag: Some("--dry-run".to_string()),
            risk_level: "low".to_string(),
            description: "Docker buildx cache".to_string(),
        },
    );

    // Homebrew
    commands.insert(
        "homebrew".to_string(),
        CleanupCommand {
            tool_id: "homebrew".to_string(),
            tool_name: "Homebrew".to_string(),
            command: vec!["brew".to_string(), "cleanup".to_string(), "-s".to_string()],
            alternatives: vec![vec!["brew".to_string(), "cleanup".to_string()]],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Caches/Homebrew"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: true,
            dry_run_flag: Some("-n".to_string()),
            risk_level: "safe".to_string(),
            description: "Homebrew caches and old versions".to_string(),
        },
    );

    // Xcode
    commands.insert(
        "xcode".to_string(),
        CleanupCommand {
            tool_id: "xcode".to_string(),
            tool_name: "Xcode".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Developer/Xcode/DerivedData".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Developer/Xcode/DerivedData"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "low".to_string(),
            description: "Xcode derived data (build artifacts)".to_string(),
        },
    );

    commands.insert(
        "xcode_archives".to_string(),
        CleanupCommand {
            tool_id: "xcode_archives".to_string(),
            tool_name: "Xcode Archives".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Developer/Xcode/Archives".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Developer/Xcode/Archives"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "high".to_string(),
            description: "Xcode archives (released app builds)".to_string(),
        },
    );

    // JetBrains IDEs
    commands.insert(
        "jetbrains".to_string(),
        CleanupCommand {
            tool_id: "jetbrains".to_string(),
            tool_name: "JetBrains IDEs".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Caches/JetBrains".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Caches/JetBrains"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "low".to_string(),
            description: "JetBrains IDE caches".to_string(),
        },
    );

    commands.insert(
        "jetbrains_logs".to_string(),
        CleanupCommand {
            tool_id: "jetbrains_logs".to_string(),
            tool_name: "JetBrains Logs".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Logs/JetBrains".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Logs/JetBrains"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "JetBrains IDE logs".to_string(),
        },
    );

    // VS Code
    commands.insert(
        "vscode".to_string(),
        CleanupCommand {
            tool_id: "vscode".to_string(),
            tool_name: "VS Code".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Application Support/Code/Cache".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Application Support/Code/Cache"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "VS Code cache".to_string(),
        },
    );

    // AI Tools
    commands.insert(
        "goose".to_string(),
        CleanupCommand {
            tool_id: "goose".to_string(),
            tool_name: "Goose AI".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/.local/state/goose".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".local/state/goose"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "Goose AI logs and state".to_string(),
        },
    );

    commands.insert(
        "claude".to_string(),
        CleanupCommand {
            tool_id: "claude".to_string(),
            tool_name: "Claude Code".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/.claude/logs".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".claude/logs"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "Claude Code logs".to_string(),
        },
    );

    // Android
    commands.insert(
        "android_sdk".to_string(),
        CleanupCommand {
            tool_id: "android_sdk".to_string(),
            tool_name: "Android SDK".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Android/sdk/.temp".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Android/sdk/.temp"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "low".to_string(),
            description: "Android SDK temp files".to_string(),
        },
    );

    commands.insert(
        "android_avd".to_string(),
        CleanupCommand {
            tool_id: "android_avd".to_string(),
            tool_name: "Android Emulator".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/.android/avd/*.avd/cache".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".android/avd"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "medium".to_string(),
            description: "Android emulator caches".to_string(),
        },
    );

    // Flutter
    commands.insert(
        "flutter".to_string(),
        CleanupCommand {
            tool_id: "flutter".to_string(),
            tool_name: "Flutter".to_string(),
            command: vec!["flutter".to_string(), "clean".to_string()],
            alternatives: vec![],
            paths: vec![],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "low".to_string(),
            description: "Flutter build cache (per-project)".to_string(),
        },
    );

    // Generic caches
    commands.insert(
        "system_cache".to_string(),
        CleanupCommand {
            tool_id: "system_cache".to_string(),
            tool_name: "System Cache".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Caches/*".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::cache_dir().unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "medium".to_string(),
            description: "User cache directory (broad cleanup)".to_string(),
        },
    );

    commands.insert(
        "system_logs".to_string(),
        CleanupCommand {
            tool_id: "system_logs".to_string(),
            tool_name: "System Logs".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/Library/Logs/*".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join("Library/Logs"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "User logs directory".to_string(),
        },
    );

    // Trash
    commands.insert(
        "trash".to_string(),
        CleanupCommand {
            tool_id: "trash".to_string(),
            tool_name: "Trash".to_string(),
            command: vec![
                "rm".to_string(),
                "-rf".to_string(),
                "~/.Trash/*".to_string(),
            ],
            alternatives: vec![],
            paths: vec![dirs::home_dir()
                .map(|h| h.join(".Trash"))
                .unwrap_or_default()],
            requires_sudo: false,
            supports_dry_run: false,
            dry_run_flag: None,
            risk_level: "safe".to_string(),
            description: "Empty trash".to_string(),
        },
    );

    commands
}

/// Get cleanup command for a specific tool
pub fn get_command(tool_id: &str) -> Option<CleanupCommand> {
    get_cleanup_commands().get(tool_id).cloned()
}
