# Component YAML Schema Reference

## Overview

This document provides the complete YAML schema specification for xzagentz component frontmatter. All components must include valid YAML frontmatter following this schema.

## Schema Structure

```yaml
---
component:
  name: string (required)
  category: string (required)
  version: string (required)
  description: string (optional)
  languages: array<string> (optional)
  tier: string (optional, required for tools)
  sections: array<object> (optional)
---
```

## Required Fields

### component.name

**Type**: `string`

**Description**: Unique identifier for the component

**Format**: Lowercase letters, numbers, and underscores only

**Examples**:
- `error_handling`
- `git_conventions`
- `rust_standards`
- `docker_essentials`

**Invalid Examples**:
- `Error-Handling` (uppercase, hyphens)
- `error handling` (spaces)
- `errorHandling` (camelCase)

**Validation Rules**:
- Must be unique within category
- Must match filename (without `.md` extension)
- Lowercase only
- Underscores for word separation
- No special characters except underscore

### component.category

**Type**: `string`

**Description**: Component category determining purpose and size limit

**Valid Values**:
- `core` - Essential development practices (200 line limit)
- `general` - Common development topics (150 line limit)
- `languages` - Language-specific best practices (300 line limit)
- `tools` - Tool usage and configuration (100-200 line limit)

**Examples**:
```yaml
category: "core"
category: "general"
category: "languages"
category: "tools"
```

**Validation Rules**:
- Must be one of the four valid values
- Case-sensitive (lowercase only)
- Determines size constraint validation

### component.version

**Type**: `string`

**Description**: Component version following semantic versioning

**Format**: `MAJOR.MINOR.PATCH`

**Examples**:
- `1.0.0` - Initial release
- `1.2.0` - New features added
- `1.2.3` - Bug fixes
- `2.0.0` - Breaking changes

**Validation Rules**:
- Must follow semantic versioning (semver)
- Three numeric parts separated by dots
- No prefixes (no `v1.0.0`)
- No suffixes (no `1.0.0-beta`)

## Optional Fields

### component.description

**Type**: `string`

**Description**: Brief one-line summary of component purpose

**Length**: Recommended 50-100 characters

**Examples**:
```yaml
description: "Error handling best practices and patterns"
description: "Git workflow and commit conventions"
description: "Rust ownership and borrowing guidelines"
```

**Guidelines**:
- Keep concise and descriptive
- No period at end
- Describe what, not how
- User-facing language

### component.languages

**Type**: `array<string>`

**Description**: Programming languages this component supports

**Valid Values**:
- `rust`
- `python`
- `go`
- `typescript`
- `javascript`
- `bash`
- `shell`

**Examples**:
```yaml
languages: ["rust", "python"]
languages: ["typescript", "javascript"]
languages: []  # Universal component
```

**Guidelines**:
- Include all languages with specific sections
- Omit if component is language-agnostic
- Use lowercase language names
- Order alphabetically

### component.tier

**Type**: `string`

**Description**: Complexity tier for tool components

**Valid Values**:
- `essential` - Basic usage (100 line limit)
- `comprehensive` - Advanced usage (200 line limit)

**Required For**: Components with `category: "tools"`

**Invalid For**: Components with other categories

**Examples**:
```yaml
# Valid for tools
category: "tools"
tier: "essential"

# Invalid for non-tools
category: "core"
tier: "essential"  # Error: tier not allowed for core
```

**Validation Rules**:
- Required if category is "tools"
- Must not be present if category is not "tools"
- Must be "essential" or "comprehensive"

### component.sections

**Type**: `array<object>`

**Description**: Metadata for component sections

**Section Object Schema**:
```yaml
sections:
  - id: string
    title: string
    languages: array<string>
    size: integer
```

**Example**:
```yaml
sections:
  - id: "overview"
    title: "Overview"
    size: 15
  - id: "rust_implementation"
    title: "Rust Implementation"
    languages: ["rust"]
    size: 50
```

**Guidelines**:
- Optional metadata for tooling
- Not validated by default
- Used for documentation generation
- Can be auto-generated

## Complete Examples

### Core Component Example

```yaml
---
component:
  name: "error_handling"
  category: "core"
  version: "1.0.0"
  description: "Error handling best practices and patterns"
  languages: ["rust", "python", "go"]
---
```

### General Component Example

```yaml
---
component:
  name: "code_review"
  category: "general"
  version: "1.2.0"
  description: "Code review process and guidelines"
---
```

### Language Component Example

```yaml
---
component:
  name: "rust_ownership"
  category: "languages"
  version: "2.0.0"
  description: "Rust ownership, borrowing, and lifetimes"
  languages: ["rust"]
---
```

