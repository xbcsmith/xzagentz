//! Architecture document domain models
//!
//! This module defines the domain models for representing architecture documents
//! that serve as input for implementation plan generation.
//!
//! # Models
//!
//! - [`ArchitectureDocument`]: The root document containing sections and components
//! - [`Section`]: A hierarchical section within the document
//! - [`Component`]: A system component described in the architecture
//! - [`Requirement`]: A functional or non-functional requirement
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::{ArchitectureDocument, Section, Component};
//!
//! let section = Section::new("Introduction", 1)
//!     .with_content("This document describes the system architecture");
//!
//! let component = Component::new("API Gateway", "api-gateway")
//!     .with_description("Handles all incoming HTTP requests");
//!
//! let doc = ArchitectureDocument::new("System Architecture")
//!     .with_sections(vec![section])
//!     .with_components(vec![component]);
//!
//! assert_eq!(doc.title(), "System Architecture");
//! assert_eq!(doc.sections().len(), 1);
//! ```

use serde::{Deserialize, Serialize};

/// Represents a parsed architecture document
///
/// An ArchitectureDocument is the structured representation of an architecture
/// specification, typically parsed from markdown. It contains hierarchical
/// sections, identified components, and extracted requirements.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::ArchitectureDocument;
///
/// let doc = ArchitectureDocument::new("E-commerce Platform Architecture")
///     .with_description("High-level architecture for e-commerce system");
///
/// assert_eq!(doc.title(), "E-commerce Platform Architecture");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureDocument {
    title: String,
    description: Option<String>,
    sections: Vec<Section>,
    components: Vec<Component>,
    requirements: Vec<Requirement>,
}

impl ArchitectureDocument {
    /// Creates a new architecture document
    ///
    /// # Arguments
    ///
    /// * `title` - The document title
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::ArchitectureDocument;
    ///
    /// let doc = ArchitectureDocument::new("My Architecture");
    /// assert_eq!(doc.title(), "My Architecture");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            sections: Vec::new(),
            components: Vec::new(),
            requirements: Vec::new(),
        }
    }

    /// Sets the description (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::ArchitectureDocument;
    ///
    /// let doc = ArchitectureDocument::new("Title")
    ///     .with_description("Description text");
    ///
    /// assert_eq!(doc.description(), Some("Description text"));
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the sections (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{ArchitectureDocument, Section};
    ///
    /// let section = Section::new("Overview", 1);
    /// let doc = ArchitectureDocument::new("Title")
    ///     .with_sections(vec![section]);
    ///
    /// assert_eq!(doc.sections().len(), 1);
    /// ```
    pub fn with_sections(mut self, sections: Vec<Section>) -> Self {
        self.sections = sections;
        self
    }

    /// Sets the components (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{ArchitectureDocument, Component};
    ///
    /// let component = Component::new("API", "api");
    /// let doc = ArchitectureDocument::new("Title")
    ///     .with_components(vec![component]);
    ///
    /// assert_eq!(doc.components().len(), 1);
    /// ```
    pub fn with_components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    /// Sets the requirements (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{ArchitectureDocument, Requirement};
    ///
    /// let req = Requirement::new("req-1", "System must be scalable");
    /// let doc = ArchitectureDocument::new("Title")
    ///     .with_requirements(vec![req]);
    ///
    /// assert_eq!(doc.requirements().len(), 1);
    /// ```
    pub fn with_requirements(mut self, requirements: Vec<Requirement>) -> Self {
        self.requirements = requirements;
        self
    }

    /// Returns the document title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the document description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns a reference to the sections
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    /// Returns a reference to the components
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Returns a reference to the requirements
    pub fn requirements(&self) -> &[Requirement] {
        &self.requirements
    }

    /// Adds a section to the document
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{ArchitectureDocument, Section};
    ///
    /// let mut doc = ArchitectureDocument::new("Title");
    /// doc.add_section(Section::new("New Section", 1));
    ///
    /// assert_eq!(doc.sections().len(), 1);
    /// ```
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// Adds a component to the document
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{ArchitectureDocument, Component};
    ///
    /// let mut doc = ArchitectureDocument::new("Title");
    /// doc.add_component(Component::new("Database", "db"));
    ///
    /// assert_eq!(doc.components().len(), 1);
    /// ```
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    /// Adds a requirement to the document
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{ArchitectureDocument, Requirement};
    ///
    /// let mut doc = ArchitectureDocument::new("Title");
    /// doc.add_requirement(Requirement::new("req-1", "Must be fast"));
    ///
    /// assert_eq!(doc.requirements().len(), 1);
    /// ```
    pub fn add_requirement(&mut self, requirement: Requirement) {
        self.requirements.push(requirement);
    }

    /// Validates the architecture document
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, Err with reason if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::ArchitectureDocument;
    ///
    /// let doc = ArchitectureDocument::new("Valid Title");
    /// assert!(doc.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Document title cannot be empty".to_string());
        }

        // Validate all sections
        for section in &self.sections {
            section.validate()?;
        }

        // Validate all components
        for component in &self.components {
            component.validate()?;
        }

        // Validate all requirements
        for requirement in &self.requirements {
            requirement.validate()?;
        }

        Ok(())
    }
}

