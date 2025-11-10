// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

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
pub mod fileio;
pub mod ollama;
pub mod templates;
pub mod writers;

pub use embedded::EmbeddedResources;
pub use fileio::{FileIoError, MarkdownArchitectureParser, MarkdownPlanWriter};
pub use ollama::{
    GeneratorError, ModelInfo, OllamaArchitectureGenerator, OllamaClient, OllamaConfig,
    OllamaError, OllamaPlanGenerator,
};
pub use templates::{FileTemplateRepository, RepositoryError};
pub use writers::{MarkdownArchitectureWriter, WriterError};
