# AGENTS.md - AI Agent Development Guidelines

**CRITICAL**: This file contains mandatory rules for AI agents working on agentz.
Non-compliance will result in rejected code.

---

## Quick Reference for AI Agents

### BEFORE YOU START ANY TASK

**YOU MUST verify these are installed:**

```bash
go version  # Must be 1.21 or higher
command -v golangci-lint || echo "Install from https://golangci-lint.run/usage/install/"
```

**YOU MUST run these commands and ALL MUST PASS:**

```bash
# 1. Format code
make fmt

# 2. Run go vet
make vet

# 3. Lint with zero warnings
make lint

# 4. Run all tests (must achieve >70% coverage)
make test
```

**Expected Output**: All commands complete successfully with zero errors and zero warnings.

### AFTER YOU COMPLETE ANY TASK

**YOU MUST verify:**

- [ ] `make fmt` applied successfully
- [ ] `make vet` passes with zero errors
- [ ] `make lint` shows zero warnings
- [ ] `make test` passes with >70% coverage
- [ ] Documentation file created in `docs/explanations/` with lowercase_filename.md

**IF ANY CHECK FAILS, YOU MUST FIX IT BEFORE PROCEEDING.**

---

## CRITICAL RULES - NEVER VIOLATE

### Rule 1: File Extensions (MOST VIOLATED)

**YOU MUST:**

- Use `.yaml` extension for ALL YAML files
- Use `.md` extension for ALL Markdown files
- Use `.go` extension for ALL Go files

**NEVER:**

- ❌ Use `.yml` extension (even though common in industry)
- ❌ Use `.MD` or `.markdown` extensions

**Examples:**

```text
✅ CORRECT:
   config/production.yaml
   config/development.yaml
   .github/workflows/test.yaml

❌ WRONG:
   config/production.yml
   config/development.yml
   .github/workflows/test.yml
```

**Why This Matters**: CI/CD pipelines expect `.yaml`. Using `.yml` will cause build failures.

### Rule 2: Markdown File Naming (SECOND MOST VIOLATED)

**YOU MUST:**

- Use lowercase letters ONLY
- Use underscores to separate words
- Exception: `README.md` is the ONLY uppercase filename allowed

**NEVER:**

- ❌ Use CamelCase (ImplementationPlan.md)
- ❌ Use kebab-case (implementation-plan.md)
- ❌ Use spaces (Implementation Plan.md)
- ❌ Use uppercase (IMPLEMENTATION_PLAN.md)

**Examples:**

```text
✅ CORRECT:
   docs/explanations/component_architecture.md
   docs/how_to/setup_development.md
   docs/reference/api_specification.md
   README.md (ONLY exception)

❌ WRONG:
   docs/explanations/Component-Architecture.md
   docs/explanations/ComponentArchitecture.md
   docs/explanations/ARCHITECTURE.md
   docs/how_to/setup-development.md
   docs/how_to/Setup Development.md
```

**Why This Matters**: Inconsistent naming breaks documentation linking and makes files hard to find.

### Rule 3: No Emojis Anywhere (THIRD MOST VIOLATED)

**YOU MUST:**

- Write ALL documentation without emojis
- Write ALL code comments without emojis
- Write ALL commit messages without emojis

**NEVER:**

- ❌ Use emojis in code: `// ✅ This function works`
- ❌ Use emojis in docs: `## Setup Guide 🚀`
- ❌ Use emojis in commits: `feat: add auth ✨`

**ONLY EXCEPTION**: This AGENTS.md file uses emojis for visual markers to help you follow rules.

**Why This Matters**: Emojis cause encoding issues and make documentation unprofessional.

### Rule 4: Code Quality Gates (MUST ALL PASS)

**YOU MUST ensure ALL of these pass before claiming task complete:**

```bash
# Run in this exact order:

# 1. Format (auto-fixes issues)
make fmt

# 2. Vet (catches common mistakes)
make vet

# 3. Lint (treats warnings as errors)
make lint

# 4. Tests (must have >70% coverage)
make test
```

**Expected Results:**

```text
✅ make fmt      → "Formatting code..."
✅ make vet      → "Running go vet..." with 0 errors
✅ make lint     → "0 issues."
✅ make test     → "PASS" with coverage report
```

