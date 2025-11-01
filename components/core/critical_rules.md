---
component:
  name: critical_rules
  category: core
  version: 2.0.0
  description: Critical rules that must be followed for all code contributions
  languages:
    - rust
    - python
    - golang
    - typescript
    - bash
  sections:
    - id: file_naming
      language_specific: false
      required: true
    - id: code_quality
      language_specific: true
      required: true
    - id: documentation
      language_specific: true
      required: true
    - id: error_handling
      language_specific: true
      required: true
    - id: testing
      language_specific: true
      required: true
    - id: git_conventions
      language_specific: false
      required: true
---

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
CORRECT:
   docs/architecture_overview.md
   docs/how_to_setup.md
   README.md (only exception)

WRONG:
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
CORRECT:
   config/production.yaml
   docker-compose.yaml
   .github/workflows/ci.yaml

WRONG:
   config/production.yml
   docker-compose.yml
```

---

## Rule 2: Code Quality Gates

**ALL of these MUST pass before claiming task complete:**

<!-- LANG:rust -->

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
cargo fmt         → No output (all files formatted)
cargo check       → "Finished" with 0 errors
cargo clippy      → "Finished" with 0 warnings
cargo test        → "test result: ok. X passed; 0 failed"
```

<!-- /LANG -->

<!-- LANG:python -->

```bash
# 1. Format code (auto-fixes issues)
black .
isort .

# 2. Type checking
mypy src/ --strict

# 3. Lint (enforces code quality)
pylint src/
flake8 src/

# 4. Tests (must pass with >80% coverage)
pytest --cov=src --cov-report=html --cov-fail-under=80
```

**Expected Results:**

```text
black .          → "All done! ✨ 🍰 ✨"
isort .          → No output or "Skipped X files"
mypy src/        → "Success: no issues found"
pylint src/      → "Your code has been rated at 10.00/10"
pytest           → "passed, 80% coverage"
```

<!-- /LANG -->

<!-- LANG:golang -->

```bash
# 1. Format code (auto-fixes issues)
gofmt -w .
goimports -w .

# 2. Vet code (checks for common mistakes)
go vet ./...

# 3. Lint (enforces code quality)
golangci-lint run

# 4. Tests (must pass with >80% coverage)
go test ./... -v -race -coverprofile=coverage.out
go tool cover -func=coverage.out
```

**Expected Results:**

```text
gofmt -w .               → No output (all files formatted)
go vet ./...             → No output (no issues found)
golangci-lint run        → No output or all checks pass
go test                  → "PASS" with coverage >80%
```

<!-- /LANG -->

<!-- LANG:typescript -->

```bash
# 1. Format code (auto-fixes issues)
npm run format
# or: prettier --write "src/**/*.ts"

# 2. Type checking
npm run type-check
# or: tsc --noEmit

# 3. Lint (enforces code quality)
npm run lint
# or: eslint "src/**/*.ts"

# 4. Tests (must pass with >80% coverage)
npm test -- --coverage --coverageThreshold='{"global":{"lines":80}}'
```

**Expected Results:**

```text
prettier         → Files formatted successfully
tsc --noEmit     → No output (no type errors)
eslint           → No warnings or errors
npm test         → "Tests passed", coverage >80%
```

<!-- /LANG -->

<!-- LANG:bash -->

```bash
# 1. Check syntax
bash -n script.sh

# 2. Lint (enforces code quality)
shellcheck script.sh

# 3. Format (if using shfmt)
shfmt -w script.sh

# 4. Tests (if using bats)
bats test/
```

**Expected Results:**

```text
bash -n          → No output (syntax valid)
shellcheck       → No warnings or errors
bats test/       → All tests pass
```

<!-- /LANG -->

**IF ANY FAIL**: Stop immediately and fix before proceeding.

---

## Rule 3: Documentation is Mandatory

**YOU MUST:**

1. **Doc Comments for ALL Public Items**

<!-- LANG:rust -->

````rust
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
````

<!-- /LANG -->

<!-- LANG:python -->

```python
def function(param: Type) -> Type:
    """Brief description of function.

    Args:
        param: Description of parameter

    Returns:
        Description of return value

    Raises:
        ErrorType: When condition occurs

    Examples:
        >>> result = function(arg)
        >>> assert result == expected
    """
    # Implementation
```

<!-- /LANG -->

<!-- LANG:golang -->

```go
// Function provides a brief description of what it does.
//
// Parameters:
//   - param: Description of parameter
//
// Returns:
//   - Type: Description of return value
//   - error: Description of error conditions
//
// Example:
//
//	result, err := Function(arg)
//	if err != nil {
//	    return err
//	}
func Function(param Type) (Type, error) {
    // Implementation
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

````typescript
/**
 * Brief description of function
 *
 * @param param - Description of parameter
 * @returns Description of return value
 * @throws {ErrorType} When condition occurs
 *
 * @example
 * ```typescript
 * const result = function(arg);
 * expect(result).toBe(expected);
 * ```
 */
export function function(param: Type): Type {
    // Implementation
}
````

<!-- /LANG -->

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

<!-- LANG:rust -->

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

<!-- /LANG -->

<!-- LANG:python -->

- Use custom exception classes for domain errors
- Raise exceptions for error conditions
- Document exceptions in docstrings
- Use context managers for resource cleanup

**NEVER:**

- Use bare `except:` clauses
- Ignore exceptions silently
- Return None to indicate errors
- Use assertions for validation

**Correct Pattern:**

```python
class ConfigError(Exception):
    """Raised when configuration is invalid."""
    pass

class ConfigReadError(ConfigError):
    """Raised when config file cannot be read."""
    pass

