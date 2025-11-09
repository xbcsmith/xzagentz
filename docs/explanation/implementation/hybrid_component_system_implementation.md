# Hybrid Component System Implementation Plan

## Overview

This document outlines the implementation plan for adding a hybrid component
distribution system to xzagentz. The system will support embedded default
components (using rust-embed), local filesystem overrides, and a cascading loader
that checks local components first before falling back to embedded defaults.

This enhancement directly supports the workflow: New Project → Architecture →
Implementation Plan → AGENTS.md → AI Execution, by allowing users to customize
templates while maintaining sensible defaults.

## Goals

1. **Embed default components** in the binary for zero-configuration usage
2. **Support local overrides** in `~/.config/xzagentz/components/` for
   customization
3. **Implement cascading loader** that checks local first, then embedded
4. **Maintain backward compatibility** with existing component loading
5. **Enable user workflow** for custom architecture and implementation plan
   templates

## Non-Goals

1. Remote component loading (deferred to future enhancement)
2. Component versioning/dependency management (out of scope)
3. Component marketplace or registry (not needed for MVP)

## Architecture

### Current State

```text
src/domain/components/
├── mod.rs             # Component types and traits
├── metadata.rs        # Metadata structures
└── validator.rs       # Validation logic

src/infrastructure/component_loaders/
├── mod.rs
└── file_loader.rs     # FileLoader - reads from filesystem

components/            # Currently loose files
├── core/
├── languages/
├── tools/
└── general/
```

### Target State

```text
src/domain/components/
├── mod.rs             # Component trait and types
├── metadata.rs        # Metadata structures
├── validator.rs       # Validation logic
└── loader.rs          # NEW: Loader trait definition

src/infrastructure/component_loaders/
├── mod.rs
├── file_loader.rs     # FileLoader - reads from filesystem
├── embedded_loader.rs # NEW: EmbeddedLoader - reads from rust-embed
└── cascade_loader.rs  # NEW: CascadingLoader - tries local then embedded

components/
├── defaults/          # NEW: Embedded in binary via rust-embed
│   ├── core/
│   ├── languages/
│   ├── tools/
│   └── general/
└── README.md

~/.config/xzagentz/    # User's local override location
└── components/
    ├── core/
    ├── languages/
    ├── tools/
    ├── general/
    └── custom/        # User-specific categories
```

### Component Resolution Flow

```text
1. User requests component "architecture"
2. CascadingLoader.load("core/architecture.md")
3. Try FileLoader.load("~/.config/xzagentz/components/core/architecture.md")
   ├─ Found? Return local component
   └─ Not found? Continue to step 4
4. Try EmbeddedLoader.load("defaults/core/architecture.md")
   ├─ Found? Return embedded component
   └─ Not found? Return error
```

## Implementation Phases

### Phase 1: Embedded Components (Priority: HIGH)

**Goal**: Embed default components in binary using rust-embed

**Tasks**:

1. **Reorganize component directory structure**

   - Create `components/defaults/` directory
   - Move existing components from `components/*` to `components/defaults/*`
   - Update `components/README.md` to explain new structure
   - Verify all existing components are moved

2. **Add rust-embed dependency**

   - Add `rust-embed = "8.0"` to `Cargo.toml`
   - Add `serde_yaml = "0.9"` if not already present
   - Add `thiserror = "1.0"` for error handling
   - Run `cargo update` to fetch dependencies

3. **Implement Loader trait**

   - Create `src/domain/components/loader.rs`
   - Define `ComponentLoader` trait with methods:
     - `load(&self, path: &str) -> Result<Component, LoaderError>`
     - `load_all(&self, category: &str) -> Result<Vec<Component>, LoaderError>`
     - `load_by_name(&self, name: &str) -> Result<Component, LoaderError>`
   - Add `LoaderError` enum with `thiserror` for error handling
   - Export trait in `src/domain/components/mod.rs`

