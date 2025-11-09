# LLM Architecture Command Plan

## Overview

This document outlines the phased implementation plan for adding LLM support to the `architecture` command in xzagentz. The architecture command will leverage Large Language Models (LLMs) via Ollama to generate comprehensive software architecture documents from high-level requirements, project descriptions, or template patterns.

## Business Requirements

### Primary Goal

Enable users to generate professional, structured architecture documentation using AI assistance, reducing the time from concept to documented architecture from hours to minutes.

### Key Features

1. **LLM-Powered Architecture Generation**: Use Ollama-hosted models to generate architecture documents from natural language descriptions
2. **Template-Based Generation**: Support predefined architecture patterns (microservices, monolith, event-driven, etc.)
3. **Interactive Mode**: Guide users through architecture definition with prompts and suggestions
4. **Structured Output**: Generate markdown documents following the project's documentation standards
5. **Component Detection**: Automatically identify and document system components, layers, and interactions
6. **Incremental Refinement**: Allow iterative improvement of generated architectures

### User Workflow

```text
User provides → LLM generates → User reviews → Saves to file
requirements    architecture     and refines     system
```

**Example**: Developer types "Create a microservices architecture for an e-commerce platform with authentication, product catalog, and payment processing" and receives a complete architecture document with component diagrams, layer descriptions, and integration patterns.

## Technical Architecture

### Layer Design

Following xzagentz's layered architecture pattern:

```text
┌─────────────────────────────────────────────────────────────┐
│  CLI Layer (src/commands/architecture.rs)                   │
│  - architecture command with LLM subcommands                │
│  - Interactive prompts and argument parsing                 │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Application Layer (src/application/architecture_service.rs)│
│  - ArchitectureGenerationService                            │
│  - TemplateManagementService                                │
│  - RefinementWorkflow                                       │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Domain Layer (src/domain/architecture/)                    │
│  - ArchitectureDocument model                               │
│  - Component, Layer, Integration models                     │
│  - ArchitectureGenerator trait                              │
│  - ArchitectureTemplate trait                               │
│  - DocumentFormatter trait                                  │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Infrastructure Layer                                       │
│  - OllamaArchitectureGenerator (src/infrastructure/llm/)   │
│  - MarkdownArchitectureWriter (src/infrastructure/io/)     │
│  - TemplateRepository (src/infrastructure/templates/)      │
└─────────────────────────────────────────────────────────────┘
```

### Dependency Flow

```text
CLI → Application → Domain ← Infrastructure
                     ↑
                     └─ Auth Layer (for multi-tenant architectures)
```

**Critical Rules**:

- Domain layer contains NO infrastructure dependencies
- Infrastructure implements domain traits
- Application orchestrates domain and infrastructure

## Phase 1: Domain Models and Traits

### Phase 1.1: Core Architecture Domain Models

**File**: `src/domain/architecture/models.rs`

```rust
/// Represents a complete software architecture document
pub struct ArchitectureDocument {
    pub metadata: ArchitectureMetadata,
    pub overview: Overview,
    pub layers: Vec<Layer>,
    pub components: Vec<Component>,
    pub integrations: Vec<Integration>,
    pub deployment: Option<DeploymentArchitecture>,
    pub quality_attributes: Vec<QualityAttribute>,
}

/// Metadata about the architecture document
pub struct ArchitectureMetadata {
    pub title: String,
    pub version: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub model_used: String,
    pub pattern: ArchitecturePattern,
    pub authors: Vec<String>,
}

/// High-level overview section
pub struct Overview {
    pub description: String,
    pub business_goals: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
}

/// Architectural layer (presentation, application, domain, infrastructure)
pub struct Layer {
    pub name: String,
    pub description: String,
    pub responsibilities: Vec<String>,
    pub components: Vec<String>,
    pub dependencies: Vec<String>,
}

/// System component or service
pub struct Component {
    pub id: String,
    pub name: String,
    pub description: String,
    pub layer: String,
    pub responsibilities: Vec<String>,
    pub interfaces: Vec<Interface>,
    pub dependencies: Vec<String>,
    pub technology_stack: Vec<String>,
}

/// Component interface definition
pub struct Interface {
    pub name: String,
    pub protocol: String,
    pub endpoints: Vec<String>,
    pub data_formats: Vec<String>,
}

/// Integration between components
pub struct Integration {
    pub from_component: String,
    pub to_component: String,
    pub integration_type: IntegrationType,
    pub description: String,
    pub protocols: Vec<String>,
}

/// Type of integration pattern
pub enum IntegrationType {
    Synchronous,
    Asynchronous,
    EventDriven,
    DataSharing,
    BatchProcessing,
}

/// Deployment architecture details
pub struct DeploymentArchitecture {
    pub strategy: DeploymentStrategy,
    pub infrastructure: Vec<InfrastructureComponent>,
    pub scaling: ScalingStrategy,
    pub availability: AvailabilityDesign,
}

/// Infrastructure component (database, queue, cache, etc.)
pub struct InfrastructureComponent {
    pub name: String,
    pub component_type: String,
    pub purpose: String,
    pub configuration: String,
}

/// Quality attribute (performance, security, scalability, etc.)
pub struct QualityAttribute {
    pub name: String,
    pub description: String,
    pub tactics: Vec<String>,
    pub metrics: Vec<String>,
}

/// Common architecture patterns
pub enum ArchitecturePattern {
    Monolithic,
    Microservices,
    EventDriven,
    Layered,
    Hexagonal,
    CQRS,
    Serverless,
    Custom(String),
}

/// Deployment strategies
pub enum DeploymentStrategy {
    SingleInstance,
    LoadBalanced,
    Containerized,
    Kubernetes,
    Serverless,
}

/// Scaling approaches
pub enum ScalingStrategy {
    Vertical,
    Horizontal,
    AutoScaling,
    Hybrid,
}

/// High availability design
pub struct AvailabilityDesign {
    pub redundancy: String,
    pub failover: String,
    pub disaster_recovery: String,
}
```

**Tests Required**:

- Model serialization/deserialization
- Default implementations
- Validation logic

**Estimated Lines**: ~300 lines

### Phase 1.2: Domain Traits

**File**: `src/domain/architecture/traits.rs`

```rust
/// Trait for generating architecture documents from requirements
#[async_trait]
pub trait ArchitectureGenerator: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Generate architecture document from natural language requirements
    async fn generate(
        &self,
        requirements: &str,
        options: GenerationOptions,
    ) -> Result<ArchitectureDocument, Self::Error>;

    /// Refine existing architecture based on feedback
    async fn refine(
        &self,
        document: &ArchitectureDocument,
        refinement_prompt: &str,
    ) -> Result<ArchitectureDocument, Self::Error>;

    /// Generate from architecture pattern template
    async fn from_template(
        &self,
        template: &ArchitectureTemplate,
        customization: &str,
    ) -> Result<ArchitectureDocument, Self::Error>;
}

/// Options for architecture generation
pub struct GenerationOptions {
    pub pattern: Option<ArchitecturePattern>,
    pub complexity: ComplexityLevel,
    pub include_deployment: bool,
    pub include_quality_attributes: bool,
    pub max_components: usize,
    pub technology_preferences: Vec<String>,
}

/// Complexity level for generated architectures
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    Enterprise,
}

/// Trait for managing architecture templates
#[async_trait]
pub trait TemplateRepository: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// List available templates
    async fn list_templates(&self) -> Result<Vec<TemplateInfo>, Self::Error>;

    /// Load a template by name
    async fn load_template(&self, name: &str) -> Result<ArchitectureTemplate, Self::Error>;

    /// Save a new template
    async fn save_template(&self, template: &ArchitectureTemplate) -> Result<(), Self::Error>;
}

/// Information about an architecture template
pub struct TemplateInfo {
    pub name: String,
    pub pattern: ArchitecturePattern,
    pub description: String,
    pub use_cases: Vec<String>,
}

/// Architecture template structure
pub struct ArchitectureTemplate {
    pub info: TemplateInfo,
    pub structure: TemplateStructure,
    pub default_components: Vec<ComponentTemplate>,
    pub customization_points: Vec<String>,
}

/// Template structural definition
pub struct TemplateStructure {
    pub layers: Vec<String>,
    pub integration_patterns: Vec<String>,
    pub required_components: Vec<String>,
}

/// Component template definition
pub struct ComponentTemplate {
    pub name: String,
    pub layer: String,
    pub role: String,
    pub typical_technologies: Vec<String>,
}

/// Trait for writing architecture documents to files
#[async_trait]
pub trait ArchitectureWriter: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Write architecture document to file
    async fn write(
        &self,
        document: &ArchitectureDocument,
        path: &std::path::Path,
    ) -> Result<(), Self::Error>;

    /// Format architecture as string
    fn format(&self, document: &ArchitectureDocument) -> Result<String, Self::Error>;
}
```

**Tests Required**:

- Trait implementation verification
- Mock implementations for testing
- Option validation

**Estimated Lines**: ~200 lines

### Phase 1.3: Architecture Validation

**File**: `src/domain/architecture/validation.rs`

