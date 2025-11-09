# How to Author Components

This guide explains how to create and maintain components for the xzagentz system.

## Overview

Components are reusable documentation units that provide language-specific guidance. Each component consists of YAML frontmatter (metadata) and content with language-specific sections.

## Component Structure

### Basic Template

Every component file has this structure:

```markdown
---
component:
  name: component_name
  category: core|general|languages|tools
  version: 2.0.0
  tier: essential|comprehensive  # Only for tools category
  description: Brief description of the component
  languages:
    - rust
    - python
    - golang
    - typescript
  sections:
    - id: section_identifier
      language_specific: true|false
      required: true|false
    - id: another_section
      language_specific: true
      required: true
---

# Component Title

## Universal Section

Content that applies to all languages goes here.

---

## Language-Specific Section

<!-- LANG:rust -->

Rust-specific content here.

<!-- /LANG:rust -->

<!-- LANG:python -->

Python-specific content here.

<!-- /LANG:python -->
```

## Step-by-Step Authoring Process

### Step 1: Choose the Correct Category

Select the appropriate category for your component:

- **core**: Fundamental development practices (error handling, testing, code quality)
- **general**: Project management and workflow (git, documentation, CI/CD)
- **languages**: Language-specific features and idioms
- **tools**: Tool-specific guidance (git, markdown, kubernetes, docker)

### Step 2: Create the File

Place your component in the correct directory:

```bash
# Core components
components/core/your_component.md

# General components
components/general/your_component.md

# Language-specific components
components/languages/your_component.md

# Tool components (with tier)
components/tools/tool_name_essential.md
components/tools/tool_name_comprehensive.md
```

### Step 3: Write the Frontmatter

The frontmatter defines component metadata:

```yaml
---
component:
  name: your_component_name          # Unique identifier (lowercase_with_underscores)
  category: core                      # Category (see Step 1)
  version: 2.0.0                      # Version (use semantic versioning)
  tier: essential                     # Only for tools: essential or comprehensive
  description: What this component provides  # One-line description
  languages:                          # Supported languages
    - rust
    - python
    - golang
    - typescript
    - bash
  sections:                           # Define content sections
    - id: principles                  # Section identifier
      language_specific: false        # Universal or language-specific
      required: true                  # Whether section is mandatory
    - id: patterns
      language_specific: true         # This section has per-language content
      required: true
---
```

### Step 4: Write Universal Content

Start with content that applies to all languages:

```markdown
# Component Title

## Core Principles

Universal principles that apply regardless of language:

1. Principle one
2. Principle two
3. Principle three

Key concepts that every developer should understand.
```

### Step 5: Write Language-Specific Content

Use language markers to separate content by language:

```markdown
---

## Language-Specific Patterns

<!-- LANG:rust -->

### Rust Implementation

Rust-specific guidance and examples:

```rust
pub fn example() -> Result<(), Error> {
    // Rust code example
    Ok(())
}
```

Key Rust considerations:
- Use Result for error handling
- Leverage the type system
- Follow Rust idioms

<!-- /LANG:rust -->

<!-- LANG:python -->

### Python Implementation

Python-specific guidance:

```python
def example() -> None:
    """Python code example."""
    pass
```

Key Python considerations:
- Use type hints
- Follow PEP 8
- Use context managers

<!-- /LANG:python -->
```

### Step 6: Validate Your Component

Run validation to ensure your component meets requirements:

```bash
# Format your code
cargo fmt --all

# Run validation tests
cargo test

# Run the CI validation locally
cargo test size_validation_test

# Check specific component
cargo run -- validate components/core/your_component.md
```

## Size Limits by Category

Stay within these limits to ensure components remain focused:

| Category | Tier | Maximum Lines | Warning Threshold |
|----------|------|---------------|-------------------|
| core | - | 500 | 400 (80%) |
| general | - | 800 | 640 (80%) |
| languages | - | 600 | 480 (80%) |
| tools | essential | 300 | 240 (80%) |
| tools | comprehensive | 800 | 640 (80%) |

**Total project limit**: 10,000 lines across all components

To check your component size:

```bash
# Count non-empty, non-comment lines
grep -v '^[[:space:]]*$' components/core/your_component.md | \
  grep -v '^[[:space:]]*<!--' | \
  wc -l
```

## Best Practices

### 1. Keep Components Focused

Each component should address a single topic:

- **Good**: `error_handling.md` covers error handling patterns
- **Bad**: `coding_standards.md` tries to cover everything

### 2. Provide Complete Examples

Always include runnable, realistic examples:

```rust
// BAD: Incomplete example
fn process() {
    // ...
}

// GOOD: Complete example with context
use std::fs::File;
use std::io::Read;

pub fn process_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### 3. Use Consistent Section IDs

Section IDs should be:
- Lowercase with underscores
- Descriptive of content
- Consistent across components

Good section IDs:
- `core_principles`
- `error_patterns`
- `testing_strategies`
- `best_practices`

### 4. Mark Language Specificity Correctly

Set `language_specific: true` only when content differs by language:

```yaml
sections:
  # Universal principles - same for all languages
  - id: principles
    language_specific: false
    required: true

  # Implementation patterns - different per language
  - id: patterns
    language_specific: true
    required: true
