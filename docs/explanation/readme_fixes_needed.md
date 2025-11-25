# README.md Fixes Required - Action Plan

## Critical Issues Summary

The README.md contains **5 broken commands** out of 7 examples (71% failure
rate). Users following the documented examples will encounter immediate errors.

---

## Fixes Required (By Priority)

### Priority 1: Remove Non-Existent Commands

#### Line 54: Remove `render` Command

**Current (BROKEN)**:

```bash
# Render component for specific language
xzagentz render --component error_handling --language rust
```

**Action**: DELETE these lines entirely - command does not exist

---

### Priority 2: Fix Incorrect Command Syntax

#### Line 76: Fix `create` Command

**Current (BROKEN)**:

```bash
# Create a new project using embedded resources
xzagentz create my-project --template rust_binary --language rust
```

**Replace With**:

```bash
# Create a new AGENTS.md file using a template
xzagentz create --output AGENTS.md --template default

# Create with interactive mode
xzagentz create --interactive
```

**Note**: Also update the comment - this creates AGENTS.md files, NOT projects.

---

#### Line 57: Fix `validate` Command

**Current (BROKEN)**:

```bash
# Generate validation report
xzagentz validate --report
```

**Replace With**:

```bash
# Generate detailed validation report
xzagentz validate --detailed

# Validate and fix common issues
xzagentz validate --detailed --fix
```

---

#### Line 63: Fix `list` Command

**Current (BROKEN)**:

```bash
# List available components
xzagentz list --category core
```

**Replace With**:

```bash
# List available components
xzagentz list components

# List components filtered by category
xzagentz list components --category core
```

---

### Priority 3: Update Section Narrative

#### Lines 67-70: Update "Embedded Resources" Section Description

**Current Text**:

> xzagentz includes all components and templates embedded in the binary,
> allowing it to work out of the box without any setup. You can also customize
> resources by extracting and modifying them.

**Action**: This text is accurate - keep as-is.

However, the **examples that follow** are broken (see Line 76 fix above).

---

## Complete Corrected "Basic Usage" Section

Replace lines 48-66 with:

````markdown
### Basic Usage

```bash
# Validate an AGENTS.md file
xzagentz validate AGENTS.md

# Validate with detailed output
xzagentz validate --detailed

# List available components
xzagentz list components

# List components by category
xzagentz list components --category core

# List available templates
xzagentz list templates

# Check all components with tests
cargo test size_validation_test
```
````

````

---

## Complete Corrected "Using Embedded Resources" Section

Replace lines 71-84 with:

```markdown
### Using Embedded Resources (Default)

No setup required - just run commands:

```bash
# Create a new AGENTS.md file
xzagentz create --output AGENTS.md

# Create with interactive mode
xzagentz create --interactive

# List embedded components
xzagentz list components

# List embedded templates
xzagentz list templates
````

````

---

## Validation Checklist

After making changes, verify:

- [ ] All bash code blocks contain only working commands
- [ ] No references to `render` subcommand
- [ ] No references to `--report` flag
- [ ] `list` commands include required subcommand
- [ ] `create` command has no positional arguments
- [ ] Comments accurately describe what commands do
- [ ] Test all commands: `cargo run -- <command>`

---

## Testing Commands

Run these to verify all README examples work:

```bash
# Test validation
cargo run -- validate AGENTS.md
cargo run -- validate --detailed

# Test listing
cargo run -- list components
cargo run -- list components --category core
cargo run -- list templates

# Test creation (creates file, so test in temp dir)
cd /tmp
cargo run -- create --output test-agents.md
rm test-agents.md

# Test init
cargo run -- init --dry-run
````

**Expected**: All commands should execute without "error: unexpected argument"
or "error: unrecognized subcommand" messages.

---

## Additional Recommendations

### Add CI Check for README Examples

Create a test that extracts bash code blocks from README.md and validates them:

```rust
#[test]
fn test_readme_examples_are_valid() {
    // Parse README.md
    // Extract bash code blocks
    // Run each command with --help or --dry-run
    // Assert no errors
}
```

### Cross-Reference Documentation

Ensure consistency between:

- `README.md` (user-facing quick start)
- `docs/reference/cli_commands.md` (complete reference - already accurate)
- `tests/cli_commands_validation_tests.rs` (integration tests - already passing)

### Consider Adding

A "Common Mistakes" section in README.md:

```markdown
## Common Mistakes

**❌ Wrong**: `xzagentz list --category core` **✅ Correct**:
`xzagentz list components --category core`

**❌ Wrong**: `xzagentz validate --report` **✅ Correct**:
`xzagentz validate --detailed`

**❌ Wrong**: `xzagentz render --component X` **✅ Correct**: Command does not
exist (use `list` and `validate` instead)
```

---

## Summary

| Section            | Lines | Action                          | Effort      |
| ------------------ | ----- | ------------------------------- | ----------- |
| Basic Usage        | 48-66 | Replace with corrected examples | 5 min       |
| Embedded Resources | 71-84 | Replace with corrected examples | 5 min       |
| Remove render      | 54-55 | Delete lines                    | 1 min       |
| **Total**          | -     | -                               | **~15 min** |

**Impact**: Fixes 71% failure rate → 0% failure rate for documented commands.

---

## References

- Full Analysis: `docs/explanation/readme_cli_commands_analysis.md`
- Accurate CLI Reference: `docs/reference/cli_commands.md`
- CLI Help: `cargo run -- --help`