/// A section within the architecture document
///
/// Sections represent the hierarchical structure of the document, typically
/// corresponding to markdown headings. Sections can contain subsections.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::Section;
///
/// let parent = Section::new("Architecture Overview", 1)
///     .with_content("This section describes the high-level architecture");
///
/// let child = Section::new("Components", 2)
///     .with_content("Description of system components");
///
/// let parent = parent.with_subsections(vec![child]);
///
/// assert_eq!(parent.subsections().len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    heading: String,
    level: u8,
    content: String,
    subsections: Vec<Section>,
}

impl Section {
    /// Creates a new section
    ///
    /// # Arguments
    ///
    /// * `heading` - The section heading text
    /// * `level` - The heading level (1-6 for markdown)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Section;
    ///
    /// let section = Section::new("Introduction", 1);
    /// assert_eq!(section.heading(), "Introduction");
    /// assert_eq!(section.level(), 1);
    /// ```
    pub fn new(heading: impl Into<String>, level: u8) -> Self {
        Self {
            heading: heading.into(),
            level,
            content: String::new(),
            subsections: Vec::new(),
        }
    }

    /// Sets the content (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Section;
    ///
    /// let section = Section::new("Title", 1)
    ///     .with_content("Content text");
    ///
    /// assert_eq!(section.content(), "Content text");
    /// ```
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Sets the subsections (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Section;
    ///
    /// let subsection = Section::new("Subsection", 2);
    /// let section = Section::new("Title", 1)
    ///     .with_subsections(vec![subsection]);
    ///
    /// assert_eq!(section.subsections().len(), 1);
    /// ```
    pub fn with_subsections(mut self, subsections: Vec<Section>) -> Self {
        self.subsections = subsections;
        self
    }

    /// Returns the heading text
    pub fn heading(&self) -> &str {
        &self.heading
    }

    /// Returns the heading level
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Returns the content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns a reference to the subsections
    pub fn subsections(&self) -> &[Section] {
        &self.subsections
    }

    /// Adds a subsection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Section;
    ///
    /// let mut section = Section::new("Parent", 1);
    /// section.add_subsection(Section::new("Child", 2));
    ///
    /// assert_eq!(section.subsections().len(), 1);
    /// ```
    pub fn add_subsection(&mut self, subsection: Section) {
        self.subsections.push(subsection);
    }

    /// Validates the section
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, Err with reason if invalid
    pub fn validate(&self) -> Result<(), String> {
        if self.heading.trim().is_empty() {
            return Err("Section heading cannot be empty".to_string());
        }

        if self.level == 0 || self.level > 6 {
            return Err(format!(
                "Section level must be between 1 and 6, got {}",
                self.level
            ));
        }

        // Validate all subsections
        for subsection in &self.subsections {
            subsection.validate()?;
        }

        Ok(())
    }
}

/// A system component described in the architecture
///
/// Components represent logical or physical parts of the system being designed.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::Component;
///
/// let component = Component::new("Authentication Service", "auth-service")
///     .with_description("Handles user authentication and authorization")
///     .with_technology("Rust with Actix-web");
///
/// assert_eq!(component.name(), "Authentication Service");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Component {
    name: String,
    id: String,
    description: Option<String>,
    technology: Option<String>,
}

impl Component {
    /// Creates a new component
    ///
    /// # Arguments
    ///
    /// * `name` - The component name
    /// * `id` - A unique identifier for the component
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Component;
    ///
    /// let component = Component::new("API Gateway", "api-gateway");
    /// assert_eq!(component.name(), "API Gateway");
    /// assert_eq!(component.id(), "api-gateway");
    /// ```
    pub fn new(name: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: id.into(),
            description: None,
            technology: None,
        }
    }

    /// Sets the description (builder pattern)
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the technology (builder pattern)
    pub fn with_technology(mut self, technology: impl Into<String>) -> Self {
        self.technology = Some(technology.into());
        self
    }

    /// Returns the component name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the component ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the component description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the technology
    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    /// Validates the component
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Component name cannot be empty".to_string());
        }

        if self.id.trim().is_empty() {
            return Err("Component ID cannot be empty".to_string());
        }

        Ok(())
    }
}

/// A requirement from the architecture
///
/// Requirements can be functional or non-functional specifications that
/// the implementation must satisfy.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::Requirement;
///
/// let requirement = Requirement::new(
///     "req-performance-1",
///     "System must handle 10,000 requests per second",
/// )
/// .with_category("Performance");
///
/// assert_eq!(requirement.id(), "req-performance-1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Requirement {
    id: String,
    description: String,
    category: Option<String>,
    priority: Option<String>,
}

