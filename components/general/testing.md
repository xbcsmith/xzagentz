# Testing Guidelines

This section provides comprehensive testing guidelines and best practices for this project.

---

## Testing Philosophy

**Core Principles:**
- Test behavior, not implementation
- Write tests before fixing bugs
- Keep tests simple and readable
- Test one thing per test
- Make tests deterministic
- Achieve >80% code coverage

---

## Test Types

### Unit Tests

**Purpose**: Test individual functions and methods in isolation

**Location**: Same file as code, in `#[cfg(test)]` module

**Example:**
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

**Purpose**: Test module interactions and public API

**Location**: `tests/` directory

**Example:**
```rust
// tests/component_integration.rs
use xzagentz::component::ComponentLoader;
use tempfile::tempdir;

#[test]
fn test_load_and_validate_component() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("component.md");

    std::fs::write(&path, "# Component\n\nContent").unwrap();

    let loader = ComponentLoader::new();
    let component = loader.load(&path).unwrap();

    assert!(component.is_valid());
}
```

### Doc Tests

**Purpose**: Test code examples in documentation

**Location**: In doc comments

**Example:**
```rust
/// Load a component from file
///
/// # Examples
///
/// ```
/// use xzagentz::component::load_component;
/// use std::path::Path;
///
/// let component = load_component(Path::new("test.md")).unwrap();
/// assert!(!component.is_empty());
/// ```
pub fn load_component(path: &Path) -> Result<Component> {
    // Implementation
}
```

---

## Test Organization

### Test Module Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test helpers
    mod helpers {
        pub fn create_test_component() -> Component {
            Component::new("test")
        }
    }

    // Success cases
    #[test]
    fn test_valid_component() {
        let comp = helpers::create_test_component();
        assert!(comp.is_valid());
    }

    // Error cases
    #[test]
    fn test_invalid_component() {
        let comp = Component::new("");
        assert!(comp.is_err());
    }

    // Edge cases
    #[test]
    fn test_boundary_condition() {
        // Test edge cases
    }
}
```

---

## Test Naming

### Naming Convention

**Format**: `test_{function}_{condition}_{expected}`

**Examples:**
```rust
#[test]
fn test_parse_with_valid_input_returns_ok() { }

#[test]
fn test_parse_with_empty_input_returns_error() { }

#[test]
fn test_parse_with_invalid_format_returns_parse_error() { }

#[test]
fn test_component_with_max_size_succeeds() { }
```

---

## Assertions

### Basic Assertions

```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, not_expected);

// Boolean
assert!(condition);
assert!(!condition);

// Custom message
assert_eq!(actual, expected, "Expected {}, got {}", expected, actual);
```

### Result Assertions

```rust
// Check if Ok
let result = function();
assert!(result.is_ok());

// Check if Err
assert!(result.is_err());

// Unwrap and check value
assert_eq!(result.unwrap(), expected_value);

// Pattern matching
match result {
    Ok(value) => assert_eq!(value, expected),
    Err(e) => panic!("Unexpected error: {}", e),
}
```

### Error Type Assertions

```rust
use crate::error::Error;

#[test]
fn test_returns_not_found_error() {
    let result = load_component("nonexistent.md");

    assert!(result.is_err());

    match result.unwrap_err() {
        Error::NotFound { path } => {
            assert_eq!(path, "nonexistent.md");
        }
        other => panic!("Wrong error type: {:?}", other),
    }
}
```

---

## Test Fixtures

### Using Setup Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TestEnvironment {
        TestEnvironment {
            temp_dir: tempfile::tempdir().unwrap(),
        }
    }

    #[test]
    fn test_with_setup() {
        let env = setup();
        // Use env in test
    }
}
```

### Shared Test Data

```rust
#[cfg(test)]
mod test_data {
    pub const VALID_COMPONENT: &str = r#"
# Component Name

Component content here.
"#;

    pub const INVALID_COMPONENT: &str = "";
}

#[test]
fn test_parse_valid() {
    let result = parse(test_data::VALID_COMPONENT);
    assert!(result.is_ok());
}
```

---

## Testing Async Code

### Async Test Example

```rust
use tokio::test;

#[tokio::test]
async fn test_async_load() {
    let component = load_async("test.md").await.unwrap();
    assert!(!component.is_empty());
}

#[tokio::test]
async fn test_concurrent_loads() {
    let handles: Vec<_> = (0..10)
        .map(|i| tokio::spawn(async move {
            load_async(&format!("test{}.md", i)).await
        }))
        .collect();

    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }
}
```

---

## Mocking and Test Doubles

### Trait-Based Mocking

```rust
pub trait ComponentLoader {
    fn load(&self, path: &Path) -> Result<Component>;
}

#[cfg(test)]
mod tests {
    struct MockLoader {
        should_fail: bool,
    }

