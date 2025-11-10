// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Architecture document writers infrastructure
//!
//! This module provides infrastructure for writing architecture documents
//! to various output formats, including Markdown.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::{
//!     ArchitectureWriter, ArchitectureDocument, ArchitectureMetadata,
//!     Overview, ArchitecturePattern,
//! };
//! use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
//! use chrono::Utc;
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let writer = MarkdownArchitectureWriter::new();
//!
//! let document = ArchitectureDocument {
//!     metadata: ArchitectureMetadata {
//!         title: "My System".to_string(),
//!         version: "1.0.0".to_string(),
//!         generated_at: Utc::now(),
//!         model_used: "llama3.2:3b".to_string(),
//!         pattern: ArchitecturePattern::Microservices,
//!         authors: vec!["Team".to_string()],
//!     },
//!     overview: Overview {
//!         description: "System description".to_string(),
//!         business_goals: vec![],
//!         constraints: vec![],
//!         assumptions: vec![],
//!     },
//!     layers: vec![],
//!     components: vec![],
//!     integrations: vec![],
//!     deployment: None,
//!     quality_attributes: vec![],
//! };
//!
//! writer.write(&document, Path::new("architecture.md")).await?;
//! # Ok(())
//! # }
//! ```

pub mod markdown_writer;

pub use markdown_writer::{MarkdownArchitectureWriter, WriterError};
