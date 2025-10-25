//! xzagentz - AI Agent Development Guidelines and Template Manager
//!
//! This crate provides functionality for managing AI agent development guidelines,
//! project templates, implementation plans, and prompt generation.
//!
//! # Architecture
//!
//! The crate is organized into the following modules:
//!
//! - `core`: Core data structures and types
//! - `error`: Error handling framework
//! - `config`: Project configuration management
//! - `templates`: Template loading and management
//! - `components`: Component system for AGENTS.md
//! - `plans`: Architecture and implementation plan management
//! - `prompts`: Prompt generation system
//! - `cli`: Command-line interface (when used as binary)
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::core::{ComponentType, ProjectMetadata};
//!
//! let metadata = ProjectMetadata {
//!     name: "my-project".to_string(),
//!     project_type: "rust_binary".to_string(),
//!     language: "rust".to_string(),
//!     description: Some("My project description".to_string()),
//!     version: "0.1.0".to_string(),
//!     author: Some("Author Name".to_string()),
//! };
//! ```

pub mod cli;
pub mod components;
pub mod config;
pub mod core;
pub mod error;
pub mod parser;
pub mod plans;
pub mod prompts;
pub mod templates;

// Re-export commonly used types
pub use error::{Error, Result};
