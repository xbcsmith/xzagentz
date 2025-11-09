# Phase 4 CLI Foundation Implementation

## Overview

Phase 4 implements the command-line interface foundation for xzagentz using clap v4 with derive macros. This phase provides the core CLI structure, argument parsing, and implements two essential commands: list and validate.

## Components Delivered

### Core CLI Structure

- `src/cli/mod.rs` (551 lines) - Main CLI module with clap v4 structs
- `src/cli/output.rs` (145 lines) - Output formatting utilities
- `src/cli/list.rs` (285 lines) - List command implementation
- `src/cli/validate.rs` (475 lines) - Validate command implementation
- `src/main.rs` (130 lines) - Updated main entry point with command dispatch

Total: approximately 1,586 lines of production code

### Dependencies Added

- `serde_json = "1.0"` - JSON output formatting support

## Implementation Details

### CLI Architecture

The CLI is structured using clap v4 derive macros for type-safe argument parsing:

```rust
pub struct Cli {
    pub verbose: bool,
    pub config_dir: Option<PathBuf>,
    pub template_dir: Option<PathBuf>,
    pub component_dir: Option<PathBuf>,
    pub format: OutputFormat,
    pub command: Commands,
}
```

Global flags are available to all subcommands and can be set via environment variables:
- `XZAGENTZ_CONFIG_DIR` - Custom configuration directory
- `XZAGENTZ_TEMPLATE_DIR` - Custom templates directory
- `XZAGENTZ_COMPONENT_DIR` - Custom components directory

### Output Formatting System

The output module provides utilities for formatting CLI output in both human-readable and JSON formats:

```rust
pub fn print_output<T>(format: OutputFormat, data: &T) -> io::Result<()>
where
    T: Serialize + std::fmt::Display
```

This allows all commands to support both interactive use and machine parsing via the `--format json` flag.

### List Command

The list command displays available components and templates:

```bash
xzagentz list components [--category core]
xzagentz list templates [--detailed]
```

Key features:
- Lists components from all four categories (core, languages, tools, general)
- Supports filtering by category
- Shows first non-empty line as summary
- Template listing with optional details (component count, description)
- Graceful handling of missing directories

Implementation highlights:
- Uses `ComponentLoader` to load components by type
- Uses `TemplateLoader` to discover and load templates
- Converts internal structures to display-friendly formats
- Supports both human and JSON output

### Validate Command

The validate command checks AGENTS.md files for compliance with project standards:

```bash
xzagentz validate [FILE] [--detailed] [--fix]
```

Validation checks:
- Emoji usage detection (ERROR level)
- Markdown filename case checking (ERROR for uppercase, WARNING for kebab-case)
- YAML extension checking (ERROR for .yml)
- Code block language identifiers (WARNING if missing)
- Section structure validation (INFO level, only in detailed mode)

Severity levels:
- ERROR - Must fix (causes validation failure)
- WARNING - Should fix (does not fail validation)
- INFO - Suggestions (does not fail validation)

Example validation output:

```text
Validation Report: AGENTS.md
============================================================
Total: 101 (Errors: 61, Warnings: 40, Info: 0)

Issues:
  [ERROR] Line 66: Emoji usage detected
  Suggestion: Remove all emojis from documentation
  [ERROR] Line 245: Reference to .yml file
  Suggestion: Change .yml to .yaml
```

### Command Dispatch

The main.rs file now provides a clean command dispatch system:

```rust
fn run(cli: Cli) -> Result<(), Error> {
    let component_dir = cli.get_component_dir();
    let template_dir = cli.get_template_dir();

    match cli.command {
        Commands::List { target } => {
            list::execute(&target, &component_dir, &template_dir,
                         cli.format, cli.verbose)?;
        }
        Commands::Validate { file, detailed, fix } => {
            validate::execute(&file, detailed, fix, cli.format, cli.verbose)?;
        }
        // Other commands return NotImplemented error
    }
    Ok(())
}
```

### Error Handling Enhancements

Added new error variants to support CLI operations:

```rust
pub enum Error {
    ComponentLoadError(String),
    ValidationError(String),
    NotImplemented(String),
    // ... existing variants
}
```

These errors provide clear messages for users and support the CLI's error reporting needs.

## Testing

Test coverage: 222 tests passed (138 unit + 3 integration + 81 doc)

### Unit Tests

Each CLI module includes comprehensive unit tests:

```rust
#[test]
fn test_parse_list_components() {
    let cli = Cli::parse_from(["xzagentz", "list", "components"]);
    assert!(matches!(cli.command, Commands::List { .. }));
}

#[test]
fn test_validate_with_flags() {
    let cli = Cli::parse_from(["xzagentz", "validate", "--detailed", "--fix"]);
    // Verify flags are parsed correctly
}
```

