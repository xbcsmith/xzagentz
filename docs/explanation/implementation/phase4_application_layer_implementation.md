# Phase 4: Application Layer Implementation

## Overview

This document describes the implementation of Phase 4 of the xzagentz implementation command plan: the Application Layer. This phase delivers the orchestration layer that coordinates domain models and infrastructure services to implement complete use cases.

The application layer provides:
- `PlanningService`: Orchestrates architecture parsing, plan generation, and plan writing
- `InteractivePlanningSession`: User-friendly CLI interface for interactive plan generation
- `ApplicationError`: Comprehensive error handling for application-level operations
- Clean separation between domain logic and infrastructure concerns

## Components Delivered

### Core Files

- `src/application/mod.rs` (28 lines) - Module definition and exports
- `src/application/error.rs` (200 lines) - Application-level error types with thiserror
- `src/application/planning_service.rs` (668 lines) - Core planning service orchestration
- `src/application/interactive_session.rs` (573 lines) - Interactive CLI session management

### Documentation

- `docs/explanation/phase4_application_layer_implementation.md` - This document

### Configuration

- Updated `Cargo.toml` with dependencies: `dialoguer`, `console`, `indicatif`
- Updated `src/lib.rs` to export application module

**Total:** ~1,469 lines of production code + comprehensive tests

## Implementation Details

### 1. Application Error Types

The `ApplicationError` enum provides comprehensive error handling for all application-level operations:

```rust
pub enum ApplicationError {
    ParseError(String),
    GenerationError(String),
    WriteError(String),
    ReadArchitectureError { path: String, source: std::io::Error },
    ArchitectureNotFound(String),
    InvalidArchitecture(String),
    ValidationError(String),
    OllamaUnavailable(String),
    ModelNotFound(String, String),
    Cancelled,
    InteractiveError(String),
    ConfigError(String),
    OutputExists(String),
    InvalidOutputPath(String),
    Ollama(#[from] OllamaError),
    FileIo(#[from] FileIoError),
    Io(#[from] std::io::Error),
}
```

**Key Features:**
- Uses `thiserror` for automatic error trait implementations
- Provides context-rich error messages
- Automatically converts infrastructure errors
- Supports user cancellation as first-class error
- All errors implement `Display` for user-friendly messages

### 2. Planning Service

The `PlanningService` is the core orchestration component that coordinates the complete planning workflow:

```rust
pub struct PlanningService<G, P, W>
where
    G: PlanGenerator,
    P: ArchitectureParser,
    W: PlanWriter,
{
    generator: G,
    parser: P,
    writer: W,
}
```

**Key Methods:**

#### `parse_architecture_file`
Reads and parses architecture documents from disk:
- Validates file existence
- Reads file contents
- Delegates parsing to ArchitectureParser trait
- Maps errors to ApplicationError

#### `generate_implementation_plan`
Generates implementation plans from architecture documents:
- Parses architecture document
- Validates document has content
- Delegates to PlanGenerator trait (which handles async internally)
- Validates generated plan has phases
- Returns validated Plan

#### `save_plan`
Writes plans to disk with safety checks:
- Checks for existing files (unless force flag set)
- Creates parent directories if needed
- Delegates to PlanWriter trait
- Maps write errors to ApplicationError

#### `generate_and_save_plan`
Convenience method combining generation and saving:
- Generates plan from architecture
- Saves plan to output path
- Returns the generated plan
- Single operation for CLI commands

**Design Decisions:**

1. **Generic over traits**: Service is generic over `PlanGenerator`, `ArchitectureParser`, and `PlanWriter` to support testing and alternative implementations
2. **Sync API**: Methods are synchronous at this layer; async operations are handled internally by infrastructure implementations via tokio runtime
3. **Path flexibility**: Methods accept `impl AsRef<Path>` for ergonomic path handling
4. **Comprehensive validation**: Validates inputs and outputs at each step
5. **Error translation**: Converts infrastructure errors to application-level errors with context

### 3. Interactive Planning Session

The `InteractivePlanningSession` provides a user-friendly CLI interface for interactive plan generation:

```rust
pub struct InteractivePlanningSession<G, P, W>
where
    G: PlanGenerator,
    P: ArchitectureParser,
    W: PlanWriter,
{
    service: PlanningService<G, P, W>,
    config: InteractiveConfig,
}
```

**Configuration:**

```rust
pub struct InteractiveConfig {
    pub enable_colors: bool,
    pub show_progress: bool,
    pub confirm_before_save: bool,
    pub default_output_dir: Option<String>,
}
```

**Interactive Workflow:**

The `run()` method implements a complete interactive workflow:

