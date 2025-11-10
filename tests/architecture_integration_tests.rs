// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for architecture generation functionality
//!
//! These tests verify end-to-end workflows for the architecture command,
//! including generation from requirements, template-based generation,
//! and architecture refinement.

use std::fs;
use tempfile::TempDir;
use xzagentz::application::architecture_service::{ArchitectureService, ServiceError};
use xzagentz::domain::architecture::{
    ArchitectureDocument, ArchitectureGenerator, ArchitectureMetadata, ArchitecturePattern,
    ArchitectureTemplate, ComplexityLevel, Component, GenerationOptions, Layer, Overview,
};
use xzagentz::infrastructure::templates::FileTemplateRepository;
use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;

// Mock generator for testing without Ollama dependency
struct MockArchitectureGenerator {
    should_fail: bool,
}

impl MockArchitectureGenerator {
    fn new() -> Self {
        Self { should_fail: false }
    }

    #[allow(dead_code)]
    fn with_failure() -> Self {
        Self { should_fail: true }
    }
}

#[async_trait::async_trait]
impl ArchitectureGenerator for MockArchitectureGenerator {
    type Error = MockError;

    async fn generate(
        &self,
        requirements: &str,
        options: &GenerationOptions,
    ) -> Result<ArchitectureDocument, Self::Error> {
        if self.should_fail {
            return Err(MockError::Generation);
        }

        Ok(create_mock_architecture_document(requirements, options))
    }

    async fn refine(
        &self,
        document: &ArchitectureDocument,
        refinement_request: &str,
    ) -> Result<ArchitectureDocument, Self::Error> {
        if self.should_fail {
            return Err(MockError::Refinement);
        }

        let mut refined = document.clone();
        refined.metadata.version = format!("{}-refined", document.metadata.version);
        refined.overview.description = format!(
            "{}\n\nRefinement: {}",
            document.overview.description, refinement_request
        );
        Ok(refined)
    }

    async fn generate_from_template(
        &self,
        template: &ArchitectureTemplate,
        customization: &str,
    ) -> Result<ArchitectureDocument, Self::Error> {
        if self.should_fail {
            return Err(MockError::Template);
        }

        Ok(create_mock_architecture_from_template(
            template,
            customization,
        ))
    }
}

#[derive(Debug)]
enum MockError {
    Generation,
    Refinement,
    Template,
}

impl std::fmt::Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MockError::Generation => write!(f, "Generation failed"),
            MockError::Refinement => write!(f, "Refinement failed"),
            MockError::Template => write!(f, "Template generation failed"),
        }
    }
}

impl std::error::Error for MockError {}

// Helper functions

fn create_mock_architecture_document(
    requirements: &str,
    options: &GenerationOptions,
) -> ArchitectureDocument {
    use xzagentz::domain::architecture::ArchitectureValidator;

    let pattern = options
        .pattern
        .clone()
        .unwrap_or(ArchitecturePattern::Layered);

    let doc = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: format!("Architecture for: {}", requirements),
            version: "1.0.0".to_string(),
            generated_at: chrono::Utc::now(),
            model_used: "mock-model".to_string(),
            pattern,
            authors: vec!["Mock Generator".to_string()],
        },
        overview: Overview {
            description: format!("Mock architecture based on: {}", requirements),
            business_goals: vec!["Goal 1".to_string(), "Goal 2".to_string()],
            constraints: vec!["Constraint 1".to_string()],
            assumptions: vec!["Assumption 1".to_string()],
        },
        layers: vec![
            Layer {
                name: "Presentation Layer".to_string(),
                description: "Handles user interactions".to_string(),
                responsibilities: vec!["UI rendering".to_string()],
                components: vec!["web-ui".to_string()],
                dependencies: vec!["Application Layer".to_string()],
            },
            Layer {
                name: "Application Layer".to_string(),
                description: "Business logic".to_string(),
                responsibilities: vec!["Use cases".to_string()],
                components: vec!["app-service".to_string()],
                dependencies: vec!["Domain Layer".to_string()],
            },
        ],
        components: vec![
            Component {
                id: "web-ui".to_string(),
                name: "Web UI".to_string(),
                description: "Frontend application".to_string(),
                layer: "Presentation Layer".to_string(),
                responsibilities: vec!["Display data".to_string()],
                interfaces: vec![],
                dependencies: vec!["app-service".to_string()],
                technology_stack: vec!["React".to_string()],
            },
            Component {
                id: "app-service".to_string(),
                name: "Application Service".to_string(),
                description: "Core business logic".to_string(),
                layer: "Application Layer".to_string(),
                responsibilities: vec!["Process requests".to_string()],
                interfaces: vec![],
                dependencies: vec![],
                technology_stack: vec!["Rust".to_string()],
            },
        ],
        integrations: vec![],
        deployment: None,
        quality_attributes: vec![],
    };

    // Validate before returning
    let validator = ArchitectureValidator;
    validator
        .validate(&doc)
        .expect("Mock document must be valid");
    doc
}