**IF ANY FAIL**: Stop immediately and fix before proceeding.

### Rule 5: Documentation is Mandatory

**YOU MUST:**

- Create documentation file in `docs/explanations/` for EVERY feature/task
- Use filename pattern: `{feature_name}_implementation.md` or `{phase}_summary.md`
- Include: Overview, Components, Implementation Details, Testing, Examples
- Add package comments to EVERY package
- Add doc comments to EVERY exported function, type, constant, variable
- Follow Go documentation conventions (complete sentences, start with name)

**NEVER:**

- ❌ Skip documentation because "code is self-documenting"
- ❌ Put documentation in wrong directory
- ❌ Forget to specify language in code blocks

**Examples:**

```go
// Package component provides functionality for loading and managing
// markdown components for the agentz template system.
package component

// Component represents a markdown component with metadata.
// It contains the component's name, category, content, and validation rules.
type Component interface {
    Name() string
    Category() string
    Content() string
    Metadata() *Metadata
    Validate() error
}

// Load reads a component from the filesystem and returns a Component instance.
// It returns an error if the file cannot be read or parsed.
//
// Example:
//
//   loader := NewLoader()
//   comp, err := loader.Load(ctx, "components/python.md")
//   if err != nil {
//       log.Fatal(err)
//   }
func Load(ctx context.Context, path string) (Component, error) {
    // Implementation
}
```

**Documentation File Structure:**

```markdown
# Feature Name Implementation

## Overview

Brief description of what was implemented

## Components Delivered

- File 1: Description (X lines)
- File 2: Description (Y lines)

## Implementation Details

Technical explanation with code examples

## Testing

Test coverage and validation results

## Usage Examples

Complete, runnable examples

## References

- Link to architecture docs
- Link to related features
```

---

## Project Overview

### Identity

- **Name**: agentz
- **Type**: CLI tool for generating modular AGENTS.md files
- **Language**: Go 1.21+
- **Key Features**: Modular components, TOML configuration, template system, project detection

### Architecture (Layered Design)

**CRITICAL**: YOU MUST respect these layer boundaries:

```text
┌──────────────────────────────────────────────┐
│  CLI Layer (cmd/)                            │
│  - Command definitions, flags, help text     │
├──────────────────────────────────────────────┤
│  Public API Layer (pkg/)                     │
│  - component/ - Component management         │
│  - config/    - Configuration handling       │
│  - template/  - Template rendering           │
│  - project/   - Project detection            │
├──────────────────────────────────────────────┤
│  Internal Layer (internal/)                  │
│  - Implementation details (not exported)     │
└──────────────────────────────────────────────┘
```

**Package Dependencies (MUST FOLLOW):**