4. **Implement EmbeddedLoader**

   - Create `src/infrastructure/component_loaders/embedded_loader.rs`
   - Use `#[derive(RustEmbed)]` to embed `components/defaults/**/*.md`
   - Implement `EmbeddedLoader` struct
   - Implement `ComponentLoader` trait for `EmbeddedLoader`
   - Implement `new()` constructor
   - Reuse existing `parse_markdown_component` function
   - Add proper error handling with `Result<T, LoaderError>`

5. **Add tests for EmbeddedLoader**

   - Create tests in `embedded_loader.rs` with `#[cfg(test)]`
   - Test loading valid embedded component
   - Test loading non-existent component (should return Err)
   - Test loading all components by category
   - Test loading component by name
   - Ensure >80% code coverage

6. **Update build process**
   - Verify rust-embed works in build
   - Test binary size impact (should be minimal, approximately 100KB for all components)
   - Update `Makefile` if needed
   - Document embedded resources in README

**Deliverables**:

- `components/defaults/` directory with all existing components
- `src/domain/components/loader.rs` (approximately 80 lines)
- `src/infrastructure/component_loaders/embedded_loader.rs` (approximately 200 lines)
- Updated `components/README.md`
- Updated `Cargo.toml` with dependencies

**Estimated Effort**: 4-5 hours

**Validation Criteria**:

- All existing components accessible via EmbeddedLoader
- All tests pass with >80% coverage
- `cargo fmt --all` passes
- `cargo check --all-targets --all-features` passes
- `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- `cargo test --all-features` passes
- Binary builds successfully with embedded components

### Phase 2: Configuration for Local Components (Priority: HIGH)

**Goal**: Add configuration support for local component directory

**Tasks**:

1. **Extend Config struct**

   - Add `components_path` field to `src/infrastructure/config/mod.rs`
   - Set default to `~/.config/xzagentz/components`
   - Support environment variable `XZAGENTZ_COMPONENTS_PATH`
   - Use `PathBuf` type for path storage
   - Add serde serialization support

2. **Add path resolution helper**

   - Create `src/infrastructure/config/paths.rs`
   - Implement `resolve_components_path()` function
   - Handle tilde expansion for `~/` using `dirs` crate
   - Handle environment variable substitution using `std::env`
   - Return `Result<PathBuf, ConfigError>`
   - Ensure path is absolute

3. **Add dirs crate dependency**

   - Add `dirs = "5.0"` to `Cargo.toml` for XDG directory support
   - Use `dirs::config_dir()` for platform-specific config directory

4. **Update configuration loading**

   - Modify configuration parser to load components path
   - Add validation for components path
   - Ensure path is created if it doesn't exist (with user confirmation)
   - Add proper error messages for path issues

5. **Add tests for path configuration**
   - Create tests in `paths.rs` with `#[cfg(test)]`
   - Test default path resolution
   - Test environment variable override
   - Test tilde expansion
   - Test absolute path handling
   - Test path creation scenarios
   - Mock environment variables in tests

**Deliverables**:

- Updated `src/infrastructure/config/mod.rs` with `components_path` field
- `src/infrastructure/config/paths.rs` (approximately 150 lines)
- Tests in `paths.rs` (approximately 200 lines)
- Updated `Cargo.toml` with `dirs` dependency
- Updated configuration documentation

**Estimated Effort**: 3-4 hours

**Validation Criteria**:

- Configuration correctly resolves components path
- Environment variable override works
- All tests pass with >80% coverage
- `cargo fmt --all` passes
- `cargo check --all-targets --all-features` passes
- `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- `cargo test --all-features` passes

### Phase 3: Cascading Loader (Priority: HIGH)

**Goal**: Implement loader that tries local first, then embedded

**Tasks**:

1. **Update FileLoader to implement trait**

   - Modify `src/infrastructure/component_loaders/file_loader.rs`
   - Implement `ComponentLoader` trait for `FileLoader`
   - Ensure existing functionality preserved
   - Use `PathBuf` for path handling
   - Add proper error propagation with `?` operator

2. **Implement CascadingLoader**

   - Create `src/infrastructure/component_loaders/cascade_loader.rs`
   - Implement `CascadingLoader` struct with two `Box<dyn ComponentLoader>` fields
   - Implement `new(local: Box<dyn ComponentLoader>, embedded: Box<dyn ComponentLoader>)` constructor
   - Implement `ComponentLoader` trait
   - Implement `load()` - try local, fallback to embedded using `or_else`
   - Implement `load_all()` - merge local and embedded results
   - Implement `load_by_name()` - try local, fallback to embedded
   - Add `tracing` debug logs for which loader was used

3. **Handle component merging for load_all**

   - When loading all components, combine local and embedded
   - Local components override embedded by name (use HashMap for deduplication)
   - Preserve category filtering
   - Maintain component uniqueness
   - Return sorted results for consistency

4. **Add tracing support**

   - Add `tracing = "0.1"` to `Cargo.toml` if not present
   - Log debug messages: "Loading from local", "Falling back to embedded"
   - Use structured logging with component paths

5. **Add comprehensive tests**
   - Create tests in `cascade_loader.rs` with `#[cfg(test)]`
   - Test local override of embedded component
   - Test fallback to embedded when local missing
   - Test `load_all` merging logic with duplicates
   - Test local-only components
   - Test embedded-only components
   - Use mock loaders for testing (consider `mockall` crate)
   - Achieve >80% code coverage

**Deliverables**:

- Updated `src/infrastructure/component_loaders/file_loader.rs`
- `src/infrastructure/component_loaders/cascade_loader.rs` (approximately 250 lines)
- Tests in `cascade_loader.rs` (approximately 350 lines)
- Updated `src/infrastructure/component_loaders/mod.rs` to export new types

**Estimated Effort**: 5-6 hours

**Validation Criteria**:

- CascadingLoader correctly prioritizes local over embedded
- `load_all` correctly merges and deduplicates components
- All tests pass with >80% coverage
- `cargo fmt --all` passes
- `cargo check --all-targets --all-features` passes
- `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- `cargo test --all-features` passes
- Logging works correctly with tracing

### Phase 4: CLI Integration (Priority: MEDIUM)

**Goal**: Update CLI commands to use cascading loader

**Tasks**:

1. **Add clap dependency if needed**

   - Ensure `clap = { version = "4.0", features = ["derive"] }` in `Cargo.toml`
   - Update to use derive API if not already

2. **Update initialization commands**

   - Modify `src/api/cli/init.rs` to use CascadingLoader
   - Load components path from config
   - Initialize FileLoader with local path
   - Initialize EmbeddedLoader
   - Create CascadingLoader with both loaders
   - Pass loader to initialization logic

3. **Update generate command**

   - Modify `src/api/cli/generate.rs` to use CascadingLoader
   - Add `--verbose` flag to display component source (local vs embedded)
   - Use `tracing::info!` to log component sources when verbose

4. **Add components management commands**

   - Create `src/api/cli/components.rs`
   - Implement `ComponentsCommand` enum with subcommands:
     - `List` - show all available components
     - `Show { name: String }` - display component details
     - `Export { name: String, output: PathBuf }` - export to local directory
     - `Path` - show local components directory
   - Use `#[derive(Parser)]` for command structure
   - Implement command handlers in separate functions
   - Add proper error handling and user-friendly messages

5. **Update help text and documentation**
   - Update command help strings to explain local vs embedded
   - Add examples for customizing components
   - Document local component directory structure
   - Use clap's `about` and `long_about` attributes

**Deliverables**:

- Updated `src/api/cli/init.rs`
- Updated `src/api/cli/generate.rs`
- New `src/api/cli/components.rs` (approximately 300 lines)
- Tests in `components.rs` (approximately 250 lines)
- Updated command documentation in doc comments

