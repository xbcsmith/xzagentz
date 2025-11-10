// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Project configuration management
//!
//! This module provides functionality for managing project configuration files,
//! including metadata, template information, component lists, and project options.

use crate::config::resolution::{
    default_components_dir, default_config_dir, default_templates_dir,
};
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Project configuration file name
pub const CONFIG_FILE_NAME: &str = ".xzagentz.toml";

/// Project configuration
///
/// Stores all configuration data for a project, including metadata,
/// template information, component selections, and project-specific options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    /// Project metadata
    pub project: ProjectMetadata,
    /// Template configuration
    pub template: TemplateConfig,
    /// Component selections
    #[serde(default)]
    pub components: ComponentConfig,
    /// Placeholder values
    #[serde(default)]
    pub placeholders: HashMap<String, String>,
    /// Project options
    #[serde(default)]
    pub options: ProjectOptions,
}

/// Project metadata section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,
    /// Project type (e.g., "rust_binary", "web_service")
    #[serde(rename = "type")]
    pub project_type: String,
    /// Primary programming language
    pub language: String,
    /// Project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Project version
    #[serde(default = "default_version")]
    pub version: String,
    /// Project author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Creation timestamp
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

/// Template configuration section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateConfig {
    /// Template name
    pub name: String,
    /// Template version
    #[serde(default = "default_version")]
    pub version: String,
}

/// Component configuration section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComponentConfig {
    /// Core components
    #[serde(default)]
    pub core: Vec<String>,
    /// Language-specific components
    #[serde(default)]
    pub languages: Vec<String>,
    /// Tool-specific components
    #[serde(default)]
    pub tools: Vec<String>,
    /// General components
    #[serde(default)]
    pub general: Vec<String>,
}

/// Project options section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectOptions {
    /// Automatically update AGENTS.md on changes
    #[serde(default = "default_true")]
    pub auto_update: bool,
    /// Validate AGENTS.md on save
    #[serde(default = "default_true")]
    pub validate_on_save: bool,
    /// Create backups when modifying files
    #[serde(default = "default_true")]
    pub backup_enabled: bool,
}

impl Default for ProjectOptions {
    fn default() -> Self {
        Self {
            auto_update: true,
            validate_on_save: true,
            backup_enabled: true,
        }
    }
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_true() -> bool {
    true
}

impl ProjectConfig {
    /// Creates a new project configuration
    ///
    /// # Arguments
    ///
    /// * `name` - Project name
    /// * `project_type` - Project type
    /// * `language` - Programming language
    /// * `template_name` - Template name
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::config::ProjectConfig;
    ///
    /// let config = ProjectConfig::new(
    ///     "my-project".to_string(),
    ///     "rust_binary".to_string(),
    ///     "rust".to_string(),
    ///     "default".to_string(),
    /// );
    /// assert_eq!(config.project.name, "my-project");
    /// ```
    pub fn new(
        name: String,
        project_type: String,
        language: String,
        template_name: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            project: ProjectMetadata {
                name,
                project_type,
                language,
                description: None,
                version: default_version(),
                author: None,
                created_at: now,
                updated_at: now,
            },
            template: TemplateConfig {
                name: template_name,
                version: default_version(),
            },
            components: ComponentConfig::default(),
            placeholders: HashMap::new(),
            options: ProjectOptions::default(),
        }
    }

