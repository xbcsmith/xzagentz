# How to Implement Language-Agnostic Components

## Quick Start Guide

This guide walks you through the steps to begin implementing the language-agnostic component system for xzagentz. Follow these steps in order to start Phase 1 of the implementation.

## Prerequisites

Before starting, ensure you have:

- Rust toolchain installed (1.70 or later)
- Read the implementation plan: `docs/explanation/language_agnostic_component_system_implementation_plan.md`
- Read the executive summary: `docs/explanation/language_agnostic_components_summary.md`
- Reviewed the project rules in `AGENTS.md`

## Step 1: Set Up Feature Branch

```bash
# Create and switch to feature branch
git checkout -b pr-xzagentz-lang-agnostic

# Verify you're on the correct branch
git branch --show-current
```

## Step 2: Add Required Dependencies

Update `Cargo.toml` with any missing dependencies:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
thiserror = "1.0"
tracing = "0.1"
regex = "1.10"
lazy_static = "1.4"
```

Run dependency check:

```bash
cargo check
```

## Step 3: Create New Module Files

Create the three new module files:

```bash
# Create language filter module
touch src/components/language_filter.rs

# Create renderer module
touch src/components/renderer.rs

# Create metadata module
touch src/components/metadata.rs
```

## Step 4: Update Module Declarations

Edit `src/components/mod.rs` to include the new modules:

```rust
pub mod language_filter;
pub mod metadata;
pub mod renderer;

pub use language_filter::{LanguageSection, parse_language_sections};
pub use metadata::{ComponentMetadata, SectionMetadata};
pub use renderer::{RenderConfig, render_component};
```

## Step 5: Implement Metadata Module First

Start with `src/components/metadata.rs` as it has no dependencies:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for field {field}: {value}")]
    InvalidValue { field: String, value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    pub name: String,
    pub category: String,
    pub version: String,
    pub tier: Option<String>,
    pub description: Option<String>,
    pub languages: Option<Vec<String>>,
    pub sections: Option<Vec<SectionMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMetadata {
    pub id: String,
    pub language_specific: bool,
    pub languages: Option<Vec<String>>,
    pub required: Option<bool>,
}

impl ComponentMetadata {
    pub fn validate(&self) -> Result<(), MetadataError> {
        // Validate category
        let valid_categories = ["core", "general", "languages", "tools"];
        if !valid_categories.contains(&self.category.as_str()) {
            return Err(MetadataError::InvalidValue {
                field: "category".to_string(),
                value: self.category.clone(),
            });
        }

        // Validate tier if present
        if let Some(tier) = &self.tier {
            let valid_tiers = ["essential", "comprehensive"];
            if !valid_tiers.contains(&tier.as_str()) {
                return Err(MetadataError::InvalidValue {
                    field: "tier".to_string(),
                    value: tier.clone(),
                });
            }
        }

        Ok(())
    }
}
```

## Step 6: Write Tests for Metadata Module

Add tests at the bottom of `src/components/metadata.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_validation_valid() {
        let metadata = ComponentMetadata {
            name: "test".to_string(),
            category: "core".to_string(),
            version: "1.0.0".to_string(),
            tier: None,
            description: None,
            languages: None,
            sections: None,
        };

        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn test_metadata_validation_invalid_category() {
        let metadata = ComponentMetadata {
            name: "test".to_string(),
            category: "invalid".to_string(),
            version: "1.0.0".to_string(),
            tier: None,
            description: None,
            languages: None,
            sections: None,
        };

        assert!(metadata.validate().is_err());
    }
}
```

## Step 7: Run Quality Gates

After each module implementation, run all quality checks:

```bash
# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Run linter (treat warnings as errors)
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features
```

All commands must pass before proceeding.

## Step 8: Implement Language Filter Module

Next, implement `src/components/language_filter.rs`:

```rust
use regex::Regex;
use lazy_static::lazy_static;
use thiserror::Error;

lazy_static! {
    static ref LANG_START: Regex = Regex::new(r"^<!--\s*LANG:(\w+|\*)\s*-->$").unwrap();
    static ref LANG_END: Regex = Regex::new(r"^<!--\s*/LANG\s*-->$").unwrap();
}

#[derive(Error, Debug)]
pub enum LanguageFilterError {
    #[error("Invalid marker at line {line}: {marker}")]
    InvalidMarker { line: usize, marker: String },

    #[error("Unclosed section starting at line {0}")]
    UnclosedSection(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LanguageSection {
    pub language: String,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
}

pub fn parse_language_sections(
    content: &str,
) -> Result<Vec<LanguageSection>, LanguageFilterError> {
    let mut sections = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if let Some(captures) = LANG_START.captures(line) {
            let language = captures.get(1).unwrap().as_str().to_string();
            let start_line = i + 1;

            // Find matching end marker
            let mut section_content = Vec::new();
            i += 1;

            while i < lines.len() {
                let current = lines[i].trim();
                if LANG_END.is_match(current) {
                    sections.push(LanguageSection {
                        language,
                        content: section_content.join("\n"),
                        start_line,
                        end_line: i,
                    });
                    break;
                }
                section_content.push(lines[i]);
                i += 1;
            }

            if i >= lines.len() {
                return Err(LanguageFilterError::UnclosedSection(start_line));
            }
        }

        i += 1;
    }

    Ok(sections)
}
```

## Step 9: Write Tests for Language Filter

Add comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_section() {
        let content = r#"
<!-- LANG:rust -->
Rust content
<!-- /LANG -->
"#;

        let sections = parse_language_sections(content).unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].language, "rust");
        assert!(sections[0].content.contains("Rust content"));
    }

    #[test]
    fn test_parse_multiple_sections() {
        let content = r#"
<!-- LANG:rust -->
Rust content
<!-- /LANG -->

<!-- LANG:python -->
Python content
<!-- /LANG -->
"#;

        let sections = parse_language_sections(content).unwrap();
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].language, "rust");
        assert_eq!(sections[1].language, "python");
    }

    #[test]
    fn test_parse_agnostic_section() {
        let content = r#"
<!-- LANG:* -->
Agnostic content
<!-- /LANG -->
"#;

        let sections = parse_language_sections(content).unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].language, "*");
    }

    #[test]
    fn test_unclosed_section() {
        let content = r#"
<!-- LANG:rust -->
Rust content
"#;

        let result = parse_language_sections(content);
        assert!(result.is_err());
    }
}
```

## Step 10: Track Progress

Use the checklist to track your progress:

```bash
# Open the checklist
open docs/explanation/language_agnostic_components_checklist.md
```

Mark tasks as complete as you finish them.

## Step 11: Commit Your Work

After each module is complete with tests passing:

```bash
# Add changes
git add .

# Commit with proper format
git commit -m "feat(components): implement metadata module (xzagentz-lang-agnostic)

Add ComponentMetadata and SectionMetadata structs with YAML
serialization support. Implement validation for metadata fields."

# Push to remote
git push origin pr-xzagentz-lang-agnostic
```

## Common Issues and Solutions

### Issue: Clippy warnings about unused code

**Solution**: This is expected during early implementation. Add `#[allow(dead_code)]` temporarily, but remove before final review.

### Issue: Tests fail with "cannot find type in scope"

**Solution**: Ensure all modules are properly declared in `mod.rs` and imports are correct.

### Issue: Regex compilation errors

**Solution**: Verify regex patterns are valid. Use https://regex101.com to test patterns.

## Next Steps

After completing the metadata and language_filter modules:

1. Implement the renderer module
2. Update the component loader
3. Write integration tests
4. Move to Phase 2: Component refactoring

## Getting Help

- Review the detailed implementation plan: `docs/explanation/language_agnostic_component_system_implementation_plan.md`
- Check the component improvement analysis: `docs/explanation/component_improvement_plan.md`
- Review project rules: `AGENTS.md`
- Use the tracking checklist: `docs/explanation/language_agnostic_components_checklist.md`

## Quality Checklist

Before considering Phase 1 complete, verify:

- [ ] All three modules implemented (metadata, language_filter, renderer)
- [ ] All unit tests pass
- [ ] Integration tests written and passing
- [ ] Test coverage greater than 80%
- [ ] No clippy warnings
- [ ] All code formatted
- [ ] All public APIs documented
- [ ] Quality gates pass

---

**Document Version**: 1.0.0
**Last Updated**: 2024
**Related Documents**:
- `docs/explanation/language_agnostic_component_system_implementation_plan.md`
- `docs/explanation/language_agnostic_components_checklist.md`
- `AGENTS.md`