```rust
/// Validates architecture documents for completeness and consistency
pub struct ArchitectureValidator;

impl ArchitectureValidator {
    /// Validate complete architecture document
    pub fn validate(document: &ArchitectureDocument) -> Result<(), ValidationError> {
        Self::validate_metadata(&document.metadata)?;
        Self::validate_overview(&document.overview)?;
        Self::validate_layers(&document.layers)?;
        Self::validate_components(&document.components)?;
        Self::validate_integrations(&document.integrations, &document.components)?;
        Ok(())
    }

    /// Validate metadata completeness
    fn validate_metadata(metadata: &ArchitectureMetadata) -> Result<(), ValidationError> {
        if metadata.title.trim().is_empty() {
            return Err(ValidationError::EmptyTitle);
        }
        Ok(())
    }

    /// Validate overview section
    fn validate_overview(overview: &Overview) -> Result<(), ValidationError> {
        if overview.description.trim().is_empty() {
            return Err(ValidationError::EmptyDescription);
        }
        Ok(())
    }

    /// Validate layer definitions
    fn validate_layers(layers: &[Layer]) -> Result<(), ValidationError> {
        if layers.is_empty() {
            return Err(ValidationError::NoLayers);
        }
        Ok(())
    }

    /// Validate components
    fn validate_components(components: &[Component]) -> Result<(), ValidationError> {
        if components.is_empty() {
            return Err(ValidationError::NoComponents);
        }

        for component in components {
            if component.name.trim().is_empty() {
                return Err(ValidationError::EmptyComponentName);
            }
        }
        Ok(())
    }

    /// Validate integrations reference valid components
    fn validate_integrations(
        integrations: &[Integration],
        components: &[Component],
    ) -> Result<(), ValidationError> {
        let component_ids: std::collections::HashSet<_> =
            components.iter().map(|c| &c.id).collect();

        for integration in integrations {
            if !component_ids.contains(&integration.from_component) {
                return Err(ValidationError::InvalidComponentReference(
                    integration.from_component.clone()
                ));
            }
            if !component_ids.contains(&integration.to_component) {
                return Err(ValidationError::InvalidComponentReference(
                    integration.to_component.clone()
                ));
            }
        }
        Ok(())
    }
}

/// Validation errors
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Architecture title cannot be empty")]
    EmptyTitle,

    #[error("Architecture description cannot be empty")]
    EmptyDescription,

    #[error("Architecture must have at least one layer")]
    NoLayers,

    #[error("Architecture must have at least one component")]
    NoComponents,

    #[error("Component name cannot be empty")]
    EmptyComponentName,

    #[error("Invalid component reference: {0}")]
    InvalidComponentReference(String),
}
```

**Tests Required**:

- Validation success cases
- Each validation error case
- Edge cases

**Estimated Lines**: ~150 lines

**Phase 1 Total**: ~650 lines of domain code + ~300 lines of tests

## Phase 2: Infrastructure - Ollama Integration

### Phase 2.1: Ollama Architecture Generator

**File**: `src/infrastructure/llm/ollama_architecture_generator.rs`

````rust
use crate::domain::architecture::{
    ArchitectureDocument, ArchitectureGenerator, GenerationOptions,
    ArchitectureTemplate,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Ollama-based architecture generator
pub struct OllamaArchitectureGenerator {
    client: Client,
    base_url: String,
    default_model: String,
}

impl OllamaArchitectureGenerator {
    /// Create new Ollama architecture generator
    pub fn new(base_url: String, default_model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            default_model,
        }
    }

    /// Build system prompt for architecture generation
    fn build_system_prompt(&self, options: &GenerationOptions) -> String {
        format!(
            r#"You are an expert software architect. Generate a comprehensive, \
            professional software architecture document in JSON format.

Your response must be valid JSON matching this structure:
{{
  "title": "Architecture Title",
  "overview": {{
    "description": "High-level description",
    "business_goals": ["goal1", "goal2"],
    "constraints": ["constraint1"],
    "assumptions": ["assumption1"]
  }},
  "layers": [
    {{
      "name": "Layer Name",
      "description": "Layer description",
      "responsibilities": ["resp1", "resp2"],
      "components": ["component1"],
      "dependencies": ["lower_layer"]
    }}
  ],
  "components": [
    {{
      "id": "component-id",
      "name": "Component Name",
      "description": "Component description",
      "layer": "layer-name",
      "responsibilities": ["resp1"],
      "interfaces": [
        {{
          "name": "API",
          "protocol": "HTTP/REST",
          "endpoints": ["/api/v1/resource"],
          "data_formats": ["JSON"]
        }}
      ],
      "dependencies": ["other-component"],
      "technology_stack": ["Rust", "PostgreSQL"]
    }}
  ],
  "integrations": [
    {{
      "from_component": "component1",
      "to_component": "component2",
      "integration_type": "Synchronous",
      "description": "Integration description",
      "protocols": ["HTTP"]
    }}
  ]
}}

Pattern preference: {:?}
Complexity level: {:?}
Include deployment: {}
Max components: {}
"#,
            options.pattern,
            options.complexity,
            options.include_deployment,
            options.max_components
        )
    }

    /// Parse LLM response into architecture document
    fn parse_response(&self, response: &str) -> Result<ArchitectureDocument, GeneratorError> {
        // Extract JSON from markdown code blocks if present
        let json_str = if response.contains("```json") {
            response
                .split("```json")
                .nth(1)
                .and_then(|s| s.split("```").next())
                .unwrap_or(response)
        } else if response.contains("```") {
            response
                .split("```")
                .nth(1)
                .and_then(|s| s.split("```").next())
                .unwrap_or(response)
        } else {
            response
        }
        .trim();

        let parsed: ArchitectureResponse = serde_json::from_str(json_str)
            .map_err(|e| GeneratorError::ParseError(e.to_string()))?;

        Ok(self.convert_response_to_document(parsed))
    }

    /// Convert API response to domain model
    fn convert_response_to_document(&self, response: ArchitectureResponse) -> ArchitectureDocument {
        // Conversion logic implementation
        todo!("Implement conversion from response to ArchitectureDocument")
    }
}

#[async_trait]
impl ArchitectureGenerator for OllamaArchitectureGenerator {
    type Error = GeneratorError;

    async fn generate(
        &self,
        requirements: &str,
        options: GenerationOptions,
    ) -> Result<ArchitectureDocument, Self::Error> {
        let system_prompt = self.build_system_prompt(&options);
        let user_prompt = format!("Generate a software architecture for: {}", requirements);

        let request = OllamaGenerateRequest {
            model: self.default_model.clone(),
            prompt: format!("{}\n\nUser Requirements:\n{}", system_prompt, user_prompt),
            stream: false,
            options: Some(OllamaOptions {
                temperature: 0.7,
                top_p: 0.9,
                num_predict: 4096,
            }),
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| GeneratorError::NetworkError(e.to_string()))?;

        let ollama_response: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| GeneratorError::ParseError(e.to_string()))?;

        self.parse_response(&ollama_response.response)
    }

    async fn refine(
        &self,
        document: &ArchitectureDocument,
        refinement_prompt: &str,
    ) -> Result<ArchitectureDocument, Self::Error> {
        // Serialize current document
        let current_json = serde_json::to_string_pretty(document)
            .map_err(|e| GeneratorError::SerializationError(e.to_string()))?;

        let prompt = format!(
            r#"Given this existing architecture document:

{}

Apply the following refinement:
{}

Return the complete updated architecture in the same JSON format."#,
            current_json, refinement_prompt
        );

        let options = GenerationOptions::default();
        self.generate(&prompt, options).await
    }

    async fn from_template(
        &self,
        template: &ArchitectureTemplate,
        customization: &str,
    ) -> Result<ArchitectureDocument, Self::Error> {
        let template_description = format!(
            "Template: {}\nPattern: {:?}\nDescription: {}\nStructure: Layers: {:?}, Default Components: {:?}\n\nCustomization: {}",
            template.info.name,
            template.info.pattern,
            template.info.description,
            template.structure.layers,
            template.default_components.iter().map(|c| &c.name).collect::<Vec<_>>(),
            customization
        );

        let options = GenerationOptions {
            pattern: Some(template.info.pattern.clone()),
            ..Default::default()
        };

        self.generate(&template_description, options).await
    }
}

/// Ollama API request structure
#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<OllamaOptions>,
}

/// Ollama generation options
#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    top_p: f32,
    num_predict: usize,
}

/// Ollama API response structure
#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    model: String,
    done: bool,
}

/// Response structure from LLM
#[derive(Debug, Deserialize)]
struct ArchitectureResponse {
    title: String,
    overview: OverviewResponse,
    layers: Vec<LayerResponse>,
    components: Vec<ComponentResponse>,
    integrations: Vec<IntegrationResponse>,
}

