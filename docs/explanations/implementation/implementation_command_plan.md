# Implementation Command Plan

## Overview

This document outlines the detailed implementation plan for a new `implementation` command in xzagentz. The command will integrate with Ollama to generate phased implementation plans from architecture documents, featuring interactive planning capabilities and file output functionality.

## Business Requirements

### Primary Goal

Enable users to automatically generate structured, phased implementation plans from architecture documents using local LLM inference via Ollama.

### Key Features

1. **Ollama Integration**: Connect to local Ollama instance to leverage LLM capabilities
2. **Interactive Planning**: Guide users through plan generation with prompts and confirmations
3. **File Output**: Write generated plans to markdown files following project conventions
4. **Architecture Parsing**: Parse and extract structured information from architecture documents
5. **Phased Approach**: Generate plans organized by implementation phases

### User Workflow

```text
User Input: architecture.md → CLI parses → Ollama generates plan → User reviews interactively → Plan saved to file
```

## Technical Architecture

### Layer Design

Following xzagentz layered architecture:

```text
┌────────────────────────────────────────────┐
│  CLI Layer (src/cli/implementation.rs)     │
│  - Command argument parsing                │
│  - Interactive prompts and confirmations   │
│  - Output formatting and display           │
├────────────────────────────────────────────┤
│  Application Layer                         │
│  (src/application/planning/)               │
│  - PlanningService - orchestration         │
│  - PlanGenerationUseCase                   │
│  - InteractivePlanningSession              │
├────────────────────────────────────────────┤
│  Domain Layer (src/domain/planning/)       │
│  - Plan, Phase, Task models                │
│  - PlanGenerator trait                     │
│  - ArchitectureDocument model              │
│  - NO infrastructure dependencies          │
├────────────────────────────────────────────┤
│  Infrastructure Layer                      │
│  (src/infrastructure/)                     │
│  - ollama/client.rs - HTTP client          │
│  - ollama/models.rs - API types            │
│  - planning/file_writer.rs                 │
│  - planning/markdown_formatter.rs          │
└────────────────────────────────────────────┘
```

### Dependency Flow

- CLI → Application → Domain
- Infrastructure → Domain (implements traits)
- NO Domain → Infrastructure dependencies

## Phase 1: Domain Models and Traits

### Phase 1.1: Core Domain Models

**Location**: `src/domain/planning/mod.rs`

```rust
/// Represents a complete implementation plan
pub struct Plan {
    pub title: String,
    pub description: String,
    pub phases: Vec<Phase>,
    pub metadata: PlanMetadata,
}

/// Represents a single implementation phase
pub struct Phase {
    pub id: PhaseId,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
    pub dependencies: Vec<PhaseId>,
    pub estimated_duration: Option<Duration>,
}

/// Represents a task within a phase
pub struct Task {
    pub name: String,
    pub description: String,
    pub acceptance_criteria: Vec<String>,
    pub components: Vec<String>,
}

/// Metadata about plan generation
pub struct PlanMetadata {
    pub generated_at: DateTime<Utc>,
    pub model_used: String,
    pub source_document: PathBuf,
    pub version: String,
}
```

**Deliverables**:
- `src/domain/planning/mod.rs` - Module root
- `src/domain/planning/models.rs` - Core models
- `src/domain/planning/phase.rs` - Phase model
- `src/domain/planning/task.rs` - Task model
- Unit tests with >80% coverage

**Tests Required**:
- Plan creation and validation
- Phase dependency validation
- Task acceptance criteria parsing
- Metadata serialization

### Phase 1.2: Domain Traits

**Location**: `src/domain/planning/traits.rs`

```rust
/// Trait for generating implementation plans
pub trait PlanGenerator {
    type Error: std::error::Error;

    fn generate_plan(
        &self,
        architecture: &ArchitectureDocument,
        options: &PlanOptions,
    ) -> Result<Plan, Self::Error>;
}

/// Trait for parsing architecture documents
pub trait ArchitectureParser {
    type Error: std::error::Error;

    fn parse(&self, content: &str) -> Result<ArchitectureDocument, Self::Error>;
}

/// Trait for writing plans to storage
pub trait PlanWriter {
    type Error: std::error::Error;

    fn write_plan(&self, plan: &Plan, output_path: &Path) -> Result<(), Self::Error>;
}
```