impl Requirement {
    /// Creates a new requirement
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the requirement
    /// * `description` - Description of what is required
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Requirement;
    ///
    /// let req = Requirement::new("req-1", "Must be secure");
    /// assert_eq!(req.id(), "req-1");
    /// ```
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            category: None,
            priority: None,
        }
    }

    /// Sets the category (builder pattern)
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the priority (builder pattern)
    pub fn with_priority(mut self, priority: impl Into<String>) -> Self {
        self.priority = Some(priority.into());
        self
    }

    /// Returns the requirement ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the requirement description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the category
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    /// Returns the priority
    pub fn priority(&self) -> Option<&str> {
        self.priority.as_deref()
    }

    /// Validates the requirement
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("Requirement ID cannot be empty".to_string());
        }

        if self.description.trim().is_empty() {
            return Err("Requirement description cannot be empty".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_document_new() {
        let doc = ArchitectureDocument::new("Test Architecture");
        assert_eq!(doc.title(), "Test Architecture");
        assert_eq!(doc.description(), None);
        assert_eq!(doc.sections().len(), 0);
        assert_eq!(doc.components().len(), 0);
        assert_eq!(doc.requirements().len(), 0);
    }

    #[test]
    fn test_architecture_document_with_description() {
        let doc = ArchitectureDocument::new("Title").with_description("This is a test description");
        assert_eq!(doc.description(), Some("This is a test description"));
    }

    #[test]
    fn test_architecture_document_add_section() {
        let mut doc = ArchitectureDocument::new("Title");
        doc.add_section(Section::new("Section 1", 1));
        assert_eq!(doc.sections().len(), 1);
    }

    #[test]
    fn test_architecture_document_validation() {
        let doc = ArchitectureDocument::new("Valid Title");
        assert!(doc.validate().is_ok());

        let invalid_doc = ArchitectureDocument::new("");
        assert!(invalid_doc.validate().is_err());
    }

    #[test]
    fn test_section_new() {
        let section = Section::new("Introduction", 1);
        assert_eq!(section.heading(), "Introduction");
        assert_eq!(section.level(), 1);
        assert_eq!(section.content(), "");
    }

    #[test]
    fn test_section_with_content() {
        let section = Section::new("Title", 1).with_content("Content here");
        assert_eq!(section.content(), "Content here");
    }

    #[test]
    fn test_section_with_subsections() {
        let subsection = Section::new("Subsection", 2);
        let section = Section::new("Main", 1).with_subsections(vec![subsection]);
        assert_eq!(section.subsections().len(), 1);
    }

    #[test]
    fn test_section_validation() {
        let valid = Section::new("Valid", 1);
        assert!(valid.validate().is_ok());

        let empty_heading = Section::new("", 1);
        assert!(empty_heading.validate().is_err());

        let invalid_level = Section::new("Test", 0);
        assert!(invalid_level.validate().is_err());

        let invalid_level2 = Section::new("Test", 7);
        assert!(invalid_level2.validate().is_err());
    }

    #[test]
    fn test_component_new() {
        let component = Component::new("API Gateway", "api-gateway");
        assert_eq!(component.name(), "API Gateway");
        assert_eq!(component.id(), "api-gateway");
    }

    #[test]
    fn test_component_with_description() {
        let component = Component::new("Service", "service").with_description("Handles requests");
        assert_eq!(component.description(), Some("Handles requests"));
    }

    #[test]
    fn test_component_validation() {
        let valid = Component::new("Valid", "valid-id");
        assert!(valid.validate().is_ok());

        let empty_name = Component::new("", "id");
        assert!(empty_name.validate().is_err());

        let empty_id = Component::new("Name", "");
        assert!(empty_id.validate().is_err());
    }

    #[test]
    fn test_requirement_new() {
        let req = Requirement::new("req-1", "Must be secure");
        assert_eq!(req.id(), "req-1");
        assert_eq!(req.description(), "Must be secure");
    }

    #[test]
    fn test_requirement_with_category() {
        let req = Requirement::new("req-1", "Description").with_category("Security");
        assert_eq!(req.category(), Some("Security"));
    }

    #[test]
    fn test_requirement_validation() {
        let valid = Requirement::new("req-1", "Valid description");
        assert!(valid.validate().is_ok());

        let empty_id = Requirement::new("", "Description");
        assert!(empty_id.validate().is_err());

        let empty_desc = Requirement::new("req-1", "");
        assert!(empty_desc.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let doc = ArchitectureDocument::new("Test")
            .with_description("Description")
            .with_sections(vec![Section::new("Section", 1)])
            .with_components(vec![Component::new("Component", "comp-1")])
            .with_requirements(vec![Requirement::new("req-1", "Requirement")]);

        let json = serde_json::to_string(&doc).unwrap();
        let deserialized: ArchitectureDocument = serde_json::from_str(&json).unwrap();

        assert_eq!(doc.title(), deserialized.title());
        assert_eq!(doc.sections().len(), deserialized.sections().len());
    }
}