def load_config(path: str) -> Config:
    """Load configuration from file.

    Args:
        path: Path to config file

    Returns:
        Parsed configuration

    Raises:
        ConfigReadError: If file cannot be read
        ConfigError: If config is invalid
    """
    try:
        with open(path, 'r') as f:
            data = yaml.safe_load(f)
    except IOError as e:
        raise ConfigReadError(f"Cannot read {path}: {e}")

    if not validate(data):
        raise ConfigError(f"Invalid config in {path}")

    return Config.from_dict(data)
```

<!-- /LANG -->

<!-- LANG:golang -->

- Return errors as the last return value
- Use custom error types for domain errors
- Wrap errors with context using `fmt.Errorf`
- Check all errors explicitly

**NEVER:**

- Ignore errors with `_`
- Use panic for expected errors
- Return nil error without checking
- Log and return the same error

**Correct Pattern:**

```go
type ConfigError struct {
    Path string
    Err  error
}

func (e *ConfigError) Error() string {
    return fmt.Sprintf("config error in %s: %v", e.Path, e.Err)
}

func LoadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, &ConfigError{Path: path, Err: err}
    }

    var config Config
    if err := yaml.Unmarshal(data, &config); err != nil {
        return nil, &ConfigError{
            Path: path,
            Err:  fmt.Errorf("parse error: %w", err),
        }
    }

    return &config, nil
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

- Use custom Error classes for domain errors
- Throw errors for exceptional conditions
- Use Result type for expected failures
- Document errors in JSDoc

**NEVER:**

- Return null/undefined for errors
- Use any type for error handling
- Swallow errors silently
- Throw non-Error objects

**Correct Pattern:**

```typescript
class ConfigError extends Error {
  constructor(message: string, public readonly path: string) {
    super(message);
    this.name = "ConfigError";
  }
}

async function loadConfig(path: string): Promise<Config> {
  let data: string;
  try {
    data = await fs.readFile(path, "utf-8");
  } catch (err) {
    throw new ConfigError(`Failed to read config: ${err.message}`, path);
  }

  try {
    return JSON.parse(data) as Config;
  } catch (err) {
    throw new ConfigError(`Failed to parse config: ${err.message}`, path);
  }
}
```

<!-- /LANG -->

---

## Rule 5: Testing Requirements

**YOU MUST:**

- Write tests for ALL public functions
- Test both success AND failure cases
- Test edge cases and boundary conditions
- Achieve >80% code coverage
- Use descriptive test names

<!-- LANG:rust -->

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

<!-- /LANG -->

<!-- LANG:python -->

**Test Structure:**

```python
import pytest
from mymodule import function

class TestFunction:
    def test_with_valid_input(self):
        """Test function with valid input."""
        # Arrange
        input_data = create_valid_input()

        # Act
        result = function(input_data)

        # Assert
        assert result == expected_value

    def test_with_invalid_input(self):
        """Test function raises error with invalid input."""
        with pytest.raises(ValueError):
            function(invalid_input)

    def test_edge_case(self):
        """Test function with boundary conditions."""
        result = function(edge_case_input)
        assert result is not None
```

<!-- /LANG -->

<!-- LANG:golang -->

**Test Structure:**

```go
package mypackage

import (
    "testing"
)

func TestFunctionWithValidInput(t *testing.T) {
    // Arrange
    input := createValidInput()

    // Act
    result, err := Function(input)

    // Assert
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if result != expected {
        t.Errorf("got %v, want %v", result, expected)
    }
}

func TestFunctionWithInvalidInput(t *testing.T) {
    result, err := Function(invalidInput)
    if err == nil {
        t.Error("expected error, got nil")
    }
}

func TestFunctionEdgeCase(t *testing.T) {
    // Test boundary conditions
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

**Test Structure:**

```typescript
import { describe, it, expect } from '@jest/globals';
import { function } from './module';

describe('function', () => {
    it('should return expected value with valid input', () => {
        // Arrange
        const input = createValidInput();

        // Act
        const result = function(input);

        // Assert
        expect(result).toBe(expectedValue);
    });

    it('should throw error with invalid input', () => {
        expect(() => function(invalidInput)).toThrow();
    });

    it('should handle edge cases', () => {
        const result = function(edgeCaseInput);
        expect(result).toBeDefined();
    });
});
```

<!-- /LANG -->

---

## Rule 6: Git Commit Conventions

**Format:**

```text
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Rules:**

1. Type MUST be: `feat|fix|docs|style|refactor|perf|test|chore`
2. Description MUST be lowercase
3. Description MUST use imperative mood ("add" not "added")
4. First line MUST be ≤72 characters

**Examples:**

```text
CORRECT:
feat(auth): add JWT token refresh endpoint (PROJ-1234)
fix(api): handle edge case in validation (PROJ-5678)
docs(readme): update installation steps (PROJ-9012)

WRONG:
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

- Use emojis in code
- Use emojis in documentation
- Use emojis in commit messages

---

## Validation Checklist

**Before claiming task complete, verify ALL:**

- [ ] File naming follows conventions (lowercase_underscore.md, .yaml not .yml)
- [ ] All quality gates pass (format, check, lint, test)
- [ ] All public items have doc comments with examples
- [ ] Implementation documentation created in `docs/explanations/`
- [ ] No emojis in code, docs, or commits
- [ ] Commit message follows conventional format
- [ ] Error handling uses appropriate patterns
- [ ] Tests cover success, failure, and edge cases
- [ ] Code coverage >80%

---

## Emergency Quick Reference

**If you remember nothing else:**

1. **File Extensions**: `.yaml` NOT `.yml`, `.md` with lowercase_underscore
2. **Quality Gates**: All language-specific commands MUST pass
3. **Documentation**: Create file in `docs/explanations/` with implementation summary

**These three rules will prevent 90% of rejections.**