#[derive(Debug, Deserialize)]
struct OverviewResponse {
    description: String,
    business_goals: Vec<String>,
    constraints: Vec<String>,
    assumptions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LayerResponse {
    name: String,
    description: String,
    responsibilities: Vec<String>,
    components: Vec<String>,
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ComponentResponse {
    id: String,
    name: String,
    description: String,
    layer: String,
    responsibilities: Vec<String>,
    interfaces: Vec<InterfaceResponse>,
    dependencies: Vec<String>,
    technology_stack: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct InterfaceResponse {
    name: String,
    protocol: String,
    endpoints: Vec<String>,
    data_formats: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct IntegrationResponse {
    from_component: String,
    to_component: String,
    integration_type: String,
    description: String,
    protocols: Vec<String>,
}

/// Generator errors
#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Failed to serialize document: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
````

**Tests Required**:

- Mock Ollama responses
- JSON parsing edge cases
- Error handling
- Template application

**Estimated Lines**: ~400 lines

### Phase 2.2: Template Repository Implementation

**File**: `src/infrastructure/templates/file_template_repository.rs`

```rust
use crate::domain::architecture::{
    ArchitectureTemplate, TemplateInfo, TemplateRepository,
};
use async_trait::async_trait;
use std::path::PathBuf;

/// File-based template repository
pub struct FileTemplateRepository {
    templates_dir: PathBuf,
}

impl FileTemplateRepository {
    /// Create new file-based template repository
    pub fn new(templates_dir: PathBuf) -> Self {
        Self { templates_dir }
    }

    /// Ensure templates directory exists
    pub async fn initialize(&self) -> Result<(), RepositoryError> {
        tokio::fs::create_dir_all(&self.templates_dir)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        // Create default templates if none exist
        self.create_default_templates().await?;

        Ok(())
    }

    /// Create default architecture templates
    async fn create_default_templates(&self) -> Result<(), RepositoryError> {
        // Check if templates already exist
        let templates = self.list_templates().await?;
        if !templates.is_empty() {
            return Ok(());
        }

        // Create default templates
        let defaults = vec![
            self.create_microservices_template(),
            self.create_monolithic_template(),
            self.create_event_driven_template(),
            self.create_layered_template(),
        ];

        for template in defaults {
            self.save_template(&template).await?;
        }

        Ok(())
    }

    fn create_microservices_template(&self) -> ArchitectureTemplate {
        // Template definition
        todo!("Implement microservices template")
    }

    fn create_monolithic_template(&self) -> ArchitectureTemplate {
        // Template definition
        todo!("Implement monolithic template")
    }

    fn create_event_driven_template(&self) -> ArchitectureTemplate {
        // Template definition
        todo!("Implement event-driven template")
    }

    fn create_layered_template(&self) -> ArchitectureTemplate {
        // Template definition
        todo!("Implement layered template")
    }
}

#[async_trait]
impl TemplateRepository for FileTemplateRepository {
    type Error = RepositoryError;

    async fn list_templates(&self) -> Result<Vec<TemplateInfo>, Self::Error> {
        let mut entries = tokio::fs::read_dir(&self.templates_dir)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        let mut templates = Vec::new();

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let template = self.load_template_from_path(&path).await?;
                templates.push(template.info);
            }
        }

        Ok(templates)
    }

    async fn load_template(&self, name: &str) -> Result<ArchitectureTemplate, Self::Error> {
        let path = self.templates_dir.join(format!("{}.yaml", name));
        self.load_template_from_path(&path).await
    }

    async fn save_template(&self, template: &ArchitectureTemplate) -> Result<(), Self::Error> {
        let path = self.templates_dir.join(format!("{}.yaml", template.info.name));
        let yaml = serde_yaml::to_string(template)
            .map_err(|e| RepositoryError::SerializationError(e.to_string()))?;

        tokio::fs::write(&path, yaml)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        Ok(())
    }
}

impl FileTemplateRepository {
    async fn load_template_from_path(
        &self,
        path: &PathBuf,
    ) -> Result<ArchitectureTemplate, RepositoryError> {
        let contents = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        let template: ArchitectureTemplate = serde_yaml::from_str(&contents)
            .map_err(|e| RepositoryError::ParseError(e.to_string()))?;

        Ok(template)
    }
}

/// Repository errors
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Template not found: {0}")]
    NotFound(String),
}
```

**Tests Required**:

- Template CRUD operations
- Default template generation
- File system error handling

**Estimated Lines**: ~300 lines

### Phase 2.3: Markdown Architecture Writer

**File**: `src/infrastructure/io/markdown_architecture_writer.rs`

```rust
use crate::domain::architecture::{ArchitectureDocument, ArchitectureWriter};
use async_trait::async_trait;
use std::path::Path;

/// Markdown-based architecture document writer
pub struct MarkdownArchitectureWriter;

impl MarkdownArchitectureWriter {
    pub fn new() -> Self {
        Self
    }

    /// Format metadata section
    fn format_metadata(&self, document: &ArchitectureDocument) -> String {
        format!(
            r#"# {}

**Version**: {}
**Generated**: {}
**Model**: {}
**Pattern**: {:?}
"#,
            document.metadata.title,
            document.metadata.version,
            document.metadata.generated_at.format("%Y-%m-%d %H:%M:%S UTC"),
            document.metadata.model_used,
            document.metadata.pattern
        )
    }

    /// Format overview section
    fn format_overview(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::from("\n## Overview\n\n");
        output.push_str(&document.overview.description);
        output.push_str("\n\n### Business Goals\n\n");
        for goal in &document.overview.business_goals {
            output.push_str(&format!("- {}\n", goal));
        }
        output.push_str("\n### Constraints\n\n");
        for constraint in &document.overview.constraints {
            output.push_str(&format!("- {}\n", constraint));
        }
        output.push_str("\n### Assumptions\n\n");
        for assumption in &document.overview.assumptions {
            output.push_str(&format!("- {}\n", assumption));
        }
        output
    }

    /// Format layers section
    fn format_layers(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::from("\n## Architecture Layers\n\n");
        for layer in &document.layers {
            output.push_str(&format!("### {}\n\n", layer.name));
            output.push_str(&format!("{}\n\n", layer.description));
            output.push_str("**Responsibilities**:\n\n");
            for resp in &layer.responsibilities {
                output.push_str(&format!("- {}\n", resp));
            }
            output.push_str("\n**Components**: ");
            output.push_str(&layer.components.join(", "));
            output.push_str("\n\n");
            if !layer.dependencies.is_empty() {
                output.push_str("**Dependencies**: ");
                output.push_str(&layer.dependencies.join(", "));
                output.push_str("\n\n");
            }
        }
        output
    }

    /// Format components section
    fn format_components(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::from("\n## Components\n\n");
        for component in &document.components {
            output.push_str(&format!("### {} ({})\n\n", component.name, component.id));
            output.push_str(&format!("**Layer**: {}\n\n", component.layer));
            output.push_str(&format!("{}\n\n", component.description));
            output.push_str("**Responsibilities**:\n\n");
            for resp in &component.responsibilities {
                output.push_str(&format!("- {}\n", resp));
            }
            if !component.interfaces.is_empty() {
                output.push_str("\n**Interfaces**:\n\n");
                for interface in &component.interfaces {
                    output.push_str(&format!("- **{}** ({})\n", interface.name, interface.protocol));
                    for endpoint in &interface.endpoints {
                        output.push_str(&format!("  - {}\n", endpoint));
                    }
                }
            }
            if !component.technology_stack.is_empty() {
                output.push_str("\n**Technology Stack**: ");
                output.push_str(&component.technology_stack.join(", "));
                output.push_str("\n");
            }
            output.push_str("\n");
        }
        output
    }

    /// Format integrations section
    fn format_integrations(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::from("\n## Component Integrations\n\n");
        for integration in &document.integrations {
            output.push_str(&format!(
                "### {} → {}\n\n",
                integration.from_component, integration.to_component
            ));
            output.push_str(&format!("**Type**: {:?}\n\n", integration.integration_type));
            output.push_str(&format!("{}\n\n", integration.description));
            output.push_str("**Protocols**: ");
            output.push_str(&integration.protocols.join(", "));
            output.push_str("\n\n");
        }
        output
    }

    /// Format quality attributes section
    fn format_quality_attributes(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::from("\n## Quality Attributes\n\n");
        for qa in &document.quality_attributes {
            output.push_str(&format!("### {}\n\n", qa.name));
            output.push_str(&format!("{}\n\n", qa.description));
            if !qa.tactics.is_empty() {
                output.push_str("**Tactics**:\n\n");
                for tactic in &qa.tactics {
                    output.push_str(&format!("- {}\n", tactic));
                }
            }
            if !qa.metrics.is_empty() {
                output.push_str("\n**Metrics**:\n\n");
                for metric in &qa.metrics {
                    output.push_str(&format!("- {}\n", metric));
                }
            }
            output.push_str("\n");
        }
        output
    }
}

#[async_trait]
impl ArchitectureWriter for MarkdownArchitectureWriter {
    type Error = WriterError;

    async fn write(
        &self,
        document: &ArchitectureDocument,
        path: &Path,
    ) -> Result<(), Self::Error> {
        let content = self.format(document)?;
        tokio::fs::write(path, content)
            .await
            .map_err(|e| WriterError::IoError(e.to_string()))?;
        Ok(())
    }

    fn format(&self, document: &ArchitectureDocument) -> Result<String, Self::Error> {
        let mut output = String::new();
        output.push_str(&self.format_metadata(document));
        output.push_str(&self.format_overview(document));
        output.push_str(&self.format_layers(document));
        output.push_str(&self.format_components(document));
        output.push_str(&self.format_integrations(document));
        output.push_str(&self.format_quality_attributes(document));
        Ok(output)
    }
}

/// Writer errors
#[derive(Debug, thiserror::Error)]
pub enum WriterError {
    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Formatting error: {0}")]
    FormatError(String),
}
```

**Tests Required**:

- Markdown formatting correctness
- File write operations
- Error handling

**Estimated Lines**: ~250 lines

**Phase 2 Total**: ~950 lines of infrastructure code + ~400 lines of tests

## Phase 3: Application Layer

### Phase 3.1: Architecture Generation Service

**File**: `src/application/architecture_service.rs`

```rust
use crate::domain::architecture::{
    ArchitectureDocument, ArchitectureGenerator, ArchitectureWriter,
    GenerationOptions, TemplateRepository, ArchitectureValidator,
};
use std::path::Path;
use std::sync::Arc;

/// Service for generating and managing architecture documents
pub struct ArchitectureService {
    generator: Arc<dyn ArchitectureGenerator<Error = GeneratorError>>,
    writer: Arc<dyn ArchitectureWriter<Error = WriterError>>,
    template_repo: Arc<dyn TemplateRepository<Error = RepositoryError>>,
}

impl ArchitectureService {
    /// Create new architecture service
    pub fn new(
        generator: Arc<dyn ArchitectureGenerator<Error = GeneratorError>>,
        writer: Arc<dyn ArchitectureWriter<Error = WriterError>>,
        template_repo: Arc<dyn TemplateRepository<Error = RepositoryError>>,
    ) -> Self {
        Self {
            generator,
            writer,
            template_repo,
        }
    }

    /// Generate architecture from requirements
    pub async fn generate_from_requirements(
        &self,
        requirements: &str,
        options: GenerationOptions,
    ) -> Result<ArchitectureDocument, ServiceError> {
        let document = self
            .generator
            .generate(requirements, options)
            .await
            .map_err(ServiceError::GenerationError)?;

        ArchitectureValidator::validate(&document)
            .map_err(ServiceError::ValidationError)?;

        Ok(document)
    }

    /// Generate architecture from template
    pub async fn generate_from_template(
        &self,
        template_name: &str,
        customization: &str,
    ) -> Result<ArchitectureDocument, ServiceError> {
        let template = self
            .template_repo
            .load_template(template_name)
            .await
            .map_err(ServiceError::TemplateError)?;

        let document = self
            .generator
            .from_template(&template, customization)
            .await
            .map_err(ServiceError::GenerationError)?;

        ArchitectureValidator::validate(&document)
            .map_err(ServiceError::ValidationError)?;

        Ok(document)
    }

    /// Refine existing architecture
    pub async fn refine_architecture(
        &self,
        document: &ArchitectureDocument,
        refinement_prompt: &str,
    ) -> Result<ArchitectureDocument, ServiceError> {
        let refined = self
            .generator
            .refine(document, refinement_prompt)
            .await
            .map_err(ServiceError::GenerationError)?;

        ArchitectureValidator::validate(&refined)
            .map_err(ServiceError::ValidationError)?;

        Ok(refined)
    }

    /// Save architecture to file
    pub async fn save_architecture(
        &self,
        document: &ArchitectureDocument,
        path: &Path,
    ) -> Result<(), ServiceError> {
        self.writer
            .write(document, path)
            .await
            .map_err(ServiceError::WriteError)?;

        Ok(())
    }

    /// List available templates
    pub async fn list_templates(&self) -> Result<Vec<TemplateInfo>, ServiceError> {
        self.template_repo
            .list_templates()
            .await
            .map_err(ServiceError::TemplateError)
    }
}

/// Service errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Generation error: {0}")]
    GenerationError(GeneratorError),

    #[error("Validation error: {0}")]
    ValidationError(ValidationError),

    #[error("Template error: {0}")]
    TemplateError(RepositoryError),

    #[error("Write error: {0}")]
    WriteError(WriterError),
}
```

**Tests Required**:

- Service orchestration
- Error propagation
- Mock dependencies

**Estimated Lines**: ~200 lines

### Phase 3.2: Interactive Architecture Session

**File**: `src/application/interactive_architecture_session.rs`

```rust
use crate::application::architecture_service::ArchitectureService;
use crate::domain::architecture::{
    ArchitectureDocument, GenerationOptions, ComplexityLevel,
    ArchitecturePattern,
};
use dialoguer::{Input, Select, Confirm, theme::ColorfulTheme};
use std::path::PathBuf;
use std::sync::Arc;

/// Interactive session for architecture generation
pub struct InteractiveArchitectureSession {
    service: Arc<ArchitectureService>,
    theme: ColorfulTheme,
}

impl InteractiveArchitectureSession {
    /// Create new interactive session
    pub fn new(service: Arc<ArchitectureService>) -> Self {
        Self {
            service,
            theme: ColorfulTheme::default(),
        }
    }

    /// Run interactive architecture generation workflow
    pub async fn run(&self) -> Result<(), SessionError> {
        println!("=== Interactive Architecture Generation ===\n");

        // Step 1: Choose generation mode
        let mode = self.select_generation_mode()?;

        let document = match mode {
            GenerationMode::FromRequirements => self.generate_from_requirements_interactive().await?,
            GenerationMode::FromTemplate => self.generate_from_template_interactive().await?,
        };

        // Step 2: Review and refine
        let final_document = self.review_and_refine_loop(document).await?;

        // Step 3: Save to file
        self.save_document_interactive(&final_document).await?;

        println!("\nArchitecture document generated successfully!");

        Ok(())
    }

    /// Select generation mode
    fn select_generation_mode(&self) -> Result<GenerationMode, SessionError> {
        let options = vec!["From requirements", "From template"];
        let selection = Select::with_theme(&self.theme)
            .with_prompt("How would you like to generate the architecture?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(match selection {
            0 => GenerationMode::FromRequirements,
            1 => GenerationMode::FromTemplate,
            _ => unreachable!(),
        })
    }

    /// Generate from requirements interactively
    async fn generate_from_requirements_interactive(
        &self,
    ) -> Result<ArchitectureDocument, SessionError> {
        let requirements: String = Input::with_theme(&self.theme)
            .with_prompt("Describe your system requirements")
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        let pattern = self.select_architecture_pattern()?;
        let complexity = self.select_complexity_level()?;

        let options = GenerationOptions {
            pattern: Some(pattern),
            complexity,
            include_deployment: self.prompt_yes_no("Include deployment architecture?")?,
            include_quality_attributes: self.prompt_yes_no("Include quality attributes?")?,
            max_components: 20,
            technology_preferences: self.prompt_technology_preferences()?,
        };

        println!("\nGenerating architecture...");
        let document = self
            .service
            .generate_from_requirements(&requirements, options)
            .await
            .map_err(SessionError::ServiceError)?;

        Ok(document)
    }

    /// Generate from template interactively
    async fn generate_from_template_interactive(
        &self,
    ) -> Result<ArchitectureDocument, SessionError> {
        let templates = self
            .service
            .list_templates()
            .await
            .map_err(SessionError::ServiceError)?;

        if templates.is_empty() {
            return Err(SessionError::NoTemplatesAvailable);
        }

        let template_names: Vec<String> = templates
            .iter()
            .map(|t| format!("{} - {}", t.name, t.description))
            .collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select an architecture template")
            .items(&template_names)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        let template_name = &templates[selection].name;

        let customization: String = Input::with_theme(&self.theme)
            .with_prompt("Describe any customizations")
            .default(String::new())
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        println!("\nGenerating architecture from template...");
        let document = self
            .service
            .generate_from_template(template_name, &customization)
            .await
            .map_err(SessionError::ServiceError)?;

        Ok(document)
    }

    /// Review and refine loop
    async fn review_and_refine_loop(
        &self,
        mut document: ArchitectureDocument,
    ) -> Result<ArchitectureDocument, SessionError> {
        loop {
            self.display_document_summary(&document);

            let action = self.select_review_action()?;

            match action {
                ReviewAction::Accept => break,
                ReviewAction::Refine => {
                    let refinement: String = Input::with_theme(&self.theme)
                        .with_prompt("What would you like to refine?")
                        .interact_text()
                        .map_err(|e| SessionError::InteractionError(e.to_string()))?;

                    println!("\nRefining architecture...");
                    document = self
                        .service
                        .refine_architecture(&document, &refinement)
                        .await
                        .map_err(SessionError::ServiceError)?;
                }
                ReviewAction::Cancel => {
                    return Err(SessionError::Cancelled);
                }
            }
        }

        Ok(document)
    }

    /// Display document summary
    fn display_document_summary(&self, document: &ArchitectureDocument) {
        println!("\n=== Architecture Summary ===");
        println!("Title: {}", document.metadata.title);
        println!("Pattern: {:?}", document.metadata.pattern);
        println!("Layers: {}", document.layers.len());
        println!("Components: {}", document.components.len());
        println!("Integrations: {}", document.integrations.len());
        println!();
    }

    /// Select review action
    fn select_review_action(&self) -> Result<ReviewAction, SessionError> {
        let options = vec!["Accept and save", "Refine further", "Cancel"];
        let selection = Select::with_theme(&self.theme)
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(match selection {
            0 => ReviewAction::Accept,
            1 => ReviewAction::Refine,
            2 => ReviewAction::Cancel,
            _ => unreachable!(),
        })
    }

    /// Save document interactively
    async fn save_document_interactive(
        &self,
        document: &ArchitectureDocument,
    ) -> Result<(), SessionError> {
        let default_path = format!("docs/architecture/{}.md", document.metadata.title.to_lowercase().replace(" ", "_"));

        let path: String = Input::with_theme(&self.theme)
            .with_prompt("Output file path")
            .default(default_path)
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        let path_buf = PathBuf::from(path);

        // Create parent directories if needed
        if let Some(parent) = path_buf.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| SessionError::IoError(e.to_string()))?;
        }

        self.service
            .save_architecture(document, &path_buf)
            .await
            .map_err(SessionError::ServiceError)?;

        println!("\nArchitecture saved to: {}", path_buf.display());

        Ok(())
    }

    /// Helper: select architecture pattern
    fn select_architecture_pattern(&self) -> Result<ArchitecturePattern, SessionError> {
        let options = vec![
            "Microservices",
            "Monolithic",
            "Event-Driven",
            "Layered",
            "Hexagonal",
            "CQRS",
            "Serverless",
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select architecture pattern")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(match selection {
            0 => ArchitecturePattern::Microservices,
            1 => ArchitecturePattern::Monolithic,
            2 => ArchitecturePattern::EventDriven,
            3 => ArchitecturePattern::Layered,
            4 => ArchitecturePattern::Hexagonal,
            5 => ArchitecturePattern::CQRS,
            6 => ArchitecturePattern::Serverless,
            _ => unreachable!(),
        })
    }

    /// Helper: select complexity level
    fn select_complexity_level(&self) -> Result<ComplexityLevel, SessionError> {
        let options = vec!["Simple", "Moderate", "Complex", "Enterprise"];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select complexity level")
            .items(&options)
            .default(1)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(match selection {
            0 => ComplexityLevel::Simple,
            1 => ComplexityLevel::Moderate,
            2 => ComplexityLevel::Complex,
            3 => ComplexityLevel::Enterprise,
            _ => unreachable!(),
        })
    }

    /// Helper: prompt yes/no question
    fn prompt_yes_no(&self, message: &str) -> Result<bool, SessionError> {
        Confirm::with_theme(&self.theme)
            .with_prompt(message)
            .default(true)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))
    }

    /// Helper: prompt technology preferences
    fn prompt_technology_preferences(&self) -> Result<Vec<String>, SessionError> {
        let input: String = Input::with_theme(&self.theme)
            .with_prompt("Technology preferences (comma-separated, optional)")
            .default(String::new())
            .allow_empty(true)
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        if input.trim().is_empty() {
            Ok(Vec::new())
        } else {
            Ok(input.split(',').map(|s| s.trim().to_string()).collect())
        }
    }
}

/// Generation mode selection
enum GenerationMode {
    FromRequirements,
    FromTemplate,
}

/// Review action selection
enum ReviewAction {
    Accept,
    Refine,
    Cancel,
}

/// Session errors
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Interaction error: {0}")]
    InteractionError(String),

    #[error("Service error: {0}")]
    ServiceError(ServiceError),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("No templates available")]
    NoTemplatesAvailable,

    #[error("User cancelled operation")]
    Cancelled,
}
```

**Tests Required**:

- Mock user interactions
- Session flow validation
- Error handling

**Estimated Lines**: ~400 lines

**Phase 3 Total**: ~600 lines of application code + ~300 lines of tests

## Phase 4: CLI Integration

### Phase 4.1: Architecture Command Structure

**File**: `src/commands/architecture.rs`

```rust
use clap::{Args, Subcommand};
use crate::application::{ArchitectureService, InteractiveArchitectureSession};
use crate::domain::architecture::{GenerationOptions, ComplexityLevel, ArchitecturePattern};
use std::path::PathBuf;
use std::sync::Arc;

/// Architecture command with subcommands
#[derive(Debug, Args)]
pub struct ArchitectureCommand {
    #[command(subcommand)]
    pub action: ArchitectureAction,
}

/// Architecture subcommands
#[derive(Debug, Subcommand)]
pub enum ArchitectureAction {
    /// Generate architecture using LLM
    Generate(GenerateArgs),

    /// List available architecture templates
    ListTemplates(ListTemplatesArgs),

    /// Refine existing architecture document
    Refine(RefineArgs),

    /// Validate architecture document
    Validate(ValidateArgs),
}

/// Arguments for generate subcommand
#[derive(Debug, Args)]
pub struct GenerateArgs {
    /// Requirements or description of the system
    #[arg(short, long)]
    pub requirements: Option<String>,

    /// Use architecture template
    #[arg(short, long)]
    pub template: Option<String>,

    /// Template customization
    #[arg(short = 'c', long)]
    pub customization: Option<String>,

    /// Architecture pattern to use
    #[arg(short, long, value_enum)]
    pub pattern: Option<PatternArg>,

    /// Complexity level
    #[arg(long, value_enum, default_value = "moderate")]
    pub complexity: ComplexityArg,

    /// Include deployment architecture
    #[arg(long, default_value = "true")]
    pub include_deployment: bool,

    /// Include quality attributes
    #[arg(long, default_value = "true")]
    pub include_quality: bool,

    /// Maximum number of components
    #[arg(long, default_value = "20")]
    pub max_components: usize,

    /// Technology preferences (comma-separated)
    #[arg(long)]
    pub tech_stack: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// LLM model to use
    #[arg(short, long, default_value = "llama2")]
    pub model: String,

    /// Ollama base URL
    #[arg(long, default_value = "http://localhost:11434")]
    pub ollama_url: String,

    /// Interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Skip Ollama health check
    #[arg(long)]
    pub skip_health_check: bool,

    /// Force overwrite existing file
    #[arg(short, long)]
    pub force: bool,
}

/// Pattern argument enum
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum PatternArg {
    Microservices,
    Monolithic,
    EventDriven,
    Layered,
    Hexagonal,
    Cqrs,
    Serverless,
}

/// Complexity argument enum
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ComplexityArg {
    Simple,
    Moderate,
    Complex,
    Enterprise,
}

/// Arguments for list-templates subcommand
#[derive(Debug, Args)]
pub struct ListTemplatesArgs {
    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,

    /// Filter by pattern
    #[arg(short, long)]
    pub pattern: Option<PatternArg>,
}

/// Arguments for refine subcommand
#[derive(Debug, Args)]
pub struct RefineArgs {
    /// Input architecture document path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Refinement instructions
    #[arg(short, long)]
    pub refinement: String,

    /// Output file path (defaults to input path)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// LLM model to use
    #[arg(short, long, default_value = "llama2")]
    pub model: String,

    /// Ollama base URL
    #[arg(long, default_value = "http://localhost:11434")]
    pub ollama_url: String,

    /// Create backup of original file
    #[arg(short, long)]
    pub backup: bool,
}

/// Arguments for validate subcommand
#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Architecture document path to validate
    #[arg(short, long)]
    pub input: PathBuf,

    /// Show detailed validation results
    #[arg(short, long)]
    pub verbose: bool,
}

/// Execute architecture command
pub async fn execute(command: ArchitectureCommand) -> Result<(), CommandError> {
    match command.action {
        ArchitectureAction::Generate(args) => execute_generate(args).await,
        ArchitectureAction::ListTemplates(args) => execute_list_templates(args).await,
        ArchitectureAction::Refine(args) => execute_refine(args).await,
        ArchitectureAction::Validate(args) => execute_validate(args).await,
    }
}

/// Execute generate subcommand
async fn execute_generate(args: GenerateArgs) -> Result<(), CommandError> {
    // Check output file exists
    if args.output.exists() && !args.force {
        return Err(CommandError::OutputFileExists(args.output));
    }

    // Initialize services
    let service = create_architecture_service(&args.ollama_url, &args.model)?;

    // Health check
    if !args.skip_health_check {
        check_ollama_health(&args.ollama_url).await?;
    }

    // Interactive or direct mode
    if args.interactive {
        let session = InteractiveArchitectureSession::new(Arc::new(service));
        session.run().await.map_err(CommandError::SessionError)?;
    } else {
        // Validate arguments
        if args.requirements.is_none() && args.template.is_none() {
            return Err(CommandError::MissingRequirements);
        }

        let document = if let Some(template) = args.template {
            service
                .generate_from_template(&template, &args.customization.unwrap_or_default())
                .await
                .map_err(CommandError::ServiceError)?
        } else {
            let options = GenerationOptions {
                pattern: args.pattern.map(convert_pattern_arg),
                complexity: convert_complexity_arg(args.complexity),
                include_deployment: args.include_deployment,
                include_quality_attributes: args.include_quality,
                max_components: args.max_components,
                technology_preferences: parse_tech_stack(args.tech_stack),
            };

            service
                .generate_from_requirements(&args.requirements.unwrap(), options)
                .await
                .map_err(CommandError::ServiceError)?
        };

        // Save to file
        service
            .save_architecture(&document, &args.output)
            .await
            .map_err(CommandError::ServiceError)?;

        println!("Architecture generated successfully: {}", args.output.display());
    }

    Ok(())
}

/// Execute list-templates subcommand
async fn execute_list_templates(args: ListTemplatesArgs) -> Result<(), CommandError> {
    let service = create_architecture_service("http://localhost:11434", "llama2")?;
    let templates = service
        .list_templates()
        .await
        .map_err(CommandError::ServiceError)?;

    if templates.is_empty() {
        println!("No templates available");
        return Ok(());
    }

    println!("Available Architecture Templates:\n");
    for template in templates {
        if let Some(pattern_filter) = &args.pattern {
            let filter_pattern = convert_pattern_arg(pattern_filter.clone());
            if template.pattern != filter_pattern {
                continue;
            }
        }

        println!("  {}", template.name);
        println!("    Pattern: {:?}", template.pattern);
        println!("    Description: {}", template.description);

        if args.verbose {
            println!("    Use cases:");
            for use_case in &template.use_cases {
                println!("      - {}", use_case);
            }
        }
        println!();
    }

    Ok(())
}

/// Execute refine subcommand
async fn execute_refine(args: RefineArgs) -> Result<(), CommandError> {
    // Load existing document
    let content = tokio::fs::read_to_string(&args.input)
        .await
        .map_err(|e| CommandError::IoError(e.to_string()))?;

    // Parse document (implement parser)
    let document = parse_architecture_document(&content)?;

    // Initialize service
    let service = create_architecture_service(&args.ollama_url, &args.model)?;

    // Create backup if requested
    if args.backup {
        let backup_path = args.input.with_extension("md.bak");
        tokio::fs::copy(&args.input, &backup_path)
            .await
            .map_err(|e| CommandError::IoError(e.to_string()))?;
        println!("Backup created: {}", backup_path.display());
    }

    // Refine architecture
    let refined = service
        .refine_architecture(&document, &args.refinement)
        .await
        .map_err(CommandError::ServiceError)?;

    // Save refined document
    let output_path = args.output.unwrap_or(args.input);
    service
        .save_architecture(&refined, &output_path)
        .await
        .map_err(CommandError::ServiceError)?;

    println!("Architecture refined successfully: {}", output_path.display());

    Ok(())
}

/// Execute validate subcommand
async fn execute_validate(args: ValidateArgs) -> Result<(), CommandError> {
    let content = tokio::fs::read_to_string(&args.input)
        .await
        .map_err(|e| CommandError::IoError(e.to_string()))?;

    let document = parse_architecture_document(&content)?;

    match ArchitectureValidator::validate(&document) {
        Ok(_) => {
            println!("Architecture document is valid");
            if args.verbose {
                println!("\nValidation Details:");
                println!("  Title: {}", document.metadata.title);
                println!("  Layers: {}", document.layers.len());
                println!("  Components: {}", document.components.len());
                println!("  Integrations: {}", document.integrations.len());
            }
        }
        Err(e) => {
            eprintln!("Validation failed: {}", e);
            return Err(CommandError::ValidationError(e));
        }
    }

    Ok(())
}

/// Helper: Create architecture service
fn create_architecture_service(
    ollama_url: &str,
    model: &str,
) -> Result<ArchitectureService, CommandError> {
    let generator = Arc::new(OllamaArchitectureGenerator::new(
        ollama_url.to_string(),
        model.to_string(),
    ));
    let writer = Arc::new(MarkdownArchitectureWriter::new());
    let template_repo = Arc::new(FileTemplateRepository::new(
        PathBuf::from("templates/architecture"),
    ));

    Ok(ArchitectureService::new(generator, writer, template_repo))
}

/// Helper: Check Ollama health
async fn check_ollama_health(base_url: &str) -> Result<(), CommandError> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/tags", base_url))
        .send()
        .await
        .map_err(|_| CommandError::OllamaUnavailable)?;

    if !response.status().is_success() {
        return Err(CommandError::OllamaUnavailable);
    }

    Ok(())
}

/// Helper: Convert pattern argument
fn convert_pattern_arg(arg: PatternArg) -> ArchitecturePattern {
    match arg {
        PatternArg::Microservices => ArchitecturePattern::Microservices,
        PatternArg::Monolithic => ArchitecturePattern::Monolithic,
        PatternArg::EventDriven => ArchitecturePattern::EventDriven,
        PatternArg::Layered => ArchitecturePattern::Layered,
        PatternArg::Hexagonal => ArchitecturePattern::Hexagonal,
        PatternArg::Cqrs => ArchitecturePattern::CQRS,
        PatternArg::Serverless => ArchitecturePattern::Serverless,
    }
}

/// Helper: Convert complexity argument
fn convert_complexity_arg(arg: ComplexityArg) -> ComplexityLevel {
    match arg {
        ComplexityArg::Simple => ComplexityLevel::Simple,
        ComplexityArg::Moderate => ComplexityLevel::Moderate,
        ComplexityArg::Complex => ComplexityLevel::Complex,
        ComplexityArg::Enterprise => ComplexityLevel::Enterprise,
    }
}

/// Helper: Parse technology stack
fn parse_tech_stack(tech_stack: Option<String>) -> Vec<String> {
    tech_stack
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
        .unwrap_or_default()
}

/// Helper: Parse architecture document from markdown
fn parse_architecture_document(content: &str) -> Result<ArchitectureDocument, CommandError> {
    // Implementation would parse markdown back to ArchitectureDocument
    // For now, placeholder
    todo!("Implement markdown parsing")
}

/// Command errors
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Output file already exists: {0}")]
    OutputFileExists(PathBuf),

    #[error("Missing requirements or template")]
    MissingRequirements,

    #[error("Ollama service unavailable")]
    OllamaUnavailable,

    #[error("Service error: {0}")]
    ServiceError(ServiceError),

    #[error("Session error: {0}")]
    SessionError(SessionError),

    #[error("Validation error: {0}")]
    ValidationError(ValidationError),

    #[error("I/O error: {0}")]
    IoError(String),
}
```

**Tests Required**:

- Command argument parsing
- Each subcommand execution
- Error handling
- Interactive flow

**Estimated Lines**: ~450 lines

### Phase 4.2: Update Main Commands Enum

**File**: `src/commands/mod.rs`

```rust
pub mod architecture;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    // ... existing commands ...

    /// Generate and manage software architecture documents with LLM
    #[command(name = "architecture")]
    Architecture(architecture::ArchitectureCommand),
}
```

**Estimated Lines**: ~10 lines

**Phase 4 Total**: ~460 lines of CLI code + ~200 lines of tests

## Phase 5: Configuration and Templates

### Phase 5.1: Configuration Support

**File**: `config/default.yaml`

```yaml
# LLM Architecture Configuration
architecture:
  # Ollama configuration
  ollama:
    base_url: "http://localhost:11434"
    default_model: "llama2"
    timeout_seconds: 300
    max_retries: 3

  # Generation defaults
  generation:
    default_pattern: "layered"
    default_complexity: "moderate"
    include_deployment: true
    include_quality_attributes: true
    max_components: 20

  # Output configuration
  output:
    default_directory: "docs/architecture"
    format: "markdown"
    create_backup: true

  # Template configuration
  templates:
    directory: "templates/architecture"
    auto_create_defaults: true

  # Interactive mode settings
  interactive:
    enable_colors: true
    show_progress: true
    confirm_before_save: true
    show_token_usage: false
```

**Estimated Lines**: ~30 lines

### Phase 5.2: Default Architecture Templates

**File**: `templates/architecture/microservices.yaml`

```yaml
info:
  name: "microservices"
  pattern: "Microservices"
  description: "Distributed microservices architecture with API gateway"
  use_cases:
    - "Large-scale distributed systems"
    - "Independent service deployment"
    - "Polyglot technology stacks"
    - "Team autonomy and scalability"

structure:
  layers:
    - "API Gateway"
    - "Service Layer"
    - "Data Layer"
    - "Infrastructure Layer"

  integration_patterns:
    - "REST APIs"
    - "Event-driven messaging"
    - "Service mesh"

  required_components:
    - "API Gateway"
    - "Service Discovery"
    - "Configuration Service"
    - "Monitoring and Logging"

default_components:
  - name: "API Gateway"
    layer: "API Gateway"
    role: "Single entry point, routing, authentication"
    typical_technologies:
      - "Kong"
      - "Nginx"
      - "AWS API Gateway"

  - name: "Service Registry"
    layer: "Infrastructure Layer"
    role: "Service discovery and health checking"
    typical_technologies:
      - "Consul"
      - "Eureka"
      - "etcd"

  - name: "Message Broker"
    layer: "Infrastructure Layer"
    role: "Asynchronous communication between services"
    typical_technologies:
      - "Kafka"
      - "RabbitMQ"
      - "Redis Streams"

customization_points:
  - "Number and type of microservices"
  - "Communication patterns (sync vs async)"
  - "Data management strategy (per-service databases)"
  - "Deployment platform (Kubernetes, Docker Swarm, etc.)"
```

**Additional Templates**:

- `templates/architecture/monolithic.yaml`
- `templates/architecture/event_driven.yaml`
- `templates/architecture/layered.yaml`

**Estimated Lines**: ~200 lines total

### Phase 5.3: User Documentation

**File**: `docs/how_to/generate_architecture_with_llm.md`

````markdown
# How to Generate Architecture with LLM

This guide explains how to use the architecture command to generate software architecture documents using LLM assistance.

## Prerequisites

- Ollama installed and running
- At least one LLM model downloaded (recommended: llama2, mistral, or codellama)

## Basic Usage

### Generate from Requirements

```bash
xzagentz architecture generate \
  --requirements "E-commerce platform with user auth, product catalog, and payment" \
  --pattern microservices \
  --output docs/architecture/ecommerce_architecture.md
```
````

### Generate from Template

```bash
xzagentz architecture generate \
  --template microservices \
  --customization "Add recommendation engine and analytics service" \
  --output docs/architecture/custom_architecture.md
```

### Interactive Mode

```bash
xzagentz architecture generate --interactive
```

Interactive mode guides you through:

1. Choosing generation mode (requirements or template)
2. Providing system requirements or selecting template
3. Configuring options (pattern, complexity, etc.)
4. Reviewing and refining the generated architecture
5. Saving to file

## Advanced Options

### Specify Architecture Pattern

```bash
xzagentz architecture generate \
  --requirements "IoT platform" \
  --pattern event-driven \
  --complexity enterprise \
  --output docs/architecture/iot_platform.md
```

### Technology Preferences

```bash
xzagentz architecture generate \
  --requirements "Real-time analytics platform" \
  --tech-stack "Rust,PostgreSQL,Redis,Kafka" \
  --output docs/architecture/analytics.md
```

### Custom Ollama Configuration

```bash
xzagentz architecture generate \
  --requirements "Mobile backend API" \
  --ollama-url "http://custom-host:11434" \
  --model "codellama" \
  --output docs/architecture/mobile_api.md
```

## Refining Architecture

Refine an existing architecture document:

```bash
xzagentz architecture refine \
  --input docs/architecture/ecommerce_architecture.md \
  --refinement "Add caching layer and improve security considerations" \
  --backup
```

## Listing Templates

View available architecture templates:

```bash
xzagentz architecture list-templates

# Detailed view
xzagentz architecture list-templates --verbose

# Filter by pattern
xzagentz architecture list-templates --pattern microservices
```

## Validating Architecture

Validate an architecture document:

```bash
xzagentz architecture validate \
  --input docs/architecture/myarch.md \
  --verbose
```

## Configuration

Default settings can be configured in `config/default.yaml`:

```yaml
architecture:
  ollama:
    base_url: "http://localhost:11434"
    default_model: "llama2"
  generation:
    default_pattern: "layered"
    default_complexity: "moderate"
```

## Troubleshooting

### Ollama Not Available

```text
Error: Ollama service unavailable
```

Solution: Ensure Ollama is running:

```bash
ollama serve
```

### Model Not Found

```text
Error: Model 'llama2' not found
```

Solution: Pull the model:

```bash
ollama pull llama2
```

### Generation Timeout

```text
Error: Request timeout
```

Solution: Increase timeout in configuration or use smaller model:

```bash
xzagentz architecture generate \
  --model "llama2:7b" \
  --requirements "Simple web app"
```

## Best Practices

1. Start with interactive mode to explore options
2. Use templates for common patterns
3. Provide detailed requirements for better results
4. Review and refine generated architectures
5. Validate architecture documents before committing
6. Use technology preferences to guide LLM choices

## Examples

### Example 1: Microservices E-commerce

```bash
xzagentz architecture generate \
  --requirements "E-commerce platform with user management, product catalog, shopping cart, order processing, payment integration, and inventory management" \
  --pattern microservices \
  --complexity complex \
  --include-deployment \
  --tech-stack "Rust,PostgreSQL,Redis,Kafka" \
  --output docs/architecture/ecommerce.md
```

### Example 2: Event-Driven IoT Platform

```bash
xzagentz architecture generate \
  --template event-driven \
  --customization "IoT device management with real-time telemetry processing, device provisioning, firmware updates, and analytics dashboard" \
  --complexity enterprise \
  --output docs/architecture/iot_platform.md
```

### Example 3: Monolithic CMS

```bash
xzagentz architecture generate \
  --requirements "Content management system with WYSIWYG editor, media library, user roles, and publishing workflow" \
  --pattern monolithic \
  --complexity moderate \
  --output docs/architecture/cms.md
```



**Estimated Lines**: ~250 lines

**Phase 5 Total**: ~480 lines of configuration and documentation

## Phase 6: Testing and Quality Assurance

### Phase 6.1: Unit Test Coverage

**Target Coverage**: Greater than 80% for all modules

**Test Files**:

1. `tests/domain/architecture/models_test.rs` - Domain model tests
2. `tests/domain/architecture/validation_test.rs` - Validation logic tests
3. `tests/infrastructure/llm/ollama_generator_test.rs` - LLM integration tests (with mocks)
4. `tests/infrastructure/templates/repository_test.rs` - Template repository tests
5. `tests/infrastructure/io/markdown_writer_test.rs` - Writer tests
6. `tests/application/architecture_service_test.rs` - Service orchestration tests
7. `tests/commands/architecture_test.rs` - CLI command tests

**Example Test Structure**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_document_creation() {
        let metadata = ArchitectureMetadata {
            title: "Test Architecture".to_string(),
            version: "1.0.0".to_string(),
            generated_at: chrono::Utc::now(),
            model_used: "llama2".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec!["test".to_string()],
        };

        let overview = Overview {
            description: "Test description".to_string(),
            business_goals: vec!["Goal 1".to_string()],
            constraints: vec![],
            assumptions: vec![],
        };

        let document = ArchitectureDocument {
            metadata,
            overview,
            layers: vec![],
            components: vec![],
            integrations: vec![],
            deployment: None,
            quality_attributes: vec![],
        };

        assert_eq!(document.metadata.title, "Test Architecture");
    }

    #[tokio::test]
    async fn test_ollama_generator_with_mock() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "response": r#"{"title": "Test"}"#,
                "model": "llama2",
                "done": true
            })))
            .mount(&mock_server)
            .await;

        let generator = OllamaArchitectureGenerator::new(
            mock_server.uri(),
            "llama2".to_string(),
        );

        let options = GenerationOptions::default();
        let result = generator.generate("test requirements", options).await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_empty_title_fails() {
        let mut document = create_valid_document();
        document.metadata.title = String::new();

        let result = ArchitectureValidator::validate(&document);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::EmptyTitle));
    }
}
````

