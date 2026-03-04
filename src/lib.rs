//! DiskCortex Library
//!
//! Core library for DiskCortex - Enterprise-grade disk cleanup automation

pub mod config;
pub mod cleaner;
pub mod tools;

// TUI module
pub mod tui;

// Daemon module (always available for library consumers)
pub mod daemon;
