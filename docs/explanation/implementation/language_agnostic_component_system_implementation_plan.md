# Language-Agnostic Component System Implementation Plan

## Overview

This document provides a comprehensive, actionable plan to refactor the xzagentz component system from a Rust-specific implementation to a language-agnostic system that injects language-specific instructions conditionally. The goal is to support multiple programming languages (Python, Go, Rust, TypeScript, Bash) while maintaining concise, focused output files.

## Executive Summary

**Problem**: Current component system contains Rust-specific examples in core and general components, making them unsuitable for projects in other languages.

**Solution**: Implement a hybrid component system with:
- Language-agnostic core content with conditional language-specific sections
- Template rendering engine that filters content based on target language
- Component tiering (essential vs comprehensive) to control output size
- Strict size limits to ensure generated files remain under 1,200 lines

**Expected Outcome**: Generate clean, language-appropriate AGENTS.md files for any supported language, reducing file sizes by 60-75% while improving relevance.

## Goals and Success Criteria

### Primary Goals

1. Enable language-agnostic components with conditional language sections
2. Implement rendering engine that outputs only target language content
3. Achieve target file sizes: 1,000-1,200 lines for typical projects
4. Support Python, Go, Rust, TypeScript, and Bash equally
5. Maintain backward compatibility during transition

### Success Metrics

**Quantitative:**
- Generated file size: 1,000-1,200 lines (currently 3,700+ for Python)
- Language purity: 0% code examples in wrong language
- Test coverage: greater than 80% for all new code
- Component contribution limits: enforced per component type
- All quality gates pass: cargo fmt, check, clippy, test

**Qualitative:**
- Code examples match target language idioms
- No cross-language pollution in output
- Maintainable component structure
- Clear separation of concerns

## Architecture Design

### Component Structure

Each component file will have three parts:

1. YAML Frontmatter (metadata)
2. Language-Agnostic Content (always rendered)
3. Language-Specific Sections (conditionally rendered)

```markdown
---
component:
  name: critical_rules
  category: core
  version: 2.0.0
  languages:
    - python
    - golang
    - rust
    - typescript
    - bash
  sections:
    - id: error_handling
      language_specific: true
      languages: [python, golang, rust, typescript, bash]
    - id: testing_requirements
      language_specific: true
      languages: [python, golang, rust, typescript, bash]
---

# Critical Rules

Language-agnostic content here...

<!-- LANG:* -->
Generic error handling principles...
<!-- /LANG -->

<!-- LANG:rust -->
Rust-specific error handling with Result<T, E>...
<!-- /LANG -->

<!-- LANG:python -->
Python-specific error handling with try/except...
<!-- /LANG -->
```

### Language Section Syntax

Use HTML comment markers for language-specific sections:

```markdown
<!-- LANG:rust -->
Rust-specific content here
<!-- /LANG -->

<!-- LANG:python -->
Python-specific content here
<!-- /LANG -->

<!-- LANG:* -->
Language-agnostic fallback content
<!-- /LANG -->
```

**Marker Rules:**
- `<!-- LANG:language_name -->` starts a language-specific section
- `<!-- /LANG -->` ends a language-specific section
- `<!-- LANG:* -->` indicates agnostic/fallback content
- Supported languages: `rust`, `python`, `golang`, `typescript`, `bash`
- Markers must be on their own line
- Sections can be nested within markdown structure

### Rendering Algorithm

```
1. Parse component YAML frontmatter
2. Parse markdown content for language sections
3. Determine target language from config
4. For each language section:
   a. Check if language matches target (exact match)
   b. If match: include section content
   c. If no match: check for LANG:* fallback
   d. If fallback exists: include fallback content
   e. If no fallback: skip section
5. Assemble final output with only matched sections
6. Validate size constraints
7. Return rendered component
```

### Component Categories and Size Limits

