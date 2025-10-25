# xzagentz Implementation Plan

## Overview

This document provides a phased implementation plan for the xzagentz project, a component-based system for building project-specific AGENTS.md files using Rust. The implementation is broken down into manageable phases, each with clear deliverables, acceptance criteria, and testing requirements.

## Project Summary

**Name**: xzagentz

**Type**: Command-line tool (Rust binary)

**Purpose**: Generate, update, and manage AGENTS.md files using a modular component system

**Key Features**:

- Component-based markdown generation
- TOML template system with placeholder replacement
- CLI commands: create, update, add, list, validate
- Interactive mode for guided generation
- Compliance verification
- Prompt generation for implementation workflow

## Architecture Overview

### Directory Structure

```text
xzagentz/
├── Cargo.toml                        # Rust project manifest
├── README.md                         # Project overview
├── AGENTS.md                         # Development guidelines
├── src/
│   ├── main.rs                       # CLI entry point
│   ├── lib.rs                        # Library root
│   ├── cli/
│   │   ├── mod.rs                    # CLI module
│   │   ├── commands.rs               # Command implementations
│   │   └── args.rs                   # Argument parsing
│   ├── components/
│   │   ├── mod.rs                    # Component module
│   │   ├── loader.rs                 # Component loading
│   │   └── validator.rs              # Component validation
│   ├── templates/
│   │   ├── mod.rs                    # Template module
│   │   ├── embedded.rs               # Embedded template definitions
│   │   ├── loader.rs                 # Priority-based template loading
│   │   ├── metadata.rs               # Template metadata
│   │   ├── parser.rs                 # TOML parsing
│   │   └── renderer.rs               # Placeholder replacement
│   ├── config/
│   │   ├── mod.rs                    # Config module
│   │   ├── loader.rs                 # Config file loading
│   │   └── project.rs                # Project configuration
│   ├── agents/
│   │   ├── mod.rs                    # Agents module
│   │   ├── create.rs                 # Create logic
│   │   ├── update.rs                 # Update logic
│   │   ├── add.rs                    # Add logic
│   │   └── validate.rs               # Validate logic
│   ├── plans/
│   │   ├── mod.rs                    # Plans module
│   │   ├── structures.rs             # Plan data structures
│   │   ├── parser.rs                 # Plan parser
│   │   ├── architecture.rs           # Architecture plan manager
│   │   └── implementation.rs         # Implementation plan manager
│   └── prompts/
│       ├── mod.rs                    # Prompt module
│       ├── generator.rs              # Prompt generation
│       ├── template.rs               # Prompt templates
│       ├── progress.rs               # Progress tracking
│       └── compliance.rs             # Compliance verification
├── components/
│   ├── core/
│   │   ├── header.md
│   │   ├── critical_rules.md
│   │   └── learning_resources.md
│   ├── languages/
│   │   ├── python.md
│   │   ├── rust.md
│   │   ├── golang.md
│   │   ├── typescript.md
│   │   └── bash.md
│   ├── tools/
│   │   ├── git.md
│   │   ├── npm.md
│   │   └── markdown.md
│   └── general/
│       ├── development.md
│       ├── testing.md
│       └── documentation.md
├── templates/
│   ├── projects/                     # Project configuration templates (TOML)
│   │   ├── python_cli.toml
│   │   ├── rust_binary.toml
│   │   ├── golang_service.toml
│   │   ├── npm_webapp.toml
│   │   └── bash_scripts.toml
│   ├── plans/                        # Plan templates (embedded in binary)
│   │   ├── architecture_plan_rust_binary.md
│   │   ├── architecture_plan_web_service.md
│   │   ├── architecture_plan_cli.md
│   │   ├── architecture_plan_library.md
│   │   └── implementation_plan.md
│   └── prompts/                      # Prompt templates (embedded in binary)
│       ├── phase_prompt.md
│       ├── section_prompt.md
│       └── task_prompt.md
├── tests/
│   ├── integration/
│   │   ├── create_tests.rs
│   │   ├── update_tests.rs
│   │   ├── validate_tests.rs
│   │   └── config_tests.rs
│   └── fixtures/
│       ├── test_components/
│       ├── test_templates/
│       └── test_configs/
├── docs/
│   ├── explanations/
│   │   ├── implementation_plan.md    # This document
│   │   ├── architecture.md
│   │   ├── component_system.md
│   │   ├── template_system_design.md
│   │   └── project_config_system.md
│   ├── how_to/
│   │   ├── create_components.md
│   │   ├── create_templates.md
│   │   └── customize_templates.md
│   └── reference/
│       ├── cli_reference.md
│       ├── component_format.md
│       └── config_format.md
└── examples/
    ├── python_cli_example.md
    ├── rust_binary_example.md
    └── golang_service_example.md

# User Project Structure (after xzagentz is used)
user-project/
├── .xzagentz.toml                    # Project config (saves user choices)
├── .implementation_progress          # Progress tracking (TOML)
├── AGENTS.md                         # Generated agents file
├── README.md                         # User's readme
├── plans/                            # Generated plans (optional)
│   ├── architecture_plan.md
│   └── implementation_plan.md
└── prompts/                          # Generated prompts (optional)
    ├── prompt_1_1.md
    ├── prompt_1_2.md
    └── ...
```

### Technology Stack

- **Language**: Rust (stable)
- **CLI Framework**: clap (v4.x)
- **TOML Parsing**: toml (v0.8.x)
- **Error Handling**: thiserror, anyhow
- **Testing**: Built-in Rust testing + integration tests
- **File Operations**: std::fs, walkdir

### Design Principles

1. **Modularity**: Each component is self-contained
2. **Extensibility**: Easy to add new components and templates
3. **Validation**: Strong validation at every step
4. **Error Handling**: Clear, actionable error messages
5. **Testing**: Comprehensive unit and integration tests
6. **Documentation**: Complete inline and external documentation

## Phase 1: Project Foundation and Core Structure

**Duration**: Week 1

**Goal**: Establish project structure, dependencies, and core types

### 1.1 Project Initialization

**Tasks**:

1. Create Cargo project with workspace structure
2. Configure dependencies in Cargo.toml
3. Set up directory structure
4. Create initial module files with documentation
5. Configure CI/CD basics

**Deliverables**:

- `Cargo.toml` with all dependencies
- `src/lib.rs` with module declarations
- `src/main.rs` with basic CLI structure
- Module files: `cli/mod.rs`, `components/mod.rs`, `templates/mod.rs`, `agents/mod.rs`, `prompts/mod.rs`
- `.github/workflows/rust.yaml` (basic CI)

**Acceptance Criteria**:

- `cargo build` completes successfully
- `cargo test` runs (even with no tests yet)
- `cargo clippy` shows zero warnings
- `cargo fmt --check` passes
- Directory structure matches architecture

**Testing**:

- Smoke test: project compiles
- Module imports work correctly

### 1.2 Core Data Structures

**Tasks**:

1. Define Component struct and ComponentType enum
2. Define Template struct and TemplateConfig
3. Define ProjectMetadata struct
4. Implement Display and Debug traits
5. Create comprehensive unit tests

**Deliverables**:

- `src/components/mod.rs` with Component types
- `src/templates/mod.rs` with Template types
- `src/lib.rs` with shared types
- Unit tests for each type

**Acceptance Criteria**:

- All types are well-documented with doc comments
- All types implement necessary traits
- All types have unit tests
- Test coverage exceeds 80 percent

**Testing**:

```rust
#[test]
fn test_component_type_from_path()
#[test]
fn test_template_config_default()
#[test]
fn test_project_metadata_validation()
```

### 1.3 Error Handling Framework

**Tasks**:

1. Create custom error types using thiserror
2. Define error variants for each module
3. Implement From traits for error conversions
4. Add error context helpers
5. Create error handling tests

**Deliverables**:

- `src/error.rs` with XzagentzError enum
- Error types for: ComponentError, TemplateError, ValidationError, IoError
- Error conversion implementations
- Error formatting with user-friendly messages

**Acceptance Criteria**:

- All error types implement Error trait
- Error messages are descriptive and actionable
- Error chain preserves context
- All error paths tested

**Testing**:

```rust
#[test]
fn test_component_not_found_error()
#[test]
fn test_template_parse_error()
#[test]
fn test_error_chain_preserves_context()
```

## Phase 2: Component System

**Duration**: Week 2

**Goal**: Implement component loading, parsing, and validation

### 2.1 Component File Structure

**Tasks**:

1. Create markdown component files in components/ directory
2. Implement standard sections for each component type
3. Define placeholder syntax and patterns
4. Document component format specification
5. Create example components

**Deliverables**:

- `components/core/header.md`
- `components/core/critical_rules.md`
- `components/core/learning_resources.md`
- `components/languages/rust.md`
- `components/languages/python.md`
- `components/languages/golang.md`
- `components/tools/git.md`
- `components/tools/markdown.md`
- `components/general/development.md`
- `components/general/testing.md`
- `components/general/documentation.md`
- `docs/reference/component_format.md`

**Acceptance Criteria**:

- All component files use lowercase snake_case names
- All code blocks have language identifiers
- All placeholders follow @placeholder-name@ format
- No emojis in component files
- Component documentation is complete

**Testing**:

- Manual review of component files
- Validation script to check format

### 2.2 Component Loader

**Tasks**:

1. Implement ComponentLoader struct
2. Create methods to load components from filesystem
3. Implement component discovery and indexing
4. Add caching for loaded components
5. Handle component not found errors
6. Write comprehensive tests

**Deliverables**:

- `src/components/loader.rs`
- ComponentLoader implementation
- Component caching mechanism
- Error handling for missing components

**Acceptance Criteria**:

- Can load components by type and name
- Handles missing components gracefully
- Caches loaded components for performance
- Returns helpful error messages
- All code paths tested

**Testing**:

```rust
#[test]
fn test_load_component_success()
#[test]
fn test_load_component_not_found()
#[test]
fn test_load_all_components_in_category()
#[test]
fn test_component_caching()
```

### 2.3 Component Validator

**Tasks**:

1. Implement ComponentValidator struct
2. Create validation rules for component format
3. Validate markdown structure
4. Validate placeholder syntax
5. Check for common issues (emojis, code blocks, etc.)
6. Write validation tests

**Deliverables**:

- `src/components/validator.rs`
- ComponentValidator implementation
- Validation rule engine
- Detailed validation reports

**Acceptance Criteria**:

- Validates markdown heading structure
- Checks for language identifiers in code blocks
- Detects emoji usage
- Validates placeholder format
- Returns actionable validation errors
- All validation rules tested

**Testing**:

```rust
#[test]
fn test_validate_component_structure()
#[test]
fn test_detect_missing_language_identifier()
#[test]
fn test_detect_emoji_usage()
#[test]
fn test_validate_placeholder_format()
```

## Phase 3: Template System

**Duration**: Week 3

**Goal**: Implement TOML template parsing and placeholder replacement

### 3.1 Template Parser

**Tasks**:

1. Implement TemplateParser struct
2. Parse TOML template files
3. Extract project metadata and component lists
4. Validate template structure
5. Handle parsing errors gracefully
6. Write parser tests

**Deliverables**:

- `src/templates/parser.rs`
- TemplateParser implementation
- TOML deserialization logic
- Template validation

**Acceptance Criteria**:

- Parses valid TOML templates correctly
- Extracts all project fields
- Extracts component lists by category
- Returns clear errors for invalid TOML
- Handles missing optional fields
- All parsing scenarios tested

**Testing**:

```rust
#[test]
fn test_parse_valid_template()
#[test]
fn test_parse_invalid_toml()
#[test]
fn test_parse_missing_required_fields()
#[test]
fn test_parse_optional_fields()
```

### 3.2 Template Files

**Tasks**:

1. Create TOML template files for each project type
2. Define project metadata sections
3. Specify component selections
4. Document template format
5. Create example templates

**Deliverables**:

- `templates/python_cli.toml`
- `templates/rust_binary.toml`
- `templates/golang_service.toml`
- `templates/npm_webapp.toml`
- `templates/bash_scripts.toml`
- `docs/reference/template_format.md`

**Acceptance Criteria**:

- All template files use .yaml extension (NOT .yml)
- All templates have valid TOML syntax
- All templates include required fields
- Templates cover common project types
- Template documentation is complete

**Testing**:

- Parse all template files successfully
- Validate template structure

### 3.3 Placeholder Renderer

**Tasks**:

1. Implement PlaceholderRenderer struct
2. Create placeholder detection logic
3. Implement replacement algorithm
4. Handle missing placeholder values
5. Support nested placeholders
6. Write renderer tests

**Deliverables**:

- `src/templates/renderer.rs`
- PlaceholderRenderer implementation
- Placeholder replacement engine
- Error handling for undefined placeholders

**Acceptance Criteria**:

- Detects all @placeholder@ patterns
- Replaces placeholders with provided values
- Handles missing values appropriately
- Preserves markdown formatting
- Supports case transformations (snake_case, kebab-case)
- All replacement scenarios tested

**Testing**:

```rust
#[test]
fn test_replace_simple_placeholder()
#[test]
fn test_replace_multiple_placeholders()
#[test]
fn test_handle_missing_placeholder()
#[test]
fn test_case_transformation()
```

## Phase 4: CLI Foundation

**Duration**: Week 4

**Goal**: Implement CLI argument parsing and basic commands

### 4.1 CLI Argument Parser

**Tasks**:

1. Define CLI structure with clap
2. Create command enums and argument structs
3. Implement global options
4. Add help text and examples
5. Write argument parsing tests

**Deliverables**:

- `src/cli/args.rs`
- Cli struct with clap derives
- Command enums: Create, Update, Add, List, Validate
- Argument validation
- Help text for all commands

**Acceptance Criteria**:

- All commands have clear help text
- Arguments are properly validated
- Provides usage examples
- Handles invalid arguments gracefully
- Help text follows conventions

**Testing**:

```rust
#[test]
fn test_parse_create_command()
#[test]
fn test_parse_with_flags()
#[test]
fn test_invalid_arguments()
#[test]
fn test_help_text_generated()
```

### 4.2 List Command

**Tasks**:

1. Implement list command logic
2. Discover available components
3. Discover available templates
4. Format output (plain, table, JSON)
5. Add filtering options
6. Write list command tests

**Deliverables**:

- `src/agents/mod.rs` with list_components()
- Component discovery logic
- Template discovery logic
- Output formatting options
- Integration tests

**Acceptance Criteria**:

- Lists all available components by category
- Lists all available templates
- Supports multiple output formats
- Output is well-formatted and readable
- Handles empty directories gracefully

**Testing**:

```rust
#[test]
fn test_list_components()
#[test]
fn test_list_templates()
#[test]
fn test_list_with_filter()
#[test]
fn test_json_output_format()
```

### 4.3 Validate Command

**Tasks**:

1. Implement validate command logic
2. Validate existing AGENTS.md file
3. Check for common issues
4. Generate validation report
5. Support strict mode
6. Write validate command tests

**Deliverables**:

- `src/agents/validate.rs`
- AGENTS.md validation logic
- Validation report generation
- Exit codes for CI/CD integration

**Acceptance Criteria**:

- Validates markdown structure
- Checks for required sections
- Detects AGENTS.md rule violations
- Generates detailed report
- Returns appropriate exit codes
- All validation rules tested

**Testing**:

```rust
#[test]
fn test_validate_valid_file()
#[test]
fn test_validate_invalid_file()
#[test]
fn test_validate_missing_sections()
#[test]
fn test_validation_report_format()
```

## Phase 5: Create Command Implementation

**Duration**: Week 5-6

**Goal**: Implement the core create command with all its features

### 5.1 Basic Create Command

**Tasks**:

1. Implement create command logic
2. Load components based on flags
3. Combine components in correct order
4. Write output to file
5. Handle file overwrite protection
6. Write create command tests

**Deliverables**:

- `src/agents/create.rs`
- Component loading and ordering logic
- File writing with overwrite protection
- Progress output

**Acceptance Criteria**:

- Creates AGENTS.md from component flags
- Maintains correct section order
- Prevents accidental overwrites
- Shows progress information
- Handles errors gracefully
- All scenarios tested

**Testing**:

```rust
#[test]
fn test_create_basic_agents_file()
#[test]
fn test_create_prevents_overwrite()
#[test]
fn test_create_with_force_flag()
#[test]
fn test_create_component_ordering()
```

### 5.2 Template-Based Creation

**Tasks**:

1. Implement template loading
2. Parse template configuration
3. Load components from template spec
4. Replace placeholders
5. Write integration tests

**Deliverables**:

- Template-based creation in create.rs
- Template and component integration
- Placeholder replacement integration

**Acceptance Criteria**:

- Loads and parses template files
- Loads correct components from template
- Replaces all placeholders
- Generates complete AGENTS.md
- Template errors are clear

**Testing**:

```rust
#[test]
fn test_create_from_template()
#[test]
fn test_template_with_placeholders()
#[test]
fn test_invalid_template_handling()
```

### 5.3 Interactive Mode

**Tasks**:

1. Implement interactive prompt system
2. Ask for project metadata
3. Allow component selection
4. Preview generated content
5. Confirm before writing
6. Write interactive tests

**Deliverables**:

- Interactive mode in create.rs
- User prompts for metadata
- Component selection UI
- Preview functionality

**Acceptance Criteria**:

- Prompts for all required metadata
- Provides component selection menu
- Shows preview before writing
- Allows user to cancel
- Handles invalid input gracefully

**Testing**:

```rust
#[test]
fn test_interactive_metadata_collection()
#[test]
fn test_interactive_component_selection()
#[test]
fn test_interactive_preview()
```

### 5.4 README.md Context Parsing

**Tasks**:

1. Implement README.md parser
2. Extract project metadata from README
3. Infer project type and language
4. Pre-fill interactive prompts
5. Write parser tests

**Deliverables**:

- `src/agents/readme_parser.rs`
- Metadata extraction logic
- Project type inference
- Language detection

**Acceptance Criteria**:

- Extracts project name and description
- Detects primary language from content
- Infers project type from README
- Provides sensible defaults
- Handles malformed README files

**Testing**:

```rust
#[test]
fn test_parse_readme_metadata()
#[test]
fn test_detect_language_from_readme()
#[test]
fn test_infer_project_type()
#[test]
fn test_handle_missing_readme()
```

## Phase 6: Update and Add Commands

**Duration**: Week 7

**Goal**: Implement update and add commands for modifying existing files

### 6.1 AGENTS.md Parser

**Tasks**:

1. Implement AGENTS.md parser
2. Extract existing sections
3. Identify section boundaries
4. Parse section metadata
5. Write parser tests

**Deliverables**:

- `src/agents/parser.rs`
- Section extraction logic
- Section boundary detection
- Parsed section structure

**Acceptance Criteria**:

- Parses existing AGENTS.md files
- Extracts all sections correctly
- Identifies section types
- Handles malformed sections
- Preserves section content

**Testing**:

```rust
#[test]
fn test_parse_agents_file()
#[test]
fn test_extract_sections()
#[test]
fn test_identify_section_boundaries()
#[test]
fn test_handle_malformed_sections()
```

### 6.2 Update Command

**Tasks**:

1. Implement update command logic
2. Parse existing AGENTS.md
3. Replace specified section
4. Maintain file structure
5. Create backup before updating
6. Write update tests

**Deliverables**:

- `src/agents/update.rs`
- Section replacement logic
- Backup creation
- File structure preservation

**Acceptance Criteria**:

- Updates specified section only
- Preserves other sections unchanged
- Creates backup file
- Maintains markdown formatting
- Handles missing sections gracefully

**Testing**:

```rust
#[test]
fn test_update_existing_section()
#[test]
fn test_update_creates_backup()
#[test]
fn test_update_preserves_other_sections()
#[test]
fn test_update_nonexistent_section()
```

### 6.3 Add Command

**Tasks**:

1. Implement add command logic
2. Parse existing AGENTS.md
3. Insert new section at correct position
4. Maintain section ordering
5. Prevent duplicate sections
6. Write add tests

**Deliverables**:

- `src/agents/add.rs`
- Section insertion logic
- Position calculation
- Duplicate detection

**Acceptance Criteria**:

- Adds new section to existing file
- Inserts at correct position
- Prevents duplicate sections
- Maintains section order
- Creates backup before adding

**Testing**:

```rust
#[test]
fn test_add_new_section()
#[test]
fn test_add_maintains_order()
#[test]
fn test_add_prevents_duplicates()
#[test]
fn test_add_to_empty_file()
```

### 6.4 Project Configuration Management

**Tasks**:

1. Define project configuration structure (TOML)
2. Implement config file save/load functionality
3. Store user choices in `.xzagentz.toml`
4. Support config file updates
5. Write config management tests

**Deliverables**:

- `src/config/mod.rs` - Config module declaration
- `src/config/project.rs` - Project configuration data structures
- `src/config/loader.rs` - Config file loading and saving
- `.xzagentz.toml` schema definition
- Config validation logic
- Config migration support (version updates)

**Acceptance Criteria**:

- Config saved to `.xzagentz.toml` in project root
- Config includes project metadata (name, type, language)
- Config includes selected components and template choices
- Config includes custom placeholder values
- Config can be reloaded to recreate or update AGENTS.md
- Config file uses `.toml` extension (NOT `.yml`)
- Config validates on load with clear error messages
- Supports incremental updates (add/remove components)

**TOML Schema**:

```toml
[project]
name = "my-project"
type = "rust_binary"
language = "rust"
description = "A sample Rust binary project"
created_at = "2024-01-15T10:30:00Z"
updated_at = "2024-01-15T10:30:00Z"

[template]
name = "rust_binary"
version = "1.0.0"

[components]
core = ["header", "critical_rules"]
languages = ["rust"]
tools = ["git", "cargo"]
general = ["development", "testing", "documentation"]

[placeholders]
project_name = "my-project"
project_type = "binary"
version = "0.1.0"
author = "John Doe"

[options]
auto_update = false
validate_on_save = true
backup_enabled = true
```

**Testing**:

```rust
#[test]
fn test_save_project_config()
#[test]
fn test_load_project_config()
#[test]
fn test_config_validation()
#[test]
fn test_config_update()
#[test]
fn test_missing_config_error()
#[test]
fn test_invalid_toml_error()
#[test]
fn test_config_migration()
```

**CLI Integration**:

```bash
# Create AGENTS.md and save config
xzagentz create --save-config

# Use existing config to recreate/update
xzagentz create --from-config

# Update config
xzagentz config set project.name "new-name"
xzagentz config add components.tools "docker"

# Show current config
xzagentz config show

# Validate config file
xzagentz config validate
```

