# Component System Troubleshooting Guide

Complete troubleshooting reference for the xzagentz component system.

## Overview

This guide helps diagnose and resolve common issues with component authoring, validation, and rendering.

## Quick Diagnostics

### Run All Checks

```bash
# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Validate components
cargo test size_validation_test
```

### Check Specific Component

```bash
# Validate single component
cargo run -- validate components/core/your_component.md

# Render component for specific language
cargo run -- render --component your_component --language rust

# Check component size
grep -cv '^[[:space:]]*$' components/core/your_component.md
```

## Common Issues

### Issue 1: Missing Frontmatter

**Symptoms**:
```
Error: Missing frontmatter in component
Component must start with YAML frontmatter between --- markers
```

**Cause**: Component file doesn't have YAML frontmatter or markers are incorrect.

**Solution**:

Ensure your component starts with proper frontmatter:

```markdown
---
component:
  name: component_name
  category: core
  version: 2.0.0
  description: Component description
  languages:
    - rust
  sections:
    - id: section_id
      language_specific: false
      required: true
---

# Component Content
```

**Checklist**:
- [ ] File starts with `---` on first line
- [ ] Frontmatter ends with `---` on its own line
- [ ] No content before opening `---`
- [ ] YAML is properly indented (2 spaces)

### Issue 2: Invalid YAML Syntax

**Symptoms**:
```
Error: Failed to parse YAML frontmatter
YAML error at line 5: expected value
```

**Cause**: YAML syntax error in frontmatter.

**Common YAML mistakes**:

```yaml
# BAD: Missing space after colon
name:component_name

# GOOD: Space after colon
name: component_name

# BAD: Wrong indentation (tabs or wrong spaces)
component:
name: test

# GOOD: Consistent 2-space indentation
component:
  name: test

# BAD: Unquoted string with special characters
description: Component: test & guide

# GOOD: Quoted string with special characters
description: "Component: test & guide"

# BAD: Missing array marker
languages:
  rust
  python

# GOOD: Array with dashes
languages:
  - rust
  - python
```

**Solution**:

1. Validate YAML syntax:
   ```bash
   # Extract frontmatter
   sed -n '/^---$/,/^---$/p' components/core/your_component.md > /tmp/frontmatter.yaml

   # Validate with yamllint (if installed)
   yamllint /tmp/frontmatter.yaml
   ```

2. Check common issues:
   - Use 2 spaces for indentation (no tabs)
   - Add space after colons
   - Use `-` for array items
   - Quote strings with special characters
   - Match opening/closing brackets and braces

### Issue 3: Missing Required Fields

**Symptoms**:
```
Error: Missing required field in component frontmatter: description
```

**Cause**: Required frontmatter field is missing.

**Required fields**:
- `name`
- `category`
- `version`
- `description`
- `languages` (array with at least one language)
- `sections` (array with at least one section)

**Solution**:

Ensure all required fields are present:

```yaml
---
component:
  name: example_component          # Required
  category: core                    # Required
  version: 2.0.0                    # Required
  description: Brief description    # Required
  languages:                        # Required (at least one)
    - rust
  sections:                         # Required (at least one)
    - id: principles
      language_specific: false
      required: true
---
```

### Issue 4: Unbalanced Language Markers

**Symptoms**:
```
Error: Unbalanced language markers
Opening marker <!-- LANG:rust --> at line 45 has no closing marker
```

**Cause**: Language section opened but not closed.

**Solution**:

1. Find unbalanced markers:
   ```bash
   # List all opening markers
   grep -n "<!-- LANG:" components/core/your_component.md

   # List all closing markers
   grep -n "<!-- /LANG:" components/core/your_component.md
   ```

2. Ensure every opening has a matching closing:
   ```markdown
   <!-- LANG:rust -->
   Content here
   <!-- /LANG:rust -->

   <!-- LANG:python -->
   Content here
   <!-- /LANG:python -->
   ```

3. Common mistakes:
   ```markdown
   # BAD: Missing closing marker
   <!-- LANG:rust -->
   Content

   # BAD: Wrong closing marker
   <!-- LANG:rust -->
   Content
   <!-- /LANG:python -->

   # BAD: Case mismatch
   <!-- LANG:rust -->
   Content
   <!-- /LANG:Rust -->

   # GOOD: Properly balanced
   <!-- LANG:rust -->
   Content
   <!-- /LANG:rust -->
   ```

### Issue 5: Invalid Language Name

**Symptoms**:
```
Error: Invalid language marker: LANG:Rust
Supported languages: rust, python, golang, typescript, bash
```

**Cause**: Language name in marker doesn't match supported languages.

**Solution**:

Use exact language names (case-sensitive):