- ✅ cmd → pkg (CLI uses public packages)
- ✅ pkg → internal (public packages can use internal)
- ❌ internal → pkg (NEVER - internal is private)
- ❌ pkg → cmd (NEVER - packages shouldn't depend on CLI)

**Package Organization Rules:**

- `cmd/` - Only CLI-related code (commands, flags, execution)
- `pkg/` - Public APIs that could be imported by other projects
- `internal/` - Private implementation details, helpers, utilities
- `docs/` - All documentation (Diataxis framework)
- `examples/` - Example AGENTS.md files and templates

---

## Development Workflow

### Step-by-Step Process (FOLLOW EXACTLY)

#### Phase 1: Preparation

1. **Understand the Task**
   - Read requirements completely
   - Identify which packages are affected
   - Check for existing similar code

2. **Search Existing Code**

   ```bash
   # Find relevant files
   grep -r "FunctionName" pkg/
   find pkg/ -name "*feature*.go"
   ```

3. **Plan Changes**
   - List files to create/modify
   - Identify tests needed
   - Determine documentation category

#### Phase 2: Implementation

1. **Write Code**

   ```go
   // Package example demonstrates the documentation pattern.
   package example

   import (
       "context"
       "fmt"
   )

   // Process processes the input and returns a result.
   // It validates the input before processing.
   //
   // Parameters:
   //   - ctx: Context for cancellation and timeouts
   //   - input: The input string to process
   //
   // Returns:
   //   - string: The processed result
   //   - error: An error if processing fails
   //
   // Example:
   //
   //   result, err := Process(ctx, "test")
   //   if err != nil {
   //       log.Fatal(err)
   //   }
   func Process(ctx context.Context, input string) (string, error) {
       if input == "" {
           return "", fmt.Errorf("input cannot be empty")
       }
       // Implementation
       return input, nil
   }
   ```

2. **Write Tests (MANDATORY)**

   ```go
   package example

   import (
       "context"
       "testing"
   )

   func TestProcess_ValidInput(t *testing.T) {
       // Arrange
       ctx := context.Background()
       input := "test"

       // Act
       result, err := Process(ctx, input)

       // Assert
       if err != nil {
           t.Fatalf("expected no error, got %v", err)
       }
       if result != input {
           t.Errorf("expected %s, got %s", input, result)
       }
   }

   func TestProcess_EmptyInput(t *testing.T) {
       ctx := context.Background()
       _, err := Process(ctx, "")
       if err == nil {
           t.Error("expected error for empty input")
       }
   }

   func TestProcess_WithCancellation(t *testing.T) {
       ctx, cancel := context.WithCancel(context.Background())
       cancel() // Cancel immediately

       _, err := Process(ctx, "test")
       if err == nil {
           t.Error("expected context cancellation error")
       }
   }
   ```

3. **Run Quality Checks Incrementally**

   ```bash
   # After writing code
   make fmt
   make vet

   # After writing tests
   make test

   # Before committing - verify all checks pass
   make verify
   ```

#### Phase 3: Documentation

**YOU MUST create** `docs/explanations/{feature}_implementation.md`:

````markdown
# Feature Name Implementation

## Overview

Brief description of what was implemented and why.

## Components Delivered

- `pkg/path/file.go` (XXX lines) - Description
- `pkg/path/file_test.go` (YYY lines) - Test coverage
- `docs/explanations/feature.md` (ZZZ lines) - This document

Total: ~N,NNN lines

## Implementation Details

### Component 1: Name

Description with code examples:

```go
func Example() {
    // Code
}
```
````

### Component 2: Name

More details...

## Testing

Test coverage: XX% (must be >70%)

```text
PASS
coverage: XX.X% of statements
```

## Usage Examples

Complete, runnable examples:

```go
package main

import "github.com/bcsmit/agentz/pkg/component"

func main() {
    loader := component.NewLoader()
    // Usage example
}
```

## Validation Results

- ✅ `make fmt` passed
- ✅ `make vet` passed
- ✅ `make lint` shows zero warnings
- ✅ `make test` passed with >70% coverage
- ✅ Documentation complete

## References

- Architecture: `docs/explanations/architecture.md`
- Implementation Plan: `docs/explanations/implementation_plan.md`

````

#### Phase 4: Validation (CRITICAL)

**Run these commands and verify output:**

```bash
# 1. Format check
make fmt
# Expected: "Formatting code..."

# 2. Vet check
make vet
# Expected: "Running go vet..." with 0 errors

# 3. Lint check
make lint
# Expected: "0 issues."

# 4. Test check
make test
# Expected: "PASS" with coverage >70%

# 5. Full verification
make verify
# Expected: "All checks passed!"

# 6. Verify documentation created
ls -la docs/explanations/*{feature}*.md
# Expected: File exists with lowercase filename

# 7. Verify no emoji in docs
grep -r "[😀-🙏]" docs/ --exclude="AGENTS.md"
# Expected: No matches
````

**IF ANY VALIDATION FAILS: Stop and fix immediately.**

---

## Go Coding Standards

### Error Handling (MANDATORY PATTERNS)

**YOU MUST:**

- Return `error` as the last return value for ALL functions that can fail
- Check errors immediately with `if err != nil`
- Wrap errors with context using `fmt.Errorf` with `%w` verb
- Use descriptive error messages that include context

**NEVER:**

- ❌ Ignore errors with `_`
- ❌ Use `panic()` for expected errors
- ❌ Return errors without context

**Correct Patterns:**

```go
// ✅ GOOD - Proper error handling
package config

import (
    "fmt"
    "os"

    "github.com/pelletier/go-toml/v2"
)

// Config represents the application configuration.
type Config struct {
    Version string `toml:"version"`
    Name    string `toml:"name"`
}

// Load reads and parses a configuration file.
// It returns an error if the file cannot be read or parsed.
func Load(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("failed to read config file %s: %w", path, err)
    }

    var config Config
    if err := toml.Unmarshal(data, &config); err != nil {
        return nil, fmt.Errorf("failed to parse config file %s: %w", path, err)
    }

    if err := config.Validate(); err != nil {
        return nil, fmt.Errorf("invalid configuration: %w", err)
    }

    return &config, nil
}

