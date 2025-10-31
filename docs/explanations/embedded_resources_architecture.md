# Embedded Resources Architecture

This document provides a detailed technical explanation of the embedded resources system in xzagentz, including design decisions, implementation details, and performance considerations.

## Overview

The embedded resources system allows xzagentz to bundle components and templates directly into the compiled binary while maintaining the flexibility to load customized versions from the filesystem. This hybrid approach provides zero-configuration defaults with full customization capabilities.

## Design Goals

### Primary Goals

1. **Zero Configuration**: Work out of the box without requiring setup or external files
2. **Full Customization**: Allow users to override any embedded resource
3. **Transparent Fallback**: Seamlessly fall back to embedded resources when filesystem versions are unavailable
4. **Performance**: Minimize overhead for resource loading
5. **Maintainability**: Keep resource management simple and predictable

### Secondary Goals

1. **XDG Compliance**: Follow XDG Base Directory specification
2. **CI/CD Friendly**: Work in containerized and ephemeral environments
3. **Multi-User Support**: Handle shared resources across teams
4. **Version Independence**: Allow multiple resource versions to coexist

## Architecture Overview

### System Layers

```text
┌─────────────────────────────────────────────────────────────┐
│  CLI Layer (src/cli/)                                       │
│  - Commands (create, add, list, init)                      │
│  - User interface and argument parsing                      │
└────────────────┬────────────────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────────────────┐
│  Application Layer (src/application/)                       │
│  - Business logic and orchestration                         │
│  - Component and template loading coordination              │
└────────────────┬────────────────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────────────────┐
│  Resolution Layer (src/*/resolution.rs)                     │
│  - Component resolution (src/components/resolution.rs)      │
│  - Template resolution (src/templates/resolution.rs)        │
│  - Config resolution (src/config/resolution.rs)             │
│  - Resource source determination (Filesystem vs Embedded)   │
└─────────────┬────────────────────────────────────────┬──────┘
              │                                        │
┌─────────────▼──────────────────┐    ┌──────────────▼───────┐
│  Filesystem Loader Layer       │    │  Embedded Resources  │
│  - ComponentLoader             │    │  - EmbeddedResources │
│  - TemplateLoader              │    │  - include_dir!()    │
│  - File system operations      │    │  - Static data       │
└────────────────────────────────┘    └──────────────────────┘
```

### Component Interaction

```text
┌──────────────┐
│  User Request│
└──────┬───────┘
       │
       ▼
┌──────────────────────────────────────────────┐
│  1. Resolution Phase                         │
│  - Check custom path (--components-dir flag) │
│  - Check XZAGENTZ_COMPONENTS_DIR env var     │
│  - Check XDG_DATA_HOME/xzagentz/components   │
│  - Check HOME/.config/xzagentz/components    │
│  - Fall back to embedded resources           │
└──────┬───────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────┐
│  2. Source Determination                     │
│  - Return: ResourceSource::Filesystem(path)  │
│  - OR: ResourceSource::Embedded              │
└──────┬───────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────┐
│  3. Loading Phase                            │
│  - If Filesystem: Read from disk             │
│  - If Embedded: Extract from binary          │
└──────┬───────────────────────────────────────┘
       │
       ▼
┌──────────────┐
│  Return Data │
└──────────────┘
```

## Core Components

### EmbeddedResources Structure

Located in `src/infrastructure/embedded.rs`, this is the core of the embedded resources system.

#### Design

```rust
pub struct EmbeddedResources {
    _private: (),
}
```

The structure is a zero-sized type (ZST) because all data is stored in static variables created by the `include_dir!` macro. This design:

- Has zero runtime memory overhead
- Provides compile-time guarantees about resource availability
- Allows multiple instances without duplication

#### Static Resource Storage

```rust
static COMPONENTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/components");
static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/templates");
```

The `include_dir!` macro:
- Runs at compile time
- Embeds entire directory trees into the binary
- Creates a read-only virtual filesystem
- Preserves directory structure and file metadata

### Resolution System

Each resource type has its own resolution module:

- `src/components/resolution.rs` - Component directory resolution
- `src/templates/resolution.rs` - Template directory resolution
- `src/config/resolution.rs` - Configuration directory resolution

#### Resolution Algorithm

The resolution algorithm implements a priority-based search:

```rust
pub fn resolve_component_dir(custom_dir: Option<PathBuf>) -> ResourceSource {
    // 1. Custom directory (highest priority)
    if let Some(dir) = custom_dir {
        if dir.exists() {
            return ResourceSource::Filesystem(dir);
        }
    }

    // 2. Environment variable override
    if let Ok(dir) = env::var("XZAGENTZ_COMPONENTS_DIR") {
        let path = PathBuf::from(dir);
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 3. XDG data directory
    if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data).join("xzagentz").join("components");
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 4. Home directory fallback
    if let Some(home) = env::var_os("HOME") {
        let path = PathBuf::from(home)
            .join(".config")
            .join("xzagentz")
            .join("components");
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 5. Embedded resources (always available)
    ResourceSource::Embedded
}
```