| Category | Max Lines Per Component | Max Total Contribution |
|----------|-------------------------|------------------------|
| Core     | 150 lines              | 300 lines (all core)   |
| General  | 200 lines              | 600 lines (all general)|
| Language | 400 lines              | 400 lines (one lang)   |
| Tool Essential | 150 lines        | 300 lines (2-3 tools)  |
| Tool Comprehensive | 600 lines    | 1,200 lines (2 tools)  |
| **Total Project** | N/A          | **1,500 lines max**    |

## Implementation Phases

### Phase 1: Foundation and Parser (Week 1)

#### Tasks

1. **Design Metadata Schema**
   - Define YAML frontmatter structure
   - Specify required vs optional fields
   - Document metadata validation rules

2. **Implement Language Section Parser**
   - Create `src/components/language_filter.rs`
   - Parse HTML comment markers
   - Extract language-specific sections
   - Handle nested sections correctly
   - Support LANG:* fallback

3. **Implement Conditional Renderer**
   - Create `src/components/renderer.rs`
   - Filter sections by target language
   - Apply fallback logic
   - Enforce size constraints
   - Add debug logging with tracing

4. **Update Component Loader**
   - Parse YAML frontmatter in `src/components/loader.rs`
   - Validate metadata schema
   - Support both old and new formats

5. **Write Comprehensive Tests**
   - Unit tests for parser
   - Unit tests for renderer
   - Integration tests for full pipeline
   - Test all supported languages
   - Test fallback scenarios
   - Test error cases

#### Deliverables

- `src/components/language_filter.rs` (300-400 lines)
- `src/components/renderer.rs` (400-500 lines)
- Updated `src/components/loader.rs` (50-100 lines changed)
- Test suite (500+ lines)
- Documentation for new modules

#### Acceptance Criteria

- All tests pass with greater than 80% coverage
- Parser correctly identifies all language sections
- Renderer outputs only target language content
- Size limits enforced correctly
- No clippy warnings
- All documentation complete

### Phase 2: Core Component Refactoring (Week 2)

#### Tasks

1. **Refactor critical_rules.md**
   - Add YAML frontmatter with metadata
   - Create agnostic versions of all rules
   - Add language sections for:
     - Error handling (rust, python, golang, typescript, bash)
     - Testing requirements (all languages)
     - Documentation standards (all languages)
     - Quality gates (all languages)
   - Remove Rust-specific content from agnostic sections
   - Test generation for each language

2. **Refactor learning_resources.md**
   - Add YAML frontmatter
   - Create language-agnostic learning principles
   - Add language-specific resource sections
   - Use placeholders for project-specific paths
   - Test generation for each language

3. **Review header.md**
   - Verify if language-specific changes needed
   - Add metadata if required
   - Keep concise (target: 50-60 lines)

4. **Validate Output Quality**
   - Generate AGENTS.md for Python project
   - Generate AGENTS.md for Go project
   - Generate AGENTS.md for TypeScript project
   - Verify no cross-language pollution
   - Measure file sizes
   - Collect feedback

#### Deliverables

- Refactored `components/core/critical_rules.md` (400-500 lines source, 100-120 lines per language output)
- Refactored `components/core/learning_resources.md` (300-400 lines source, 80-100 lines output)
- Updated `components/core/header.md` if needed
- Test outputs for all languages (5 files)
- Quality comparison report

#### Acceptance Criteria

- Each core component has complete language coverage
- Generated outputs contain zero cross-language pollution
- File sizes meet targets for each language
- All examples are runnable and idiomatic
- Component validator passes all checks

### Phase 3: General Component Refactoring (Week 3)

#### Tasks

1. **Refactor testing.md**
   - Add YAML frontmatter
   - Create agnostic testing principles
   - Add language-specific sections:
     - Test structure (AAA pattern per language)
     - Test frameworks (pytest, cargo test, jest, etc.)
     - Mocking patterns (per language)
     - Coverage tools (per language)
   - Remove Rust-specific bias
   - Reduce verbosity (target: 150 lines output per language)

2. **Refactor development.md**
   - Add YAML frontmatter
   - Create agnostic development workflow
   - Add language-specific sections:
     - Dependency management (pip, cargo, npm, etc.)
     - Build commands (per language)
     - Linting/formatting (per language)
     - Debug tools (per language)
   - Reduce generic content (target: 180 lines output per language)