// Validate checks if the configuration is valid.
func (c *Config) Validate() error {
    if c.Name == "" {
        return fmt.Errorf("name cannot be empty")
    }
    if c.Version == "" {
        return fmt.Errorf("version cannot be empty")
    }
    return nil
}

// ❌ BAD - Ignoring errors
func LoadBad(path string) *Config {
    data, _ := os.ReadFile(path) // NEVER
    var config Config
    _ = toml.Unmarshal(data, &config) // NEVER
    return &config
}

// ⚠️ ACCEPTABLE - panic with justification
func MustLoad(path string) *Config {
    config, err := Load(path)
    if err != nil {
        // Only acceptable during initialization/startup
        panic(fmt.Sprintf("FATAL: failed to load config: %v", err))
    }
    return config
}
```

### Testing Standards (MANDATORY)

**YOU MUST:**

- Write tests for ALL exported functions
- Test both success and failure cases
- Test edge cases and boundaries
- Achieve >70% code coverage
- Use descriptive test names: `Test{Function}_{Condition}`
- Use table-driven tests for multiple cases

**Test Structure Template:**

```go
package component

import (
    "context"
    "testing"
)

func TestLoad_ValidFile(t *testing.T) {
    ctx := context.Background()
    loader := NewLoader()

    component, err := loader.Load(ctx, "testdata/valid.md")
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if component.Name() != "python" {
        t.Errorf("expected name 'python', got %s", component.Name())
    }
}

func TestLoad_FileNotFound(t *testing.T) {
    ctx := context.Background()
    loader := NewLoader()

    _, err := loader.Load(ctx, "nonexistent.md")
    if err == nil {
        t.Fatal("expected error for nonexistent file")
    }
}

func TestLoad_InvalidFormat(t *testing.T) {
    ctx := context.Background()
    loader := NewLoader()

    _, err := loader.Load(ctx, "testdata/invalid.md")
    if err == nil {
        t.Fatal("expected error for invalid format")
    }
}

// Table-driven test for multiple cases
func TestValidate(t *testing.T) {
    tests := []struct {
        name    string
        input   *Component
        wantErr bool
    }{
        {
            name:    "valid component",
            input:   &Component{name: "test", category: "lang"},
            wantErr: false,
        },
        {
            name:    "empty name",
            input:   &Component{name: "", category: "lang"},
            wantErr: true,
        },
        {
            name:    "empty category",
            input:   &Component{name: "test", category: ""},
            wantErr: true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := tt.input.Validate()
            if (err != nil) != tt.wantErr {
                t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
            }
        })
    }
}
```

---

## Git Conventions

### Branch Naming (MANDATORY FORMAT)

**YOU MUST use this format:**

```text
{type}/{description}
```

**Types:**

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation only
- `refactor` - Code restructuring
- `test` - Adding tests
- `chore` - Maintenance

**Examples:**

```text
✅ CORRECT:
   feat/component-loader
   fix/config-validation
   docs/update-readme
   refactor/template-renderer

❌ WRONG:
   Feature/ComponentLoader   (uppercase)
   component-loader          (missing type)
   feat_component_loader     (underscore instead of slash)
```

### Commit Messages (MANDATORY FORMAT)

**Format:**

```text
<type>(<scope>): <description>

[optional body explaining why change was made]

[optional footer with breaking changes]
```

**Rules (MUST FOLLOW ALL):**

1. Type MUST be one of: `feat|fix|docs|style|refactor|perf|test|chore`
2. Scope is optional but recommended (e.g., component, config, cli)
3. Description MUST be lowercase
4. Description MUST use imperative mood ("add" not "added")
5. First line MUST be ≤72 characters
6. Blank line between subject and body (if body exists)

**Types Explained:**

- `feat` - New feature (adds functionality)
- `fix` - Bug fix (fixes incorrect behavior)
- `docs` - Documentation only (no code changes)
- `style` - Code formatting (no logic changes)
- `refactor` - Code restructuring (no behavior changes)
- `perf` - Performance improvements
- `test` - Adding/fixing tests
- `chore` - Build process, dependencies, tools

**Examples:**

```text
✅ CORRECT:
feat(component): add markdown component loader
fix(config): handle missing TOML fields gracefully
docs(readme): update installation instructions
refactor(template): simplify placeholder resolution
test(loader): add test for empty file handling

