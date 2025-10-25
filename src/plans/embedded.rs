//! Embedded template system for plans and prompts
//!
//! This module provides embedded templates that are compiled into the binary
//! using the `include_str!` macro. This allows the tool to work out-of-box
//! without requiring external template files.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::plans::embedded::EmbeddedTemplates;
//!
//! let templates = EmbeddedTemplates::new();
//! let template = templates.get_plan_template("architecture_plan_rust_binary");
//! assert!(template.is_ok());
//! ```

use crate::error::{Error, Result};
use crate::plans::structures::PlanTemplate;
use std::collections::HashMap;

/// Embedded template for Rust binary architecture plans
const ARCHITECTURE_PLAN_RUST_BINARY: &str =
    include_str!("../../templates/plans/architecture_plan_rust_binary.md");

/// Category of embedded template
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateCategory {
    /// Architecture plan templates
    Plans,
    /// Prompt templates
    Prompts,
}

impl TemplateCategory {
    /// Returns the string representation of the category
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plans => "plans",
            Self::Prompts => "prompts",
        }
    }
}

/// Metadata for an embedded template
#[derive(Debug, Clone)]
pub struct EmbeddedTemplate {
    /// Template name
    pub name: String,
    /// Template category
    pub category: TemplateCategory,
    /// Template description
    pub description: String,
    /// Template version
    pub version: String,
    /// Template content
    pub content: String,
}

impl EmbeddedTemplate {
    /// Creates a new embedded template
    pub fn new(
        name: impl Into<String>,
        category: TemplateCategory,
        description: impl Into<String>,
        version: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            category,
            description: description.into(),
            version: version.into(),
            content: content.into(),
        }
    }

    /// Converts to PlanTemplate structure
    pub fn to_plan_template(&self) -> PlanTemplate {
        PlanTemplate::new(
            &self.name,
            self.category.as_str(),
            &self.description,
            &self.version,
            &self.content,
        )
    }
}

/// Registry of embedded templates
pub struct EmbeddedTemplates {
    templates: HashMap<String, EmbeddedTemplate>,
}

impl EmbeddedTemplates {
    /// Creates a new embedded templates registry with all embedded templates
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::embedded::EmbeddedTemplates;
    ///
    /// let templates = EmbeddedTemplates::new();
    /// assert!(templates.list_templates().len() > 0);
    /// ```
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Architecture plan templates
        templates.insert(
            "architecture_plan_rust_binary".to_string(),
            EmbeddedTemplate::new(
                "architecture_plan_rust_binary",
                TemplateCategory::Plans,
                "Architecture plan template for Rust binary applications",
                "1.0.0",
                ARCHITECTURE_PLAN_RUST_BINARY,
            ),
        );

        Self { templates }
    }

    /// Gets a template by name
    ///
    /// # Arguments
    ///
    /// * `name` - The template name
    ///
    /// # Returns
    ///
    /// Returns Some(template) if found, None otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::embedded::EmbeddedTemplates;
    ///
    /// let templates = EmbeddedTemplates::new();
    /// let template = templates.get("architecture_plan_rust_binary");
    /// assert!(template.is_some());
    /// ```
    pub fn get(&self, name: &str) -> Option<&EmbeddedTemplate> {
        self.templates.get(name)
    }

    /// Gets a plan template by name
    ///
    /// # Arguments
    ///
    /// * `name` - The template name
    ///
    /// # Returns
    ///
    /// Returns Ok(template) if found, Err otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::embedded::EmbeddedTemplates;
    ///
    /// let templates = EmbeddedTemplates::new();
    /// let template = templates.get_plan_template("architecture_plan_rust_binary");
    /// assert!(template.is_ok());
    /// ```
    pub fn get_plan_template(&self, name: &str) -> Result<PlanTemplate> {
        self.templates
            .get(name)
            .map(|t| t.to_plan_template())
            .ok_or_else(|| Error::template_not_found(name))
    }

    /// Lists all available template names
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::embedded::EmbeddedTemplates;
    ///
    /// let templates = EmbeddedTemplates::new();
    /// let names = templates.list_templates();
    /// assert!(names.contains(&"architecture_plan_rust_binary".to_string()));
    /// ```
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Lists templates by category
    ///
    /// # Arguments
    ///
    /// * `category` - The category to filter by
    ///
    /// # Returns
    ///
    /// Returns a vector of template names in the specified category
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::embedded::{EmbeddedTemplates, TemplateCategory};
    ///
    /// let templates = EmbeddedTemplates::new();
    /// let plans = templates.list_by_category(TemplateCategory::Plans);
    /// assert!(plans.len() > 0);
    /// ```
    pub fn list_by_category(&self, category: TemplateCategory) -> Vec<String> {
        self.templates
            .iter()
            .filter(|(_, t)| t.category == category)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Checks if a template exists
    ///
    /// # Arguments
    ///
    /// * `name` - The template name
    ///
    /// # Returns
    ///
    /// Returns true if the template exists, false otherwise
    pub fn exists(&self, name: &str) -> bool {
        self.templates.contains_key(name)
    }

    /// Returns the total number of embedded templates
    pub fn count(&self) -> usize {
        self.templates.len()
    }
}