**Estimated Effort**: 5-6 hours

**Validation Criteria**:

- CLI commands successfully use cascading loader
- Component management commands work correctly
- Help text is clear and accurate using `--help`
- All tests pass with >80% coverage
- `cargo fmt --all` passes
- `cargo check --all-targets --all-features` passes
- `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- `cargo test --all-features` passes
- Commands have proper error messages

### Phase 5: Default Component Templates (Priority: MEDIUM)

**Goal**: Create high-quality default components for common workflows

**Tasks**:

1. **Create core components**

   - `components/defaults/core/architecture.md` - Architecture template
   - `components/defaults/core/implementation_plan.md` - Implementation plan
     template
   - `components/defaults/core/agents_template.md` - AGENTS.md structure
     template
   - `components/defaults/core/project_overview.md` - Project overview template
   - All must have valid YAML frontmatter

2. **Enhance language components**

   - Review existing language components (Rust, Go, Python, etc.)
   - Ensure consistent structure and quality
   - Add missing common languages (TypeScript, Java, C++, etc.)
   - Include best practices and conventions for each language
   - Add Rust-specific idioms and patterns

3. **Add workflow components**

   - `components/defaults/general/testing_strategy.md` - Testing approach
   - `components/defaults/general/documentation_standards.md` - Doc standards
   - `components/defaults/general/code_review.md` - Review process
   - `components/defaults/general/git_workflow.md` - Git conventions
   - Align with AGENTS.md standards

4. **Validate all components**
   - Ensure all have valid YAML frontmatter with required fields
   - Verify metadata completeness (name, description, category, tags)
   - Check for consistent markdown formatting
   - Test loading with validation using component validator
   - Run `cargo test` to ensure all embedded

**Deliverables**:

- 4 new core components
- Enhanced language components (minimum 6 languages)
- 4 new workflow components
- All components validated and tested

**Estimated Effort**: 4-5 hours

**Validation Criteria**:

- All components load successfully via EmbeddedLoader
- All components pass validation rules
- Metadata is complete and accurate
- Content is high quality and useful
- No emojis in any component
- All filenames use lowercase with underscores
- `cargo test --all-features` passes

### Phase 6: Documentation and Examples (Priority: MEDIUM)

**Goal**: Comprehensive documentation for the hybrid component system

**Tasks**:

1. **Update main documentation**

   - Update `README.md` with hybrid component explanation
   - Add section on customizing components
   - Add examples of local overrides
   - Document component directory structure
   - Follow markdown standards (no emojis, proper formatting)

2. **Create usage guides**

   - Create `docs/how_to/customize_components.md`
   - Create `docs/how_to/create_custom_templates.md`
   - Create `docs/tutorials/getting_started_custom_workflow.md`
   - All filenames must use lowercase_with_underscores.md
   - Follow Diataxis framework for categorization

3. **Create reference documentation**

   - Create `docs/reference/component_structure.md`
   - Document YAML frontmatter schema with required fields
   - Document component categories (core, languages, tools, general, custom)
   - Document validation rules and error messages
   - Include code examples in proper format

4. **Add practical examples**

   - Create `examples/custom_components/` directory
   - Add example custom architecture template
   - Add example custom implementation plan template
   - Add example project-specific component
   - Include README in examples directory

5. **Create implementation summary**

   - Create `docs/explanation/hybrid_component_system_summary.md`
   - Document all components delivered with line counts
   - Include implementation details and architecture
   - Document testing approach and coverage
   - Include validation results
   - Add usage examples

6. **Update CHANGELOG**
   - Document new features in `CHANGELOG.md`
   - Document breaking changes (if any)
   - Provide migration guide if needed
   - Follow keep-a-changelog format

**Deliverables**:

- Updated `README.md`
- 3 new how-to guides (customize_components, create_custom_templates, getting_started_custom_workflow)
- 1 new tutorial
- 1 new reference document (component_structure)
- 1 implementation summary (hybrid_component_system_summary)
- Example custom components with README
- Updated `CHANGELOG.md`

**Estimated Effort**: 4-5 hours

**Validation Criteria**:

- Documentation is clear and complete
- Examples work correctly and can be loaded
- All documentation follows AGENTS.md standards
- No emojis in documentation
- All filenames use lowercase_with_underscores.md
- All code blocks specify language or path
- Markdownlint passes if configured
- Links in documentation are valid

## Testing Strategy

### Unit Tests

**Component Loading**:

- Test EmbeddedLoader with valid/invalid paths
- Test FileLoader with local components
- Test CascadingLoader precedence rules
- Test component validation logic
- Test metadata parsing with serde_yaml
- Use `#[test]` attribute for all tests
- Achieve >80% code coverage

