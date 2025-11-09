# xzagentz Architecture Reference

## Overview

xzagentz is a Rust-based CLI tool for managing language-agnostic development components with intelligent rendering, validation, and code generation capabilities. The system uses a layered architecture that separates concerns across parsing, validation, rendering, and code generation domains.

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Interface                          │
│              (Commands: create, update, add,                │
│               validate, list, prompt)                       │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────────────┐
│                   Application Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Validator  │  │   Renderer   │  │   Prompt     │     │
│  │   Services   │  │   Services   │  │  Generator   │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────────────┐
│                    Domain Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Component   │  │   Template   │  │     Plan     │     │
│  │   System     │  │   System     │  │   System     │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└────────────────────┬────────────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────────────┐
│                  Infrastructure Layer                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ File System  │  │    Parser    │  │    Config    │     │
│  │   (I/O)      │  │   (YAML/MD)  │  │   Loader     │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

#### CLI Interface Layer

- **Location**: `src/cli/`, `src/main.rs`
- **Purpose**: User interaction and command routing
- **Responsibilities**:
  - Parse command-line arguments
  - Validate user input
  - Route to appropriate application services
  - Format and display output
  - Handle user errors gracefully

**Key Modules**:

- `src/cli/mod.rs` - Command definitions and routing
- `src/cli/create.rs` - Create command implementation
- `src/cli/update.rs` - Update command implementation
- `src/cli/add.rs` - Add command implementation
- `src/cli/validate.rs` - Validate command implementation
- `src/cli/list.rs` - List command implementation
- `src/cli/prompt.rs` - Prompt generation command

#### Application Layer

- **Location**: `src/components/`, `src/templates/`, `src/prompts/`
- **Purpose**: Business logic and orchestration
- **Responsibilities**:
  - Coordinate domain services
  - Implement use cases
  - Manage workflow state
  - Apply business rules
  - Generate output artifacts

**Key Modules**:

- `src/components/loader.rs` - Component loading and caching
- `src/components/renderer.rs` - Language-specific rendering
- `src/components/validator.rs` - Component validation
- `src/templates/renderer.rs` - Template rendering
- `src/templates/validator.rs` - Template validation
- `src/prompts/generator.rs` - AI prompt generation

#### Domain Layer

- **Location**: `src/components/`, `src/templates/`, `src/plans/`, `src/validator/`
- **Purpose**: Core business models and rules
- **Responsibilities**:
  - Define domain entities
  - Enforce business constraints
  - Implement domain logic
  - Provide validation rules
  - Maintain invariants

**Key Modules**:

- `src/components/metadata.rs` - Component metadata models
- `src/components/language_filter.rs` - Language filtering logic
- `src/templates/mod.rs` - Template models
- `src/plans/structures.rs` - Plan data structures
- `src/validator/size.rs` - Size validation logic

#### Infrastructure Layer

- **Location**: `src/parser/`, `src/config/`, `src/core/`
- **Purpose**: External dependencies and utilities
- **Responsibilities**:
  - File system operations
  - YAML and Markdown parsing
  - Configuration management
  - Error handling infrastructure
  - Logging and diagnostics

**Key Modules**:

- `src/parser/mod.rs` - Parsing infrastructure
- `src/parser/agents.rs` - AGENTS.md parsing
- `src/config/project.rs` - Project configuration
- `src/error.rs` - Error types and handling

## Component System Architecture

### Component Structure

Components are the fundamental unit of content in xzagentz. Each component consists of:

1. **YAML Frontmatter**: Structured metadata
2. **Universal Content**: Language-agnostic documentation
3. **Language-Specific Sections**: Conditional content per language

