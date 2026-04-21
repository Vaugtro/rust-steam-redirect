//! Steam Proton Game Wrapper
//!
//! A wrapper that redirects Steam game execution to another program based on a configuration file.
//! Useful for launching mod managers, custom launchers, or alternative executables instead of the
//! original Steam game.
//!
//! # Features
//!
//! - Redirect Steam game execution to any program
//! - Support for relative paths (`./`, `../`) and absolute paths
//! - Handle paths with spaces using quotes
//! - Automatic config file detection (searches current and parent directories)
//! - Simple INI-style configuration
//! - Pass arguments to the wrapped program
//! - Exit with the same status code as the wrapped program
//!
//! # Configuration
//!
//! Create a `redirect/config.cfg` file in the wrapper directory or a parent directory:
//!
//! ```text
//! # Simple relative path
//! program=./game.exe
//!
//! # Absolute path with spaces (use quotes)
//! program="C:\Program Files\ModOrganizer2\ModOrganizer.exe"
//!
//! # With arguments
//! program="./launcher.exe" --config game.ini
//! ```
//!
//! # Example
//!
//! ```no_run
//! use std::path::Path;
//! use steam_redirect::config::load_config;
//!
//! let exe_dir = Path::new(".");
//! let (config_dir, program_args) = load_config(exe_dir)?;
//! println!("Program args: {:?}", program_args);
//! # Ok::<(), steam_redirect::error::WrapperError>(())
//! ```

pub mod config;
pub mod error;
pub mod executor;
pub mod path;

pub use error::{Result, WrapperError};