```markdown
Supported markers:
<!-- LANG:rust -->       ✓ Correct
<!-- LANG:python -->     ✓ Correct
<!-- LANG:golang -->     ✓ Correct
<!-- LANG:typescript --> ✓ Correct
<!-- LANG:bash -->       ✓ Correct

Invalid markers:
<!-- LANG:Rust -->       ✗ Wrong case
<!-- LANG:go -->         ✗ Use 'golang'
<!-- LANG:ts -->         ✗ Use 'typescript'
<!-- LANG:shell -->      ✗ Use 'bash'
```

### Issue 6: Size Limit Exceeded

**Symptoms**:
```
Error: Component size exceeds limit
Component: error_handling
Category: core
Actual: 650 lines
Limit: 500 lines
```

**Cause**: Component exceeds category size limit.

**Size limits**:
- core: 500 lines
- general: 800 lines
- languages: 600 lines
- tools (essential): 300 lines
- tools (comprehensive): 800 lines

**Solution**:

1. Check current size:
   ```bash
   # Count non-empty lines
   grep -cv '^[[:space:]]*$' components/core/your_component.md
   ```

2. Reduce size:
   - Remove redundant content
   - Extract examples to separate files
   - Split into multiple focused components
   - For tools: split into essential/comprehensive tiers
   - Move detailed explanations to reference docs

3. Example split:
   ```bash
   # Before: git.md (1000 lines)
   components/tools/git.md

   # After: Split by tier
   components/tools/git_essential.md (300 lines)
   components/tools/git_comprehensive.md (800 lines)
   ```

### Issue 7: Invalid Category

**Symptoms**:
```
Error: Invalid category: Core
Valid categories: core, general, languages, tools
```

**Cause**: Category field has invalid value or wrong case.

**Solution**:

Use lowercase category names:

```yaml
category: core        # ✓ Correct
category: general     # ✓ Correct
category: languages   # ✓ Correct
category: tools       # ✓ Correct

category: Core        # ✗ Wrong case
category: CORE        # ✗ Wrong case
category: misc        # ✗ Invalid category
```

### Issue 8: Missing Tier for Tools

**Symptoms**:
```
Error: Tool component missing required field: tier
Category 'tools' requires tier: essential or comprehensive
```

**Cause**: Component with `category: tools` doesn't specify tier.

**Solution**:

Add tier field for tool components:

```yaml
---
component:
  name: git_essential
  category: tools
  tier: essential           # Required for tools
  version: 2.0.0
  description: Essential Git commands
  languages:
    - bash
  sections:
    - id: basic_commands
      language_specific: false
      required: true
---
```

### Issue 9: Invalid Version Format

**Symptoms**:
```
Error: Invalid version format: 2.0
Expected semantic version: MAJOR.MINOR.PATCH
```

**Cause**: Version doesn't follow semantic versioning format.

**Solution**:

Use semantic versioning (X.Y.Z):

```yaml
version: 2.0.0    # ✓ Correct
version: 2.1.0    # ✓ Correct
version: 2.1.3    # ✓ Correct

version: 2.0      # ✗ Missing patch
version: 2        # ✗ Missing minor and patch
version: v2.0.0   # ✗ No 'v' prefix
version: 2.0.0.1  # ✗ Too many parts
```

### Issue 10: Test Failures

**Symptoms**:
```
test size_validation_test ... FAILED
```

**Cause**: Component validation test failed.

**Solution**:

1. Run tests with verbose output:
   ```bash
   cargo test size_validation_test -- --nocapture
   ```

2. Review test output for specific failures

3. Fix issues one at a time:
   - Fix frontmatter errors first
   - Then fix marker balance
   - Finally address size issues

4. Re-run tests after each fix

### Issue 11: Rendering Issues

**Symptoms**:
- Component renders with wrong language content
- Missing sections in rendered output
- Universal content not showing

**Cause**: Rendering configuration or language marker issues.

**Solution**:

1. Verify language markers are correct:
   ```bash
   # Check marker syntax
   grep "<!-- LANG:" components/core/your_component.md
   ```

2. Test rendering for each language:
   ```bash
   cargo run -- render --component your_component --language rust
   cargo run -- render --component your_component --language python
   ```

3. Check section configuration:
   ```yaml
   sections:
     - id: principles
       language_specific: false  # Should render for all languages
       required: true
     - id: patterns
       language_specific: true   # Should have language markers
       required: true
   ```

### Issue 12: CI Validation Failure

**Symptoms**:
```
GitHub Actions workflow failed
Component validation job failed
```

**Cause**: Component doesn't pass CI validation checks.

**Solution**:

1. Run CI checks locally:
   ```bash
   # Run all validation steps
   cargo fmt --all
   cargo check --all-targets --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   ```

2. Fix each failure:
   - Format errors: Run `cargo fmt --all`
   - Compilation errors: Fix code issues
   - Clippy warnings: Address linting issues
   - Test failures: Fix failing tests

3. Review CI logs for specific error details