fn create_mock_architecture_from_template(
    template: &ArchitectureTemplate,
    customization: &str,
) -> ArchitectureDocument {
    use xzagentz::domain::architecture::ArchitectureValidator;

    let layers: Vec<Layer> = template
        .structure
        .layers
        .iter()
        .enumerate()
        .map(|(idx, layer_name)| Layer {
            name: layer_name.clone(),
            description: format!("Description for {}", layer_name),
            responsibilities: vec![format!("{} responsibilities", layer_name)],
            components: vec![format!("component-{}", idx)],
            dependencies: vec![],
        })
        .collect();

    let components: Vec<Component> = template
        .structure
        .layers
        .iter()
        .enumerate()
        .map(|(idx, layer_name)| Component {
            id: format!("component-{}", idx),
            name: format!("Component {}", idx),
            description: format!("Component for {}", layer_name),
            layer: layer_name.clone(),
            responsibilities: vec![format!("Handle {} operations", layer_name)],
            interfaces: vec![],
            dependencies: vec![],
            technology_stack: vec!["Rust".to_string()],
        })
        .collect();

    let doc = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: format!("Architecture from template: {}", template.info.name),
            version: "1.0.0".to_string(),
            generated_at: chrono::Utc::now(),
            model_used: "mock-model".to_string(),
            pattern: template.info.pattern.clone(),
            authors: vec!["Mock Generator".to_string()],
        },
        overview: Overview {
            description: format!(
                "{}\n\nCustomization: {}",
                template.info.description, customization
            ),
            business_goals: vec!["Template-based goal".to_string()],
            constraints: vec![],
            assumptions: vec![],
        },
        layers,
        components,
        integrations: vec![],
        deployment: None,
        quality_attributes: vec![],
    };

    // Validate before returning
    let validator = ArchitectureValidator;
    validator
        .validate(&doc)
        .expect("Mock template document must be valid");
    doc
}

async fn setup_test_service() -> (
    ArchitectureService<
        MockArchitectureGenerator,
        MarkdownArchitectureWriter,
        FileTemplateRepository,
    >,
    TempDir,
) {
    let temp_dir = TempDir::new().unwrap();
    let templates_dir = temp_dir.path().join("templates");
    fs::create_dir_all(&templates_dir).unwrap();

    let generator = MockArchitectureGenerator::new();
    let writer = MarkdownArchitectureWriter::new();
    let repo = FileTemplateRepository::new(templates_dir.clone());
    repo.initialize().await.unwrap();

    let service = ArchitectureService::new(generator, writer, repo);
    (service, temp_dir)
}

// Integration Tests

