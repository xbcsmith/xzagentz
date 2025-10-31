//! Component system for managing AGENTS.md sections
//!
//! This module provides functionality for loading, validating, and managing
//! components that make up the AGENTS.md file. Components are organized by
//! type (core, languages, tools, general) and can be loaded from embedded
//! resources or custom filesystem locations.
//!
//! # Architecture
//!
//! - `Component`: Main data structure representing a component
//! - `ComponentLoader`: Loads components from filesystem or embedded resources
//! - `ComponentValidator`: Validates component structure and content
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::{Component, ComponentLoader};
//! use xzagentz::core::ComponentType;
//!
//! let loader = ComponentLoader::new(None);
//! // Load a component (would need actual files/embedded resources)
//! // let component = loader.load("rust_standards", ComponentType::Core)?;
//! ```

use crate::core::ComponentType;
use std::collections::HashMap;

pub mod language_filter;
pub mod loader;
pub mod metadata;
pub mod renderer;
pub mod resolution;
pub mod validator;

pub use language_filter::{LanguageFilter, LanguageSection};
pub use loader::ComponentLoader;
pub use metadata::{ComponentInfo, ComponentMetadata, SectionMetadata};
pub use renderer::{ComponentRenderer, RenderConfig, RenderStats};
pub use resolution::{default_components_dir, resolve_component_dir, ResourceSource};
pub use validator::ComponentValidator;

/// Represents a single component section for AGENTS.md
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    /// Component name (e.g., "rust_standards", "git_conventions")
    pub name: String,

    /// Component type/category
    pub component_type: ComponentType,

    /// Markdown content of the component
    pub content: String,

    /// Component metadata (optional key-value pairs)
    pub metadata: HashMap<String, String>,
}

impl Component {
    /// Creates a new Component
    ///
    /// # Arguments
    ///
    /// * `name` - Component name (lowercase_snake_case)
    /// * `component_type` - Type/category of the component
    /// * `content` - Markdown content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::Component;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = Component::new(
    ///     "rust_standards",
    ///     ComponentType::Core,
    ///     "# Rust Standards\n\nContent here..."
    /// );
    /// ```
    pub fn new(
        name: impl Into<String>,
        component_type: ComponentType,
        content: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            component_type,
            content: content.into(),
            metadata: HashMap::new(),
        }
    }

    /// Creates a new Component with metadata
    ///
    /// # Arguments
    ///
    /// * `name` - Component name
    /// * `component_type` - Type/category of the component
    /// * `content` - Markdown content
    /// * `metadata` - Component metadata
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::Component;
    /// use xzagentz::core::ComponentType;
    /// use std::collections::HashMap;
    ///
    /// let mut metadata = HashMap::new();
    /// metadata.insert("version".to_string(), "1.0".to_string());
    ///
    /// let component = Component::with_metadata(
    ///     "rust_standards",
    ///     ComponentType::Core,
    ///     "# Content",
    ///     metadata
    /// );
    /// ```
    pub fn with_metadata(
        name: impl Into<String>,
        component_type: ComponentType,
        content: impl Into<String>,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            name: name.into(),
            component_type,
            content: content.into(),
            metadata,
        }
    }

    /// Returns the component's heading level (number of # characters)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::Component;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = Component::new(
    ///     "test",
    ///     ComponentType::Core,
    ///     "## Heading\n\nContent"
    /// );
    ///
    /// assert_eq!(component.heading_level(), Some(2));
    /// ```
    pub fn heading_level(&self) -> Option<usize> {
        self.content
            .lines()
            .find(|line| line.trim_start().starts_with('#'))
            .map(|line| line.trim_start().chars().take_while(|&c| c == '#').count())
    }

    /// Extracts the title from the component content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::Component;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = Component::new(
    ///     "test",
    ///     ComponentType::Core,
    ///     "## Rust Standards\n\nContent"
    /// );
    ///
    /// assert_eq!(component.title(), Some("Rust Standards".to_string()));
    /// ```
    pub fn title(&self) -> Option<String> {
        self.content
            .lines()
            .find(|line| line.trim_start().starts_with('#'))
            .map(|line| line.trim_start().trim_start_matches('#').trim().to_string())
    }

    /// Returns the word count of the component content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::Component;
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = Component::new(
    ///     "test",
    ///     ComponentType::Core,
    ///     "This has five words total"
    /// );
    ///
    /// assert_eq!(component.word_count(), 5);
    /// ```
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    /// Checks if the component is empty (no content or only whitespace)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::Component;
    /// use xzagentz::core::ComponentType;
    ///
    /// let empty = Component::new("test", ComponentType::Core, "   ");
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = Component::new("test", ComponentType::Core, "Content");
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_new() {
        let component = Component::new(
            "rust_standards",
            ComponentType::Core,
            "# Rust Standards\n\nContent here",
        );

        assert_eq!(component.name, "rust_standards");
        assert_eq!(component.component_type, ComponentType::Core);
        assert!(component.content.contains("Rust Standards"));
        assert!(component.metadata.is_empty());
    }

    #[test]
    fn test_component_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());
        metadata.insert("author".to_string(), "test".to_string());

        let component = Component::with_metadata(
            "test_component",
            ComponentType::Languages,
            "Content",
            metadata.clone(),
        );

        assert_eq!(component.metadata.len(), 2);
        assert_eq!(component.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_component_heading_level() {
        let component1 = Component::new("test", ComponentType::Core, "# Level 1");
        assert_eq!(component1.heading_level(), Some(1));

        let component2 = Component::new("test", ComponentType::Core, "## Level 2");
        assert_eq!(component2.heading_level(), Some(2));

        let component3 = Component::new("test", ComponentType::Core, "### Level 3");
        assert_eq!(component3.heading_level(), Some(3));

        let component4 = Component::new("test", ComponentType::Core, "No heading");
        assert_eq!(component4.heading_level(), None);
    }

    #[test]
    fn test_component_title() {
        let component = Component::new(
            "test",
            ComponentType::Core,
            "## Rust Coding Standards\n\nContent",
        );
        assert_eq!(component.title(), Some("Rust Coding Standards".to_string()));

        let no_title = Component::new("test", ComponentType::Core, "No heading here");
        assert_eq!(no_title.title(), None);
    }

    #[test]
    fn test_component_word_count() {
        let component = Component::new(
            "test",
            ComponentType::Core,
            "This is a test with seven words",
        );
        assert_eq!(component.word_count(), 7);

        let empty = Component::new("test", ComponentType::Core, "");
        assert_eq!(empty.word_count(), 0);
    }

    #[test]
    fn test_component_is_empty() {
        let empty1 = Component::new("test", ComponentType::Core, "");
        assert!(empty1.is_empty());

        let empty2 = Component::new("test", ComponentType::Core, "   \n  \t  ");
        assert!(empty2.is_empty());

        let non_empty = Component::new("test", ComponentType::Core, "Content");
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_component_clone() {
        let original = Component::new("test", ComponentType::Core, "Content");
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.name, cloned.name);
        assert_eq!(original.component_type, cloned.component_type);
        assert_eq!(original.content, cloned.content);
    }
}