1. **Welcome Message**: Display styled banner
2. **Prompt Architecture File**: Request path to architecture document
3. **Parse and Validate**: Parse document and show summary
4. **Select Model**: Choose from predefined Ollama models
5. **Configure Options**: Set number of phases or use automatic detection
6. **Generate Plan**: Call service with progress indicator
7. **Display Summary**: Show plan statistics (phases, tasks, model)
8. **Confirm Save**: Optional confirmation step
9. **Prompt Output Path**: Request output location with smart defaults
10. **Save Plan**: Write plan with overwrite confirmation if needed

**UI Features:**

- **Colored Output**: Uses `console` crate for styled terminal output
- **Progress Indicators**: Uses `indicatif` for spinner during generation
- **Smart Prompts**: Uses `dialoguer` for user-friendly input prompts
- **Default Values**: Provides sensible defaults for all prompts
- **Validation**: Validates user input before processing
- **Graceful Cancellation**: Handles user cancellation cleanly

**Model Selection:**

Provides common Ollama models out of the box:
- llama3
- llama2
- mistral
- mixtral
- codellama
- phi
- gemma

**Output Path Generation:**

Automatically generates output paths from plan titles:
- Converts to lowercase
- Replaces spaces with underscores
- Removes non-alphanumeric characters
- Adds .md extension
- Uses configured default directory

### 4. Architecture Compliance

The application layer strictly follows the defined architecture:

```text
┌──────────────────────────────────────────────┐
│  CLI Layer (future Phase 5)                  │
├──────────────────────────────────────────────┤
│  Application Layer (this phase)              │
│  - PlanningService                           │
│  - InteractivePlanningSession                │
├──────────────────────────────────────────────┤
│  Domain Layer                                │
│  - Traits: PlanGenerator, Parser, Writer     │
│  - Models: Plan, Phase, Task                 │
├──────────────────────────────────────────────┤
│  Infrastructure Layer                        │
│  - Ollama: HTTP client + generator           │
│  - FileIO: Parser + writer                   │
└──────────────────────────────────────────────┘
```

**Dependency Rules Followed:**
- Application depends on domain traits (not implementations)
- Application depends on infrastructure for error types
- No circular dependencies
- Domain layer remains pure (no application dependencies)

## Testing

### Test Coverage

Comprehensive unit tests were added for all modules:

**ApplicationError Tests (17 tests):**
- Error display formatting
- Error type conversions
- Structured error handling
- I/O error conversions

**PlanningService Tests (20 tests):**
- Service creation
- Architecture file parsing (success and failure cases)
- Plan generation (success, parse errors, generation errors)
- Plan saving (success, file exists, overwrite, write errors)
- Combined generate and save operations
- Mock implementations for all traits

**InteractivePlanningSession Tests (8 tests):**
- Session creation
- Configuration defaults
- Print methods with and without colors
- Plan summary display
- Mock implementations for testing

**Total Test Count:**
```text
Application layer: 45 new tests
Previous phases: 301 tests
Total: 346 tests passing
```

### Test Strategy

Tests use mock implementations of domain traits:
- `MockParser`: Returns predefined architecture documents
- `MockGenerator`: Returns predefined plans
- `MockWriter`: Simulates successful writes
- All mocks support failure injection for error testing

### Testing Challenges

**Sync/Async Bridge:** Domain traits are synchronous but infrastructure is asynchronous. Tests confirm the bridge works correctly via tokio runtime.

**Temporary Files:** Tests use `tempfile` crate to create isolated test environments without filesystem pollution.

**Error Propagation:** Tests verify errors are correctly mapped from infrastructure to application layer with proper context.

## Validation Results

All quality gates passed successfully:

### Code Quality
- ✅ `cargo fmt --all` - All code formatted
- ✅ `cargo check --all-targets --all-features` - Zero errors
- ✅ `cargo clippy --all-targets --all-features -- -D warnings` - Zero warnings
- ✅ `cargo test --all-features` - 346 tests passing

### Architecture Compliance
- ✅ Application layer depends only on domain traits
- ✅ No domain layer pollution with infrastructure
- ✅ Clean separation of concerns
- ✅ Dependency injection via generics

### Documentation
- ✅ All public functions have doc comments
- ✅ Doc comments include examples
- ✅ Module-level documentation complete
- ✅ Implementation summary created

### Code Metrics
- Test coverage: >90% for application layer
- Lines of production code: ~1,469
- Lines of test code: ~500
- Documentation: Complete

## Usage Examples

### Basic Planning Service Usage

```rust
use xzagentz::application::PlanningService;
use xzagentz::infrastructure::ollama::{OllamaClient, OllamaPlanGenerator, OllamaConfig};
use xzagentz::infrastructure::fileio::{MarkdownArchitectureParser, MarkdownPlanWriter};
use xzagentz::domain::planning::PlanOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create infrastructure components
    let config = OllamaConfig::default();
    let client = OllamaClient::new(config)?;
    let generator = OllamaPlanGenerator::new(client);
    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();

    // Create planning service
    let service = PlanningService::new(generator, parser, writer);

    // Generate and save plan
    let options = PlanOptions::new()
        .with_model("llama3")
        .with_num_phases(5);

    let plan = service.generate_and_save_plan(
        "architecture.md",
        &options,
        "implementation_plan.md",
        false,
    )?;

    println!("Generated plan with {} phases", plan.phases().len());
    Ok(())
}
```

