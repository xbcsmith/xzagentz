# How to Validate Components

## Overview

This guide explains how to validate xzagentz components to ensure they meet quality standards, follow formatting rules, and comply with size constraints. Component validation is essential for maintaining consistency and preventing errors in generated documentation.

## Prerequisites

- xzagentz installed and working
- Basic understanding of component structure
- Components to validate (embedded or custom)

## Quick Start

### Validate a Single Component

```bash
xzagentz validate components/core/error_handling.md
```

Expected output if valid:
```
Validation passed: components/core/error_handling.md
```

### Validate with Detailed Report

```bash
xzagentz validate components/core/error_handling.md --detailed
```

Shows comprehensive analysis including line counts, marker validation, and warnings.

## Understanding Validation Rules

### YAML Frontmatter Validation

Components must have valid YAML frontmatter with required fields:

**Required Fields**:
- `component.name` - Lowercase with underscores
- `component.category` - Must be: core, general, languages, tools
- `component.version` - Semantic versioning (MAJOR.MINOR.PATCH)

**Optional Fields**:
- `component.description` - Brief component summary
- `component.languages` - Array of supported languages
- `component.tier` - For tools category: essential or comprehensive

**Example Valid Frontmatter**:
```yaml
---
component:
  name: "error_handling"
  category: "core"
  version: "1.0.0"
  description: "Error handling best practices"
  languages: ["rust", "python"]
---
```

### Size Constraint Validation

Components have size limits based on category:

| Category  | Tier          | Line Limit |
|-----------|---------------|------------|
| core      | N/A           | 200        |
| general   | N/A           | 150        |
| languages | N/A           | 300        |
| tools     | essential     | 100        |
| tools     | comprehensive | 200        |

Size includes all content except YAML frontmatter.

### Language Marker Validation

Language-specific sections must use correct markers:

**Valid Markers**:
- `<!-- RUST_START -->` and `<!-- RUST_END -->`
- `<!-- PYTHON_START -->` and `<!-- PYTHON_END -->`
- `<!-- GO_START -->` and `<!-- GO_END -->`
- `<!-- TYPESCRIPT_START -->` and `<!-- TYPESCRIPT_END -->`
- `<!-- BASH_START -->` and `<!-- BASH_END -->`

**Rules**:
1. Markers must be on their own line
2. Every START marker must have matching END marker
3. Markers cannot be nested
4. Language name must be uppercase
5. Markers must use HTML comment syntax

### Markdown Formatting Validation

**Checks**:
- No emojis anywhere in content
- Code blocks specify language
- Headers follow proper hierarchy
- No YAML frontmatter in body content
- Consistent whitespace

## Validation Commands

### Basic Validation

Validate single file:
```bash
xzagentz validate path/to/component.md
```

Exit codes:
- `0` - Validation passed
- `1` - Validation failed

### Detailed Validation

Get comprehensive report:
```bash
xzagentz validate path/to/component.md --detailed
```

Output includes:
- Total line count
- Size constraint status
- Frontmatter validation results
- Marker pairing validation
- Language section analysis
- Warnings for approaching limits

Example detailed output:
```
Validating: components/core/error_handling.md

Frontmatter: Valid
  - name: error_handling
  - category: core
  - version: 1.0.0
  - languages: rust, python, go

Size Analysis:
  - Total lines: 178
  - Limit: 200 (core category)
  - Status: OK (89% of limit)
  - Warning: Approaching size limit

Language Markers:
  - RUST_START/RUST_END: Matched
  - PYTHON_START/PYTHON_END: Matched
  - GO_START/GO_END: Matched

Validation: PASSED with 1 warning
```

### Automatic Fixing

Fix common issues automatically:
```bash
xzagentz validate path/to/component.md --fix
```

Automatic fixes:
- Normalize frontmatter formatting
- Fix marker spacing
- Remove trailing whitespace
- Correct marker capitalization

Creates backup before modifying:
```
Original: component.md
Backup: component.md.bak
Fixed: component.md
```

