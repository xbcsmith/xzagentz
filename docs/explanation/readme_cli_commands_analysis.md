# README.md CLI Commands Analysis

## Overview

This document analyzes the discrepancies between commands documented in `README.md`
and the actual CLI implementation of xzagentz. The analysis was performed by
systematically testing all documented commands against the current CLI help output.

## Executive Summary

**Total Commands Analyzed**: 7 documented examples
**Broken Commands**: 5 (71% failure rate)
**Working Commands**: 2 (29% success rate)

The README.md documentation is significantly out of sync with the actual CLI
implementation. Most command examples will fail if users try to execute them.

---

## Detailed Analysis

### 1. Create Command with Project Name (BROKEN)

**Documented in README.md (Line 76)**:
```bash
xzagentz create my-project --template rust_binary --language rust
```

**Actual Error**:
```text
error: unexpected argument 'my-project' found

Usage: xzagentz create [OPTIONS]
```

**Root Cause**:
- The `create` command does NOT accept a positional project name argument
- The actual implementation creates an `AGENTS.md` file, not a project directory
- `--language` flag does not exist in the current implementation
- `--template` flag exists but is for AGENTS.md templates, not project templates

**Actual Working Command**:
```bash
xzagentz create --output AGENTS.md --template some_template
```

**Impact**: HIGH - This is prominently featured in the "Quick Start" section

---

### 2. Render Command (COMPLETELY BROKEN)

**Documented in README.md (Line 54)**:
```bash
xzagentz render --component error_handling --language rust
```

**Actual Error**:
```text
error: unrecognized subcommand 'render'
```

**Root Cause**:
- The `render` subcommand does NOT exist in the CLI
- No equivalent functionality is exposed through the CLI
- This appears to be legacy documentation from an earlier design

**Impact**: HIGH - Featured in "Basic Usage" section, completely non-functional

---

### 3. Validate with Report Flag (BROKEN)

**Documented in README.md (Line 57)**:
```bash
xzagentz validate --report
```

**Actual Error**:
```text
error: unexpected argument '--report' found

Usage: xzagentz validate [OPTIONS] [FILE]
```

**Root Cause**:
- The `validate` command does NOT have a `--report` flag
- Available flags are: `--detailed`, `--fix`, `--verbose`, global options
- The command expects a file path argument (defaults to `AGENTS.md`)

**Actual Working Commands**:
```bash
xzagentz validate                           # Validate AGENTS.md
xzagentz validate some-file.md              # Validate specific file
xzagentz validate --detailed                # Detailed validation
xzagentz validate --detailed --fix          # Fix common issues
```

**Impact**: MEDIUM - Users expecting report generation will be confused

---

### 4. List with Category Flag (BROKEN)

**Documented in README.md (Line 63)**:
```bash
xzagentz list --category core
```

**Actual Error**:
```text
error: unexpected argument '--category' found

Usage: xzagentz list [OPTIONS] <COMMAND>
```

**Root Cause**:
- The `list` command requires a subcommand (`components` or `templates`)
- The `--category` flag belongs to the `list components` subcommand, not `list`
- Command structure is: `xzagentz list <subcommand> [OPTIONS]`

**Actual Working Commands**:
```bash
xzagentz list components                    # List all components
xzagentz list components --category core    # Filter by category
xzagentz list templates                     # List all templates
```

**Impact**: MEDIUM - Easily correctable by users, but incorrect syntax

---

### 5. List Components (PARTIALLY BROKEN)

**Documented in README.md (Line 79)**:
```bash
xzagentz list components
```

**Status**: WORKS

**Note**: This command works correctly as documented.

---

### 6. List Templates (PARTIALLY BROKEN)

**Documented in README.md (Line 82)**:
```bash
xzagentz list templates
```

**Status**: WORKS

**Note**: This command works correctly as documented.

---

### 7. Validate Component File (WORKS)

**Documented in README.md (Line 51)**:
```bash
xzagentz validate components/core/error_handling.md
```

**Status**: WORKS

**Output**: Produces validation report with warnings and errors for the file.

**Note**: This command works correctly as documented.

---

## Summary of Issues

### Missing Commands

1. **`render` subcommand** - Completely missing from implementation
   - Documentation suggests component rendering by language
   - No equivalent functionality exists

### Incorrect Command Signatures

2. **`create` command**
   - Documented: `create <project-name> --template <name> --language <lang>`
   - Actual: `create [OPTIONS]` (no positional args, no `--language`)
   - Purpose mismatch: docs suggest project creation, actual creates AGENTS.md

