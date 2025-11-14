# Migrating to Component System v2

This guide helps you migrate legacy components to the new v2 component system with YAML frontmatter and language-specific sections.

## Overview

Component System v2 introduces:

- **YAML frontmatter**: Structured metadata for components
- **Language markers**: Explicit language-specific sections
- **Size enforcement**: Category-based line limits
- **Tier system**: Essential vs comprehensive tool components
- **Automated validation**: CI-driven quality checks

## Migration Timeline

### Backward Compatibility

The v2 system maintains backward compatibility with legacy components during the transition period:

- **Phase 1 (Weeks 1-2)**: Both formats supported
- **Phase 2 (Weeks 3-4)**: v2 preferred, legacy deprecated
- **Phase 3 (Week 5+)**: Legacy components archived, v2 required

## Pre-Migration Checklist

Before migrating components, ensure:

- [ ] You understand the new component structure (see `docs/how_to/authoring_components.md`)
- [ ] You have identified which components need migration
- [ ] You have backed up existing components
- [ ] You have reviewed size limits for your component category
- [ ] You have the validation tools installed

## Migration Process

### Step 1: Identify Legacy Components

Find components that need migration:

```bash
# Find components without YAML frontmatter
grep -L "^---" components/**/*.md

# Find components without language markers
grep -L "<!-- LANG:" components/**/*.md
```

### Step 2: Analyze Existing Content

For each legacy component:

1. Identify the category (core, general, languages, tools)
2. Determine if content is language-specific
3. Check current line count
4. Identify which languages are covered

Example analysis:

```bash
# Count lines (excluding empty lines)
grep -cv '^[[:space:]]*$' components/core/legacy_component.md

# Search for language-specific content
grep -n "Rust\|Python\|Go" components/core/legacy_component.md
```

### Step 3: Create v2 Component Structure

#### Add YAML Frontmatter

Transform legacy header into structured frontmatter:

**Legacy format:**

```markdown
# Error Handling

This component covers error handling in Rust, Python, and Go.
```

**v2 format:**

```markdown
---
component:
  name: error_handling
  category: core
  version: 2.0.0
  description: Error handling standards and patterns for robust code
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
```

#### Frontmatter Template

Use this template for all components:

```yaml
---
component:
  name: component_name # Lowercase with underscores
  category: core # core|general|languages|tools
  version: 2.0.0 # Start at 2.0.0 for migrated components
  tier: essential # Only for tools category
  description: Brief description # One-line summary
  languages: # List all supported languages
    - rust
    - python
    - golang
    - typescript
    - bash
  sections: # Define content structure
    - id: section_id
      language_specific: true|false
      required: true|false
---
```

### Step 4: Add Language Markers

Wrap language-specific content in markers:

**Legacy format:**

````markdown
## Error Handling Patterns

In Rust, use Result types:

```rust
fn process() -> Result<(), Error> {
    // code
}
```
````

In Python, use exceptions:

```python
def process():
    try:
        # code
    except Exception as e:
        raise
```

````

**v2 format:**

```markdown
## Error Handling Patterns

<!-- LANG:rust -->

In Rust, use Result types:

```rust
fn process() -> Result<(), Error> {
    // code
}
````

<!-- /LANG:rust -->

<!-- LANG:python -->

In Python, use exceptions:

```python
def process():
    try:
        # code
    except Exception as e:
        raise
```

<!-- /LANG:python -->

````

### Step 5: Restructure Content

Organize content into universal and language-specific sections:

**Universal sections** (no language markers):
- Core principles
- Architectural concepts
- Language-agnostic best practices

**Language-specific sections** (with markers):
- Syntax examples
- Library usage
- Language-specific idioms
- Tool integration

### Step 6: Enforce Size Limits

Check and reduce size if needed:

```bash
# Check component size
grep -cv '^[[:space:]]*$' components/core/your_component.md

# If over limit, consider:
# - Splitting into multiple components
# - Moving details to reference docs
# - Creating essential vs comprehensive versions (tools only)
````

Size limits:

| Category  | Tier          | Max Lines |
| --------- | ------------- | --------- |
| core      | -             | 500       |
| general   | -             | 800       |
| languages | -             | 600       |
| tools     | essential     | 300       |
| tools     | comprehensive | 800       |

### Step 7: Validate Migration

Run validation checks:

```bash
# Format code
cargo fmt --all

# Compile and check
cargo check --all-targets --all-features

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all-features

# Validate specific component
cargo test -- your_component_name
```

### Step 8: Update Documentation

Update any documentation that references the migrated component:

- README.md references
- Tutorial mentions
- Example code
- Related components

## Migration Examples

### Example 1: Core Component

**Before (legacy):**

```markdown
# Testing Standards

All code must have tests. Use appropriate testing frameworks.

For Rust, use the built-in test framework.
For Python, use pytest.
For Go, use the testing package.
```

**After (v2):**

````markdown
---
component:
  name: testing_standards
  category: core
  version: 2.0.0
  description: Testing requirements and best practices
  languages:
    - rust
    - python
    - golang
  sections:
    - id: principles
      language_specific: false
      required: true
    - id: frameworks
      language_specific: true
      required: true
---

# Testing Standards

## Core Principles

All code must have tests to ensure correctness and prevent regressions.

Key testing principles:

- Write tests before fixing bugs
- Aim for high code coverage
- Test edge cases and error conditions

---

## Testing Frameworks

<!-- LANG:rust -->

Use Rust's built-in test framework:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```
````

<!-- /LANG:rust -->

<!-- LANG:python -->

Use pytest for Python testing:

```python
def test_example():
    assert 2 + 2 == 4
```

<!-- /LANG:python -->

<!-- LANG:golang -->

Use Go's testing package:

```go
func TestExample(t *testing.T) {
    if 2 + 2 != 4 {
        t.Error("Math is broken")
    }
}
```

<!-- /LANG:golang -->

````

### Example 2: Tool Component with Tiers

**Before (single component):**

```markdown
# Git Guide

Basic commands, advanced workflows, rebasing, cherry-picking,
submodules, hooks, etc. (1000+ lines)
````

**After (split into tiers):**

`components/tools/git_essential.md` (300 lines):

```markdown
---
component:
  name: git_essential
  category: tools
  version: 2.0.0
  tier: essential
  description: Essential Git commands and workflows
  languages:
    - bash
  sections:
    - id: basic_commands
      language_specific: false
      required: true
---

# Git - Essential Guide

## Basic Commands

Essential Git operations for daily work:

- `git clone <url>` - Clone repository
- `git add <files>` - Stage changes
- `git commit -m "message"` - Commit changes
- `git push` - Push to remote
- `git pull` - Pull from remote
```

`components/tools/git_comprehensive.md` (800 lines):

```markdown
---
component:
  name: git_comprehensive
  category: tools
  version: 2.0.0
  tier: comprehensive
  description: Comprehensive Git reference with advanced workflows
  languages:
    - bash
  sections:
    - id: advanced_workflows
      language_specific: false
      required: true
---

# Git - Comprehensive Guide

Includes everything from essential, plus:

## Advanced Workflows

- Interactive rebasing
- Cherry-picking commits
- Managing submodules
- Custom hooks
- Bisecting
- Reflog recovery
```

## Automated Migration Script

For bulk migrations, use the migration helper:

```bash
# Convert legacy component to v2 format
cargo run -- migrate components/legacy/component.md \
  --output components/core/component.md \
  --category core

# Dry run to preview changes
cargo run -- migrate components/legacy/component.md \
  --dry-run

# Migrate entire directory
cargo run -- migrate components/legacy/ \
  --output components/core/ \
  --category core
```

## Handling Edge Cases

### Components Without Clear Language Boundaries

If content is mixed without clear language separation:

1. Extract universal principles first
2. Create separate language sections for examples
3. Consider splitting into multiple components if too large

### Components Exceeding Size Limits

Options for oversized components:

1. **Split by topic**: Create multiple focused components
2. **Split by tier**: Essential vs comprehensive (tools only)
3. **Move to reference**: Keep component concise, move details to `docs/reference/`
4. **Extract examples**: Move large examples to `examples/` directory

### Components with Outdated Content

Use migration as an opportunity to:

1. Update deprecated practices
2. Remove obsolete information
3. Add modern best practices
4. Improve examples

## Validation and Testing

After migration, verify:

```bash
# All components pass validation
cargo test size_validation_test

# Frontmatter is correct
cargo test frontmatter_validation

# Language markers are balanced
cargo test marker_validation

# Components render correctly
cargo run -- render --component your_component --language rust
```

## Common Migration Issues

### Issue 1: Frontmatter Syntax Errors

**Problem**: YAML parsing fails

**Solution**: Validate YAML syntax:

```bash
# Use a YAML validator
yamllint components/core/your_component.md

# Check indentation (2 spaces, not tabs)
# Check for trailing spaces
# Ensure colons have space after them
```

### Issue 2: Unbalanced Language Markers

**Problem**: Opening marker without closing marker

**Solution**:

```bash
# Find unbalanced markers
grep -n "<!-- LANG:" components/core/your_component.md
grep -n "<!-- /LANG:" components/core/your_component.md

# Ensure count matches
```

### Issue 3: Size Limit Exceeded

**Problem**: Migrated component too large

**Solution**:

1. Review content for redundancy
2. Extract language-agnostic content to separate component
3. For tools, split into essential/comprehensive tiers
4. Move detailed examples to reference documentation

### Issue 4: Missing Language Content

**Problem**: Some languages missing in language-specific sections

**Solution**:

1. Add placeholder content for missing languages
2. Mark section as `required: false` if not all languages covered
3. Remove unsupported languages from frontmatter languages list

## Post-Migration Tasks

After completing migration:

1. **Archive legacy components**:

   ```bash
   mkdir -p archive/legacy
   mv components/legacy/* archive/legacy/
   ```

2. **Update references**:

   - Update README.md
   - Update documentation links
   - Update CI configuration

3. **Validate entire system**:

   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   ```

4. **Update CHANGELOG**:
   Document migrated components and breaking changes

5. **Tag release**:
   ```bash
   git tag -a v2.0.0 -m "Component System v2"
   git push origin v2.0.0
   ```

## Getting Help

If you encounter migration issues:

1. **Review documentation**:

   - `docs/how_to/authoring_components.md`
   - `docs/reference/component_configuration.md`
   - `docs/how_to/troubleshooting.md`

2. **Check examples**:

   - `examples/components/core_example.md`
   - `examples/components/tool_essential_example.md`

3. **Review existing migrations**:

   - Look at successfully migrated components in `components/core/`
   - Compare before/after for patterns

4. **Validate incrementally**:
   - Test each component after migration
   - Don't migrate all at once

## Migration Checklist

For each component:

- [ ] YAML frontmatter added with all required fields
- [ ] Language markers wrap all language-specific content
- [ ] Universal content outside language markers
- [ ] Component size within category limits
- [ ] All language markers balanced and closed
- [ ] Validation tests pass
- [ ] Documentation updated
- [ ] Examples work correctly
- [ ] Version set to 2.0.0
- [ ] Legacy component archived

## Summary

Migration to v2 provides:

- Better structure and maintainability
- Automated validation
- Clear language separation
- Size enforcement
- Improved rendering

Follow this guide systematically to ensure smooth migration with minimal disruption.