### JSON Output

Get validation results in JSON format for automation:
```bash
xzagentz validate path/to/component.md --format json
```

Example JSON output:
```json
{
  "file": "components/core/error_handling.md",
  "valid": true,
  "errors": [],
  "warnings": [
    "Component size is 89% of limit (178/200 lines)"
  ],
  "frontmatter": {
    "name": "error_handling",
    "category": "core",
    "version": "1.0.0"
  },
  "size": {
    "lines": 178,
    "limit": 200,
    "percentage": 89
  }
}
```

## Common Validation Errors

### Error: Missing Required Frontmatter Field

**Error Message**:
```
Error: Missing required field 'name' in component frontmatter
```

**Cause**: Component frontmatter missing required field

**Solution**: Add missing field to frontmatter
```yaml
---
component:
  name: "my_component"  # Add this
  category: "general"
  version: "1.0.0"
---
```

### Error: Invalid Category

**Error Message**:
```
Error: Invalid category 'custom'. Must be one of: core, general, languages, tools
```

**Cause**: Category field has invalid value

**Solution**: Use valid category
```yaml
component:
  category: "general"  # Must be: core, general, languages, tools
```

### Error: Invalid Version Format

**Error Message**:
```
Error: Version must follow semantic versioning (MAJOR.MINOR.PATCH)
```

**Cause**: Version field not in semantic versioning format

**Solution**: Use semantic versioning
```yaml
component:
  version: "1.0.0"  # Must be MAJOR.MINOR.PATCH format
```

### Error: Component Exceeds Size Limit

**Error Message**:
```
Error: Component exceeds size limit (250 lines, limit 150 for category 'general')
```

**Cause**: Component content exceeds category size limit

**Solutions**:

1. Split into multiple smaller components
2. Move to category with larger limit
3. Remove redundant content
4. Extract examples to separate files

### Error: Unmatched Language Marker

**Error Message**:
```
Error: Found RUST_START marker without matching RUST_END marker
```

**Cause**: Language section not properly closed

**Solution**: Ensure every START has matching END
```markdown
<!-- RUST_START -->
Content here
<!-- RUST_END -->  <!-- Add this -->
```

### Error: Nested Language Markers

**Error Message**:
```
Error: Language markers cannot be nested
```

**Cause**: Language section inside another language section

**Solution**: Close first section before starting new one
```markdown
<!-- RUST_START -->
Rust content
<!-- RUST_END -->

<!-- PYTHON_START -->  <!-- Not nested -->
Python content
<!-- PYTHON_END -->
```

### Error: Invalid Marker Format

**Error Message**:
```
Error: Invalid language marker format 'rust_start'
```

**Cause**: Marker not using correct format

**Solution**: Use proper marker format
```markdown
<!-- RUST_START -->  ✓ Correct (uppercase, HTML comment)
<!-- rust_start -->  ✗ Wrong (lowercase)
<RUST_START>        ✗ Wrong (not HTML comment)
```

### Warning: Approaching Size Limit

**Warning Message**:
```
Warning: Component size is 85% of limit (127/150 lines)
```

**Cause**: Component close to size limit

**Action**: Consider refactoring before hitting limit to avoid future validation failures

## Validating Multiple Components

### Validate All Components in Directory

```bash
# Validate all components in directory
for file in components/**/*.md; do
  echo "Validating: $file"
  xzagentz validate "$file"
done
```

### Batch Validation Script

Create validation script for CI/CD:

```bash
#!/bin/bash
# validate-all-components.sh

set -e

COMPONENT_DIR="components"
FAILED=0

for category in core general languages tools; do
  echo "Validating $category components..."
  for file in "$COMPONENT_DIR/$category"/*.md; do
    if [ -f "$file" ]; then
      if ! xzagentz validate "$file"; then
        echo "FAILED: $file"
        FAILED=$((FAILED + 1))
      fi
    fi
  done
done

if [ $FAILED -gt 0 ]; then
  echo "Validation failed: $FAILED components"
  exit 1
fi

echo "All components validated successfully"
exit 0
```

