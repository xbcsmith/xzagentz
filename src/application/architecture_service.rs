//! Architecture Generation Service
//!
//! This module provides the main service for orchestrating architecture generation
//! using LLM (via Ollama), template management, and document persistence.
//!
//! # Architecture
//!
//! The `ArchitectureService` coordinates between:
//! - Domain layer: Architecture models and validation
//! - Infrastructure layer: Ollama generator, template repository, document writers
//!
//! # Examples
//!
//! ```no_run
//! use xzagentz::application::ArchitectureService;
//! use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
//! use xzagentz::infrastructure::templates::FileTemplateRepository;
//! use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
//! use xzagentz::domain::architecture::{GenerationOptions, ArchitecturePattern, ComplexityLevel};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use xzagentz::infrastructure::OllamaConfig;
//!
//! let config = OllamaConfig::default();
//! let generator = OllamaArchitectureGenerator::new(config)?;
//! let writer = MarkdownArchitectureWriter::new();
//! let template_repo = FileTemplateRepository::new("./templates/architecture".into());
//! template_repo.initialize().await?;
//!
//! let service = ArchitectureService::new(generator, writer, template_repo);
//!
//! let options = GenerationOptions {
//!     pattern: Some(ArchitecturePattern::Microservices),
//!     complexity: ComplexityLevel::Moderate,
//!     include_deployment: true,
//!     include_quality_attributes: true,
//!     max_components: Some(10),
//!     technology_preferences: vec!["Rust".to_string(), "PostgreSQL".to_string()],
//! };
//!
//! let document = service.generate_from_requirements(
//!     "Build an e-commerce platform with product catalog, cart, and checkout",
//!     options
//! ).await?;
//!
//! service.save_architecture(&document, "architecture.md").await?;
//! # Ok(())
//! # }
//! ```

use crate::domain::architecture::{
    ArchitectureDocument, ArchitectureGenerator, ArchitectureWriter, GenerationOptions,
    TemplateInfo, TemplateRepository, ValidationError,
};
use std::path::Path;
use thiserror::Error;

/// Service for architecture generation workflows
///
/// Coordinates architecture generation, validation, refinement, and persistence.
///
/// # Type Parameters
///
/// * `G` - Implementation of `ArchitectureGenerator` trait
/// * `W` - Implementation of `ArchitectureWriter` trait
/// * `T` - Implementation of `TemplateRepository` trait
pub struct ArchitectureService<G, W, T>
where
    G: ArchitectureGenerator,
    W: ArchitectureWriter,
    T: TemplateRepository,
{
    generator: G,
    writer: W,
    template_repo: T,
}