✅ CORRECT with body:
feat(template): add distributed component loading

Implements parallel loading of multiple components.
Adds caching layer for frequently accessed components.

❌ WRONG:
Added component loader                           # Wrong mood, no type
feat(component): Add Component Loader            # Wrong case
feat: add loader                                 # Missing scope
add component loader                             # No type
feat(component): add component loader feature that allows users to... # Too long
```

---

## Documentation Organization (Diataxis Framework)

**YOU MUST categorize documentation correctly:**

### Category 1: Tutorials (`docs/tutorials/`)

**Purpose**: Learning-oriented, step-by-step lessons

**Use for**:

- Getting started guides
- Learning path tutorials
- Hands-on examples

**Example**: `docs/tutorials/getting_started.md`

### Category 2: How-To Guides (`docs/how_to/`)

**Purpose**: Task-oriented, problem-solving recipes

**Use for**:

- Installation steps
- Configuration guides
- Troubleshooting procedures

**Example**: `docs/how_to/setup_development.md`

### Category 3: Explanations (`docs/explanations/`) ← DEFAULT FOR YOUR SUMMARIES

**Purpose**: Understanding-oriented, conceptual discussion

**Use for**:

- Architecture explanations
- Design decisions
- Implementation summaries ← **YOU TYPICALLY CREATE THESE**
- Concept clarifications

**Example**: `docs/explanations/component_architecture_implementation.md`

### Category 4: Reference (`docs/reference/`)

**Purpose**: Information-oriented, technical specifications

**Use for**:

- API documentation
- Configuration reference
- Command reference

**Example**: `docs/reference/api_specification.md`

### Decision Tree: Where to Put Documentation?

```text
Is it a step-by-step tutorial?
├─ YES → docs/tutorials/
└─ NO
   ├─ Is it solving a specific task?
   │  ├─ YES → docs/how_to/
   │  └─ NO
   │     ├─ Is it explaining concepts/architecture?
   │     │  ├─ YES → docs/explanations/  ← MOST COMMON FOR AI AGENTS
   │     │  └─ NO
   │     │     └─ Is it reference material?
   │     │        └─ YES → docs/reference/
```

---

## Common Pitfalls and How to Avoid Them

### Pitfall 1: Using `.yml` Instead of `.yaml`

**ISSUE**: `.yml` is common in industry, so agents default to it

**WHY IT FAILS**: Our CI/CD expects `.yaml` extension only

**PREVENTION**:

```bash
# ✅ Before creating any YAML file, use full extension
touch config/production.yaml

# ❌ Never use short extension
touch config/production.yml  # Will cause CI failure
```

**FIX IF YOU MADE THIS MISTAKE**:

```bash
# Rename all .yml to .yaml
find . -name "*.yml" -exec sh -c 'mv "$0" "${0%.yml}.yaml"' {} \;
```

### Pitfall 2: Uppercase or CamelCase in Documentation Filenames

**ISSUE**: Agents use CamelCase or capitalization for readability

**WHY IT FAILS**: Breaks documentation links, inconsistent naming

**PREVENTION**:

```bash
# ✅ Always use lowercase_with_underscores
touch docs/explanations/component_loader_implementation.md

# ❌ Never use these patterns
touch docs/explanations/ComponentLoaderImplementation.md  # CamelCase
touch docs/explanations/Component-Loader-Implementation.md # Capitalized
touch docs/explanations/COMPONENT_LOADER.md # Uppercase
```

**FIX IF YOU MADE THIS MISTAKE**:

```bash
# Rename to lowercase with underscores
mv docs/explanations/ComponentLoader.md \
   docs/explanations/component_loader.md
```

### Pitfall 3: Forgetting to Run `go fmt`

**ISSUE**: Code works but fails CI due to formatting

**WHY IT FAILS**: CI runs formatting checks which fail if code isn't formatted

**PREVENTION**:

```bash
# ALWAYS run before committing
make fmt