3. **Refactor documentation.md**
   - Add YAML frontmatter
   - Already has multi-language examples (good starting point)
   - Add missing languages (Rust, Bash)
   - Ensure consistent coverage
   - Apply conciseness pass (target: 120 lines output per language)

4. **Apply Conciseness Pass**
   - Remove philosophical discussions
   - Focus on actionable instructions
   - One example per concept
   - Remove redundant explanations
   - Keep only project-relevant guidance

#### Deliverables

- Refactored `components/general/testing.md` (500-600 lines source, 150 lines output)
- Refactored `components/general/development.md` (600-700 lines source, 180 lines output)
- Refactored `components/general/documentation.md` (400-500 lines source, 120 lines output)
- Updated component authoring guidelines
- Conciseness checklist for component authors

#### Acceptance Criteria

- All general components have complete language coverage
- Verbosity reduced by 40-50% in output
- Actionable instructions only (no philosophy)
- All examples are practical and relevant
- Output sizes meet targets

### Phase 4: Tool Component Tiering (Week 4)

#### Tasks

1. **Create Essential Tool Variants**
   - `components/tools/git_essential.md` (120 lines)
     - Commit message format
     - Branch naming conventions
     - Critical git rules only
   - `components/tools/markdown_essential.md` (100 lines)
     - File naming conventions
     - Basic formatting rules
     - Code block requirements
   - Add YAML frontmatter with tier metadata

2. **Update Comprehensive Tool Components**
   - Mark `components/tools/git.md` as comprehensive variant
   - Mark `components/tools/markdown.md` as comprehensive variant
   - Add YAML frontmatter
   - Ensure consistency with essential variants

3. **Implement Smart Defaults**
   - Update config to support component variants
   - Define project type mappings:
     - CLI tool: git_essential only
     - Library: git_essential + markdown_essential (if has docs)
     - Web service: git_essential
     - Documentation project: git_comprehensive + markdown_comprehensive
   - Add configuration options to override defaults

4. **Create Component Selection Logic**
   - Update loader to support variant selection
   - Implement tier-based filtering
   - Add configuration validation
   - Handle missing components gracefully

#### Deliverables

- `components/tools/git_essential.md` (120-150 lines)
- `components/tools/markdown_essential.md` (100-120 lines)
- Updated `components/tools/git.md` with tier metadata
- Updated `components/tools/markdown.md` with tier metadata
- Component selection logic in loader
- Configuration schema updates
- Smart defaults documentation

#### Acceptance Criteria

- Essential variants are concise and focused
- Comprehensive variants maintain full detail
- Smart defaults work for all project types
- Configuration allows manual override
- All tests pass for variant selection

### Phase 5: Size Enforcement and Validation (Week 5)

#### Tasks

1. **Implement Size Enforcement**
   - Add size tracking to renderer
   - Enforce per-component limits
   - Enforce total project limits
   - Add warning thresholds
   - Provide detailed size reports

2. **Create Validation Suite**
   - Generate outputs for all test scenarios:
     - Python CLI project
     - Go microservice
     - Rust library
     - TypeScript web app
     - Bash scripts
   - Measure and record sizes
   - Validate language purity
   - Check for cross-language pollution

3. **Collect Metrics**
   - File size distribution
   - Component contribution breakdown
   - Language coverage statistics
   - Quality scores per language
   - Before/after comparison

4. **Optimize Components**
   - Identify oversized components
   - Apply additional conciseness
   - Rebalance content distribution
   - Re-test and re-measure

5. **Edge Case Handling**
   - Test with minimal config
   - Test with maximal config
   - Test missing language sections
   - Test invalid metadata
   - Ensure graceful degradation

#### Deliverables

- Size enforcement in renderer (100-150 lines)
- Validation test suite (300-400 lines)
- Metrics collection report
- Optimized components (if needed)
- Edge case test coverage
- Performance benchmarks

#### Acceptance Criteria

