//! File I/O infrastructure for xzagentz
//!
//! This module provides infrastructure components for reading and writing
//! files, including markdown parsing and plan serialization.
//!
//! # Architecture
//!
//! The file I/O integration consists of several components:
//!
//! - [`MarkdownPlanWriter`]: Writes plans to markdown files
//! - [`MarkdownArchitectureParser`]: Parses markdown architecture documents
//! - [`FileIoError`]: Error types for file operations
//!
//! # Examples
//!
//! ## Writing a Plan
//!
//! ```rust,no_run
//! use xzagentz::infrastructure::fileio::MarkdownPlanWriter;
//! use xzagentz::domain::planning::{PlanWriter, Plan, PlanMetadata};
//! use std::path::Path;
//! use chrono::Utc;
//! use std::path::PathBuf;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let writer = MarkdownPlanWriter::new();
//! let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("arch.md"));
//! let plan = Plan::new("Implementation Plan", "Description", vec![], metadata);
//!
//! writer.write_plan(&plan, Path::new("plan.md"))?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Parsing an Architecture Document
//!
//! ```rust
//! use xzagentz::infrastructure::fileio::MarkdownArchitectureParser;
//! use xzagentz::domain::planning::ArchitectureParser;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = MarkdownArchitectureParser::new();
//! let markdown = r#"
//! # My Architecture
//!
//! This is the architecture description.
//!
//! ## Components
//!
//! - API Server: Handles requests
//! - Database: Stores data
//!
//! ## Requirements
//!
//! - Must be scalable
//! - Must be secure
//! "#;
//!
//! let architecture = parser.parse(markdown)?;
//! println!("Parsed: {}", architecture.title());
//! println!("Components: {}", architecture.components().len());
//! # Ok(())
//! # }
//! ```
//!
//! # Error Handling
//!
//! All operations return [`FileIoError`] which provides detailed information
//! about failures:
//!
//! ```rust
//! use xzagentz::infrastructure::fileio::FileIoError;
//!
//! fn handle_error(error: FileIoError) {
//!     match error {
//!         FileIoError::FileNotFound { path } => {
//!             eprintln!("File not found: {}", path);
//!         }
//!         FileIoError::PermissionDenied { path } => {
//!             eprintln!("Permission denied: {}", path);
//!         }
//!         _ => eprintln!("Error: {}", error),
//!     }
//! }
//! ```

pub mod architecture_parser;
pub mod error;
pub mod plan_writer;

pub use architecture_parser::MarkdownArchitectureParser;
pub use error::FileIoError;
pub use plan_writer::MarkdownPlanWriter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all public types are accessible
        let _parser = MarkdownArchitectureParser::new();
        let _writer = MarkdownPlanWriter::new();
        let _error = FileIoError::file_not_found("test.md");
    }

    #[test]
    fn test_parser_creation() {
        let parser = MarkdownArchitectureParser::new();
        assert!(format!("{:?}", parser).contains("MarkdownArchitectureParser"));
    }

    #[test]
    fn test_writer_creation() {
        let writer = MarkdownPlanWriter::new();
        assert!(format!("{:?}", writer).contains("MarkdownPlanWriter"));
    }
}