## Phase 7: Plan Management and Prompt Generation System

**Duration**: Week 8-9 (2 weeks)

**Goal**: Implement comprehensive plan management system (architecture and implementation planning) plus prompt generation, compliance verification, and progress tracking with embedded template system

### 7.0 Template System Foundation

**Tasks**:

1. Implement embedded template system using `include_str!` macro
2. Create template loader with priority system (custom > user config > embedded)
3. Implement template export functionality
4. Add template listing and metadata
5. Integrate with CLI commands
6. Write template system tests

**Deliverables**:

- `src/templates/mod.rs` - Public template API
- `src/templates/embedded.rs` - Embedded template definitions using `include_str!`
- `src/templates/loader.rs` - Priority-based template loading
- `src/templates/metadata.rs` - Template metadata and validation
- `templates/plans/architecture_plan_rust_binary.md` - Embedded architecture template
- `templates/plans/architecture_plan_web_service.md` - Web service template
- `templates/plans/architecture_plan_cli.md` - CLI tool template
- `templates/plans/architecture_plan_library.md` - Library template
- `templates/plans/implementation_plan.md` - Implementation template
- `templates/prompts/phase_prompt.md` - Phase prompt template
- `templates/prompts/section_prompt.md` - Section prompt template
- `templates/prompts/task_prompt.md` - Task prompt template
- CLI commands: `templates list`, `templates show`, `templates export`
- Global `--template-dir` flag support

**Acceptance Criteria**:

- Templates compiled into binary using `include_str!`
- Tool works out-of-box with no external files required
- Template loader checks custom dir > user config > embedded
- User can export templates to `~/.config/xzagentz/templates/`
- User can override templates via `--template-dir` flag
- Templates include YAML frontmatter with metadata
- All template files use `.md` extension (not `.markdown`)
- All template filenames use snake_case (not CamelCase)
- Binary size increase is minimal (~50KB)
- Works completely offline

**Testing**:

```rust
#[test]
fn test_embedded_templates_exist()
#[test]
fn test_load_embedded_template()
#[test]
fn test_priority_custom_over_user_config()
#[test]
fn test_priority_user_config_over_embedded()
#[test]
fn test_export_single_template()
#[test]
fn test_export_all_templates()
#[test]
fn test_list_templates()
#[test]
fn test_template_not_found_error()
#[test]
fn test_template_metadata_parsing()
```

### 7.1 Plan Templates and Data Structures

**Tasks**:

1. Define architecture plan data structures
2. Define implementation plan data structures
3. Create architecture plan templates (Rust Binary, Web Service, CLI, etc.)
4. Create implementation plan template structure
5. Write template tests

**Deliverables**:

- `src/plans/mod.rs` - Module declaration
- `src/plans/structures.rs` - Plan data structures
- `ArchitecturePlan` struct
- `ImplementationPlan` struct
- `Phase` struct
- `Section` struct
- `PlanMetadata` struct
- Template loading integration with `TemplateLoader`

**Acceptance Criteria**:

- All data structures defined with proper types
- Plan structures support parsing from markdown
- Plan structures support serialization to TOML/JSON
- Templates loaded via `TemplateLoader` from embedded or filesystem
- Templates use snake_case filenames (e.g., `architecture_plan_rust_binary.md`)
- Templates follow markdown best practices (no emojis, proper code blocks)
- All fields properly documented with `///` doc comments
- Serde serialization/deserialization support
- Integration with template system for plan creation

**Testing**:

```rust
#[test]
fn test_architecture_plan_structure()
#[test]
fn test_implementation_plan_structure()
#[test]
fn test_template_rust_binary()
#[test]
fn test_template_web_service()
#[test]
fn test_template_serialization()
```

### 7.2 Plan Parser

**Tasks**:

1. Implement markdown parser for architecture plans
2. Implement markdown parser for implementation plans
3. Extract phase metadata (number, name, duration, goal)
4. Extract section metadata (number, name, tasks, deliverables, criteria)
5. Handle nested structures and bullet points
6. Write parser tests

**Deliverables**:

- `src/plans/parser.rs` - Plan parsers
- Architecture plan parser
- Implementation plan parser
- Section extraction logic
- Metadata extraction utilities

**Acceptance Criteria**:

- Parses architecture plans correctly
- Parses implementation plans correctly
- Extracts all phases with metadata
- Extracts all sections with deliverables
- Handles malformed markdown gracefully
- Preserves section ordering
- Returns detailed error messages

**Testing**:

```rust
#[test]
fn test_parse_architecture_plan()
#[test]
fn test_parse_implementation_plan()
#[test]
fn test_extract_phases()
#[test]
fn test_extract_sections()
#[test]
fn test_parse_deliverables()
#[test]
fn test_parse_acceptance_criteria()
#[test]
fn test_handle_malformed_markdown()
```

### 7.3 Architecture Plan Manager

**Tasks**:

1. Implement architecture plan creation from templates
2. Implement interactive architecture plan creation
3. Verify architecture plan completeness
4. Write architecture manager tests

**Deliverables**:

- `src/plans/architecture.rs` - Architecture plan manager
- `ArchitecturePlanManager` struct
- Template-based creation
- Interactive creation mode
- Completeness verification

**Acceptance Criteria**:

- Creates architecture plans from templates
- Supports interactive plan creation
- Verifies plan has all required sections
- Identifies missing components
- Returns detailed verification reports

**Testing**:

```rust
#[test]
fn test_create_from_template()
#[test]
fn test_create_interactive()
#[test]
fn test_verify_completeness()
#[test]
fn test_missing_sections_detected()
```

### 7.4 Implementation Plan Manager

**Tasks**:

1. Implement implementation plan generation from architecture plans
2. Configure generator options (phase duration, testing, etc.)
3. Verify implementation plan structure
4. Write implementation manager tests

**Deliverables**:

- `src/plans/implementation.rs` - Implementation plan manager
- `ImplementationPlanGenerator` struct
- `GeneratorConfig` struct
- Architecture-to-phases conversion
- Structure verification

**Acceptance Criteria**:

- Generates implementation plans from architecture plans
- Converts architecture layers to implementation phases
- Supports configuration of phase duration and scope
- Verifies phase dependencies and ordering
- Identifies missing deliverables
- Returns detailed verification reports

**Testing**:

```rust
#[test]
fn test_generate_from_architecture()
#[test]
fn test_architecture_to_phases()
#[test]
fn test_verify_plan_structure()
#[test]
fn test_phase_dependencies()
#[test]
fn test_configuration_options()
```

### 7.5 Prompt Generation System

**Overview**:

The Prompt Generation System converts implementation plan sections into structured prompts for AI agents. Each prompt includes context from the architecture plan, relevant AGENTS.md rules, specific tasks to complete, and validation checklists. The system supports both batch generation (all prompts at once) and interactive generation (prompt-by-prompt with preview).

**Tasks**:

1. Implement prompt template data structures with metadata
2. Create prompt template renderer with placeholder support
3. Implement prompt generator orchestrator for batch operations
4. Build interactive generation workflow with preview
5. Add overwrite protection and backup functionality
6. Support custom output directories and naming schemes
7. Integrate with embedded template system
8. Write comprehensive prompt generator tests

**Deliverables**:

- `src/prompts/mod.rs` - Public prompt API and re-exports
- `src/prompts/template.rs` - Prompt template structures and rendering
- `src/prompts/generator.rs` - Prompt generation orchestrator
- `src/prompts/context.rs` - Context extraction from plans
- `templates/prompts/phase_prompt.md` - Embedded phase-level prompt template
- `templates/prompts/section_prompt.md` - Embedded section-level prompt template
- `templates/prompts/task_prompt.md` - Embedded task-level prompt template
- CLI integration in `src/cli/prompt.rs`
- Generated prompts output to `prompts/` directory

