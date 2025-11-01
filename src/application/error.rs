//! Application Layer Error Types
//!
//! This module defines error types specific to the application layer,
//! which orchestrates domain and infrastructure operations.

use thiserror::Error;

/// Application layer errors
///
/// These errors represent failures in application-level operations,
/// such as planning service workflows or interactive session handling.
#[derive(Error, Debug)]
pub enum ApplicationError {
    /// Failed to parse architecture document
    #[error("Failed to parse architecture document: {0}")]
    ParseError(String),

    /// Failed to generate implementation plan
    #[error("Failed to generate implementation plan: {0}")]
    GenerationError(String),

    /// Failed to write plan to file
    #[error("Failed to write plan to file: {0}")]
    WriteError(String),

    /// Failed to read architecture file
    #[error("Failed to read architecture file '{path}': {source}")]
    ReadArchitectureError {
        path: String,
        source: std::io::Error,
    },

    /// Architecture file not found
    #[error("Architecture file not found: {0}")]
    ArchitectureNotFound(String),

    /// Invalid architecture document
    #[error("Invalid architecture document: {0}")]
    InvalidArchitecture(String),

    /// Plan validation failed
    #[error("Plan validation failed: {0}")]
    ValidationError(String),

    /// Ollama service unavailable
    #[error("Ollama service unavailable: {0}")]
    OllamaUnavailable(String),

    /// Model not found
    #[error("Model '{0}' not found. Available models: {1}")]
    ModelNotFound(String, String),

    /// User cancelled operation
    #[error("Operation cancelled by user")]
    Cancelled,

    /// Interactive session error
    #[error("Interactive session error: {0}")]
    InteractiveError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Output file already exists
    #[error("Output file already exists: {0}")]
    OutputExists(String),

    /// Invalid output path
    #[error("Invalid output path: {0}")]
    InvalidOutputPath(String),

    /// Infrastructure Ollama error propagation
    #[error("Ollama error: {0}")]
    Ollama(#[from] crate::infrastructure::ollama::error::OllamaError),

    /// Infrastructure File I/O error propagation
    #[error("File I/O error: {0}")]
    FileIo(#[from] crate::infrastructure::fileio::error::FileIoError),

    /// Standard I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for application layer operations
pub type Result<T> = std::result::Result<T, ApplicationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ApplicationError::ParseError("invalid syntax".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to parse architecture document: invalid syntax"
        );
    }

    #[test]
    fn test_generation_error() {
        let err = ApplicationError::GenerationError("LLM timeout".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to generate implementation plan: LLM timeout"
        );
    }

    #[test]
    fn test_write_error() {
        let err = ApplicationError::WriteError("permission denied".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to write plan to file: permission denied"
        );
    }

    #[test]
    fn test_read_architecture_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = ApplicationError::ReadArchitectureError {
            path: "/path/to/arch.md".to_string(),
            source: io_err,
        };
        assert!(err.to_string().contains("/path/to/arch.md"));
    }

    #[test]
    fn test_architecture_not_found() {
        let err = ApplicationError::ArchitectureNotFound("/path/to/arch.md".to_string());
        assert_eq!(
            err.to_string(),
            "Architecture file not found: /path/to/arch.md"
        );
    }

    #[test]
    fn test_validation_error() {
        let err = ApplicationError::ValidationError("missing phases".to_string());
        assert_eq!(err.to_string(), "Plan validation failed: missing phases");
    }

    #[test]
    fn test_ollama_unavailable() {
        let err = ApplicationError::OllamaUnavailable("connection refused".to_string());
        assert_eq!(
            err.to_string(),
            "Ollama service unavailable: connection refused"
        );
    }

    #[test]
    fn test_model_not_found() {
        let err =
            ApplicationError::ModelNotFound("llama2".to_string(), "llama3, mistral".to_string());
        assert_eq!(
            err.to_string(),
            "Model 'llama2' not found. Available models: llama3, mistral"
        );
    }

    #[test]
    fn test_cancelled() {
        let err = ApplicationError::Cancelled;
        assert_eq!(err.to_string(), "Operation cancelled by user");
    }

    #[test]
    fn test_interactive_error() {
        let err = ApplicationError::InteractiveError("input error".to_string());
        assert_eq!(err.to_string(), "Interactive session error: input error");
    }

    #[test]
    fn test_output_exists() {
        let err = ApplicationError::OutputExists("plan.md".to_string());
        assert_eq!(err.to_string(), "Output file already exists: plan.md");
    }

    #[test]
    fn test_invalid_output_path() {
        let err = ApplicationError::InvalidOutputPath("/invalid/path".to_string());
        assert_eq!(err.to_string(), "Invalid output path: /invalid/path");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let err: ApplicationError = io_err.into();
        assert!(err.to_string().contains("I/O error"));
    }
}