**Deliverables**:
- `src/domain/planning/traits.rs` - Trait definitions
- Documentation with examples
- Mock implementations for testing

**Tests Required**:
- Trait contract tests
- Mock implementation tests

### Phase 1.3: Architecture Document Model

**Location**: `src/domain/planning/architecture.rs`

```rust
/// Represents a parsed architecture document
pub struct ArchitectureDocument {
    pub title: String,
    pub sections: Vec<Section>,
    pub components: Vec<Component>,
    pub requirements: Vec<Requirement>,
}

/// A section within the architecture document
pub struct Section {
    pub heading: String,
    pub level: u8,
    pub content: String,
    pub subsections: Vec<Section>,
}
```

**Deliverables**:
- `src/domain/planning/architecture.rs` - Architecture models
- Parsing logic for markdown structure
- Tests for document structure

## Phase 2: Infrastructure - Ollama Integration

### Phase 2.1: Ollama HTTP Client

**Location**: `src/infrastructure/ollama/client.rs`

**Dependencies to Add**:
```toml
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.35", features = ["full"] }
```

```rust
/// Client for interacting with Ollama API
pub struct OllamaClient {
    base_url: String,
    http_client: reqwest::Client,
    timeout: Duration,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Result<Self, OllamaError>;

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, OllamaError>;

    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) -> Result<GenerateResponse, OllamaError>;

    pub async fn generate_stream(
        &self,
        request: GenerateRequest,
    ) -> Result<impl Stream<Item = Result<GenerateChunk>>, OllamaError>;

    pub async fn health_check(&self) -> Result<(), OllamaError>;
}
```

**Deliverables**:
- `src/infrastructure/ollama/mod.rs` - Module root
- `src/infrastructure/ollama/client.rs` - HTTP client
- `src/infrastructure/ollama/models.rs` - API request/response types
- `src/infrastructure/ollama/error.rs` - Error types
- Integration tests with mock Ollama server

**Tests Required**:
- Connection establishment
- Request serialization
- Response deserialization
- Error handling (timeout, connection refused, invalid response)
- Health check functionality

### Phase 2.2: Ollama Configuration

**Location**: `src/infrastructure/ollama/config.rs`

```rust
/// Configuration for Ollama integration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OllamaConfig {
    pub base_url: String,
    pub default_model: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            default_model: "llama3.2:3b".to_string(),
            timeout_seconds: 300,
            max_retries: 3,
        }
    }
}
```

**Deliverables**:
- `src/infrastructure/ollama/config.rs` - Configuration struct
- Environment variable support
- Configuration file loading
- Tests for config parsing

### Phase 2.3: LLM Plan Generator Implementation

**Location**: `src/infrastructure/ollama/plan_generator.rs`

```rust
/// Implementation of PlanGenerator using Ollama
pub struct OllamaPlanGenerator {
    client: OllamaClient,
    config: GeneratorConfig,
}

impl PlanGenerator for OllamaPlanGenerator {
    type Error = OllamaError;

    fn generate_plan(
        &self,
        architecture: &ArchitectureDocument,
        options: &PlanOptions,
    ) -> Result<Plan, Self::Error> {
        // 1. Build prompt from architecture document
        // 2. Send to Ollama
        // 3. Parse response into Plan structure
        // 4. Validate plan completeness
    }
}
```

**Deliverables**:
- `src/infrastructure/ollama/plan_generator.rs` - Generator implementation
- `src/infrastructure/ollama/prompts.rs` - Prompt templates
- Response parsing logic
- Tests with mock responses

**Tests Required**:
- Prompt generation from architecture
- Response parsing
- Error handling for malformed responses
- Validation of generated plans

## Phase 3: Infrastructure - File I/O

### Phase 3.1: Plan File Writer

**Location**: `src/infrastructure/planning/file_writer.rs`

