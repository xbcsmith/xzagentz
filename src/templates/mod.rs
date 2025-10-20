//! Template system for managing project templates
//!
//! This module provides functionality for loading, parsing, validating, and rendering
//! project templates. Templates define collections of components with metadata and
//! placeholder values.
//!
//! # Architecture
//!
//! - `Template`: Core data structure representing a parsed template
//! - `TemplateLoader`: Loads templates from filesystem with caching
//! - `TemplateValidator`: Validates template structure and component references
//! - `PlaceholderRenderer`: Renders templates by replacing placeholders
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::templates::{Template, TemplateComponent};
//! use xzagentz::core::ComponentType;
//!
//! let template = Template {
//!     name: "rust-binary".to_string(),
//!     description: "Template for Rust CLI binary project".to_string(),
//!     version: Some("1.0.0".to_string()),
//!     author: None,
//!     metadata: Default::default(),
//!     components: vec![
//!         TemplateComponent {
//!             component_type: ComponentType::Core,
//!             name: "quick_reference".to_string(),
//!             required: true,
//!             order: None,
//!         },
//!     ],
//! };
//!
//! assert_eq!(template.name, "rust-binary");
//! assert_eq!(template.components.len(), 1);
//! ```

mod loader;
mod renderer;
mod validator;

pub use loader::TemplateLoader;
pub use renderer::PlaceholderRenderer;
pub use validator::TemplateValidator;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::ComponentType;

/// Represents a component reference in a template
///
/// Templates reference components by type and name, and can specify
/// whether the component is required and its ordering.
///
/// # Examples
///
/// ```
/// use xzagentz::templates::TemplateComponent;
/// use xzagentz::core::ComponentType;
///
/// let component = TemplateComponent {
///     component_type: ComponentType::Core,
///     name: "quick_reference".to_string(),
///     required: true,
///     order: Some(1),
/// };
///
/// assert!(component.required);
/// assert_eq!(component.order, Some(1));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateComponent {
    /// The type/category of the component
    #[serde(rename = "type")]
    pub component_type: ComponentType,

    /// The name of the component file (without extension)
    pub name: String,

    /// Whether this component is required in the template
    #[serde(default = "default_true")]
    pub required: bool,

    /// Optional ordering hint for component placement
    #[serde(default)]
    pub order: Option<usize>,
}

fn default_true() -> bool {
    true
}

impl TemplateComponent {
    /// Creates a new template component reference
    ///
    /// # Arguments
    ///
    /// * `component_type` - The type/category of the component
    /// * `name` - The component name
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateComponent;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = TemplateComponent::new(
    ///     ComponentType::Languages,
    ///     "rust".to_string()
    /// );
    ///
    /// assert_eq!(component.component_type, ComponentType::Languages);
    /// assert_eq!(component.name, "rust");
    /// assert!(component.required); // default is true
    /// ```
    pub fn new(component_type: ComponentType, name: String) -> Self {
        Self {
            component_type,
            name,
            required: true,
            order: None,
        }
    }

    /// Creates a component reference with custom required flag
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateComponent;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = TemplateComponent::with_required(
    ///     ComponentType::Tools,
    ///     "docker".to_string(),
    ///     false
    /// );
    ///
    /// assert!(!component.required);
    /// ```
    pub fn with_required(component_type: ComponentType, name: String, required: bool) -> Self {
        Self {
            component_type,
            name,
            required,
            order: None,
        }
    }

    /// Creates a component reference with ordering
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateComponent;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = TemplateComponent::with_order(
    ///     ComponentType::Core,
    ///     "overview".to_string(),
    ///     true,
    ///     1
    /// );
    ///
    /// assert_eq!(component.order, Some(1));
    /// ```
    pub fn with_order(
        component_type: ComponentType,
        name: String,
        required: bool,
        order: usize,
    ) -> Self {
        Self {
            component_type,
            name,
            required,
            order: Some(order),
        }
    }
}

/// Represents a project template
///
/// Templates define collections of components with metadata and default
/// placeholder values. They can be loaded from TOML files and used to
/// generate AGENTS.md files.
///
/// # Examples
///
/// ```
/// use xzagentz::templates::{Template, TemplateComponent};
/// use xzagentz::core::ComponentType;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// metadata.insert("category".to_string(), "development".to_string());
///
/// let template = Template {
///     name: "rust-binary".to_string(),
///     description: "Rust CLI application template".to_string(),
///     version: Some("1.0.0".to_string()),
///     author: Some("xzagentz".to_string()),
///     metadata,
///     components: vec![
///         TemplateComponent::new(ComponentType::Core, "quick_reference".to_string()),
///         TemplateComponent::new(ComponentType::Languages, "rust".to_string()),
///     ],
/// };
///
/// assert_eq!(template.components.len(), 2);
/// assert!(template.version.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    /// Template identifier name
    pub name: String,

    /// Human-readable description of the template
    pub description: String,

    /// Optional version string
    #[serde(default)]
    pub version: Option<String>,

    /// Optional author information
    #[serde(default)]
    pub author: Option<String>,

    /// Additional metadata key-value pairs
    #[serde(default)]
    pub metadata: HashMap<String, String>,

    /// List of component references
    #[serde(default)]
    pub components: Vec<TemplateComponent>,
}

impl Template {
    /// Creates a new template with required fields
    ///
    /// # Arguments
    ///
    /// * `name` - Template identifier
    /// * `description` - Template description
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::Template;
    ///
    /// let template = Template::new(
    ///     "web-service".to_string(),
    ///     "RESTful web service template".to_string()
    /// );
    ///
    /// assert_eq!(template.name, "web-service");
    /// assert!(template.components.is_empty());
    /// ```
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            version: None,
            author: None,
            metadata: HashMap::new(),
            components: Vec::new(),
        }
    }

