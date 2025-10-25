# Development Guidelines

This section provides general development guidelines and workflows for this project.

---

## Development Environment Setup

### Required Tools

- **Rust**: Latest stable version (1.70+)
- **Git**: Version control (2.30+)
- **Text Editor/IDE**: VS Code, RustRover, or vim
- **Make**: Build automation (optional)

### Recommended Tools

- **cargo-watch**: Auto-rebuild on file changes
- **cargo-edit**: Manage dependencies from CLI
- **cargo-audit**: Security vulnerability scanning
- **cargo-outdated**: Check for dependency updates

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional components
rustup component add clippy rustfmt

# Install recommended tools
cargo install cargo-watch cargo-edit cargo-audit cargo-outdated
```

---

## Development Workflow

### Daily Workflow

1. **Pull Latest Changes**
   ```bash
   git checkout main
   git pull origin main
   ```

2. **Create Feature Branch**
   ```bash
   git checkout -b pr-proj-1234
   ```

3. **Develop with Fast Feedback**
   ```bash
   # Terminal 1: Auto-rebuild on changes
   cargo watch -x check -x test

   # Terminal 2: Make changes
   vim src/main.rs
   ```

4. **Run Quality Checks**
   ```bash
   make check  # Or run commands individually
   ```

5. **Commit Changes**
   ```bash
   git add -A
   git commit -m "feat(module): add feature (PROJ-1234)"
   ```

6. **Push to Remote**
   ```bash
   git push origin pr-proj-1234
   ```

### Before Creating Pull Request

**MUST pass all quality gates:**

```bash
# 1. Format
cargo fmt --all

# 2. Compile
cargo check --all-targets --all-features

# 3. Lint (zero warnings required)
cargo clippy --all-targets --all-features -- -D warnings

# 4. Test (>80% coverage required)
cargo test --all-features

# 5. Documentation
cargo doc --no-deps
```

---

## Code Organization

### Module Structure

```text
src/
├── main.rs              # Binary entry point
├── lib.rs               # Library root
├── cli/
│   ├── mod.rs           # CLI module root
│   ├── create.rs        # Create command
│   └── validate.rs      # Validate command
├── core/
│   ├── mod.rs           # Core module root
│   ├── component.rs     # Component types
│   └── template.rs      # Template types
└── error.rs             # Error definitions
```

### File Organization

**Each module file should contain:**
1. Module documentation
2. Imports (grouped: std, external, internal)
3. Type definitions
4. Implementation blocks
5. Private helper functions
6. Tests (in `#[cfg(test)]` module)

**Example:**
```rust
//! Module documentation
//!
//! Detailed description of what this module does.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Public struct documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    name: String,
}

impl Component {
    /// Constructor documentation
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let comp = Component::new("test".to_string());
        assert_eq!(comp.name, "test");
    }
}
```

---

## Debugging

### Debug Printing

```rust
// Quick debug
dbg!(&variable);

// Debug with context
println!("Debug: {:?}", variable);

// Pretty print
println!("{:#?}", complex_struct);
```

### Debug Logging

```rust
use log::{debug, info, warn, error};

fn process() {
    debug!("Starting process");
    info!("Processing item: {}", item);
    warn!("Potential issue detected");
    error!("Process failed: {}", err);
}
```

### Running with Debug Output

```bash
# Set log level
RUST_LOG=debug cargo run

# Specific module
RUST_LOG=myproject::module=debug cargo run

# With backtrace
RUST_BACKTRACE=1 cargo run
```

### Using Debugger

**VS Code (CodeLLDB extension):**
1. Set breakpoints in editor
2. Press F5 to start debugging
3. Use debug console for evaluation

**Command Line (rust-lldb):**
```bash
rust-lldb target/debug/myapp
(lldb) b main.rs:42
(lldb) run
(lldb) p variable
```

---

## Testing Strategy

### Test Pyramid

1. **Unit Tests** (70%): Test individual functions
2. **Integration Tests** (20%): Test module interactions
3. **End-to-End Tests** (10%): Test complete workflows