### Tool Component Example (Essential)

```yaml
---
component:
  name: "docker_essentials"
  category: "tools"
  version: "1.0.0"
  tier: "essential"
  description: "Basic Docker usage for containerization"
  languages: ["bash"]
---
```

### Tool Component Example (Comprehensive)

```yaml
---
component:
  name: "docker_advanced"
  category: "tools"
  version: "1.0.0"
  tier: "comprehensive"
  description: "Advanced Docker patterns and optimization"
  languages: ["bash"]
---
```

## Size Limits by Category

| Category  | Tier          | Line Limit |
|-----------|---------------|------------|
| core      | N/A           | 200        |
| general   | N/A           | 150        |
| languages | N/A           | 300        |
| tools     | essential     | 100        |
| tools     | comprehensive | 200        |

Line count excludes YAML frontmatter and includes all Markdown content.

## Validation Rules

### Frontmatter Structure

- Must start and end with `---` on separate lines
- Must be valid YAML syntax
- Must contain `component` key at root level
- All required fields must be present

### Field Validation

**Name Validation**:
```regex
^[a-z][a-z0-9_]*$
```

**Category Validation**:
```regex
^(core|general|languages|tools)$
```

**Version Validation**:
```regex
^\d+\.\d+\.\d+$
```

**Tier Validation**:
```regex
^(essential|comprehensive)$
```

### Cross-Field Validation

- If `category` is "tools", `tier` must be present
- If `category` is not "tools", `tier` must not be present
- If `languages` is specified, component should have language markers

## Common Validation Errors

### Missing Required Field

**Error**:
```
Error: Missing required field 'name' in component frontmatter
```

**Fix**:
```yaml
component:
  name: "my_component"  # Add missing field
  category: "general"
  version: "1.0.0"
```

### Invalid Category

**Error**:
```
Error: Invalid category 'custom'. Must be: core, general, languages, tools
```

**Fix**:
```yaml
component:
  category: "general"  # Use valid category
```

### Invalid Version Format

**Error**:
```
Error: Version must follow semantic versioning (MAJOR.MINOR.PATCH)
```

**Fix**:
```yaml
component:
  version: "1.0.0"  # Correct format
```

### Missing Tier for Tools

**Error**:
```
Error: Tools category component missing required 'tier' field
```

**Fix**:
```yaml
component:
  category: "tools"
  tier: "essential"  # Add tier field
```

### Invalid Tier for Non-Tools

**Error**:
```
Error: Tier field not allowed for category 'core'
```

**Fix**:
```yaml
component:
  category: "core"
  # Remove tier field - only valid for tools
```

## Schema Extensions

### Future Fields

Reserved for future use:
- `component.author` - Component author information
- `component.license` - Component license
- `component.dependencies` - Component dependencies
- `component.tags` - Searchable tags
- `component.deprecated` - Deprecation flag

### Custom Fields

Components can include custom fields under a `custom` or `metadata` key:

```yaml
---
component:
  name: "example"
  category: "core"
  version: "1.0.0"
  metadata:
    team: "platform"
    reviewer: "john.doe"
---
```

Custom fields are preserved but not validated.

## YAML Best Practices

### Use Quotes for Strings

```yaml
# Recommended
name: "error_handling"
description: "Error handling best practices"

# Also valid but less consistent
name: error_handling
description: Error handling best practices
```

### Array Formatting

```yaml
# Inline (recommended for short lists)
languages: ["rust", "python"]

# Block (better for long lists)
languages:
  - rust
  - python
  - go
  - typescript
```

### Avoid Special Characters

```yaml
# Good
description: "Error handling best practices"

# Problematic (may need escaping)
description: Error handling: best practices & patterns
```

### Indentation

Use 2 spaces for indentation (YAML standard):

```yaml
component:
  name: "example"
  sections:
    - id: "overview"
      title: "Overview"
```

## Validation Tools

### Command Line Validation

```bash
# Validate component
xzagentz validate components/core/error_handling.md

# Detailed validation
xzagentz validate components/core/error_handling.md --detailed
```

### Programmatic Validation

```rust
use xzagentz::components::metadata::ComponentMetadata;

let yaml = r#"
---
component:
  name: "example"
  category: "core"
  version: "1.0.0"
---
"#;

let metadata: ComponentMetadata = serde_yaml::from_str(yaml)?;
// Validated during deserialization
```

## See Also

- Component Format Reference: `docs/reference/component_format.md`
- Component Configuration: `docs/reference/component_configuration.md`
- Creating Components Tutorial: `docs/tutorials/creating_custom_component.md`
- Validation Guide: `docs/how_to/validate_components.md`