    /// Creates a template with version
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::Template;
    ///
    /// let template = Template::with_version(
    ///     "api-service".to_string(),
    ///     "API service template".to_string(),
    ///     "2.0.0".to_string()
    /// );
    ///
    /// assert_eq!(template.version, Some("2.0.0".to_string()));
    /// ```
    pub fn with_version(name: String, description: String, version: String) -> Self {
        Self {
            name,
            description,
            version: Some(version),
            author: None,
            metadata: HashMap::new(),
            components: Vec::new(),
        }
    }

    /// Adds a component to the template
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new(
    ///     "example".to_string(),
    ///     "Example template".to_string()
    /// );
    ///
    /// template.add_component(
    ///     TemplateComponent::new(ComponentType::Core, "overview".to_string())
    /// );
    ///
    /// assert_eq!(template.components.len(), 1);
    /// ```
    pub fn add_component(&mut self, component: TemplateComponent) {
        self.components.push(component);
    }

    /// Returns required components
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(TemplateComponent::with_required(
    ///     ComponentType::Core,
    ///     "required".to_string(),
    ///     true
    /// ));
    /// template.add_component(TemplateComponent::with_required(
    ///     ComponentType::Tools,
    ///     "optional".to_string(),
    ///     false
    /// ));
    ///
    /// let required = template.required_components();
    /// assert_eq!(required.len(), 1);
    /// assert_eq!(required[0].name, "required");
    /// ```
    pub fn required_components(&self) -> Vec<&TemplateComponent> {
        self.components.iter().filter(|c| c.required).collect()
    }

    /// Returns optional components
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(TemplateComponent::with_required(
    ///     ComponentType::Core,
    ///     "required".to_string(),
    ///     true
    /// ));
    /// template.add_component(TemplateComponent::with_required(
    ///     ComponentType::Tools,
    ///     "optional".to_string(),
    ///     false
    /// ));
    ///
    /// let optional = template.optional_components();
    /// assert_eq!(optional.len(), 1);
    /// assert_eq!(optional[0].name, "optional");
    /// ```
    pub fn optional_components(&self) -> Vec<&TemplateComponent> {
        self.components.iter().filter(|c| !c.required).collect()
    }

    /// Returns components sorted by order (if specified)
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new("test".to_string(), "Test".to_string());
    /// template.add_component(TemplateComponent::with_order(
    ///     ComponentType::Core,
    ///     "third".to_string(),
    ///     true,
    ///     3
    /// ));
    /// template.add_component(TemplateComponent::with_order(
    ///     ComponentType::Core,
    ///     "first".to_string(),
    ///     true,
    ///     1
    /// ));
    ///
    /// let sorted = template.sorted_components();
    /// assert_eq!(sorted[0].name, "first");
    /// assert_eq!(sorted[1].name, "third");
    /// ```
    pub fn sorted_components(&self) -> Vec<&TemplateComponent> {
        let mut components: Vec<&TemplateComponent> = self.components.iter().collect();
        components.sort_by_key(|c| c.order.unwrap_or(usize::MAX));
        components
    }

    /// Validates the template structure
    ///
    /// Checks that:
    /// - Name is not empty
    /// - Description is not empty
    /// - At least one component is defined
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::{Template, TemplateComponent};
    /// use xzagentz::core::ComponentType;
    ///
    /// let mut template = Template::new(
    ///     "valid".to_string(),
    ///     "Valid template".to_string()
    /// );
    /// template.add_component(
    ///     TemplateComponent::new(ComponentType::Core, "test".to_string())
    /// );
    ///
    /// assert!(template.validate().is_ok());
    /// ```
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::Error::ValidationError(
                "Template name cannot be empty".to_string(),
            ));
        }

        if self.description.trim().is_empty() {
            return Err(crate::Error::ValidationError(
                "Template description cannot be empty".to_string(),
            ));
        }

        if self.components.is_empty() {
            return Err(crate::Error::ValidationError(
                "Template must have at least one component".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_component_new() {
        let component = TemplateComponent::new(ComponentType::Core, "test".to_string());
        assert_eq!(component.component_type, ComponentType::Core);
        assert_eq!(component.name, "test");
        assert!(component.required);
        assert_eq!(component.order, None);
    }

    #[test]
    fn test_template_component_with_required() {
        let component =
            TemplateComponent::with_required(ComponentType::Tools, "docker".to_string(), false);
        assert_eq!(component.component_type, ComponentType::Tools);
        assert!(!component.required);
    }

    #[test]
    fn test_template_component_with_order() {
        let component =
            TemplateComponent::with_order(ComponentType::Languages, "rust".to_string(), true, 5);
        assert_eq!(component.order, Some(5));
        assert!(component.required);
    }

    #[test]
    fn test_template_new() {
        let template = Template::new("test".to_string(), "Test template".to_string());
        assert_eq!(template.name, "test");
        assert_eq!(template.description, "Test template");
        assert!(template.version.is_none());
        assert!(template.components.is_empty());
    }

    #[test]
    fn test_template_with_version() {
        let template = Template::with_version(
            "test".to_string(),
            "Test template".to_string(),
            "1.0.0".to_string(),
        );
        assert_eq!(template.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_template_add_component() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "overview".to_string(),
        ));
        assert_eq!(template.components.len(), 1);
    }

    #[test]
    fn test_template_required_components() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_required(
            ComponentType::Core,
            "required".to_string(),
            true,
        ));
        template.add_component(TemplateComponent::with_required(
            ComponentType::Tools,
            "optional".to_string(),
            false,
        ));

        let required = template.required_components();
        assert_eq!(required.len(), 1);
        assert_eq!(required[0].name, "required");
    }

    #[test]
    fn test_template_optional_components() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_required(
            ComponentType::Core,
            "required".to_string(),
            true,
        ));
        template.add_component(TemplateComponent::with_required(
            ComponentType::Tools,
            "optional".to_string(),
            false,
        ));

        let optional = template.optional_components();
        assert_eq!(optional.len(), 1);
        assert_eq!(optional[0].name, "optional");
    }

    #[test]
    fn test_template_sorted_components() {
        let mut template = Template::new("test".to_string(), "Test".to_string());
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "third".to_string(),
            true,
            3,
        ));
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "first".to_string(),
            true,
            1,
        ));
        template.add_component(TemplateComponent::with_order(
            ComponentType::Core,
            "second".to_string(),
            true,
            2,
        ));

        let sorted = template.sorted_components();
        assert_eq!(sorted[0].name, "first");
        assert_eq!(sorted[1].name, "second");
        assert_eq!(sorted[2].name, "third");
    }

    #[test]
    fn test_template_validate_success() {
        let mut template = Template::new("valid".to_string(), "Valid template".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        assert!(template.validate().is_ok());
    }

    #[test]
    fn test_template_validate_empty_name() {
        let mut template = Template::new("".to_string(), "Valid description".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        assert!(template.validate().is_err());
    }

    #[test]
    fn test_template_validate_empty_description() {
        let mut template = Template::new("valid".to_string(), "".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "test".to_string(),
        ));

        assert!(template.validate().is_err());
    }

    #[test]
    fn test_template_validate_no_components() {
        let template = Template::new("valid".to_string(), "Valid description".to_string());

        assert!(template.validate().is_err());
    }

    #[test]
    fn test_template_component_serde() {
        let component = TemplateComponent::new(ComponentType::Core, "test".to_string());
        let serialized = toml::to_string(&component).unwrap();
        let deserialized: TemplateComponent = toml::from_str(&serialized).unwrap();
        assert_eq!(component, deserialized);
    }

    #[test]
    fn test_template_serde() {
        let mut template = Template::new("test".to_string(), "Test template".to_string());
        template.add_component(TemplateComponent::new(
            ComponentType::Core,
            "overview".to_string(),
        ));

        let serialized = toml::to_string(&template).unwrap();
        let deserialized: Template = toml::from_str(&serialized).unwrap();
        assert_eq!(template, deserialized);
    }
}