### Integration Tests

The main.rs includes tests verifying command dispatch:

```rust
#[test]
fn test_run_create_not_implemented() {
    let cli = Cli::parse_from(["xzagentz", "create"]);
    let result = run(cli);
    assert!(matches!(result, Err(Error::NotImplemented(_))));
}
```

### Validation Tests

Validator tests cover all check types:

```rust
#[test]
fn test_check_emojis() {
    let content = "This has an emoji: 😀";
    let issues = check_emojis(content);
    assert!(!issues.is_empty());
    assert_eq!(issues[0].severity, Severity::Error);
}

#[test]
fn test_check_yaml_extensions() {
    let content = "config.yml is wrong";
    let issues = check_yaml_extensions(content);
    assert_eq!(issues.len(), 1);
}
```

## Usage Examples

### List all components

```bash
xzagentz list components
```

Output:
```text
Available Components (12)
==================================================
  Core/file_extensions: Use .yaml not .yml
  Core/naming_conventions: Lowercase with underscores
  ...
```

### List components in specific category

```bash
xzagentz list components --category core
```

### List templates with details

```bash
xzagentz list templates --detailed
```

Output:
```text
Available Templates (3)
==================================================
  rust_binary: Rust binary project template (8 components)
  web_service: Web service template (12 components)
  ...
```

### Validate AGENTS.md file

```bash
xzagentz validate AGENTS.md
```

### Validate with detailed report

```bash
xzagentz validate --detailed AGENTS.md
```

### JSON output for tooling

```bash
xzagentz --format json list components > components.json
xzagentz --format json validate AGENTS.md > report.json
```

### Verbose mode for debugging

```bash
xzagentz -v list components
```

Output includes:
```text
DEBUG: Component dir: "components"
DEBUG: Template dir: "templates"
DEBUG: Loading components...
DEBUG: Found 12 components
```

## Design Decisions

### Clap v4 Derive API

Chose clap v4 with derive macros over builder API for:
- Type safety at compile time
- Less boilerplate code
- Automatic help generation
- Built-in validation
- Better IDE support

### Global Options

Made key options global (verbose, directories, format) so they work with all commands without repetition.

### Output Formatting Abstraction

Created a generic output formatting system that supports both Display and Serialize, allowing commands to focus on logic rather than presentation.

### Error Severity Levels

Implemented three severity levels (Error, Warning, Info) to distinguish between:
- Must-fix issues (errors block success)
- Should-fix issues (warnings are noted but allowed)
- Nice-to-have suggestions (info only shown in detailed mode)

### Graceful Degradation

List command gracefully handles missing directories by:
- Showing empty results without error in normal mode
- Showing warnings in verbose mode
- Never failing just because a directory does not exist

This allows the tool to work in various project states.

### Environment Variable Support

All directory options support environment variables for:
- CI/CD integration
- User preferences via shell configuration
- Docker container configuration

## Validation Results

All quality checks passed:

```bash
cargo fmt --all                                      # PASSED
cargo check --all-targets --all-features             # PASSED
cargo clippy --all-targets --all-features -- -D warnings  # PASSED (0 warnings)
cargo test --all-features                            # PASSED (222 tests)
```

Test results:
- 138 unit tests passed
- 3 integration tests passed
- 81 documentation tests passed
- Zero clippy warnings
- Zero compilation errors

## Command Reference

### Global Options

```text
-v, --verbose                Enable verbose output
--config-dir <PATH>          Configuration directory
--template-dir <PATH>        Templates directory
--component-dir <PATH>       Components directory
--format <human|json>        Output format
```

### List Command

```text
xzagentz list components [--category <CAT>]
xzagentz list templates [--detailed]
```

### Validate Command

```text
xzagentz validate [FILE] [--detailed] [--fix]
```

Note: The `--fix` flag is recognized but not yet implemented. It displays a warning.

## Future Commands

Placeholders exist for future phases:
- `create` - Generate new AGENTS.md files (Phase 5)
- `update` - Modify existing AGENTS.md sections (Phase 6)
- `add` - Add components to existing files (Phase 6)

These commands return `Error::NotImplemented` with helpful messages directing users to the implementation timeline.

## References

- Implementation Plan: `docs/explanation/implementation_plan.md`
- Phase 3: `docs/explanation/phase3_template_system_implementation.md`
- AGENTS.md Rules: `AGENTS.md`
- Clap Documentation: https://docs.rs/clap/latest/clap/

---

**Phase Status**: Complete

**Next Phase**: Phase 5 - Create Command Implementation
