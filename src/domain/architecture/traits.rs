//! Domain traits for architecture generation and management
//!
//! This module defines the core contracts (traits) that infrastructure implementations
//! must satisfy. Following dependency inversion, the domain layer defines what it needs,
//! and the infrastructure layer provides implementations.
//!
//! # Traits
//!
//! - [`ArchitectureGenerator`]: Generate architecture documents from requirements
//! - [`TemplateRepository`]: Manage architecture templates
//! - [`ArchitectureWriter`]: Write architecture documents to various formats
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::{
//!     ArchitectureGenerator, GenerationOptions, ComplexityLevel, ArchitecturePattern,
//! };
//! use async_trait::async_trait;
//! use thiserror::Error;
//!
//! #[derive(Error, Debug)]
//! #[error("Generator error: {0}")]
//! struct MyError(String);
//!
//! // Example implementation (actual impl would be in infrastructure layer)
//! struct MyGenerator;
//!
//! #[async_trait]
//! impl ArchitectureGenerator for MyGenerator {
//!     type Error = MyError;
//!
//!     async fn generate(
//!         &self,
//!         requirements: &str,
//!         options: &GenerationOptions,
//!     ) -> Result<xzagentz::domain::architecture::ArchitectureDocument, Self::Error> {
//!         // Implementation details
//!         unimplemented!()
//!     }
//!
//!     async fn refine(
//!         &self,
//!         document: &xzagentz::domain::architecture::ArchitectureDocument,
//!         feedback: &str,
//!     ) -> Result<xzagentz::domain::architecture::ArchitectureDocument, Self::Error> {
//!         unimplemented!()
//!     }
//!
//!     async fn generate_from_template(
//!         &self,
//!         template: &xzagentz::domain::architecture::ArchitectureTemplate,
//!         customization: &str,
//!     ) -> Result<xzagentz::domain::architecture::ArchitectureDocument, Self::Error> {
//!         unimplemented!()
//!     }
//! }
//! ```

use crate::domain::architecture::models::{ArchitectureDocument, ArchitecturePattern};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Generates architecture documents from requirements using LLM
///
/// This trait abstracts the LLM integration, allowing different implementations
/// (Ollama, OpenAI, Claude, etc.) to be swapped without changing domain logic.
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::domain::architecture::{
///     ArchitectureGenerator, GenerationOptions, ComplexityLevel, ArchitecturePattern,
/// };
///
/// async fn generate_architecture<G: ArchitectureGenerator>(
///     generator: &G,
///     requirements: &str,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     let options = GenerationOptions {
///         pattern: Some(ArchitecturePattern::Microservices),
///         complexity: ComplexityLevel::Moderate,
///         include_deployment: true,
///         include_quality_attributes: true,
///         max_components: Some(10),
///         technology_preferences: vec!["Rust".to_string(), "PostgreSQL".to_string()],
///     };
///
///     let document = generator.generate(requirements, &options).await?;
///     println!("Generated architecture: {}", document.metadata.title);
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ArchitectureGenerator: Send + Sync {
    /// Error type returned by this generator
    type Error: std::error::Error + Send + Sync + 'static;

    /// Generate a complete architecture document from requirements
    ///
    /// # Arguments
    ///
    /// * `requirements` - Natural language description of system requirements
    /// * `options` - Generation options (pattern, complexity, preferences)
    ///
    /// # Returns
    ///
    /// Returns a complete `ArchitectureDocument` or an error
    ///
    /// # Errors
    ///
    /// Returns error if LLM is unavailable, response parsing fails, or validation fails
    async fn generate(
        &self,
        requirements: &str,
        options: &GenerationOptions,
    ) -> Result<ArchitectureDocument, Self::Error>;

    /// Refine an existing architecture based on feedback
    ///
    /// # Arguments
    ///
    /// * `document` - Existing architecture document to refine
    /// * `feedback` - Refinement instructions (e.g., "Add caching layer")
    ///
    /// # Returns
    ///
    /// Returns refined `ArchitectureDocument` or an error
    ///
    /// # Errors
    ///
    /// Returns error if LLM is unavailable or refinement fails
    async fn refine(
        &self,
        document: &ArchitectureDocument,
        feedback: &str,
    ) -> Result<ArchitectureDocument, Self::Error>;

    /// Generate architecture from a template with customization
    ///
    /// # Arguments
    ///
    /// * `template` - Base template to use
    /// * `customization` - Customization instructions
    ///
    /// # Returns
    ///
    /// Returns customized `ArchitectureDocument` or an error
    ///
    /// # Errors
    ///
    /// Returns error if template is invalid or generation fails
    async fn generate_from_template(
        &self,
        template: &ArchitectureTemplate,
        customization: &str,
    ) -> Result<ArchitectureDocument, Self::Error>;
}

/// Options for architecture generation
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{
///     GenerationOptions, ComplexityLevel, ArchitecturePattern,
/// };
///
/// let options = GenerationOptions {
///     pattern: Some(ArchitecturePattern::EventDriven),
///     complexity: ComplexityLevel::Enterprise,
///     include_deployment: true,
///     include_quality_attributes: true,
///     max_components: Some(20),
///     technology_preferences: vec![
///         "Kafka".to_string(),
///         "Kubernetes".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    /// Preferred architecture pattern (None for LLM to decide)
    pub pattern: Option<ArchitecturePattern>,
    /// System complexity level
    pub complexity: ComplexityLevel,
    /// Include deployment architecture section
    pub include_deployment: bool,
    /// Include quality attributes section
    pub include_quality_attributes: bool,
    /// Maximum number of components (None for unlimited)
    pub max_components: Option<usize>,
    /// Preferred technologies (languages, frameworks, databases)
    pub technology_preferences: Vec<String>,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            pattern: None,
            complexity: ComplexityLevel::Moderate,
            include_deployment: true,
            include_quality_attributes: true,
            max_components: None,
            technology_preferences: Vec::new(),
        }
    }
}

