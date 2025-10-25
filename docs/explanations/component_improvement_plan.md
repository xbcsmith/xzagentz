# Component Improvement Plan

## Overview

This document provides a comprehensive plan to improve the agentz component
system based on analysis of generated AGENTS.md examples. The current system has
significant quality issues where language-agnostic components contain
language-specific code (primarily Go), resulting in poor quality generated files
for non-Go projects.

## Current State Analysis

### Example Quality Assessment

#### Python CLI Example (python_cli_example.md)

**Status**: ❌ POOR QUALITY

**Issues Identified:**

1. **File size**: 3,700+ lines (should be ~500-800 for CLI tool)
2. **Go code pollution**: Contains extensive Go-specific content:
   - Go function signatures (`func Load(ctx context.Context, path string)`)
   - Go package patterns and documentation
   - Go context propagation patterns
   - Go error handling with `fmt.Errorf` and `%w` verb
   - Go-specific testing patterns with table-driven tests
3. **Mixed content**: Python sections exist but are buried in Go content
4. **Generic bloat**: Includes massive generic sections (Git workflow, Markdown
   standards) that aren't language-adapted

**What Works:**

- Python-specific sections are well-written when present
- Virtual environment setup is correct
- Black/ruff/mypy configuration is appropriate

#### Rust Library Example (rust_library_example.md)

**Status**: ✅ GOOD QUALITY

**Strengths:**

1. **Focused content**: ~500 lines, appropriately sized
2. **Rust-specific throughout**: All examples use Rust patterns
3. **Well-structured**: Clear sections for Rust development
4. **Appropriate examples**: Error types, builders, trait patterns

**Minor Issues:**

- Could use more Rust-specific testing patterns (proptest examples are good)

#### Bash Scripts Example (bash_scripts_example.md)

**Status**: ✅ GOOD QUALITY

**Strengths:**

1. **Concise**: Appropriately brief for shell scripts
2. **Bash-specific**: All examples relevant to bash
3. **Practical**: Error handling, argument parsing, shellcheck integration

#### TypeScript Web Example (typescript_web_example.md)

**Status**: ✅ GOOD QUALITY

**Strengths:**

1. **Focused**: TypeScript/React patterns throughout
2. **Type safety emphasis**: Proper TypeScript patterns
3. **Web-specific**: API calls, React components

#### Go Service Example (go_service_example.md)

**Status**: ✅ GOOD QUALITY

**Strengths:**

1. **Go-specific**: Appropriate Go patterns
2. **Microservice focus**: HTTP handlers, middleware
3. **Well-structured**: Proper Go project layout

### Root Cause Analysis

**Problem**: Core and General components contain Go-specific code examples that
appear in ALL generated AGENTS.md files regardless of target language.

**Affected Components:**

1. **components/core/critical_rules.md**
   - Contains Go error handling examples
   - Go context propagation patterns
   - Go-specific testing patterns
   - Go documentation format

2. **components/core/learning_resources.md**
   - Go package structure examples
   - Go-specific file references

3. **components/general/testing.md**
   - Go table-driven test examples
   - Go testing package patterns
   - Go-specific test structure

4. **components/general/development.md**
   - Generic but uses language-agnostic placeholders inconsistently
   - Example commands not adapted to language

5. **components/general/documentation.md**
   - Has language-specific sections but Go appears first
   - Examples for multiple languages but not consistently applied

## Improvement Strategy

### Strategy 1: Language-Agnostic Core Components

**Approach**: Completely remove language-specific examples from core components.
Use abstract descriptions and placeholders.

**Pros:**

- Truly universal content
- No language pollution
- Easier to maintain

**Cons:**

- Less actionable for AI agents
- Requires language components to be comprehensive
- May result in generic, less useful content

### Strategy 2: Template Variables for Code Examples

**Approach**: Use template variables that get replaced based on detected project
language.

**Pros:**

- Maintains concrete examples
- Language-appropriate for each project
- Core components can demonstrate concepts