4. Ensure all checks pass locally before pushing

## Validation Workflow

### Step-by-Step Validation

```bash
# Step 1: Format code
cargo fmt --all
# Expected: No output

# Step 2: Check compilation
cargo check --all-targets --all-features
# Expected: "Finished" with 0 errors

# Step 3: Lint code
cargo clippy --all-targets --all-features -- -D warnings
# Expected: "Finished" with 0 warnings

# Step 4: Run tests
cargo test --all-features
# Expected: "test result: ok. X passed; 0 failed"

# Step 5: Validate components
cargo test size_validation_test
# Expected: All components pass or warn only
```

### Validation Checklist

Before committing components:

- [ ] YAML frontmatter present and valid
- [ ] All required fields included
- [ ] Category and version correct
- [ ] Language markers balanced
- [ ] Component size within limits
- [ ] All tests pass
- [ ] Code formatted with cargo fmt
- [ ] No clippy warnings
- [ ] Documentation updated

## Performance Issues

### Issue: Slow Validation

**Symptoms**: Validation takes too long

**Solution**:

1. Validate specific components instead of all:
   ```bash
   # Instead of validating all
   cargo test size_validation_test

   # Validate specific component
   cargo run -- validate components/core/specific_component.md
   ```

2. Use incremental builds:
   ```bash
   # Use release mode for faster execution
   cargo build --release
   ./target/release/xzagentz validate components/core/your_component.md
   ```

### Issue: Large Component Files

**Symptoms**: Components take long to parse or render

**Solution**:

1. Split large components into smaller ones
2. Use essential/comprehensive tiers for tools
3. Extract large code examples to separate files
4. Move detailed reference material to docs/reference/

## Debugging Tips

### Enable Debug Logging

```bash
# Enable detailed logging
RUST_LOG=debug cargo run

# Enable trace logging (very verbose)
RUST_LOG=trace cargo run

# Log specific module
RUST_LOG=xzagentz::validator=debug cargo run
```

### Manual Component Inspection

```bash
# Extract and inspect frontmatter
sed -n '/^---$/,/^---$/p' components/core/your_component.md

# Count lines by section
awk '/^## / {print; count=0} /./ {count++} END {print "Lines:", count}' \
  components/core/your_component.md

# Find all language sections
grep -n "<!-- LANG:" components/core/your_component.md

# Verify marker balance
echo "Opening: $(grep -c '<!-- LANG:' components/core/your_component.md)"
echo "Closing: $(grep -c '<!-- /LANG:' components/core/your_component.md)"
```

### Testing Individual Components

Create a minimal test:

```rust
#[cfg(test)]
mod tests {
    use xzagentz::validator::size::SizeValidator;

    #[test]
    fn test_my_component() {
        let validator = SizeValidator::new();
        let content = std::fs::read_to_string(
            "components/core/my_component.md"
        ).unwrap();

        let result = validator.validate_content(&content, "core", None);

        if let Err(e) = result {
            panic!("Validation failed: {}", e);
        }
    }
}
```

## Getting Help

### Documentation Resources

- **Authoring Guide**: `docs/how_to/authoring_components.md`
- **Configuration Reference**: `docs/reference/component_configuration.md`
- **Migration Guide**: `docs/how_to/migrating_to_v2.md`
- **Architecture**: `docs/explanation/language_agnostic_component_system_implementation_plan.md`

### Example Components

Review working examples:
- `examples/components/core_example.md`
- `examples/components/tool_essential_example.md`
- `components/core/error_handling.md`

### Validation Report

Generate detailed validation report:

```bash
# Run validation and generate report
cargo run -- validate --report

# Report includes:
# - All validated components
# - Size statistics
# - Warnings and errors
# - Per-category breakdown
```

## Preventive Measures

### Pre-Commit Checks

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Run tests
cargo test --all-features

# Validate components
cargo test size_validation_test

echo "All checks passed!"
```

### Editor Integration

Configure your editor to validate on save:

**VS Code** (`.vscode/settings.json`):
```json
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.check.command": "clippy"
}
```

### Automated Testing

Set up CI to catch issues early:

```yaml
# .github/workflows/component_validation.yaml
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
```

## Summary

Most common issues and quick fixes:

| Issue | Quick Fix |
|-------|-----------|
| Missing frontmatter | Add YAML block at file start |
| Invalid YAML | Check indentation and syntax |
| Unbalanced markers | Ensure every `<!-- LANG:X -->` has `<!-- /LANG:X -->` |
| Size exceeded | Split component or reduce content |
| Invalid category | Use lowercase: core, general, languages, tools |
| Missing tier | Add `tier: essential` or `comprehensive` for tools |
| Test failures | Run `cargo test -- --nocapture` to see details |
| CI failures | Run all checks locally before pushing |

For additional help, refer to the documentation resources listed above.