**Estimated Lines**: ~800 lines of tests

### Phase 6.2: Integration Tests

**File**: `tests/integration/architecture_generation_test.rs`

```rust
#[tokio::test]
async fn test_end_to_end_architecture_generation() {
    // Setup: Mock Ollama server
    let mock_server = setup_mock_ollama().await;

    // Execute: Generate architecture
    let args = GenerateArgs {
        requirements: Some("Test system".to_string()),
        template: None,
        pattern: Some(PatternArg::Layered),
        complexity: ComplexityArg::Simple,
        output: temp_output_path(),
        model: "llama2".to_string(),
        ollama_url: mock_server.uri(),
        interactive: false,
        skip_health_check: true,
        force: true,
        // ... other defaults
    };

    let result = execute_generate(args).await;

    // Assert: Success and file exists
    assert!(result.is_ok());
    assert!(temp_output_path().exists());

    // Verify: Content is valid
    let content = std::fs::read_to_string(temp_output_path()).unwrap();
    assert!(content.contains("# "));
    assert!(content.contains("## Overview"));
}

#[tokio::test]
async fn test_template_based_generation() {
    let mock_server = setup_mock_ollama().await;

    // Setup templates
    setup_test_templates().await;

    let args = GenerateArgs {
        requirements: None,
        template: Some("microservices".to_string()),
        customization: Some("E-commerce".to_string()),
        output: temp_output_path(),
        ollama_url: mock_server.uri(),
        skip_health_check: true,
        // ... other defaults
    };

    let result = execute_generate(args).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_refine_architecture_workflow() {
    // Create initial architecture
    let initial_path = create_test_architecture().await;

    // Setup mock
    let mock_server = setup_mock_ollama().await;

    let args = RefineArgs {
        input: initial_path.clone(),
        refinement: "Add security layer".to_string(),
        output: None,
        model: "llama2".to_string(),
        ollama_url: mock_server.uri(),
        backup: true,
    };

    let result = execute_refine(args).await;
    assert!(result.is_ok());

    // Verify backup was created
    assert!(initial_path.with_extension("md.bak").exists());
}
```

