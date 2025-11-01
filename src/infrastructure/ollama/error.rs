//! Error types for Ollama infrastructure
//!
//! This module defines error types specific to Ollama HTTP client operations
//! and LLM interactions.
//!
//! # Error Types
//!
//! - [`OllamaError`]: Main error type for Ollama operations
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::infrastructure::ollama::OllamaError;
//!
//! fn check_error() -> Result<(), OllamaError> {
//!     Err(OllamaError::ServiceUnavailable {
//!         message: "Ollama service is not running".to_string(),
//!     })
//! }
//!
//! let result = check_error();
//! assert!(result.is_err());
//! ```

use thiserror::Error;

/// Errors that can occur during Ollama operations
///
/// This enum covers all error cases for Ollama HTTP client interactions,
/// including network errors, service errors, and response parsing failures.
///
/// # Examples
///
/// ```rust
/// use xzagentz::infrastructure::ollama::OllamaError;
///
/// let error = OllamaError::ServiceUnavailable {
///     message: "Connection refused".to_string(),
/// };
///
/// assert!(error.to_string().contains("Connection refused"));
/// ```
#[derive(Error, Debug)]
pub enum OllamaError {
    /// HTTP request failed
    #[error("HTTP request failed: {message}")]
    HttpError {
        /// The underlying error message
        message: String,
    },

    /// Ollama service is unavailable
    #[error("Ollama service unavailable: {message}")]
    ServiceUnavailable {
        /// Details about the service availability issue
        message: String,
    },

    /// Invalid response from Ollama
    #[error("Invalid response from Ollama: {message}")]
    InvalidResponse {
        /// Description of the response issue
        message: String,
    },

    /// Model not found
    #[error("Model '{model}' not found")]
    ModelNotFound {
        /// The name of the model that was not found
        model: String,
    },

    /// Request timeout
    #[error("Request timed out after {seconds} seconds")]
    Timeout {
        /// Number of seconds before timeout
        seconds: u64,
    },

    /// JSON serialization/deserialization error
    #[error("JSON error: {message}")]
    JsonError {
        /// The JSON error message
        message: String,
    },

    /// Plan generation error
    #[error("Plan generation failed: {message}")]
    GenerationError {
        /// Description of the generation failure
        message: String,
    },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError {
        /// Description of the configuration issue
        message: String,
    },

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {seconds} seconds")]
    RateLimitExceeded {
        /// Number of seconds to wait before retrying
        seconds: u64,
    },

    /// Generic error for wrapping other error types
    #[error("Ollama error: {0}")]
    Other(String),
}

impl OllamaError {
    /// Creates an HTTP error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::http("Connection reset by peer");
    /// assert!(error.to_string().contains("Connection reset"));
    /// ```
    pub fn http(message: impl Into<String>) -> Self {
        Self::HttpError {
            message: message.into(),
        }
    }

    /// Creates a service unavailable error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::service_unavailable("Service not running");
    /// assert!(error.to_string().contains("unavailable"));
    /// ```
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            message: message.into(),
        }
    }

    /// Creates an invalid response error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::invalid_response("Missing required field");
    /// assert!(error.to_string().contains("Invalid response"));
    /// ```
    pub fn invalid_response(message: impl Into<String>) -> Self {
        Self::InvalidResponse {
            message: message.into(),
        }
    }

    /// Creates a model not found error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::model_not_found("llama3.2:3b");
    /// assert!(error.to_string().contains("llama3.2:3b"));
    /// ```
    pub fn model_not_found(model: impl Into<String>) -> Self {
        Self::ModelNotFound {
            model: model.into(),
        }
    }

    /// Creates a timeout error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::timeout(30);
    /// assert!(error.to_string().contains("30 seconds"));
    /// ```
    pub fn timeout(seconds: u64) -> Self {
        Self::Timeout { seconds }
    }

    /// Creates a JSON error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::json("Expected string, got number");
    /// assert!(error.to_string().contains("JSON error"));
    /// ```
    pub fn json(message: impl Into<String>) -> Self {
        Self::JsonError {
            message: message.into(),
        }
    }

    /// Creates a generation error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::generation("Failed to parse plan");
    /// assert!(error.to_string().contains("generation failed"));
    /// ```
    pub fn generation(message: impl Into<String>) -> Self {
        Self::GenerationError {
            message: message.into(),
        }
    }

    /// Creates a configuration error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let error = OllamaError::config("Invalid base URL");
    /// assert!(error.to_string().contains("Configuration error"));
    /// ```
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }

    /// Checks if the error is retryable
    ///
    /// Some errors like temporary network failures or rate limits
    /// can be retried, while others like invalid models should not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaError;
    ///
    /// let timeout = OllamaError::timeout(30);
    /// assert!(timeout.is_retryable());
    ///
    /// let not_found = OllamaError::model_not_found("invalid");
    /// assert!(!not_found.is_retryable());
    /// ```
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout { .. }
                | Self::ServiceUnavailable { .. }
                | Self::RateLimitExceeded { .. }
                | Self::HttpError { .. }
        )
    }
}

impl From<reqwest::Error> for OllamaError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::Timeout { seconds: 30 }
        } else if error.is_connect() {
            Self::ServiceUnavailable {
                message: format!("Failed to connect: {}", error),
            }
        } else {
            Self::HttpError {
                message: error.to_string(),
            }
        }
    }
}

impl From<serde_json::Error> for OllamaError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError {
            message: error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_error_creation() {
        let error = OllamaError::http("connection failed");
        assert!(error.to_string().contains("connection failed"));
    }

    #[test]
    fn test_service_unavailable_error() {
        let error = OllamaError::service_unavailable("not running");
        assert!(error.to_string().contains("unavailable"));
    }

    #[test]
    fn test_invalid_response_error() {
        let error = OllamaError::invalid_response("bad format");
        assert!(error.to_string().contains("Invalid response"));
    }

    #[test]
    fn test_model_not_found_error() {
        let error = OllamaError::model_not_found("llama3:latest");
        assert!(error.to_string().contains("llama3:latest"));
        assert!(error.to_string().contains("not found"));
    }

    #[test]
    fn test_timeout_error() {
        let error = OllamaError::timeout(60);
        assert!(error.to_string().contains("60 seconds"));
    }

    #[test]
    fn test_json_error() {
        let error = OllamaError::json("parse failed");
        assert!(error.to_string().contains("JSON error"));
    }

    #[test]
    fn test_generation_error() {
        let error = OllamaError::generation("invalid plan");
        assert!(error.to_string().contains("generation failed"));
    }

    #[test]
    fn test_config_error() {
        let error = OllamaError::config("bad url");
        assert!(error.to_string().contains("Configuration error"));
    }

    #[test]
    fn test_is_retryable_timeout() {
        let error = OllamaError::timeout(30);
        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_service_unavailable() {
        let error = OllamaError::service_unavailable("down");
        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_rate_limit() {
        let error = OllamaError::RateLimitExceeded { seconds: 60 };
        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_model_not_found() {
        let error = OllamaError::model_not_found("invalid");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_is_not_retryable_config() {
        let error = OllamaError::config("bad config");
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(json_error.is_err());

        let ollama_error: OllamaError = json_error.unwrap_err().into();
        assert!(matches!(ollama_error, OllamaError::JsonError { .. }));
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = OllamaError::http("test");
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_debug_implementation() {
        let error = OllamaError::timeout(30);
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Timeout"));
    }

    #[test]
    fn test_display_implementation() {
        let error = OllamaError::model_not_found("test-model");
        let display_str = format!("{}", error);
        assert!(display_str.contains("test-model"));
        assert!(display_str.contains("not found"));
    }
}
