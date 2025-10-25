# Documentation Guidelines

This section provides documentation best practices and standards for this project.

---

## Documentation Philosophy

**Core Principles:**
- Documentation is code - treat it with same care
- Write for your future self and others
- Keep documentation close to code
- Update docs when code changes
- Examples are worth a thousand words
- Make documentation discoverable

---

## Documentation Types

### Code Documentation

**Doc Comments** - Inline documentation using `///` or `//!`

```rust
/// Loads a component from the specified file path
///
/// This function reads the component file, validates its structure,
/// and returns a parsed Component instance.
///
/// # Arguments
///
/// * `path` - Path to the component file
///
/// # Returns
///
/// Returns `Ok(Component)` on success, or an error if:
/// - File doesn't exist
/// - File format is invalid
/// - Validation fails
///
/// # Errors
///
/// - `Error::NotFound` - File not found at path
/// - `Error::ParseError` - Invalid file format
/// - `Error::ValidationError` - Component validation failed
///
/// # Examples
///
/// ```
/// use xzagentz::component::load_component;
/// use std::path::Path;
///
/// let component = load_component(Path::new("components/header.md"))?;
/// assert!(component.is_valid());
/// ```
///
/// # Panics
///
/// This function does not panic under normal circumstances.
pub fn load_component(path: &Path) -> Result<Component> {
    // Implementation
}
```

### Module Documentation

**Module-level docs** using `//!`

```rust
//! Component loading and validation module
//!
//! This module provides functionality for loading component files from
//! the filesystem, validating their structure, and caching results.
//!
//! # Overview
//!
//! Components are markdown files with specific structure requirements.
//! The loader validates format, extracts metadata, and manages caching.
//!
//! # Examples
//!
//! ```
//! use xzagentz::component::ComponentLoader;
//!
//! let loader = ComponentLoader::new();
//! let component = loader.load("components/core/header.md")?;
//! ```
//!
//! # Architecture
//!
//! - `ComponentLoader` - Main loading interface
//! - `ComponentValidator` - Structure validation
//! - `ComponentCache` - Result caching
```

### README Documentation

**Project README.md** - Project overview and quick start

**Required Sections:**
- Project title and description
- Installation instructions
- Quick start / usage examples
- Links to detailed documentation
- License and contribution info

### Reference Documentation

**API Reference** - Generated from doc comments

```bash
# Generate documentation
cargo doc --no-deps --open
```

**Manual Reference** - `docs/reference/`
- API specification
- Configuration reference
- CLI command reference
- Error codes reference

### Guides and Tutorials

**How-To Guides** - `docs/how_to/`
- Task-oriented instructions
- Step-by-step procedures
- Problem-solving recipes

**Tutorials** - `docs/tutorials/`
- Learning-oriented lessons
- Complete workflows
- Hands-on examples

**Explanations** - `docs/explanations/`
- Understanding-oriented content
- Architecture discussions
- Design decisions
- Implementation summaries

---

## Doc Comment Standards

### Required Sections

**Minimum requirements for public items:**

1. **Brief description** (one line)
2. **Arguments section** (`# Arguments`) for functions with parameters
3. **Returns section** (`# Returns`) for functions that return values
4. **Errors section** (`# Errors`) for fallible functions
5. **Examples section** (`# Examples`) with runnable code

**Optional but recommended:**

- `# Panics` - When function can panic
- `# Safety` - For unsafe code
- `# Examples` - Multiple examples for complex APIs
- Extended description after brief

### Section Order

```rust
/// Brief one-line description
///
/// Extended description with more details about behavior,
/// usage patterns, and important notes.
///
/// # Arguments
///
/// * `param1` - Description
/// * `param2` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When and why errors are returned
///
/// # Panics
///
/// When and why panics occur
///
/// # Safety
///
/// Safety invariants for unsafe code
///
/// # Examples
///
/// Basic usage:
/// ```
/// // Example code
/// ```
///
/// Advanced usage:
/// ```
/// // Example code
/// ```
```

---

## Writing Effective Examples

### Runnable Examples

**Examples must compile and run:**

```rust
/// # Examples
///
/// ```
/// use xzagentz::Component;
///
/// let comp = Component::new("test");
/// assert_eq!(comp.name(), "test");
/// ```
```

### Hide Setup Code

**Use `#` prefix to hide lines:**