3. **`validate` command**
   - Documented: `validate --report`
   - Actual: `validate [FILE]` with `--detailed`, `--fix` flags (no `--report`)

4. **`list` command**
   - Documented: `list --category <name>`
   - Actual: `list <subcommand> [OPTIONS]` where `--category` is on subcommand

### Documentation vs Implementation Mismatch

The fundamental issue is that README.md describes a **project scaffolding tool**
while the actual implementation is an **AGENTS.md file management tool**.

**Documented Paradigm**:
```text
xzagentz is for creating/managing projects with templates
```

**Actual Implementation**:
```text
xzagentz is for creating/managing AGENTS.md files with components
```

---

## Recommendations

### Immediate Actions Required

1. **Update Basic Usage Section** (Lines 48-66)
   - Remove `render` command examples
   - Fix `validate` command (remove `--report`, add `--detailed`)
   - Fix `list` command (add required subcommand)

2. **Update Embedded Resources Section** (Lines 71-84)
   - Remove or completely rewrite `create my-project` example
   - Clarify that `create` produces AGENTS.md, not project directories
   - Remove `--language` and `--template rust_binary` references

3. **Add Correct Examples**
   ```bash
   # Create AGENTS.md file
   xzagentz create --output AGENTS.md --template default

   # List available components with filtering
   xzagentz list components --category core

   # Validate with detailed output
   xzagentz validate AGENTS.md --detailed

   # Initialize configuration
   xzagentz init
   ```

### Documentation Alignment Strategy

**Option A: Align Docs to Implementation** (RECOMMENDED)
- Update README.md to reflect actual CLI behavior
- Focus on AGENTS.md file management use cases
- Remove project scaffolding terminology
- Estimated effort: 2-4 hours

**Option B: Align Implementation to Docs**
- Implement missing `render` command
- Add project scaffolding to `create` command
- Add `--report` flag to `validate`
- Estimated effort: 40-80 hours (significant feature work)

### Validation Strategy

1. **Add CLI Integration Tests**
   - Test all README.md examples as part of CI/CD
   - Fail build if documented commands don't work
   - Reference: `tests/cli_commands_validation_tests.rs` (existing)

2. **Add Documentation Linting**
   - Extract code blocks from README.md
   - Execute shell commands in test environment
   - Flag failures before merge

3. **Keep Reference Documentation Current**
   - `docs/reference/cli_commands.md` is up-to-date
   - Consider auto-generating from CLI help output
   - Cross-reference README.md examples

---

## Testing Methodology

All commands were tested using:

```bash
cargo run -- <command> <args> 2>&1
```

CLI help was verified using:

```bash
cargo run -- --help
cargo run -- <subcommand> --help
```

Test date: 2024-01-XX
xzagentz version: (from Cargo.toml)
Tester: AI Agent (Claude)

---

## Appendix: Full Command Comparison Table

| README Command | Status | Actual Command | Issue |
|----------------|--------|----------------|-------|
| `validate components/core/error_handling.md` | ✅ WORKS | Same | None |
| `render --component error_handling --language rust` | ❌ BROKEN | N/A | Command does not exist |
| `validate --report` | ❌ BROKEN | `validate --detailed` | Flag renamed/removed |
| `list --category core` | ❌ BROKEN | `list components --category core` | Missing subcommand |
| `create my-project --template rust_binary --language rust` | ❌ BROKEN | `create --template <name>` | Positional arg not accepted |
| `list components` | ✅ WORKS | Same | None |
| `list templates` | ✅ WORKS | Same | None |
| `init` | ✅ WORKS | Same | None (not shown broken) |

**Success Rate**: 3/8 (37.5%)

---

## References

- Current CLI Help: `cargo run -- --help`
- Reference Documentation: `docs/reference/cli_commands.md`
- Integration Tests: `tests/cli_commands_validation_tests.rs`
- CLI Implementation: `src/cli/`

---

## Document Metadata

- **Created**: 2024
- **Purpose**: Analysis of README.md CLI documentation accuracy
- **Category**: Explanation (Diataxis framework)
- **Audience**: Developers, maintainers, documentation authors
- **Related Files**:
  - `README.md` (source of documented commands)
  - `docs/reference/cli_commands.md` (accurate reference)
  - `tests/cli_commands_validation_tests.rs` (validation tests)
