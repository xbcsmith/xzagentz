# Component System Design

## Overview

This document explains the design philosophy, architecture, and implementation decisions behind xzagentz's component system. Understanding these concepts helps you create effective components and leverage the system's full capabilities.

## Design Philosophy

### Why Language-Agnostic Components?

Traditional development guidelines are tightly coupled to specific languages or frameworks, requiring separate documentation for each technology stack. This creates:

- **Duplication**: Same concepts repeated across language-specific guides
- **Maintenance burden**: Updates must be synchronized across multiple documents
- **Inconsistency**: Guidelines drift between language implementations
- **Discovery problems**: Hard to find relevant guidance

xzagentz solves this with language-agnostic components that:

- **Write once, use many**: Universal guidance with language-specific sections
- **Automatic extraction**: Render only relevant language content
- **Consistent principles**: Core concepts remain consistent across languages
- **Composable**: Mix and match components for specific needs

### Core Design Principles

**1. Single Responsibility**

Each component focuses on one specific topic:
- Error handling (not error handling + logging + monitoring)
- Testing standards (not testing + CI/CD + deployment)
- Git conventions (not Git + version control + branching + releases)

This ensures components remain focused, maintainable, and reusable.

**2. Size Constraints**

Components enforce size limits based on category:

| Category  | Limit | Rationale |
|-----------|-------|-----------|
| Core      | 200   | Essential practices need depth |
| General   | 150   | Common topics stay concise |
| Languages | 300   | Language specifics need examples |
| Tools     | 100-200 | Tools vary by complexity tier |

Size limits prevent:
- Information overload
- Component sprawl
- Maintenance nightmares
- Analysis paralysis

If a component exceeds its limit, it signals the need to split into focused units.

**3. Language Fallback**

Components provide graceful degradation:

```
Request: Rust-specific error handling
Available: Universal error handling + Rust section
Result: Universal + Rust content

Request: Rust-specific error handling
Available: Universal error handling only
Result: Universal content (fallback)

Request: Rust-specific error handling
Available: Python section only
Result: Universal content (no cross-language fallback)
```

This ensures users always get useful guidance, even when language-specific content is unavailable.

**4. Composition Over Inheritance**

Components compose into larger documents rather than inheriting from templates:

```
AGENTS.md =
  core/error_handling +
  core/testing_standards +
  development/git_conventions +
  languages/rust_standards +
  tools/docker_essentials
```

Benefits:
- Flexible document structure
- Reusable across projects
- Mix different granularities
- Easy to update individual components

## Architecture

### Component Structure

```
Component
├── Frontmatter (YAML)
│   ├── Metadata (name, category, version)
│   ├── Configuration (languages, tier, description)
│   └── Validation rules
├── Universal Content (Markdown)
│   ├── Applies to all languages
│   ├── Core concepts
│   └── General guidance
└── Language Sections (Markdown with markers)
    ├── RUST_START/RUST_END
    ├── PYTHON_START/PYTHON_END
    ├── GO_START/GO_END
    └── etc.
```

### Processing Pipeline

```
1. Load Component
   ↓
2. Parse Frontmatter (YAML)
   ↓
3. Validate Metadata
   ↓
4. Extract Content
   ↓
5. Identify Language Markers
   ↓
6. Filter by Target Language
   ↓
7. Render Final Content
   ↓
8. Validate Size Constraints
   ↓
9. Output
```

### Category System

**Core Components** (`core/`)
- Purpose: Essential development practices
- Examples: error_handling, testing_standards, security_fundamentals
- Size limit: 200 lines
- Rationale: Core concepts need comprehensive coverage

**General Components** (`general/`)
- Purpose: Common development topics
- Examples: code_review, documentation_standards, performance_tips
- Size limit: 150 lines
- Rationale: General topics stay concise and focused

