# Embedded Resources Implementation Plan

## Overview

This document outlines the plan to embed components and templates in the xzagentz binary, enabling the tool to function without external filesystem dependencies while still supporting customization through user-defined directories.

## Goals

1. Embed all components and templates at compile time into the binary
2. Add CLI command to extract embedded resources to user's config directory
3. Implement intelligent resource resolution with fallback hierarchy
4. Support environment variables for custom resource paths
5. Maintain backward compatibility with existing filesystem-based loading
6. Enable standalone binary distribution without external dependencies

## Problem Statement

Currently, xzagentz requires external `components/` and `templates/` directories to function. This creates several issues:

- Binary cannot be distributed standalone
- Users must maintain separate directory structures
- Installation is more complex than necessary
- Deployment in containerized environments requires volume mounts
- Missing components cause runtime errors rather than using sensible defaults

## Solution Architecture

### Resource Resolution Hierarchy

The system will resolve resources in the following order:

1. Environment variable path (if specified)
   - `XZAGENTZ_COMPONENTS` or `XZAGENTZ_COMPONENT_DIR`
   - `XZAGENTZ_TEMPLATES` or `XZAGENTZ_TEMPLATE_DIR`
2. User config directory: `~/.config/xzagentz/{components,templates}`
3. Embedded resources (fallback, always available)

This hierarchy ensures:
- Maximum flexibility for users
- Graceful degradation
- Zero external dependencies for basic functionality
- Easy customization when needed

### Architecture Layers Affected

```text
┌──────────────────────────────────────────────┐
│  API Layer (src/api/)                        │
│  - New CLI command: init                     │
├──────────────────────────────────────────────┤
│  Application Layer (src/application/)        │
│  - Resource extraction logic                 │
├──────────────────────────────────────────────┤
│  Infrastructure Layer (src/infrastructure/)  │
│  - EmbeddedResources module (NEW)            │
│  - Modified ComponentLoader                  │
│  - Modified TemplateLoader                   │
└──────────────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Embedded Resources Infrastructure

**Objective**: Create infrastructure for embedding and accessing resources at compile time.

#### Components to Implement

1. **New Module**: `src/infrastructure/embedded.rs`
   - Embed `components/` directory using `include_dir!` macro
   - Embed `templates/` directory using `include_dir!` macro
   - Provide API to access embedded files
   - Support listing embedded resources
   - Support reading embedded file contents

2. **Dependencies to Add** (Cargo.toml):
   ```toml
   include_dir = "0.7"  # For embedding directories at compile time
   ```

3. **Public API Design**:
   ```rust
   pub struct EmbeddedResources;

   impl EmbeddedResources {
       pub fn get_component(path: &str) -> Option<&'static str>;
       pub fn get_template(path: &str) -> Option<&'static str>;
       pub fn list_components(component_type: ComponentType) -> Vec<String>;
       pub fn list_templates() -> Vec<String>;
       pub fn extract_to(destination: &Path) -> Result<()>;
   }
   ```

**Files to Create**:
- `src/infrastructure/mod.rs` (if not exists)
- `src/infrastructure/embedded.rs`

**Estimated Lines**: ~300-400 lines including tests

### Phase 2: Enhanced Loaders

**Objective**: Modify existing loaders to support embedded resources as fallback.

#### ComponentLoader Modifications

**File**: `src/components/loader.rs`

**Changes Required**:

1. Add new field to track resource source:
   ```rust
   pub enum ResourceSource {
       Filesystem(PathBuf),
       Embedded,
   }
   ```

2. Modify `ComponentLoader::new()` to accept optional path and use resolution hierarchy

3. Add method `resolve_component_dir()`:
   ```rust
   fn resolve_component_dir() -> ResourceSource {
       // 1. Check XZAGENTZ_COMPONENTS env var
       // 2. Check XZAGENTZ_COMPONENT_DIR env var
       // 3. Check ~/.config/xzagentz/components
       // 4. Fall back to embedded
   }
   ```

4. Modify `load()` method to check embedded resources if filesystem fails

5. Modify `list()` method to return embedded resources if no filesystem dir exists

**Backward Compatibility**: Existing code using `ComponentLoader::new(Some(path))` continues to work unchanged.

#### TemplateLoader Modifications

**File**: `src/templates/loader.rs`

**Changes Required**:

1. Add `ResourceSource` enum (same as ComponentLoader)

2. Modify `TemplateLoader::new()` to use resolution hierarchy

3. Add method `resolve_template_dir()`:
   ```rust
   fn resolve_template_dir() -> ResourceSource {
       // 1. Check XZAGENTZ_TEMPLATES env var
       // 2. Check XZAGENTZ_TEMPLATE_DIR env var
       // 3. Check ~/.config/xzagentz/templates
       // 4. Fall back to embedded
   }
   ```

4. Modify `load()` to check embedded templates if filesystem fails

5. Modify `list()` to return embedded templates if no filesystem dir exists

**Backward Compatibility**: Existing code using `TemplateLoader::with_directory()` continues to work unchanged.

**Estimated Lines**: ~200-300 lines of modifications plus tests

### Phase 3: CLI Init Command

**Objective**: Provide user-friendly command to extract embedded resources to config directory.

#### New CLI Command

**File**: `src/cli/init.rs`

**Command Specification**:
```bash
xzagentz init [OPTIONS]

