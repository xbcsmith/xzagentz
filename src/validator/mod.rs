//! Validation module for components
//!
//! This module provides validation functionality for components, including
//! size validation, metadata validation, and structural validation.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::validator::size::{SizeValidator, SizeLimits};
//!
//! let limits = SizeLimits::default();
//! let validator = SizeValidator::new(limits);
//!
//! let content = "# Title\nSome content\n";
//! let validation = validator.validate_content("core", None, content)?;
//! assert!(validation.is_valid);
//! # Ok::<(), xzagentz::Error>(())
//! ```

pub mod size;

pub use size::{SizeLimits, SizeReport, SizeValidation, SizeValidator};