**Estimated Lines**: ~300 lines

### Phase 6.3: Quality Gates

All checks must pass before merging:

```bash
# Format
cargo fmt --all

# Compile check
cargo check --all-targets --all-features

# Lint (zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Tests (greater than 80% coverage)
cargo test --all-features

# Documentation generation
cargo doc --no-deps
```

**Phase 6 Total**: ~1100 lines of test code

## Phase 7: Implementation Summary Documentation

### Phase 7.1: Create Implementation Summary

**File**: `docs/explanation/llm_architecture_implementation.md`

````markdown
# LLM Architecture Command Implementation

## Overview

Implementation of LLM-powered architecture document generation for xzagentz, enabling users to create professional software architecture documentation using Ollama-hosted language models.

## Components Delivered

### Domain Layer

- `src/domain/architecture/models.rs` (300 lines) - Core architecture models
- `src/domain/architecture/traits.rs` (200 lines) - Domain traits
- `src/domain/architecture/validation.rs` (150 lines) - Validation logic

### Infrastructure Layer

- `src/infrastructure/llm/ollama_architecture_generator.rs` (400 lines) - Ollama integration
- `src/infrastructure/templates/file_template_repository.rs` (300 lines) - Template management
- `src/infrastructure/io/markdown_architecture_writer.rs` (250 lines) - Markdown output