**Configuration**:

- Test path resolution logic
- Test environment variable handling
- Test default values
- Test tilde expansion with dirs crate
- Test error cases (invalid paths, permissions)

**CLI Commands**:

- Test component list command output
- Test component export command functionality
- Test init with cascading loader
- Test generate with cascading loader
- Test command parsing with clap

### Integration Tests

**End-to-End Workflows**:

- Create `tests/integration_tests.rs`
- Install fresh binary → use embedded components
- Create local override → verify precedence
- Generate AGENTS.md with mixed components
- Export embedded component → modify → use modified version
- Test full CLI workflows

### Test Coverage Goals

- Unit test coverage: >80% for all new code
- Integration test coverage: All major workflows
- Edge cases: Invalid paths, file permissions, invalid formats, malformed YAML
- Use `cargo tarpaulin` or `cargo llvm-cov` for coverage reporting

## Migration Plan

### Backward Compatibility

**Existing Users**:

- No breaking changes to public API
- Existing component references continue to work
- FileLoader still available for direct use
- Configuration format unchanged (new field is optional)
- Default behavior uses cascading loader

**Migration Steps**:

1. Update xzagentz binary with `cargo install` or build from source
2. Embedded components work immediately (no action needed)
3. Optional: Create local overrides in `~/.config/xzagentz/components/`
4. Optional: Use `xzagentz components export` to customize embedded components

### Rollout Strategy

1. **Phase 1-2**: Internal testing with embedded components
2. **Phase 3**: Beta release with cascading loader
3. **Phase 4-5**: Full release with CLI commands and default templates
4. **Phase 6**: Documentation and examples complete

## Risk Assessment

### Technical Risks

**Risk**: rust-embed increases binary size

- **Likelihood**: Medium
- **Impact**: Low
- **Mitigation**: Components are text (Markdown), estimated <100KB total; acceptable for convenience

**Risk**: Path resolution issues across platforms (Windows, macOS, Linux)

- **Likelihood**: Medium
- **Impact**: Medium
- **Mitigation**: Extensive testing on all platforms; use `std::path::Path` and `PathBuf`; use `dirs` crate for platform-specific paths

**Risk**: File permissions on local component directory

- **Likelihood**: Low
- **Impact**: Low
- **Mitigation**: Clear error messages using `thiserror`; documentation on setup; handle permission errors gracefully

**Risk**: Trait object overhead for ComponentLoader

- **Likelihood**: Low
- **Impact**: Very Low
- **Mitigation**: Component loading is not performance-critical; acceptable overhead for flexibility

### User Experience Risks

**Risk**: Confusion about local vs embedded components

- **Likelihood**: Medium
- **Impact**: Medium
- **Mitigation**: Clear documentation; `--verbose` mode shows source; helpful error messages with context

**Risk**: Accidental override of embedded components

- **Likelihood**: Low
- **Impact**: Low
- **Mitigation**: Component export command makes intentional overrides explicit; documentation emphasizes deliberate customization

## Success Criteria

### Functional Requirements