```rust
/// Writes plans to markdown files
pub struct MarkdownPlanWriter {
    formatter: MarkdownFormatter,
}

impl PlanWriter for MarkdownPlanWriter {
    type Error = PlanWriteError;

    fn write_plan(&self, plan: &Plan, output_path: &Path) -> Result<(), Self::Error> {
        // 1. Format plan as markdown
        // 2. Validate output path
        // 3. Create parent directories if needed
        // 4. Write atomically with temp file
    }
}
```

**Deliverables**:
- `src/infrastructure/planning/mod.rs` - Module root
- `src/infrastructure/planning/file_writer.rs` - File writing
- `src/infrastructure/planning/markdown_formatter.rs` - Markdown formatting
- Tests with tempfile

**Tests Required**:
- File creation
- Atomic writes
- Directory creation
- Permission errors
- Markdown formatting validation

### Phase 3.2: Architecture Parser Implementation

**Location**: `src/infrastructure/planning/architecture_parser.rs`

```rust
/// Parses markdown architecture documents
pub struct MarkdownArchitectureParser;

impl ArchitectureParser for MarkdownArchitectureParser {
    type Error = ParseError;

    fn parse(&self, content: &str) -> Result<ArchitectureDocument, Self::Error> {
        // 1. Parse markdown structure
        // 2. Extract sections by heading level
        // 3. Identify components and requirements
        // 4. Build ArchitectureDocument
    }
}
```

**Deliverables**:
- `src/infrastructure/planning/architecture_parser.rs` - Parser implementation
- Markdown parsing utilities
- Tests with sample architecture documents

**Tests Required**:
- Heading extraction
- Section nesting
- Component identification
- Requirement parsing
- Edge cases (empty document, malformed markdown)

## Phase 4: Application Layer

### Phase 4.1: Planning Service

**Location**: `src/application/planning/service.rs`

```rust
/// Orchestrates the planning workflow
pub struct PlanningService {
    generator: Box<dyn PlanGenerator<Error = PlanningError>>,
    parser: Box<dyn ArchitectureParser<Error = PlanningError>>,
    writer: Box<dyn PlanWriter<Error = PlanningError>>,
}

impl PlanningService {
    pub fn new(
        generator: Box<dyn PlanGenerator<Error = PlanningError>>,
        parser: Box<dyn ArchitectureParser<Error = PlanningError>>,
        writer: Box<dyn PlanWriter<Error = PlanningError>>,
    ) -> Self;

    pub async fn generate_implementation_plan(
        &self,
        architecture_path: &Path,
        options: PlanOptions,
    ) -> Result<Plan, PlanningError>;

    pub async fn save_plan(
        &self,
        plan: &Plan,
        output_path: &Path,
    ) -> Result<(), PlanningError>;
}
```

**Deliverables**:
- `src/application/planning/mod.rs` - Module root
- `src/application/planning/service.rs` - Service implementation
- `src/application/planning/options.rs` - Configuration options
- `src/application/planning/error.rs` - Error types
- Tests with mock dependencies

**Tests Required**:
- End-to-end workflow
- Error propagation
- Dependency injection
- Service composition

### Phase 4.2: Interactive Planning Session

**Location**: `src/application/planning/interactive.rs`

**Dependencies to Add**:
```toml
dialoguer = "0.11"
console = "0.15"
indicatif = "0.17"
```

```rust
/// Manages interactive planning workflow with user
pub struct InteractivePlanningSession {
    service: PlanningService,
    config: InteractiveConfig,
}

impl InteractivePlanningSession {
    pub async fn run(&mut self) -> Result<Plan, PlanningError> {
        // 1. Prompt for architecture file
        // 2. Confirm Ollama connection
        // 3. Select model
        // 4. Generate plan with progress indicator
        // 5. Review plan phases with user
        // 6. Allow phase editing/refinement
        // 7. Confirm and save
    }

    fn prompt_architecture_file(&self) -> Result<PathBuf, PlanningError>;
    fn select_ollama_model(&self, models: &[ModelInfo]) -> Result<String, PlanningError>;
    fn review_phases(&self, phases: &[Phase]) -> Result<ReviewAction, PlanningError>;
    fn prompt_output_path(&self, default: &Path) -> Result<PathBuf, PlanningError>;
}
```

