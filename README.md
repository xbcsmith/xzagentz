# xzagentz

[![Component Validation](https://github.com/xbcsmith/xzagentz/workflows/Component%20Validation/badge.svg)](https://github.com/xbcsmith/xzagentz/actions)
[![Rust](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance Rust CLI tool for managing language-agnostic development components with intelligent rendering, validation, and code generation capabilities.

## Overview

xzagentz provides a comprehensive system for authoring, validating, and rendering development guidance components that adapt to multiple programming languages. Components are reusable documentation units with language-specific sections, automatic size validation, tier-based complexity management, and CI-driven quality assurance.

### Key Features

- **Language-Agnostic Components**: Write once, render for multiple languages (Rust, Python, Go, TypeScript, Bash)
- **Intelligent Rendering**: Automatic language-specific content extraction with fallback to universal content
- **Size Enforcement**: Category-based line limits ensure components remain focused and maintainable
- **Tier System**: Essential vs comprehensive tool components for different use cases and complexity levels
- **Automated Validation**: CI-driven validation of frontmatter, markers, and size constraints
- **Type-Safe Architecture**: Leverages Rust's type system for robust validation and error handling
- **Comprehensive Documentation**: Complete guides for users and developers following Diataxis framework
- **Production Ready**: All quality gates passing, 621 tests, v2.0.0 released

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/xbcsmith/xzagentz.git
cd xzagentz

# Build the project
cargo build --release

# Run tests
cargo test --all-features

# Install globally
cargo install --path .
```

### Prerequisites

- Rust 1.70 or later
- Cargo

### Basic Usage

```bash
# Validate a component
xzagentz validate components/core/error_handling.md

# Render component for specific language
xzagentz render --component error_handling --language rust

# Generate validation report
xzagentz validate --report

# Check all components
cargo test size_validation_test

# List available components
xzagentz list --category core
```

## Embedded Resources

xzagentz includes all components and templates embedded in the binary, allowing it to work out of the box without any setup. You can also customize resources by extracting and modifying them.

### Using Embedded Resources (Default)

No setup required - just run commands:

```bash
# Create a new project using embedded resources
xzagentz create my-project --template rust_binary --language rust

# List embedded components
xzagentz list components

# List embedded templates
xzagentz list templates
```

### Customizing Resources

Extract resources for customization:

```bash
# Initialize and extract to default location (~/.config/xzagentz)
xzagentz init

# Or extract to custom location
xzagentz init --config-dir ./my-config
```

This creates:

```text
~/.config/xzagentz/
├── components/
│   ├── core/
│   ├── development/
│   ├── documentation/
│   └── testing/
└── templates/
    └── plans/
```

Modify extracted files and xzagentz will automatically use your customized versions.

### Environment Variables

Control resource locations with environment variables:

```bash
# Use custom components directory
export XZAGENTZ_COMPONENTS_DIR=/path/to/components

# Use custom templates directory
export XZAGENTZ_TEMPLATES_DIR=/path/to/templates

# Use custom config directory
export XZAGENTZ_CONFIG_DIR=/path/to/config
```

### Resource Resolution

xzagentz searches for resources in this order:

1. Custom directory (via CLI flag)
2. Environment variable (XZAGENTZ\_\*\_DIR)
3. XDG directories (XDG_DATA_HOME)
4. Home directory (~/.config/xzagentz)
5. Embedded resources (always available)

For more details, see:

- Setup Guide: `docs/how_to/setup_custom_resources.md`
- Environment Variables: `docs/reference/environment_variables.md`
- Architecture: `docs/explanations/embedded_resources_architecture.md`

## Component System

### Component Structure

Components consist of YAML frontmatter and markdown content with language-specific sections:

```markdown
---
component:
  name: error_handling
  category: core
  version: 2.0.0
  description: Error handling standards and patterns
  languages:
    - rust
    - python
    - golang
    - typescript
  sections:
    - id: principles
      language_specific: false
      required: true
    - id: patterns
      language_specific: true
      required: true
---

# Error Handling

## Universal Principles

Content that applies to all languages...

---

## Language-Specific Patterns

<!-- LANG:rust -->

Rust-specific error handling with Result types...

<!-- /LANG:rust -->

<!-- LANG:python -->

Python-specific exception handling...

<!-- /LANG:python -->

<!-- LANG:typescript -->

TypeScript-specific error handling...

<!-- /LANG:typescript -->
```

### Supported Languages

xzagentz provides full support for five programming languages:

- **Rust**: Systems programming with memory safety
- **Python**: Scripting and data processing
- **Go**: Concurrent services and CLI tools
- **TypeScript**: Type-safe web applications
- **Bash**: Shell scripting and automation

Each language has dedicated components with language-specific guidelines, patterns, and best practices.

### Component Categories

| Category      | Description                       | Size Limit    | Components | Use Case                              |
| ------------- | --------------------------------- | ------------- | ---------- | ------------------------------------- |
| **core**      | Fundamental development practices | 500 lines     | 5          | Error handling, testing, code quality |
| **general**   | Project management and workflow   | 800 lines     | 8          | Git, documentation, CI/CD             |
| **languages** | Language-specific features        | 600 lines     | 4          | Language idioms and patterns          |
| **tools**     | Tool-specific guidance            | 300-800 lines | 12         | Docker, Kubernetes, Git, etc.         |

**Total**: 29 components across 4 categories

### Tool Tiers

Tool components support two complexity tiers for flexible documentation depth:

- **Essential** (300 lines): Core commands and common workflows for daily use
- **Comprehensive** (800 lines): Complete reference with advanced features, edge cases, and optimization

This tiering system allows projects to choose appropriate documentation depth based on team needs and experience levels.

## Component Library

### Core Components (5)

- `critical_rules.md` - Essential development rules and practices
- `error_handling.md` - Error handling patterns across languages
- `testing_standards.md` - Testing requirements and best practices
- `header.md` - Project header and documentation templates
- `learning_resources.md` - Learning materials and references

### General Components (8)

- `git_workflow.md` - Git conventions and branching strategies
- `documentation_standards.md` - Documentation guidelines and requirements
- `code_review.md` - Code review practices and checklists
- `ci_cd_integration.md` - CI/CD patterns and automation
- `project_structure.md` - Project organization and directory layouts
- `dependency_management.md` - Dependency handling and versioning
- `security_practices.md` - Security guidelines and threat mitigation
- `performance_optimization.md` - Performance patterns and profiling

### Language Components (4)

- `rust.md` - Rust language guidelines, patterns, and idioms (400 lines)
- `python.md` - Python language guidelines and best practices (260 lines)
- `golang.md` - Go language guidelines and concurrency patterns (370 lines)
- `typescript.md` - TypeScript guidelines, type system, and patterns (565 lines)

### Tool Components (12)

Essential and comprehensive tiers for each tool:

- `git_essential.md` / `git_comprehensive.md` - Git version control
- `markdown_essential.md` / `markdown_comprehensive.md` - Markdown documentation
- `docker_essential.md` / `docker_comprehensive.md` - Docker containerization
- `kubernetes_essential.md` / `kubernetes_comprehensive.md` - Kubernetes orchestration
- `github_actions_essential.md` / `github_actions_comprehensive.md` - GitHub CI/CD
- `aws_essential.md` / `aws_comprehensive.md` - AWS cloud services

## Architecture

### Project Structure

```
xzagentz/
├── src/
│   ├── lib.rs              # Library root
│   ├── main.rs             # CLI entry point
│   ├── error.rs            # Error types and handling
│   ├── cli/                # Command-line interface
│   │   ├── mod.rs
│   │   ├── create.rs
│   │   ├── update.rs
│   │   ├── add.rs
│   │   ├── validate.rs
│   │   ├── list.rs
│   │   └── prompt.rs
│   ├── components/         # Component system
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   ├── renderer.rs
│   │   ├── metadata.rs
│   │   ├── validator.rs
│   │   └── language_filter.rs
│   ├── templates/          # Template system
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   ├── renderer.rs
│   │   └── validator.rs
│   ├── prompts/            # Prompt generation
│   │   ├── mod.rs
│   │   ├── generator.rs
│   │   ├── context.rs
│   │   └── template.rs
│   ├── plans/              # Plan management
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   └── structures.rs
│   ├── validator/          # Validation system
│   │   ├── mod.rs
│   │   └── size.rs
│   ├── parser/             # Parsing infrastructure
│   │   ├── mod.rs
│   │   └── agents.rs
│   ├── config/             # Configuration
│   │   ├── mod.rs
│   │   └── project.rs
│   └── core/               # Core utilities
│       └── mod.rs
├── components/             # Component library (29 components)
│   ├── core/               # Core components (5)
│   ├── general/            # General components (8)
│   ├── languages/          # Language components (4)
│   └── tools/              # Tool components (12)
├── tests/                  # Integration tests
│   ├── core_components_test.rs
│   ├── general_components_test.rs
│   ├── tool_component_tiering_test.rs
│   └── size_validation_test.rs
├── examples/               # Example components
│   └── components/
│       ├── core_example.md
│       ├── tool_essential_example.md
│       └── tool_comprehensive_example.md
└── docs/                   # Documentation
    ├── how_to/             # Task-oriented guides
    │   ├── authoring_components.md
    │   └── migrating_to_v2.md
    ├── reference/          # Technical specifications
    │   ├── architecture.md
    │   ├── component_configuration.md
    │   └── troubleshooting.md
    └── explanations/       # Conceptual documentation
        ├── implementation.md
        └── language_agnostic_component_system_implementation_plan.md
```

### Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Interface                          │
│        (create, update, add, validate, list, prompt)        │
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

## Documentation

### For Users

- **[Authoring Components](docs/how_to/authoring_components.md)**: Complete guide to creating components with examples and best practices
- **[Migrating to v2](docs/how_to/migrating_to_v2.md)**: Step-by-step upgrade guide from legacy components to v2 format
- **[Configuration Reference](docs/reference/component_configuration.md)**: Complete configuration documentation with schema reference
- **[Troubleshooting](docs/reference/troubleshooting.md)**: Common issues, diagnostics, and solutions

### For Developers

- **[Architecture Reference](docs/reference/architecture.md)**: Comprehensive system architecture and design patterns
- **[Implementation Guide](docs/explanations/implementation.md)**: Consolidated implementation documentation covering all 6 phases
- **[Implementation Plan](docs/explanations/language_agnostic_component_system_implementation_plan.md)**: Original design decisions and requirements
- **[AGENTS.md](AGENTS.md)**: AI agent development guidelines and quality standards

### Examples

See `examples/components/` for reference implementations:

- `core_example.md` - Code review practices (core category, 416 lines)
- `tool_essential_example.md` - Docker essential tier (371 lines)
- `tool_comprehensive_example.md` - Docker comprehensive tier (648 lines)

## Development

### Building from Source

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check --all-targets --all-features

# Clean build artifacts
cargo clean
```

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test size_validation_test
cargo test core_components_test

# Run with output
cargo test -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test

# Generate test coverage report
cargo tarpaulin --all-features
```

**Test Statistics**:

- Total tests: 621
- Unit tests: 345
- Integration tests: 276
- Coverage: >80%
- Status: 100% passing

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter (treats warnings as errors)
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Security audit (requires cargo-audit)
cargo audit
```

### Pre-Commit Workflow

Before committing changes, run all quality checks:

1. Format code: `cargo fmt --all`
2. Check compilation: `cargo check --all-targets --all-features`
3. Run linter: `cargo clippy --all-targets --all-features -- -D warnings`
4. Run tests: `cargo test --all-features`
5. Validate components: `cargo test size_validation_test`

All checks must pass before code can be merged.

## CI/CD

### GitHub Actions

Automated validation runs on every pull request and push to main branches:

- **Frontmatter Validation**: Ensures YAML is valid and complete with all required fields
- **Marker Validation**: Verifies language markers are balanced and properly closed
- **Size Validation**: Enforces category-specific and project-wide line limits
- **Test Suite**: Runs all 621 unit and integration tests
- **Linting**: Runs clippy with zero-warning enforcement
- **Validation Report**: Generates size report artifact for review

View validation status: [Actions](https://github.com/xbcsmith/xzagentz/actions)

### Validation Workflow

```yaml
name: Component Validation
on: [pull_request, push]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all-features
      - run: cargo test size_validation_test
      - name: Generate validation report
        run: cargo run -- validate --report
      - uses: actions/upload-artifact@v3
        with:
          name: validation-report
          path: validation_report.md
```

## Contributing

### Component Guidelines

1. **Stay within size limits**: Check category-specific limits before submission
2. **Use proper frontmatter**: Include all required fields (name, category, version, description, languages, sections)
3. **Balance markers**: Every `<!-- LANG:X -->` must have a matching `<!-- /LANG:X -->`
4. **Write tests**: Add tests for new functionality with >80% coverage target
5. **Update documentation**: Keep docs in sync with code changes
6. **Follow conventions**: Use established patterns from existing components

### Development Guidelines

Follow the comprehensive rules in [AGENTS.md](AGENTS.md):

- Use `lowercase_with_underscores` for filenames
- Use `.yaml` extension (not `.yml`)
- Use `.md` extension for markdown (not `.MD` or `.markdown`)
- No emojis in code or documentation
- All code must pass `cargo fmt` and `cargo clippy`
- Maintain >80% test coverage
- Document all public APIs with doc comments
- Include runnable examples in documentation

### Submitting Changes

1. Create branch: `pr-{jira-issue}` (lowercase)
2. Make changes following all guidelines
3. Run complete pre-commit workflow
4. Write descriptive commit messages
5. Submit pull request with clear description
6. Ensure CI passes all checks

Commit message format:

```
<type>(<scope>): <description> (JIRA-ISSUE)

type: feat|fix|docs|style|refactor|perf|test|chore
scope: component|validator|cli|docs|etc
```

**Examples**:

```
feat(validator): add tier-aware size validation (PROJ-1234)
fix(parser): handle edge case in marker balance check (PROJ-5678)
docs(how-to): add migration examples for v2 upgrade (PROJ-9012)
```

## Versioning

This project uses [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes to component format or public API
- **MINOR**: New features, backward compatible additions
- **PATCH**: Bug fixes, documentation updates, minor improvements

**Current version**: 2.0.0

**Version history**:

- v2.0.0 (2024-12-19): Language-agnostic component system with full validation
- v1.0.0 (2024-11-01): Initial release with basic component management

## Size Limits

Components must stay within these limits to ensure focused, maintainable documentation:

| Category  | Tier          | Max Lines | Warning @ | Components |
| --------- | ------------- | --------- | --------- | ---------- |
| core      | -             | 500       | 400 (80%) | 5          |
| general   | -             | 800       | 640 (80%) | 8          |
| languages | -             | 600       | 480 (80%) | 4          |
| tools     | essential     | 300       | 240 (80%) | 6          |
| tools     | comprehensive | 800       | 640 (80%) | 6          |

**Project-wide limit**: 10,000 lines total across all components
**Warning threshold**: 8,000 lines (80%)

Line counting excludes:

- Blank lines
- YAML frontmatter (between `---` markers)
- Whitespace-only lines

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Project Status

### Implementation Phases

- **Phase 1**: Complete - Foundation and Parser (YAML frontmatter, language markers)
- **Phase 2**: Complete - Core Component Refactoring (5 core components migrated)
- **Phase 3**: Complete - General Component Refactoring (8 general components migrated)
- **Phase 4**: Complete - Tool Component Tiering (12 tool components with tiers)
- **Phase 5**: Complete - Size Enforcement and Validation (automated validation, CI integration)
- **Phase 6**: Complete - Documentation and Release (comprehensive guides, examples, v2.0.0)

### Current Status

- **Version**: 2.0.0 - Production Ready
- **Tests**: 621 passing (100% success rate)
- **Coverage**: >80% across all modules
- **Components**: 29 components across 4 categories
- **Documentation**: Complete (4,500+ lines)
- **Quality Gates**: All passing
- **CI/CD**: Fully automated validation

## Statistics

### Code Statistics

- **Implementation code**: ~10,000 lines
- **Component library**: ~15,000 lines
- **Tests**: ~2,500 lines (621 tests)
- **Documentation**: ~6,200 lines
- **Total project**: ~33,700 lines

### Component Statistics

- **Total components**: 29
- **Core**: 5 components (~1,754 lines)
- **General**: 8 components (~5,000 lines)
- **Languages**: 4 components (~2,200 lines)
- **Tools**: 12 components (~6,046 lines)

### Quality Metrics

- **Test coverage**: >80%
- **Clippy warnings**: 0
- **Components within limits**: 26/29 (90%)
- **Components with warnings**: 3/29 (10%)
- **Components exceeding limits**: 0/29 (0%)

## Resources

- **Repository**: https://github.com/xbcsmith/xzagentz
- **Issues**: https://github.com/xbcsmith/xzagentz/issues
- **CI/CD**: https://github.com/xbcsmith/xzagentz/actions
- **Documentation**: https://github.com/xbcsmith/xzagentz/tree/main/docs
- **Releases**: https://github.com/xbcsmith/xzagentz/releases

## Acknowledgments

Built with Rust and designed for high-performance component management with language-agnostic rendering capabilities. Implements the Diataxis documentation framework for comprehensive, maintainable documentation.

Special thanks to the Rust community for excellent tooling and the open-source ecosystem for inspiration and support.
