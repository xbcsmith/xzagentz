// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Core data structures for xzagentz
//!
//! This module contains the fundamental data types used throughout the application,
//! including component types, template configurations, and project metadata.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::core::{ComponentType, ProjectMetadata, TemplateConfig};
//!
//! // Create project metadata
//! let metadata = ProjectMetadata {
//!     name: "my-project".to_string(),
//!     project_type: "rust_binary".to_string(),
//!     language: "rust".to_string(),
//!     description: Some("A Rust binary project".to_string()),
//!     version: "0.1.0".to_string(),
//!     author: Some("John Doe".to_string()),
//! };
//!
//! // Determine component type from path
//! let component_type = ComponentType::from_path("components/core/file_extensions.md");
//! assert_eq!(component_type, Some(ComponentType::Core));
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use crate::error::{Error, Result};

/// Type of component in the AGENTS.md file structure
///
/// Components are organized into categories based on their purpose.
/// Each component type corresponds to a directory in the components structure.
///
/// # Examples
///
/// ```rust
/// use xzagentz::core::ComponentType;
/// use std::str::FromStr;
///
/// let core_type = ComponentType::Core;
/// assert_eq!(core_type.as_str(), "core");
///
/// let from_str = ComponentType::from_str("languages").unwrap();
/// assert_eq!(from_str, ComponentType::Languages);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    /// Core rules and guidelines that apply to all projects
    Core,
    /// Language-specific guidelines (Rust, Python, Go, etc.)
    Languages,
    /// Tool-specific configurations (clippy, black, prettier, etc.)
    Tools,
    /// General development practices and workflows
    General,
}

impl ComponentType {
    /// Returns the string representation of the component type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ComponentType;
    ///
    /// assert_eq!(ComponentType::Core.as_str(), "core");
    /// assert_eq!(ComponentType::Languages.as_str(), "languages");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Languages => "languages",
            Self::Tools => "tools",
            Self::General => "general",
        }
    }

    /// Determines the component type from a file path
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ComponentType;
    ///
    /// let path = "components/core/file_extensions.md";
    /// assert_eq!(ComponentType::from_path(path), Some(ComponentType::Core));
    ///
    /// let path2 = "components/languages/rust.md";
    /// assert_eq!(ComponentType::from_path(path2), Some(ComponentType::Languages));
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let path_str = path.as_ref().to_string_lossy();
        let components: Vec<&str> = path_str.split('/').collect();

        if components.len() >= 2 && components[0] == "components" {
            Self::from_str(components[1]).ok()
        } else {
            None
        }
    }

    /// Returns all component types
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ComponentType;
    ///
    /// let all_types = ComponentType::all();
    /// assert_eq!(all_types.len(), 4);
    /// assert!(all_types.contains(&ComponentType::Core));
    /// ```
    pub fn all() -> Vec<Self> {
        vec![Self::Core, Self::Languages, Self::Tools, Self::General]
    }
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ComponentType {
    type Err = Error;

    /// Parses a ComponentType from a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ComponentType;
    /// use std::str::FromStr;
    ///
    /// let core = ComponentType::from_str("core").unwrap();
    /// assert_eq!(core, ComponentType::Core);
    ///
    /// assert!(ComponentType::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "core" => Ok(Self::Core),
            "languages" => Ok(Self::Languages),
            "tools" => Ok(Self::Tools),
            "general" => Ok(Self::General),
            _ => Err(Error::other(format!("Invalid component type: {}", s))),
        }
    }
}

