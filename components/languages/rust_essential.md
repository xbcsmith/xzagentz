---
component:
    name: rust_essential
    category: languages
    version: "1.0"
    description: Essential Rust guidelines and critical rules (concise version)
    tier: essential
    languages:
        - rust
---

# Rust Development Guidelines (Essential)

## 1. Critical Rules

### File Extensions
- Use `.rs` for all Rust source files
- Use `.toml` for configuration files (NOT `.tml`)

### Naming Conventions
- Types/Traits: `PascalCase` (e.g., `ComponentLoader`, `ConfigError`)
- Functions/Variables: `snake_case` (e.g., `load_config`, `parse_template`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRIES`, `DEFAULT_TIMEOUT`)
- Modules: `snake_case` (e.g., `mod component_loader;`)

### Quality Gates (MUST ALL PASS)
```bash
cargo fmt --all                                      # Format code
cargo check --all-targets --all-features             # Compile check
cargo clippy --all-targets --all-features -- -D warnings  # Lint (zero warnings)
cargo test --all-features                            # Run tests (>80% coverage)
```

## 2. Error Handling (MANDATORY)

### Use Result and thiserror
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to read file: {path}")]
    FileRead { path: String },
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### Error Propagation
```rust
pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;  // Use ? operator
    let config = serde_yaml::from_str(&content)?;
    Ok(config)
}
```

### NEVER Use unwrap/expect Without Justification
```rust
// BAD - will panic on error
let config = load_config().unwrap();

// GOOD - handle errors properly
let config = load_config().map_err(|e| /* handle */)?;

// ACCEPTABLE - with clear justification
// SAFETY: This environment variable is set at compile time
let version = env!("CARGO_PKG_VERSION");
```

## 3. Testing Requirements

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_with_valid_input() {
        let result = my_function("valid");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_function_with_invalid_input() {
        let result = my_function("");
        assert!(result.is_err());
    }

    #[test]
    fn test_function_edge_case() {
        let result = my_function("boundary_value");
        assert!(matches!(result, Ok(_)));
    }
}
```

### Test Requirements
- Write tests for ALL public functions
- Test success cases, failure cases, and edge cases
- Achieve >80% code coverage
- Use descriptive names: `test_{function}_{condition}_{expected}`

## 4. Documentation Standards

### Function Documentation
```rust
/// Loads configuration from the specified path
///
/// # Arguments
///
/// * `path` - Path to the configuration file
///
/// # Returns
///
/// Returns `Ok(Config)` on success, or error if file cannot be read
///
/// # Errors
///
/// Returns `AppError::FileRead` if file does not exist
/// Returns `AppError::Parse` if YAML is invalid
///
/// # Examples
///
/// ```
/// use myapp::config::load_config;
/// 
/// let config = load_config(Path::new("config.yaml"))?;
/// ```
pub fn load_config(path: &Path) -> Result<Config> {
    // Implementation
}
```

## 5. Common Patterns

### Prefer Borrowing
```rust
// GOOD - accepts borrowed data
fn validate(config: &Config) -> Result<()> { /* ... */ }

// LESS IDEAL - takes ownership unnecessarily
fn validate(config: Config) -> Result<()> { /* ... */ }
```

### Use Iterators
```rust
// PREFERRED
let names: Vec<_> = items.iter()
    .filter(|i| i.is_valid())
    .map(|i| i.name.clone())
    .collect();

// LESS IDIOMATIC
let mut names = Vec::new();
for item in &items {
    if item.is_valid() {
        names.push(item.name.clone());
    }
}
```

### Derive Common Traits
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Component {
    name: String,
    content: String,
}
```

## 6. Quick Command Reference

```bash
# Development
cargo build                    # Debug build
cargo build --release          # Release build
cargo check                    # Fast compile check

# Quality
cargo fmt --all               # Format code
cargo clippy -- -D warnings   # Lint (treat warnings as errors)

# Testing
cargo test                    # Run all tests
cargo test --lib             # Library tests only
cargo test test_name         # Specific test
cargo test -- --nocapture    # Show println output

# Documentation
cargo doc --open             # Generate and open docs
```

## 7. Essential Clippy Fixes

```rust
// AVOID needless_return
fn get_name() -> String {
    "name".to_string()  // No return keyword needed
}

// AVOID field_reassign_with_default
let config = Config {
    name: "test".to_string(),
    ..Default::default()
};

// USE ? operator for error propagation
fn process() -> Result<()> {
    let data = load_data()?;  // Not .unwrap()
    validate(&data)?;
    Ok(())
}
```

## 8. Validation Workflow

```text
1. Write code with /// doc comments
2. Add tests (>80% coverage)
3. Run: cargo fmt --all
4. Run: cargo check --all-targets --all-features
5. Run: cargo clippy --all-targets --all-features -- -D warnings
6. Run: cargo test --all-features
7. All checks MUST pass before committing
```
