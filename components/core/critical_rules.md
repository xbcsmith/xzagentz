# Critical Rules - MUST FOLLOW

**CRITICAL**: These rules are mandatory. Non-compliance will result in rejected code.

---

## Rule 1: File Naming Conventions

### Markdown Files

**YOU MUST:**
- Use lowercase letters ONLY
- Use underscores to separate words
- Use `.md` extension (NOT `.MD` or `.markdown`)
- Exception: `README.md` is the ONLY uppercase filename allowed

**Examples:**
```text
✅ CORRECT:
   docs/architecture_overview.md
   docs/how_to_setup.md
   README.md (only exception)

❌ WRONG:
   docs/Architecture-Overview.md
   docs/ArchitectureOverview.md
   docs/ARCHITECTURE.md
   docs/architecture.MD
```

### YAML Files

**YOU MUST:**
- Use `.yaml` extension (NOT `.yml`)
- Apply to ALL YAML files without exception

**Examples:**
```text
✅ CORRECT:
   config/production.yaml
   docker-compose.yaml
   .github/workflows/ci.yaml

❌ WRONG:
   config/production.yml
   docker-compose.yml
```

---

## Rule 2: Code Quality Gates

**ALL of these MUST pass before claiming task complete:**

```bash
# 1. Format code (auto-fixes issues)
cargo fmt --all

# 2. Compile check (fast verification)
cargo check --all-targets --all-features

# 3. Lint (treats warnings as errors)
cargo clippy --all-targets --all-features -- -D warnings

# 4. Tests (must pass with >80% coverage)
cargo test --all-features
```

**Expected Results:**
```text
✅ cargo fmt         → No output (all files formatted)
✅ cargo check       → "Finished" with 0 errors
✅ cargo clippy      → "Finished" with 0 warnings
✅ cargo test        → "test result: ok. X passed; 0 failed"
```

**IF ANY FAIL**: Stop immediately and fix before proceeding.

---

## Rule 3: Documentation is Mandatory

**YOU MUST:**

1. **Doc Comments for ALL Public Items**
   ```rust
   /// Brief description of function
   ///
   /// # Arguments
   ///
   /// * `param` - Description
   ///
   /// # Returns
   ///
   /// Description of return value
   ///
   /// # Errors
   ///
   /// Returns `ErrorType` if condition
   ///
   /// # Examples
   ///
   /// ```
   /// use crate::module::function;
   ///
   /// let result = function(arg);
   /// assert_eq!(result, expected);
   /// ```
   pub fn function(param: Type) -> Result<Type, Error> {
       // Implementation
   }
   ```

2. **Implementation Documentation File**
   - Create in `docs/explanations/` for EVERY feature/task
   - Use filename pattern: `{feature_name}_implementation.md`
   - Include: Overview, Components, Implementation Details, Testing, Examples

3. **Code Block Language Identifiers**
   - ALWAYS specify language in markdown code blocks
   - Use triple backticks with language name

**NEVER:**
- Skip documentation because "code is self-documenting"
- Omit language identifiers in code blocks
- Leave public APIs undocumented

---

## Rule 4: Error Handling Patterns

**YOU MUST:**

- Use `Result<T, E>` for ALL recoverable errors
- Use `?` operator for error propagation
- Use `thiserror` for custom error types
- Provide descriptive error messages

**NEVER:**
- Use `unwrap()` without justification comment
- Use `expect()` without descriptive message
- Ignore errors with `let _ =`
- Use `panic!` for recoverable errors

**Correct Pattern:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read file: {0}")]
    ReadError(String),

    #[error("Invalid syntax: {0}")]
    ParseError(String),
}

pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadError(e.to_string()))?;

    let config: Config = toml::from_str(&contents)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;

    Ok(config)
}
```

---

## Rule 5: Testing Requirements

**YOU MUST:**

- Write tests for ALL public functions
- Test both success AND failure cases
- Test edge cases and boundary conditions
- Achieve >80% code coverage
- Use descriptive test names: `test_{function}_{condition}_{expected}`

**Test Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_with_valid_input() {
        // Arrange
        let input = create_valid_input();

        // Act
        let result = function(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_function_with_invalid_input() {
        let result = function(invalid_input);
        assert!(result.is_err());
    }

    #[test]
    fn test_function_edge_case() {
        // Test boundary conditions
    }
}
```

---

## Rule 6: Git Commit Conventions

**Format:**
```text
<type>(<scope>): <description> (<JIRA-ISSUE>)

[optional body]

[optional footer]
```

**Rules:**
1. Type MUST be: `feat|fix|docs|style|refactor|perf|test|chore`
2. Description MUST be lowercase
3. Description MUST use imperative mood ("add" not "added")
4. JIRA issue MUST be uppercase
5. First line MUST be ≤72 characters

**Examples:**
```text
✅ CORRECT:
feat(auth): add JWT token refresh endpoint (PROJ-1234)
fix(api): handle edge case in validation (PROJ-5678)
docs(readme): update installation steps (PROJ-9012)

❌ WRONG:
Added JWT token (proj-1234)          # Wrong mood, no type
feat: add jwt (PROJ-1234)            # Missing scope
add jwt refresh (PROJ-1234)          # No type
```

---

## Rule 7: No Emojis

**YOU MUST:**
- Write ALL documentation without emojis
- Write ALL code comments without emojis
- Write ALL commit messages without emojis

**NEVER:**
- Use emojis in code: `// ✅ This works`
- Use emojis in docs: `## Setup Guide 🚀`
- Use emojis in commits: `feat: add auth ✨`

---

## Validation Checklist

**Before claiming task complete, verify ALL:**

- [ ] File naming follows conventions (lowercase_underscore.md, .yaml not .yml)
- [ ] `cargo fmt --all` applied successfully
- [ ] `cargo check --all-targets --all-features` passes (0 errors)
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows 0 warnings
- [ ] `cargo test --all-features` passes (>80% coverage)
- [ ] All public items have doc comments with examples
- [ ] Implementation documentation created in `docs/explanations/`
- [ ] No emojis in code, docs, or commits
- [ ] Commit message follows conventional format
- [ ] Error handling uses `Result<T, E>` and `?` operator
- [ ] Tests cover success, failure, and edge cases

---

## Emergency Quick Reference

**If you remember nothing else:**

1. **File Extensions**: `.yaml` NOT `.yml`, `.md` with lowercase_underscore
2. **Quality Gates**: All four cargo commands MUST pass (fmt, check, clippy, test)
3. **Documentation**: Create file in `docs/explanations/` with implementation summary

**These three rules will prevent 90% of rejections.**