/// Template configuration for AGENTS.md generation
///
/// Contains metadata about a template including its components, placeholders,
/// and generation options.
///
/// # Examples
///
/// ```rust
/// use xzagentz::core::{TemplateConfig, ComponentType};
/// use std::collections::HashMap;
///
/// let mut config = TemplateConfig {
///     name: "rust_binary".to_string(),
///     version: "1.0.0".to_string(),
///     description: Some("Rust binary template".to_string()),
///     components: vec![ComponentType::Core, ComponentType::Languages],
///     placeholders: HashMap::new(),
///     sections: vec!["Quick Reference".to_string(), "Project Overview".to_string()],
/// };
///
/// config.placeholders.insert("project_name".to_string(), "my-app".to_string());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateConfig {
    /// Template name
    pub name: String,

    /// Template version
    pub version: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Component types to include
    pub components: Vec<ComponentType>,

    /// Placeholder values for template rendering
    #[serde(default)]
    pub placeholders: HashMap<String, String>,

    /// Section names to include (in order)
    pub sections: Vec<String>,
}

impl TemplateConfig {
    /// Creates a new TemplateConfig with defaults
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::TemplateConfig;
    ///
    /// let config = TemplateConfig::new("rust_binary", "1.0.0");
    /// assert_eq!(config.name, "rust_binary");
    /// assert_eq!(config.version, "1.0.0");
    /// assert!(config.components.is_empty());
    /// ```
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: None,
            components: Vec::new(),
            placeholders: HashMap::new(),
            sections: Vec::new(),
        }
    }

    /// Validates the template configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::TemplateConfig;
    ///
    /// let config = TemplateConfig::new("", "1.0.0");
    /// assert!(config.validate().is_err());
    ///
    /// let valid_config = TemplateConfig::new("rust_binary", "1.0.0");
    /// assert!(valid_config.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::configuration("Template name cannot be empty"));
        }

        if self.version.is_empty() {
            return Err(Error::configuration("Template version cannot be empty"));
        }

        Ok(())
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            components: Vec::new(),
            placeholders: HashMap::new(),
            sections: Vec::new(),
        }
    }
}

/// Project metadata for configuration and generation
///
/// Contains information about the project being managed, including
/// its type, language, and other relevant metadata.
///
/// # Examples
///
/// ```rust
/// use xzagentz::core::ProjectMetadata;
///
/// let metadata = ProjectMetadata {
///     name: "my-project".to_string(),
///     project_type: "rust_binary".to_string(),
///     language: "rust".to_string(),
///     description: Some("My project".to_string()),
///     version: "0.1.0".to_string(),
///     author: Some("Alice".to_string()),
/// };
///
/// assert!(metadata.validate().is_ok());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,

    /// Project type (e.g., "rust_binary", "web_service", "library")
    pub project_type: String,

    /// Primary programming language
    pub language: String,

    /// Optional project description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Project version
    pub version: String,

    /// Optional author name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

