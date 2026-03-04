//! DiskCortex - Enterprise-grade disk cleanup automation
//!
//! A modern, colorful TUI for scanning and cleaning development tool caches.

mod tools;
mod tui;
pub mod config;
pub mod cleaner;
pub mod daemon;

use anyhow::Result;
use config::ConfigLoader;

fn main() -> Result<()> {
    // Load configuration
    let loader = ConfigLoader::new();
    let _config = loader.load().unwrap_or_default();
    
    // Run the TUI
    tui::run()
}
