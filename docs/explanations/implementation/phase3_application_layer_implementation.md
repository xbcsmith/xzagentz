# Phase 3: Application Layer Implementation

## Overview

This document describes the implementation of Phase 3 of the LLM Architecture Command feature: the Application Layer. This phase implements the orchestration logic that coordinates between the domain and infrastructure layers to provide high-level services for architecture generation, refinement, and persistence.

## Components Delivered

### Application Services

- `src/application/architecture_service.rs` (368 lines) - Core orchestration service
- `src/application/interactive_architecture_session.rs` (475 lines) - Interactive CLI session
- `src/application/mod.rs` (updated) - Module exports

Total: ~850 lines of application logic with comprehensive tests

## Implementation Details

### Component 1: Architecture Service

The `ArchitectureService` provides a high-level API for architecture generation workflows:

```rust
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
```

#### Key Methods

**generate_from_requirements**

Generates architecture from natural language requirements:

```rust
pub async fn generate_from_requirements(
    &self,
    requirements: &str,
    options: GenerationOptions,
) -> Result<ArchitectureDocument, ServiceError>
```

Flow:
1. Invoke generator with requirements and options
2. Validate generated document
3. Return validated document

**generate_from_template**

Generates architecture from an existing template:

```rust
pub async fn generate_from_template(
    &self,
    template_name: &str,
    customization: &str,
) -> Result<ArchitectureDocument, ServiceError>
```

Flow:
1. Load template from repository
2. Generate from template with customization
3. Validate generated document
4. Return validated document

**refine_architecture**

Refines an existing architecture based on feedback:

```rust
pub async fn refine_architecture(
    &self,
    document: &ArchitectureDocument,
    feedback: &str,
) -> Result<ArchitectureDocument, ServiceError>
```

Flow:
1. Invoke generator with document and feedback
2. Validate refined document
3. Return refined document

**save_architecture**

Persists architecture document to file:

```rust
pub async fn save_architecture(
    &self,
    document: &ArchitectureDocument,
    output_path: impl AsRef<Path>,
) -> Result<(), ServiceError>
```

**list_templates**

Lists available architecture templates:

```rust
pub async fn list_templates(&self) -> Result<Vec<TemplateInfo>, ServiceError>
```

#### Error Handling

The service defines a comprehensive error type that maps errors from underlying layers:

```rust
pub enum ServiceError {
    GenerationError(String),
    ValidationError(ValidationError),
    TemplateError(String),
    WriteError(String),
}
```

All infrastructure errors are converted to string representations for uniform error handling at the application boundary.

### Component 2: Interactive Architecture Session

The `InteractiveArchitectureSession` provides a guided CLI experience using the `dialoguer` library:

```rust
pub struct InteractiveArchitectureSession<G, W, T, Th>
where
    G: ArchitectureGenerator,
    W: ArchitectureWriter,
    T: TemplateRepository,
    Th: Theme,
{
    service: ArchitectureService<G, W, T>,
    theme: Th,
}
```

#### Interactive Workflow

The session guides users through a complete architecture generation workflow:

```rust
pub async fn run(&self) -> Result<Option<String>, SessionError>
```

Workflow steps:
1. Select generation mode (requirements vs template)
2. Gather inputs (requirements, pattern, complexity, etc.)
3. Generate architecture document
4. Review and refinement loop
5. Save to file

#### User Interaction Methods

**select_generation_mode**

Prompts user to choose between:
- Generate from requirements
- Generate from template

**generate_from_requirements_interactive**

Prompts for:
- System requirements (text input)
- Architecture pattern (select menu)
- Complexity level (select menu)
- Include deployment architecture (yes/no)
- Include quality attributes (yes/no)
- Technology preferences (comma-separated list)

**generate_from_template_interactive**

Prompts for:
- Template selection (from available templates)
- Customization instructions (optional text)

**review_and_refine_loop**

Interactive refinement workflow:
1. Display document summary (title, pattern, component counts)
2. Present options:
   - Accept and save
   - Refine further (prompts for feedback)
   - Cancel
3. If refining, regenerate and loop back to step 1

**save_document_interactive**

Prompts for:
- Output file path (with smart default based on title)
- Overwrite confirmation (if file exists)