- [ ] Embedded components load from binary using rust-embed
- [ ] Local components override embedded by name
- [ ] CascadingLoader correctly implements precedence rules
- [ ] CLI commands use cascading loader by default
- [ ] Component management commands work correctly
- [ ] All existing functionality preserved

### Quality Requirements

- [ ] All unit tests pass
- [ ] Test coverage >80% for new code
- [ ] `cargo fmt --all` passes
- [ ] `cargo check --all-targets --all-features` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] `cargo test --all-features` passes
- [ ] No security warnings from `cargo audit`
- [ ] Documentation complete and accurate

### User Experience Requirements

- [ ] Zero configuration for default usage
- [ ] Clear documentation for customization
- [ ] Helpful error messages with context
- [ ] Intuitive CLI commands with good help text
- [ ] Examples demonstrate workflows

## Timeline Estimates

| Phase                        | Effort | Dependencies | Start | Duration |
| ---------------------------- | ------ | ------------ | ----- | -------- |
| Phase 1: Embedded Components | 4-5h   | None         | Day 1 | 1 day    |
| Phase 2: Configuration       | 3-4h   | None         | Day 1 | 0.5 days |
| Phase 3: Cascading Loader    | 5-6h   | Phase 1, 2   | Day 2 | 1 day    |
| Phase 4: CLI Integration     | 5-6h   | Phase 3      | Day 3 | 1 day    |
| Phase 5: Default Templates   | 4-5h   | Phase 1      | Day 4 | 0.5 days |
| Phase 6: Documentation       | 4-5h   | All          | Day 4 | 0.5 days |

**Total Estimated Effort**: 25-31 hours (3-4 working days)

## Open Questions

1. **Should we support component versioning?**

   - Decision: Not in initial implementation, defer to future enhancement
   - Rationale: Adds complexity, not required for MVP workflow

2. **Should we validate local components on load?**

   - Decision: Yes, same validation as embedded components
   - Rationale: Prevents runtime errors, maintains quality, fails fast

3. **Should we cache local components in memory?**

   - Decision: Yes, implement caching with lazy loading
   - Rationale: Performance optimization for repeated loads

4. **Should we support remote components in Phase 1?**

   - Decision: No, defer to future enhancement
   - Rationale: Adds complexity (caching, versioning, security), not needed for
     core workflow

5. **Should we provide a component migration tool?**

   - Decision: Not needed, backward compatible
   - Rationale: Existing components become embedded, no user action required

6. **Should we use async for file I/O?**
   - Decision: No, use synchronous I/O for simplicity
   - Rationale: Component loading is not performance-critical; simpler implementation

## Dependencies

### Rust Crates

- `rust-embed = "8.0"` - Embedding static assets
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `serde_yaml = "0.9"` - YAML parsing
- `thiserror = "1.0"` - Error handling
- `dirs = "5.0"` - Platform-specific directories
- `tracing = "0.1"` - Structured logging
- `clap = { version = "4.0", features = ["derive"] }` - CLI parsing
- `walkdir = "2.0"` - Directory traversal (if needed)

### Build Tools

- Rust 1.70+ (already required)
- cargo-clippy (already in use)
- cargo-fmt (already in use)
- make (already in use)

### External Dependencies

None. This is entirely internal to xzagentz.

## References

- Component Distribution Strategy:
  `docs/explanation/component_distribution_strategy.md` (if exists)
- Component Loading Architecture: `src/infrastructure/component_loaders/`
- Configuration Management: `src/infrastructure/config/`
- AGENTS.md Guidelines: `AGENTS.md`
- rust-embed crate: <https://docs.rs/rust-embed/>
- RustEmbed derive macro: <https://github.com/pyrossh/rust-embed>

## Appendix A: File Checklist

### New Files to Create