```
┌─────────────────────────────────────────┐
│         YAML Frontmatter                │
│  - name, category, version              │
│  - languages supported                  │
│  - section definitions                  │
│  - tier (for tools)                     │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│       Universal Content                 │
│  - Core principles                      │
│  - Language-agnostic guidelines         │
│  - General best practices               │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│    Language-Specific Sections           │
│  <!-- LANG:rust -->                     │
│    Rust-specific content                │
│  <!-- /LANG:rust -->                    │
│  <!-- LANG:python -->                   │
│    Python-specific content              │
│  <!-- /LANG:python -->                  │
└─────────────────────────────────────────┘
```

### Component Categories

| Category      | Purpose                           | Size Limit    | Count |
| ------------- | --------------------------------- | ------------- | ----- |
| **core**      | Fundamental development practices | 500 lines     | ~5    |
| **general**   | Project management workflows      | 800 lines     | ~8    |
| **languages** | Language-specific features        | 600 lines     | ~4    |
| **tools**     | Tool-specific guidance            | 300-800 lines | ~12   |

### Component Metadata Model

```rust
pub struct ComponentMetadata {
    pub name: String,
    pub category: String,
    pub version: String,
    pub tier: Option<String>,
    pub description: String,
    pub languages: Vec<String>,
    pub sections: Vec<SectionDefinition>,
}

pub struct SectionDefinition {
    pub id: String,
    pub language_specific: bool,
    pub required: bool,
}
```

## Parsing Architecture

### Frontmatter Parser

**Location**: `src/parser/`

**Purpose**: Extract and validate YAML frontmatter from component files

**Process**:

1. Locate frontmatter delimiters (`---`)
2. Extract YAML content
3. Parse with `serde_yaml`
4. Validate required fields
5. Return structured metadata

```rust
pub fn parse_frontmatter(content: &str) -> Result<ComponentMetadata> {
    // Extract YAML between --- markers
    // Parse with serde_yaml
    // Validate required fields
    // Return ComponentMetadata
}
```

### Language Marker Parser

**Location**: `src/components/language_filter.rs`

**Purpose**: Extract language-specific sections from markdown content

**Marker Syntax**:

- Opening: `<!-- LANG:language_name -->`
- Closing: `<!-- /LANG:language_name -->`
- Universal: `<!-- LANG:* -->`

**Supported Languages**:

- `rust`
- `python`
- `golang`
- `typescript`
- `bash`

**Process**:

1. Scan content for language markers
2. Track nesting and matching
3. Extract content by language
4. Validate marker balance
5. Return language-specific sections

## Rendering Architecture

### Component Renderer

**Location**: `src/components/renderer.rs`

**Purpose**: Generate language-specific output from components

**Rendering Pipeline**:

```
Input Component
      ↓
Parse Frontmatter
      ↓
Extract Sections
      ↓
Filter by Language ──→ (Match target language)
      ↓
Assemble Output
      ↓
Validate Size
      ↓
Return Rendered Content
```

**Rendering Rules**:

1. Include all universal content (no markers or `LANG:*`)
2. Include sections matching target language exactly
3. Include `LANG:*` sections if no language-specific match
4. Exclude sections for other languages
5. Validate total size within limits

### Template Renderer

**Location**: `src/templates/renderer.rs`

**Purpose**: Generate complete AGENTS.md files from templates

**Template Syntax**:

- Component placeholders: `{{component:name}}`
- Conditional sections: `{{#if condition}}...{{/if}}`
- Variable substitution: `{{variable}}`

**Process**:

1. Load template file
2. Parse placeholder syntax
3. Load referenced components
4. Render components for target language
5. Substitute into template
6. Validate final output
7. Write to destination

## Validation Architecture

### Size Validator

**Location**: `src/validator/size.rs`

**Purpose**: Enforce component size limits programmatically

**Validation Model**:

```rust
pub struct SizeValidator {
    limits: SizeLimits,
}

pub struct SizeLimits {
    pub core_max: usize,              // 500
    pub general_max: usize,           // 800
    pub language_max: usize,          // 600
    pub tool_essential_max: usize,    // 300
    pub tool_comprehensive_max: usize, // 800
    pub total_max: usize,             // 10000
    pub warn_threshold: f64,          // 0.8
}

pub struct SizeValidation {
    pub name: String,
    pub category: String,
    pub tier: Option<String>,
    pub actual_lines: usize,
    pub max_lines: usize,
    pub is_valid: bool,
    pub is_warning: bool,
}
```