```

### 5. Balance Essential vs Comprehensive

For tool components, distinguish between tiers:

**Essential tier** (300 lines max):
- Core commands only
- Most common workflows
- Critical safety practices
- Minimal but sufficient

**Comprehensive tier** (800 lines max):
- Advanced features
- Edge cases
- Performance optimization
- Complete reference

### 6. Write Clear Descriptions

Component descriptions should be concise and informative:

- **Good**: "Error handling standards and patterns for robust code"
- **Bad**: "Errors" or "How to handle errors and exceptions and failures"

### 7. Keep Examples Realistic

Examples should reflect real-world usage:

```rust
// BAD: Toy example
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// GOOD: Realistic example
pub fn validate_config(config: &Config) -> Result<(), ConfigError> {
    if config.port < 1024 {
        return Err(ConfigError::InvalidPort(config.port));
    }
    if config.timeout_ms == 0 {
        return Err(ConfigError::InvalidTimeout);
    }
    Ok(())
}
```

## Language Markers Reference

### Supported Languages

Use these exact marker names:

- `<!-- LANG:rust -->`
- `<!-- LANG:python -->`
- `<!-- LANG:golang -->`
- `<!-- LANG:typescript -->`
- `<!-- LANG:bash -->`

### Marker Rules

1. **Always close markers**: Every `<!-- LANG:X -->` needs `<!-- /LANG:X -->`
2. **No nesting**: Language sections cannot be nested
3. **Exact spelling**: Use exact language names (case-sensitive)
4. **Whitespace**: Markers must be on their own line

Valid:
```markdown
<!-- LANG:rust -->
Content here
<!-- /LANG:rust -->
```

Invalid:
```markdown
<!-- LANG:Rust --> (wrong case)
<!-- LANG:rust --> content on same line
<!-- lang:rust --> (wrong case)
```

## Common Mistakes to Avoid

### 1. Missing Closing Markers

```markdown
<!-- LANG:rust -->
Content here
<!-- Forgot closing marker! -->

<!-- LANG:python -->
This will cause validation errors
```

### 2. Exceeding Size Limits

If your component exceeds limits:
- Split into multiple focused components
- Move some content to reference documentation
- For tools, move advanced content to comprehensive tier

### 3. Language-Agnostic Content in Language Sections

```markdown
<!-- LANG:rust -->
# Bad: This principle applies to all languages
Always validate user input before processing.
<!-- /LANG:rust -->

# Good: Put universal content outside language sections
Always validate user input before processing.

<!-- LANG:rust -->
Use the validator crate for input validation.
<!-- /LANG:rust -->
```

### 4. Inconsistent Frontmatter

```yaml
# BAD: Inconsistent formatting
component:
  name: My Component  # Should be lowercase_with_underscores
  Category: Core      # Should be lowercase
  VERSION: "2.0"      # Should be lowercase key, proper semver

# GOOD: Consistent formatting
component:
  name: my_component
  category: core
  version: 2.0.0
```

## Testing Your Component

### Automated Validation

The CI system validates:
- YAML frontmatter presence and correctness
- Required fields (name, category, version, description)
- Language marker balance (all opened markers are closed)
- Size limits per category and tier
- Total project size limit

### Manual Testing

1. **Render test**: Ensure component renders correctly
   ```bash
   cargo run -- render --component your_component --language rust
   ```

2. **Validation test**: Check for errors
   ```bash
   cargo test -- your_component
   ```

3. **Size check**: Verify you're within limits
   ```bash
   cargo test size_validation_test
   ```

## Updating Existing Components

### Making Changes

1. Update the version in frontmatter (semantic versioning)
2. Make your content changes
3. Update language sections as needed
4. Run validation
5. Update related documentation

### Version Bumping Guidelines

- **Major version** (2.0.0 → 3.0.0): Breaking changes to structure or content
- **Minor version** (2.0.0 → 2.1.0): New sections or languages added
- **Patch version** (2.0.0 → 2.0.1): Corrections, clarifications, examples

## Getting Help

### Validation Errors

If validation fails, check:
1. YAML frontmatter syntax (use a YAML validator)
2. All required fields present
3. Language markers balanced and closed
4. Component size within limits
5. File in correct category directory

### Examples and Templates

See `examples/components/` for reference implementations:
- `examples/components/core_example.md`
- `examples/components/tool_essential_example.md`
- `examples/components/tool_comprehensive_example.md`

### Documentation

- **Architecture**: `docs/explanation/language_agnostic_component_system_implementation_plan.md`
- **Configuration**: `docs/reference/component_configuration.md`
- **Troubleshooting**: `docs/reference/troubleshooting.md`
- **Migration**: `docs/how_to/migrating_to_v2.md`

## Summary

To create a component:

1. Choose the correct category and tier
2. Create file in the appropriate directory
3. Write YAML frontmatter with metadata
4. Write universal content first
5. Add language-specific sections with markers
6. Stay within size limits
7. Validate before committing
8. Document in appropriate Diataxis category

Follow these guidelines to create high-quality, maintainable components that serve developers across multiple languages and tools.