impl ProjectMetadata {
    /// Creates new ProjectMetadata
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ProjectMetadata;
    ///
    /// let metadata = ProjectMetadata::new(
    ///     "my-app",
    ///     "rust_binary",
    ///     "rust",
    ///     "0.1.0"
    /// );
    /// assert_eq!(metadata.name, "my-app");
    /// ```
    pub fn new(
        name: impl Into<String>,
        project_type: impl Into<String>,
        language: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            project_type: project_type.into(),
            language: language.into(),
            description: None,
            version: version.into(),
            author: None,
        }
    }

    /// Validates the project metadata
    ///
    /// # Errors
    ///
    /// Returns an error if any required field is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ProjectMetadata;
    ///
    /// let invalid = ProjectMetadata::new("", "rust_binary", "rust", "0.1.0");
    /// assert!(invalid.validate().is_err());
    ///
    /// let valid = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
    /// assert!(valid.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::InvalidProject {
                reason: "Project name cannot be empty".to_string(),
            });
        }

        if self.project_type.is_empty() {
            return Err(Error::InvalidProject {
                reason: "Project type cannot be empty".to_string(),
            });
        }

        if self.language.is_empty() {
            return Err(Error::InvalidProject {
                reason: "Project language cannot be empty".to_string(),
            });
        }

        if self.version.is_empty() {
            return Err(Error::InvalidProject {
                reason: "Project version cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    /// Returns project metadata as placeholder map
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::ProjectMetadata;
    ///
    /// let metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
    /// let placeholders = metadata.to_placeholders();
    ///
    /// assert_eq!(placeholders.get("project_name"), Some(&"my-app".to_string()));
    /// assert_eq!(placeholders.get("project_type"), Some(&"rust_binary".to_string()));
    /// ```
    pub fn to_placeholders(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("project_name".to_string(), self.name.clone());
        map.insert("project_type".to_string(), self.project_type.clone());
        map.insert("language".to_string(), self.language.clone());
        map.insert("version".to_string(), self.version.clone());

        if let Some(desc) = &self.description {
            map.insert("description".to_string(), desc.clone());
        }

        if let Some(author) = &self.author {
            map.insert("author".to_string(), author.clone());
        }

        map
    }
}

/// Project configuration stored in .xzagentz.toml
///
/// Contains all configuration for a project including metadata,
/// template information, and custom settings.
///
/// # Examples
///
/// ```rust
/// use xzagentz::core::{ProjectConfig, ProjectMetadata};
/// use chrono::Utc;
///
/// let metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
/// let config = ProjectConfig::new(metadata, "rust_binary");
///
/// assert_eq!(config.project.name, "my-app");
/// assert_eq!(config.template_name, "rust_binary");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project metadata
    pub project: ProjectMetadata,

    /// Template name used for this project
    pub template_name: String,

    /// Template version
    pub template_version: String,

    /// When the project was created
    pub created_at: DateTime<Utc>,

    /// When the configuration was last updated
    pub updated_at: DateTime<Utc>,

    /// Custom placeholder values
    #[serde(default)]
    pub placeholders: HashMap<String, String>,

    /// Selected components
    #[serde(default)]
    pub components: Vec<ComponentType>,

    /// Custom template directory path (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_dir: Option<String>,
}

impl ProjectConfig {
    /// Creates a new ProjectConfig
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::core::{ProjectConfig, ProjectMetadata};
    ///
    /// let metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
    /// let config = ProjectConfig::new(metadata, "rust_binary");
    ///
    /// assert_eq!(config.template_name, "rust_binary");
    /// ```
    pub fn new(project: ProjectMetadata, template_name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            project,
            template_name: template_name.into(),
            template_version: "1.0.0".to_string(),
            created_at: now,
            updated_at: now,
            placeholders: HashMap::new(),
            components: Vec::new(),
            template_dir: None,
        }
    }

    /// Updates the configuration timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_type_as_str() {
        assert_eq!(ComponentType::Core.as_str(), "core");
        assert_eq!(ComponentType::Languages.as_str(), "languages");
        assert_eq!(ComponentType::Tools.as_str(), "tools");
        assert_eq!(ComponentType::General.as_str(), "general");
    }

    #[test]
    fn test_component_type_from_str() {
        use std::str::FromStr;

        assert_eq!(
            ComponentType::from_str("core").unwrap(),
            ComponentType::Core
        );
        assert_eq!(
            ComponentType::from_str("languages").unwrap(),
            ComponentType::Languages
        );
        assert_eq!(
            ComponentType::from_str("tools").unwrap(),
            ComponentType::Tools
        );
        assert_eq!(
            ComponentType::from_str("general").unwrap(),
            ComponentType::General
        );
        assert!(ComponentType::from_str("invalid").is_err());
    }

    #[test]
    fn test_component_type_from_path() {
        assert_eq!(
            ComponentType::from_path("components/core/file_extensions.md"),
            Some(ComponentType::Core)
        );
        assert_eq!(
            ComponentType::from_path("components/languages/rust.md"),
            Some(ComponentType::Languages)
        );
        assert_eq!(
            ComponentType::from_path("components/tools/clippy.md"),
            Some(ComponentType::Tools)
        );
        assert_eq!(
            ComponentType::from_path("components/general/workflow.md"),
            Some(ComponentType::General)
        );
        assert_eq!(ComponentType::from_path("invalid/path.md"), None);
    }

    #[test]
    fn test_component_type_all() {
        let all = ComponentType::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&ComponentType::Core));
        assert!(all.contains(&ComponentType::Languages));
        assert!(all.contains(&ComponentType::Tools));
        assert!(all.contains(&ComponentType::General));
    }

    #[test]
    fn test_component_type_display() {
        assert_eq!(format!("{}", ComponentType::Core), "core");
        assert_eq!(format!("{}", ComponentType::Languages), "languages");
    }

    #[test]
    fn test_template_config_new() {
        let config = TemplateConfig::new("rust_binary", "1.0.0");
        assert_eq!(config.name, "rust_binary");
        assert_eq!(config.version, "1.0.0");
        assert!(config.components.is_empty());
        assert!(config.placeholders.is_empty());
    }

    #[test]
    fn test_template_config_default() {
        let config = TemplateConfig::default();
        assert_eq!(config.name, "default");
        assert_eq!(config.version, "1.0.0");
        assert!(config.components.is_empty());
    }

    #[test]
    fn test_template_config_validate() {
        let valid = TemplateConfig::new("rust_binary", "1.0.0");
        assert!(valid.validate().is_ok());

        let invalid_name = TemplateConfig::new("", "1.0.0");
        assert!(invalid_name.validate().is_err());

        let invalid_version = TemplateConfig::new("rust_binary", "");
        assert!(invalid_version.validate().is_err());
    }

    #[test]
    fn test_project_metadata_new() {
        let metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
        assert_eq!(metadata.name, "my-app");
        assert_eq!(metadata.project_type, "rust_binary");
        assert_eq!(metadata.language, "rust");
        assert_eq!(metadata.version, "0.1.0");
    }

    #[test]
    fn test_project_metadata_validation() {
        let valid = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
        assert!(valid.validate().is_ok());

        let invalid_name = ProjectMetadata::new("", "rust_binary", "rust", "0.1.0");
        assert!(invalid_name.validate().is_err());

        let invalid_type = ProjectMetadata::new("my-app", "", "rust", "0.1.0");
        assert!(invalid_type.validate().is_err());

        let invalid_lang = ProjectMetadata::new("my-app", "rust_binary", "", "0.1.0");
        assert!(invalid_lang.validate().is_err());

        let invalid_version = ProjectMetadata::new("my-app", "rust_binary", "rust", "");
        assert!(invalid_version.validate().is_err());
    }

    #[test]
    fn test_project_metadata_to_placeholders() {
        let mut metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
        metadata.description = Some("A test project".to_string());
        metadata.author = Some("Alice".to_string());

        let placeholders = metadata.to_placeholders();
        assert_eq!(
            placeholders.get("project_name"),
            Some(&"my-app".to_string())
        );
        assert_eq!(
            placeholders.get("project_type"),
            Some(&"rust_binary".to_string())
        );
        assert_eq!(placeholders.get("language"), Some(&"rust".to_string()));
        assert_eq!(placeholders.get("version"), Some(&"0.1.0".to_string()));
        assert_eq!(
            placeholders.get("description"),
            Some(&"A test project".to_string())
        );
        assert_eq!(placeholders.get("author"), Some(&"Alice".to_string()));
    }

    #[test]
    fn test_project_config_new() {
        let metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
        let config = ProjectConfig::new(metadata, "rust_binary");

        assert_eq!(config.project.name, "my-app");
        assert_eq!(config.template_name, "rust_binary");
        assert_eq!(config.template_version, "1.0.0");
    }

    #[test]
    fn test_project_config_touch() {
        let metadata = ProjectMetadata::new("my-app", "rust_binary", "rust", "0.1.0");
        let mut config = ProjectConfig::new(metadata, "rust_binary");

        let original_time = config.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        config.touch();

        assert!(config.updated_at > original_time);
    }
}