- All project types meet size targets (1,000-1,200 lines)
- Size limits enforced correctly
- Detailed error messages for violations
- Metrics show 60-75% size reduction from baseline
- All edge cases handled gracefully
- Performance acceptable (sub-second generation)

### Phase 6: Documentation and Release (Week 6)

#### Tasks

1. **Update Component Authoring Guidelines**
   - Document new metadata format
   - Explain language section syntax
   - Provide examples for each section type
   - Define quality standards
   - Create component templates

2. **Create Migration Guide**
   - Document old vs new format
   - Provide migration scripts (if needed)
   - Explain backward compatibility
   - Define deprecation timeline
   - Provide troubleshooting guide

3. **Update Project Documentation**
   - Update README.md with new capabilities
   - Update architecture documentation
   - Document configuration options
   - Add language support matrix
   - Update CLI help text

4. **Create Examples**
   - Example components for each category
   - Example configurations for each language
   - Example generated outputs
   - Troubleshooting examples

5. **Write Release Notes**
   - Summarize changes
   - Highlight breaking changes
   - Document new features
   - Provide upgrade path
   - Link to migration guide

#### Deliverables

- `docs/how_to/authoring_components.md` (new)
- `docs/explanation/migration_guide.md` (new)
- Updated `README.md`
- Updated architecture docs
- Example components in `examples/components/`
- Example configurations in `examples/configs/`
- Release notes in `CHANGELOG.md`

#### Acceptance Criteria

- All documentation complete and accurate
- Examples are working and tested
- Migration guide is comprehensive
- Component authoring guidelines are clear
- Release notes capture all changes

## Technical Specifications

### YAML Frontmatter Schema

```yaml
---
component:
  name: string              # Component identifier (required)
  category: string          # core|general|languages|tools (required)
  version: string           # Semantic version (required)
  tier: string              # essential|comprehensive (optional, for tools)
  description: string       # Brief description (optional)
  languages:                # List of supported languages (optional)
    - rust
    - python
    - golang
    - typescript
    - bash
  sections:                 # Section metadata (optional)
    - id: string            # Section identifier
      language_specific: bool  # true if has language variants
      languages: [string]   # Languages with specific content
      required: bool        # true if section must be present
---
```

### Language Section Markers

**Format:**
```markdown
<!-- LANG:language_name -->
Content for specific language
<!-- /LANG -->
```

**Rules:**
1. Markers must be on their own line
2. No whitespace inside markers
3. Language names are lowercase
4. Use `*` for agnostic/fallback content
5. Sections can span multiple paragraphs
6. Code blocks must be within LANG sections
7. Nesting is allowed but not recommended

### Configuration Schema

```yaml
# In xzagentz.yaml or CLI options
project:
  type: cli|library|service|docs
  language: rust|python|golang|typescript|bash

components:
  tools:
    git: essential|comprehensive|none
    markdown: essential|comprehensive|none
  include:
    - custom_component_name
  exclude:
    - component_to_skip

limits:
  max_total_lines: 1500      # Hard limit
  warn_at_lines: 1200        # Warning threshold
  per_component_max: 300     # Per-component limit

rendering:
  strict_language: true      # Error on missing language section
  fallback_to_agnostic: true # Use LANG:* if specific missing
  include_metadata: false    # Include component metadata in output
```

### Error Handling

All operations must use `Result<T, E>` with descriptive errors:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Missing required frontmatter field: {0}")]
    MissingFrontmatter(String),

    #[error("Invalid language section marker at line {line}: {marker}")]
    InvalidMarker { line: usize, marker: String },

    #[error("No content found for language '{language}' in section '{section}'")]
    MissingLanguageContent { language: String, section: String },

    #[error("Component '{name}' exceeds size limit: {actual} > {limit} lines")]
    SizeExceeded { name: String, actual: usize, limit: usize },

    #[error("Unclosed language section starting at line {0}")]
    UnclosedSection(usize),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, ComponentError>;
