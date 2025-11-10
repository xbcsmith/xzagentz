// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Markdown utilities module
//!
//! This module provides utilities for working with markdown content,
//! including cleaning and normalization operations.
//!
//! # Modules
//!
//! - `cleaner` - Markdown content cleaning and normalization
//!
//! # Examples
//!
//! ```
//! use xzagentz::markdown::cleaner::MarkdownCleaner;
//!
//! let content = "Some markdown with issues";
//! let cleaned = MarkdownCleaner::clean(content).unwrap();
//! ```

pub mod cleaner;

// Re-export commonly used items
pub use cleaner::MarkdownCleaner;