- [ ] `src/domain/components/loader.rs`
- [ ] `src/infrastructure/component_loaders/embedded_loader.rs`
- [ ] `src/infrastructure/component_loaders/cascade_loader.rs`
- [ ] `src/infrastructure/config/paths.rs`
- [ ] `src/api/cli/components.rs`
- [ ] `docs/how_to/customize_components.md`
- [ ] `docs/how_to/create_custom_templates.md`
- [ ] `docs/tutorials/getting_started_custom_workflow.md`
- [ ] `docs/reference/component_structure.md`
- [ ] `docs/explanation/hybrid_component_system_summary.md`
- [ ] `components/defaults/core/architecture.md`
- [ ] `components/defaults/core/implementation_plan.md`
- [ ] `components/defaults/core/agents_template.md`
- [ ] `examples/custom_components/README.md`
- [ ] `tests/integration_tests.rs`

### Files to Modify

- [ ] `src/domain/components/mod.rs` - Export Loader trait
- [ ] `src/infrastructure/component_loaders/mod.rs` - Export new loaders
- [ ] `src/infrastructure/component_loaders/file_loader.rs` - Implement Loader trait
- [ ] `src/infrastructure/config/mod.rs` - Add components_path field
- [ ] `src/api/cli/init.rs` - Use CascadingLoader
- [ ] `src/api/cli/generate.rs` - Use CascadingLoader
- [ ] `src/api/cli/mod.rs` - Add components command
- [ ] `README.md` - Document hybrid components
- [ ] `CHANGELOG.md` - Document new features
- [ ] `components/README.md` - Explain new structure
- [ ] `Cargo.toml` - Add dependencies

### Directories to Create

- [ ] `components/defaults/` - Move existing components here
- [ ] `examples/custom_components/` - Example customizations
- [ ] `tests/` - Integration tests (if not exists)

## Appendix B: Code Snippets

### Loader Trait

```rust
use thiserror::Error;

/// Errors that can occur when loading components
#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Component not found: {0}")]
    NotFound(String),

    #[error("Failed to parse component: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

/// Trait for loading components from various sources
pub trait ComponentLoader {
    /// Load a component from the specified path
    fn load(&self, path: &str) -> Result<Component, LoaderError>;

    /// Load all components in a category
    fn load_all(&self, category: &str) -> Result<Vec<Component>, LoaderError>;

    /// Load a component by name (searches all categories)
    fn load_by_name(&self, name: &str) -> Result<Component, LoaderError>;
}
```

### EmbeddedLoader Skeleton

```rust
use rust_embed::RustEmbed;
use crate::domain::components::{Component, ComponentLoader, LoaderError};

#[derive(RustEmbed)]
#[folder = "components/defaults/"]
struct EmbeddedComponents;

/// Loader for components embedded in the binary
pub struct EmbeddedLoader;

impl EmbeddedLoader {
    /// Create a new embedded loader
    pub fn new() -> Self {
        Self
    }

    /// Parse a markdown component from embedded content
    fn parse_component(&self, path: &str, content: &str) -> Result<Component, LoaderError> {
        // Implementation reuses existing parsing logic
        parse_markdown_component(content)
            .map_err(|e| LoaderError::ParseError(e.to_string()))
    }
}

impl Default for EmbeddedLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentLoader for EmbeddedLoader {
    fn load(&self, path: &str) -> Result<Component, LoaderError> {
        let file = EmbeddedComponents::get(path)
            .ok_or_else(|| LoaderError::NotFound(path.to_string()))?;

        let content = std::str::from_utf8(file.data.as_ref())
            .map_err(|e| LoaderError::ParseError(e.to_string()))?;

        self.parse_component(path, content)
    }

    fn load_all(&self, category: &str) -> Result<Vec<Component>, LoaderError> {
        let mut components = Vec::new();
        let prefix = format!("{}/", category);

        for file_path in EmbeddedComponents::iter() {
            if file_path.starts_with(&prefix) && file_path.ends_with(".md") {
                if let Ok(component) = self.load(&file_path) {
                    components.push(component);
                }
            }
        }

        Ok(components)
    }

    fn load_by_name(&self, name: &str) -> Result<Component, LoaderError> {
        for file_path in EmbeddedComponents::iter() {
            if let Ok(component) = self.load(&file_path) {
                if component.name == name {
                    return Ok(component);
                }
            }
        }

        Err(LoaderError::NotFound(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_embedded_component() {
        let loader = EmbeddedLoader::new();
        let result = loader.load("core/architecture.md");
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_nonexistent_component() {
        let loader = EmbeddedLoader::new();
        let result = loader.load("nonexistent/component.md");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_all_by_category() {
        let loader = EmbeddedLoader::new();
        let result = loader.load_all("core");
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
```