```

## Code Structure

### New Files

```
src/components/
├── mod.rs                    (existing, update)
├── loader.rs                 (existing, update)
├── validator.rs              (existing, update)
├── language_filter.rs        (new)
├── renderer.rs               (new)
└── metadata.rs               (new)
```

### Module Responsibilities

**language_filter.rs**
- Parse language section markers
- Extract language-specific content
- Validate marker syntax
- Handle nested sections

**renderer.rs**
- Filter content by target language
- Apply fallback logic
- Enforce size constraints
- Assemble final output

**metadata.rs**
- Define metadata structures
- Parse YAML frontmatter
- Validate metadata schema
- Provide metadata queries

## Testing Strategy

### Unit Tests

**language_filter.rs**
- Parse simple language section
- Parse multiple language sections
- Parse nested sections
- Handle malformed markers
- Handle unclosed sections
- Extract correct content

**renderer.rs**
- Render with exact language match
- Render with fallback to agnostic
- Render with missing content
- Enforce size limits
- Handle empty content

**metadata.rs**
- Parse valid frontmatter
- Reject invalid frontmatter
- Validate required fields
- Handle optional fields

### Integration Tests

```rust
#[test]
fn test_render_rust_component() {
    let source = r#"
---
component:
  name: test
  category: core
  languages: [rust, python]
---
# Test Component

<!-- LANG:* -->
Agnostic content
<!-- /LANG -->

<!-- LANG:rust -->
Rust specific
<!-- /LANG -->

<!-- LANG:python -->
Python specific
<!-- /LANG -->
"#;

    let rendered = render_component(source, "rust").unwrap();
    assert!(rendered.contains("Agnostic content"));
    assert!(rendered.contains("Rust specific"));
    assert!(!rendered.contains("Python specific"));
}
```

### Test Coverage Requirements

- Greater than 80% line coverage
- All error paths tested
- All supported languages tested
- Edge cases covered
- Integration tests for full pipeline

## Migration Strategy

### Backward Compatibility

**Support both formats during transition:**

1. Detect format by checking for YAML frontmatter
2. If frontmatter present: use new system
3. If no frontmatter: use legacy system with warning
4. Log deprecation warnings for old format
5. Provide migration tool (optional)

### Migration Timeline

**Week 1-6**: Implement new system (this plan)
**Week 7-8**: Parallel support for old and new formats
**Week 9-10**: Migrate all default components to new format
**Week 11-12**: Deprecation warnings for old format
**Week 13+**: Remove old format support in next major version

### Migration Tool (Optional)

```bash
# Convert old component to new format
xzagentz migrate component path/to/component.md

# Convert all components in directory
xzagentz migrate components path/to/components/

# Validate component format
xzagentz validate component path/to/component.md
```

## Risk Assessment and Mitigation

### Risk 1: Parsing Complexity

**Risk**: Language section parsing may be complex and error-prone

**Mitigation**:
- Use well-tested regex patterns
- Comprehensive test coverage
- Clear error messages
- Fallback to raw content on parse errors

### Risk 2: Size Constraint Violations

**Risk**: Components may exceed size limits after refactoring

**Mitigation**:
- Iterative optimization approach
- Monitor sizes during development
- Provide detailed size reports
- Allow configuration overrides for power users

### Risk 3: Language Coverage Gaps

**Risk**: Not all sections may have content for all languages

**Mitigation**:
- LANG:* fallback for all sections
- Validation warnings for missing languages
- Prioritize common languages first
- Allow graceful degradation

### Risk 4: Backward Compatibility Issues

**Risk**: Breaking changes may affect existing users

**Mitigation**:
- Support both formats during transition
- Comprehensive migration guide
- Deprecation warnings
- Version component format

### Risk 5: Performance Degradation

**Risk**: Parsing and filtering may slow generation

**Mitigation**:
- Benchmark performance
- Optimize hot paths
- Cache parsed components
- Profile and optimize as needed

## Dependencies

### New Rust Crates

```toml
[dependencies]
# Existing
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
thiserror = "1.0"
tracing = "0.1"