impl Default for EmbeddedTemplates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_templates_exist() {
        let templates = EmbeddedTemplates::new();
        assert!(!templates.list_templates().is_empty());
    }

    #[test]
    fn test_load_embedded_template() {
        let templates = EmbeddedTemplates::new();
        let template = templates.get("architecture_plan_rust_binary");
        assert!(template.is_some());
        assert!(!template.unwrap().content.is_empty());
    }

    #[test]
    fn test_get_plan_template() {
        let templates = EmbeddedTemplates::new();
        let result = templates.get_plan_template("architecture_plan_rust_binary");
        assert!(result.is_ok());
        let template = result.unwrap();
        assert_eq!(template.name, "architecture_plan_rust_binary");
        assert_eq!(template.category, "plans");
    }

    #[test]
    fn test_list_templates() {
        let templates = EmbeddedTemplates::new();
        let names = templates.list_templates();
        assert!(names.contains(&"architecture_plan_rust_binary".to_string()));
    }

    #[test]
    fn test_list_by_category() {
        let templates = EmbeddedTemplates::new();
        let plans = templates.list_by_category(TemplateCategory::Plans);
        assert!(!plans.is_empty());
        assert!(plans.contains(&"architecture_plan_rust_binary".to_string()));
    }

    #[test]
    fn test_template_not_found_error() {
        let templates = EmbeddedTemplates::new();
        let result = templates.get_plan_template("nonexistent_template");
        assert!(result.is_err());
    }

    #[test]
    fn test_template_exists() {
        let templates = EmbeddedTemplates::new();
        assert!(templates.exists("architecture_plan_rust_binary"));
        assert!(!templates.exists("nonexistent"));
    }

    #[test]
    fn test_template_category_as_str() {
        assert_eq!(TemplateCategory::Plans.as_str(), "plans");
        assert_eq!(TemplateCategory::Prompts.as_str(), "prompts");
    }

    #[test]
    fn test_template_metadata_parsing() {
        let templates = EmbeddedTemplates::new();
        let template = templates.get("architecture_plan_rust_binary").unwrap();
        assert_eq!(template.name, "architecture_plan_rust_binary");
        assert_eq!(template.category, TemplateCategory::Plans);
        assert!(!template.description.is_empty());
        assert_eq!(template.version, "1.0.0");
    }

    #[test]
    fn test_embedded_template_to_plan_template() {
        let embedded = EmbeddedTemplate::new(
            "test",
            TemplateCategory::Plans,
            "Test template",
            "1.0.0",
            "# Test content",
        );
        let plan_template = embedded.to_plan_template();
        assert_eq!(plan_template.name, "test");
        assert_eq!(plan_template.category, "plans");
        assert_eq!(plan_template.content, "# Test content");
    }
}
