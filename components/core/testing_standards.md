---
component:
  name: testing_standards
  category: core
  version: 2.0.0
  description: Testing standards and best practices for comprehensive test coverage
  languages:
    - rust
    - python
    - golang
    - typescript
  sections:
    - id: philosophy
      language_specific: false
      required: true
    - id: test_structure
      language_specific: true
      required: true
    - id: naming_conventions
      language_specific: true
      required: true
    - id: assertions
      language_specific: true
      required: true
    - id: test_fixtures
      language_specific: true
      required: true
    - id: coverage
      language_specific: true
      required: true
---

# Testing Standards

## Testing Philosophy

**Core Principles:**

- Test behavior, not implementation
- Write tests before fixing bugs (TDD for bug fixes)
- Keep tests simple and readable
- Test one thing per test
- Make tests deterministic (no random values, no timing dependencies)
- Achieve minimum 80% code coverage
- Test both success AND failure paths
- Test edge cases and boundary conditions

**Test Pyramid:**

1. **Unit Tests** (70%): Fast, isolated, test single functions/methods
2. **Integration Tests** (20%): Test component interactions
3. **End-to-End Tests** (10%): Test complete workflows

---

## Test Structure and Organization

Write tests following the Arrange-Act-Assert (AAA) pattern for clarity.

<!-- LANG:rust -->

### Rust Test Structure

**Unit Tests (same file as code):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions for test setup
    fn create_test_config() -> Config {
        Config {
            name: "test".to_string(),
            value: 42,
        }
    }

    #[test]
    fn test_config_creation_with_valid_data() {
        // Arrange
        let name = "test";
        let value = 42;

        // Act
        let config = Config::new(name, value);

        // Assert
        assert_eq!(config.name, name);
        assert_eq!(config.value, value);
    }

    #[test]
    fn test_config_validation_rejects_empty_name() {
        // Arrange
        let config = Config {
            name: String::new(),
            value: 42,
        };

        // Act
        let result = config.validate();

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name"));
    }

    #[test]
    fn test_config_edge_case_max_value() {
        let config = Config::new("test", i32::MAX);
        assert!(config.validate().is_ok());
    }
}
```

**Integration Tests (tests/ directory):**

```rust
// tests/integration_test.rs
use myapp::{Config, ConfigLoader};
use tempfile::tempdir;