**Cons:**

- Complex templating system needed
- Harder to maintain multiple example sets
- Template syntax in markdown may be confusing

### Strategy 3: Conditional Section Inclusion (RECOMMENDED)

**Approach**: Use YAML metadata to mark sections as language-specific and
include/exclude based on project language.

**CRITICAL**: Components STORE multiple language versions, but the template
engine RENDERS only the target language version. This keeps output files short
and concise.

**Pros:**

- Best of both worlds: concrete examples that are language-appropriate
- Clean separation of concerns
- Maintainable through YAML metadata
- Can have default/fallback examples
- **Output files only contain ONE language version (keeps them short)**

**Cons:**

- Requires implementation of conditional rendering
- More complex component structure

## Recommended Solution: Hybrid Approach

### Implementation Plan

#### Phase 1: Component Metadata Enhancement

**Add language-specific section markers to YAML:**

```yaml
component:
  name: critical_rules
  category: core
  version: 2.0.0
  sections:
    - id: quality_gates
      language_specific: true
      default_language: agnostic
    - id: error_handling
      language_specific: true
      default_language: agnostic
    - id: testing_requirements
      language_specific: true
      default_language: agnostic
```

**Section structure in markdown:**

````markdown
### Rule 2: Error Handling

<!-- BEGIN LANGUAGE_SECTION: error_handling -->
<!-- LANGUAGE: agnostic -->

**YOU MUST:**

- Handle ALL errors explicitly
- Provide actionable error messages
- Use language-appropriate error handling patterns

<!-- END LANGUAGE_SECTION -->

<!-- BEGIN LANGUAGE_SECTION: error_handling -->
<!-- LANGUAGE: golang -->

**YOU MUST:**

- Return errors as the last return value
- Wrap errors with context using `fmt.Errorf` with `%w`
- Never ignore errors with `_`

```go
// Example Go error handling
data, err := os.ReadFile(path)
if err != nil {
    return fmt.Errorf("failed to read config: %w", err)
}
```
````

<!-- END LANGUAGE_SECTION -->

<!-- BEGIN LANGUAGE_SECTION: error_handling -->
<!-- LANGUAGE: python -->

**YOU MUST:**

- Raise appropriate exception types
- Use try/except for error handling
- Provide descriptive error messages

```python
# Example Python error handling
try:
    with open(path, 'r') as f:
        data = f.read()
except IOError as e:
    raise ConfigError(f"Failed to read config: {e}") from e
```

<!-- END LANGUAGE_SECTION -->

````

#### Phase 2: Component Refactoring

**Priority Order:**

1. **critical_rules.md** (HIGHEST PRIORITY)
2. **testing.md** (HIGH PRIORITY)
3. **learning_resources.md** (MEDIUM PRIORITY)
4. **development.md** (MEDIUM PRIORITY)
5. **documentation.md** (LOW PRIORITY - already has multi-language examples)

**For Each Component:**

1. Identify all language-specific code examples
2. Create language-agnostic version
3. Create language-specific versions for:
   - Python
   - Go
   - Rust
   - TypeScript/JavaScript
   - Bash
4. Add conditional section markers
5. Update YAML metadata

#### Phase 3: Template Engine Updates

**Required Changes:**

1. Parse language-specific section markers
2. Detect project language (from config or auto-detect)
3. Include appropriate language sections:
   - If exact match exists: use language-specific section
   - If no match: use agnostic section
   - If agnostic missing: use default language (Go) but warn
4. **REMOVE ALL UNUSED LANGUAGE SECTIONS FROM OUTPUT** ← CRITICAL FOR SHORT FILES

**Key Principle**: The component markdown files contain ALL language versions,
but the generated AGENTS.md contains ONLY the target language version.

**Example:**
- `critical_rules.md` source: ~500 lines (with Python, Go, Rust, TS, Bash sections)
- Generated Python AGENTS.md: ~100 lines (only Python sections included)
- Generated Go AGENTS.md: ~100 lines (only Go sections included)