**Data Structures**:

```rust
/// Prompt template with metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Template name (e.g., "section_prompt")
    pub name: String,
    /// Template version
    pub version: String,
    /// Template description
    pub description: String,
    /// Template content with placeholders
    pub content: String,
    /// Required context variables
    pub required_context: Vec<String>,
    /// Optional context variables
    pub optional_context: Vec<String>,
}

/// Context for generating a prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptContext {
    /// Project metadata
    pub project_name: String,
    pub project_type: String,
    pub project_description: String,

    /// Phase information
    pub phase_number: usize,
    pub phase_title: String,
    pub phase_goal: String,

    /// Section information
    pub section_number: String,
    pub section_title: String,
    pub section_tasks: Vec<String>,
    pub section_deliverables: Vec<String>,
    pub section_acceptance_criteria: Vec<String>,

    /// Architecture context
    pub architecture_overview: String,
    pub related_components: Vec<String>,
    pub dependencies: Vec<String>,

    /// AGENTS.md rules (relevant to this section)
    pub relevant_rules: Vec<String>,

    /// File paths and structure
    pub output_files: Vec<String>,
    pub test_files: Vec<String>,
}

/// Prompt generator configuration
#[derive(Debug, Clone)]
pub struct PromptGeneratorConfig {
    /// Output directory for generated prompts
    pub output_dir: PathBuf,
    /// Template directory (custom templates)
    pub template_dir: Option<PathBuf>,
    /// Overwrite existing prompts without asking
    pub force_overwrite: bool,
    /// Create backup before overwriting
    pub create_backup: bool,
    /// Prompt naming scheme
    pub naming_scheme: PromptNamingScheme,
    /// Include full architecture in context
    pub include_full_architecture: bool,
    /// Include all AGENTS.md rules or only relevant ones
    pub include_all_rules: bool,
}

/// Naming scheme for generated prompts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptNamingScheme {
    /// prompt_1_1.md, prompt_1_2.md, etc.
    PhaseSection,
    /// prompt_phase1_section1.md, etc.
    Descriptive,
    /// phase_1_project_foundation.md, etc.
    Readable,
}

/// Main prompt generator
pub struct PromptGenerator {
    config: PromptGeneratorConfig,
    templates: HashMap<String, PromptTemplate>,
    embedded_templates: EmbeddedTemplates,
}

impl PromptGenerator {
    /// Create new prompt generator with configuration
    pub fn new(config: PromptGeneratorConfig) -> Result<Self>;

    /// Generate all prompts from implementation plan
    pub fn generate_all(
        &self,
        impl_plan: &ImplementationPlan,
        arch_plan: &ArchitecturePlan,
    ) -> Result<Vec<PathBuf>>;

    /// Generate prompt for specific section
    pub fn generate_section(
        &self,
        context: &PromptContext,
    ) -> Result<PathBuf>;

    /// Interactive generation with preview
    pub fn generate_interactive(
        &self,
        impl_plan: &ImplementationPlan,
        arch_plan: &ArchitecturePlan,
    ) -> Result<Vec<PathBuf>>;

    /// Render prompt from template and context
    fn render_prompt(
        &self,
        template: &PromptTemplate,
        context: &PromptContext,
    ) -> Result<String>;

    /// Check if prompt file exists and handle overwrite
    fn handle_existing_file(&self, path: &Path) -> Result<bool>;

    /// Create backup of existing file
    fn create_backup(&self, path: &Path) -> Result<PathBuf>;

    /// Format prompt filename from context
    fn format_filename(&self, context: &PromptContext) -> String;
}
```

**Prompt Template Format**:

```markdown
# Phase {{phase_number}}: {{phase_title}}

## Section {{section_number}}: {{section_title}}

## Context

You are implementing {{section_title}} as part of the {{project_name}} project.

### Project Overview

{{project_description}}

### Architecture Context

{{architecture_overview}}

### Related Components

{{#each related_components}}

- {{this}}
  {{/each}}

### Dependencies

{{#each dependencies}}

- {{this}}
  {{/each}}

## Critical Rules from AGENTS.md

{{#each relevant_rules}}

### {{this.title}}

{{this.content}}
{{/each}}

## Tasks

Your goal is to complete the following tasks:

{{#each section_tasks}}
{{@index}}. {{this}}
{{/each}}

## Deliverables

You must create the following files:

{{#each section_deliverables}}

- {{this}}
  {{/each}}

### Expected Files

{{#each output_files}}

- `{{this}}`
  {{/each}}

### Test Files

{{#each test_files}}

- `{{this}}`
  {{/each}}

## Acceptance Criteria

Your implementation will be validated against these criteria:

{{#each section_acceptance_criteria}}

- [ ] {{this}}
      {{/each}}

## Quality Checklist

Before completing this section, verify:

- [ ] `cargo fmt --all` applied successfully
- [ ] `cargo check --all-targets --all-features` passes with zero errors
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] `cargo test --all-features` passes with >80% coverage
- [ ] All public items have doc comments with examples
- [ ] Documentation file created in `docs/explanations/`
- [ ] All filenames follow naming conventions (lowercase_with_underscores.md)
- [ ] All YAML files use `.yaml` extension (not `.yml`)
- [ ] No emojis in code or documentation

## Implementation Notes

1. Start by reading the architecture plan section
2. Create file stubs with proper module structure
3. Implement core functionality with error handling
4. Add comprehensive tests (unit, integration, doc)
5. Run quality checks incrementally
6. Create documentation file
7. Verify all acceptance criteria met

## References

- Architecture Plan: `docs/explanations/architecture_plan.md`
- AGENTS.md Rules: `AGENTS.md`
- Implementation Plan: `docs/explanations/implementation_plan.md`
- Phase {{phase_number}} Overview: See implementation plan

---

Generated by xzagentz v{{version}} on {{timestamp}}
```

**Acceptance Criteria**:

- Prompt templates loaded from embedded resources or custom directory
- Templates use proper placeholder syntax compatible with renderer
- Context extraction pulls relevant data from architecture and implementation plans
- Generated prompts include all required sections: Context, Rules, Tasks, Deliverables, Acceptance Criteria, Quality Checklist
- AGENTS.md rules filtered to only include relevant rules for the section
- Architecture context includes related components and dependencies
- Prompts written to `prompts/prompt_X_Y.md` by default
- Batch mode generates all prompts with progress indicator
- Interactive mode shows preview before writing each prompt
- User can skip, edit, or accept each prompt in interactive mode
- Existing prompts not overwritten unless `--force` flag provided
- Backup created automatically when overwriting (unless disabled)
- Custom output directory supported via `--output-dir` flag
- Multiple naming schemes supported (phase_section, descriptive, readable)
- All generated prompts are valid markdown with no syntax errors
- All generated prompts follow AGENTS.md conventions (lowercase filenames, .md extension, no emojis)
- Generated prompts include timestamp and generator version in footer
- Error handling for missing templates, invalid context, file I/O errors
- Progress tracking integration (marks sections as "prompt generated")
- Template validation before generation
- Dry-run mode to preview filenames without writing

**Example Generated Prompt**:

```markdown
# Phase 2: Component System

## Section 2.1: Component File Structure

## Context

You are implementing Component File Structure as part of the xzagentz project.

### Project Overview

xzagentz is a CLI tool for generating and managing AGENTS.md files for AI-assisted development.

### Architecture Context

The component system loads predefined content blocks from markdown files organized by category.
Each component is a self-contained markdown document with proper structure.

### Related Components

- Template System (loads and renders templates)
- Component Validator (validates component structure)
- CLI List Command (displays available components)

### Dependencies

- File system access (std::fs)
- Markdown parsing (optional)
- Error handling framework

## Critical Rules from AGENTS.md

### Rule 1: File Extensions

All Markdown files MUST use `.md` extension (not `.markdown` or `.MD`)
All YAML files MUST use `.yaml` extension (not `.yml`)

### Rule 2: Markdown File Naming

All markdown filenames MUST use lowercase_with_underscores
Exception: README.md is the ONLY uppercase filename allowed

### Rule 5: Documentation is Mandatory

Create documentation file in `docs/explanations/` for EVERY feature/task

## Tasks

Your goal is to complete the following tasks:

1. Create components directory structure under `components/`
2. Organize components by category: core, languages, tools, general
3. Create sample component files for each category
4. Ensure all component files follow markdown best practices
5. Add component file structure documentation

## Deliverables

You must create the following files:

- Component directory structure
- Sample component files in each category
- Documentation explaining component organization
- Tests validating directory structure

### Expected Files

- `components/core/critical_rules.md`
- `components/core/quick_reference.md`
- `components/languages/rust.md`
- `components/tools/cargo.md`
- `components/general/communication.md`
- `docs/explanations/component_structure.md`

### Test Files

- Tests in `src/components/tests.rs` or inline tests

## Acceptance Criteria

- [ ] Directory structure created under `components/` with subdirectories: core, languages, tools, general
- [ ] At least 2 sample components in each category
- [ ] All component filenames use lowercase_with_underscores.md
- [ ] All components are valid markdown with proper headers
- [ ] No emojis in component content
- [ ] Documentation file created explaining structure
- [ ] Tests verify expected files exist
- [ ] Tests verify valid markdown structure

## Quality Checklist

Before completing this section, verify:

- [ ] `cargo fmt --all` applied successfully
- [ ] `cargo check --all-targets --all-features` passes with zero errors
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] `cargo test --all-features` passes with >80% coverage
- [ ] All public items have doc comments with examples
- [ ] Documentation file created in `docs/explanations/`
- [ ] All filenames follow naming conventions (lowercase_with_underscores.md)
- [ ] All YAML files use `.yaml` extension (not `.yml`)
- [ ] No emojis in code or documentation

## Implementation Notes

1. Start by reading the architecture plan section
2. Create file stubs with proper module structure
3. Implement core functionality with error handling
4. Add comprehensive tests (unit, integration, doc)
5. Run quality checks incrementally
6. Create documentation file
7. Verify all acceptance criteria met

## References

- Architecture Plan: `docs/explanations/architecture_plan.md`
- AGENTS.md Rules: `AGENTS.md`
- Implementation Plan: `docs/explanations/implementation_plan.md`
- Phase 2 Overview: See implementation plan

---

Generated by xzagentz v0.1.0 on 2024-01-15T10:30:00Z
```

**CLI Integration**:

```bash
# Generate all prompts at once
xzagentz prompt generate --all

# Generate specific section prompt
xzagentz prompt generate --phase 2 --section 1

# Interactive generation with preview
xzagentz prompt generate --interactive

# Custom output directory
xzagentz prompt generate --all --output-dir ~/my-prompts

# Force overwrite without confirmation
xzagentz prompt generate --all --force

# Dry run (preview filenames)
xzagentz prompt generate --all --dry-run

# Use custom templates
xzagentz prompt generate --all --template-dir ~/.config/xzagentz/templates

# Different naming scheme
xzagentz prompt generate --all --naming-scheme readable

# Show generated prompt without saving
xzagentz prompt show --phase 2 --section 1
```

**Testing**:

```rust
#[test]
fn test_render_prompt_template() {
    let template = PromptTemplate {
        name: "test".to_string(),
        content: "Phase {{phase_number}}: {{phase_title}}".to_string(),
        required_context: vec!["phase_number".to_string(), "phase_title".to_string()],
        ..Default::default()
    };

    let context = PromptContext {
        phase_number: 1,
        phase_title: "Foundation".to_string(),
        ..Default::default()
    };

    let result = render_template(&template, &context).unwrap();
    assert_eq!(result, "Phase 1: Foundation");
}

#[test]
fn test_generate_all_prompts() {
    let config = PromptGeneratorConfig::default();
    let generator = PromptGenerator::new(config).unwrap();

    let impl_plan = load_test_implementation_plan();
    let arch_plan = load_test_architecture_plan();

    let generated = generator.generate_all(&impl_plan, &arch_plan).unwrap();

    assert!(generated.len() > 0);
    for path in generated {
        assert!(path.exists());
        assert!(path.extension().unwrap() == "md");
    }
}

#[test]
fn test_generate_single_section() {
    let config = PromptGeneratorConfig::default();
    let generator = PromptGenerator::new(config).unwrap();

    let context = create_test_context(2, 1);
    let path = generator.generate_section(&context).unwrap();

    assert!(path.exists());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("Phase 2"));
    assert!(content.contains("Section 2.1"));
}

#[test]
fn test_interactive_generation() {
    // Test interactive mode with mocked input
    let config = PromptGeneratorConfig::default();
    let generator = PromptGenerator::new(config).unwrap();

    // Mock user selecting: yes, no, edit, yes
    let impl_plan = load_test_implementation_plan();
    let arch_plan = load_test_architecture_plan();

    let generated = generator.generate_interactive(&impl_plan, &arch_plan).unwrap();

    // Should have generated some prompts (not all if user said no to some)
    assert!(generated.len() > 0);
}

#[test]
fn test_prevent_overwrite() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config = PromptGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf(),
        force_overwrite: false,
        ..Default::default()
    };
    let generator = PromptGenerator::new(config).unwrap();

    let context = create_test_context(1, 1);

    // Generate once
    let path1 = generator.generate_section(&context).unwrap();
    let content1 = std::fs::read_to_string(&path1).unwrap();

    // Try to generate again (should fail or skip)
    let result = generator.generate_section(&context);
    assert!(result.is_err() || result.unwrap() == path1);

    // Content should be unchanged
    let content2 = std::fs::read_to_string(&path1).unwrap();
    assert_eq!(content1, content2);
}

#[test]
fn test_custom_output_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let custom_dir = temp_dir.path().join("my-prompts");

    let config = PromptGeneratorConfig {
        output_dir: custom_dir.clone(),
        ..Default::default()
    };
    let generator = PromptGenerator::new(config).unwrap();

    let context = create_test_context(1, 1);
    let path = generator.generate_section(&context).unwrap();

    assert!(path.starts_with(&custom_dir));
    assert!(path.exists());
}

#[test]
fn test_naming_schemes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let context = create_test_context(2, 3);

    // Test PhaseSection naming
    let config = PromptGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf(),
        naming_scheme: PromptNamingScheme::PhaseSection,
        ..Default::default()
    };
    let generator = PromptGenerator::new(config).unwrap();
    let path = generator.generate_section(&context).unwrap();
    assert_eq!(path.file_name().unwrap(), "prompt_2_3.md");

    // Test Descriptive naming
    let config = PromptGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf(),
        naming_scheme: PromptNamingScheme::Descriptive,
        ..Default::default()
    };
    let generator = PromptGenerator::new(config).unwrap();
    let path = generator.generate_section(&context).unwrap();
    assert_eq!(path.file_name().unwrap(), "prompt_phase2_section3.md");

    // Test Readable naming
    let config = PromptGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf(),
        naming_scheme: PromptNamingScheme::Readable,
        ..Default::default()
    };
    let generator = PromptGenerator::new(config).unwrap();
    let path = generator.generate_section(&context).unwrap();
    assert!(path.file_name().unwrap().to_str().unwrap().starts_with("phase_2_"));
}

#[test]
fn test_context_extraction() {
    let impl_plan = load_test_implementation_plan();
    let arch_plan = load_test_architecture_plan();

    let context = extract_context(&impl_plan, &arch_plan, 2, 1).unwrap();

    assert_eq!(context.phase_number, 2);
    assert_eq!(context.section_number, "2.1");
    assert!(!context.section_tasks.is_empty());
    assert!(!context.section_deliverables.is_empty());
    assert!(!context.section_acceptance_criteria.is_empty());
}

#[test]
fn test_relevant_rules_extraction() {
    let context = create_test_context(2, 1);
    context.section_title = "Component File Structure".to_string();

    let rules = extract_relevant_rules(&context).unwrap();

    // Should include file naming and structure rules
    assert!(rules.iter().any(|r| r.contains("File Extensions")));
    assert!(rules.iter().any(|r| r.contains("Markdown File Naming")));
}

#[test]
fn test_backup_creation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("prompt_1_1.md");

    std::fs::write(&file_path, "original content").unwrap();

    let config = PromptGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf(),
        force_overwrite: true,
        create_backup: true,
        ..Default::default()
    };
    let generator = PromptGenerator::new(config).unwrap();

    let backup_path = generator.create_backup(&file_path).unwrap();

    assert!(backup_path.exists());
    assert!(backup_path.to_str().unwrap().contains(".bak"));

    let backup_content = std::fs::read_to_string(&backup_path).unwrap();
    assert_eq!(backup_content, "original content");
}
```

