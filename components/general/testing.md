---
component:
  name: testing_standards
  category: general
  version: 2.0.0
  description: Testing standards and practices for comprehensive test coverage
  languages:
    - rust
    - python
    - golang
    - typescript
    - bash
  sections:
    - id: principles
      language_specific: false
      required: true
    - id: test_types
      language_specific: false
      required: true
    - id: test_structure
      language_specific: true
      required: true
    - id: assertions
      language_specific: true
      required: true
    - id: mocking
      language_specific: true
      required: true
    - id: coverage
      language_specific: true
      required: true
---

# Testing Standards

## Core Principles

Follow these universal testing principles:

1. **Test Behavior, Not Implementation**: Focus on what code does, not how
2. **Write Tests First for Bugs**: Reproduce bugs with failing tests before fixing
3. **Keep Tests Simple**: Tests should be easier to understand than the code they test
4. **Test One Thing**: Each test validates a single behavior or condition
5. **Make Tests Deterministic**: Same inputs always produce same results
6. **Achieve >80% Coverage**: Minimum code coverage threshold for all projects
7. **Use Descriptive Names**: Test names should describe what they validate

---

## Test Types

### Unit Tests

Test individual functions and methods in isolation.

**Characteristics:**

- Fast execution (milliseconds)
- No external dependencies
- Test single units of code
- Mock dependencies

### Integration Tests

Test interactions between modules and components.

**Characteristics:**

- Moderate execution time
- May use test databases or services
- Test multiple components together
- Validate contracts between modules

### End-to-End Tests

Test complete workflows from user perspective.

**Characteristics:**

- Slower execution
- Use real or production-like environment
- Test entire system
- Focus on critical user journeys

**Test Pyramid:**

- 70% Unit Tests
- 20% Integration Tests
- 10% End-to-End Tests

---

## Test Structure

Organize tests using the Arrange-Act-Assert pattern.

<!-- LANG:rust -->

