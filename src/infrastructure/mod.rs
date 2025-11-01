//! Infrastructure layer for xzagentz
//!
//! This module contains infrastructure concerns such as embedded resources,
//! external service integrations, and cross-cutting infrastructure utilities.
//!
//! # Architecture
//!
//! The infrastructure layer provides:
//! - Embedded resources (components and templates)
//! - Resource extraction and management
//! - Fallback mechanisms for missing filesystem resources
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::infrastructure::embedded::EmbeddedResources;
//!
//! let resources = EmbeddedResources::new();
//! let components = resources.list_components();
//! println!("Available components: {:?}", components);
//! ```

pub mod embedded;
pub mod ollama;

pub use embedded::EmbeddedResources;
pub use ollama::{ModelInfo, OllamaClient, OllamaConfig, OllamaError, OllamaPlanGenerator};