    /// Loads configuration from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// Returns the loaded ProjectConfig
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The TOML syntax is invalid
    /// - Required fields are missing
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::config::ProjectConfig;
    /// use std::path::Path;
    ///
    /// let config = ProjectConfig::load(Path::new(".xzagentz.toml")).unwrap();
    /// println!("Loaded project: {}", config.project.name);
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|e| {
            Error::config_error(format!(
                "Failed to read config file '{}': {}",
                path.display(),
                e
            ))
        })?;

        let config: ProjectConfig = toml::from_str(&content).map_err(|e| {
            Error::config_error(format!(
                "Failed to parse config file '{}': {}",
                path.display(),
                e
            ))
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Saves configuration to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the configuration should be saved
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::config::ProjectConfig;
    /// use std::path::Path;
    ///
    /// let config = ProjectConfig::new(
    ///     "my-project".to_string(),
    ///     "rust_binary".to_string(),
    ///     "rust".to_string(),
    ///     "default".to_string(),
    /// );
    /// config.save(Path::new(".xzagentz.toml")).unwrap();
    /// ```
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.validate()?;

        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::config_error(format!("Failed to serialize config: {}", e)))?;

        fs::write(path.as_ref(), content).map_err(|e| {
            Error::config_error(format!(
                "Failed to write config file '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Loads configuration from the current directory
    ///
    /// Looks for `.xzagentz.toml` in the current directory
    pub fn load_from_current_dir() -> Result<Self> {
        let path = PathBuf::from(CONFIG_FILE_NAME);
        Self::load(&path)
    }

    /// Saves configuration to the current directory
    pub fn save_to_current_dir(&self) -> Result<()> {
        let path = PathBuf::from(CONFIG_FILE_NAME);
        self.save(&path)
    }

    /// Validates the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project name is empty
    /// - Project type is empty
    /// - Language is empty
    /// - Template name is empty
    pub fn validate(&self) -> Result<()> {
        if self.project.name.trim().is_empty() {
            return Err(Error::validation_error("Project name cannot be empty"));
        }

        if self.project.project_type.trim().is_empty() {
            return Err(Error::validation_error("Project type cannot be empty"));
        }

        if self.project.language.trim().is_empty() {
            return Err(Error::validation_error("Project language cannot be empty"));
        }

        if self.template.name.trim().is_empty() {
            return Err(Error::validation_error("Template name cannot be empty"));
        }

        Ok(())
    }

    /// Updates the configuration with new values
    ///
    /// Updates the `updated_at` timestamp automatically
    pub fn update(&mut self) {
        self.project.updated_at = Utc::now();
    }

    /// Checks if a configuration file exists in the current directory
    pub fn exists_in_current_dir() -> bool {
        PathBuf::from(CONFIG_FILE_NAME).exists()
    }

    /// Adds a component to the configuration
    ///
    /// # Arguments
    ///
    /// * `category` - Component category (core, languages, tools, general)
    /// * `name` - Component name
    pub fn add_component(&mut self, category: &str, name: String) {
        let list = match category {
            "core" => &mut self.components.core,
            "languages" => &mut self.components.languages,
            "tools" => &mut self.components.tools,
            "general" => &mut self.components.general,
            _ => return,
        };

        if !list.contains(&name) {
            list.push(name);
        }
        self.update();
    }

    /// Adds a placeholder value
    pub fn add_placeholder(&mut self, key: String, value: String) {
        self.placeholders.insert(key, value);
        self.update();
    }

    /// Returns the default configuration directory
    ///
    /// This is a convenience method that delegates to the resolution module.
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::config::ProjectConfig;
    ///
    /// let dir = ProjectConfig::default_config_dir();
    /// assert!(dir.ends_with("xzagentz") || dir.ends_with(".config/xzagentz"));
    /// ```
    pub fn default_config_dir() -> PathBuf {
        default_config_dir()
    }

    /// Returns the default components directory
    ///
    /// This is a convenience method that delegates to the resolution module.
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::config::ProjectConfig;
    ///
    /// let dir = ProjectConfig::default_components_dir();
    /// assert!(dir.ends_with(".config/xzagentz/components"));
    /// ```
    pub fn default_components_dir() -> PathBuf {
        default_components_dir()
    }

    /// Returns the default templates directory
    ///
    /// This is a convenience method that delegates to the resolution module.
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::config::ProjectConfig;
    ///
    /// let dir = ProjectConfig::default_templates_dir();
    /// assert!(dir.ends_with(".config/xzagentz/templates"));
    /// ```
    pub fn default_templates_dir() -> PathBuf {
        default_templates_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_new_project_config() {
        let config = ProjectConfig::new(
            "test-project".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        assert_eq!(config.project.name, "test-project");
        assert_eq!(config.project.project_type, "rust_binary");
        assert_eq!(config.project.language, "rust");
        assert_eq!(config.template.name, "default");
        assert!(config.options.auto_update);
        assert!(config.options.validate_on_save);
        assert!(config.options.backup_enabled);
    }

    #[test]
    fn test_save_project_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.toml");

        let config = ProjectConfig::new(
            "test".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        let result = config.save(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());
    }

    #[test]
    fn test_load_project_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.toml");

        let mut config = ProjectConfig::new(
            "test-load".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );
        config.project.description = Some("Test description".to_string());

        config.save(&config_path).unwrap();

        let loaded = ProjectConfig::load(&config_path).unwrap();
        assert_eq!(loaded.project.name, "test-load");
        assert_eq!(
            loaded.project.description,
            Some("Test description".to_string())
        );
    }

    #[test]
    fn test_config_validation() {
        let mut config = ProjectConfig::new(
            "test".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        assert!(config.validate().is_ok());

        config.project.name = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_update() {
        let mut config = ProjectConfig::new(
            "test".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        let original_updated_at = config.project.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        config.update();

        assert!(config.project.updated_at > original_updated_at);
    }

    #[test]
    fn test_missing_config_error() {
        let result = ProjectConfig::load("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_toml_error() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.toml");

        fs::write(&config_path, "invalid toml content {{{").unwrap();

        let result = ProjectConfig::load(&config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_migration() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("old.toml");

        let minimal_config = r#"
[project]
name = "old-project"
type = "rust_binary"
language = "rust"

[template]
name = "default"
"#;

        fs::write(&config_path, minimal_config).unwrap();

        let loaded = ProjectConfig::load(&config_path).unwrap();
        assert_eq!(loaded.project.name, "old-project");
        assert!(loaded.components.core.is_empty());
        assert!(loaded.options.auto_update);
    }

    #[test]
    fn test_add_component() {
        let mut config = ProjectConfig::new(
            "test".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        config.add_component("core", "quick_reference".to_string());
        config.add_component("languages", "rust".to_string());

        assert_eq!(config.components.core.len(), 1);
        assert_eq!(config.components.languages.len(), 1);
        assert!(config
            .components
            .core
            .contains(&"quick_reference".to_string()));
    }

    #[test]
    fn test_add_component_no_duplicates() {
        let mut config = ProjectConfig::new(
            "test".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        config.add_component("core", "quick_reference".to_string());
        config.add_component("core", "quick_reference".to_string());

        assert_eq!(config.components.core.len(), 1);
    }

    #[test]
    fn test_add_placeholder() {
        let mut config = ProjectConfig::new(
            "test".to_string(),
            "rust_binary".to_string(),
            "rust".to_string(),
            "default".to_string(),
        );

        config.add_placeholder("custom_key".to_string(), "custom_value".to_string());

        assert_eq!(
            config.placeholders.get("custom_key"),
            Some(&"custom_value".to_string())
        );
    }

    #[test]
    fn test_default_options() {
        let options = ProjectOptions::default();
        assert!(options.auto_update);
        assert!(options.validate_on_save);
        assert!(options.backup_enabled);
    }

    #[test]
    fn test_component_config_default() {
        let components = ComponentConfig::default();
        assert!(components.core.is_empty());
        assert!(components.languages.is_empty());
        assert!(components.tools.is_empty());
        assert!(components.general.is_empty());
    }

    #[test]
    fn test_default_config_dir() {
        let dir = ProjectConfig::default_config_dir();
        let dir_str = dir.to_string_lossy();
        assert!(
            dir_str.contains(".config/xzagentz") || dir_str.ends_with("xzagentz"),
            "Expected config dir to contain .config/xzagentz or end with xzagentz, got: {}",
            dir_str
        );
    }

    #[test]
    fn test_default_components_dir() {
        let dir = ProjectConfig::default_components_dir();
        assert!(dir
            .to_string_lossy()
            .contains(".config/xzagentz/components"));
    }

    #[test]
    fn test_default_templates_dir() {
        let dir = ProjectConfig::default_templates_dir();
        assert!(dir.to_string_lossy().contains(".config/xzagentz/templates"));
    }
}
