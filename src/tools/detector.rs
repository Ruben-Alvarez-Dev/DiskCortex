//! Tool Detection System

use super::registry::{ToolCategory, RiskLevel};
use std::path::PathBuf;

/// A detected tool with size information
#[derive(Debug, Clone)]
pub struct DetectedTool {
    pub id: String,
    pub name: String,
    pub category: ToolCategory,
    pub paths: Vec<PathBuf>,
    pub size: u64,
    pub risk: RiskLevel,
    pub description: String,
}

/// Tool detector
pub struct ToolDetector {
    home: PathBuf,
}

impl ToolDetector {
    pub fn new() -> Self {
        Self {
            home: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
        }
    }

    pub fn detect_all(&self) -> Vec<DetectedTool> {
        let mut tools = Vec::new();
        
        // AI Tools
        self.check_and_add(&mut tools, "goose", "Goose AI", ToolCategory::AiTool,
            &[self.home.join(".local/state/goose")], RiskLevel::Safe);
        
        #[cfg(target_os = "macos")]
        self.check_and_add(&mut tools, "claude-desktop", "Claude Desktop", ToolCategory::AiTool,
            &[self.home.join("Library/Application Support/Claude")], RiskLevel::Low);
        
        self.check_and_add(&mut tools, "cursor", "Cursor IDE", ToolCategory::Ide,
            &[self.home.join(".cursor")], RiskLevel::Medium);
        
        // Package Managers
        self.check_and_add(&mut tools, "npm", "npm", ToolCategory::PackageManager,
            &[self.home.join(".npm")], RiskLevel::Safe);
        
        self.check_and_add(&mut tools, "yarn", "Yarn", ToolCategory::PackageManager,
            &[self.home.join(".cache/yarn")], RiskLevel::Safe);
        
        self.check_and_add(&mut tools, "pip", "pip", ToolCategory::PackageManager,
            &[self.home.join(".cache/pip")], RiskLevel::Safe);
        
        self.check_and_add(&mut tools, "cargo", "Cargo", ToolCategory::PackageManager,
            &[self.home.join(".cargo/registry")], RiskLevel::Low);
        
        // Build Systems
        self.check_and_add(&mut tools, "gradle", "Gradle", ToolCategory::BuildSystem,
            &[self.home.join(".gradle/caches")], RiskLevel::Low);
        
        // IDEs
        self.check_and_add(&mut tools, "vscode", "VS Code", ToolCategory::Ide,
            &[self.home.join(".vscode/extensions")], RiskLevel::Medium);
        
        tools
    }
    
    fn check_and_add(
        &self,
        tools: &mut Vec<DetectedTool>,
        id: &str,
        name: &str,
        category: ToolCategory,
        cache_paths: &[PathBuf],
        risk: RiskLevel,
    ) {
        let exists = cache_paths.iter().any(|p| p.exists()) || which::which(id).is_ok();
        
        if exists {
            let size = cache_paths.iter()
                .filter(|p| p.exists())
                .map(|p| self.calculate_path_size(p))
                .sum();
            
            tools.push(DetectedTool {
                id: id.to_string(),
                name: name.to_string(),
                category,
                paths: cache_paths.to_vec(),
                size,
                risk,
                description: format!("{} cache and artifacts", name),
            });
        }
    }
    
    fn calculate_path_size(&self, path: &PathBuf) -> u64 {
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    }
}

impl Default for ToolDetector {
    fn default() -> Self { Self::new() }
}