#[tokio::test]
async fn test_end_to_end_architecture_generation() {
    let (service, temp_dir) = setup_test_service().await;

    let requirements = "Build a web application with REST API and database";
    let options = GenerationOptions {
        pattern: Some(ArchitecturePattern::Layered),
        complexity: ComplexityLevel::Moderate,
        include_deployment: true,
        include_quality_attributes: true,
        max_components: Some(10),
        technology_preferences: vec!["Rust".to_string(), "PostgreSQL".to_string()],
    };

    let document = service
        .generate_from_requirements(requirements, options)
        .await
        .expect("Generation should succeed");

    assert!(document.metadata.title.contains(requirements));
    assert_eq!(document.metadata.pattern, ArchitecturePattern::Layered);
    assert!(!document.layers.is_empty());
    assert!(!document.components.is_empty());

    let output_path = temp_dir.path().join("test_architecture.md");
    service
        .save_architecture(&document, &output_path)
        .await
        .expect("Save should succeed");

    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("# Architecture"));
    assert!(content.contains(requirements));
}

#[tokio::test]
async fn test_template_based_generation() {
    let (service, temp_dir) = setup_test_service().await;

    let templates = service
        .list_templates()
        .await
        .expect("List templates should succeed");

    assert!(!templates.is_empty(), "Should have default templates");

    let microservices_template = templates
        .iter()
        .find(|t| t.name == "microservices")
        .expect("Should have microservices template");

    let customization = "Add authentication and payment services";

    let document = service
        .generate_from_template(&microservices_template.name, customization)
        .await
        .expect("Template generation should succeed");

    assert!(document.metadata.title.contains("microservices"));
    assert_eq!(
        document.metadata.pattern,
        ArchitecturePattern::Microservices
    );
    assert!(document.overview.description.contains(customization));

    let output_path = temp_dir.path().join("test_template_arch.md");
    service
        .save_architecture(&document, &output_path)
        .await
        .expect("Save should succeed");

    assert!(output_path.exists());
}

#[tokio::test]
async fn test_refine_architecture_workflow() {
    let (service, temp_dir) = setup_test_service().await;

    let requirements = "Simple CRUD API";
    let options = GenerationOptions {
        pattern: Some(ArchitecturePattern::Layered),
        complexity: ComplexityLevel::Simple,
        include_deployment: false,
        include_quality_attributes: false,
        max_components: Some(5),
        technology_preferences: vec![],
    };

    let initial_document = service
        .generate_from_requirements(requirements, options)
        .await
        .expect("Initial generation should succeed");

    let original_version = initial_document.metadata.version.clone();
    let original_description = initial_document.overview.description.clone();

    let refinement_request = "Add caching layer and improve security";

    let refined_document = service
        .refine_architecture(&initial_document, refinement_request)
        .await
        .expect("Refinement should succeed");

    assert_ne!(refined_document.metadata.version, original_version);
    assert!(refined_document
        .overview
        .description
        .contains(refinement_request));
    assert!(refined_document.overview.description.len() > original_description.len());

    let output_path = temp_dir.path().join("test_refined_arch.md");
    service
        .save_architecture(&refined_document, &output_path)
        .await
        .expect("Save refined document should succeed");

    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains(refinement_request));
}

#[tokio::test]
async fn test_generate_with_different_patterns() {
    let (service, _temp_dir) = setup_test_service().await;

    let patterns = vec![
        ArchitecturePattern::Layered,
        ArchitecturePattern::Microservices,
        ArchitecturePattern::EventDriven,
        ArchitecturePattern::Monolithic,
    ];

    for pattern in &patterns {
        let options = GenerationOptions {
            pattern: Some(pattern.clone()),
            complexity: ComplexityLevel::Simple,
            include_deployment: false,
            include_quality_attributes: false,
            max_components: Some(5),
            technology_preferences: vec![],
        };

        let document = service
            .generate_from_requirements("Test application", options)
            .await
            .expect("Generation should succeed for all patterns");

        assert_eq!(&document.metadata.pattern, pattern);
    }
}