### Application Layer

- `src/application/architecture_service.rs` (200 lines) - Service orchestration
- `src/application/interactive_architecture_session.rs` (400 lines) - Interactive CLI

### CLI Layer

- `src/commands/architecture.rs` (450 lines) - Command implementation

### Configuration and Templates

- `config/default.yaml` (30 lines) - Default configuration
- `templates/architecture/*.yaml` (200 lines) - Architecture templates

### Documentation

- `docs/how_to/generate_architecture_with_llm.md` (250 lines) - User guide
- `docs/explanation/llm_architecture_implementation.md` (this file)

### Tests

- Unit tests (800 lines) - Domain, infrastructure, application tests
- Integration tests (300 lines) - End-to-end workflows

**Total**: Approximately 3,230 lines of production code + 1,100 lines of tests

## Implementation Details

### Architecture Pattern

The implementation follows xzagentz's layered architecture:

1. **Domain Layer**: Pure business logic with no infrastructure dependencies
2. **Application Layer**: Orchestrates domain and infrastructure
3. **Infrastructure Layer**: External integrations (Ollama, file I/O, templates)
4. **CLI Layer**: User interface and command handling

### Key Design Decisions

1. **Trait-Based Abstraction**: Domain traits allow swapping LLM providers
2. **Template System**: Reusable architecture patterns as YAML templates
3. **Interactive Mode**: Dialoguer-based CLI for guided experience
4. **Incremental Refinement**: Support for iterative architecture improvement
5. **Validation Layer**: Ensures generated architectures meet quality standards

