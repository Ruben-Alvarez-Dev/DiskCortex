//! Application state and logic

use crate::tools::detector::{DetectedTool, ToolDetector};
use crate::tools::registry::{ToolCategory, RiskLevel};
use std::collections::HashSet;

/// Current view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Overview,
    Tools,
    Cleanup,
    Settings,
}

/// Application state
pub struct App {
    pub view: ViewMode,
    pub detector: ToolDetector,
    pub detected_tools: Vec<DetectedTool>,
    pub selected: usize,
    pub selected_for_cleanup: HashSet<String>,
    pub current_tab: usize,
    pub categories: Vec<ToolCategory>,
    pub show_help: bool,
    pub scanning: bool,
    pub frame: u64,
    pub total_size: u64,
    pub status: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            view: ViewMode::Overview,
            detector: ToolDetector::new(),
            detected_tools: Vec::new(),
            selected: 0,
            selected_for_cleanup: HashSet::new(),
            current_tab: 0,
            categories: vec![
                ToolCategory::AiTool,
                ToolCategory::Ide,
                ToolCategory::PackageManager,
                ToolCategory::Runtime,
                ToolCategory::Container,
                ToolCategory::BuildSystem,
                ToolCategory::Mobile,
                ToolCategory::Database,
                ToolCategory::Other,
            ],
            show_help: false,
            scanning: false,
            frame: 0,
            total_size: 0,
            status: "Press 's' to scan for tools".to_string(),
        }
    }

    pub fn scan(&mut self) {
        self.scanning = true;
        self.detected_tools = self.detector.detect_all();
        self.detected_tools.sort_by(|a, b| b.size.cmp(&a.size));
        self.total_size = self.detected_tools.iter().map(|t| t.size).sum();
        self.scanning = false;
        self.status = format!("Found {} tools using {}", self.detected_tools.len(), format_size(self.total_size));
    }

    pub fn prev_item(&mut self) { if self.selected > 0 { self.selected -= 1; } }
    pub fn next_item(&mut self) { if self.selected < self.detected_tools.len().saturating_sub(1) { self.selected += 1; } }
    pub fn prev_tab(&mut self) { if self.current_tab > 0 { self.current_tab -= 1; } }
    pub fn next_tab(&mut self) { if self.current_tab < self.categories.len() - 1 { self.current_tab += 1; } }
    
    pub fn toggle_select(&mut self) {
        if let Some(t) = self.detected_tools.get(self.selected) {
            if self.selected_for_cleanup.contains(&t.id) {
                self.selected_for_cleanup.remove(&t.id);
            } else {
                self.selected_for_cleanup.insert(t.id.clone());
            }
        }
    }
    
    pub fn select_all(&mut self) {
        for t in &self.detected_tools {
            self.selected_for_cleanup.insert(t.id.clone());
        }
    }
    
    pub fn deselect_all(&mut self) {
        self.selected_for_cleanup.clear();
    }
    
    pub fn tick(&mut self) { self.frame = self.frame.wrapping_add(1); }
}

impl Default for App {
    fn default() -> Self { Self::new() }
}

pub fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;
    if bytes >= GB { format!("{:.1} GB", bytes as f64 / GB as f64) }
    else if bytes >= MB { format!("{:.1} MB", bytes as f64 / MB as f64) }
    else if bytes >= KB { format!("{:.1} KB", bytes as f64 / KB as f64) }
    else { format!("{} B", bytes) }
}