**Parser Logic:**

```go
type LanguageSection struct {
    ID       string
    Language string
    Content  string
}

func RenderComponent(component Component, targetLanguage string) string {
    sections := parseLanguageSections(component.Content)
    var renderedContent strings.Builder

    for _, section := range sections {
        appropriate := selectAppropriateSection(
            section.ID,
            targetLanguage,
            sections,
        )

        // ONLY include the selected language version
        // All other language versions are discarded
        renderedContent.WriteString(appropriate.Content)
    }

    return renderedContent.String()
}

func selectAppropriateSection(
    sectionID string,
    targetLang string,
    allSections []LanguageSection,
) LanguageSection {
    // 1. Try exact language match (e.g., "python")
    // 2. Fall back to agnostic version
    // 3. Fall back to default (with warning)

    // IMPORTANT: Returns ONLY ONE section per section ID
    // This ensures output contains no unused language examples
}
````

**File Size Impact Example:**

```
Component Source (critical_rules.md):
├── Agnostic sections: ~50 lines
├── Python sections: ~100 lines
├── Go sections: ~100 lines
├── Rust sections: ~100 lines
├── TypeScript sections: ~100 lines
└── Bash sections: ~100 lines
Total: ~550 lines

Generated Output (Python project):
├── Agnostic sections: ~50 lines
└── Python sections: ~100 lines
Total: ~150 lines ✓ (NOT 550 lines!)

Generated Output (Go project):
├── Agnostic sections: ~50 lines
└── Go sections: ~100 lines
Total: ~150 lines ✓
```

## Detailed Component Improvements

### 1. critical_rules.md Improvements

**Current Issues:**

- All examples are Go-specific
- Error handling section is Go-only
- Testing examples use Go patterns
- Documentation examples use Go doc comments

**Required Changes:**

#### Error Handling Section

**Agnostic Version:**

```markdown
### Rule 2: Error Handling

**YOU MUST:**

- Handle ALL errors explicitly
- Never silently ignore errors
- Provide context when propagating errors
- Use descriptive error messages
- Follow language-specific error handling best practices
```

**Language-Specific Examples:**

- **Go**: `fmt.Errorf` with `%w`, error returns
- **Python**: Exception types, try/except, raise from
- **Rust**: Result<T, E>, ?, thiserror
- **TypeScript**: try/catch, Error types, throw
- **Bash**: set -e, error_exit functions, return codes

#### Testing Requirements Section

**Agnostic Version:**

```markdown
### Rule 3: Testing Requirements

**YOU MUST:**

- Write tests for ALL public/exported APIs
- Test both success and failure cases
- Test edge cases and boundary conditions
- Achieve minimum 80% code coverage
- Use descriptive test names
```

**Language-Specific Examples:**

- **Go**: table-driven tests, Test{Function}\_{Scenario}
- **Python**: pytest fixtures, test*{function}*{scenario}
- **Rust**: #[test], #[should_panic], proptest
- **TypeScript**: describe/it, jest, vitest
- **Bash**: bats framework, @test

#### Documentation Section

**Agnostic Version:**

```markdown
### Rule 4: Documentation

**YOU MUST:**

- Document all public APIs
- Explain the "why" not just the "what"
- Provide usage examples
- Keep documentation close to code
- Update docs when changing behavior
```

**Language-Specific Examples:**

- **Go**: Package/function doc comments, godoc format
- **Python**: Docstrings (Google/NumPy style), type hints
- **Rust**: /// doc comments, //! module docs, cargo doc
- **TypeScript**: JSDoc/TSDoc comments, @param/@returns
- **Bash**: Function header comments, usage functions

### 2. testing.md Improvements

**Current Issues:**

- Heavy Go bias in examples
- Table-driven tests are Go-specific pattern
- Mock patterns use Go interfaces

**Required Changes:**

#### Test Structure Section

**Agnostic Version:**

```markdown
### Test Structure