    impl ComponentLoader for MockLoader {
        fn load(&self, _path: &Path) -> Result<Component> {
            if self.should_fail {
                Err(Error::NotFound)
            } else {
                Ok(Component::new("test"))
            }
        }
    }

    #[test]
    fn test_with_mock() {
        let loader = MockLoader { should_fail: false };
        let result = process_component(&loader);
        assert!(result.is_ok());
    }
}
```

---

## Table-Driven Tests

### Parameterized Test Pattern

```rust
#[test]
fn test_parse_various_inputs() {
    let test_cases = vec![
        ("valid input", Ok(Expected)),
        ("", Err(Error::Empty)),
        ("invalid", Err(Error::Parse)),
    ];

    for (input, expected) in test_cases {
        let result = parse(input);
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
```

### Structured Test Cases

```rust
#[test]
fn test_validation_rules() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: bool,
    }

    let cases = vec![
        TestCase {
            name: "valid input",
            input: "valid",
            expected: true,
        },
        TestCase {
            name: "empty input",
            input: "",
            expected: false,
        },
    ];

    for case in cases {
        let result = validate(case.input);
        assert_eq!(
            result, case.expected,
            "Test '{}' failed", case.name
        );
    }
}
```

---

## Testing File I/O

### Using tempfile

```rust
use tempfile::{tempdir, NamedTempFile};

#[test]
fn test_load_from_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.md");

    std::fs::write(&file_path, "content").unwrap();

    let result = load_component(&file_path);
    assert!(result.is_ok());
}

#[test]
fn test_save_to_file() {
    let mut file = NamedTempFile::new().unwrap();
    let component = Component::new("test");

    component.save(file.path()).unwrap();

    let content = std::fs::read_to_string(file.path()).unwrap();
    assert!(content.contains("test"));
}
```

---

## Property-Based Testing

### Using proptest

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_parse_doesnt_panic(s in "\\PC*") {
        let _ = parse(&s);
    }

    #[test]
    fn test_roundtrip(name in "[a-zA-Z0-9]+") {
        let component = Component::new(&name);
        let serialized = component.serialize();
        let deserialized = Component::deserialize(&serialized).unwrap();
        assert_eq!(component, deserialized);
    }
}
```

---

## Test Coverage

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View coverage in browser
open tarpaulin-report.html
```

### Coverage Requirements

- **Minimum**: 80% overall coverage
- **Critical paths**: 100% coverage
- **Public API**: 100% coverage
- **Error paths**: 100% coverage

---

## Running Tests

### Basic Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests in module
cargo test module::tests::

# Run with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Run single-threaded
cargo test -- --test-threads=1
```

### Test Filtering

```bash
# Run tests matching pattern
cargo test parse

# Run integration tests only
cargo test --test integration_test

# Run doc tests only
cargo test --doc

# Run lib tests only
cargo test --lib
```

---

## Continuous Testing

### Watch Mode

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-run tests on file changes
cargo watch -x test

# Run tests and checks
cargo watch -x check -x test -x clippy
```

---

## Test Best Practices

### Do's

- Write tests first for bugs (TDD for fixes)
- Test edge cases and boundaries
- Use descriptive test names
- Keep tests independent
- Test one behavior per test
- Use arrange-act-assert pattern
- Clean up resources (use Drop or defer)

### Don'ts

- Don't test implementation details
- Don't make tests depend on each other
- Don't use sleep or timing in tests
- Don't skip error cases
- Don't leave commented-out tests
- Don't test third-party code

---

## Common Testing Patterns

### Arrange-Act-Assert

```rust
#[test]
fn test_component_validation() {
    // Arrange
    let component = Component::new("test");

    // Act
    let result = component.validate();

    // Assert
    assert!(result.is_ok());
}
```

### Given-When-Then

```rust
#[test]
fn test_component_lifecycle() {
    // Given a new component
    let mut component = Component::new("test");

    // When we update it
    component.update("new content");

    // Then it should be valid
    assert!(component.is_valid());
}
```

---

## Debugging Tests

### Show Output

```bash
# Show println! output
cargo test -- --nocapture

# Show test names as they run
cargo test -- --nocapture --test-threads=1
```

### Using dbg! in Tests

```rust
#[test]
fn test_debug() {
    let value = compute();
    dbg!(&value);
    assert_eq!(value, expected);
}
```

### Test-Specific Logging

```rust
#[test]
fn test_with_logging() {
    env_logger::init();
    log::debug!("Starting test");
    // Test code
}
```

---

## Performance Testing

### Benchmarks

```rust
#![feature(test)]

extern crate test;

#[bench]
fn bench_parse(b: &mut test::Bencher) {
    let input = "test input";
    b.iter(|| {
        parse(input)
    });
}
```

### Criterion Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| parse(black_box("test input")))
    });
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
```

---

## Additional Resources

- **Rust Testing Guide**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **proptest**: https://proptest-rs.github.io/proptest/
- **criterion**: https://bheisler.github.io/criterion.rs/
