# xzagentz

[![Component Validation](https://github.com/xbcsmith/xzagentz/workflows/Component%20Validation/badge.svg)](https://github.com/xbcsmith/xzagentz/actions)
[![Rust](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance Rust CLI tool for managing language-agnostic development components with intelligent rendering and validation.

## Overview

xzagentz provides a structured system for authoring, validating, and rendering development guidance components that adapt to multiple programming languages. Components are reusable documentation units with language-specific sections, automatic size validation, and tier-based complexity management.

### Key Features

- **Language-Agnostic Components**: Write once, render for multiple languages (Rust, Python, Go, TypeScript, Bash)
- **Intelligent Rendering**: Automatic language-specific content extraction with fallback to universal content
- **Size Enforcement**: Category-based line limits ensure components remain focused and maintainable
- **Tier System**: Essential vs comprehensive tool components for different use cases
- **Automated Validation**: CI-driven validation of frontmatter, markers, and size constraints
- **Type-Safe Architecture**: Leverages Rust's type system for robust validation and error handling

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
```

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
```

### Component Categories

| Category      | Description                       | Size Limit    | Use Case                              |
| ------------- | --------------------------------- | ------------- | ------------------------------------- |
| **core**      | Fundamental development practices | 500 lines     | Error handling, testing, code quality |
| **general**   | Project management and workflow   | 800 lines     | Git, documentation, CI/CD             |
| **languages** | Language-specific features        | 600 lines     | Language idioms and patterns          |
| **tools**     | Tool-specific guidance            | 300-800 lines | Docker, Kubernetes, etc.              |

### Tool Tiers

Tool components support two complexity tiers:

- **Essential** (300 lines): Core commands and common workflows for daily use
- **Comprehensive** (800 lines): Complete reference with advanced features and edge cases

## Architecture

### Project Structure

```
xzagentz/
├── src/
│   ├── lib.rs              # Library root
│   ├── error.rs            # Error types
│   ├── parser/             # Component parsing
│   │   ├── mod.rs
│   │   ├── frontmatter.rs  # YAML frontmatter parsing
│   │   └── markers.rs      # Language marker handling
│   ├── renderer/           # Component rendering
│   │   └── mod.rs
│   └── validator/          # Validation system
│       ├── mod.rs
│       └── size.rs         # Size validation
├── components/             # Component library
│   ├── core/               # Core components
│   ├── general/            # General components
│   ├── languages/          # Language components
│   └── tools/              # Tool components
├── tests/                  # Integration tests
├── examples/               # Example components
└── docs/                   # Documentation
    ├── how_to/             # Task-oriented guides
    ├── reference/          # Technical specifications
    └── explanations/       # Conceptual documentation
```

### Layered Architecture

```
┌─────────────────────────────────────┐
│  CLI Interface                      │
├─────────────────────────────────────┤
│  Validation Layer                   │
│  - Size validation                  │
│  - Frontmatter validation           │
│  - Marker validation                │
├─────────────────────────────────────┤
│  Rendering Layer                    │
│  - Language-specific rendering      │
│  - Universal content fallback       │
├─────────────────────────────────────┤
│  Parser Layer                       │
│  - YAML frontmatter parsing         │
│  - Language marker extraction       │
├─────────────────────────────────────┤
│  Domain Layer                       │
│  - Component models                 │
│  - Validation rules                 │
└─────────────────────────────────────┘
```

## Documentation

### For Users

- **[Authoring Components](docs/how_to/authoring_components.md)**: Complete guide to creating components
- **[Migrating to v2](docs/how_to/migrating_to_v2.md)**: Upgrade guide from legacy components
- **[Configuration Reference](docs/reference/component_configuration.md)**: Complete configuration documentation
- **[Troubleshooting](docs/reference/troubleshooting.md)**: Common issues and solutions

### For Developers

- **[Implementation Plan](docs/explanations/language_agnostic_component_system_implementation_plan.md)**: Architecture and design decisions
- **[Phase 5 Implementation](docs/explanations/phase_5_size_enforcement_implementation.md)**: Size validation details
- **[AGENTS.md](AGENTS.md)**: AI agent development guidelines

### Examples

See `examples/components/` for reference implementations:

- `core_example.md` - Core category component
- `tool_essential_example.md` - Essential tier tool
- `tool_comprehensive_example.md` - Comprehensive tier tool

## Development

### Building from Source

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check --all-targets --all-features
```

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test size_validation_test

# Run with output
cargo test -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Security audit (requires cargo-audit)
cargo audit
```

### Pre-Commit Workflow

Before committing changes:

1. Format code: `cargo fmt --all`
2. Check compilation: `cargo check --all-targets --all-features`
3. Run linter: `cargo clippy --all-targets --all-features -- -D warnings`
4. Run tests: `cargo test --all-features`
5. Validate components: `cargo test size_validation_test`

## CI/CD

### GitHub Actions

Automated validation runs on every pull request and push to main:

- **Frontmatter Validation**: Ensures YAML is valid and complete
- **Marker Validation**: Verifies language markers are balanced
- **Size Validation**: Enforces category and project-wide limits
- **Test Suite**: Runs all unit and integration tests
- **Validation Report**: Generates size report artifact

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
      - run: cargo test --all-features
      - run: cargo test size_validation_test
```

## Contributing

### Component Guidelines

1. **Stay within size limits**: Check limits for your category
2. **Use proper frontmatter**: Include all required fields
3. **Balance markers**: Every `<!-- LANG:X -->` needs `<!-- /LANG:X -->`
4. **Write tests**: Add tests for new functionality
5. **Update documentation**: Keep docs in sync with changes

### Development Guidelines

- Follow the rules in [AGENTS.md](AGENTS.md)
- Use lowercase_with_underscores for filenames
- Use `.yaml` extension (not `.yml`)
- No emojis in code or documentation
- All code must pass `cargo fmt` and `cargo clippy`
- Maintain >80% test coverage

### Submitting Changes

1. Create branch: `pr-{jira-issue}` (lowercase)
2. Make changes following guidelines
3. Run quality checks (see Pre-Commit Workflow)
4. Write descriptive commit messages
5. Submit pull request

Commit message format:

```
<type>(<scope>): <description> (JIRA-ISSUE)

type: feat|fix|docs|style|refactor|perf|test|chore
```

## Versioning

This project uses [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes to component format or API
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes and documentation updates

Current version: **2.0.0**

## Size Limits

Components must stay within these limits:

| Category  | Tier          | Max Lines | Warning @ |
| --------- | ------------- | --------- | --------- |
| core      | -             | 500       | 400 (80%) |
| general   | -             | 800       | 640 (80%) |
| languages | -             | 600       | 480 (80%) |
| tools     | essential     | 300       | 240 (80%) |
| tools     | comprehensive | 800       | 640 (80%) |

**Project-wide limit**: 10,000 lines total across all components

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Project Status

- **Phase 1-4**: Complete (Parser, Refactoring, Tool Tiers)
- **Phase 5**: Complete (Size Enforcement and Validation)
- **Phase 6**: Complete (Documentation and Release)
- **Version**: 2.0.0 - Production Ready

## Resources

- **Repository**: https://github.com/xbcsmith/xzagentz
- **Issues**: https://github.com/xbcsmith/xzagentz/issues
- **CI/CD**: https://github.com/xbcsmith/xzagentz/actions

## Acknowledgments

Built with Rust and designed for high-performance component management with language-agnostic rendering capabilities.
