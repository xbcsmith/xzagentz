// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Template management infrastructure
//!
//! This module provides infrastructure for managing architecture templates,
//! including file-based storage and retrieval.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::TemplateRepository;
//! use xzagentz::infrastructure::templates::FileTemplateRepository;
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let repo = FileTemplateRepository::new(PathBuf::from("templates/architecture"));
//! repo.initialize().await?;
//!
//! let templates = repo.list_templates().await?;
//! println!("Available templates: {}", templates.len());
//! # Ok(())
//! # }
//! ```

pub mod file_repository;

pub use file_repository::{FileTemplateRepository, RepositoryError};