### CascadingLoader Skeleton

```rust
use crate::domain::components::{Component, ComponentLoader, LoaderError};
use std::collections::HashMap;
use tracing::debug;

/// Loader that tries local filesystem first, then falls back to embedded
pub struct CascadingLoader {
    local: Box<dyn ComponentLoader>,
    embedded: Box<dyn ComponentLoader>,
}

impl CascadingLoader {
    /// Create a new cascading loader
    pub fn new(local: Box<dyn ComponentLoader>, embedded: Box<dyn ComponentLoader>) -> Self {
        Self { local, embedded }
    }
}

impl ComponentLoader for CascadingLoader {
    fn load(&self, path: &str) -> Result<Component, LoaderError> {
        match self.local.load(path) {
            Ok(component) => {
                debug!("Loaded component from local: {}", path);
                Ok(component)
            }
            Err(_) => {
                debug!("Local component not found, falling back to embedded: {}", path);
                self.embedded.load(path)
            }
        }
    }

    fn load_all(&self, category: &str) -> Result<Vec<Component>, LoaderError> {
        let mut components_map: HashMap<String, Component> = HashMap::new();

        // Load embedded first (as defaults)
        if let Ok(embedded_components) = self.embedded.load_all(category) {
            for component in embedded_components {
                components_map.insert(component.name.clone(), component);
            }
        }

        // Load local (overrides embedded)
        if let Ok(local_components) = self.local.load_all(category) {
            for component in local_components {
                debug!("Local override for component: {}", component.name);
                components_map.insert(component.name.clone(), component);
            }
        }

        let mut components: Vec<Component> = components_map.into_values().collect();
        components.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(components)
    }

    fn load_by_name(&self, name: &str) -> Result<Component, LoaderError> {
        self.local
            .load_by_name(name)
            .or_else(|_| {
                debug!("Component not found locally, checking embedded: {}", name);
                self.embedded.load_by_name(name)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_overrides_embedded() {
        // Test that local components override embedded ones
        // Mock implementation needed
    }

    #[test]
    fn test_fallback_to_embedded() {
        // Test fallback when local component doesn't exist
        // Mock implementation needed
    }

    #[test]
    fn test_load_all_merges_correctly() {
        // Test that load_all merges and deduplicates
        // Mock implementation needed
    }
}
```

## Appendix C: Validation Commands

Before claiming any phase complete, run these commands:

```bash
# 1. Format code
cargo fmt --all

# 2. Check compilation
cargo check --all-targets --all-features

# 3. Lint with zero warnings
cargo clippy --all-targets --all-features -- -D warnings

# 4. Run all tests (must achieve >80% coverage)
cargo test --all-features

# 5. Run integration tests
cargo test --test integration_tests

# 6. Check for security vulnerabilities
cargo audit

# 7. Verify documentation builds
cargo doc --no-deps

# 8. Verify all documentation files exist
ls -la docs/explanation/hybrid_component_system_summary.md
ls -la docs/how_to/customize_components.md
ls -la docs/how_to/create_custom_templates.md

# 9. Verify components directory structure
ls -la components/defaults/core/
ls -la components/defaults/languages/

# 10. Build release binary and check size
cargo build --release
ls -lh target/release/xzagentz
```

Expected results: All commands complete successfully with zero errors and zero
warnings. Test coverage >80% for all new code.
