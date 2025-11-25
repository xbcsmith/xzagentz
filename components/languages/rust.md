---
component:
    name: rust
    category: languages
    version: "1.0"
    description: Rust-specific development guidelines, patterns, and best practices
    languages:
        - rust
---

# Rust Language Guidelines

This section provides Rust-specific guidelines, patterns, and best practices for this project.

---

## Rust Version

**Required**: Rust stable (latest)

Check your version:
```bash
rustc --version
cargo --version
```

Update Rust:
```bash
rustup update stable
```

---

## Code Style and Formatting

### Formatting Rules

**ALWAYS use rustfmt:**
```bash
cargo fmt --all
```

**Standard settings** (enforced by rustfmt.toml):
- Indentation: 4 spaces
- Max line width: 100 characters
- Use trailing commas in multi-line expressions
- Import grouping: std, external crates, internal modules

### Naming Conventions

**Types and Traits**
- Use `PascalCase` for types, traits, and enum variants
- Examples: `ConfigLoader`, `PromptGenerator`, `Error`

**Functions and Variables**
- Use `snake_case` for functions, methods, and variables
- Examples: `load_config`, `parse_template`, `user_input`

**Constants**
- Use `SCREAMING_SNAKE_CASE` for constants
- Examples: `MAX_RETRIES`, `DEFAULT_TIMEOUT`, `VERSION`

**Modules**
- Use `snake_case` for module names
- Examples: `mod component_loader;`, `mod error_handler;`

---

## Error Handling

### Using Result and thiserror

**Define custom error types:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Component not found: {path}")]
    NotFound { path: String },

    #[error("Failed to parse component: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },
}

pub type Result<T> = std::result::Result<T, ComponentError>;
```

### Error Propagation

**Use the ? operator:**
```rust
pub fn load_and_parse(path: &Path) -> Result<Component> {
    let content = std::fs::read_to_string(path)?;
    let component = parse_component(&content)?;
    component.validate()?;
    Ok(component)
}
```

### Error Context

**Add context with map_err:**
```rust
let config = std::fs::read_to_string(path)
    .map_err(|e| Error::FileIo {
        path: path.to_path_buf(),
        source: e,
    })?;
```

---

## Ownership and Borrowing

### Prefer Borrowing

**Use references when you don't need ownership:**
```rust
// Good - accepts borrowed data
fn validate_config(config: &Config) -> Result<()> {
    // Validation logic
    Ok(())
}

// Less ideal - takes ownership unnecessarily
fn validate_config(config: Config) -> Result<()> {
    // Validation logic
    Ok(())
}
```

### String Slices vs String

**Function parameters should prefer &str:**
```rust
// Preferred - more flexible
fn create_component(name: &str) -> Component {
    Component {
        name: name.to_string(),
    }
}

// Less flexible - requires String
fn create_component(name: String) -> Component {
    Component { name }
}
```

### Clone Judiciously

**Only clone when necessary:**
```rust
// Avoid unnecessary clones
let name = config.name.clone(); // Only if you need ownership

// Better - use references
let name = &config.name;
```

---

## Type Safety

### Use NewType Pattern

**Wrap primitive types for type safety:**
```rust
pub struct ComponentName(String);

impl ComponentName {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidName);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### Use Enums for States

**Model states explicitly:**
```rust
pub enum LoadState {
    NotLoaded,
    Loading { progress: f64 },
    Loaded { data: Vec<u8> },
    Failed { error: String },
}
```

---

## Pattern Matching

### Exhaustive Matching

**Always handle all cases:**
```rust
match result {
    Ok(value) => process(value),
    Err(e) => handle_error(e),
}
```

### Use if let for Single Cases

**Cleaner than match for single patterns:**
```rust
// Good
if let Some(config) = load_config()? {
    use_config(config);
}

// Verbose
match load_config()? {
    Some(config) => use_config(config),
    None => {}
}
```

---

## Collections and Iterators

### Prefer Iterators

**Use iterator chains instead of loops:**
```rust
// Preferred
let valid_components: Vec<_> = components
    .iter()
    .filter(|c| c.is_valid())
    .map(|c| c.name.clone())
    .collect();

// Less idiomatic
let mut valid_components = Vec::new();
for component in &components {
    if component.is_valid() {
        valid_components.push(component.name.clone());
    }
}
```

### Common Iterator Patterns

```rust
// Transform and collect
let names: Vec<String> = items.iter().map(|i| i.name.clone()).collect();

// Filter and find first
let first_valid = items.iter().find(|i| i.is_valid());

// Check if any/all match
let has_errors = items.iter().any(|i| i.has_error());
let all_valid = items.iter().all(|i| i.is_valid());

// Partition into two groups
let (valid, invalid): (Vec<_>, Vec<_>) = items
    .into_iter()
    .partition(|i| i.is_valid());
```

---

## Trait Implementation

### Derive When Possible

**Use derive for common traits:**
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Component {
    name: String,
    content: String,
}
```

### Implement Display for User-Facing Types

```rust
use std::fmt;

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentError::NotFound { path } => {
                write!(f, "Component not found at: {}", path)
            }
            ComponentError::ParseError(msg) => {
                write!(f, "Parse error: {}", msg)
            }
        }
    }
}
```

---

## Async/Await (If Applicable)

### Use async When I/O Bound

```rust
use tokio::fs;