**Language Components** (`languages/`)
- Purpose: Language-specific best practices
- Examples: rust_ownership, python_async, go_concurrency
- Size limit: 300 lines
- Rationale: Language specifics need code examples and idioms

**Tool Components** (`tools/`)
- Purpose: Tool usage and configuration
- Examples: docker, kubernetes, ci_cd_pipelines
- Size limit: 100 (essential) or 200 (comprehensive)
- Rationale: Tools have varying complexity levels

### Tier System

Applied only to `tools` category:

**Essential Tier**
- 100 line limit
- Basic usage patterns
- Common configurations
- Quick reference
- Beginner to intermediate users

**Comprehensive Tier**
- 200 line limit
- Advanced features
- Complex configurations
- Troubleshooting
- Advanced users

Rationale: Tools vary widely in complexity. Docker basics fit in 100 lines, but advanced Docker patterns need 200 lines. Tiers provide flexibility without arbitrary limits.

## Implementation Details

### Language Marker System

**Design Decision**: HTML comments for markers

```markdown
<!-- RUST_START -->
Rust-specific content
<!-- RUST_END -->
```

**Why HTML comments?**
- Valid Markdown (invisible in rendered output)
- Easy to parse with regex
- Clear visual distinction
- Standard syntax across editors
- No custom syntax to learn

**Alternative considered**: Custom markers like `@RUST_START@`
- Rejected: Not valid Markdown, visible in renders, confusing to users

### Size Validation

**Design Decision**: Line-based counting

Components counted by lines, excluding frontmatter:

```rust
let lines: Vec<&str> = content.lines().collect();
let size = lines.len();

if size > category_limit {
    return Err(ValidationError::SizeExceeded { size, limit: category_limit });
}
```

**Why lines, not characters or words?**
- Language agnostic (works for all languages)
- Editor-friendly (all editors show line counts)
- Predictable (1 line = 1 count, simple)
- Industry standard (most linters use line counts)

**Alternative considered**: Character count
- Rejected: Varies by language verbosity, hard to estimate

**Alternative considered**: Word count
- Rejected: Code blocks inflate counts unpredictably

### Frontmatter Validation

**Design Decision**: YAML for metadata

```yaml
---
component:
  name: "error_handling"
  category: "core"
  version: "1.0.0"
---
```

**Why YAML?**
- Human readable and writable
- Standard for Markdown frontmatter
- Rich data types (arrays, nested objects)
- Good tooling support
- Industry standard (Jekyll, Hugo, etc.)

**Alternative considered**: TOML
- Rejected: Less common in Markdown ecosystem

**Alternative considered**: JSON
- Rejected: Not human-friendly (trailing commas, quotes everywhere)

### Embedded Resources

**Design Decision**: Compile-time embedding with `include_str!`

```rust
const CORE_ERROR_HANDLING: &str = include_str!("../components/core/error_handling.md");
```

**Why compile-time embedding?**
- Zero-dependency distribution (single binary)
- No runtime file I/O for defaults
- Guaranteed availability
- Version-locked resources
- Fast access (no disk reads)

**Override mechanism**:
1. Check custom directory (highest priority)
2. Check environment variable path
3. Check XDG directories
4. Check home directory
5. Use embedded resource (fallback)

This ensures flexibility while maintaining zero-config operation.

### Component Resolution

**Design Decision**: Filesystem-based with caching

```rust
pub struct ComponentLoader {
    custom_dir: Option<PathBuf>,
    cache: RefCell<HashMap<String, Component>>,
}
```

**Why filesystem-based?**
- Natural organization (directories = categories)
- Easy to browse and edit
- Version control friendly
- Standard tooling works (grep, find, diff)
- No database overhead

**Why caching?**
- Components rarely change during execution
- Parsing YAML has overhead
- Multiple renders of same component
- Performance optimization

**Cache invalidation**: Manual `clear_cache()` method when needed

## Design Trade-offs

### Chosen: Line Limits vs Content Quality

**Trade-off**: Strict size limits may force compromise on content quality