# May need to add
regex = "1.10"  # For language section parsing
lazy_static = "1.4"  # For compiled regex patterns
```

### Version Compatibility

- Rust: 1.70 or later (for latest stable features)
- All dependencies: Use latest stable versions
- No breaking changes to public API

## Timeline and Effort Estimate

### Detailed Breakdown

| Phase | Duration | Effort (hours) | Deliverables |
|-------|----------|----------------|--------------|
| Phase 1: Foundation | 1 week | 30-40 | Parser, renderer, tests |
| Phase 2: Core Components | 1 week | 25-30 | 3 refactored components |
| Phase 3: General Components | 1 week | 30-35 | 3 refactored components |
| Phase 4: Tool Tiering | 1 week | 20-25 | Essential variants |
| Phase 5: Validation | 1 week | 25-30 | Metrics, optimization |
| Phase 6: Documentation | 1 week | 15-20 | Docs, examples, release |
| **Total** | **6 weeks** | **145-180 hours** | **Complete system** |

### Critical Path

1. Phase 1 (foundation) blocks all other phases
2. Phases 2-4 can partially overlap after Phase 1 complete
3. Phase 5 requires Phases 2-4 complete
4. Phase 6 can start during Phase 5

### Milestones

- **End of Week 1**: Parser and renderer working with tests
- **End of Week 2**: Core components refactored and validated
- **End of Week 3**: General components refactored and validated
- **End of Week 4**: Tool tiering implemented and tested
- **End of Week 5**: All size targets met and validated
- **End of Week 6**: Documentation complete, ready for release

## Quality Gates

### Before Each Phase Complete

- [ ] All tests pass: `cargo test --all-features`
- [ ] No warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code formatted: `cargo fmt --all`
- [ ] Documentation complete: All public APIs have doc comments
- [ ] Test coverage: Greater than 80% for new code
- [ ] Integration tests: Full pipeline tested

### Before Final Release

- [ ] All phases complete
- [ ] All quality gates passed
- [ ] Generated outputs validated for all languages
- [ ] Size targets met for all project types
- [ ] Documentation complete and accurate
- [ ] Examples working and tested
- [ ] Migration guide reviewed
- [ ] Release notes written
- [ ] No critical or high-priority bugs

## Appendix

### Example Component with Language Sections

```markdown
---
component:
  name: error_handling
  category: core
  version: 2.0.0
  languages:
    - rust
    - python
    - golang
    - typescript
    - bash
  sections:
    - id: error_handling_rules
      language_specific: true
      required: true
---

# Error Handling Standards

## Core Principles

<!-- LANG:* -->

**YOU MUST:**
- Handle ALL errors explicitly
- Never silently ignore errors
- Provide context when propagating errors
- Use descriptive error messages
- Follow language-specific best practices

<!-- /LANG -->

## Language-Specific Patterns

<!-- LANG:rust -->

**YOU MUST:**

- Use `Result<T, E>` for all fallible operations
- Use `?` operator for error propagation
- Use `thiserror` crate for custom error types
- Never use `unwrap()` without justification

**Example:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(String),

    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}

pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadError(e.to_string()))?;

    let config: Config = serde_yaml::from_str(&contents)
        .map_err(|e| ConfigError::ValidationError(e.to_string()))?;

    config.validate()?;
    Ok(config)
}
```

<!-- /LANG -->

<!-- LANG:python -->

**YOU MUST:**

- Raise appropriate exception types
- Use try/except for error handling
- Provide descriptive error messages with context
- Use exception chaining with `from` keyword

**Example:**

```python
class ConfigError(Exception):
    """Configuration loading error"""
    pass

def load_config(path: str) -> dict:
    """Load configuration from YAML file

    Args:
        path: Path to configuration file

    Returns:
        Parsed configuration dictionary

    Raises:
        ConfigError: If file cannot be read or parsed
    """
    try:
        with open(path, 'r') as f:
            config = yaml.safe_load(f)
    except IOError as e:
        raise ConfigError(f"Failed to read config: {e}") from e
    except yaml.YAMLError as e:
        raise ConfigError(f"Invalid YAML: {e}") from e

    validate_config(config)
    return config
