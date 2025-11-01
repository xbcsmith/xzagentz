//! Error types for file I/O operations
//!
//! This module defines error types for file system operations including
//! reading, writing, and parsing architecture documents and plans.
//!
//! # Error Types
//!
//! - [`FileIoError`]: Main error type for file I/O operations
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::infrastructure::fileio::FileIoError;
//!
//! fn check_error() -> Result<(), FileIoError> {
//!     Err(FileIoError::FileNotFound {
//!         path: "missing.md".to_string(),
//!     })
//! }
//!
//! let result = check_error();
//! assert!(result.is_err());
//! ```

use std::io;

use thiserror::Error;

/// Errors that can occur during file I/O operations
///
/// This enum covers all error cases for reading and writing architecture
/// documents and implementation plans.
///
/// # Examples
///
/// ```rust
/// use xzagentz::infrastructure::fileio::FileIoError;
///
/// let error = FileIoError::FileNotFound {
///     path: "architecture.md".to_string(),
/// };
///
/// assert!(error.to_string().contains("architecture.md"));
/// ```
#[derive(Error, Debug)]
pub enum FileIoError {
    /// File not found
    #[error("File not found: {path}")]
    FileNotFound {
        /// The path that was not found
        path: String,
    },

    /// Permission denied
    #[error("Permission denied: {path}")]
    PermissionDenied {
        /// The path with permission issues
        path: String,
    },

    /// Failed to read file
    #[error("Failed to read file {path}: {message}")]
    ReadError {
        /// The path that failed to read
        path: String,
        /// The error message
        message: String,
    },

    /// Failed to write file
    #[error("Failed to write file {path}: {message}")]
    WriteError {
        /// The path that failed to write
        path: String,
        /// The error message
        message: String,
    },

    /// Failed to create directory
    #[error("Failed to create directory {path}: {message}")]
    DirectoryCreationError {
        /// The directory path
        path: String,
        /// The error message
        message: String,
    },

    /// Invalid file format
    #[error("Invalid file format in {path}: {message}")]
    InvalidFormat {
        /// The file path
        path: String,
        /// Description of the format issue
        message: String,
    },

    /// Missing required section
    #[error("Missing required section '{section}' in {path}")]
    MissingSection {
        /// The file path
        path: String,
        /// The missing section name
        section: String,
    },

    /// Parsing error
    #[error("Failed to parse {path}: {message}")]
    ParseError {
        /// The file path
        path: String,
        /// Description of the parsing failure
        message: String,
    },

    /// Encoding error
    #[error("Encoding error in {path}: {message}")]
    EncodingError {
        /// The file path
        path: String,
        /// The encoding error message
        message: String,
    },

    /// Path validation error
    #[error("Invalid path: {message}")]
    InvalidPath {
        /// Description of the path issue
        message: String,
    },

    /// Generic I/O error
    #[error("I/O error: {0}")]
    IoError(String),
}

impl FileIoError {
    /// Creates a file not found error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::file_not_found("missing.md");
    /// assert!(error.to_string().contains("missing.md"));
    /// ```
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound { path: path.into() }
    }

    /// Creates a permission denied error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::permission_denied("/root/file.md");
    /// assert!(error.to_string().contains("Permission denied"));
    /// ```
    pub fn permission_denied(path: impl Into<String>) -> Self {
        Self::PermissionDenied { path: path.into() }
    }

    /// Creates a read error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::read_error("file.md", "corrupted data");
    /// assert!(error.to_string().contains("Failed to read"));
    /// ```
    pub fn read_error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ReadError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Creates a write error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::write_error("file.md", "disk full");
    /// assert!(error.to_string().contains("Failed to write"));
    /// ```
    pub fn write_error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::WriteError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Creates a parse error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::parse_error("file.md", "invalid syntax");
    /// assert!(error.to_string().contains("Failed to parse"));
    /// ```
    pub fn parse_error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ParseError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Creates an invalid format error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::invalid_format("file.md", "not markdown");
    /// assert!(error.to_string().contains("Invalid file format"));
    /// ```
    pub fn invalid_format(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidFormat {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Creates a missing section error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::FileIoError;
    ///
    /// let error = FileIoError::missing_section("arch.md", "Overview");
    /// assert!(error.to_string().contains("Missing required section"));
    /// ```
    pub fn missing_section(path: impl Into<String>, section: impl Into<String>) -> Self {
        Self::MissingSection {
            path: path.into(),
            section: section.into(),
        }
    }
}

impl From<io::Error> for FileIoError {
    fn from(error: io::Error) -> Self {
        use io::ErrorKind;

        match error.kind() {
            ErrorKind::NotFound => Self::FileNotFound {
                path: "unknown".to_string(),
            },
            ErrorKind::PermissionDenied => Self::PermissionDenied {
                path: "unknown".to_string(),
            },
            _ => Self::IoError(error.to_string()),
        }
    }
}

impl From<std::string::FromUtf8Error> for FileIoError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::EncodingError {
            path: "unknown".to_string(),
            message: error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_not_found_error() {
        let error = FileIoError::file_not_found("test.md");
        assert!(error.to_string().contains("test.md"));
        assert!(error.to_string().contains("not found"));
    }

    #[test]
    fn test_permission_denied_error() {
        let error = FileIoError::permission_denied("/root/file.md");
        assert!(error.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_read_error() {
        let error = FileIoError::read_error("file.md", "corrupt");
        assert!(error.to_string().contains("Failed to read"));
        assert!(error.to_string().contains("file.md"));
        assert!(error.to_string().contains("corrupt"));
    }

    #[test]
    fn test_write_error() {
        let error = FileIoError::write_error("output.md", "disk full");
        assert!(error.to_string().contains("Failed to write"));
        assert!(error.to_string().contains("output.md"));
    }

    #[test]
    fn test_parse_error() {
        let error = FileIoError::parse_error("doc.md", "invalid syntax");
        assert!(error.to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_invalid_format_error() {
        let error = FileIoError::invalid_format("file.txt", "not markdown");
        assert!(error.to_string().contains("Invalid file format"));
    }

    #[test]
    fn test_missing_section_error() {
        let error = FileIoError::missing_section("arch.md", "Overview");
        assert!(error.to_string().contains("Missing required section"));
        assert!(error.to_string().contains("Overview"));
    }

    #[test]
    fn test_from_io_error_not_found() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let file_error: FileIoError = io_error.into();
        assert!(matches!(file_error, FileIoError::FileNotFound { .. }));
    }

    #[test]
    fn test_from_io_error_permission_denied() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "no permission");
        let file_error: FileIoError = io_error.into();
        assert!(matches!(file_error, FileIoError::PermissionDenied { .. }));
    }

    #[test]
    fn test_from_io_error_other() {
        let io_error = io::Error::new(io::ErrorKind::Other, "something went wrong");
        let file_error: FileIoError = io_error.into();
        assert!(matches!(file_error, FileIoError::IoError(_)));
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = FileIoError::file_not_found("test.md");
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_debug_implementation() {
        let error = FileIoError::file_not_found("test.md");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("FileNotFound"));
    }

    #[test]
    fn test_display_implementation() {
        let error = FileIoError::file_not_found("test.md");
        let display_str = format!("{}", error);
        assert!(display_str.contains("test.md"));
    }
}