### LLM Integration

Ollama integration uses:

- HTTP API for model communication
- JSON-structured prompts for consistent output
- Error handling with retries
- Response parsing with fallbacks

### Template System

Templates provide:

- Predefined architecture patterns
- Customization points
- Default component suggestions
- Integration pattern guidance

## Testing

### Coverage

- Domain layer: 95% coverage
- Infrastructure layer: 85% coverage
- Application layer: 90% coverage
- CLI layer: 80% coverage
- Overall: Greater than 85% coverage

### Test Strategy

1. **Unit Tests**: Test each component in isolation with mocks
2. **Integration Tests**: Test end-to-end workflows with mock Ollama
3. **Validation Tests**: Ensure architecture documents meet standards

## Usage Examples

### Basic Generation

```bash
xzagentz architecture generate \
  --requirements "Microservices e-commerce platform" \
  --pattern microservices \
  --output docs/architecture/ecommerce.md
```
````

### Template-Based

```bash
xzagentz architecture generate \
  --template event-driven \
  --customization "IoT platform with real-time processing" \
  --output docs/architecture/iot.md
```

### Interactive Mode

```bash
xzagentz architecture generate --interactive
```

## Validation Results

- All cargo fmt checks pass
- All cargo clippy checks pass with zero warnings
- All tests pass with greater than 85% coverage
- Documentation complete and accurate
- Follows all AGENTS.md rules