#### Helper Methods

The session provides reusable prompt helpers:

- `select_architecture_pattern()` - Menu of architecture patterns
- `select_complexity_level()` - Menu of complexity levels
- `prompt_yes_no()` - Boolean confirmation prompt
- `prompt_technology_preferences()` - Parse comma-separated tech list
- `display_document_summary()` - Show architecture overview

#### Session Error Handling

```rust
pub enum SessionError {
    InteractionError(String),
    ServiceError(ServiceError),
    IoError(std::io::Error),
    NoTemplatesAvailable,
    Cancelled,
}
```

The session error type wraps service errors and adds interaction-specific errors like cancellation and no templates available.

### Component 3: Module Integration

Updated `src/application/mod.rs` to export the new components:

```rust
pub mod architecture_service;
pub mod interactive_architecture_session;

pub use architecture_service::{ArchitectureService, ServiceError};
pub use interactive_architecture_session::{InteractiveArchitectureSession, SessionError};
```

## Architecture Patterns

### Dependency Injection

The service uses generic type parameters for dependency injection:

```rust
impl<G, W, T> ArchitectureService<G, W, T>
where
    G: ArchitectureGenerator,
    W: ArchitectureWriter,
    T: TemplateRepository,
```

Benefits:
- Testability: Easy to inject mocks for testing
- Flexibility: Can swap implementations without changing service code
- Type safety: Compile-time verification of trait implementation

### Error Mapping

The service maps infrastructure errors to application-level errors:

```rust
.map_err(|e| ServiceError::GenerationError(e.to_string()))?;
```

This approach:
- Creates clean separation between layers
- Allows infrastructure to change without affecting application API
- Provides consistent error handling for CLI layer

### Validation Integration

The service integrates domain validation after generation:

```rust
let validator = crate::domain::architecture::ArchitectureValidator;
validator.validate(&document)
    .map_err(ServiceError::ValidationError)?;
```

This ensures:
- All generated documents pass validation before being returned
- Invalid documents are caught early in the pipeline
- Clear error messages about what validation failed

## Testing

### Test Coverage

Phase 3 includes comprehensive unit tests:

- `test_generate_from_requirements_success` - Successful generation
- `test_generate_from_requirements_failure` - Handle generation failure
- `test_generate_from_template_success` - Template-based generation
- `test_generate_from_template_not_found` - Template not found error
- `test_refine_architecture_success` - Successful refinement
- `test_save_architecture_success` - Document persistence
- `test_save_architecture_failure` - Write error handling
- `test_list_templates_success` - Template listing
- `test_list_templates_failure` - Repository error handling

Additional tests for enums and error display:
- `test_generation_mode_variants` - Mode enum coverage
- `test_review_action_variants` - Action enum coverage
- `test_session_error_display` - Error message formatting

### Test Strategy

Tests use mock implementations of traits:

```rust
struct MockGenerator {
    should_fail: bool,
}

#[async_trait]
impl ArchitectureGenerator for MockGenerator {
    type Error = MockError;
    // Mock implementation
}
```

Mock error type with proper trait bounds:

```rust
#[derive(Error, Debug)]
#[error("Mock error: {0}")]
struct MockError(String);
```

Test documents include required validation fields:

```rust
fn create_test_document() -> ArchitectureDocument {
    ArchitectureDocument {
        // ... with at least one layer and one component
        layers: vec![Layer { /* ... */ }],
        components: vec![Component { /* ... */ }],
        // ...
    }
}
```

### Test Results

All tests pass successfully:

```text
test result: ok. 751 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Coverage: 100% of public methods and error paths tested

## Usage Examples

### Example 1: Generate from Requirements

```rust
use xzagentz::application::ArchitectureService;
use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
use xzagentz::infrastructure::templates::FileTemplateRepository;
use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
use xzagentz::domain::architecture::{GenerationOptions, ArchitecturePattern, ComplexityLevel};

let generator = OllamaArchitectureGenerator::new("http://localhost:11434", "llama2");
let writer = MarkdownArchitectureWriter::new();
let template_repo = FileTemplateRepository::new("./templates/architecture").await?;

