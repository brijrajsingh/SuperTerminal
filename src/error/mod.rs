/// Custom error types for the SuperTerminal application
use thiserror::Error;

/// Main error type for SuperTerminal
#[derive(Error, Debug)]
pub enum SuperTerminalError {
    #[error("OpenAI API error: {0}")]
    OpenAIError(#[from] async_openai::error::OpenAIError),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("User cancelled the operation")]
    UserCancelled,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("API key not found. Please set OPENAI_API_KEY environment variable")]
    MissingApiKey,
}

/// Result type alias for SuperTerminal operations
pub type Result<T> = std::result::Result<T, SuperTerminalError>;