### 7.6 Progress Tracking and Compliance Verification

**Tasks**:

1. Implement progress state management
2. Persist progress to file (TOML format)
3. Implement AGENTS.md rule checker
4. Define compliance rules per category
5. Generate violation reports with fix suggestions
6. Write progress and compliance tests

**Deliverables**:

- `src/prompts/progress.rs` - Progress tracking
- `src/prompts/compliance.rs` - Compliance verification
- `ProgressTracker` struct
- `ProgressState` struct
- `ComplianceChecker` struct
- `ComplianceRule` struct
- `Violation` struct
- `ComplianceReport` struct
- `.implementation_progress` file format (TOML)

**Acceptance Criteria**:

- Tracks current section number
- Maintains list of completed sections
- Saves progress to `.implementation_progress`
- Calculates completion percentage
- Checks all AGENTS.md critical rules
- Generates detailed violation reports
- Provides fix suggestions
- Returns proper exit codes for CI/CD
- Handles corrupted progress files

**Testing**:

```rust
#[test]
fn test_save_progress()
#[test]
fn test_load_progress()
#[test]
fn test_calculate_completion()
#[test]
fn test_verify_markdown_naming()
#[test]
fn test_verify_code_blocks()
#[test]
fn test_verify_yaml_extensions()
#[test]
fn test_generate_compliance_report()
#[test]
fn test_fix_suggestions()
```

### 7.7 Template CLI Commands

**Tasks**:

1. Implement `xzagentz templates list` command
2. Implement `xzagentz templates show <name>` command
3. Implement `xzagentz templates export [name]` command
4. Add global `--template-dir <DIR>` flag
5. Integrate template system with plan/prompt commands
6. Write CLI integration tests

**Deliverables**:

- `src/cli/templates.rs` - Template CLI commands
- `TemplatesArgs` struct
- `TemplatesCommand` enum
- Template listing with category filtering
- Template detail view
- Template export to filesystem
- Global template directory configuration

**Acceptance Criteria**:

- `templates list` shows all available templates
- `templates list --category plans` filters by category
- `templates show <name>` displays template content and metadata
- `templates export` exports all templates to user config directory
- `templates export <name>` exports single template
- `templates export --output <dir>` exports to custom directory
- `--template-dir` flag works with all plan/prompt commands
- Config file supports `templates.custom_dir` setting
- Helpful error messages for missing templates
- Output includes user config directory path

**Testing**:

```rust
#[test]
fn test_templates_list_command()
#[test]
fn test_templates_list_with_category()
#[test]
fn test_templates_show_command()
#[test]
fn test_templates_export_all()
#[test]
fn test_templates_export_single()
#[test]
fn test_templates_export_to_custom_dir()
#[test]
fn test_global_template_dir_flag()
#[test]
fn test_template_not_found_error()
```

### 7.8 Plan and Prompt CLI Integration

**Tasks**:

1. Add `plan` subcommand to CLI
2. Implement architecture plan commands
3. Implement implementation plan commands
4. Add `prompt` subcommand to CLI
5. Implement prompt generation commands
6. Write CLI integration tests

**Deliverables**:

- CLI command structure for plan management
- `xzagentz plan architecture create` command
- `xzagentz plan architecture verify` command
- `xzagentz plan implementation create` command
- `xzagentz plan implementation verify` command
- `xzagentz plan list-templates` command
- `xzagentz prompt generate` command
- `xzagentz prompt show <section>` command
- `xzagentz prompt progress` command
- `xzagentz prompt verify` command
- Complete help text for all commands

**Acceptance Criteria**:

- All plan subcommands work correctly
- All prompt subcommands work correctly
- Help text is clear and accurate
- Supports `--template`, `--from-architecture` flags
- Supports `--batch` and `--interactive` flags
- Supports custom `--plan` and `--output` paths
- Verify commands support `--strict` mode for CI/CD
- All commands have proper error handling
- All commands are documented

**Testing**:

```rust
#[test]
fn test_plan_create_with_embedded_template()
#[test]
fn test_plan_create_with_custom_template_dir()
#[test]
fn test_plan_create_interactive_template_selection()
#[test]
fn test_plan_implementation_from_architecture()
#[test]
fn test_plan_verify_command()
#[test]
fn test_prompt_generate_uses_templates()
#[test]
fn test_prompt_generate_all()
#[test]
fn test_prompt_generate_interactive()
#[test]
fn test_prompt_show_command()
#[test]
fn test_prompt_progress_tracking()
#[test]
fn test_prompt_verify_compliance()
#[test]
fn test_template_dir_flag_integration()
```

## Phase 8: Integration and Polish

**Duration**: Week 9

**Goal**: Integration testing, documentation, and final polish

### 8.1 Integration Tests

**Tasks**:

1. Write end-to-end integration tests
2. Test complete workflows
3. Test error scenarios
4. Test with real component files
5. Test config file persistence
6. Measure test coverage

**Deliverables**:

- `tests/integration/create_tests.rs`
- `tests/integration/update_tests.rs`
- `tests/integration/validate_tests.rs`
- `tests/integration/prompts_tests.rs`
- `tests/integration/config_tests.rs`
- `tests/integration/templates_tests.rs`
- Test fixtures in tests/fixtures/

**Acceptance Criteria**:

- All commands tested end-to-end
- Error scenarios covered
- Config save/load workflow tested
- Template loading priority tested
- Test coverage exceeds 80 percent
- Integration tests pass consistently
- Test fixtures are realistic

**Testing**:

```rust
#[test]
fn test_create_command_end_to_end()
#[test]
fn test_update_command_workflow()
#[test]
fn test_validate_detects_issues()
#[test]
fn test_prompt_generation_workflow()
#[test]
fn test_config_persistence_workflow()
#[test]
fn test_template_loading_priority()
```

### 8.2 Documentation

**Tasks**:

1. Write comprehensive README.md
2. Create architecture documentation
3. Write component creation guide
4. Write template creation guide
5. Write implementation plan creation guide
6. Create CLI reference documentation

**Deliverables**:

