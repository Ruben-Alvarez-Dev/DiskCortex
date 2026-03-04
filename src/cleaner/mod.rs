//! Cleanup system with native tool commands
//!
//! Uses tool-native cleanup commands for safety:
//! - npm cache clean --force
//! - pip cache purge
//! - docker system prune
//! - brew cleanup
//! etc.

mod commands;
mod executor;
mod dry_run;
mod report;

pub use commands::*;
pub use executor::*;
pub use dry_run::*;
pub use report::*;