**Validation Process**:

1. Count non-empty lines
2. Determine category limit
3. Apply tier modifier (if tool)
4. Check against limit
5. Calculate warning threshold
6. Return validation result

**Warning Thresholds**:

- Warn at 80% of limit
- Error at 100% of limit
- Allow passing with warnings

### Component Validator

**Location**: `src/components/validator.rs`

**Purpose**: Validate component structure and content

**Validations**:

1. **Frontmatter**: YAML syntax, required fields
2. **Language Markers**: Balance, syntax, valid languages
3. **Section Definitions**: Match content sections
4. **Size Limits**: Per-category and total
5. **Version Format**: Semantic versioning
6. **Category**: Valid category names

### Template Validator

**Location**: `src/templates/validator.rs`

**Purpose**: Validate template syntax and component references

**Validations**:

1. **Placeholder Syntax**: Correct format
2. **Component References**: Components exist
3. **Required Components**: All required present
4. **Component Order**: Logical ordering
5. **Size Projections**: Estimated output size

## Prompt Generation Architecture

### Context Builder

**Location**: `src/prompts/context.rs`

**Purpose**: Build context for AI prompt generation

**Context Sources**:

1. Project configuration
2. Plan structure
3. Component metadata
4. File contents
5. Validation results

**Context Model**:

```rust
pub struct PromptContext {
    pub project_type: String,
    pub language: String,
    pub phase: String,
    pub tasks: Vec<String>,
    pub components: Vec<String>,
    pub constraints: Vec<String>,
}
```

### Prompt Generator

**Location**: `src/prompts/generator.rs`

**Purpose**: Generate structured AI prompts from plans

**Generation Process**:

1. Load implementation plan
2. Parse phase structure
3. Extract deliverables and tasks
4. Build context with components
5. Apply prompt template
6. Format with constraints
7. Return complete prompt

**Prompt Structure**:

```
Phase: [Phase Name]
Objective: [Phase objective]
Context: [Project and component info]
Tasks: [Enumerated tasks]
Deliverables: [Expected outputs]
Constraints: [Size limits, quality gates]
Acceptance Criteria: [Validation requirements]
```

## Configuration Architecture

### Project Configuration

**Location**: `src/config/project.rs`

**Format**: YAML (`.yaml` extension required)

**Configuration Schema**:

```yaml
project:
  type: cli|library|service|docs
  language: rust|python|golang|typescript|bash

components:
  tools:
    git: essential|comprehensive|none
    docker: essential|comprehensive|none
  include:
    - pattern1
    - pattern2
  exclude:
    - pattern3

limits:
  max_total_lines: 10000
  warn_at_lines: 8000
  per_component_max:
    core: 500
    general: 800

rendering:
  strict_language: true
  fallback_to_agnostic: true
  include_metadata: false
```

**Configuration Loading**:

1. Search for `config/project.yaml`
2. Parse YAML structure
3. Apply defaults for missing values
4. Validate configuration
5. Return typed configuration object

## Error Handling Architecture

### Error Types

**Location**: `src/error.rs`

**Error Hierarchy**:

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing frontmatter in component")]
    MissingFrontmatter,

    #[error("Invalid marker at line {line}: {marker}")]
    InvalidMarker { line: usize, marker: String },

    #[error("Missing language content for {language} in section {section}")]
    MissingLanguageContent { language: String, section: String },

    #[error("Size limit exceeded for '{name}': {actual} lines > {limit} lines")]
    SizeExceeded { name: String, actual: usize, limit: usize },

    #[error("Unclosed language section")]
    UnclosedSection,

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Error Handling Strategy**:

1. Use `Result<T, Error>` for all fallible operations
2. Propagate errors with `?` operator
3. Add context with `.map_err()`
4. Provide descriptive error messages
5. Include relevant context (line numbers, paths, etc.)

## Data Flow

### Component Loading Flow

```
User Request (language)
    ↓
Load Component Files
    ↓
Parse Frontmatter
    ↓
Validate Structure
    ↓
Parse Language Sections
    ↓
Filter by Target Language
    ↓
Validate Size
    ↓
Cache Result
    ↓
Return Rendered Component
```

### Template Rendering Flow

```
Load Template
    ↓
Parse Placeholders
    ↓
Load Referenced Components
    ↓
Render Each Component (language-specific)
    ↓
Substitute into Template
    ↓
Validate Total Size
    ↓
Apply Formatting
    ↓
Write Output File
```

### Validation Flow

```
Input Component/Template
    ↓
Validate Frontmatter
    ↓
Validate Language Markers
    ↓
Validate Size Limits
    ↓
Validate Section Definitions
    ↓
Generate Validation Report
    ↓
Return Success/Failure with Details
```

## CI/CD Architecture

### Validation Pipeline

**Location**: `.github/workflows/component_validation.yaml`

**Pipeline Stages**:

```
1. Checkout Code
    ↓
2. Setup Rust Toolchain
    ↓
3. Install Dependencies
    ↓
4. Format Check (cargo fmt)
    ↓
5. Compilation Check (cargo check)
    ↓
6. Linting (cargo clippy)
    ↓
7. Run Tests (cargo test)
    ↓
8. Validate Frontmatter
    ↓
9. Validate Language Markers
    ↓
10. Validate Size Limits
    ↓
11. Generate Size Report
    ↓
12. Upload Artifacts
```

**Quality Gates**:

- All stages must pass
- Zero clippy warnings (`-D warnings`)
- Test coverage >80%
- Size limits enforced
- No unclosed markers

## Module Organization

### Source Tree Structure

```
src/
├── cli/                    # Command-line interface
│   ├── mod.rs
│   ├── create.rs
│   ├── update.rs
│   ├── add.rs
│   ├── validate.rs
│   ├── list.rs
│   └── prompt.rs
├── components/             # Component system
│   ├── mod.rs
│   ├── loader.rs
│   ├── renderer.rs
│   ├── metadata.rs
│   ├── validator.rs
│   └── language_filter.rs
├── templates/              # Template system
│   ├── mod.rs
│   ├── loader.rs
│   ├── renderer.rs
│   └── validator.rs
├── prompts/                # Prompt generation
│   ├── mod.rs
│   ├── generator.rs
│   ├── context.rs
│   ├── progress.rs
│   └── template.rs
├── plans/                  # Plan management
│   ├── mod.rs
│   ├── parser.rs
│   ├── structures.rs
│   └── embedded.rs
├── validator/              # Validation system
│   ├── mod.rs
│   └── size.rs
├── parser/                 # Parsing utilities
│   ├── mod.rs
│   └── agents.rs
├── config/                 # Configuration
│   ├── mod.rs
│   └── project.rs
├── core/                   # Core utilities
│   └── mod.rs
├── error.rs                # Error types
├── lib.rs                  # Library root
└── main.rs                 # CLI entry point
```

### Dependency Graph

```
main.rs
  ↓
cli/*
  ↓
templates/renderer, prompts/generator
  ↓
components/renderer, validator/size
  ↓
components/loader, components/validator
  ↓
parser/*, config/project
  ↓
error.rs
```

## Design Patterns

### Builder Pattern

Used for complex object construction:

```rust
let validator = SizeValidator::builder()
    .core_max(500)
    .general_max(800)
    .warn_threshold(0.8)
    .build();
```

### Strategy Pattern

Used for language-specific rendering:

```rust
trait LanguageRenderer {
    fn render(&self, content: &str, language: &str) -> Result<String>;
}

impl LanguageRenderer for ComponentRenderer {
    fn render(&self, content: &str, language: &str) -> Result<String> {
        // Language-specific rendering logic
    }
}
```