#[tokio::test]
async fn test_generate_with_complexity_levels() {
    let (service, _temp_dir) = setup_test_service().await;

    let complexity_levels = vec![
        ComplexityLevel::Simple,
        ComplexityLevel::Moderate,
        ComplexityLevel::Complex,
        ComplexityLevel::Enterprise,
    ];

    for complexity in complexity_levels {
        let options = GenerationOptions {
            pattern: Some(ArchitecturePattern::Layered),
            complexity,
            include_deployment: false,
            include_quality_attributes: false,
            max_components: Some(10),
            technology_preferences: vec![],
        };

        let document = service
            .generate_from_requirements("Test application", options)
            .await
            .expect("Generation should succeed for all complexity levels");

        assert!(!document.metadata.title.is_empty());
        assert!(!document.components.is_empty());
    }
}

#[tokio::test]
async fn test_save_to_different_locations() {
    let (service, temp_dir) = setup_test_service().await;

    let options = GenerationOptions {
        pattern: Some(ArchitecturePattern::Layered),
        complexity: ComplexityLevel::Simple,
        include_deployment: false,
        include_quality_attributes: false,
        max_components: Some(5),
        technology_preferences: vec![],
    };

    let document = service
        .generate_from_requirements("Test", options)
        .await
        .expect("Generation should succeed");

    let paths = vec![
        temp_dir.path().join("architecture.md"),
        temp_dir.path().join("subdir").join("architecture.md"),
        temp_dir
            .path()
            .join("another")
            .join("nested")
            .join("arch.md"),
    ];

    for path in paths {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        service
            .save_architecture(&document, &path)
            .await
            .expect("Save should succeed");

        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("# Architecture"));
    }
}

#[tokio::test]
async fn test_list_available_templates() {
    let (service, _temp_dir) = setup_test_service().await;

    let templates = service
        .list_templates()
        .await
        .expect("List templates should succeed");

    assert!(!templates.is_empty());

    // Check for some expected templates (may vary by implementation)
    assert!(
        templates
            .iter()
            .any(|t| t.name.contains("microservices") || t.name.contains("service")),
        "Should have a microservices-related template"
    );

    for template in &templates {
        assert!(!template.name.is_empty());
        assert!(!template.description.is_empty());
        assert!(!template.use_cases.is_empty());
    }
}

#[tokio::test]
async fn test_error_handling_invalid_template() {
    let (service, _temp_dir) = setup_test_service().await;

    let result = service
        .generate_from_template("nonexistent-template", "customization")
        .await;

    assert!(result.is_err());
    match result {
        Err(ServiceError::TemplateError(_)) => {}
        _ => panic!("Expected TemplateError"),
    }
}

#[tokio::test]
async fn test_markdown_output_format() {
    let (service, temp_dir) = setup_test_service().await;

    let options = GenerationOptions {
        pattern: Some(ArchitecturePattern::Microservices),
        complexity: ComplexityLevel::Moderate,
        include_deployment: true,
        include_quality_attributes: true,
        max_components: Some(8),
        technology_preferences: vec!["Rust".to_string(), "Kubernetes".to_string()],
    };

    let document = service
        .generate_from_requirements("E-commerce platform", options)
        .await
        .expect("Generation should succeed");

    let output_path = temp_dir.path().join("formatted_output.md");
    service
        .save_architecture(&document, &output_path)
        .await
        .expect("Save should succeed");

    let content = fs::read_to_string(&output_path).unwrap();

    // Check for basic markdown structure
    assert!(content.contains("# Architecture") || content.contains("Architecture"));
    assert!(content.contains("Metadata") || content.contains("metadata"));
    assert!(content.contains("Overview") || content.contains("overview"));
    assert!(content.contains("Components") || content.contains("components"));
}

#[tokio::test]
async fn test_multiple_sequential_generations() {
    let (service, _temp_dir) = setup_test_service().await;

    let options = GenerationOptions {
        pattern: Some(ArchitecturePattern::Layered),
        complexity: ComplexityLevel::Simple,
        include_deployment: false,
        include_quality_attributes: false,
        max_components: Some(5),
        technology_preferences: vec![],
    };

    for i in 0..5 {
        let requirements = format!("Test application {}", i);
        let result = service
            .generate_from_requirements(&requirements, options.clone())
            .await;

        assert!(result.is_ok());
        let document = result.unwrap();
        assert!(document.metadata.title.contains(&requirements));
    }
}
