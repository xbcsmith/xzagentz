# Learning Resources and References

This section provides curated learning resources, references, and guides for AI agents working on this project.

---

## Official Documentation

### Rust Language
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rust Standard Library**: https://doc.rust-lang.org/std/
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/

### Key Crates Used in This Project
- **clap**: https://docs.rs/clap/ (CLI argument parsing)
- **serde**: https://serde.rs/ (Serialization/deserialization)
- **toml**: https://docs.rs/toml/ (TOML parsing)
- **thiserror**: https://docs.rs/thiserror/ (Error handling)
- **anyhow**: https://docs.rs/anyhow/ (Error context)

---

## Project-Specific Resources

### Architecture Documentation
- `docs/explanations/architecture.md` - System architecture overview
- `docs/explanations/implementation_plan.md` - Detailed implementation roadmap
- `docs/reference/component_format.md` - Component file structure specification

### Development Guides
- `docs/how_to/setup_development_environment.md` - Local setup instructions
- `docs/how_to/running_tests.md` - Testing workflow
- `docs/how_to/debugging_guide.md` - Debugging tips and tools

### Templates and Examples
- `templates/` - Available templates for project generation
- `components/` - Reusable component files
- `examples/` - Example usage and workflows

---

## Best Practices

### Rust Idioms and Patterns

**Error Handling**
- Prefer `Result<T, E>` over panics for recoverable errors
- Use `?` operator for error propagation
- Provide context with error messages
- Use `thiserror` for custom error types

**Ownership and Borrowing**
- Follow borrowing rules strictly
- Use references (`&T`) when you don't need ownership
- Use `Clone` judiciously (understand the cost)
- Prefer `&str` over `String` for function parameters

**Collections**
- Use `Vec<T>` for dynamic arrays
- Use `HashMap<K, V>` for key-value storage
- Prefer iterators over index-based loops
- Use `collect()` to build collections from iterators

**Pattern Matching**
- Use `match` for exhaustive pattern matching
- Use `if let` for single-pattern matching
- Leverage destructuring in patterns
- Use `_` for ignored values

### Testing Patterns

**Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange - Set up test data
        let input = create_test_input();

        // Act - Execute the function
        let result = function_under_test(input);

        // Assert - Verify expectations
        assert_eq!(result, expected_value);
    }
}
```

**Integration Tests**
- Place in `tests/` directory
- Test public API surfaces
- Simulate real-world usage
- Verify cross-module interactions

**Test Coverage Goals**
- Minimum 80% code coverage
- 100% coverage for critical paths
- Test both happy and error paths
- Include edge cases and boundaries

### Documentation Patterns

**Module-Level Documentation**
```rust
//! Module description
//!
//! This module provides functionality for...
//!
//! # Examples
//!
//! ```
//! use crate::module::Item;
//!
//! let item = Item::new();
//! ```
```

**Function Documentation**
```rust
/// Brief one-line description
///
/// More detailed explanation of what the function does,
/// including any important behavior or side effects.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Returns `ErrorType` when condition occurs
///
/// # Examples
///
/// ```
/// use crate::module::function;
///
/// let result = function(arg)?;
/// ```
pub fn function(param: Type) -> Result<Type, Error> {
    // Implementation
}
```

---

## Common Patterns in This Project

### Configuration Loading
```rust
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    field: String,
}

fn load_config(path: &str) -> Result<Config, Error> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
```

### CLI Command Pattern
```rust
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct CommandArgs {
    #[arg(short, long)]
    pub option: String,
}

pub fn execute(args: CommandArgs) -> Result<()> {
    // Implementation
    Ok(())
}
```

### Component Loading Pattern
```rust
fn load_component(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)
        .map_err(|e| Error::FileIo {
            path: path.to_path_buf(),
            source: e,
        })?;

    validate_component(&content)?;
    Ok(content)
}
```

---

## Troubleshooting Guide

### Common Compilation Errors

**Borrow Checker Issues**
- Error: "cannot borrow as mutable"
- Solution: Reduce scope of mutable borrows, use `clone()` if needed

**Lifetime Issues**
- Error: "lifetime may not live long enough"
- Solution: Add explicit lifetime annotations or restructure ownership

**Type Mismatches**
- Error: "expected X, found Y"
- Solution: Check type signatures, use `.into()` or explicit conversions

### Common Test Failures

**File Not Found in Tests**
- Use `tempfile` crate for temporary test files
- Ensure test data files are in correct location
- Use relative paths from project root

**Flaky Tests**
- Avoid depending on timing
- Use deterministic test data
- Isolate tests from each other

### Clippy Warnings

**field_reassign_with_default**
- Use struct literal initialization with `..Default::default()`

**needless_return**
- Remove explicit `return` at end of function

**redundant_pattern_matching**
- Use `is_ok()` or `is_err()` instead of `matches!`

---

## Quick Reference Commands

### Development Workflow
```bash
# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Run specific test
cargo test test_name -- --nocapture

# Generate documentation
cargo doc --open
```

### Git Workflow
```bash
# Create feature branch
git checkout -b pr-PROJ-1234

# Stage changes
git add -A

# Commit with conventional format
git commit -m "feat(module): add feature (PROJ-1234)"

# Push to remote
git push origin pr-PROJ-1234
```

---

## Additional Resources

### Rust Community
- **Rust Users Forum**: https://users.rust-lang.org/
- **Rust Reddit**: https://www.reddit.com/r/rust/
- **This Week in Rust**: https://this-week-in-rust.org/

### Learning Materials
- **Rust Design Patterns**: https://rust-unofficial.github.io/patterns/
- **Rust Cookbook**: https://rust-lang-nursery.github.io/rust-cookbook/
- **Awesome Rust**: https://github.com/rust-unofficial/awesome-rust

### Tools
- **cargo-edit**: Add/remove dependencies from CLI
- **cargo-watch**: Auto-recompile on file changes
- **cargo-audit**: Security vulnerability scanning
- **cargo-outdated**: Check for dependency updates

---

## Getting Help

When stuck:

1. **Check Documentation**: Review relevant docs in `docs/` directory
2. **Search Existing Code**: Use `grep` to find similar patterns
3. **Run Tests**: Execute tests to understand expected behavior
4. **Read Error Messages**: Rust errors are usually very helpful
5. **Consult References**: Use official Rust documentation
6. **Review This File**: Check if your question is answered here

Remember: Understanding is better than copy-pasting. Take time to learn the patterns.