### Repository Pattern

Used for component loading and caching:

```rust
pub struct ComponentLoader {
    cache: HashMap<String, Component>,
    base_path: PathBuf,
}

impl ComponentLoader {
    pub fn load(&mut self, name: &str) -> Result<&Component> {
        if !self.cache.contains_key(name) {
            let component = self.load_from_disk(name)?;
            self.cache.insert(name.to_string(), component);
        }
        Ok(&self.cache[name])
    }
}
```

### Factory Pattern

Used for error construction:

```rust
impl Error {
    pub fn size_exceeded(name: impl Into<String>, actual: usize, limit: usize) -> Self {
        Error::SizeExceeded {
            name: name.into(),
            actual,
            limit,
        }
    }
}
```

## Performance Considerations

### Caching

- **Component Cache**: Loaded components cached in memory
- **Parse Cache**: Parsed frontmatter cached per component
- **Validation Cache**: Validation results cached during session

### Lazy Loading

- Components loaded on-demand
- Templates parsed when first accessed
- Configuration loaded once at startup

### Incremental Validation

- Validate only changed components
- Skip validation for cached, unchanged components
- Fast-path for previously validated content

## Security Considerations

### Input Validation

- Sanitize file paths to prevent directory traversal
- Validate YAML structure before parsing
- Limit component size to prevent resource exhaustion
- Validate language markers to prevent injection

### Safe Defaults

- Default to strict validation
- Fail closed on errors
- Require explicit configuration overrides
- Validate all user inputs

## Extensibility

### Adding New Languages

1. Add language identifier to supported languages list
2. Create language-specific component content
3. Update renderer to handle new language
4. Add validation for language-specific syntax
5. Update documentation

### Adding New Component Categories

1. Define category in configuration schema
2. Add size limits for new category
3. Update validator with category rules
4. Create directory structure
5. Document category purpose and guidelines

### Adding New Commands

1. Create command module in `src/cli/`
2. Implement command logic
3. Register command in CLI router
4. Add tests for command behavior
5. Document command usage

## Testing Strategy

### Unit Tests

- Test individual functions in isolation
- Mock external dependencies
- Cover edge cases and error conditions
- Achieve >80% code coverage

### Integration Tests

- Test component system end-to-end
- Validate real component files
- Test rendering with actual templates
- Verify validation results

### Test Organization

```
tests/
├── core_components_test.rs
├── general_components_test.rs
├── tool_component_tiering_test.rs
├── size_validation_test.rs
└── integration_test.rs
```

## Deployment Architecture

### Build Artifacts

- Single binary executable
- Embedded default components
- Embedded templates
- Configuration examples

### Distribution

- Cargo install from crates.io
- GitHub releases with binaries
- Docker image (optional)
- Platform-specific packages

## Version Strategy

### Semantic Versioning

- **MAJOR**: Breaking changes to component format or API
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes and documentation

### Compatibility

- Maintain backward compatibility within major versions
- Provide migration tools for major version upgrades
- Document breaking changes in CHANGELOG

## Future Architecture Considerations

### Planned Enhancements

1. **Plugin System**: Extensible rendering and validation
2. **Remote Components**: Load components from repositories
3. **Component Registry**: Centralized component discovery
4. **Live Preview**: Real-time rendering feedback
5. **Language Server**: IDE integration support

### Scalability

- Support for large component libraries (1000+ components)
- Parallel component processing
- Incremental rendering
- Distributed validation

## Related Documentation

- **Implementation Plan**: `docs/explanation/implementation/language_agnostic_component_system_implementation_plan.md`
- **Configuration Reference**: `docs/reference/component_configuration.md`
- **How-To Guide**: `docs/how_to/authoring_components.md`
- **Troubleshooting**: `docs/reference/troubleshooting.md`

---

**Document Version**: 2.0.0
**Last Updated**: 2024-12-19
**Status**: Current