/// Complexity level of the system being designed
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::ComplexityLevel;
///
/// let simple = ComplexityLevel::Simple; // 1-3 components
/// let moderate = ComplexityLevel::Moderate; // 4-8 components
/// let complex = ComplexityLevel::Complex; // 9-15 components
/// let enterprise = ComplexityLevel::Enterprise; // 16+ components
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplexityLevel {
    /// Simple system (1-3 components)
    Simple,
    /// Moderate complexity (4-8 components)
    Moderate,
    /// Complex system (9-15 components)
    Complex,
    /// Enterprise-scale system (16+ components)
    Enterprise,
}

/// Repository for managing architecture templates
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::domain::architecture::TemplateRepository;
///
/// async fn list_available_templates<R: TemplateRepository>(
///     repo: &R,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     let templates = repo.list_templates().await?;
///     for template in templates {
///         println!("{}: {}", template.name, template.description);
///     }
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait TemplateRepository: Send + Sync {
    /// Error type returned by this repository
    type Error: std::error::Error + Send + Sync + 'static;

    /// List all available templates
    ///
    /// # Returns
    ///
    /// Returns a vector of `TemplateInfo` or an error
    ///
    /// # Errors
    ///
    /// Returns error if repository is unavailable or reading fails
    async fn list_templates(&self) -> Result<Vec<TemplateInfo>, Self::Error>;

    /// Load a specific template by name
    ///
    /// # Arguments
    ///
    /// * `name` - Template name (e.g., "microservices")
    ///
    /// # Returns
    ///
    /// Returns `ArchitectureTemplate` or an error
    ///
    /// # Errors
    ///
    /// Returns error if template not found or parsing fails
    async fn load_template(&self, name: &str) -> Result<ArchitectureTemplate, Self::Error>;

    /// Save a template for reuse
    ///
    /// # Arguments
    ///
    /// * `template` - Template to save
    ///
    /// # Errors
    ///
    /// Returns error if writing fails or template is invalid
    async fn save_template(&self, template: &ArchitectureTemplate) -> Result<(), Self::Error>;
}

/// Metadata about an architecture template
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{TemplateInfo, ArchitecturePattern};
///
/// let info = TemplateInfo {
///     name: "microservices".to_string(),
///     pattern: ArchitecturePattern::Microservices,
///     description: "Cloud-native microservices architecture".to_string(),
///     use_cases: vec![
///         "Scalable web applications".to_string(),
///         "Distributed systems".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateInfo {
    /// Template name (kebab-case)
    pub name: String,
    /// Architecture pattern
    pub pattern: ArchitecturePattern,
    /// Template description
    pub description: String,
    /// Common use cases for this template
    pub use_cases: Vec<String>,
}

/// Complete architecture template
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{
///     ArchitectureTemplate, TemplateInfo, TemplateStructure,
///     ComponentTemplate, ArchitecturePattern,
/// };
///
/// let template = ArchitectureTemplate {
///     info: TemplateInfo {
///         name: "layered".to_string(),
///         pattern: ArchitecturePattern::Layered,
///         description: "Traditional layered architecture".to_string(),
///         use_cases: vec!["Monolithic applications".to_string()],
///     },
///     structure: TemplateStructure {
///         layers: vec!["API".to_string(), "Application".to_string(), "Domain".to_string()],
///         integration_patterns: vec!["Synchronous".to_string()],
///         required_components: vec!["database".to_string()],
///     },
///     default_components: vec![],
///     customization_points: vec!["Add caching layer".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureTemplate {
    /// Template metadata
    pub info: TemplateInfo,
    /// Template structure (layers, patterns, requirements)
    pub structure: TemplateStructure,
    /// Default components to include
    pub default_components: Vec<ComponentTemplate>,
    /// Points where template can be customized
    pub customization_points: Vec<String>,
}