## References

- Implementation Plan: `docs/explanation/llm_architecture_command_plan.md`
- User Guide: `docs/how_to/generate_architecture_with_llm.md`
- Domain Models: `src/domain/architecture/models.rs`
- Ollama Documentation: https://ollama.ai/docs

````

**Estimated Lines**: ~200 lines

**Phase 7 Total**: ~200 lines of implementation documentation

## Dependencies Summary

### New Dependencies Required

Add to `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

# HTTP client for Ollama
reqwest = { version = "0.11", features = ["json"] }

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Interactive CLI
dialoguer = "0.11"
console = "0.15"
indicatif = "0.17"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
# Existing dev dependencies...

# Mock HTTP server for testing
wiremock = "0.5"
mockall = "0.12"
tempfile = "3.8"
````

## Risk Assessment and Mitigation

### Risk 1: Ollama Service Unavailable

**Impact**: High - Users cannot generate architectures

**Mitigation**:

- Health check before generation
- Clear error messages with setup instructions
- `--skip-health-check` flag for CI/CD
- Fallback to template-only mode

### Risk 2: LLM Response Quality

**Impact**: Medium - Generated architectures may be incomplete or incorrect

**Mitigation**:

- Structured prompts with explicit JSON schema
- Response validation before saving
- Interactive refinement loop
- Template-based generation as alternative
- Clear documentation on reviewing output

### Risk 3: Response Parsing Failures

**Impact**: Medium - JSON parsing may fail on malformed responses

**Mitigation**:

- Multiple parsing strategies (with/without markdown code blocks)
- Graceful error messages
- Retry logic with adjusted prompts
- Validation at multiple stages

### Risk 4: Large Architecture Generation Timeout

**Impact**: Low - Complex architectures may exceed timeout

**Mitigation**:

- Configurable timeout values
- Progress indicators
- Complexity level controls max component count
- Streaming support (future enhancement)

### Risk 5: Template Management Complexity

**Impact**: Low - Users may need custom templates

**Mitigation**:

- Default templates cover common patterns
- Template validation on load
- Clear template schema documentation
- Template creation guide in docs

## Success Criteria

### Functional Requirements

1. Users can generate architecture from natural language requirements
2. Users can generate architecture from predefined templates
3. Users can refine existing architectures
4. Users can validate architecture documents
5. Interactive mode guides users through generation
6. Output follows project documentation standards

### Quality Requirements

1. All code passes `cargo fmt` without changes
2. All code passes `cargo clippy` with zero warnings
3. Test coverage exceeds 80% overall, 85% for critical paths
4. All tests pass consistently
5. Documentation is complete and accurate

### User Experience Requirements

1. Clear error messages with actionable solutions
2. Progress indicators for long-running operations
3. Interactive mode is intuitive and helpful
4. Generated architectures are professional and complete
5. Command-line interface is consistent with existing commands

### Performance Requirements

1. Simple architectures generate in under 30 seconds
2. Complex architectures generate in under 2 minutes
3. Template loading is under 1 second
4. File I/O operations are efficient

## Timeline Estimate

**Total Estimated Time**: 4-5 weeks (160-200 hours)

### Phase 1: Domain Models and Traits (1 week)

- Models: 2 days
- Traits: 2 days
- Validation: 1 day

### Phase 2: Infrastructure - Ollama Integration (1.5 weeks)

- Ollama generator: 3 days
- Template repository: 2 days
- Markdown writer: 2 days

### Phase 3: Application Layer (1 week)

- Architecture service: 2 days
- Interactive session: 3 days

### Phase 4: CLI Integration (0.5 weeks)

- Command structure: 1 day
- Subcommand implementations: 1.5 days

### Phase 5: Configuration and Templates (0.5 weeks)

- Configuration: 0.5 days
- Templates: 1 day
- Documentation: 1 day

### Phase 6: Testing and QA (0.5 weeks)

- Unit tests: 1.5 days
- Integration tests: 1 day
- Quality gates: 0.5 days

### Phase 7: Documentation (2 days)

- Implementation summary: 1 day
- Final review and polish: 1 day

**Note**: Timeline assumes one developer working full-time. Adjust for part-time work or multiple developers.

## Appendix A: Example Prompt Template

The following prompt template is used for architecture generation:

```text
You are an expert software architect. Generate a comprehensive, professional software architecture document in JSON format.

Your response must be valid JSON matching this structure:
{
  "title": "Architecture Title",
  "overview": {
    "description": "High-level description of the system",
    "business_goals": ["goal1", "goal2"],
    "constraints": ["constraint1", "constraint2"],
    "assumptions": ["assumption1"]
  },
  "layers": [
    {
      "name": "Layer Name",
      "description": "Layer description",
      "responsibilities": ["resp1", "resp2"],
      "components": ["component1", "component2"],
      "dependencies": ["lower_layer"]
    }
  ],
  "components": [
    {
      "id": "component-id",
      "name": "Component Name",
      "description": "Component description",
      "layer": "layer-name",
      "responsibilities": ["resp1", "resp2"],
      "interfaces": [
        {
          "name": "API",
          "protocol": "HTTP/REST",
          "endpoints": ["/api/v1/resource"],
          "data_formats": ["JSON"]
        }
      ],
      "dependencies": ["other-component"],
      "technology_stack": ["Technology1", "Technology2"]
    }
  ],
  "integrations": [
    {
      "from_component": "component1",
      "to_component": "component2",
      "integration_type": "Synchronous",
      "description": "Integration description",
      "protocols": ["HTTP", "gRPC"]
    }
  ],
  "quality_attributes": [
    {
      "name": "Performance",
      "description": "Performance requirements and approach",
      "tactics": ["Caching", "Load balancing"],
      "metrics": ["Response time < 200ms", "Throughput > 1000 req/s"]
    }
  ]
}

Pattern preference: Microservices
Complexity level: Moderate
Include deployment: true
Max components: 20
Technology preferences: Rust, PostgreSQL, Redis

User Requirements:
[User's requirements text here]
```

## Appendix B: File Structure

Complete file structure after implementation:

```text
xzagentz/
├── src/
│   ├── domain/
│   │   └── architecture/
│   │       ├── mod.rs
│   │       ├── models.rs
│   │       ├── traits.rs
│   │       └── validation.rs
│   ├── infrastructure/
│   │   ├── llm/
│   │   │   ├── mod.rs
│   │   │   └── ollama_architecture_generator.rs
│   │   ├── templates/
│   │   │   ├── mod.rs
│   │   │   └── file_template_repository.rs
│   │   └── io/
│   │       ├── mod.rs
│   │       └── markdown_architecture_writer.rs
│   ├── application/
│   │   ├── architecture_service.rs
│   │   └── interactive_architecture_session.rs
│   └── commands/
│       ├── mod.rs
│       └── architecture.rs
├── templates/
│   └── architecture/
│       ├── microservices.yaml
│       ├── monolithic.yaml
│       ├── event_driven.yaml
│       └── layered.yaml
├── config/
│   └── default.yaml (updated)
├── docs/
│   ├── explanation/
│   │   ├── llm_architecture_command_plan.md
│   │   └── llm_architecture_implementation.md
│   └── how_to/
│       └── generate_architecture_with_llm.md
└── tests/
    ├── domain/
    │   └── architecture/
    │       ├── models_test.rs
    │       └── validation_test.rs
    ├── infrastructure/
    │   ├── llm/
    │   │   └── ollama_generator_test.rs
    │   ├── templates/
    │   │   └── repository_test.rs
    │   └── io/
    │       └── markdown_writer_test.rs
    ├── application/
    │   ├── architecture_service_test.rs
    │   └── interactive_session_test.rs
    ├── commands/
    │   └── architecture_test.rs
    └── integration/
        └── architecture_generation_test.rs
```

## Appendix C: Validation Checklist

Use this checklist before claiming implementation complete:

### Code Quality

- [ ] `cargo fmt --all` passes with no changes
- [ ] `cargo check --all-targets --all-features` passes with zero errors
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] All doc comments present with examples
- [ ] No `unwrap()` without justification

### Architecture Compliance

- [ ] Domain layer has no infrastructure dependencies
- [ ] Traits defined in domain, implemented in infrastructure
- [ ] Application layer orchestrates correctly
- [ ] Layer boundaries respected

### Documentation

- [ ] Implementation summary created in `docs/explanation/`
- [ ] User guide created in `docs/how_to/`
- [ ] Filename uses lowercase with underscores
- [ ] No emojis anywhere
- [ ] All code blocks specify language

### Testing

- [ ] Unit tests for all public functions
- [ ] Integration tests for workflows
- [ ] Test coverage exceeds 80%
- [ ] All tests pass consistently
- [ ] Mock Ollama for tests

### Files and Conventions

- [ ] All YAML files use `.yaml` extension
- [ ] All Markdown files use `.md` extension
- [ ] No uppercase in filenames except `README.md`
- [ ] Branch name: `pr-llmarch-ISSUE`
- [ ] Commit message follows conventional commits

## References

- xzagentz Architecture: `docs/explanation/architecture.md`
- Implementation Command Plan: `docs/explanation/implementation_command_plan.md`
- AGENTS.md Rules: `AGENTS.md`
- Ollama API Documentation: https://github.com/ollama/ollama/blob/main/docs/api.md
- Dialoguer Documentation: https://docs.rs/dialoguer/
- Diataxis Framework: https://diataxis.fr/
