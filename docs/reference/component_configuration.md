# Component Configuration Reference

Complete reference for configuring the xzagentz component system.

## Overview

The component system uses two configuration levels:

1. **Component-level configuration**: YAML frontmatter in each component file
2. **Project-level configuration**: Global settings for rendering and validation

## Component Frontmatter Schema

### Required Fields

Every component must include these fields in YAML frontmatter:

```yaml
---
component:
  name: string # Component identifier
  category: string # Component category
  version: string # Semantic version
  description: string # Brief description
  languages: array # Supported languages
  sections: array # Content structure
---
```

### Field Specifications

#### name

**Type**: `string`

**Format**: Lowercase with underscores

**Description**: Unique identifier for the component

**Rules**:

- Must be unique within category
- Only lowercase letters, numbers, and underscores
- No spaces or special characters
- Should be descriptive and concise

**Examples**:

```yaml
name: error_handling          # Good
name: git_essential           # Good
name: ErrorHandling           # Bad: CamelCase
name: error-handling          # Bad: hyphens
name: error handling          # Bad: spaces
```

#### category

**Type**: `string`

**Valid values**: `core`, `general`, `languages`, `tools`

**Description**: Classification determining size limits and organization

**Categories**:

- `core`: Fundamental development practices (error handling, testing, etc.)
- `general`: Project management and workflow (git, documentation, CI/CD)
- `languages`: Language-specific features and idioms
- `tools`: Tool-specific guidance with tier support

**Example**:

```yaml
category: core
```

#### version

**Type**: `string`

**Format**: Semantic versioning (MAJOR.MINOR.PATCH)

**Description**: Component version for tracking changes

**Rules**:

- Must follow semver format: `X.Y.Z`
- Start at `2.0.0` for v2 components
- Increment based on change type:
  - MAJOR: Breaking changes
  - MINOR: New features/sections
  - PATCH: Bug fixes, clarifications

**Examples**:

```yaml
version: 2.0.0                # Initial v2 version
version: 2.1.0                # Added new section
version: 2.1.1                # Fixed typo
```

#### tier

**Type**: `string`

**Valid values**: `essential`, `comprehensive`

**Required**: Only for `category: tools`

**Description**: Tool complexity tier affecting size limits

**Tiers**:

- `essential`: Core functionality only (300 line limit)
- `comprehensive`: Complete reference (800 line limit)

**Example**:

```yaml
category: tools
tier: essential # Required for tools
```

#### description

**Type**: `string`

**Max length**: ~100 characters (one line)

**Description**: Brief summary of component purpose

**Rules**:

- Single sentence or short phrase
- Descriptive and specific
- No marketing language
- Avoid redundancy with name

**Examples**:

```yaml
description: Error handling standards and patterns for robust code
description: Essential Git commands and workflows
description: Kubernetes deployment and management guide
```

#### languages

**Type**: `array` of `string`

**Valid values**: `rust`, `python`, `golang`, `typescript`, `bash`

**Description**: Languages supported by this component

**Rules**:

- List all languages with content in component
- Must match language markers in content
- Order doesn't matter
- At least one language required

**Example**:

```yaml
languages:
  - rust
  - python
  - golang
  - typescript
```

#### sections

**Type**: `array` of `object`

**Description**: Defines content structure and requirements

**Section object**:

```yaml
sections:
  - id: string # Section identifier
    language_specific: bool # Whether content varies by language
    required: bool # Whether section must exist
```

**Section fields**:

- `id`: Unique identifier (lowercase_with_underscores)
- `language_specific`:
  - `true`: Section has language markers and per-language content
  - `false`: Section is universal (same for all languages)
- `required`:
  - `true`: Component invalid without this section
  - `false`: Section is optional

**Example**:

```yaml
sections:
  - id: principles
    language_specific: false
    required: true
  - id: implementation_patterns
    language_specific: true
    required: true
  - id: advanced_techniques
    language_specific: true
    required: false
```

### Complete Frontmatter Example

```yaml
---
component:
  name: error_handling
  category: core
  version: 2.1.0
  description: Error handling standards and patterns for robust code
  languages:
    - rust
    - python
    - golang
    - typescript
  sections:
    - id: core_principles
      language_specific: false
      required: true
    - id: error_types
      language_specific: true
      required: true
    - id: error_patterns
      language_specific: true
      required: true
    - id: context_propagation
      language_specific: true
      required: true
    - id: testing_errors
      language_specific: true
      required: false
---
```