# Or directly
go fmt ./...
```

**FIX IF CI FAILS**:

```bash
make fmt
git add -u
git commit --amend --no-edit
```

### Pitfall 4: Not Handling Errors Properly

**ISSUE**: Code works in testing but fails silently in production

**WHY IT FAILS**: Ignored errors cause unexpected behavior

**PREVENTION**:

```go
// ❌ BAD - Ignoring error
data, _ := os.ReadFile("config.yaml")

// ✅ GOOD - Handles error gracefully
data, err := os.ReadFile("config.yaml")
if err != nil {
    return fmt.Errorf("failed to read config: %w", err)
}
```

**FIX IF YOU MADE THIS MISTAKE**:

```bash
# Find all ignored errors
grep -rn "_, _ =" pkg/
grep -rn ", _" pkg/

# Replace with proper error handling
```

### Pitfall 5: Missing Documentation File

**ISSUE**: Task complete but no documentation created

**WHY IT FAILS**: Knowledge is lost, future developers confused

**PREVENTION**:

```bash
# Immediately after starting a task, create doc file
touch docs/explanations/feature_name_implementation.md

# Fill it in as you work
# Add final validation section when done
```

### Pitfall 6: Emojis in Documentation

**ISSUE**: Emojis used for "visual appeal"

**WHY IT FAILS**: Encoding issues, unprofessional, breaks tooling

**PREVENTION**:

```markdown
<!-- ❌ BAD -->

# Setup Guide 🚀

## Prerequisites ✅

<!-- ✅ GOOD -->

# Setup Guide

## Prerequisites
```

**FIX IF YOU MADE THIS MISTAKE**:

```bash
# Find all emoji usage (excluding AGENTS.md)
grep -r "[😀-🙏]" docs/ --exclude="AGENTS.md"

# Remove manually
```

### Pitfall 7: Ignoring Linter Warnings

**ISSUE**: "It's just a warning, not an error"

**WHY IT FAILS**: CI treats warnings as errors in our configuration

**PREVENTION**:

```bash
# Fix ALL warnings before committing
make lint

# If you see warnings, fix them one by one
# Re-run after each fix to ensure no new warnings introduced
```

---

## Emergency Procedures

### When Quality Checks Fail

**SYSTEMATIC DEBUG PROCESS:**

```bash
# Step 1: Fix formatting (always do this first)
make fmt

# Step 2: Fix vet errors
make vet
# Read each error message
# Fix root cause, not symptoms
# Re-run after each fix

# Step 3: Fix linter warnings (one at a time)
make lint
# Fix first warning
# Re-run lint
# Repeat until zero warnings

# Step 4: Fix failing tests
make test
# Read test failure output
# Fix failing tests or update expectations
# Re-run tests

# Step 5: Verify all checks pass
make verify
```

### When Tests Fail

**DIAGNOSTIC COMMANDS:**

```bash
# Run with verbose output
go test -v ./...

# Run specific test
go test -v -run TestName ./pkg/component

# Run tests with coverage
go test -v -cover ./...

# Run with race detection
go test -v -race ./...

# Show detailed output
go test -v ./... 2>&1 | less
```

**DEBUGGING STRATEGY:**

1. Read the test failure message carefully
2. Understand what the test expects
3. Add `t.Logf()` or print statements to see actual values
4. Fix the code or update the test
5. Re-run until passing

### When Linter Reports Warnings

**FIXING PROCESS:**

```bash
# List all warnings
make lint 2>&1 | grep "warning:"

# Fix warnings by category:

# 1. Unused code
#    - Remove if truly unused
#    - Mark with _ if intentionally unused

# 2. Error handling
#    - Add proper error checks
#    - Don't ignore errors with _

# 3. Exported items without comments
#    - Add doc comments starting with item name

# 4. Complexity warnings
#    - Refactor complex functions
#    - Extract helper functions