/// Template structure definition
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::TemplateStructure;
///
/// let structure = TemplateStructure {
///     layers: vec![
///         "API Layer".to_string(),
///         "Application Layer".to_string(),
///         "Domain Layer".to_string(),
///         "Infrastructure Layer".to_string(),
///     ],
///     integration_patterns: vec![
///         "REST".to_string(),
///         "Event-Driven".to_string(),
///     ],
///     required_components: vec![
///         "api-gateway".to_string(),
///         "database".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateStructure {
    /// Layer names in this template
    pub layers: Vec<String>,
    /// Integration patterns used
    pub integration_patterns: Vec<String>,
    /// Components that must be included
    pub required_components: Vec<String>,
}

/// Template for a component
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::ComponentTemplate;
///
/// let template = ComponentTemplate {
///     name: "API Gateway".to_string(),
///     layer: "API Layer".to_string(),
///     role: "Entry point for all client requests".to_string(),
///     typical_technologies: vec![
///         "Kong".to_string(),
///         "Nginx".to_string(),
///         "Traefik".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentTemplate {
    /// Component name
    pub name: String,
    /// Layer this component belongs to
    pub layer: String,
    /// Component role and purpose
    pub role: String,
    /// Common technology choices for this component
    pub typical_technologies: Vec<String>,
}

/// Writes architecture documents to various formats
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::domain::architecture::{ArchitectureWriter, ArchitectureDocument};
/// use std::path::Path;
///
/// async fn save_architecture<W: ArchitectureWriter>(
///     writer: &W,
///     document: &ArchitectureDocument,
///     path: &Path,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     writer.write(document, path).await?;
///     println!("Architecture saved to: {}", path.display());
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ArchitectureWriter: Send + Sync {
    /// Error type returned by this writer
    type Error: std::error::Error + Send + Sync + 'static;

    /// Write architecture document to a file
    ///
    /// # Arguments
    ///
    /// * `document` - Architecture document to write
    /// * `path` - Output file path
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be written or formatting fails
    async fn write(&self, document: &ArchitectureDocument, path: &Path) -> Result<(), Self::Error>;

    /// Format architecture document as a string
    ///
    /// # Arguments
    ///
    /// * `document` - Architecture document to format
    ///
    /// # Returns
    ///
    /// Returns formatted string representation
    fn format(&self, document: &ArchitectureDocument) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_options_default() {
        let options = GenerationOptions::default();
        assert_eq!(options.complexity, ComplexityLevel::Moderate);
        assert!(options.include_deployment);
        assert!(options.include_quality_attributes);
        assert!(options.pattern.is_none());
    }

    #[test]
    fn test_complexity_levels() {
        let simple = ComplexityLevel::Simple;
        let moderate = ComplexityLevel::Moderate;
        let complex = ComplexityLevel::Complex;
        let enterprise = ComplexityLevel::Enterprise;

        assert_ne!(simple, moderate);
        assert_ne!(complex, enterprise);
    }

    #[test]
    fn test_template_info_creation() {
        let info = TemplateInfo {
            name: "test-template".to_string(),
            pattern: ArchitecturePattern::Layered,
            description: "Test template".to_string(),
            use_cases: vec!["Testing".to_string()],
        };

        assert_eq!(info.name, "test-template");
        assert_eq!(info.pattern, ArchitecturePattern::Layered);
    }

    #[test]
    fn test_template_structure() {
        let structure = TemplateStructure {
            layers: vec!["API".to_string(), "Domain".to_string()],
            integration_patterns: vec!["REST".to_string()],
            required_components: vec!["database".to_string()],
        };

        assert_eq!(structure.layers.len(), 2);
        assert!(structure
            .required_components
            .contains(&"database".to_string()));
    }

    #[test]
    fn test_component_template() {
        let template = ComponentTemplate {
            name: "API Gateway".to_string(),
            layer: "API".to_string(),
            role: "Request routing".to_string(),
            typical_technologies: vec!["Kong".to_string()],
        };

        assert_eq!(template.name, "API Gateway");
        assert_eq!(template.typical_technologies.len(), 1);
    }

    #[test]
    fn test_serialization_generation_options() {
        let options = GenerationOptions {
            pattern: Some(ArchitecturePattern::Microservices),
            complexity: ComplexityLevel::Complex,
            include_deployment: true,
            include_quality_attributes: false,
            max_components: Some(15),
            technology_preferences: vec!["Rust".to_string()],
        };

        let json = serde_json::to_string(&options).expect("Failed to serialize");
        let deserialized: GenerationOptions =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(options.complexity, deserialized.complexity);
        assert_eq!(options.max_components, deserialized.max_components);
    }
}