#[test]
fn test_load_config_from_file() {
    // Arrange
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.yaml");
    std::fs::write(&config_path, "name: test\nvalue: 42").unwrap();

    // Act
    let loader = ConfigLoader::new();
    let config = loader.load(&config_path).unwrap();

    // Assert
    assert_eq!(config.name, "test");
    assert_eq!(config.value, 42);
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Test Structure

**Unit Tests (using pytest):**

```python
import pytest
from mymodule import Config, ConfigError


class TestConfig:
    """Tests for Config class."""

    @pytest.fixture
    def valid_config_data(self):
        """Fixture providing valid config data."""
        return {"name": "test", "value": 42}

    def test_config_creation_with_valid_data(self, valid_config_data):
        """Test Config creation with valid data."""
        # Arrange
        name = valid_config_data["name"]
        value = valid_config_data["value"]

        # Act
        config = Config(name, value)

        # Assert
        assert config.name == name
        assert config.value == value

    def test_config_validation_rejects_empty_name(self):
        """Test that validation fails for empty name."""
        # Arrange
        config = Config("", 42)

        # Act & Assert
        with pytest.raises(ConfigError) as exc_info:
            config.validate()

        assert "name" in str(exc_info.value)

    def test_config_edge_case_max_value(self):
        """Test Config with maximum integer value."""
        config = Config("test", 2**31 - 1)
        config.validate()  # Should not raise


class TestConfigIntegration:
    """Integration tests for Config system."""

    def test_load_config_from_file(self, tmp_path):
        """Test loading config from file."""
        # Arrange
        config_file = tmp_path / "config.yaml"
        config_file.write_text("name: test\nvalue: 42")

        # Act
        config = load_config(str(config_file))

        # Assert
        assert config.name == "test"
        assert config.value == 42
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Test Structure

**Unit Tests (same package):**

```go
package config

import (
    "testing"
)

// Helper function for test setup
func createTestConfig() *Config {
    return &Config{
        Name:  "test",
        Value: 42,
    }
}

func TestConfig_Creation_WithValidData(t *testing.T) {
    // Arrange
    name := "test"
    value := 42

    // Act
    config := NewConfig(name, value)

    // Assert
    if config.Name != name {
        t.Errorf("expected name %s, got %s", name, config.Name)
    }
    if config.Value != value {
        t.Errorf("expected value %d, got %d", value, config.Value)
    }
}

func TestConfig_Validation_RejectsEmptyName(t *testing.T) {
    // Arrange
    config := &Config{
        Name:  "",
        Value: 42,
    }

    // Act
    err := config.Validate()

    // Assert
    if err == nil {
        t.Fatal("expected validation error for empty name")
    }
    if !strings.Contains(err.Error(), "name") {
        t.Errorf("error message should mention 'name', got: %s", err.Error())
    }
}

func TestConfig_EdgeCase_MaxValue(t *testing.T) {
    config := NewConfig("test", math.MaxInt32)
    if err := config.Validate(); err != nil {
        t.Errorf("unexpected error: %v", err)
    }
}
```

**Integration Tests (separate package):**

```go
package config_test

import (
    "os"
    "path/filepath"
    "testing"

    "myapp/config"
)

func TestLoadConfig_FromFile(t *testing.T) {
    // Arrange
    tmpDir := t.TempDir()
    configPath := filepath.Join(tmpDir, "config.yaml")
    content := []byte("name: test\nvalue: 42")
    if err := os.WriteFile(configPath, content, 0644); err != nil {
        t.Fatalf("failed to write test config: %v", err)
    }

    // Act
    cfg, err := config.LoadFromFile(configPath)

    // Assert
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if cfg.Name != "test" {
        t.Errorf("expected name 'test', got %s", cfg.Name)
    }
    if cfg.Value != 42 {
        t.Errorf("expected value 42, got %d", cfg.Value)
    }
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Test Structure

**Unit Tests (using Jest):**

```typescript
import { Config, ConfigError } from './config';

describe('Config', () => {
    // Helper function for test setup
    const createTestConfig = (): Config => {
        return new Config('test', 42);
    };

    describe('creation', () => {
        it('should create config with valid data', () => {
            // Arrange
            const name = 'test';
            const value = 42;

            // Act
            const config = new Config(name, value);

            // Assert
            expect(config.name).toBe(name);
            expect(config.value).toBe(value);
        });
    });

    describe('validation', () => {
        it('should reject empty name', () => {
            // Arrange
            const config = new Config('', 42);

            // Act & Assert
            expect(() => config.validate()).toThrow(ConfigError);
            expect(() => config.validate()).toThrow(/name/i);
        });

        it('should accept maximum safe integer', () => {
            // Arrange
            const config = new Config('test', Number.MAX_SAFE_INTEGER);

            // Act & Assert
            expect(() => config.validate()).not.toThrow();
        });
    });
});

describe('Config Integration', () => {
    it('should load config from file', async () => {
        // Arrange
        const configPath = '/tmp/test-config.json';
        await fs.writeFile(
            configPath,
            JSON.stringify({ name: 'test', value: 42 })
        );

        // Act
        const config = await loadConfig(configPath);

        // Assert
        expect(config.name).toBe('test');
        expect(config.value).toBe(42);

        // Cleanup
        await fs.unlink(configPath);
    });
});
```

<!-- /LANG -->

---

## Test Naming Conventions

Use descriptive names that explain what is being tested and expected behavior.

<!-- LANG:rust -->

### Rust Test Naming

**Pattern:** `test_{function}_{condition}_{expected_outcome}`

```rust
#[test]
fn test_parse_config_with_valid_input_returns_ok() { }

#[test]
fn test_parse_config_with_empty_input_returns_error() { }

#[test]
fn test_parse_config_with_invalid_format_returns_parse_error() { }

#[test]
fn test_component_with_max_size_succeeds() { }

#[test]
fn test_component_with_oversized_content_fails() { }
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Test Naming

**Pattern:** `test_{function}_{condition}_{expected_outcome}`

```python
def test_parse_config_with_valid_input_returns_config():
    """Test parse_config returns Config with valid input."""
    pass

def test_parse_config_with_empty_input_raises_error():
    """Test parse_config raises ConfigError with empty input."""
    pass

def test_parse_config_with_invalid_format_raises_parse_error():
    """Test parse_config raises ParseError with invalid format."""
    pass

def test_component_with_max_size_succeeds():
    """Test component creation succeeds at maximum size."""
    pass
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Test Naming

**Pattern:** `Test{Type}_{Method}_{Condition}_{ExpectedOutcome}`

```go
func TestConfig_Parse_WithValidInput_ReturnsConfig(t *testing.T) { }

func TestConfig_Parse_WithEmptyInput_ReturnsError(t *testing.T) { }

func TestConfig_Parse_WithInvalidFormat_ReturnsParseError(t *testing.T) { }

func TestComponent_Create_WithMaxSize_Succeeds(t *testing.T) { }

func TestComponent_Create_WithOversizedContent_ReturnsError(t *testing.T) { }
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Test Naming

**Pattern:** Descriptive English using `describe` and `it`

```typescript
describe('Config.parse', () => {
    it('should return Config with valid input', () => { });

    it('should throw error with empty input', () => { });

    it('should throw ParseError with invalid format', () => { });
});

describe('Component', () => {
    it('should succeed with maximum allowed size', () => { });

    it('should fail with oversized content', () => { });
});
```

<!-- /LANG -->

---

## Assertions and Expectations

Use appropriate assertion methods for clear test failures.

<!-- LANG:rust -->

### Rust Assertions

```rust
// Basic assertions
assert!(condition);
assert!(!condition);
assert_eq!(actual, expected);
assert_ne!(actual, not_expected);

// Custom messages
assert_eq!(
    actual,
    expected,
    "Expected config name to be '{}', got '{}'",
    expected,
    actual
);

// Result assertions
let result = function();
assert!(result.is_ok());
assert!(result.is_err());
assert_eq!(result.unwrap(), expected_value);

// Error matching
match result {
    Ok(value) => assert_eq!(value, expected),
    Err(e) => panic!("Unexpected error: {}", e),
}

// Pattern matching with assert
assert!(matches!(result, Ok(Config { name, .. }) if name == "test"));

// Panic assertions
#[should_panic(expected = "invalid config")]
#[test]
fn test_panics_with_invalid_config() {
    process_invalid_config();
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Assertions

```python
# Basic assertions (pytest style)
assert condition
assert not condition
assert actual == expected
assert actual != not_expected

# Custom messages
assert actual == expected, f"Expected {expected}, got {actual}"

# Exception assertions
with pytest.raises(ConfigError):
    load_invalid_config()

with pytest.raises(ConfigError, match=r"invalid.*name"):
    validate_config({"name": ""})

# Capturing exception for inspection
with pytest.raises(ConfigError) as exc_info:
    load_invalid_config()

assert exc_info.value.field == "name"
assert "empty" in str(exc_info.value)

# Approximate equality
assert actual == pytest.approx(expected, rel=1e-6)

# Collection assertions
assert item in collection
assert len(collection) == 5
assert set(actual) == set(expected)
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Assertions

```go
// Basic assertions
if actual != expected {
    t.Errorf("expected %v, got %v", expected, actual)
}

if !condition {
    t.Error("condition should be true")
}

// Fatal errors (stop test immediately)
if err != nil {
    t.Fatalf("unexpected error: %v", err)
}

// Helper for cleaner assertions
func assertEqual(t *testing.T, actual, expected interface{}) {
    t.Helper()
    if actual != expected {
        t.Errorf("expected %v, got %v", expected, actual)
    }
}

// Error assertions
if err == nil {
    t.Error("expected error, got nil")
}

if err != nil {
    t.Errorf("unexpected error: %v", err)
}

// Error type assertions
var configErr *ConfigError
if !errors.As(err, &configErr) {
    t.Errorf("expected ConfigError, got %T", err)
}

// Using testify/assert (optional)
import "github.com/stretchr/testify/assert"

assert.Equal(t, expected, actual)
assert.NoError(t, err)
assert.Error(t, err)
assert.Contains(t, "hello world", "hello")
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Assertions

```typescript
// Jest assertions
expect(actual).toBe(expected);
expect(actual).not.toBe(notExpected);
expect(actual).toEqual(expected); // Deep equality

// Truthiness
expect(condition).toBeTruthy();
expect(condition).toBeFalsy();
expect(value).toBeDefined();
expect(value).toBeNull();

// Numbers
expect(value).toBeGreaterThan(10);
expect(value).toBeLessThanOrEqual(100);
expect(value).toBeCloseTo(0.3, 5); // precision

// Strings
expect(str).toMatch(/pattern/);
expect(str).toContain('substring');

// Arrays/Objects
expect(array).toContain(item);
expect(array).toHaveLength(3);
expect(obj).toHaveProperty('key', value);
expect(obj).toMatchObject({ name: 'test' });

// Exceptions
expect(() => throwError()).toThrow();
expect(() => throwError()).toThrow(ConfigError);
expect(() => throwError()).toThrow(/error message/);

// Async assertions
await expect(promise).resolves.toBe(value);
await expect(promise).rejects.toThrow(Error);

// Custom messages
expect(actual).toBe(expected, `Expected ${expected}, got ${actual}`);
```

<!-- /LANG -->

---

## Test Fixtures and Setup

Create reusable test data and setup functions to keep tests DRY.

<!-- LANG:rust -->

### Rust Test Fixtures

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Simple fixture function
    fn create_test_config() -> Config {
        Config {
            name: "test".to_string(),
            value: 42,
        }
    }

    // Fixture with cleanup using Drop
    struct TestEnvironment {
        temp_dir: tempfile::TempDir,
        config_path: PathBuf,
    }

    impl TestEnvironment {
        fn new() -> Self {
            let temp_dir = tempfile::tempdir().unwrap();
            let config_path = temp_dir.path().join("config.yaml");
            std::fs::write(&config_path, "name: test\nvalue: 42").unwrap();

            Self {
                temp_dir,
                config_path,
            }
        }
    }

    #[test]
    fn test_with_fixture() {
        let env = TestEnvironment::new();
        let config = load_config(&env.config_path).unwrap();
        assert_eq!(config.name, "test");
        // temp_dir automatically cleaned up when env is dropped
    }

    // Shared test data
    mod test_data {
        pub const VALID_CONFIG: &str = r#"
            name: test
            value: 42
        "#;

        pub const INVALID_CONFIG: &str = "invalid yaml: : :";
    }
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Test Fixtures

```python
import pytest


@pytest.fixture
def test_config():
    """Fixture providing a test Config instance."""
    return Config("test", 42)


@pytest.fixture
def config_file(tmp_path):
    """Fixture providing a config file path with content."""
    config_path = tmp_path / "config.yaml"
    config_path.write_text("name: test\nvalue: 42")
    return config_path


@pytest.fixture
def test_environment(tmp_path):
    """Fixture providing complete test environment."""
    env = {
        "config_dir": tmp_path / "config",
        "data_dir": tmp_path / "data",
    }
    env["config_dir"].mkdir()
    env["data_dir"].mkdir()
    return env


@pytest.fixture(scope="module")
def database():
    """Module-scoped fixture for expensive setup."""
    db = setup_test_database()
    yield db
    teardown_test_database(db)


# Using fixtures in tests
def test_load_config(config_file):
    """Test loading config from file."""
    config = load_config(str(config_file))
    assert config.name == "test"


def test_with_environment(test_environment):
    """Test with full environment setup."""
    result = process_environment(test_environment)
    assert result.success


# Fixture factories
@pytest.fixture
def make_config():
    """Fixture factory for creating configs."""
    def _make_config(name="test", value=42):
        return Config(name, value)
    return _make_config


def test_with_factory(make_config):
    config1 = make_config("first", 1)
    config2 = make_config("second", 2)
    assert config1.name != config2.name
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Test Fixtures

```go
// Simple fixture function
func createTestConfig(t *testing.T) *Config {
    t.Helper()
    return &Config{
        Name:  "test",
        Value: 42,
    }
}

// Fixture with cleanup
func setupTestEnvironment(t *testing.T) (string, func()) {
    t.Helper()

    // Setup
    tmpDir := t.TempDir() // Automatically cleaned up
    configPath := filepath.Join(tmpDir, "config.yaml")
    content := []byte("name: test\nvalue: 42")
    if err := os.WriteFile(configPath, content, 0644); err != nil {
        t.Fatalf("setup failed: %v", err)
    }

    // Cleanup function
    cleanup := func() {
        // Additional cleanup if needed
        // t.TempDir() handles directory cleanup automatically
    }

    return configPath, cleanup
}

// Usage
func TestWithFixture(t *testing.T) {
    configPath, cleanup := setupTestEnvironment(t)
    defer cleanup()

    config, err := LoadConfig(configPath)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if config.Name != "test" {
        t.Errorf("expected name 'test', got %s", config.Name)
    }
}

// Table-driven tests with fixtures
func TestConfigValidation(t *testing.T) {
    tests := []struct {
        name    string
        config  *Config
        wantErr bool
    }{
        {
            name:    "valid config",
            config:  &Config{Name: "test", Value: 42},
            wantErr: false,
        },
        {
            name:    "empty name",
            config:  &Config{Name: "", Value: 42},
            wantErr: true,
        },
        {
            name:    "negative value",
            config:  &Config{Name: "test", Value: -1},
            wantErr: true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := tt.config.Validate()
            if (err != nil) != tt.wantErr {
                t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
            }
        })
    }
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Test Fixtures

```typescript
// Simple fixture functions
function createTestConfig(): Config {
    return new Config('test', 42);
}

// Jest beforeEach/afterEach
describe('Config', () => {
    let config: Config;

    beforeEach(() => {
        // Setup before each test
        config = new Config('test', 42);
    });

    afterEach(() => {
        // Cleanup after each test
        config = null;
    });

    it('should validate', () => {
        expect(() => config.validate()).not.toThrow();
    });
});

// Setup with cleanup
describe('Config Integration', () => {
    let configPath: string;

    beforeAll(async () => {
        // Setup once before all tests
        configPath = '/tmp/test-config.json';
        await fs.writeFile(
            configPath,
            JSON.stringify({ name: 'test', value: 42 })
        );
    });

    afterAll(async () => {
        // Cleanup once after all tests
        await fs.unlink(configPath);
    });

    it('should load from file', async () => {
        const config = await loadConfig(configPath);
        expect(config.name).toBe('test');
    });
});

// Fixture factories
function makeConfigFactory() {
    return (name = 'test', value = 42): Config => {
        return new Config(name, value);
    };
}

describe('with factory', () => {
    const makeConfig = makeConfigFactory();

    it('should create different configs', () => {
        const config1 = makeConfig('first', 1);
        const config2 = makeConfig('second', 2);
        expect(config1.name).not.toBe(config2.name);
    });
});

// Test data constants
const TEST_DATA = {
    VALID_CONFIG: { name: 'test', value: 42 },
    INVALID_CONFIG: { name: '', value: -1 },
    EDGE_CASE_CONFIG: { name: 'test', value: Number.MAX_SAFE_INTEGER },
};
```

<!-- /LANG -->

---

## Code Coverage

Measure and maintain high test coverage to ensure code quality.

<!-- LANG:rust -->

### Rust Coverage

```bash
# Using cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Generate coverage with specific options
cargo tarpaulin --out Html --out Lcov --output-dir coverage

# View coverage
open tarpaulin-report.html

# Check coverage threshold
cargo tarpaulin --fail-under 80
```

**Coverage Requirements:**

- Minimum 80% overall coverage
- 100% coverage for public APIs
- 100% coverage for error paths
- 100% coverage for critical business logic

<!-- /LANG -->

<!-- LANG:python -->

### Python Coverage

```bash
# Install coverage tool
pip install pytest-cov

# Run tests with coverage
pytest --cov=src --cov-report=html --cov-report=term

# Generate coverage report
pytest --cov=src --cov-report=html
open htmlcov/index.html

# Fail if coverage below threshold
pytest --cov=src --cov-fail-under=80

# Show missing lines
pytest --cov=src --cov-report=term-missing
```

**Coverage Configuration (.coveragerc):**

```ini
[run]
source = src
omit =
    */tests/*
    */test_*.py
    */__pycache__/*

[report]
exclude_lines =
    pragma: no cover
    def __repr__
    raise AssertionError
    raise NotImplementedError
    if __name__ == .__main__.:
    if TYPE_CHECKING:
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Coverage

```bash
# Run tests with coverage
go test ./... -coverprofile=coverage.out

# View coverage report
go tool cover -html=coverage.out

# Show coverage per package
go test ./... -cover

# Detailed coverage function-by-function
go tool cover -func=coverage.out

# Check coverage threshold (using script)
coverage=$(go test ./... -coverprofile=coverage.out | grep "coverage:" | awk '{print $2}' | sed 's/%//')
if [ $(echo "$coverage < 80" | bc) -eq 1 ]; then
    echo "Coverage $coverage% is below 80%"
    exit 1
fi
```

**Coverage Best Practices:**

- Test all exported functions
- Test all error paths
- Test boundary conditions
- Aim for 80%+ coverage
- Don't chase 100% coverage artificially

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Coverage

```bash
# Jest with coverage
npm test -- --coverage

# Generate HTML report
npm test -- --coverage --coverageReporters=html
open coverage/index.html

# Check coverage thresholds
npm test -- --coverage --coverageThreshold='{"global":{"lines":80,"branches":80,"functions":80,"statements":80}}'

# Coverage for specific files
npm test -- --coverage --collectCoverageFrom='src/**/*.ts'
```

**Jest Coverage Configuration (jest.config.js):**

```javascript
module.exports = {
  collectCoverage: true,
  collectCoverageFrom: [
    'src/**/*.ts',
    '!src/**/*.d.ts',
    '!src/**/*.test.ts',
  ],
  coverageThreshold: {
    global: {
      lines: 80,
      branches: 80,
      functions: 80,
      statements: 80,
    },
  },
  coverageReporters: ['text', 'html', 'lcov'],
};
```

<!-- /LANG -->

---

## Best Practices Summary

1. **Write Tests First**: For bug fixes, write failing test first (TDD)
2. **Test Behavior**: Focus on what code does, not how it does it
3. **Use AAA Pattern**: Arrange-Act-Assert for clear test structure
4. **Descriptive Names**: Test names should explain what is tested
5. **One Assertion Focus**: Each test should verify one behavior
6. **Test All Paths**: Success, failure, edge cases, and boundaries
7. **Make Tests Fast**: Use mocks/stubs for external dependencies
8. **Keep Tests Independent**: Tests should not depend on each other
9. **Maintain High Coverage**: Aim for 80%+ code coverage
10. **Clean Up Resources**: Use fixtures and cleanup functions

---

## Common Testing Anti-Patterns

**DON'T:**

- Write tests that depend on execution order
- Use sleep/timing in tests
- Test implementation details
- Skip error case testing
- Leave tests commented out
- Write tests without assertions
- Test third-party code
- Make tests too complex

**DO:**

- Make tests deterministic
- Use mocks for timing/external dependencies
- Test behavior and interfaces
- Test all error conditions
- Delete or fix failing tests
- Assert expected behavior
- Trust third-party libraries
- Keep tests simple and focused