# Re-run after each fix
make lint
```

---

## Validation Checklist

**BEFORE CLAIMING TASK IS COMPLETE, VERIFY ALL:**

### Code Quality

- [ ] `make fmt` applied successfully
- [ ] `make vet` passes with zero errors
- [ ] `make lint` shows zero warnings
- [ ] `make test` passes with >70% coverage
- [ ] No ignored errors (using `_`)
- [ ] All exported items have doc comments
- [ ] All functions have at least 2 tests (success and failure case)

### Testing

- [ ] Unit tests added for ALL new functions
- [ ] Integration tests added if needed
- [ ] Test count increased from before
- [ ] Both success and failure cases tested
- [ ] Edge cases and boundaries covered
- [ ] All tests use descriptive names: `Test{Function}_{Condition}`

### Documentation

- [ ] Documentation file created in `docs/explanations/`
- [ ] Filename uses lowercase_with_underscores.md
- [ ] README.md exception is ONLY uppercase filename
- [ ] No emojis anywhere in documentation
- [ ] All code blocks specify language (go, bash, text, yaml)
- [ ] Documentation includes: Overview, Components, Details, Testing, Examples

### Files and Structure

- [ ] All YAML files use `.yaml` extension (NOT `.yml`)
- [ ] All Markdown files use `.md` extension
- [ ] No uppercase in filenames except `README.md`
- [ ] Files placed in correct directory (cmd/, pkg/, internal/)
- [ ] Documentation in correct Diataxis category

### Git

- [ ] Branch name follows `{type}/{description}` format (lowercase)
- [ ] Commit message follows conventional commits
- [ ] Commit message first line ≤72 characters
- [ ] Commit uses imperative mood ("add" not "added")

### Architecture

- [ ] Changes respect package boundaries
- [ ] No circular dependencies introduced
- [ ] Proper separation of concerns maintained
- [ ] Internal packages don't export to pkg

---

## Quick Command Reference

### Essential Go Commands

```bash
# Build and check
go build ./...                    # Build all packages
go build -o bin/agentz main.go   # Build binary
go install                        # Install to $GOPATH/bin

# Quality
go fmt ./...                      # Format all code
go vet ./...                      # Check for common mistakes
go mod tidy                       # Clean up dependencies
go mod verify                     # Verify dependencies

# Testing
go test ./...                     # Run all tests
go test -v ./...                  # Verbose output
go test -cover ./...              # With coverage
go test -race ./...               # With race detection
go test -run TestName ./...       # Specific test

# Documentation
go doc package                    # Show package docs
go doc package.Symbol             # Show symbol docs

# Maintenance
go clean                          # Remove build artifacts
go mod download                   # Download dependencies
go list -m all                    # List all dependencies
```

### Project-Specific Commands (Makefile)

```bash
# Quality validation workflow
make fmt                          # Format code
make vet                          # Run go vet
make lint                         # Run golangci-lint
make test                         # Run tests with coverage

# Build commands
make build                        # Build binary
make clean                        # Remove artifacts
make install                      # Install to $GOPATH/bin

# Combined commands
make verify                       # Run all checks (fmt, vet, lint, test)
make dev                          # Build and run with --help
make coverage                     # Generate coverage report

# Help
make help                         # Show all available targets
```

---

## Summary: The Three Golden Rules

**If you remember nothing else, remember these:**

### Rule 1: File Extensions

```text
.yaml (NOT .yml)
.md (NOT .MD or .markdown)
.go for all Go source files
```

### Rule 2: Documentation Filenames

```text
lowercase_with_underscores.md
Exception: README.md ONLY
```

### Rule 3: Quality Checks

```text
All four make commands MUST pass before claiming done:
- make fmt
- make vet
- make lint
- make test
```

---

## The Golden Workflow

**FOLLOW THIS SEQUENCE FOR EVERY TASK:**

```text
1. Create branch: {type}/{description}
2. Implement code with proper doc comments
3. Add tests (>70% coverage)
4. Run: make fmt
5. Run: make vet
6. Run: make lint
7. Run: make test
8. Create: docs/explanations/{feature}_implementation.md
9. Commit with proper format: <type>(<scope>): <description>
10. Verify: All checklist items above are checked
```

**IF YOU FOLLOW THIS WORKFLOW, YOUR CODE WILL BE ACCEPTED.**

**IF YOU SKIP STEPS OR VIOLATE RULES, YOUR CODE WILL BE REJECTED.**

---

## Living Document

This AGENTS.md file is actively maintained and updated as the project evolves.

**Last Updated**: Section 1.1 completion (Project Setup and Architecture)

**Version**: 1.0

**Feedback**: If you find rules unclear or need clarification, document your questions in your implementation summary for review.
