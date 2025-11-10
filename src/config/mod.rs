// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Configuration module for project settings and metadata
//!
//! This module provides functionality for managing project configuration,
//! including saving, loading, and validating configuration files.

pub mod app_config;
pub mod project;
pub mod resolution;

pub use app_config::{
    AppConfig, AppConfigBuilder, ConfigError, InteractiveConfig, OllamaConfig, PlanningConfig,
};
pub use project::{ProjectConfig, ProjectOptions};
pub use resolution::{
    default_components_dir, default_config_dir, default_resource_dirs, default_templates_dir,
    resolve_config_dir,
};