**Arrange-Act-Assert (AAA) Pattern:**

1. **Arrange** - Set up test data and dependencies
2. **Act** - Execute the code under test
3. **Assert** - Verify the outcome

Use this pattern consistently across all tests for clarity and maintainability.
```

**Language-Specific Examples:**

Each language gets appropriate test framework examples showing AAA pattern.

#### Multiple Test Cases

**Current**: Only shows Go table-driven tests

**Improved**: Show each language's preferred pattern:

- **Go**: Table-driven tests with structs
- **Python**: pytest parametrize decorator
- **Rust**: Test modules with multiple #[test] functions
- **TypeScript**: describe blocks with multiple it() calls
- **Bash**: Multiple @test blocks in bats

### 3. learning_resources.md Improvements

**Current Issues:**

- Go package references hardcoded
- Go-specific file paths

**Required Changes:**

Replace specific paths with placeholders:

```markdown
### Common Patterns

The codebase follows established patterns. Look for these examples:

1. **Error Handling Pattern**
   - See: {{error-example-path}}
   - Pattern: {{error-pattern-description}}

2. **Testing Pattern**
   - See: {{test-example-path}}
   - Pattern: {{test-pattern-description}}
```

Populate placeholders based on detected language and project structure.

### 4. development.md Improvements

**Current Issues:**

- Commands are generic "make" but could be language-specific
- Setup steps could be more targeted

**Required Changes:**

Use conditional sections for:

- Dependency installation
- Code formatting commands
- Linting commands
- Test execution
- Build commands

### 5. documentation.md Improvements

**Current State**: Already has multi-language examples (Go, Python, TypeScript)

**Minor Improvements Needed:**

- Add Rust examples
- Add Bash examples
- Ensure consistent coverage across languages
- Make language selection clear in rendering

## Template System Implementation

### Conditional Rendering Syntax

**Option A: HTML-style comments (RECOMMENDED)**

````markdown
<!-- LANG:python -->

```python
# Python code
```
````

<!-- /LANG -->

<!-- LANG:golang -->

```go
// Go code
```

<!-- /LANG -->

<!-- LANG:* -->

Generic content for all languages

<!-- /LANG -->

````

**Pros:**

- Standard markdown comments
- Won't appear in rendered output if not processed
- Clear visual separation

**Option B: YAML frontmatter sections**

```yaml
---
sections:
  error_handling:
    agnostic: |
      # Content here
    python: |
      # Python-specific content
    golang: |
      # Go-specific content
---
````

**Pros:**

- Structured data
- Easy to parse
- Clear separation

**Cons:**

- Moves content out of main markdown
- Harder to maintain
- Less readable

### Rendering Algorithm

```
1. Parse component markdown file
2. Extract all language-specific sections
3. Determine target language from:
   - Explicit config parameter
   - Auto-detection from project files
   - User selection
4. For each language section:
   a. Check if target language version exists
   b. If yes: use target language version
   c. If no: check for agnostic version
   d. If no agnostic: use default language with warning
5. Assemble final markdown with selected sections
6. Render to output
```

## Implementation Phases

### Phase 1: Foundation (Week 1)

**Tasks:**

1. Design conditional rendering syntax
2. Implement markdown parser for language sections
3. Add language detection logic
4. Create component rendering engine
5. Write tests for rendering logic

**Deliverables:**

- Updated template engine with language support
- Test suite for conditional rendering
- Documentation of new syntax

### Phase 2: Core Component Refactoring (Week 2)

**Tasks:**

1. Refactor critical_rules.md
   - Create agnostic versions of all rules
   - Add Python-specific examples
   - Add Rust-specific examples
   - Add TypeScript-specific examples
   - Add Bash-specific examples
2. Update YAML metadata
3. Test generation for each language
4. Compare output quality

**Deliverables:**

- Refactored critical_rules.md
- Generated examples for all languages
- Quality comparison document

### Phase 3: General Component Refactoring (Week 3)

**Tasks:**

