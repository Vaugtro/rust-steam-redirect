/// Error types for the steam wrapper.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrapperError {
    #[error("Config file not found")]
    ConfigNotFound,

    #[error("program entry not found in config file")]
    ProgramNotFound,

    #[error("fallback entry not configured in config file")]
    FallbackNotConfigured,

    #[error("Failed to execute program: {0}")]
    ExecutionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, WrapperError>;
