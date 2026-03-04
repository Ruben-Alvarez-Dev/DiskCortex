//! DiskCortex Configuration System
//!
//! Enterprise-grade configuration with:
//! - YAML/JSON file support
//! - Environment variable overrides
//! - Default values with validation
//! - Hot-reload support

mod settings;
mod loader;
mod validator;

pub use settings::*;
pub use loader::*;
pub use validator::*;
