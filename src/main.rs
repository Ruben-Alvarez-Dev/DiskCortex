//! DiskCortex - Enterprise-grade disk cleanup automation
//!
//! A modern, colorful TUI for scanning and cleaning development tool caches.

mod tools;
mod tui;

use anyhow::Result;

fn main() -> Result<()> {
    // Run the TUI
    tui::run()
}