impl<G, W, T> ArchitectureService<G, W, T>
where
    G: ArchitectureGenerator,
    W: ArchitectureWriter,
    T: TemplateRepository,
{
    /// Creates a new architecture service
    ///
    /// # Arguments
    ///
    /// * `generator` - Implementation of architecture generator (e.g., Ollama-based)
    /// * `writer` - Implementation of architecture writer (e.g., Markdown)
    /// * `template_repo` - Implementation of template repository (e.g., file-based)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::application::ArchitectureService;
    /// use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use xzagentz::infrastructure::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// let generator = OllamaArchitectureGenerator::new(config)?;
    /// let writer = MarkdownArchitectureWriter::new();
    /// let template_repo = FileTemplateRepository::new("./templates".into());
    /// template_repo.initialize().await?;
    ///
    /// let service = ArchitectureService::new(generator, writer, template_repo);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(generator: G, writer: W, template_repo: T) -> Self {
        Self {
            generator,
            writer,
            template_repo,
        }
    }

    /// Generates architecture from requirements description
    ///
    /// # Arguments
    ///
    /// * `requirements` - Natural language description of system requirements
    /// * `options` - Generation options (pattern, complexity, etc.)
    ///
    /// # Returns
    ///
    /// Returns validated `ArchitectureDocument` on success
    ///
    /// # Errors
    ///
    /// Returns `ServiceError` if:
    /// - Generation fails
    /// - Validation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use xzagentz::application::ArchitectureService;
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// # use xzagentz::domain::architecture::{GenerationOptions, ComplexityLevel};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// let options = GenerationOptions::default();
    /// let document = service.generate_from_requirements(
    ///     "Build a real-time chat application with user authentication",
    ///     options
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_from_requirements(
        &self,
        requirements: &str,
        options: GenerationOptions,
    ) -> Result<ArchitectureDocument, ServiceError> {
        let document = self
            .generator
            .generate(requirements, &options)
            .await
            .map_err(|e| ServiceError::GenerationError(e.to_string()))?;

        // Validate generated document
        let validator = crate::domain::architecture::ArchitectureValidator;
        validator
            .validate(&document)
            .map_err(ServiceError::ValidationError)?;

        Ok(document)
    }

    /// Generates architecture from a template
    ///
    /// # Arguments
    ///
    /// * `template_name` - Name of the template to use
    /// * `customization` - Customization instructions for the template
    ///
    /// # Returns
    ///
    /// Returns validated `ArchitectureDocument` on success
    ///
    /// # Errors
    ///
    /// Returns `ServiceError` if:
    /// - Template not found
    /// - Generation fails
    /// - Validation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use xzagentz::application::ArchitectureService;
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// #
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// let document = service.generate_from_template(
    ///     "microservices",
    ///     "Add payment processing service using Stripe"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_from_template(
        &self,
        template_name: &str,
        customization: &str,
    ) -> Result<ArchitectureDocument, ServiceError> {
        // Load template
        let template = self
            .template_repo
            .load_template(template_name)
            .await
            .map_err(|e| ServiceError::TemplateError(e.to_string()))?;

        // Generate from template
        let document = self
            .generator
            .generate_from_template(&template, customization)
            .await
            .map_err(|e| ServiceError::GenerationError(e.to_string()))?;

        // Validate generated document
        let validator = crate::domain::architecture::ArchitectureValidator;
        validator
            .validate(&document)
            .map_err(ServiceError::ValidationError)?;

        Ok(document)
    }

    /// Refines an existing architecture document
    ///
    /// # Arguments
    ///
    /// * `document` - Existing architecture document
    /// * `feedback` - Refinement instructions
    ///
    /// # Returns
    ///
    /// Returns refined and validated `ArchitectureDocument` on success
    ///
    /// # Errors
    ///
    /// Returns `ServiceError` if:
    /// - Refinement fails
    /// - Validation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use xzagentz::application::ArchitectureService;
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// # use xzagentz::domain::architecture::GenerationOptions;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// # let document = service.generate_from_requirements("test", GenerationOptions::default()).await?;
    /// let refined = service.refine_architecture(
    ///     &document,
    ///     "Add caching layer using Redis between API and database"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refine_architecture(
        &self,
        document: &ArchitectureDocument,
        feedback: &str,
    ) -> Result<ArchitectureDocument, ServiceError> {
        let refined = self
            .generator
            .refine(document, feedback)
            .await
            .map_err(|e| ServiceError::GenerationError(e.to_string()))?;

        // Validate refined document
        let validator = crate::domain::architecture::ArchitectureValidator;
        validator
            .validate(&refined)
            .map_err(ServiceError::ValidationError)?;

        Ok(refined)
    }

    /// Saves architecture document to file
    ///
    /// # Arguments
    ///
    /// * `document` - Architecture document to save
    /// * `output_path` - Path where document should be written
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::WriteError` if writing fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use xzagentz::application::ArchitectureService;
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// # use xzagentz::domain::architecture::GenerationOptions;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// # let document = service.generate_from_requirements("test", GenerationOptions::default()).await?;
    /// service.save_architecture(&document, "output/architecture.md").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save_architecture(
        &self,
        document: &ArchitectureDocument,
        output_path: impl AsRef<Path>,
    ) -> Result<(), ServiceError> {
        self.writer
            .write(document, output_path.as_ref())
            .await
            .map_err(|e| ServiceError::WriteError(e.to_string()))
    }

    /// Lists available architecture templates
    ///
    /// # Returns
    ///
    /// Returns vector of `TemplateInfo` describing available templates
    ///
    /// # Errors
    ///
    /// Returns `ServiceError::TemplateError` if listing fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use xzagentz::application::ArchitectureService;
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// #
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// let templates = service.list_templates().await?;
    /// for template in templates {
    ///     println!("{}: {}", template.name, template.description);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_templates(&self) -> Result<Vec<TemplateInfo>, ServiceError> {
        self.template_repo
            .list_templates()
            .await
            .map_err(|e| ServiceError::TemplateError(e.to_string()))
    }
}

/// Errors that can occur in the architecture service
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Error during architecture generation
    #[error("Architecture generation failed: {0}")]
    GenerationError(String),

    /// Error during document validation
    #[error("Architecture validation failed: {0}")]
    ValidationError(ValidationError),

    /// Error accessing templates
    #[error("Template operation failed: {0}")]
    TemplateError(String),

    /// Error writing architecture document
    #[error("Failed to write architecture document: {0}")]
    WriteError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::architecture::{
        ArchitectureDocument, ArchitectureMetadata, ArchitecturePattern, ArchitectureTemplate,
        ComplexityLevel, GenerationOptions, Overview, TemplateInfo,
    };
    use async_trait::async_trait;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("Mock error: {0}")]
    struct MockError(String);

    // Mock generator for testing
    struct MockGenerator {
        should_fail: bool,
    }

    #[async_trait]
    impl ArchitectureGenerator for MockGenerator {
        type Error = MockError;

        async fn generate(
            &self,
            _requirements: &str,
            _options: &GenerationOptions,
        ) -> Result<ArchitectureDocument, Self::Error> {
            if self.should_fail {
                return Err(MockError("Generation failed".to_string()));
            }
            Ok(create_test_document())
        }

        async fn refine(
            &self,
            _document: &ArchitectureDocument,
            _feedback: &str,
        ) -> Result<ArchitectureDocument, Self::Error> {
            if self.should_fail {
                return Err(MockError("Refinement failed".to_string()));
            }
            Ok(create_test_document())
        }

        async fn generate_from_template(
            &self,
            _template: &ArchitectureTemplate,
            _customization: &str,
        ) -> Result<ArchitectureDocument, Self::Error> {
            if self.should_fail {
                return Err(MockError("Template generation failed".to_string()));
            }
            Ok(create_test_document())
        }
    }

    // Mock writer for testing
    struct MockWriter {
        should_fail: bool,
    }

    #[async_trait]
    impl ArchitectureWriter for MockWriter {
        type Error = MockError;

        async fn write(
            &self,
            _document: &ArchitectureDocument,
            _path: &Path,
        ) -> Result<(), Self::Error> {
            if self.should_fail {
                return Err(MockError("Write failed".to_string()));
            }
            Ok(())
        }

        fn format(&self, _document: &ArchitectureDocument) -> String {
            "# Architecture Document\n".to_string()
        }
    }

    // Mock template repository for testing
    struct MockTemplateRepository {
        should_fail: bool,
    }

    #[async_trait]
    impl TemplateRepository for MockTemplateRepository {
        type Error = MockError;

        async fn list_templates(&self) -> Result<Vec<TemplateInfo>, Self::Error> {
            if self.should_fail {
                return Err(MockError("List failed".to_string()));
            }
            Ok(vec![TemplateInfo {
                name: "test-template".to_string(),
                pattern: ArchitecturePattern::Microservices,
                description: "Test template".to_string(),
                use_cases: vec!["Testing".to_string()],
            }])
        }

        async fn load_template(&self, _name: &str) -> Result<ArchitectureTemplate, Self::Error> {
            if self.should_fail {
                return Err(MockError("Load failed".to_string()));
            }
            Ok(create_test_template())
        }

        async fn save_template(&self, _template: &ArchitectureTemplate) -> Result<(), Self::Error> {
            if self.should_fail {
                return Err(MockError("Save failed".to_string()));
            }
            Ok(())
        }
    }

    fn create_test_document() -> ArchitectureDocument {
        use crate::domain::architecture::{Component, Layer};

        ArchitectureDocument {
            metadata: ArchitectureMetadata {
                title: "Test Architecture".to_string(),
                version: "1.0".to_string(),
                generated_at: chrono::Utc::now(),
                model_used: "test-model".to_string(),
                pattern: ArchitecturePattern::Microservices,
                authors: vec!["Test Author".to_string()],
            },
            overview: Overview {
                description: "Test architecture description".to_string(),
                business_goals: vec!["Test goal".to_string()],
                constraints: vec!["Test constraint".to_string()],
                assumptions: vec!["Test assumption".to_string()],
            },
            layers: vec![Layer {
                name: "API Layer".to_string(),
                description: "Test API layer".to_string(),
                responsibilities: vec!["Handle requests".to_string()],
                components: vec!["api-gateway".to_string()],
                dependencies: vec![],
            }],
            components: vec![Component {
                id: "api-gateway".to_string(),
                name: "API Gateway".to_string(),
                description: "Main API gateway".to_string(),
                layer: "API Layer".to_string(),
                responsibilities: vec!["Route requests".to_string()],
                interfaces: vec![],
                dependencies: vec![],
                technology_stack: vec!["Rust".to_string()],
            }],
            integrations: vec![],
            deployment: None,
            quality_attributes: vec![],
        }
    }

    fn create_test_template() -> ArchitectureTemplate {
        ArchitectureTemplate {
            info: TemplateInfo {
                name: "test-template".to_string(),
                pattern: ArchitecturePattern::Microservices,
                description: "Test template".to_string(),
                use_cases: vec!["Testing".to_string()],
            },
            structure: crate::domain::architecture::TemplateStructure {
                layers: vec!["api".to_string(), "domain".to_string()],
                integration_patterns: vec!["REST".to_string()],
                required_components: vec!["gateway".to_string()],
            },
            default_components: vec![],
            customization_points: vec!["database".to_string(), "cache".to_string()],
        }
    }

    #[tokio::test]
    async fn test_generate_from_requirements_success() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let options = GenerationOptions {
            pattern: Some(ArchitecturePattern::Microservices),
            complexity: ComplexityLevel::Simple,
            include_deployment: false,
            include_quality_attributes: false,
            max_components: Some(5),
            technology_preferences: vec![],
        };

        let result = service
            .generate_from_requirements("Test requirements", options)
            .await;

        assert!(result.is_ok());
        let document = result.unwrap();
        assert_eq!(document.metadata.title, "Test Architecture");
    }

    #[tokio::test]
    async fn test_generate_from_requirements_failure() {
        let generator = MockGenerator { should_fail: true };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let options = GenerationOptions::default();
        let result = service
            .generate_from_requirements("Test requirements", options)
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::GenerationError(_)
        ));
    }

    #[tokio::test]
    async fn test_generate_from_template_success() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let result = service
            .generate_from_template("test-template", "Add payment service")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_from_template_not_found() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: true };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let result = service
            .generate_from_template("missing-template", "customization")
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TemplateError(_)
        ));
    }

    #[tokio::test]
    async fn test_refine_architecture_success() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let document = create_test_document();
        let result = service
            .refine_architecture(&document, "Add caching layer")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_architecture_success() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let document = create_test_document();
        let result = service.save_architecture(&document, "test.md").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_architecture_failure() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: true };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let document = create_test_document();
        let result = service.save_architecture(&document, "test.md").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::WriteError(_)));
    }

    #[tokio::test]
    async fn test_list_templates_success() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: false };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let result = service.list_templates().await;

        assert!(result.is_ok());
        let templates = result.unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].name, "test-template");
    }

    #[tokio::test]
    async fn test_list_templates_failure() {
        let generator = MockGenerator { should_fail: false };
        let writer = MockWriter { should_fail: false };
        let template_repo = MockTemplateRepository { should_fail: true };
        let service = ArchitectureService::new(generator, writer, template_repo);

        let result = service.list_templates().await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ServiceError::TemplateError(_)
        ));
    }
}
