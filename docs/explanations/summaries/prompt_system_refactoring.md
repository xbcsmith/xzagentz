# Prompt System Refactoring: Bash to Rust

## Overview

This document explains the refactoring of the prompt generation system from standalone bash scripts to a fully integrated Rust-based system within the xzagentz CLI.

**Date**: 2024-01-15
**Status**: Design Complete, Implementation Pending
**Impact**: Phase 7 of Implementation Plan

## Motivation

The original `agents_prompt_system.md` described a bash-based system with standalone scripts:

- `scripts/generate_prompts.sh` - Main prompt generator
- `scripts/copy_prompt.sh` - Prompt clipboard helper
- `scripts/verify_compliance.sh` - Compliance checker

**Problems with Bash Approach**:

1. **Fragmentation**: Separate scripts outside main codebase
2. **Limited Error Handling**: Bash error handling is verbose and error-prone
3. **No Type Safety**: No compile-time guarantees of correctness
4. **Platform Dependency**: Shell scripts behave differently across platforms
5. **Testing Challenges**: Difficult to unit test shell scripts
6. **Maintenance Burden**: Separate testing and release workflow
7. **No Integration**: Cannot share code with main xzagentz CLI

**Benefits of Rust Approach**:

1. **Unified Codebase**: All functionality in single Rust project
2. **Type Safety**: Compile-time guarantees prevent runtime errors
3. **Better Testing**: Comprehensive unit and integration tests
4. **Cross-Platform**: Works identically on Linux, macOS, Windows
5. **Performance**: Native binary performance vs interpreted scripts
6. **Code Reuse**: Share parsers, validators, templates with CLI
7. **Single Binary**: One executable for all xzagentz functionality

## Architectural Changes

### Original Bash Architecture

```text
scripts/
├── generate_prompts.sh      # Standalone script
├── copy_prompt.sh           # Standalone script
└── verify_compliance.sh     # Standalone script

prompts/                     # Generated prompts
└── prompt_X_Y.md

.implementation_progress     # Progress file (plain text)
```

### New Rust Architecture

```text
src/
├── prompts/
│   ├── mod.rs              # Module declaration
│   ├── parser.rs           # Implementation plan parser
│   ├── generator.rs        # Prompt generator
│   ├── template.rs         # Prompt template rendering
│   ├── progress.rs         # Progress tracking
│   └── compliance.rs       # AGENTS.md compliance checker
└── main.rs                 # CLI integration

prompts/                    # Generated prompts
└── prompt_X_Y.md

.implementation_progress    # Progress file (TOML)
```

### Module Responsibilities

#### `parser.rs` - Implementation Plan Parser

**Purpose**: Parse markdown implementation plans to extract structured data

**Key Types**:

```rust
pub struct ImplementationPlan {
    pub phases: Vec<Phase>,
    pub metadata: PlanMetadata,
}

pub struct Phase {
    pub number: u32,
    pub name: String,
    pub duration: String,
    pub goal: String,
    pub sections: Vec<Section>,
}

pub struct Section {
    pub number: String,
    pub name: String,
    pub tasks: Vec<String>,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub tests: Vec<String>,
}
```

**Responsibilities**:

- Parse markdown headers (##, ###)
- Extract phase metadata
- Extract section metadata
- Parse bullet lists for deliverables/criteria
- Handle malformed markdown gracefully

#### `template.rs` - Prompt Template Rendering

**Purpose**: Render prompts from templates with section data

**Key Types**:

```rust
pub struct PromptTemplate {
    sections: Vec<TemplateSection>,
}

pub enum TemplateSection {
    Context,
    CriticalRules,
    Task,
    PreFlightChecklist,
    ImplementationChecklist,
    Deliverables,
    AcceptanceCriteria,
    PostImplementationReview,
}
```

**Responsibilities**:

- Define prompt structure
- Render markdown from templates
- Fill in section-specific data
- Apply AGENTS.md rules per section type
- Support template customization

#### `generator.rs` - Prompt Generator

**Purpose**: Orchestrate prompt generation workflow

**Key Types**:

```rust
pub struct PromptGenerator {
    template: PromptTemplate,
    compliance_rules: ComplianceRules,
}
```

**Responsibilities**:

- Generate prompts for all sections (batch mode)
- Generate prompts one at a time (interactive mode)
- Write prompts to files
- Prevent accidental overwrites
- Coordinate with parser and template

#### `progress.rs` - Progress Tracking

**Purpose**: Track implementation progress across sessions

**Key Types**:

```rust
pub struct ProgressTracker {
    state_file: PathBuf,
    state: ProgressState,
}

pub struct ProgressState {
    pub current_section: Option<String>,
    pub completed_sections: Vec<String>,
    pub total_sections: usize,
    pub last_updated: DateTime<Utc>,
}
```

**Responsibilities**:

- Save progress to `.implementation_progress` (TOML format)
- Load progress on startup
- Mark sections as complete
- Calculate completion percentage
- Identify next section to implement
- Support progress reset

#### `compliance.rs` - Compliance Verification

**Purpose**: Verify project compliance with AGENTS.md rules

**Key Types**:

```rust
pub struct ComplianceChecker {
    rules: Vec<ComplianceRule>,
}

pub struct ComplianceRule {
    pub id: String,
    pub name: String,
    pub category: RuleCategory,
    pub check: Box<dyn Fn(&Path) -> Result<Vec<Violation>>>,
}

pub enum RuleCategory {
    Markdown,
    Yaml,
    Rust,
    Git,
    Shell,
}

pub struct Violation {
    pub rule_id: String,
    pub file: PathBuf,
    pub line: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}
```

**Responsibilities**:

- Check markdown file naming (snake_case)
- Check fenced code blocks (language identifiers)
- Check YAML extensions (`.yaml` not `.yml`)
- Check for emojis in documentation
- Check shell script conventions
- Check Rust formatting/linting
- Generate violation reports
- Provide fix suggestions
- Support CI/CD integration (exit codes)

## CLI Command Changes

### Original Bash Commands

```bash
# Generate prompts
./scripts/generate_prompts.sh
./scripts/generate_prompts.sh --batch

# Copy prompt to clipboard
./scripts/copy_prompt.sh 1.1
./scripts/copy_prompt.sh --list

# Verify compliance
./scripts/verify_compliance.sh
```

### New Rust CLI Commands

```bash
# Generate prompts
xzagentz prompt generate
xzagentz prompt generate --batch
xzagentz prompt generate --plan custom_plan.md

# Show/list prompts
xzagentz prompt show 1.1
xzagentz prompt show 1.1 --copy
xzagentz prompt list

# Progress tracking
xzagentz prompt progress
xzagentz prompt complete 1.1
xzagentz prompt next
xzagentz prompt reset

# Compliance verification
xzagentz prompt verify
xzagentz prompt verify --strict
xzagentz prompt verify --report compliance_report.md
xzagentz prompt verify --category markdown
```

## Feature Comparison

| Feature | Bash Version | Rust Version | Improvement |
|---------|--------------|--------------|-------------|
| Prompt Generation | ✓ | ✓ | Type-safe parsing |
| Interactive Mode | ✓ | ✓ | Better UX |
| Batch Mode | ✓ | ✓ | Faster execution |
| Progress Tracking | ✓ (plain text) | ✓ (TOML) | Structured format |
| Compliance Checking | ✓ (basic) | ✓ (comprehensive) | More rules, better reports |
| Clipboard Support | ✓ | ✓ | Cross-platform |
| Custom Templates | ✗ | ✓ | NEW FEATURE |
| CI/CD Integration | ✗ | ✓ | NEW FEATURE |
| Verification Reports | ✗ | ✓ | NEW FEATURE |
| Fix Suggestions | ✗ | ✓ | NEW FEATURE |
| Unit Tests | ✗ | ✓ | Full coverage |
| Cross-Platform | Partial | ✓ | Windows support |

## Migration Guide

### For Users

**Before (Bash)**:

```bash
# 1. Make scripts executable
chmod +x scripts/*.sh

# 2. Generate prompts
./scripts/generate_prompts.sh

# 3. Copy prompt
./scripts/copy_prompt.sh 1.1

# 4. Verify compliance
./scripts/verify_compliance.sh
```

**After (Rust)**:

```bash
# 1. Install xzagentz (single binary)
cargo install --path .

# 2. Generate prompts
xzagentz prompt generate

# 3. Show prompt
xzagentz prompt show 1.1 --copy

# 4. Verify compliance
xzagentz prompt verify
```

### For Developers

**Progress File Format Change**:

Before (`.implementation_progress`):

```text
5
```

After (`.implementation_progress`):

```toml
current_section = "1.2"
last_updated = "2024-01-15T10:30:00Z"

completed_sections = [
    "1.1",
]

[statistics]
total_sections = 27
completed = 1
percentage = 3.7
```

**Compliance Report Format** (new):

```markdown
# AGENTS.md Compliance Report

Generated: 2024-01-15 10:30:00 UTC

## Summary

- Total Rules Checked: 15
- Passed: 13
- Failed: 2
- Overall Status: FAIL

## Violations

### [MD-001] Markdown File Naming

**File**: `docs/explanations/DistributedTracing.md`
**Issue**: Filename uses CamelCase instead of snake_case
**Suggestion**: Rename to `distributed_tracing.md`

...
```

## Implementation Plan Changes

### Phase 7 Restructured

**Original Phase 7** (3 subsections):

- 7.1 Prompt Generator
- 7.2 Progress Tracking
- 7.3 Compliance Verification

**New Phase 7** (6 subsections):

- 7.1 Implementation Plan Parser
- 7.2 Prompt Template System
- 7.3 Prompt Generator
- 7.4 Progress Tracking
- 7.5 Compliance Verification
- 7.6 CLI Integration

**Rationale**: More granular breakdown for better testability and clearer deliverables

### Testing Strategy

Each subsection includes comprehensive unit tests:

```rust
// Example from 7.1 Implementation Plan Parser
#[test]
fn test_parse_implementation_plan()
#[test]
fn test_extract_phases()
#[test]
fn test_extract_sections()
#[test]
fn test_parse_deliverables()
#[test]
fn test_parse_acceptance_criteria()
#[test]
fn test_handle_malformed_markdown()
```

## Dependencies

### New Crate Dependencies

```toml
[dependencies]
# Existing
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
thiserror = "1.0"
anyhow = "1.0"

# New for prompt system
chrono = { version = "0.4", features = ["serde"] }
regex = "1.10"
walkdir = "2.4"

[dev-dependencies]
tempfile = "3.8"
assert_cmd = "2.0"
predicates = "3.0"
```

## Success Metrics

### Code Quality

- [ ] All modules pass `cargo fmt --check`
- [ ] All modules pass `cargo clippy -- -D warnings`
- [ ] Test coverage > 80% for all prompt modules
- [ ] All public APIs documented with examples

### Functionality

- [ ] Parse implementation plans correctly
- [ ] Generate valid markdown prompts
- [ ] Track progress persistently
- [ ] Verify all AGENTS.md rules
- [ ] Generate detailed compliance reports
- [ ] Support batch and interactive modes
- [ ] Work on Linux, macOS, Windows

### Performance

- [ ] Generate all prompts in < 1 second
- [ ] Compliance verification completes in < 5 seconds
- [ ] Binary size < 10 MB

## Risks and Mitigation

### Risk 1: Parsing Complexity

**Risk**: Markdown parsing may be fragile and break with plan format changes

**Mitigation**:

- Use well-tested regex patterns
- Comprehensive test suite with edge cases
- Graceful error handling with clear messages
- Document expected markdown format

### Risk 2: Progress File Corruption

**Risk**: TOML progress file could become corrupted

**Mitigation**:

- Validate TOML on load
- Backup progress file before writing
- Support manual progress reset
- Handle corrupted files gracefully

### Risk 3: Platform Differences

**Risk**: Clipboard access may differ across platforms

**Mitigation**:

- Use platform-agnostic clipboard library
- Provide fallback (print to stdout)
- Document platform-specific behavior
- Test on all major platforms

## Timeline

**Phase 7 Duration**: Week 8 (1 week)

**Subsection Breakdown**:

- Day 1-2: Parser (7.1) + Template (7.2)
- Day 3: Generator (7.3)
- Day 4: Progress (7.4)
- Day 5: Compliance (7.5)
- Day 6: CLI Integration (7.6)
- Day 7: Integration testing + documentation

## Next Steps

1. **Review and Approve**: Review this refactoring plan
2. **Begin Implementation**: Start with Phase 7.1 (Parser)
3. **Incremental Testing**: Test each module before moving to next
4. **Documentation**: Update `agents_prompt_system.md` as features complete
5. **Integration**: Integrate with main CLI in Phase 7.6
6. **End-to-End Testing**: Test complete workflow in Phase 8

## Conclusion

The refactoring from bash scripts to a Rust-based prompt system provides:

- **Better Integration**: Single binary, unified codebase
- **Type Safety**: Compile-time guarantees of correctness
- **Enhanced Features**: Compliance reports, fix suggestions, CI/CD support
- **Cross-Platform**: Consistent behavior on all platforms
- **Maintainability**: Easier to test, extend, and debug
- **Performance**: Native binary speed

This aligns perfectly with the xzagentz project's Rust-first approach and provides a solid foundation for automated implementation workflow management.

## References

- [agents_prompt_system.md](../../agents_prompt_system.md) - Refactored system design
- [agents_template_system.md](../../agents_template_system.md) - Template system pattern
- [implementation_plan.md](./implementation_plan.md) - Phase 7 details
- [AGENTS.md](../../AGENTS.md) - Project rules and conventions