**Deliverables**:
- `src/application/planning/interactive.rs` - Interactive session
- User prompts and confirmations
- Progress indicators
- Tests for interaction flows

**Tests Required**:
- Mock user input
- Confirmation flows
- Error handling in interactive mode
- Progress indicator updates

## Phase 5: CLI Integration

### Phase 5.1: Implementation Command

**Location**: `src/cli/implementation.rs`

```rust
/// Arguments for the implementation command
#[derive(Debug, clap::Args)]
pub struct ImplementationArgs {
    /// Path to architecture document
    #[arg(short, long, default_value = "docs/reference/architecture.md")]
    architecture: PathBuf,

    /// Output file path for generated plan
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Ollama model to use
    #[arg(short, long)]
    model: Option<String>,

    /// Ollama base URL
    #[arg(long, default_value = "http://localhost:11434")]
    ollama_url: String,

    /// Interactive mode (default: true)
    #[arg(short, long, default_value_t = true)]
    interactive: bool,

    /// Skip Ollama health check
    #[arg(long)]
    skip_health_check: bool,

    /// Number of phases to generate
    #[arg(short = 'n', long)]
    num_phases: Option<usize>,

    /// Force overwrite output file
    #[arg(short, long)]
    force: bool,
}

pub async fn execute(args: ImplementationArgs) -> anyhow::Result<()>;
```

**Deliverables**:
- `src/cli/implementation.rs` - Command implementation
- Command registration in `src/cli/mod.rs`
- Help text and examples
- Integration tests

**Tests Required**:
- Argument parsing
- Default values
- Required vs optional args
- Help text generation

### Phase 5.2: Command Execution Logic

**Location**: `src/cli/implementation.rs` (continued)

```rust
pub async fn execute(args: ImplementationArgs) -> anyhow::Result<()> {
    // 1. Initialize components
    let ollama_config = OllamaConfig {
        base_url: args.ollama_url,
        default_model: args.model.unwrap_or_else(|| "llama2".to_string()),
        ..Default::default()
    };

    // 2. Health check
    if !args.skip_health_check {
        check_ollama_connection(&ollama_config).await?;
    }

    // 3. Build service
    let service = build_planning_service(ollama_config)?;

    // 4. Execute workflow
    if args.interactive {
        run_interactive_workflow(service, args).await?;
    } else {
        run_batch_workflow(service, args).await?;
    }

    Ok(())
}
```

**Deliverables**:
- Command execution logic
- Service initialization
- Workflow routing
- Error handling and user feedback

### Phase 5.3: Update Commands Enum

**Location**: `src/cli/mod.rs`

Add to `Commands` enum:

```rust
/// Generate implementation plan from architecture document using Ollama
Implementation(implementation::ImplementationArgs),
```

**Deliverables**:
- Update `Commands` enum
- Export implementation module
- Update CLI help text
- Integration with main command router

## Phase 6: Configuration and Documentation

### Phase 6.1: Configuration File Support

**Location**: `src/config/planning.rs`

```yaml
# config/planning.yaml
ollama:
  base_url: "http://localhost:11434"
  default_model: "llama3.2:3b"
  timeout_seconds: 300

planning:
  default_phases: 5
  default_output_dir: "docs/plans"
  output_format: "markdown"

interactive:
  enable_colors: true
  show_progress: true
  confirm_before_save: true
```

**Deliverables**:
- `src/config/planning.rs` - Configuration structs
- YAML schema and validation
- Configuration loading and merging
- Tests for config precedence

### Phase 6.2: User Documentation

**Location**: `docs/how_to/generate_implementation_plan.md`

**Content Structure**:
1. Prerequisites (Ollama installation)
2. Quick start guide
3. Command-line options
4. Interactive mode walkthrough
5. Batch mode examples
6. Configuration customization
7. Troubleshooting

**Deliverables**:
- `docs/how_to/generate_implementation_plan.md` - User guide
- `docs/reference/implementation_command.md` - Command reference
- `docs/explanations/ollama_integration.md` - Architecture explanation
- Example architecture documents in `docs/examples/`

### Phase 6.3: Implementation Summary Document

**Location**: `docs/explanations/implementation_command_implementation.md`