```

<!-- /LANG -->

<!-- LANG:golang -->

**YOU MUST:**

- Return errors as the last return value
- Wrap errors with context using `fmt.Errorf` with `%w`
- Never ignore errors with blank identifier
- Check all error return values

**Example:**

```go
package config

import (
    "fmt"
    "os"
    "gopkg.in/yaml.v3"
)

type ConfigError struct {
    Path string
    Err  error
}

func (e *ConfigError) Error() string {
    return fmt.Sprintf("config error for %s: %v", e.Path, e.Err)
}

func LoadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, &ConfigError{Path: path, Err: err}
    }

    var config Config
    if err := yaml.Unmarshal(data, &config); err != nil {
        return nil, fmt.Errorf("failed to parse config: %w", err)
    }

    if err := config.Validate(); err != nil {
        return nil, fmt.Errorf("invalid config: %w", err)
    }

    return &config, nil
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

**YOU MUST:**

- Use try/catch for error handling
- Create custom Error classes for domain errors
- Include error context in messages
- Use type guards for error checking

**Example:**

```typescript
class ConfigError extends Error {
  constructor(message: string, public readonly path: string) {
    super(message);
    this.name = 'ConfigError';
  }
}

async function loadConfig(path: string): Promise<Config> {
  try {
    const contents = await fs.readFile(path, 'utf-8');
    const config = YAML.parse(contents) as Config;

    validateConfig(config);

    return config;
  } catch (error) {
    if (error instanceof Error) {
      throw new ConfigError(
        `Failed to load config: ${error.message}`,
        path
      );
    }
    throw error;
  }
}
```

<!-- /LANG -->

<!-- LANG:bash -->

**YOU MUST:**

- Use `set -e` to exit on error
- Check command exit codes explicitly when needed
- Provide descriptive error messages to stderr
- Use error handling functions for cleanup

**Example:**

```bash
#!/bin/bash
set -euo pipefail

error_exit() {
    echo "ERROR: $1" >&2
    exit 1
}

cleanup() {
    local exit_code=$?
    # Cleanup logic here
    exit $exit_code
}
trap cleanup EXIT

load_config() {
    local config_path="$1"

    if [[ ! -f "$config_path" ]]; then
        error_exit "Config file not found: $config_path"
    fi

    if ! config=$(cat "$config_path"); then
        error_exit "Failed to read config file: $config_path"
    fi

    echo "$config"
}
```

<!-- /LANG -->

## Validation

All error handling must be tested with both success and failure cases.

```

### Component Size Reference

**Target Sizes (after rendering for one language):**

| Component Type | Lines (rendered) | Lines (source with all langs) |
|----------------|------------------|-------------------------------|
| Core component | 100-150 | 400-500 |
| General component | 150-200 | 600-700 |
| Language component | 300-400 | 300-400 (single lang) |
| Tool essential | 100-150 | 400-500 |
| Tool comprehensive | 600-800 | 600-800 (mostly agnostic) |

### Language Support Matrix

| Feature | Rust | Python | Go | TypeScript | Bash |
|---------|------|--------|-----|------------|------|
| Error handling | Yes | Yes | Yes | Yes | Yes |
| Testing patterns | Yes | Yes | Yes | Yes | Yes |
| Documentation | Yes | Yes | Yes | Yes | Yes |
| Build commands | Yes | Yes | Yes | Yes | N/A |
| Type system | Yes | Yes | Yes | Yes | N/A |
| Async patterns | Yes | Yes | Yes | Yes | N/A |

## References

- Component Improvement Plan: `docs/explanation/component_improvement_plan.md`
- AGENTS.md Rules: `AGENTS.md`
- Current Component Structure: `components/`
- Diataxis Framework: https://diataxis.fr/

---

## Document Metadata

- **Version**: 1.0.0
- **Status**: Ready for Implementation
- **Last Updated**: 2024
- **Owner**: xzagentz Development Team
- **Related Documents**:
  - `docs/explanation/component_improvement_plan.md`
  - `docs/explanation/hybrid_component_system_implementation.md`
  - `AGENTS.md`

---

End of Implementation Plan