**Decision**: Enforce limits strictly

**Rationale**:
- Forces focused, high-quality content
- Prevents documentation bloat
- Encourages splitting into logical units
- Improves discoverability
- Easier maintenance

**Mitigation**: Multiple components and cross-referencing

### Chosen: Markers vs AST Parsing

**Trade-off**: HTML comment markers are simple but less powerful than AST parsing

**Decision**: Use markers

**Rationale**:
- Simplicity trumps power for this use case
- Easy to write by hand
- Clear visual feedback
- Works with any Markdown processor
- No complex parser maintenance

**Limitation**: Cannot nest language sections (acceptable trade-off)

### Chosen: Embedded vs External Resources

**Trade-off**: Embedded resources increase binary size vs external files more flexible

**Decision**: Embed with override mechanism

**Rationale**:
- Zero-config user experience
- Single binary distribution
- Guaranteed defaults
- Override when needed
- Binary size acceptable (components are text)

**Measurement**: ~200KB for all embedded components and templates (negligible)

## Extension Points

### Custom Categories

Currently hardcoded to: core, general, languages, tools

**Future extension**: Plugin system for custom categories

```rust
pub trait CategoryPlugin {
    fn name(&self) -> &str;
    fn size_limit(&self) -> usize;
    fn validate(&self, component: &Component) -> Result<(), ValidationError>;
}
```

### Custom Validators

Current validation is built-in

**Future extension**: Custom validation rules

```rust
pub trait ComponentValidator {
    fn validate(&self, component: &Component) -> Result<(), ValidationError>;
}

// Usage
loader.add_validator(Box::new(NoEmojiValidator));
loader.add_validator(Box::new(StyleGuideValidator));
```

### Custom Renderers

Current renderer is Markdown-focused

**Future extension**: Multiple output formats

```rust
pub trait Renderer {
    fn render(&self, component: &Component) -> Result<String, RenderError>;
}

// Implementations
HTMLRenderer, PDFRenderer, ManPageRenderer
```

## Lessons Learned

### Simplicity Wins

Early designs included complex inheritance hierarchies and template systems. Current design uses simple composition and proved easier to understand, maintain, and extend.

### Size Limits Are Essential

Without size limits, components grew unwieldy. Strict enforcement improved quality dramatically by forcing authors to focus on essentials.

### Embedded Resources Matter

Embedding resources makes xzagentz approachable. Users can start immediately without setup, then customize when needed. This "zero-to-productive" path is critical for adoption.

### Language Markers Work

Simple HTML comment markers proved sufficient. No need for complex AST parsing or custom syntax. Simple tools are maintainable tools.

## Future Directions

### Potential Enhancements

1. **Component Dependencies**: Declare that components require other components
2. **Version Constraints**: Specify compatible component versions
3. **Conditional Sections**: Include/exclude based on project characteristics
4. **Component Templates**: Scaffolding for creating new components
5. **Visual Editor**: GUI for component authoring and validation

### Backward Compatibility

Design commits to:
- Frontmatter schema stability (additions only, no breaking changes)
- Marker syntax stability (HTML comments forever)
- Category names stability (new categories OK, renames never)
- File format stability (Markdown + YAML frontmatter)

This ensures components written today work with future xzagentz versions.

## Summary

xzagentz component system design prioritizes:
- **Simplicity**: Easy to understand and use
- **Composability**: Mix and match components freely
- **Maintainability**: Small, focused units
- **Flexibility**: Language-agnostic with language-specific sections
- **Usability**: Zero-config with customization options

These design decisions create a robust, extensible system for managing development guidelines across projects and languages.

## Related Documentation

- Creating Components Tutorial: `docs/tutorials/creating_custom_component.md`
- Component Format Reference: `docs/reference/component_format.md`
- Tier System Explanation: `docs/explanation/tier_system.md`
- Architecture Overview: `docs/explanation/architecture.md`