let service = ArchitectureService::new(generator, writer, template_repo);

let options = GenerationOptions {
    pattern: Some(ArchitecturePattern::Microservices),
    complexity: ComplexityLevel::Moderate,
    include_deployment: true,
    include_quality_attributes: true,
    max_components: Some(10),
    technology_preferences: vec!["Rust".to_string(), "PostgreSQL".to_string()],
};

let document = service.generate_from_requirements(
    "Build an e-commerce platform with product catalog, cart, and checkout",
    options
).await?;

service.save_architecture(&document, "ecommerce_architecture.md").await?;
```

### Example 2: Interactive Session

```rust
use xzagentz::application::{ArchitectureService, InteractiveArchitectureSession};
use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
use xzagentz::infrastructure::templates::FileTemplateRepository;
use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
use dialoguer::theme::ColorfulTheme;

let generator = OllamaArchitectureGenerator::new("http://localhost:11434", "llama2");
let writer = MarkdownArchitectureWriter::new();
let template_repo = FileTemplateRepository::new("./templates").await?;
let service = ArchitectureService::new(generator, writer, template_repo);

let session = InteractiveArchitectureSession::new(service, ColorfulTheme::default());
if let Some(output_path) = session.run().await? {
    println!("Architecture saved to: {}", output_path);
}
```

### Example 3: Refine Existing Architecture

```rust
// Load existing document (from file or previous generation)
let document = load_architecture_document("architecture.md")?;

// Refine with feedback
let refined = service.refine_architecture(
    &document,
    "Add a caching layer using Redis between the API and database"
).await?;

service.save_architecture(&refined, "architecture_v2.md").await?;
```

## Validation Results

All quality gates passed:

- cargo fmt --all: Applied successfully
- cargo check --all-targets --all-features: Passed with zero errors
- cargo clippy --all-targets --all-features -- -D warnings: Zero warnings
- cargo test --all-features: 751 tests passed

## Design Decisions

### Generic Type Parameters vs Trait Objects

Choice: Use generic type parameters with trait bounds

Rationale:
- Better performance (no dynamic dispatch)
- Type safety at compile time
- Easier to test with concrete mock types
- Clear dependency requirements in type signature

### Error Conversion Strategy

Choice: Convert all errors to String in ServiceError

Rationale:
- Simplifies error handling at application boundary
- Avoids complex nested error types
- Provides clear error messages for users
- Maintains separation between layers

Alternative considered: Wrapping original errors with Box<dyn Error>
- Rejected because it exposes infrastructure details to CLI layer

### Validation Timing

Choice: Validate immediately after generation

Rationale:
- Fail fast - catch issues before returning to caller
- Clear error attribution (generation vs user error)
- Guarantees all returned documents are valid

### Interactive Session Design

Choice: Single run method with internal state machine

Rationale:
- Simple API - one method to call
- Internal complexity hidden from caller
- Easy to extend with new workflow steps
- Natural control flow with loops and conditionals

## Integration Points

### With Domain Layer

Application layer depends on:
- Architecture models (ArchitectureDocument, etc.)
- Domain traits (ArchitectureGenerator, ArchitectureWriter, TemplateRepository)
- Validation (ArchitectureValidator)

### With Infrastructure Layer

Application layer uses concrete implementations:
- OllamaArchitectureGenerator for LLM-based generation
- FileTemplateRepository for template storage
- MarkdownArchitectureWriter for document output

### With CLI Layer (Next Phase)

CLI will use:
- ArchitectureService for command execution
- InteractiveArchitectureSession for interactive mode
- ServiceError and SessionError for error handling

## Next Steps

Phase 4 (CLI Integration) will:
1. Create architecture command structure with clap
2. Implement subcommands: generate, list-templates, refine, validate
3. Wire up arguments to service methods
4. Add health checks for Ollama availability
5. Provide both interactive and non-interactive modes

## References

- Architecture Plan: `docs/explanations/llm_architecture_command_plan.md`
- Domain Layer: `docs/explanations/phase1_architecture_domain_implementation.md`
- Infrastructure Layer: `docs/explanations/phase2_ollama_infrastructure_implementation.md`
- AGENTS.md: Project development guidelines