## Language Markers

### Syntax

**Opening marker**:

```markdown
<!-- LANG:language_name -->
```

**Closing marker**:

```markdown
<!-- /LANG:language_name -->
```

### Supported Languages

| Marker            | Language   | Use Case                      |
| ----------------- | ---------- | ----------------------------- |
| `LANG:rust`       | Rust       | System programming, CLI tools |
| `LANG:python`     | Python     | Scripting, data processing    |
| `LANG:golang`     | Go         | Services, concurrent systems  |
| `LANG:typescript` | TypeScript | Web applications, Node.js     |
| `LANG:bash`       | Bash       | Shell scripts, automation     |

### Marker Rules

1. **Case sensitive**: Use exact spelling (`LANG:rust`, not `LANG:Rust`)
2. **Own line**: Markers must be on their own line
3. **Balanced**: Every opening marker needs a closing marker
4. **No nesting**: Cannot nest language sections
5. **Whitespace**: No extra spaces inside marker syntax

**Valid**:

```markdown
<!-- LANG:rust -->

Content here

<!-- /LANG:rust -->
```

**Invalid**:

```markdown
<!-- LANG:Rust -->                    # Wrong: capital R
<!--LANG:rust-->                      # Wrong: no spaces
<!-- LANG:rust --> content here       # Wrong: content on same line
<!-- lang:rust -->                    # Wrong: lowercase lang
```

### Usage Example

````markdown
## Error Handling Patterns

Universal content here applies to all languages.

---

<!-- LANG:rust -->

### Rust Error Handling

Rust-specific content and examples.

```rust
pub fn example() -> Result<(), Error> {
    Ok(())
}
```
````

<!-- /LANG:rust -->

<!-- LANG:python -->

### Python Error Handling

Python-specific content and examples.

```python
def example() -> None:
    try:
        pass
    except Exception as e:
        raise
```

<!-- /LANG:python -->

````

## Size Limits

### Per-Category Limits

Size limits enforce focused, maintainable components.

| Category | Tier | Max Lines | Warning Threshold |
|----------|------|-----------|-------------------|
| core | - | 500 | 400 (80%) |
| general | - | 800 | 640 (80%) |
| languages | - | 600 | 480 (80%) |
| tools | essential | 300 | 240 (80%) |
| tools | comprehensive | 800 | 640 (80%) |

### Project-Wide Limit

**Total limit**: 10,000 lines across all components

### Line Counting Rules

Lines counted:
- All content lines
- Code examples
- Markdown formatting
- Language markers

Lines excluded:
- Empty lines
- YAML frontmatter (between `---` markers)
- Comments outside language sections

### Size Validation

Validation runs automatically in CI:

```bash
# Check specific component
cargo test -- component_name

# Check all components
cargo test size_validation_test

# Generate size report
cargo run -- validate --report
````

### Exceeding Limits

If component exceeds limits:

1. **Split component**: Divide into multiple focused components
2. **Extract content**: Move details to reference documentation
3. **Tier split** (tools only): Create essential and comprehensive versions
4. **Remove redundancy**: Eliminate duplicate or outdated content

## Project Configuration

### Configuration File

Location: `config/project.yaml`

```yaml
project:
  type: cli # Project type
  language: rust # Primary language