Usage:
```bash
chmod +x validate-all-components.sh
./validate-all-components.sh
```

### Validation in CI/CD Pipeline

**GitHub Actions Example**:
```yaml
name: Component Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build xzagentz
        run: cargo build --release

      - name: Validate Components
        run: |
          for file in components/**/*.md; do
            ./target/release/xzagentz validate "$file" --detailed
          done
```

## Validation Best Practices

### Before Committing

Always validate components before committing:
```bash
# Validate modified component
xzagentz validate components/core/error_handling.md --detailed

# Run automatic fixes if needed
xzagentz validate components/core/error_handling.md --fix

# Verify fixes worked
xzagentz validate components/core/error_handling.md
```

### During Development

Run validation frequently during component development:
```bash
# Watch mode (requires entr or similar)
find components -name "*.md" | entr xzagentz validate
```

### Code Review Checklist

Before approving component changes:
- [ ] Validation passes with zero errors
- [ ] No warnings for size limits
- [ ] All language markers properly matched
- [ ] Frontmatter complete and valid
- [ ] No emojis in content
- [ ] Code blocks specify language

### Size Management

Monitor component size:
```bash
# Get size report
xzagentz validate component.md --detailed | grep "Size Analysis"

# If approaching limit:
# 1. Remove redundant examples
# 2. Extract sections to new components
# 3. Reference external documentation
# 4. Consider moving to larger category
```

## Integration with Development Workflow

### Pre-commit Hook

Create `.git/hooks/pre-commit`:
```bash
#!/bin/bash
# Validate components before commit

STAGED_COMPONENTS=$(git diff --cached --name-only --diff-filter=ACM | grep '\.md$')

if [ -z "$STAGED_COMPONENTS" ]; then
  exit 0
fi

for file in $STAGED_COMPONENTS; do
  if [[ $file == components/* ]]; then
    echo "Validating: $file"
    if ! xzagentz validate "$file"; then
      echo "Validation failed for $file"
      exit 1
    fi
  fi
done

exit 0
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

### Editor Integration

**VS Code**:
Add task to `.vscode/tasks.json`:
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Validate Component",
      "type": "shell",
      "command": "xzagentz",
      "args": ["validate", "${file}", "--detailed"],
      "problemMatcher": [],
      "presentation": {
        "reveal": "always"
      }
    }
  ]
}
```

Run with: `Cmd+Shift+P` → "Tasks: Run Task" → "Validate Component"

## Troubleshooting

### Validation Hangs or Freezes

**Issue**: Validation command does not complete

**Causes**:
- Very large component file
- Malformed YAML causing parser issues
- Circular references in markers

**Solutions**:
1. Check file size: `wc -l component.md`
2. Validate YAML separately: `yq eval component.md`
3. Check for unclosed markers manually

### False Positive Errors

**Issue**: Validation reports errors but file appears correct

**Solutions**:
1. Check for hidden characters: `cat -A component.md`
2. Verify line endings: `file component.md`
3. Re-save with correct encoding (UTF-8)

### Fix Command Changes Unexpected Content

**Issue**: `--fix` flag modifies content incorrectly

**Solutions**:
1. Always review backup file: `diff component.md component.md.bak`
2. Restore from backup if needed: `mv component.md.bak component.md`
3. Report issue with example file

## Summary

Component validation ensures:
- Correct frontmatter structure
- Adherence to size limits
- Proper language marker usage
- Markdown formatting standards
- Consistency across component library

Regular validation prevents errors and maintains quality across all documentation.

## Related Documentation

- Component Format Reference: `docs/reference/component_format.md`
- Creating Components Tutorial: `docs/tutorials/creating_custom_component.md`
- Component Configuration: `docs/reference/component_configuration.md`
- Troubleshooting Guide: `docs/how_to/troubleshooting.md`