### Writing Tests

**Unit Test Example:**
```rust
#[test]
fn test_parse_valid_input() {
    let input = "valid input";
    let result = parse(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}

#[test]
fn test_parse_invalid_input() {
    let result = parse("");
    assert!(result.is_err());
}
```

**Integration Test Example:**
```rust
// tests/integration_test.rs
use myproject::Component;

#[test]
fn test_component_workflow() {
    let component = Component::new("test");
    component.validate().unwrap();
    component.save("output.txt").unwrap();

    let loaded = Component::load("output.txt").unwrap();
    assert_eq!(component, loaded);
}
```

### Test Organization

- Place unit tests in same file as code
- Place integration tests in `tests/` directory
- Use `#[cfg(test)]` for test modules
- Use descriptive test names
- Test both success and failure cases

---

## Performance Optimization

### Profiling

```bash
# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph

# Memory profiling with valgrind
valgrind --tool=massif target/release/myapp

# Benchmarking
cargo bench
```

### Common Optimizations

**Avoid unnecessary cloning:**
```rust
// Bad
fn process(data: String) -> String {
    data.to_uppercase()
}

// Good
fn process(data: &str) -> String {
    data.to_uppercase()
}
```

**Use iterators instead of collecting:**
```rust
// Less efficient
let filtered: Vec<_> = items.iter().filter(|x| x.is_valid()).collect();
let result = filtered.len();

// More efficient
let result = items.iter().filter(|x| x.is_valid()).count();
```

**Reuse allocations:**
```rust
let mut buffer = String::new();
for item in items {
    buffer.clear();
    write!(&mut buffer, "{}", item).unwrap();
    process(&buffer);
}
```

---

## Dependency Management

### Adding Dependencies

```bash
# Add crate
cargo add serde

# Add with features
cargo add serde --features derive

# Add dev dependency
cargo add --dev tempfile
```

### Updating Dependencies

```bash
# Update dependencies
cargo update

# Check for outdated
cargo outdated

# Audit for vulnerabilities
cargo audit
```

### Version Pinning

```toml
[dependencies]
# Exact version
serde = "=1.0.150"

# Compatible updates (recommended)
serde = "1.0"

# Any version (avoid)
serde = "*"
```

---

## Code Review Checklist

### Before Requesting Review

- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] No debug code or commented code
- [ ] Error handling is comprehensive
- [ ] Edge cases are tested

### Reviewing Code

**Check for:**
- Correctness and logic errors
- Error handling completeness
- Test coverage adequacy
- Documentation clarity
- Performance considerations
- Security implications
- Code duplication
- Naming consistency

---

## Common Issues and Solutions

### Compilation Errors

**Issue**: "cannot borrow as mutable"
```rust
// Solution: Reduce borrow scope or use clone
let value = data.clone();
process(&mut value);
```

**Issue**: "lifetime may not live long enough"
```rust
// Solution: Add explicit lifetime or restructure
fn process<'a>(data: &'a str) -> &'a str {
    // Implementation
}
```

### Test Failures

**Issue**: Tests pass individually but fail together
- **Solution**: Tests may have shared state; use isolation

**Issue**: Flaky tests
- **Solution**: Remove timing dependencies, use deterministic data

### Performance Issues

**Issue**: Slow compilation
- **Solution**: Use `cargo check` for fast feedback
- **Solution**: Enable incremental compilation
- **Solution**: Split large modules

---

## Continuous Improvement

### Regular Tasks

**Weekly:**
- Update dependencies: `cargo update`
- Review TODO comments
- Refactor complex functions

**Monthly:**
- Audit dependencies: `cargo audit`
- Review documentation
- Update changelog

**Per Release:**
- Run full test suite
- Update version numbers
- Tag release in git
- Update documentation

---

## Additional Resources

- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **Rust Patterns**: https://rust-unofficial.github.io/patterns/
- **Rust Performance Book**: https://nnethercote.github.io/perf-book/
