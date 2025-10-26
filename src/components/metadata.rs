//! Component metadata parsing and validation
//!
//! This module provides functionality for parsing YAML frontmatter from component
//! files and validating component metadata according to the hybrid component system.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::metadata::ComponentMetadata;
//!
//! let content = r#"---
//! component:
//!   name: error_handling
//!   category: core
//!   version: 2.0.0
//!   languages: [rust, python, golang]
//! ---
//! # Error Handling
//! Content here...
//! "#;
//!
//! let (metadata, _body) = ComponentMetadata::parse_frontmatter(content)?;
//! assert_eq!(metadata.component.name, "error_handling");
//! assert_eq!(metadata.component.category, "core");
//! # Ok::<(), xzagentz::Error>(())
//! ```

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Component metadata from YAML frontmatter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentMetadata {
    /// Component configuration
    pub component: ComponentInfo,
}

/// Core component information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInfo {
    /// Component name (e.g., "error_handling", "git_conventions")
    pub name: String,

    /// Component category (core, general, languages, tools)
    pub category: String,

    /// Component version (semver format)
    pub version: String,

    /// Optional tier (essential, comprehensive) for tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Languages this component supports
    #[serde(default)]
    pub languages: Vec<String>,

    /// Section metadata
    #[serde(default)]
    pub sections: Vec<SectionMetadata>,
}

/// Metadata for a component section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionMetadata {
    /// Section identifier
    pub id: String,

    /// Whether this section has language-specific variants
    #[serde(default)]
    pub language_specific: bool,

    /// Languages this section applies to (empty = all languages)
    #[serde(default)]
    pub languages: Vec<String>,

    /// Whether this section is required
    #[serde(default)]
    pub required: bool,
}

impl ComponentMetadata {
    /// Parses YAML frontmatter from component content
    ///
    /// Extracts metadata from YAML frontmatter delimited by `---` markers
    /// and returns both the parsed metadata and the remaining content body.
    ///
    /// # Arguments
    ///
    /// * `content` - Full component file content with frontmatter
    ///
    /// # Returns
    ///
    /// Returns a tuple of (ComponentMetadata, body_content) where body_content
    /// is the markdown content after the frontmatter.
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentValidation` if:
    /// - Frontmatter delimiters are missing or malformed
    /// - YAML parsing fails
    /// - Required fields are missing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::metadata::ComponentMetadata;
    ///
    /// let content = r#"---
    /// component:
    ///   name: test
    ///   category: core
    ///   version: 1.0.0
    /// ---
    /// # Content
    /// "#;
    ///
    /// let (metadata, _body) = ComponentMetadata::parse_frontmatter(content)?;
    /// assert_eq!(metadata.component.name, "test");
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn parse_frontmatter(content: &str) -> Result<(Self, String)> {
        let trimmed = content.trim_start();

        // Check for frontmatter markers
        if !trimmed.starts_with("---") {
            return Err(Error::ComponentValidation {
                reason: "Missing YAML frontmatter delimiter (---) at start".to_string(),
            });
        }

        // Find the closing delimiter
        let after_first = &trimmed[3..]; // Skip first "---"
        let closing_pos = after_first
            .find("\n---")
            .ok_or_else(|| Error::ComponentValidation {
                reason: "Missing closing YAML frontmatter delimiter (---)".to_string(),
            })?;

        // Extract YAML content and body
        let yaml_content = &after_first[..closing_pos].trim();
        let body_start = closing_pos + 4; // Skip "\n---"
        let body = if body_start < after_first.len() {
            after_first[body_start..].trim_start().to_string()
        } else {
            String::new()
        };

        // Parse YAML
        let metadata: ComponentMetadata =
            serde_yaml::from_str(yaml_content).map_err(|e| Error::ComponentValidation {
                reason: format!("Failed to parse YAML frontmatter: {}", e),
            })?;

        // Validate required fields
        metadata.validate()?;

        Ok((metadata, body))
    }

    /// Detects if content has YAML frontmatter
    ///
    /// # Arguments
    ///
    /// * `content` - Component file content
    ///
    /// # Returns
    ///
    /// Returns `true` if the content starts with YAML frontmatter delimiters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::metadata::ComponentMetadata;
    ///
    /// assert!(ComponentMetadata::has_frontmatter("---\ntest: value\n---\nContent"));
    /// assert!(!ComponentMetadata::has_frontmatter("# No frontmatter"));
    /// ```
    pub fn has_frontmatter(content: &str) -> bool {
        content.trim_start().starts_with("---")
    }