Following AGENTS.md requirements, create comprehensive summary including:
- Overview of implementation
- Components delivered with line counts
- Implementation details
- Testing coverage
- Usage examples
- Validation results

## Phase 7: Testing and Quality Assurance

### Phase 7.1: Unit Test Coverage

**Requirements**:
- Minimum 80% code coverage
- All public functions tested
- Success and failure cases
- Edge cases and boundaries

**Test Files**:
- `src/domain/planning/tests.rs`
- `src/infrastructure/ollama/tests.rs`
- `src/infrastructure/planning/tests.rs`
- `src/application/planning/tests.rs`
- `src/cli/implementation/tests.rs`

### Phase 7.2: Integration Tests

**Location**: `tests/integration/implementation_command.rs`

```rust
#[tokio::test]
async fn test_implementation_command_end_to_end() {
    // 1. Start mock Ollama server
    // 2. Create test architecture document
    // 3. Run implementation command
    // 4. Verify plan file created
    // 5. Validate plan structure
}

#[tokio::test]
async fn test_implementation_command_with_invalid_architecture() {
    // Test error handling
}

#[tokio::test]
async fn test_implementation_command_ollama_unavailable() {
    // Test connection error handling
}
```

**Deliverables**:
- `tests/integration/implementation_command.rs` - Integration tests
- Mock Ollama server for testing
- Test fixtures and sample documents
- CI integration

### Phase 7.3: Quality Gates

Run before completion:

```bash
# Format
cargo fmt --all

# Check
cargo check --all-targets --all-features

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all-features

# Documentation
cargo doc --no-deps --open
```

All must pass with zero errors and zero warnings.

## Dependencies Summary

### New Dependencies Required

```toml
[dependencies]
# Existing dependencies remain...

# HTTP client for Ollama
reqwest = { version = "0.11", features = ["json"] }

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Interactive CLI
dialoguer = "0.11"
console = "0.15"
indicatif = "0.17"

[dev-dependencies]
# Existing dev dependencies remain...

# HTTP mocking
wiremock = "0.5"
```

## Risk Assessment and Mitigation

### Risk 1: Ollama Service Unavailable

**Mitigation**:
- Implement health check before operations
- Provide clear error messages with setup instructions
- Add `--skip-health-check` flag for advanced users
- Timeout configuration

### Risk 2: LLM Response Quality

**Mitigation**:
- Structured prompts with clear format expectations
- Response validation and parsing with fallbacks
- Interactive mode allows user review and refinement
- Allow manual editing of generated plans

### Risk 3: Large Architecture Documents

**Mitigation**:
- Implement token counting and truncation
- Break large documents into sections
- Stream responses for better UX
- Progress indicators for long operations

### Risk 4: File I/O Errors

**Mitigation**:
- Atomic writes using temp files
- Permission checks before writing
- Automatic backup creation
- Clear error messages with resolution steps

## Success Criteria

### Functional Requirements

- [ ] Successfully connect to Ollama instance
- [ ] Parse architecture.md documents
- [ ] Generate structured implementation plans
- [ ] Support interactive and batch modes
- [ ] Write plans to markdown files
- [ ] Handle errors gracefully with clear messages

### Quality Requirements

- [ ] All tests pass (>80% coverage)
- [ ] Zero clippy warnings
- [ ] Code properly formatted
- [ ] Comprehensive documentation
- [ ] Performance: Plan generation <60 seconds for typical architecture
- [ ] Memory: <100MB memory usage during generation

### User Experience Requirements

- [ ] Clear progress indicators
- [ ] Helpful error messages
- [ ] Intuitive interactive prompts
- [ ] Sensible defaults
- [ ] Example workflows in documentation

## Timeline Estimate

Assuming single developer, 4-6 hours per day:

- Phase 1: Domain Models and Traits - 2 days
- Phase 2: Infrastructure - Ollama Integration - 3 days
- Phase 3: Infrastructure - File I/O - 2 days
- Phase 4: Application Layer - 2 days
- Phase 5: CLI Integration - 2 days
- Phase 6: Configuration and Documentation - 2 days
- Phase 7: Testing and QA - 2 days

**Total Estimated Time**: 15 working days (3 weeks)