1. Refactor testing.md
2. Refactor development.md
3. Refactor learning_resources.md
4. Update documentation.md (minor changes)

**Deliverables:**

- All general components refactored
- Complete test coverage
- Updated documentation

### Phase 4: Validation and Optimization (Week 4)

**Tasks:**

1. Generate AGENTS.md for all example projects
2. Validate quality against requirements
3. Collect metrics (file size, relevance, correctness)
4. Optimize component content
5. Update component documentation

**Deliverables:**

- Quality validation report
- Optimized components
- Updated component authoring guide

## Success Metrics

### Quantitative Metrics

1. **File Size Consistency**
   - Python example: 500-800 lines (currently 3,700)
   - All examples within 20% of target size
   - No examples > 1,000 lines for simple projects
   - **Generated files contain ONLY target language sections (not all languages)**

2. **Language Purity**
   - 0% code examples in wrong language
   - 100% of code blocks use target language
   - 0% language-specific patterns for other languages
   - **Each generated AGENTS.md contains examples for ONE language only**

3. **Coverage**
   - All 5 supported languages have examples for each rule
   - All components have language-specific versions
   - Agnostic fallback exists for all sections

### Qualitative Metrics

1. **Relevance**
   - AI agents can follow examples directly
   - No confusion from mixed language patterns
   - Examples match project type and language

2. **Completeness**
   - All critical rules have actionable examples
   - Language-specific patterns are appropriate
   - No gaps in coverage

3. **Maintainability**
   - Clear separation of language-specific content
   - Easy to add new languages
   - Consistent structure across components

## Migration Path

### Backward Compatibility

**Concern**: Existing projects may depend on current component structure.

**Solution**:

1. Implement version field in component YAML
2. Support both old and new formats
3. Deprecation warning for old format
4. Migration guide for component authors
5. Automatic migration tool (optional)

### Testing Strategy

**For Each Refactored Component:**

1. Generate AGENTS.md with old version
2. Generate AGENTS.md with new version
3. Compare outputs for:
   - Content completeness
   - Language appropriateness
   - File size
   - Structure
4. Manual review by language experts
5. AI agent testing (can they follow the instructions?)

## Component Authoring Guidelines

### New Component Standards

When creating new components:

1. **Default to Agnostic**
   - Start with language-agnostic content
   - Only add language-specific examples where necessary

2. **Use Conditional Sections**
   - Mark language-specific sections clearly
   - Always provide agnostic fallback
   - Support all 5 core languages (Python, Go, Rust, TypeScript, Bash)

3. **Example Quality**
   - Realistic, runnable code
   - Appropriate to language idioms
   - Consistent with language component

4. **Avoid Language Assumptions**
   - Don't assume build tools (make vs npm vs cargo)
   - Use placeholders for commands
   - Let language component fill in details

### Review Checklist

Before submitting new/updated components:

- [ ] All code examples use correct language
- [ ] Agnostic version exists for all language-specific sections
- [ ] All 5 languages supported where code examples needed
- [ ] YAML metadata updated with section markers
- [ ] Generated output tested for all languages
- [ ] **File size appropriate (500-800 lines typical) for GENERATED output**
- [ ] **Component source may be larger (contains all languages)**
- [ ] No language-specific patterns outside language sections
- [ ] **Verified that unused language sections are NOT in generated output**
- [ ] **Each generated file contains ONLY ONE language's examples**

## Future Enhancements

### Additional Languages

**Candidates:**

- Java/Kotlin
- C/C++
- Ruby
- Swift
- C#/.NET

**Process**:

1. Create language component
2. Add language-specific sections to core/general components
3. Test generation
4. Validate quality

### AI-Powered Language Detection

**Enhancement**: Use AI to analyze project and suggest appropriate sections.

**Benefits:**

- Better project type detection
- Suggest relevant optional components
- Customize examples to project specifics

### Dynamic Example Generation

**Enhancement**: Generate code examples on-the-fly based on project code style.