- `README.md` (updated)
- `docs/explanations/architecture.md`
- `docs/explanations/component_system.md`
- `docs/explanations/template_system_design.md`
- `docs/explanations/project_config_system.md`
- `docs/how_to/create_components.md`
- `docs/how_to/create_templates.md`
- `docs/how_to/customize_templates.md`
- `docs/reference/cli_reference.md`
- `docs/reference/component_format.md`
- `docs/reference/template_format.md`
- `docs/reference/config_format.md`
- `docs/reference/implementation_format.md`

**Acceptance Criteria**:

- README.md is complete and clear
- Architecture is well-documented
- How-to guides are step-by-step
- Reference docs are comprehensive
- Config system fully documented
- Template system fully documented
- All docs follow AGENTS.md rules
- No emojis in documentation
- All filenames use snake_case

**Testing**:

- Manual review of all documentation
- Verify all links work
- Check formatting compliance

### 8.3 Examples

**Tasks**:

1. Create example output files
2. Document example scenarios
3. Create example templates
4. Test all examples
5. Add examples to documentation

**Deliverables**:

- `examples/python_cli_example.md`
- `examples/rust_binary_example.md`
- `examples/golang_service_example.md`
- Example template files
- Example component files

**Acceptance Criteria**:

- Examples cover common use cases
- Example output is valid
- Examples are well-documented
- Examples follow AGENTS.md rules
- Examples can be reproduced

**Testing**:

- Generate each example with CLI
- Validate generated output
- Verify examples work as documented

### 8.4 Performance and Optimization

**Tasks**:

1. Profile command execution
2. Optimize file I/O operations
3. Implement caching where beneficial
4. Reduce memory allocations
5. Benchmark critical paths

**Deliverables**:

- Performance benchmarks
- Optimization implementations
- Caching improvements
- Benchmark results documentation

**Acceptance Criteria**:

- Create command completes in under 1 second
- Memory usage is reasonable
- No unnecessary file reads
- Caching improves performance
- Benchmarks show improvements

**Testing**:

```rust
#[bench]
fn bench_create_command()
#[bench]
fn bench_component_loading()
#[bench]
fn bench_placeholder_replacement()
```

## Phase 9: Release Preparation

**Duration**: Week 10

**Goal**: Prepare for initial release

### 9.1 CI/CD Pipeline

**Tasks**:

1. Set up GitHub Actions workflows
2. Add automated testing
3. Add automated linting
4. Add automated releases
5. Configure branch protection

**Deliverables**:

- `.github/workflows/ci.yaml`
- `.github/workflows/release.yaml`
- Automated test runs
- Automated clippy checks
- Automated formatting checks

**Acceptance Criteria**:

- CI runs on every push
- All quality checks automated
- Releases are automated
- Branch protection enabled
- CI badge in README.md

**Testing**:

- Verify CI runs successfully
- Test release workflow
- Check all automated checks

### 9.2 Installation and Distribution

**Tasks**:

1. Add installation instructions
2. Configure cargo install support
3. Create release artifacts
4. Write changelog
5. Tag first release

**Deliverables**:

- Updated README.md with install instructions
- CHANGELOG.md
- GitHub release with binaries
- cargo publish configuration

**Acceptance Criteria**:

- cargo install works
- Release binaries available
- Installation instructions clear
- Changelog follows format
- Version tagged in git

**Testing**:

- Test cargo install
- Test binary downloads
- Verify installation steps

### 9.3 Final Quality Checks

**Tasks**:

1. Run all quality checks
2. Review all documentation
3. Test all examples
4. Verify AGENTS.md compliance
5. Conduct code review

**Deliverables**:

- Quality check results
- Documentation review notes
- Compliance verification report
- Code review feedback addressed

**Acceptance Criteria**:

- cargo fmt passes
- cargo clippy shows zero warnings
- cargo test passes with >80 percent coverage
- All documentation reviewed
- All AGENTS.md rules followed
- No known bugs

**Testing**:

- Run complete test suite
- Verify all quality gates
- Manual testing of all features

## Success Metrics

### Code Quality

- Test coverage exceeds 80 percent across all modules
- Zero clippy warnings with `-D warnings`
- All code formatted with cargo fmt
- All public items have doc comments
- All error paths tested

### Documentation

- README.md is comprehensive
- All commands documented in CLI reference
- Architecture documented in explanations
- How-to guides for common tasks
- Reference docs for formats
- All docs follow AGENTS.md rules

### Functionality

- All CLI commands work as specified
- Component system loads all types
- Template system processes all formats
- Template loading priority works correctly
- Placeholder replacement handles all cases
- Validation catches common issues
- Config file persistence works reliably
- Prompt generation creates useful prompts
- Plan management workflow is complete

### Performance

- Create command completes in under 1 second
- List command is instant
- Validate command processes files quickly
- Memory usage is reasonable
- No unnecessary file I/O

## Risk Mitigation

### Technical Risks

**Risk**: Complex placeholder replacement logic

**Mitigation**: Start with simple @key@ format, add features incrementally, test exhaustively

**Risk**: TOML parsing edge cases

**Mitigation**: Use battle-tested toml crate, validate templates early, provide clear errors

**Risk**: Markdown parsing complexity

**Mitigation**: Use line-based parsing, handle common patterns, don't aim for perfect parsing

### Process Risks

**Risk**: Scope creep

**Mitigation**: Stick to phased plan, defer non-essential features to future versions

**Risk**: Quality issues from rushed implementation

**Mitigation**: Enforce quality gates at each phase, don't move to next phase until current is complete

**Risk**: Insufficient testing

**Mitigation**: Write tests alongside implementation, aim for >80 percent coverage, include integration tests

## Dependencies

### External Crates

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"
walkdir = "2.5"
dirs = "5.0"              # Cross-platform user config directory paths
chrono = "0.4"            # Date/time for progress tracking
regex = "1.10"            # Pattern matching for compliance verification

[dev-dependencies]
tempfile = "3.12"
assert_cmd = "2.0"
predicates = "3.1"
```

### Development Tools

- rustfmt: Code formatting
- clippy: Linting
- cargo-audit: Security auditing
- cargo-tarpaulin: Code coverage (optional)

## Timeline Summary

- **Week 1**: Phase 1 - Foundation
- **Week 2**: Phase 2 - Component System
- **Week 3**: Phase 3 - Template System
- **Week 4**: Phase 4 - CLI Foundation
- **Week 5-6**: Phase 5 - Create Command
- **Week 7**: Phase 6 - Update, Add, and Config Management
- **Week 8-9**: Phase 7 - Plan Management, Prompt Generation, and Template System
- **Week 10**: Phase 8 - Integration and Polish
- **Week 11**: Phase 9 - Release Preparation

**Total Duration**: 11 weeks

**Critical Path**: Phase 1 → Phase 2 → Phase 3 → Phase 5 (create command depends on all previous work)

## Next Steps

1. Review and approve this implementation plan
2. Create GitHub project with phases as milestones
3. Create issues for each subsection
4. Begin Phase 1: Project Foundation
5. Use prompt generation system to guide implementation

## Conclusion

This implementation plan breaks down the xzagentz project into manageable phases, each with clear deliverables and acceptance criteria. By following this plan systematically and adhering to the AGENTS.md rules at every step, we will create a high-quality, well-tested, and well-documented CLI tool for generating AGENTS.md files.

The phased approach ensures that:

- Each phase builds on previous work
- Quality is maintained throughout
- Testing is comprehensive
- Documentation is complete
- AGENTS.md compliance is verified

Following this plan will result in a professional-grade Rust CLI tool that serves its intended purpose effectively.

---

**Document Version**: 1.0

**Last Updated**: 2024

**Status**: Approved for Implementation