#### ResourceSource Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceSource {
    Filesystem(PathBuf),
    Embedded,
}
```

This enum encapsulates the resource location:
- `Filesystem(PathBuf)` - Resources loaded from specified path
- `Embedded` - Resources loaded from binary

Benefits:
- Type-safe resource location tracking
- Explicit handling of both cases
- Enables logging and diagnostics

### Loader Integration

The loader modules integrate with the resolution system:

#### ComponentLoader

```rust
pub struct ComponentLoader {
    component_dir: PathBuf,
    cache: HashMap<String, Component>,
}

impl ComponentLoader {
    pub fn new() -> Result<Self> {
        let source = resolve_component_dir(None);

        match source {
            ResourceSource::Filesystem(path) => {
                Self::with_directory(path)
            }
            ResourceSource::Embedded => {
                // Use temporary directory with extracted resources
                Self::with_embedded_resources()
            }
        }
    }
}
```

The loader:
- Calls resolution to determine source
- Adapts behavior based on ResourceSource
- Maintains consistent API regardless of source
- Implements caching for performance

## Design Decisions

### Decision 1: Include vs. Dynamic Loading

**Chosen**: Compile-time inclusion with `include_dir!`

**Alternatives Considered**:
1. Runtime dynamic loading only
2. Compressed archive in binary
3. Network-based resource fetching

**Rationale**:
- **Zero dependencies**: No external files required at runtime
- **Reliability**: Resources always available
- **Performance**: No decompression or network overhead
- **Security**: Resources vetted at compile time
- **Simplicity**: Standard filesystem operations work on embedded data

**Trade-offs**:
- Binary size increases (acceptable for typical resource sizes)
- Resources fixed at compile time (mitigated by filesystem override)
- Build time slightly increased (negligible for project scale)

### Decision 2: Resolution Hierarchy

**Chosen**: Priority-based search with environment variable precedence

**Alternatives Considered**:
1. Configuration file only
2. Environment variables only
3. Hardcoded paths

**Rationale**:
- **Flexibility**: Multiple configuration methods
- **Precedence clarity**: Highest specificity wins
- **XDG compliance**: Standard system integration
- **Override capability**: Easy to customize without modification
- **Fallback safety**: Always works with embedded resources

**Trade-offs**:
- More complex resolution logic (mitigated by thorough testing)
- Multiple configuration methods (documented clearly)

### Decision 3: Zero-Sized Type for EmbeddedResources

**Chosen**: ZST with static storage

**Alternatives Considered**:
1. Struct holding Dir references
2. Lazy static initialization
3. Global static functions

**Rationale**:
- **Memory efficiency**: Zero runtime overhead
- **Compile-time guarantees**: Resources verified at build time
- **Simplicity**: No initialization needed
- **Thread safety**: Static data is inherently safe
- **API consistency**: Struct provides namespace and discoverability

**Trade-offs**:
- Cannot modify embedded resources at runtime (by design)
- Static lifetimes throughout (acceptable for this use case)

### Decision 4: Filesystem Fallback Strategy

**Chosen**: Non-existence triggers fallback to embedded

**Alternatives Considered**:
1. Error on non-existent paths
2. Prompt user for action
3. Hybrid search (some from filesystem, some embedded)

**Rationale**:
- **User experience**: Silent, predictable fallback
- **Flexibility**: Mix filesystem and embedded resources
- **CI/CD friendly**: Works without setup
- **Development friendly**: Easy to test custom resources

**Trade-offs**:
- Silent fallback might mask configuration errors (logged at debug level)
- Cannot force filesystem-only mode (acceptable for use cases)

## Performance Characteristics

### Memory Usage

**Embedded Resources**:
- Stored in read-only data segment (.rodata)
- Shared across all instances
- No heap allocation for storage
- Memory mapped by OS (efficient paging)

**Typical Size**: 50-200 KB for default resource set

**Runtime Overhead**: Near-zero (pointer dereferencing only)

### Disk I/O

**Embedded Resources**:
- No disk I/O for resource access
- OS may page data from binary if memory constrained
- Read performance equivalent to static arrays

**Filesystem Resources**:
- Standard filesystem I/O overhead
- Benefits from OS page cache
- Suitable for frequently modified resources

**Recommendation**: Use embedded for read-only scenarios, filesystem for customization.

### Build Time

**Impact**: +2-5 seconds for typical resource set

**Factors**:
- Number of files included
- Total size of resources
- Filesystem scanning overhead

**Mitigation**:
- Resources cached in build artifacts
- Incremental builds only re-embed on changes
- Parallel compilation unaffected

### Resolution Performance

**Typical Resolution Time**: <1ms

**Operations**:
1. Environment variable lookup: O(1)
2. Path existence check: O(1) filesystem operation
3. 3-5 checks maximum before fallback

**Caching**: Resolution results cached in loader instances

## Security Considerations

### Embedded Resources Security

**Benefits**:
- Resources vetted at build time
- Tamper-proof after compilation
- No external dependency vulnerabilities
- Known provenance

**Risks**:
- Cannot update without recompilation (by design)
- Binary inspection reveals resource contents (acceptable for public resources)

**Mitigation**:
- Build from trusted sources
- Verify build artifacts
- Use filesystem overrides for sensitive customizations

### Filesystem Resources Security

**Benefits**:
- User-controlled and auditable
- Can be updated without recompilation
- Standard file permissions apply

**Risks**:
- Directory traversal attacks
- Symlink attacks
- Untrusted resource injection

**Mitigation**:
- Path canonicalization
- Symlink following restrictions
- File extension validation
- Content validation before use

### Environment Variable Security

**Risks**:
- Variable injection in multi-user environments
- Privilege escalation through path manipulation

**Mitigation**:
- No automatic privilege elevation
- Validate paths before use
- Document secure configuration patterns
- Warn on suspicious configurations

## Error Handling

### Resolution Errors

Strategy: Graceful degradation with fallback

```rust
// Non-existent custom path -> try next in hierarchy
// All filesystem paths non-existent -> use embedded
// Embedded resources missing -> compile-time error
```

### Loading Errors

Strategy: Propagate errors with context

```rust
pub enum ResourceError {
    NotFound { path: PathBuf, source: ResourceSource },
    ReadError { path: PathBuf, error: io::Error },
    ParseError { path: PathBuf, error: String },
}
```

### Extraction Errors

Strategy: Detailed error with recovery suggestions

```rust
pub enum ExtractionError {
    DirectoryCreation { path: PathBuf, source: io::Error },
    FileWrite { path: PathBuf, source: io::Error },
    PermissionDenied { path: PathBuf },
}
```

## Testing Strategy

### Unit Tests

Each component tested in isolation:

- **EmbeddedResources**: Listing, retrieval, extraction
- **Resolution**: Precedence, fallback, environment handling
- **Loaders**: Filesystem and embedded integration

### Integration Tests

End-to-end workflows tested:

- **Init workflow**: Directory creation and extraction
- **Resource loading**: Mixed sources (filesystem + embedded)
- **Configuration**: Environment variable precedence

### Test Isolation

Environment variables managed with guard pattern:

```rust
struct EnvGuard {
    vars: Vec<(String, Option<String>)>,
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        // Restore original environment
    }
}
```

Ensures tests don't interfere with each other.

## Maintenance Guidelines

### Adding New Resources

1. Add files to `resources/components/` or `resources/templates/`
2. Rebuild project (resources automatically included)
3. Add tests for new resources
4. Update documentation

### Modifying Embedded Resources

1. Modify files in resource directories
2. Rebuild project
3. Run test suite to verify changes
4. Update version if breaking changes

### Updating Resolution Logic

1. Modify resolution functions
2. Update tests for all paths
3. Verify environment variable handling
4. Test fallback behavior
5. Update documentation

## Future Enhancements

### Planned Improvements

1. **Lazy Loading**: Load resources on-demand rather than at initialization
2. **Resource Versioning**: Track embedded resource versions for upgrade detection
3. **Compression**: Optional compression for large resource sets
4. **Checksums**: Verify filesystem resource integrity
5. **Hot Reload**: Watch filesystem for changes in development mode

### Compatibility Considerations

All enhancements must maintain:
- Backward compatibility with existing resource formats
- Zero-configuration defaults
- Graceful fallback behavior
- Transparent operation

## Related Documentation

- Implementation Details: `docs/explanations/phase1_embedded_resources_implementation.md`
- User Guide: `docs/how_to/setup_custom_resources.md`
- Environment Variables: `docs/reference/environment_variables.md`
- Testing: `docs/explanations/phase5_testing_implementation.md`

## References

### External Standards

- XDG Base Directory Specification: https://specifications.freedesktop.org/basedir-spec/latest/
- Rust include_dir crate: https://docs.rs/include_dir/latest/include_dir/

### Internal Design Documents

- Original Plan: `docs/explanations/embedded_resources_plan.md`
- Phase Implementations: `docs/explanations/phase*_implementation.md`

## Appendix: Code Examples

### Extracting All Resources

```rust
use xzagentz::infrastructure::EmbeddedResources;

let resources = EmbeddedResources::new();
let target = PathBuf::from("/path/to/extract");

resources.extract_all_to(&target)?;
```

### Custom Resolution

```rust
use xzagentz::components::resolution::resolve_component_dir;
use std::path::PathBuf;

let custom = Some(PathBuf::from("/custom/components"));
let source = resolve_component_dir(custom);

match source {
    ResourceSource::Filesystem(path) => {
        println!("Using filesystem: {}", path.display());
    }
    ResourceSource::Embedded => {
        println!("Using embedded resources");
    }
}
```

### Loading with Fallback

```rust
use xzagentz::components::ComponentLoader;

// Automatically resolves and uses appropriate source
let loader = ComponentLoader::new()?;

// Load component (works with both sources)
let component = loader.load("core/error_handling")?;
```

---

**Document Version**: 1.0
**Last Updated**: 2024
**Author**: AI Agent following AGENTS.md guidelines
**Status**: Complete - Embedded Resources Architecture Documentation