OPTIONS:
    --components-dir <PATH>    Target directory for components [default: ~/.config/xzagentz/components]
    --templates-dir <PATH>     Target directory for templates [default: ~/.config/xzagentz/templates]
    --force                    Overwrite existing files
    --dry-run                  Show what would be extracted without writing files
```

**Functionality**:

1. Validate target directories
2. Create directories if they don't exist
3. Extract embedded resources to filesystem
4. Report extraction results (files created, skipped, overwritten)
5. Handle conflicts (existing files) based on `--force` flag
6. Support `--dry-run` for preview

**Output Format**:
```text
Initializing xzagentz configuration...

Extracting components to: /Users/username/.config/xzagentz/components
  ✓ core/quick_reference.md
  ✓ core/critical_rules.md
  ✓ languages/rust.md
  ✓ languages/python.md
  ... (20 files)

Extracting templates to: /Users/username/.config/xzagentz/templates
  ✓ plans/rust-binary.toml
  ✓ plans/python-service.toml
  ... (5 files)

Successfully initialized xzagentz configuration!

To customize components or templates, edit files in:
  - /Users/username/.config/xzagentz/components
  - /Users/username/.config/xzagentz/templates
```

**Files to Create**:
- `src/cli/init.rs`

**Files to Modify**:
- `src/cli/mod.rs` (add init command to Commands enum)

**Estimated Lines**: ~250-350 lines including tests and documentation

### Phase 4: Configuration Updates

**Objective**: Update configuration system to support new resource paths.

#### Changes Required

**File**: `src/config/project.rs`

1. Add methods to get default resource directories:
   ```rust
   pub fn default_components_dir() -> PathBuf;
   pub fn default_templates_dir() -> PathBuf;
   pub fn default_config_dir() -> PathBuf;
   ```

2. Update `ProjectConfig` to track resource sources

**File**: `src/cli/mod.rs`

1. Environment variable standardization:
   - Support both `XZAGENTZ_COMPONENTS` and `XZAGENTZ_COMPONENT_DIR` (for compatibility)
   - Support both `XZAGENTZ_TEMPLATES` and `XZAGENTZ_TEMPLATE_DIR` (for compatibility)

2. Update CLI help text to document new behavior

**Estimated Lines**: ~100-150 lines of modifications

### Phase 5: Testing Strategy

**Objective**: Comprehensive test coverage for embedded resources functionality.

#### Unit Tests

1. **EmbeddedResources Tests** (`src/infrastructure/embedded.rs`):
   - Test component retrieval by path
   - Test template retrieval by path
   - Test listing all components
   - Test listing all templates
   - Test extraction to filesystem
   - Test extraction with existing files
   - Test extraction to non-writable directory (error case)

2. **ComponentLoader Tests** (`src/components/loader.rs`):
   - Test loading from embedded resources
   - Test fallback to embedded when filesystem missing
   - Test environment variable override
   - Test ~/.config/xzagentz/components directory
   - Test resource resolution hierarchy
   - Test listing embedded components

3. **TemplateLoader Tests** (`src/templates/loader.rs`):
   - Test loading from embedded resources
   - Test fallback to embedded when filesystem missing
   - Test environment variable override
   - Test ~/.config/xzagentz/templates directory
   - Test resource resolution hierarchy
   - Test listing embedded templates

4. **Init Command Tests** (`src/cli/init.rs`):
   - Test extraction to default directory
   - Test extraction to custom directory
   - Test --force flag behavior
   - Test --dry-run flag behavior
   - Test conflict resolution
   - Test error handling (permissions, disk space)

#### Integration Tests

**File**: `tests/integration/embedded_resources.rs` (new)

1. Test end-to-end workflow:
   - Run binary without external resources
   - Verify component loading from embedded
   - Run init command
   - Verify filesystem resources created
   - Verify subsequent loads use filesystem

2. Test environment variable override:
   - Set custom paths via env vars
   - Verify resources loaded from custom paths

3. Test missing resources:
   - Remove filesystem resources
   - Verify fallback to embedded

**Estimated Lines**: ~600-800 lines of tests

### Phase 6: Documentation

**Objective**: Document new features and usage patterns.

#### Documentation Files to Create/Update

1. **User Guide**: `docs/how_to/setup_custom_resources.md`
   - How to initialize configuration
   - How to customize components
   - How to customize templates
   - How to use environment variables
   - Troubleshooting resource loading

2. **Reference**: `docs/reference/environment_variables.md`
   - Complete list of supported environment variables
   - Resource resolution hierarchy explanation
   - Examples of each configuration method

3. **Architecture**: `docs/explanation/embedded_resources_architecture.md`
   - Technical implementation details
   - Design decisions and rationale
   - Resource resolution algorithm
   - Performance considerations

4. **README Updates**: `README.md`
   - Add section on embedded resources
   - Update installation instructions
   - Add quick start examples

5. **Code Documentation**:
   - Add comprehensive doc comments to all new public APIs
   - Include runnable examples in doc tests
   - Document error conditions

**Estimated Lines**: ~1000-1500 lines of documentation

## Technical Specifications

### Resource Resolution Algorithm

```rust
fn resolve_resource_path(
    resource_type: ResourceType,
    env_var_primary: &str,
    env_var_secondary: &str,
    default_subdir: &str,
) -> ResourceSource {
    // 1. Check primary environment variable
    if let Ok(path) = env::var(env_var_primary) {
        let path = PathBuf::from(path);
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 2. Check secondary environment variable (backward compatibility)
    if let Ok(path) = env::var(env_var_secondary) {
        let path = PathBuf::from(path);
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 3. Check user config directory
    if let Some(config_dir) = dirs::config_dir() {
        let path = config_dir.join("xzagentz").join(default_subdir);
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 4. Fall back to embedded
    ResourceSource::Embedded
}
```

### Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `XZAGENTZ_COMPONENTS` | Override component directory location | `/custom/components` |
| `XZAGENTZ_COMPONENT_DIR` | Alternative name (backward compat) | `/custom/components` |
| `XZAGENTZ_TEMPLATES` | Override template directory location | `/custom/templates` |
| `XZAGENTZ_TEMPLATE_DIR` | Alternative name (backward compat) | `/custom/templates` |
| `XZAGENTZ_CONFIG_DIR` | Override entire config directory | `/custom/config` |

### File Structure After Init

```text
~/.config/xzagentz/
├── components/
│   ├── core/
│   │   ├── quick_reference.md
│   │   ├── critical_rules.md
│   │   ├── project_overview.md
│   │   └── development_workflow.md
│   ├── general/
│   │   ├── documentation.md
│   │   └── validation.md
│   ├── languages/
│   │   ├── rust.md
│   │   ├── python.md
│   │   └── go.md
│   └── tools/
│       ├── docker.md
│       ├── git.md
│       └── ci_cd.md
└── templates/
    ├── plans/
    │   ├── rust-binary.toml
    │   ├── rust-library.toml
    │   └── python-service.toml
    └── prompts/
        ├── default.toml
        └── minimal.toml
```

## Error Handling

### Error Types to Add

```rust
#[derive(Error, Debug)]
pub enum EmbeddedResourceError {
    #[error("Failed to extract embedded resources: {0}")]
    ExtractionFailed(String),

    #[error("Resource not found in embedded data: {0}")]
    NotFound(String),

    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreation {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write file {path}: {source}")]
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },
}
```

### Error Recovery

- If filesystem resource fails to load, automatically fall back to embedded
- If extraction fails for one file, continue with remaining files
- Report all errors at end of init command
- Provide clear error messages with actionable suggestions

## Performance Considerations

### Memory Usage

- Embedded resources increase binary size by ~500KB-1MB (estimated)
- Resources are stored in read-only data section (no runtime memory overhead)
- Only accessed resources are loaded into memory (lazy loading where possible)

### Disk I/O

- Filesystem checks happen once at initialization (cached afterward)
- Embedded resource access has zero I/O overhead
- Init command is one-time operation (minimal performance impact)

### Build Time

- `include_dir!` macro increases compilation time by ~1-2 seconds
- Only affects initial compilation and when embedded files change
- Acceptable tradeoff for deployment simplicity

## Security Considerations

1. **File Permissions**: Init command respects umask, creates files with secure permissions
2. **Path Traversal**: Validate all paths to prevent directory traversal attacks
3. **Overwrite Protection**: Require explicit --force flag to overwrite existing files
4. **Read-Only Embedded Data**: Embedded resources are immutable, preventing tampering

## Migration Path

### For Existing Users

1. **No Breaking Changes**: Existing installations continue to work unchanged
2. **Optional Migration**: Users can run `xzagentz init` to migrate to new structure
3. **Backward Compatibility**: All existing CLI flags and env vars remain functional

### Upgrade Steps

```bash
# Step 1: Upgrade binary
cargo install xzagentz --version 0.2.0

# Step 2: Initialize config (optional)
xzagentz init

# Step 3: Customize if needed
vim ~/.config/xzagentz/components/core/quick_reference.md
```

## Validation Checklist

Before claiming implementation complete, verify:

### Code Quality
- [ ] `cargo fmt --all` applied successfully
- [ ] `cargo check --all-targets --all-features` passes with zero errors
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] `cargo test --all-features` passes with >80% coverage

### Functionality
- [ ] Binary works without external resources
- [ ] Init command creates correct directory structure
- [ ] Resource resolution follows documented hierarchy
- [ ] Environment variables work correctly
- [ ] Backward compatibility maintained

### Testing
- [ ] Unit tests for all new functions
- [ ] Integration tests for end-to-end workflows
- [ ] Error cases covered
- [ ] Edge cases tested

### Documentation
- [ ] All public APIs have doc comments with examples
- [ ] User guides created
- [ ] Reference documentation updated
- [ ] README.md updated
- [ ] This plan document updated with actual implementation details

## Success Criteria

1. Binary can be distributed as single executable with no external dependencies
2. Users can customize resources by running `xzagentz init`
3. All existing functionality continues to work unchanged
4. Resource resolution is transparent and predictable
5. Test coverage >80% for new code
6. All quality gates pass
7. Documentation is complete and accurate

## Risk Assessment

### Low Risk
- Adding embedded resources (well-established pattern in Rust)
- Backward compatibility (careful API design)
- Testing (comprehensive test plan)

### Medium Risk
- Cross-platform path handling (mitigated by using `dirs` crate)
- File permissions during extraction (mitigated by respecting umask)

### Mitigation Strategies
- Extensive testing on macOS, Linux, Windows
- Clear error messages for all failure scenarios
- Dry-run mode for init command
- Comprehensive documentation

## Timeline Estimate

- Phase 1 (Embedded Infrastructure): 4-6 hours
- Phase 2 (Enhanced Loaders): 4-6 hours
- Phase 3 (CLI Init Command): 3-4 hours
- Phase 4 (Configuration Updates): 2-3 hours
- Phase 5 (Testing): 4-6 hours
- Phase 6 (Documentation): 3-4 hours

**Total Estimated Time**: 20-29 hours

## Dependencies

### New Crate Dependencies
- `include_dir = "0.7"` - For embedding directories at compile time

### Existing Dependencies (no changes)
- `dirs = "5.0"` - For cross-platform config directory location
- `clap` - CLI already supports env vars
- `std::env` - For environment variable access

## Future Enhancements

Potential future improvements (out of scope for this plan):

1. **Selective Embedding**: Compile-time flags to include/exclude specific components
2. **Compression**: Compress embedded resources to reduce binary size
3. **Hot Reload**: Watch filesystem resources and reload on change
4. **Remote Resources**: Support loading components from URLs
5. **Component Registry**: Central registry of community components
6. **Versioning**: Track versions of embedded vs filesystem resources

## References

- AGENTS.md - Project development guidelines
- `include_dir` crate documentation: https://docs.rs/include_dir
- Rust embedded resources patterns: https://rust-lang-nursery.github.io/rust-cookbook/
- XDG Base Directory Specification: https://specifications.freedesktop.org/basedir-spec/

## Appendix A: File Manifest

### Files to Create
1. `src/infrastructure/mod.rs` (if not exists)
2. `src/infrastructure/embedded.rs`
3. `src/cli/init.rs`
4. `tests/integration/embedded_resources.rs`
5. `docs/how_to/setup_custom_resources.md`
6. `docs/reference/environment_variables.md`
7. `docs/explanation/embedded_resources_architecture.md`

### Files to Modify
1. `Cargo.toml` - Add include_dir dependency
2. `src/components/loader.rs` - Add embedded resource support
3. `src/templates/loader.rs` - Add embedded resource support
4. `src/cli/mod.rs` - Add init command
5. `src/error.rs` - Add EmbeddedResourceError variants
6. `README.md` - Document new features

### Estimated Total Lines
- New code: ~1500-2000 lines
- Modified code: ~500-700 lines
- Tests: ~600-800 lines
- Documentation: ~1000-1500 lines

**Grand Total**: ~3600-5000 lines

## Appendix B: Example Usage Scenarios

### Scenario 1: First-time User
```bash
# Download binary
curl -L https://github.com/xbcsmith/xzagentz/releases/latest/download/xzagentz -o xzagentz
chmod +x xzagentz

# Works immediately with embedded resources
./xzagentz create --interactive

# Optionally customize
./xzagentz init
vim ~/.config/xzagentz/components/core/critical_rules.md
```

### Scenario 2: CI/CD Pipeline
```bash
# Single binary in container, no volumes needed
FROM rust:alpine
COPY xzagentz /usr/local/bin/
RUN xzagentz create --template rust-binary --output /workspace/AGENTS.md
```

### Scenario 3: Custom Organization Templates
```bash
# Initialize with company-specific components
export XZAGENTZ_COMPONENTS=/company/standards/components
export XZAGENTZ_TEMPLATES=/company/standards/templates

xzagentz create --template company-standard
```

### Scenario 4: Development Workflow
```bash
# Developer customizes local templates
xzagentz init
cd ~/.config/xzagentz/templates
git init
git remote add origin git@company.com:standards/xzagentz-templates.git
git pull

# Use customized templates
xzagentz create --template custom-rust-service
```

---

**Document Status**: Planning Complete
**Next Steps**: Begin Phase 1 implementation after approval
**Questions/Clarifications**: None pending