    /// Validates component metadata
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if metadata is valid
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentValidation` if required fields are missing
    /// or invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::metadata::{ComponentMetadata, ComponentInfo};
    ///
    /// let metadata = ComponentMetadata {
    ///     component: ComponentInfo {
    ///         name: "test".to_string(),
    ///         category: "core".to_string(),
    ///         version: "1.0.0".to_string(),
    ///         tier: None,
    ///         description: None,
    ///         languages: vec![],
    ///         sections: vec![],
    ///     },
    /// };
    ///
    /// assert!(metadata.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate name
        if self.component.name.is_empty() {
            return Err(Error::ComponentValidation {
                reason: "Component name cannot be empty".to_string(),
            });
        }

        // Validate category
        let valid_categories = ["core", "general", "languages", "tools"];
        if !valid_categories.contains(&self.component.category.as_str()) {
            return Err(Error::ComponentValidation {
                reason: format!(
                    "Invalid category '{}'. Must be one of: {}",
                    self.component.category,
                    valid_categories.join(", ")
                ),
            });
        }

        // Validate version format (basic semver check)
        if !self.is_valid_semver(&self.component.version) {
            return Err(Error::ComponentValidation {
                reason: format!(
                    "Invalid version '{}'. Must follow semver format (e.g., 1.0.0)",
                    self.component.version
                ),
            });
        }

        // Validate tier if present
        if let Some(ref tier) = self.component.tier {
            let valid_tiers = ["essential", "comprehensive"];
            if !valid_tiers.contains(&tier.as_str()) {
                return Err(Error::ComponentValidation {
                    reason: format!(
                        "Invalid tier '{}'. Must be one of: {}",
                        tier,
                        valid_tiers.join(", ")
                    ),
                });
            }
        }

        Ok(())
    }

    /// Checks if a version string follows basic semver format
    fn is_valid_semver(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        parts.iter().all(|part| part.parse::<u32>().is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_success() {
        let content = r#"---
component:
  name: error_handling
  category: core
  version: 2.0.0
  languages: [rust, python]
  sections:
    - id: error_patterns
      language_specific: true
      languages: [rust, python]
---
# Error Handling
Content here
"#;

        let result = ComponentMetadata::parse_frontmatter(content);
        assert!(result.is_ok());

        let (metadata, body) = result.unwrap();
        assert_eq!(metadata.component.name, "error_handling");
        assert_eq!(metadata.component.category, "core");
        assert_eq!(metadata.component.version, "2.0.0");
        assert_eq!(metadata.component.languages.len(), 2);
        assert_eq!(metadata.component.sections.len(), 1);
        assert!(body.contains("# Error Handling"));
    }

    #[test]
    fn test_parse_frontmatter_with_tier() {
        let content = r#"---
component:
  name: git
  category: tools
  version: 1.0.0
  tier: essential
---
# Git
"#;

        let (metadata, _) = ComponentMetadata::parse_frontmatter(content).unwrap();
        assert_eq!(metadata.component.tier, Some("essential".to_string()));
    }

    #[test]
    fn test_parse_frontmatter_missing_delimiter() {
        let content = "# No frontmatter\nJust content";
        let result = ComponentMetadata::parse_frontmatter(content);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing YAML frontmatter delimiter"));
    }

    #[test]
    fn test_parse_frontmatter_missing_closing_delimiter() {
        let content = r#"---
component:
  name: test
# Missing closing ---
"#;
        let result = ComponentMetadata::parse_frontmatter(content);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing closing YAML frontmatter delimiter"));
    }

    #[test]
    fn test_parse_frontmatter_invalid_yaml() {
        let content = r#"---
invalid: : yaml: syntax
---
Content
"#;
        let result = ComponentMetadata::parse_frontmatter(content);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse YAML"));
    }

    #[test]
    fn test_has_frontmatter() {
        assert!(ComponentMetadata::has_frontmatter("---\ntest\n---\n"));
        assert!(ComponentMetadata::has_frontmatter("  ---\ntest\n---\n"));
        assert!(!ComponentMetadata::has_frontmatter("# No frontmatter"));
        assert!(!ComponentMetadata::has_frontmatter("Content only"));
    }

    #[test]
    fn test_validate_success() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "test".to_string(),
                category: "core".to_string(),
                version: "1.0.0".to_string(),
                tier: None,
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "".to_string(),
                category: "core".to_string(),
                version: "1.0.0".to_string(),
                tier: None,
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_category() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "test".to_string(),
                category: "invalid".to_string(),
                version: "1.0.0".to_string(),
                tier: None,
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        let result = metadata.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid category"));
    }

    #[test]
    fn test_validate_invalid_version() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "test".to_string(),
                category: "core".to_string(),
                version: "1.0".to_string(), // Not semver
                tier: None,
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        let result = metadata.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid version"));
    }

    #[test]
    fn test_validate_invalid_tier() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "test".to_string(),
                category: "tools".to_string(),
                version: "1.0.0".to_string(),
                tier: Some("invalid".to_string()),
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        let result = metadata.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid tier"));
    }

    #[test]
    fn test_validate_valid_tier() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "git".to_string(),
                category: "tools".to_string(),
                version: "1.0.0".to_string(),
                tier: Some("essential".to_string()),
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn test_section_metadata() {
        let content = r#"---
component:
  name: test
  category: core
  version: 1.0.0
  sections:
    - id: examples
      language_specific: true
      languages: [rust, python]
      required: true
---
Content
"#;

        let (metadata, _) = ComponentMetadata::parse_frontmatter(content).unwrap();
        assert_eq!(metadata.component.sections.len(), 1);

        let section = &metadata.component.sections[0];
        assert_eq!(section.id, "examples");
        assert!(section.language_specific);
        assert_eq!(section.languages.len(), 2);
        assert!(section.required);
    }

    #[test]
    fn test_is_valid_semver() {
        let metadata = ComponentMetadata {
            component: ComponentInfo {
                name: "test".to_string(),
                category: "core".to_string(),
                version: "1.0.0".to_string(),
                tier: None,
                description: None,
                languages: vec![],
                sections: vec![],
            },
        };

        assert!(metadata.is_valid_semver("1.0.0"));
        assert!(metadata.is_valid_semver("10.20.30"));
        assert!(metadata.is_valid_semver("0.0.1"));
        assert!(!metadata.is_valid_semver("1.0"));
        assert!(!metadata.is_valid_semver("1.0.0.0"));
        assert!(!metadata.is_valid_semver("v1.0.0"));
        assert!(!metadata.is_valid_semver("1.0.x"));
    }
}