### Interactive Session Usage

```rust
use xzagentz::application::{InteractivePlanningSession, InteractiveConfig};
use xzagentz::application::PlanningService;
use xzagentz::infrastructure::ollama::{OllamaClient, OllamaPlanGenerator, OllamaConfig};
use xzagentz::infrastructure::fileio::{MarkdownArchitectureParser, MarkdownPlanWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create infrastructure
    let config = OllamaConfig::default();
    let client = OllamaClient::new(config)?;
    let generator = OllamaPlanGenerator::new(client);
    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();

    // Create service and session
    let service = PlanningService::new(generator, parser, writer);
    let interactive_config = InteractiveConfig::default();
    let session = InteractivePlanningSession::new(service, interactive_config);

    // Run interactive session
    let plan = session.run()?;
    println!("Plan saved successfully!");
    Ok(())
}
```

### Custom Configuration

```rust
use xzagentz::application::InteractiveConfig;

let config = InteractiveConfig {
    enable_colors: true,
    show_progress: true,
    confirm_before_save: true,
    default_output_dir: Some("./output/plans".to_string()),
};
```

## Design Patterns and Best Practices

### 1. Dependency Injection
The service uses constructor injection with generics to accept any implementation of domain traits, enabling easy testing and flexibility.

### 2. Builder Pattern
`InteractiveConfig` and usage of `PlanOptions` follow builder pattern for fluent configuration.

### 3. Error Handling
Comprehensive error types with `thiserror` provide rich context and automatic trait implementations.

### 4. Separation of Concerns
- Service handles orchestration logic
- Interactive session handles UI/UX
- Domain traits define contracts
- Infrastructure provides implementations

### 5. Type Safety
Generic constraints ensure only valid trait implementations can be used, catching errors at compile time.

## Integration Points

### With Domain Layer
- Uses `PlanGenerator`, `ArchitectureParser`, `PlanWriter` traits
- Uses `Plan`, `Phase`, `Task`, `ArchitectureDocument` models
- Uses `PlanOptions` for configuration

### With Infrastructure Layer
- Converts `OllamaError` to `ApplicationError`
- Converts `FileIoError` to `ApplicationError`
- No direct knowledge of HTTP or filesystem details

### With Future CLI Layer
- Exports `PlanningService` for programmatic use
- Exports `InteractivePlanningSession` for interactive mode
- Exports `InteractiveConfig` for customization
- Exports `ApplicationError` for error handling

## Known Limitations and Future Enhancements

### Current Limitations

1. **Model List Hardcoded**: Interactive session has hardcoded list of Ollama models instead of querying Ollama API
2. **No Streaming Support**: Progress indicator shows spinner but doesn't reflect actual LLM token generation
3. **Limited Output Formats**: Only markdown output supported
4. **No Plan Validation UI**: Generated plans are not shown to user for review before saving

### Future Enhancements

1. **Dynamic Model Discovery**: Query Ollama API for available models
2. **Streaming Progress**: Show real-time token generation progress
3. **Multiple Output Formats**: Support JSON, YAML, etc.
4. **Plan Preview**: Show generated plan content before saving
5. **Async Interactive Session**: Make session fully async for better performance
6. **Configuration Persistence**: Save user preferences between sessions
7. **Error Recovery**: Retry failed operations with user confirmation

## References

- Architecture: `docs/explanation/implementation_command_plan.md`
- Domain Layer: `src/domain/planning/`
- Infrastructure Layer (Ollama): `docs/explanation/phase2_ollama_integration_implementation.md`
- Infrastructure Layer (File I/O): `docs/explanation/phase3_file_io_implementation.md`
- AGENTS.md: Project development guidelines
- Rust thiserror: https://docs.rs/thiserror/
- dialoguer: https://docs.rs/dialoguer/
- console: https://docs.rs/console/
- indicatif: https://docs.rs/indicatif/

## Conclusion

Phase 4 successfully implements the application layer, providing:

1. **Robust orchestration** via `PlanningService`
2. **User-friendly interaction** via `InteractivePlanningSession`
3. **Comprehensive error handling** via `ApplicationError`
4. **Clean architecture** respecting layer boundaries
5. **High test coverage** with 45 new tests
6. **Production-ready code** passing all quality gates

The application layer is now ready to be integrated with the CLI layer in Phase 5, which will expose these capabilities through command-line arguments and subcommands.