components:
  tools:
    git: essential # Tool tier selection
    kubernetes: comprehensive
    docker: essential
  include:
    - core/* # Include patterns
    - general/*
  exclude:
    - tools/deprecated_* # Exclude patterns

limits:
  max_total_lines: 10000 # Total project limit
  warn_at_lines: 8000 # Warning threshold
  per_component_max:
    core: 500
    general: 800
    languages: 600
    tools_essential: 300
    tools_comprehensive: 800

rendering:
  strict_language: true # Enforce exact language match
  fallback_to_agnostic: true # Fall back to universal content
  include_metadata: false # Include frontmatter in output
```

### Configuration Fields

#### project.type

**Valid values**: `cli`, `library`, `service`, `docs`

**Default**: `cli`

**Description**: Project type affecting component selection

#### project.language

**Valid values**: `rust`, `python`, `golang`, `typescript`, `bash`

**Description**: Primary project language for rendering

#### components.tools

**Description**: Tool tier selection

**Format**: `tool_name: essential|comprehensive|none`

**Example**:

```yaml
components:
  tools:
    git: essential
    kubernetes: comprehensive
    docker: none
```

#### components.include

**Type**: Array of glob patterns

**Description**: Components to include

**Example**:

```yaml
include:
  - core/*
  - general/*
  - tools/git_*
```

#### components.exclude

**Type**: Array of glob patterns

**Description**: Components to exclude

**Example**:

```yaml
exclude:
  - tools/deprecated_*
  - languages/legacy_*
```

#### limits

**Description**: Size enforcement configuration

**Fields**:

- `max_total_lines`: Project-wide line limit
- `warn_at_lines`: Warning threshold (typically 80% of max)
- `per_component_max`: Override per-category limits

#### rendering

**Description**: Rendering behavior configuration

**Fields**:

- `strict_language`: Require exact language match
- `fallback_to_agnostic`: Use universal content if language not found
- `include_metadata`: Include YAML frontmatter in rendered output

## Validation Configuration

### CI Validation Workflow

Location: `.github/workflows/component_validation.yaml`

```yaml
name: Component Validation

on:
  pull_request:
  push:
    branches: [main]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Validate frontmatter
        run: cargo test frontmatter_validation
      - name: Validate markers
        run: cargo test marker_validation
      - name: Validate sizes
        run: cargo test size_validation_test
      - name: Generate report
        run: cargo run -- validate --report
      - uses: actions/upload-artifact@v3
        with:
          name: validation-report
          path: validation_report.md
```

### Validation Checks

1. **Frontmatter validation**:

   - YAML syntax correct
   - All required fields present
   - Field values valid
   - Version format correct

2. **Marker validation**:

   - All opening markers closed
   - No unclosed sections
   - Language names valid
   - No nested markers

3. **Size validation**:
   - Per-component limits enforced
   - Total project limit enforced
   - Warnings at threshold
   - Tier limits enforced (tools)

## Environment Variables

### Runtime Configuration

```bash
# Override project language
XZAGENTZ_LANGUAGE=python cargo run

# Enable debug logging
RUST_LOG=debug cargo run

# Skip size validation (not recommended)
XZAGENTZ_SKIP_SIZE_VALIDATION=1 cargo run

# Custom config path
XZAGENTZ_CONFIG=custom_config.yaml cargo run
```

## Error Codes

### Validation Errors

| Code   | Error                  | Description                        |
| ------ | ---------------------- | ---------------------------------- |
| `E001` | Missing frontmatter    | Component lacks YAML frontmatter   |
| `E002` | Invalid frontmatter    | YAML syntax error                  |
| `E003` | Missing required field | Required frontmatter field missing |
| `E004` | Invalid category       | Category not recognized            |
| `E005` | Invalid version        | Version not semver format          |
| `E006` | Unbalanced markers     | Opening marker without closing     |
| `E007` | Invalid language       | Language not supported             |
| `E008` | Size exceeded          | Component exceeds size limit       |
| `E009` | Missing section        | Required section not found         |
| `E010` | Invalid tier           | Tier invalid for category          |

## Best Practices

### Frontmatter

1. Keep descriptions concise (one line)
2. List languages in alphabetical order
3. Use semantic versioning correctly
4. Define all sections upfront
5. Mark language specificity accurately

### Language Markers

1. Always close markers
2. Use exact language names
3. Keep language sections focused
4. Avoid duplicating universal content
5. Test rendering for each language

### Size Management

1. Monitor component size regularly
2. Split before hitting limits
3. Remove outdated content
4. Extract examples to separate files
5. Use tiers appropriately (tools)

### Configuration

1. Use project config for team standards
2. Commit config to version control
3. Document custom limits
4. Test config changes before deploying
5. Keep config DRY (don't repeat defaults)

## Examples

See `examples/components/` for reference implementations:

- `core_example.md` - Core category component
- `tool_essential_example.md` - Essential tier tool
- `tool_comprehensive_example.md` - Comprehensive tier tool
- `language_specific_example.md` - Language component

## Related Documentation

- **Authoring Guide**: `docs/how_to/authoring_components.md`
- **Migration Guide**: `docs/how_to/migrating_to_v2.md`
- **Troubleshooting**: `docs/how_to/troubleshooting.md`
- **Architecture**: `docs/explanation/language_agnostic_component_system_implementation_plan.md`
