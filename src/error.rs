//! Error handling framework for xzagentz
//!
//! This module provides a comprehensive error hierarchy using thiserror
//! for all error types in the application.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::error::{Error, Result};
//!
//! fn load_component(name: &str) -> Result<String> {
//!     if name.is_empty() {
//!         return Err(Error::ComponentNotFound {
//!             name: "empty".to_string(),
//!         });
//!     }
//!     Ok("component content".to_string())
//! }
//! ```

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using xzagentz Error
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for xzagentz
#[derive(Error, Debug)]
pub enum Error {
    /// Component not found error
    #[error("Component not found: {name}")]
    ComponentNotFound { name: String },

    /// Component validation error
    #[error("Component validation failed: {reason}")]
    ComponentValidation { reason: String },

    /// Template not found error
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// Template parsing error
    #[error("Template parse error: {0}")]
    TemplateParse(String),

    /// Template validation error
    #[error("Template validation error: {0}")]
    TemplateValidation(String),

    /// Configuration error
    #[error("Configuration error: {reason}")]
    Configuration { reason: String },

    /// Configuration file not found
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    /// Invalid configuration
    #[error("Invalid configuration: {reason}")]
    InvalidConfig { reason: String },

    /// File I/O error
    #[error("File I/O error at '{path}': {source}")]
    FileIo {
        path: PathBuf,
        source: std::io::Error,
    },

    /// File not found
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// Directory not found
    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: PathBuf },

    /// TOML parsing error
    #[error("Failed to parse TOML: {source}")]
    TomlParse {
        #[from]
        source: toml::de::Error,
    },

    /// TOML serialization error
    #[error("Failed to serialize TOML: {source}")]
    TomlSerialize {
        #[from]
        source: toml::ser::Error,
    },

    /// Placeholder error
    #[error("Placeholder error: {0}")]
    Placeholder(String),

    /// Missing required placeholder
    #[error("Placeholder not found: {0}")]
    PlaceholderNotFound(String),

    /// Invalid placeholder format
    #[error("Invalid placeholder format: {0}")]
    InvalidPlaceholder(String),

    /// Plan parsing error
    #[error("Failed to parse plan '{name}': {reason}")]
    PlanParse { name: String, reason: String },

    /// Plan validation error
    #[error("Plan validation failed: {reason}")]
    PlanValidation { reason: String },

    /// Prompt generation error
    #[error("Failed to generate prompt: {reason}")]
    PromptGeneration { reason: String },

    /// Invalid project structure
    #[error("Invalid project structure: {reason}")]
    InvalidProject { reason: String },

    /// Command execution error
    #[error("Command execution failed: {command}")]
    CommandFailed { command: String },

    /// Generic I/O error
    #[error("I/O error: {0}")]
    Io(String),

    /// Standard library I/O error
    #[error("I/O error: {source}")]
    StdIo {
        #[from]
        source: std::io::Error,
    },

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Regex error
    #[error("Regex error: {source}")]
    Regex {
        #[from]
        source: regex::Error,
    },

    /// Other error with context
    #[error("{message}")]
    Other { message: String },
}

impl Error {
    /// Creates a ComponentNotFound error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::error::Error;
    ///
    /// let err = Error::component_not_found("my_component");
    /// assert!(err.to_string().contains("my_component"));
    /// ```
    pub fn component_not_found(name: impl Into<String>) -> Self {
        Self::ComponentNotFound { name: name.into() }
    }

    /// Creates a TemplateNotFound error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::error::Error;
    ///
    /// let err = Error::template_not_found("rust_binary");
    /// assert!(err.to_string().contains("rust_binary"));
    /// ```
    pub fn template_not_found(name: impl Into<String>) -> Self {
        Self::TemplateNotFound(name.into())
    }

    /// Creates a TemplateParse error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::error::Error;
    ///
    /// let err = Error::template_parse("config.toml", "Invalid TOML syntax");
    /// assert!(err.to_string().contains("config.toml"));
    /// ```
    pub fn template_parse(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::TemplateParse(format!("{}: {}", name.into(), reason.into()))
    }

    /// Creates a Configuration error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::error::Error;
    ///
    /// let err = Error::configuration("Missing required field");
    /// assert!(err.to_string().contains("Missing required field"));
    /// ```
    pub fn configuration(reason: impl Into<String>) -> Self {
        Self::Configuration {
            reason: reason.into(),
        }
    }

    /// Creates a FileIo error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::error::Error;
    /// use std::path::PathBuf;
    ///
    /// let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    /// let err = Error::file_io(PathBuf::from("test.txt"), io_err);
    /// assert!(err.to_string().contains("test.txt"));
    /// ```
    pub fn file_io(path: PathBuf, source: std::io::Error) -> Self {
        Self::FileIo { path, source }
    }

    /// Creates an Other error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::error::Error;
    ///
    /// let err = Error::other("Something went wrong");
    /// assert_eq!(err.to_string(), "Something went wrong");
    /// ```
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_component_not_found_error() {
        let err = Error::component_not_found("test_component");
        assert_eq!(err.to_string(), "Component not found: test_component");
    }

    #[test]
    fn test_template_not_found_error() {
        let err = Error::template_not_found("rust_binary");
        assert_eq!(err.to_string(), "Template not found: rust_binary");
    }

    #[test]
    fn test_template_parse_error() {
        let err = Error::template_parse("config.toml", "Invalid syntax");
        assert!(err.to_string().contains("config.toml"));
        assert!(err.to_string().contains("Invalid syntax"));
    }

    #[test]
    fn test_configuration_error() {
        let err = Error::configuration("Missing required field");
        assert_eq!(
            err.to_string(),
            "Configuration error: Missing required field"
        );
    }

    #[test]
    fn test_file_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let err = Error::file_io(PathBuf::from("test.txt"), io_err);
        assert!(err.to_string().contains("test.txt"));
    }

    #[test]
    fn test_file_not_found_error() {
        let err = Error::FileNotFound {
            path: PathBuf::from("missing.txt"),
        };
        assert_eq!(err.to_string(), "File not found: missing.txt");
    }

    #[test]
    fn test_missing_placeholder_error() {
        let err = Error::PlaceholderNotFound("project_name".to_string());
        assert_eq!(err.to_string(), "Placeholder not found: project_name");
    }

    #[test]
    fn test_other_error() {
        let err = Error::other("Custom error message");
        assert_eq!(err.to_string(), "Custom error message");
    }

    #[test]
    fn test_error_chain_preserves_context() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let err = Error::file_io(PathBuf::from("/etc/config"), io_err);

        let error_string = err.to_string();
        assert!(error_string.contains("/etc/config"));
        assert!(error_string.contains("permission denied"));
    }

    #[test]
    fn test_plan_validation_error() {
        let err = Error::PlanValidation {
            reason: "Missing required section".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Plan validation failed: Missing required section"
        );
    }

    #[test]
    fn test_invalid_project_error() {
        let err = Error::InvalidProject {
            reason: "No Cargo.toml found".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Invalid project structure: No Cargo.toml found"
        );
    }

    #[test]
    fn test_helper_methods() {
        let err1 = Error::component_not_found("test");
        assert!(matches!(err1, Error::ComponentNotFound { .. }));

        let err2 = Error::template_not_found("test");
        assert!(matches!(err2, Error::TemplateNotFound(_)));

        let err3 = Error::template_parse("test", "reason");
        assert!(matches!(err3, Error::TemplateParse(_)));

        let err4 = Error::configuration("reason");
        assert!(matches!(err4, Error::Configuration { .. }));

        let err5 = Error::other("message");
        assert!(matches!(err5, Error::Other { .. }));
    }
}
