//! Configuration module for project settings and metadata
//!
//! This module provides functionality for managing project configuration,
//! including saving, loading, and validating configuration files.

pub mod project;
pub mod resolution;

pub use project::{ProjectConfig, ProjectOptions};
pub use resolution::{
    default_components_dir, default_config_dir, default_resource_dirs, default_templates_dir,
    resolve_config_dir,
};