pub async fn load_component_async(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path).await
        .map_err(|e| Error::FileIo {
            path: path.to_path_buf(),
            source: e,
        })?;
    Ok(content)
}
```

---

## Testing

### Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component = Component::new("test");
        assert_eq!(component.name(), "test");
    }

    #[test]
    fn test_validation_rejects_empty_name() {
        let result = Component::new("");
        assert!(result.is_err());
    }
}
```

### Integration Tests

Place in `tests/` directory:
```rust
use xzagentz::component::ComponentLoader;

#[test]
fn test_load_component_from_file() {
    let loader = ComponentLoader::new();
    let component = loader.load("path/to/component.md").unwrap();
    assert!(!component.content.is_empty());
}
```

### Test Helpers

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;

    pub fn create_test_component() -> Component {
        Component {
            name: "test".to_string(),
            content: "test content".to_string(),
        }
    }
}
```

---

## Documentation

### Module Documentation

```rust
//! Component loading and validation module
//!
//! This module provides functionality for loading component files
//! from the filesystem and validating their structure.
//!
//! # Examples
//!
//! ```
//! use xzagentz::component::ComponentLoader;
//!
//! let loader = ComponentLoader::new();
//! let component = loader.load("components/core/header.md")?;
//! ```
```

### Function Documentation

```rust
/// Loads a component from the specified path
///
/// # Arguments
///
/// * `path` - Path to the component file
///
/// # Returns
///
/// Returns `Ok(Component)` if successful, or an error if the file
/// cannot be read or parsed.
///
/// # Errors
///
/// Returns `ComponentError::NotFound` if the file doesn't exist.
/// Returns `ComponentError::ParseError` if the file format is invalid.
///
/// # Examples
///
/// ```
/// use xzagentz::component::load_component;
/// use std::path::Path;
///
/// let component = load_component(Path::new("component.md"))?;
/// assert!(!component.is_empty());
/// ```
pub fn load_component(path: &Path) -> Result<Component> {
    // Implementation
}
```

---

## Performance Considerations

### Avoid Unnecessary Allocations

```rust
// Good - reuse buffer
let mut buffer = String::new();
for line in lines {
    buffer.clear();
    buffer.push_str(line);
    process(&buffer);
}

// Less efficient - allocates each iteration
for line in lines {
    let buffer = line.to_string();
    process(&buffer);
}
```

### Use Cow for Borrowed or Owned Data

```rust
use std::borrow::Cow;

fn process_name(name: &str) -> Cow<str> {
    if name.contains("_") {
        Cow::Owned(name.replace("_", "-"))
    } else {
        Cow::Borrowed(name)
    }
}
```

---

## Common Patterns in This Project

### Builder Pattern

```rust
pub struct ComponentLoader {
    cache_enabled: bool,
    base_path: PathBuf,
}

impl ComponentLoader {
    pub fn new() -> Self {
        Self {
            cache_enabled: false,
            base_path: PathBuf::from("components"),
        }
    }

    pub fn with_cache(mut self) -> Self {
        self.cache_enabled = true;
        self
    }

    pub fn with_base_path(mut self, path: PathBuf) -> Self {
        self.base_path = path;
        self
    }
}

// Usage
let loader = ComponentLoader::new()
    .with_cache()
    .with_base_path(PathBuf::from("custom/path"));
```

### Visitor Pattern

```rust
pub trait ComponentVisitor {
    fn visit_component(&mut self, component: &Component);
}

impl Component {
    pub fn accept(&self, visitor: &mut dyn ComponentVisitor) {
        visitor.visit_component(self);
    }
}
```

---

## Clippy Integration

### Enable Clippy Lints

Run with warnings as errors:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Common Clippy Fixes

**field_reassign_with_default**
```rust
// Bad
let mut config = Config::default();
config.name = "test".to_string();

// Good
let config = Config {
    name: "test".to_string(),
    ..Default::default()
};
```

**needless_return**
```rust
// Bad
fn get_name() -> String {
    return "name".to_string();
}

// Good
fn get_name() -> String {
    "name".to_string()
}
```

---

## Unsafe Code

**Avoid unsafe unless absolutely necessary**

If you must use unsafe:
1. Document why it's needed
2. Document safety invariants
3. Minimize unsafe scope
4. Add comprehensive tests

```rust
/// SAFETY: Pointer is guaranteed valid by caller contract
unsafe fn read_raw(ptr: *const u8) -> u8 {
    *ptr
}
```

---

## Quick Reference

### Must-Know Cargo Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Check
cargo check                    # Fast compile check
cargo clippy                   # Lint check

# Test
cargo test                     # Run all tests
cargo test --lib              # Library tests only
cargo test test_name          # Specific test

# Documentation
cargo doc --open              # Generate and open docs
```

### Rust Toolchain Management

```bash
# Update Rust
rustup update

# Install components
rustup component add clippy rustfmt

# Check installed toolchains
rustup show
```

---

## Additional Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **Clippy Lints**: https://rust-lang.github.io/rust-clippy/