```rust
/// # Examples
///
/// ```
/// # use xzagentz::Component;
/// # use std::path::Path;
/// #
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let comp = Component::load(Path::new("test.md"))?;
/// assert!(comp.is_valid());
/// # Ok(())
/// # }
/// ```
```

### No-Run Examples

**For examples that shouldn't execute:**

```rust
/// # Examples
///
/// ```no_run
/// let server = start_server()?; // Won't actually start
/// server.listen();
/// ```
```

### Ignore Examples

**For pseudo-code or broken examples:**

```rust
/// # Examples
///
/// ```ignore
/// // Conceptual example, won't compile
/// component.do_something();
/// ```
```

---

## Markdown Documentation

### File Naming

**MANDATORY:**
- Lowercase with underscores: `implementation_plan.md`
- Exception: `README.md` only

### Code Blocks

**Always specify language:**

```markdown
```rust
fn main() {
    println!("Hello!");
}
```
```

### Internal Links

```markdown
See the [Installation](#installation) section.

See the [API Reference](./reference/api.md).
```

### External Links

```markdown
[Rust Book](https://doc.rust-lang.org/book/)

[Project Repository][repo]

[repo]: https://github.com/user/project
```

---

## Documentation Organization

### Diataxis Framework

**Tutorials** - Learning-oriented
```
docs/tutorials/
├── getting_started.md
├── first_component.md
└── advanced_workflows.md
```

**How-To Guides** - Problem-oriented
```
docs/how_to/
├── setup_environment.md
├── add_component.md
└── troubleshooting.md
```

**Explanations** - Understanding-oriented
```
docs/explanations/
├── architecture.md
├── design_decisions.md
└── implementation_plan.md
```

**Reference** - Information-oriented
```
docs/reference/
├── api_specification.md
├── cli_reference.md
└── configuration.md
```

---

## Keeping Documentation Updated

### When to Update

**Update documentation when:**
- Adding new features
- Changing public APIs
- Fixing bugs that affect behavior
- Updating dependencies with breaking changes
- Making architectural changes

### Documentation in PRs

**Pull request checklist:**
- [ ] Doc comments updated
- [ ] README updated (if needed)
- [ ] Changelog updated
- [ ] Migration guide (if breaking)
- [ ] Examples updated

### Version Documentation

**Track changes:**
```markdown
# Changelog

## [Unreleased]

### Added
- New feature X

### Changed
- Updated Y to improve Z

### Fixed
- Bug in feature A

## [1.0.0] - 2024-01-01

Initial release
```

---

## Documentation Testing

### Doc Test Examples

```rust
/// Parse configuration file
///
/// # Examples
///
/// ```
/// use xzagentz::config::parse;
///
/// let config = parse("key = value")?;
/// assert_eq!(config.get("key"), Some("value"));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse(input: &str) -> Result<Config> {
    // Implementation
}
```

### Running Doc Tests

```bash
# Run all doc tests
cargo test --doc

# Run specific module's doc tests
cargo test --doc module_name
```

---

## Style Guidelines

### Tone and Voice

- Use clear, concise language
- Write in present tense
- Use active voice
- Be direct and specific
- Avoid jargon when possible
- Define acronyms on first use

### Formatting

- One sentence per line (for source control)
- Use bullet points for lists
- Use tables for comparisons
- Use code blocks for code
- Use blockquotes for notes

### Examples

**Good:**
```markdown
The loader validates component structure. It checks format,
extracts metadata, and reports errors.
```

**Bad:**
```markdown
The loader is responsible for the validation of the component
structure, and it will check the format and extract metadata,
and it also reports any errors that might occur.
```

---

## Common Documentation Patterns

### Error Documentation

```rust
/// # Errors
///
/// Returns `Error::NotFound` if the file doesn't exist.
///
/// Returns `Error::ParseError` if the file format is invalid:
/// - Missing required headers
/// - Invalid YAML frontmatter
/// - Malformed code blocks
///
/// Returns `Error::ValidationError` if validation fails:
/// - Empty component name
/// - Invalid file extension
```

### Configuration Documentation

```rust
/// Configuration options for the component loader
///
/// # Examples
///
/// Default configuration:
/// ```
/// use xzagentz::config::LoaderConfig;
///
/// let config = LoaderConfig::default();
/// ```
///
/// Custom configuration:
/// ```
/// use xzagentz::config::LoaderConfig;
///
/// let config = LoaderConfig::new()
///     .with_cache(true)
///     .with_validate(true);
/// ```
pub struct LoaderConfig {
    /// Enable component caching
    pub cache_enabled: bool,
    /// Validate on load
    pub validate: bool,
}
```

---

## Tools and Automation

### Documentation Generation

```bash
# Generate HTML docs
cargo doc --no-deps

# Open in browser
cargo doc --no-deps --open

# Include private items
cargo doc --document-private-items
```

### Link Checking

```bash
# Install markdown link checker
cargo install markdown-link-check

# Check links
markdown-link-check docs/**/*.md
```

### Spell Checking

```bash
# Install codespell
pip install codespell

# Check spelling
codespell docs/
```

---

## Documentation Checklist

### For New Features

- [ ] Public API has doc comments
- [ ] Doc comments include examples
- [ ] Examples compile and run
- [ ] Module documentation updated
- [ ] README updated (if user-facing)
- [ ] Changelog entry added
- [ ] How-to guide created (if complex)

### For Bug Fixes

- [ ] Doc comments accurate
- [ ] Examples still work
- [ ] Known issues updated
- [ ] Changelog entry added

### Before Release

- [ ] All doc tests pass
- [ ] README is current
- [ ] Changelog is complete
- [ ] Migration guide exists (if breaking)
- [ ] Version numbers updated
- [ ] Links are valid

---

## Additional Resources

- **Rust Doc Book**: https://doc.rust-lang.org/rustdoc/
- **RFC 1574**: https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html
- **Diataxis**: https://diataxis.fr/
- **Write the Docs**: https://www.writethedocs.org/