**Benefits:**

- Examples match existing codebase style
- Consistent naming conventions
- Project-specific patterns

## Conclusion

The current component system has a critical flaw: Go-specific content pollutes
all generated files regardless of target language. This improvement plan
provides a comprehensive solution through conditional section inclusion with
language-specific examples.

**Key Changes:**

1. Refactor core/general components to be language-agnostic with conditional
   language-specific sections
2. Implement conditional rendering in template engine
3. Support Python, Go, Rust, TypeScript, and Bash equally
4. Establish quality metrics and validation process
5. Create maintainable component authoring guidelines

**Expected Outcomes:**

- Python example reduces from 3,700 to ~600 lines
- Zero cross-language pollution
- Higher quality, more actionable AI agent instructions
- Easier to add new languages
- Better developer experience

## Cumulative File Size Considerations

### Critical Question Addressed

**Q**: Does the plan account for the cumulative size when including ALL core components, ALL general components, multiple tools, AND a language component?

**A**: YES, but requires a multi-faceted approach beyond just language filtering.

### Current Component Sizes (All Sources)

**Core Components**: 472 lines total

- critical_rules.md: 223 lines
- learning_resources.md: 196 lines
- header.md: 53 lines

**General Components**: 1,974 lines total

- testing.md: 568 lines
- development.md: 624 lines
- documentation.md: 782 lines

**Tool Components**: 1,726+ lines

- git.md: 405 lines
- markdown.md: 688 lines
- npm.md: 633 lines

**Language Components**: ~300-400 lines each

**TOTAL SOURCE**: ~4,500-5,000 lines across all components

### Projected Output After Language Filtering Only

**Realistic Calculation for Python CLI**:

- Core components: ~230 lines (after filtering)
- General components: ~470 lines (after filtering)
- Python language: ~300 lines
- Git + Markdown tools: ~1,030 lines (mostly agnostic, minimal filtering)
- **TOTAL**: ~2,030 lines

**Problem**: Still too large! Target should be ~1,000-1,200 lines.

### Root Cause: Multiple Issues

1. **Language filtering alone is insufficient** - Tool components are large but mostly language-agnostic
2. **General components are verbose** - Include too much generic project management advice
3. **No component size discipline** - No limits on individual component contributions

### Enhanced Strategy: Three-Phaseed Approach

#### Phase 1: Language Filtering (Already Planned)

- Remove Go examples from Python projects
- Remove Python examples from Go projects
- **Reduction**: 30-40% in core/general components

#### Phase 2: Conciseness Pass on ALL Components

**Apply these principles to every component**:

- Focus on AI agent instructions, not human tutorials
- One example per concept (not multiple variations)
- Remove philosophical discussions about best practices
- Keep only actionable, project-specific guidance

**Target output sizes per component**:

- critical_rules.md: 100 lines (currently ~140 after filtering)
- testing.md: 150 lines (currently ~230 after filtering)
- development.md: 200 lines (currently ~440 after filtering)
- documentation.md: 120 lines (currently ~200 after filtering)
- learning_resources.md: 80 lines (currently ~140 after filtering)

#### Phase 3: Component Tiering System

**Create essential vs comprehensive variants**:

**Git Component**:

- git_essential.md: 120 lines (commit format, branch naming, critical rules only)
- git_comprehensive.md: 405 lines (full current content)
- Default: essential

**Markdown Component**:

- markdown_essential.md: 100 lines (file naming, basic formatting)
- markdown_comprehensive.md: 688 lines (full current content)
- Default: essential (only if project has docs)

**Documentation Component**:

- documentation_essential.md: 120 lines (doc requirements only)
- documentation_comprehensive.md: 782 lines (full guide)
- Default: essential

### Revised File Size Projections

**Simple CLI Project** (Python example):

```
Core components (filtered + concise):     230 lines
General components (filtered + concise):  470 lines
Python language component:                300 lines
git_essential:                           120 lines
─────────────────────────────────────────────────
TOTAL:                                 1,120 lines ✓

Current: 3,700 lines
Reduction: 70% ✓
```

