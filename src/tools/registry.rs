//! Tool Registry - Types and definitions

use serde::{Deserialize, Serialize};

/// Tool category for grouping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    AiTool,
    Ide,
    BuildSystem,
    PackageManager,
    Runtime,
    Container,
    Mobile,
    Database,
    Other,
}

impl std::fmt::Display for ToolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AiTool => write!(f, "🤖 AI Tools"),
            Self::Ide => write!(f, "💻 IDEs"),
            Self::BuildSystem => write!(f, "🔨 Build Systems"),
            Self::PackageManager => write!(f, "📦 Package Managers"),
            Self::Runtime => write!(f, "⚡ Runtimes"),
            Self::Container => write!(f, "🐳 Containers"),
            Self::Mobile => write!(f, "📱 Mobile Dev"),
            Self::Database => write!(f, "🗄️ Databases"),
            Self::Other => write!(f, "📁 Other"),
        }
    }
}

/// Risk level for cleanup operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Safe => write!(f, "🟢 Safe"),
            Self::Low => write!(f, "🟡 Low"),
            Self::Medium => write!(f, "🟠 Medium"),
            Self::High => write!(f, "🔴 High"),
        }
    }
}