### Rust Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_valid_input_returns_ok() {
        // Arrange
        let input = "valid input";

        // Act
        let result = parse(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_parse_with_invalid_input_returns_error() {
        let result = parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_edge_case() {
        let input = "x".repeat(MAX_SIZE);
        assert!(parse(&input).is_ok());
    }
}
```

**Naming Convention:** `test_{function}_{condition}_{expected}`

**Test Organization:**

- Place unit tests in same file as code
- Use `#[cfg(test)]` module
- Group related tests in sub-modules
- Integration tests in `tests/` directory

<!-- /LANG -->

<!-- LANG:python -->

### Python Test Structure

```python
import pytest

class TestParser:
    def test_parse_with_valid_input_returns_result(self):
        # Arrange
        input_data = "valid input"

        # Act
        result = parse(input_data)

        # Assert
        assert result is not None
        assert result == expected_value

    def test_parse_with_invalid_input_raises_error(self):
        with pytest.raises(ValueError):
            parse("")

    def test_parse_with_edge_case(self):
        input_data = "x" * MAX_SIZE
        assert parse(input_data) is not None
```

**Naming Convention:** `test_{function}_{condition}_{expected}`

**Test Organization:**

- Place tests in `tests/` directory
- Use `test_*.py` or `*_test.py` naming
- Mirror source structure in test directory
- Use classes to group related tests

<!-- /LANG -->

<!-- LANG:golang -->

### Go Test Structure

```go
package parser

import "testing"

func TestParseWithValidInputReturnsResult(t *testing.T) {
    // Arrange
    input := "valid input"

    // Act
    result, err := Parse(input)

    // Assert
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if result != expectedValue {
        t.Errorf("got %v, want %v", result, expectedValue)
    }
}

func TestParseWithInvalidInputReturnsError(t *testing.T) {
    _, err := Parse("")
    if err == nil {
        t.Error("expected error, got nil")
    }
}

func TestParseWithEdgeCase(t *testing.T) {
    input := strings.Repeat("x", MaxSize)
    _, err := Parse(input)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
```

**Naming Convention:** `Test{Function}{Condition}{Expected}`

**Test Organization:**

- Place tests in `*_test.go` files
- Same package as code being tested
- Use table-driven tests for multiple cases
- Subtests with `t.Run()` for grouping

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Test Structure

```typescript
import { describe, it, expect } from "vitest";

describe("Parser", () => {
  it("should parse valid input and return result", () => {
    // Arrange
    const input = "valid input";

    // Act
    const result = parse(input);

    // Assert
    expect(result).toBeDefined();
    expect(result).toBe(expectedValue);
  });

  it("should throw error for invalid input", () => {
    expect(() => parse("")).toThrow(Error);
  });

  it("should handle edge case", () => {
    const input = "x".repeat(MAX_SIZE);
    expect(() => parse(input)).not.toThrow();
  });
});
```

**Naming Convention:** `should {expected behavior} when {condition}`

**Test Organization:**

- Place tests in `*.test.ts` or `*.spec.ts` files
- Use `describe` blocks for grouping
- Mirror source structure in test directory
- Use `it` or `test` for individual test cases

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Test Structure

```bash
#!/usr/bin/env bash

test_parse_with_valid_input_returns_ok() {
    # Arrange
    local input="valid input"

    # Act
    local result
    result=$(parse "$input")
    local status=$?

    # Assert
    assertEquals "Expected success" 0 "$status"
    assertEquals "Expected value" "expected" "$result"
}

test_parse_with_invalid_input_returns_error() {
    local result
    result=$(parse "" 2>&1)
    local status=$?

    assertNotEquals "Expected failure" 0 "$status"
}

# Run tests using shunit2
. shunit2
```

**Naming Convention:** `test_{function}_{condition}_{expected}`

**Test Organization:**

- Use testing framework like `shunit2` or `bats`
- Place tests in `tests/` directory
- Source functions to test
- One test file per script

<!-- /LANG -->

---

## Assertions

Use appropriate assertions for different validation scenarios.

<!-- LANG:rust -->

### Rust Assertions

```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, not_expected);

// Boolean conditions
assert!(condition);
assert!(!condition);

// Custom messages
assert_eq!(actual, expected, "Expected {}, got {}", expected, actual);

// Result assertions
assert!(result.is_ok());
assert!(result.is_err());

// Pattern matching
assert!(matches!(error, ConfigError::ParseError { .. }));

// Floating point comparison
assert!((actual - expected).abs() < 0.0001);
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Assertions

```python
# Equality
assert actual == expected
assert actual != not_expected

# Boolean conditions
assert condition
assert not condition

# Exception handling
with pytest.raises(ValueError):
    function()

with pytest.raises(ValueError, match="error message"):
    function()

# Approximate equality
assert actual == pytest.approx(expected, rel=1e-6)

# Collection membership
assert item in collection
assert item not in collection

# Type checking
assert isinstance(obj, ExpectedType)
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Assertions

```go
// Equality
if actual != expected {
    t.Errorf("got %v, want %v", actual, expected)
}

// Error checking
if err != nil {
    t.Fatalf("unexpected error: %v", err)
}

if err == nil {
    t.Error("expected error, got nil")
}

// Boolean conditions
if !condition {
    t.Error("expected condition to be true")
}

// Deep equality
if !reflect.DeepEqual(actual, expected) {
    t.Errorf("got %+v, want %+v", actual, expected)
}

// Using testify
assert.Equal(t, expected, actual)
assert.Error(t, err)
assert.NoError(t, err)
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Assertions

```typescript
// Equality
expect(actual).toBe(expected);
expect(actual).toEqual(expected); // Deep equality

// Boolean conditions
expect(condition).toBeTruthy();
expect(condition).toBeFalsy();

// Exception handling
expect(() => function()).toThrow(Error);
expect(() => function()).toThrow('error message');

// Null/undefined
expect(value).toBeNull();
expect(value).toBeUndefined();
expect(value).toBeDefined();

// Collection membership
expect(array).toContain(item);
expect(array).toHaveLength(3);

// Object matching
expect(object).toMatchObject({ key: 'value' });

// Approximate equality
expect(actual).toBeCloseTo(expected, 2);
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Assertions

```bash
# Equality
assertEquals "message" "expected" "$actual"
assertNotEquals "message" "not_expected" "$actual"

# Null/empty checks
assertNull "message" "$value"
assertNotNull "message" "$value"

# Boolean conditions
assertTrue "message" "$condition"
assertFalse "message" "$condition"

# String matching
assertContains "message" "substring" "$string"

# File checks
assertTrue "File should exist" "[ -f $file ]"
assertTrue "Directory should exist" "[ -d $dir ]"
```

<!-- /LANG -->

---

## Mocking

Mock external dependencies to isolate code under test.

<!-- LANG:rust -->

### Rust Mocking

```rust
use mockall::*;

#[automock]
trait Database {
    fn get_user(&self, id: u64) -> Result<User>;
    fn save_user(&mut self, user: User) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_with_mock_database() {
        let mut mock_db = MockDatabase::new();

        mock_db.expect_get_user()
            .with(eq(123))
            .times(1)
            .returning(|_| Ok(User::default()));

        let service = UserService::new(mock_db);
        let result = service.process_user(123);

        assert!(result.is_ok());
    }
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Mocking

```python
from unittest.mock import Mock, patch, MagicMock

def test_service_with_mock_database():
    # Mock object
    mock_db = Mock()
    mock_db.get_user.return_value = User()

    service = UserService(mock_db)
    result = service.process_user(123)

    mock_db.get_user.assert_called_once_with(123)
    assert result is not None

def test_with_patch():
    with patch('module.Database') as mock_db:
        mock_db.return_value.get_user.return_value = User()

        service = UserService()
        result = service.process_user(123)

        assert result is not None

@patch('module.external_api')
def test_with_decorator(mock_api):
    mock_api.fetch.return_value = {'data': 'value'}
    result = process_data()
    assert result == expected
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Mocking

```go
// Interface for mocking
type Database interface {
    GetUser(id uint64) (*User, error)
    SaveUser(user *User) error
}

// Mock implementation
type MockDatabase struct {
    GetUserFunc  func(id uint64) (*User, error)
    SaveUserFunc func(user *User) error
}

func (m *MockDatabase) GetUser(id uint64) (*User, error) {
    return m.GetUserFunc(id)
}

func (m *MockDatabase) SaveUser(user *User) error {
    return m.SaveUserFunc(user)
}

// Test using mock
func TestServiceWithMockDatabase(t *testing.T) {
    mockDB := &MockDatabase{
        GetUserFunc: func(id uint64) (*User, error) {
            return &User{ID: id}, nil
        },
    }

    service := NewUserService(mockDB)
    user, err := service.ProcessUser(123)

    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if user.ID != 123 {
        t.Errorf("got ID %d, want 123", user.ID)
    }
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Mocking

```typescript
import { vi } from "vitest";

interface Database {
  getUser(id: number): Promise<User>;
  saveUser(user: User): Promise<void>;
}

describe("UserService", () => {
  it("should process user with mocked database", async () => {
    const mockDb: Database = {
      getUser: vi.fn().mockResolvedValue({ id: 123, name: "Test" }),
      saveUser: vi.fn().mockResolvedValue(undefined),
    };

    const service = new UserService(mockDb);
    const result = await service.processUser(123);

    expect(mockDb.getUser).toHaveBeenCalledWith(123);
    expect(result).toBeDefined();
  });

  it("should handle API calls with spy", () => {
    const spy = vi.spyOn(api, "fetch");
    spy.mockResolvedValue({ data: "value" });

    const result = processData();

    expect(spy).toHaveBeenCalled();
    expect(result).toBe(expected);
  });
});
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Mocking

```bash
# Override function for testing
original_curl=$(declare -f curl)

curl() {
    echo '{"status": "ok"}'
    return 0
}

test_api_call_with_mock() {
    result=$(make_api_call)
    assertEquals "Expected success" 0 $?
    assertContains "Response contains status" "$result" "ok"
}

# Restore original function
eval "$original_curl"

# Mock external commands
test_with_command_mock() {
    # Create mock in PATH
    mkdir -p "$SHUNIT_TMPDIR/bin"
    export PATH="$SHUNIT_TMPDIR/bin:$PATH"

    cat > "$SHUNIT_TMPDIR/bin/external_command" << 'EOF'
#!/bin/sh
echo "mocked output"
EOF
    chmod +x "$SHUNIT_TMPDIR/bin/external_command"

    result=$(function_using_external_command)
    assertContains "Uses mock" "$result" "mocked"
}
```

<!-- /LANG -->

---

## Coverage

Measure and maintain test coverage above 80%.

<!-- LANG:rust -->

### Rust Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# CI-friendly output
cargo tarpaulin --out Xml

# With all features
cargo tarpaulin --all-features --workspace

# View report
open coverage/index.html
```

**Coverage Targets:**

- Overall: >80%
- Critical paths: 100%
- New code: 100%

<!-- /LANG -->

<!-- LANG:python -->

### Python Coverage

```bash
# Install coverage.py
pip install coverage pytest-cov

# Run tests with coverage
pytest --cov=mypackage --cov-report=html

# Generate report
coverage run -m pytest
coverage report
coverage html

# View report
open htmlcov/index.html

# Fail if below threshold
pytest --cov=mypackage --cov-fail-under=80
```

**Coverage Targets:**

- Overall: >80%
- Critical modules: >90%
- New code: 100%

<!-- /LANG -->

<!-- LANG:golang -->

### Go Coverage

```bash
# Run tests with coverage
go test -cover ./...

# Generate coverage profile
go test -coverprofile=coverage.out ./...

# View coverage report
go tool cover -html=coverage.out

# Function-level coverage
go test -covermode=count -coverprofile=coverage.out ./...
go tool cover -func=coverage.out

# CI: fail if below threshold
go test -cover ./... | grep -v "coverage: 100.0%"
```

**Coverage Targets:**

- Overall: >80%
- Package-level: >75%
- Critical functions: 100%

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Coverage

```bash
# Install coverage tools
npm install --save-dev @vitest/coverage-v8

# Run tests with coverage
npx vitest --coverage

# Generate HTML report
npx vitest --coverage --reporter=html

# View report
open coverage/index.html

# CI: enforce thresholds (vitest.config.ts)
# coverage: {
#   lines: 80,
#   functions: 80,
#   branches: 80,
#   statements: 80
# }
```

**Coverage Targets:**

- Lines: >80%
- Branches: >75%
- Functions: >85%

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Coverage

```bash
# Install kcov
apt-get install kcov  # Debian/Ubuntu

# Run tests with coverage
kcov --exclude-pattern=/usr coverage tests/run_tests.sh

# View report
open coverage/index.html

# Using shunit2 with coverage
kcov coverage shunit2 tests/test_script.sh

# Simple line tracking
# Add to script:
# PS4='+(${BASH_SOURCE}:${LINENO}): ${FUNCNAME[0]:+${FUNCNAME[0]}(): }'
# set -x
```

**Coverage Targets:**

- Critical scripts: >80%
- Utility scripts: >70%
- Complex logic: 100%

<!-- /LANG -->

---

## Additional Guidelines

### Test Data Management

- Use fixtures for reusable test data
- Keep test data small and focused
- Clean up test artifacts after execution
- Use temporary directories for file operations

### Continuous Testing

- Run tests on every commit
- Fail builds on test failures
- Track coverage trends over time
- Run tests in parallel when possible

### Test Maintenance

- Update tests when requirements change
- Remove obsolete tests
- Refactor duplicated test code
- Keep tests as simple as production code