**Library Project**:

```
Core components:                          230 lines
General components:                       470 lines
Language component:                       300 lines
git_essential:                           120 lines
documentation_essential:                 120 lines
─────────────────────────────────────────────────
TOTAL:                                 1,240 lines ✓
```

**Documentation-Heavy Project**:

```
Core components:                          230 lines
General components:                       470 lines
Language component:                       300 lines
git_comprehensive:                       405 lines
markdown_comprehensive:                  688 lines
documentation_comprehensive:             782 lines
─────────────────────────────────────────────────
TOTAL:                                 2,875 lines

Still better than current worst case (~4,500 lines)
```

### Component Contribution Limits

**Enforce these limits during generation**:

| Component Category           | Max Contribution to Output |
| ---------------------------- | -------------------------- |
| Each core component          | 100 lines                  |
| Each general component       | 200 lines                  |
| Language component           | 400 lines                  |
| Essential tool component     | 150 lines                  |
| Comprehensive tool component | 800 lines                  |
| **Total project limit**      | **1,500 lines**            |

### Configuration-Driven Approach

**agentz.toml example**:

```toml
[project]
type = "cli"
language = "python"

[components.tools]
git = "essential"           # Use git_essential.md
markdown = { variant = "essential", optional = true }

[limits]
max_total_lines = 1200      # Hard limit
warn_at_lines = 1000        # Warning threshold
```

**Rendering logic enforces limits**:

```go
func GenerateAGENTSMD(config Config) (string, error) {
    totalLines := 0

    // Render each component
    for _, comp := range components {
        content := renderComponentFiltered(comp, config.Language)

        // Check contribution limit
        lines := countLines(content)
        if lines > comp.MaxContribution {
            return "", fmt.Errorf(
                "component %s exceeds limit: %d > %d",
                comp.Name, lines, comp.MaxContribution,
            )
        }

        totalLines += lines
    }

    // Check total limit
    if totalLines > config.Limits.MaxTotalLines {
        return "", fmt.Errorf("total exceeds limit: %d", totalLines)
    }

    return output, nil
}
```

### Updated Success Metrics

**File Size Targets**:
| Project Type | Target | Max | Current Python |
|--------------|--------|-----|----------------|
| Simple CLI | 1,000 lines | 1,200 lines | 3,700 lines ❌ |
| Library | 1,100 lines | 1,300 lines | N/A |
| Web Service | 1,200 lines | 1,400 lines | N/A |
| Docs-heavy | 1,400 lines | 2,000 lines | N/A |

**All projects achieve 60-75% size reduction from current state.**

### Updated Implementation Phases

**Phase 1** (Week 1): Language filtering

- Implement conditional rendering
- **Expected**: 30-40% reduction in core/general

**Phase 2** (Week 2): Conciseness refactoring

- Refactor all components for conciseness
- Remove verbose explanations, keep actionable guidance
- **Expected**: Additional 30-40% reduction in all components

**Phase 3** (Week 3): Component tiering

- Create essential variants of tool components
- Implement smart defaults by project type
- **Expected**: Appropriate component selection by project needs

**Phase 4** (Week 4): Validation and limits

- Enforce component contribution limits
- Validate total file sizes for all project types
- Adjust component content to meet targets

## Next Steps

1. **Immediate**: Review and approve enhanced three-Phased strategy
2. **Week 1**: Implement conditional rendering engine
3. **Week 2**: Refactor all components for conciseness (not just language filtering)
4. **Week 3**: Create essential/comprehensive variants of tool components
5. **Week 4**: Validate total sizes, enforce limits, document

**Owner**: AI Agent Development Team **Timeline**: 4 weeks **Priority**: High
**Impact**: Critical quality improvement

**Key Change**: Plan now addresses cumulative file size through language filtering + conciseness + component tiering, ensuring generated files stay under 1,500 lines even with multiple components.