## Appendix A: Example Prompt Template

```markdown
You are an expert software architect. Generate a detailed implementation plan for the following architecture.

ARCHITECTURE DOCUMENT:
{architecture_content}

REQUIREMENTS:
- Organize the plan into {num_phases} distinct phases
- Each phase should have clear deliverables
- Include dependencies between phases
- Provide estimated time for each phase
- List specific files and components to create
- Include testing requirements

OUTPUT FORMAT (strict JSON):
{
  "title": "Implementation Plan: {title}",
  "description": "...",
  "phases": [
    {
      "id": "phase-1",
      "name": "...",
      "description": "...",
      "tasks": [
        {
          "name": "...",
          "description": "...",
          "acceptance_criteria": ["...", "..."],
          "components": ["file1.rs", "file2.rs"]
        }
      ],
      "dependencies": [],
      "estimated_duration": "2 days"
    }
  ]
}
```

## Appendix B: File Structure

```text
xzagentz/
├── src/
│   ├── domain/
│   │   └── planning/
│   │       ├── mod.rs
│   │       ├── models.rs
│   │       ├── phase.rs
│   │       ├── task.rs
│   │       ├── traits.rs
│   │       ├── architecture.rs
│   │       └── tests.rs
│   ├── application/
│   │   └── planning/
│   │       ├── mod.rs
│   │       ├── service.rs
│   │       ├── interactive.rs
│   │       ├── options.rs
│   │       ├── error.rs
│   │       └── tests.rs
│   ├── infrastructure/
│   │   ├── ollama/
│   │   │   ├── mod.rs
│   │   │   ├── client.rs
│   │   │   ├── config.rs
│   │   │   ├── models.rs
│   │   │   ├── error.rs
│   │   │   ├── plan_generator.rs
│   │   │   ├── prompts.rs
│   │   │   └── tests.rs
│   │   └── planning/
│   │       ├── mod.rs
│   │       ├── file_writer.rs
│   │       ├── markdown_formatter.rs
│   │       ├── architecture_parser.rs
│   │       └── tests.rs
│   ├── config/
│   │   └── planning.rs
│   └── cli/
│       └── implementation.rs
├── tests/
│   └── integration/
│       └── implementation_command.rs
├── docs/
│   ├── how_to/
│   │   └── generate_implementation_plan.md
│   ├── reference/
│   │   └── implementation_command.md
│   ├── explanations/
│   │   ├── implementation_command_plan.md (this document)
│   │   ├── implementation_command_implementation.md (created after completion)
│   │   └── ollama_integration.md
│   └── examples/
│       └── sample_architecture.md
└── config/
    └── planning.yaml
```

## Appendix C: Validation Checklist

Before marking implementation complete, verify:

### Code Quality

- [ ] `cargo fmt --all` applied successfully
- [ ] `cargo check --all-targets --all-features` passes with zero errors
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] `cargo test --all-features` passes with >80% coverage

### Architecture Compliance

- [ ] Domain layer has no infrastructure dependencies
- [ ] Traits defined in domain layer
- [ ] Infrastructure implements domain traits
- [ ] Application layer orchestrates workflow
- [ ] CLI layer is thin, delegates to application layer

### Documentation

- [ ] All public items have doc comments
- [ ] Doc comments include examples
- [ ] How-to guide created
- [ ] Reference documentation created
- [ ] Implementation summary created in `docs/explanations/`
- [ ] All documentation uses lowercase_with_underscores.md
- [ ] No emojis in documentation

### Testing

- [ ] Unit tests for all public functions
- [ ] Integration tests for command workflow
- [ ] Mock implementations for external dependencies
- [ ] Error cases tested
- [ ] Edge cases covered

### Files and Conventions

- [ ] All YAML files use `.yaml` extension
- [ ] All Markdown files use `.md` extension
- [ ] Lowercase filenames (except README.md)
- [ ] Proper error handling with Result types
- [ ] No unwrap() without justification

## References

- AGENTS.md - Development guidelines
- docs/reference/architecture.md - Project architecture
- Ollama API Documentation: https://github.com/ollama/ollama/blob/main/docs/api.md
- Diataxis Framework: https://diataxis.fr/
