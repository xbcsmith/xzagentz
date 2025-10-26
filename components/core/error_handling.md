---
component:
  name: error_handling
  category: core
  version: 2.0.0
  description: Error handling standards and patterns for robust code
  languages:
    - rust
    - python
    - golang
    - typescript
  sections:
    - id: principles
      language_specific: false
      required: true
    - id: error_types
      language_specific: true
      required: true
    - id: error_patterns
      language_specific: true
      required: true
    - id: context_propagation
      language_specific: true
      required: true
    - id: testing_errors
      language_specific: true
      required: true
---

# Error Handling Standards

## Core Principles

Error handling is critical for building robust, maintainable software. Follow these universal principles:

1. **Make Errors Explicit**: Use type systems to communicate possible failures
2. **Provide Context**: Include relevant information about what went wrong and why
3. **Fail Fast**: Detect and report errors as early as possible
4. **Don't Swallow Errors**: Always handle errors appropriately or propagate them
5. **Make Errors Actionable**: Users should understand what went wrong and how to fix it
6. **Separate Expected from Unexpected**: Handle recoverable errors differently from bugs
7. **Document Error Conditions**: Clearly document when and why functions fail

---

## Error Types

Define clear error types for your domain. Custom error types improve code clarity and error handling.

<!-- LANG:rust -->

### Rust Error Types

Use `thiserror` for defining error types with automatic trait implementations:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read configuration file at {path}: {source}")]
    ReadError {
        path: String,
        source: std::io::Error,
    },

    #[error("Invalid configuration format: {reason}")]
    ParseError { reason: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Validation failed: {0}")]
    ValidationError(String),
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
```

**Key Features:**
- `#[error("...")]` defines display message
- `#[from]` enables automatic conversion
- Include context in error variants
- Use enum variants for different error categories

<!-- /LANG -->

<!-- LANG:python -->

### Python Error Types

Define custom exception classes for domain-specific errors:

```python
class ConfigError(Exception):
    """Base exception for configuration errors."""

    def __init__(self, message: str, path: Optional[str] = None):
        self.message = message
        self.path = path
        super().__init__(self.message)

    def __str__(self) -> str:
        if self.path:
            return f"Configuration error in {self.path}: {self.message}"
        return f"Configuration error: {self.message}"


class ConfigReadError(ConfigError):
    """Raised when configuration file cannot be read."""
    pass


class ConfigParseError(ConfigError):
    """Raised when configuration has invalid format."""

    def __init__(self, message: str, path: str, line: Optional[int] = None):
        super().__init__(message, path)
        self.line = line

    def __str__(self) -> str:
        if self.line:
            return f"Parse error in {self.path} at line {self.line}: {self.message}"
        return super().__str__()


class ValidationError(ConfigError):
    """Raised when configuration validation fails."""

    def __init__(self, field: str, reason: str):
        self.field = field
        self.reason = reason
        super().__init__(f"Validation failed for '{field}': {reason}")
```

**Key Features:**
- Inherit from appropriate base exception
- Include relevant context as attributes
- Override `__str__` for clear error messages
- Use hierarchy for error categorization

<!-- /LANG -->

<!-- LANG:golang -->

### Go Error Types

Define custom error types implementing the error interface:

```go
package config

import "fmt"

// ConfigError represents configuration-related errors
type ConfigError struct {
    Path    string
    Reason  string
    Wrapped error
}

func (e *ConfigError) Error() string {
    if e.Wrapped != nil {
        return fmt.Sprintf("config error in %s: %s: %v", e.Path, e.Reason, e.Wrapped)
    }
    return fmt.Sprintf("config error in %s: %s", e.Path, e.Reason)
}

func (e *ConfigError) Unwrap() error {
    return e.Wrapped
}

// ReadError represents file reading errors
type ReadError struct {
    Path string
    Err  error
}

func (e *ReadError) Error() string {
    return fmt.Sprintf("failed to read %s: %v", e.Path, e.Err)
}

func (e *ReadError) Unwrap() error {
    return e.Err
}

// ValidationError represents validation failures
type ValidationError struct {
    Field  string
    Reason string
}

func (e *ValidationError) Error() string {
    return fmt.Sprintf("validation failed for '%s': %s", e.Field, e.Reason)
}

// NotFoundError represents missing resources
type NotFoundError struct {
    Resource string
}

func (e *NotFoundError) Error() string {
    return fmt.Sprintf("%s not found", e.Resource)
}
```

**Key Features:**
- Implement `Error() string` method
- Implement `Unwrap() error` for error chains
- Include context in struct fields
- Use descriptive type names

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Error Types

Define custom Error classes with proper type information:

```typescript
export class ConfigError extends Error {
  constructor(
    message: string,
    public readonly path?: string,
    public readonly cause?: Error
  ) {
    super(message);
    this.name = "ConfigError";
    Object.setPrototypeOf(this, ConfigError.prototype);
  }

  toString(): string {
    if (this.path) {
      return `ConfigError in ${this.path}: ${this.message}`;
    }
    return `ConfigError: ${this.message}`;
  }
}

export class ConfigReadError extends ConfigError {
  constructor(path: string, cause: Error) {
    super(`Failed to read configuration file`, path, cause);
    this.name = "ConfigReadError";
    Object.setPrototypeOf(this, ConfigReadError.prototype);
  }
}

export class ConfigParseError extends ConfigError {
  constructor(
    message: string,
    path: string,
    public readonly line?: number
  ) {
    super(message, path);
    this.name = "ConfigParseError";
    Object.setPrototypeOf(this, ConfigParseError.prototype);
  }

  toString(): string {
    if (this.line) {
      return `${this.name} in ${this.path} at line ${this.line}: ${this.message}`;
    }
    return super.toString();
  }
}

export class ValidationError extends Error {
  constructor(
    public readonly field: string,
    public readonly reason: string
  ) {
    super(`Validation failed for '${field}': ${reason}`);
    this.name = "ValidationError";
    Object.setPrototypeOf(this, ValidationError.prototype);
  }
}
```

**Key Features:**
- Extend Error class
- Set `name` property
- Call `Object.setPrototypeOf` for instanceof checks
- Include context as public readonly properties
- Override `toString()` for custom formatting

<!-- /LANG -->

---

## Error Handling Patterns

Use consistent patterns for handling and propagating errors throughout your codebase.

<!-- LANG:rust -->

### Rust Error Patterns

**Pattern 1: Using the ? Operator**

```rust
pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadError {
            path: path.to_string(),
            source: e,
        })?;

    let config: Config = serde_yaml::from_str(&contents)
        .map_err(|e| ConfigError::ParseError {
            reason: e.to_string(),
        })?;

    validate_config(&config)?;

    Ok(config)
}
```

**Pattern 2: Adding Context**

```rust
use anyhow::{Context, Result};

pub fn process_file(path: &str) -> Result<()> {
    let config = load_config(path)
        .context(format!("Failed to load config from {}", path))?;

    apply_config(&config)
        .context("Failed to apply configuration")?;

    Ok(())
}
```

**Pattern 3: Matching Specific Errors**

```rust
pub fn handle_config_error(path: &str) -> Result<Config, String> {
    match load_config(path) {
        Ok(config) => Ok(config),
        Err(ConfigError::ReadError { path, .. }) => {
            // Attempt fallback
            load_default_config()
        }
        Err(ConfigError::ParseError { reason }) => {
            Err(format!("Configuration syntax error: {}", reason))
        }
        Err(e) => Err(format!("Configuration error: {}", e)),
    }
}
```

**Pattern 4: Never Use unwrap() in Production**

```rust
// WRONG - will panic on error
let config = load_config(path).unwrap();

// CORRECT - handle error appropriately
let config = load_config(path)
    .expect("Config file must exist at startup");  // Only if truly unrecoverable

// BETTER - propagate error
let config = load_config(path)?;

// BEST - handle error meaningfully
let config = match load_config(path) {
    Ok(config) => config,
    Err(e) => {
        log::error!("Failed to load config: {}", e);
        return Err(ApplicationError::Startup(e.into()));
    }
};
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Error Patterns

**Pattern 1: Raise with Context**

```python
def load_config(path: str) -> Config:
    """Load configuration from file.

    Args:
        path: Path to configuration file

    Returns:
        Parsed configuration

    Raises:
        ConfigReadError: If file cannot be read
        ConfigParseError: If file format is invalid
        ValidationError: If configuration is invalid
    """
    try:
        with open(path, 'r') as f:
            data = yaml.safe_load(f)
    except IOError as e:
        raise ConfigReadError(f"Cannot read file: {e}", path) from e
    except yaml.YAMLError as e:
        raise ConfigParseError(f"Invalid YAML syntax: {e}", path) from e

    if data is None:
        raise ConfigParseError("Empty configuration file", path)

    try:
        return validate_and_parse(data)
    except ValidationError as e:
        # Re-raise with additional context
        e.path = path
        raise
```

**Pattern 2: Using Context Managers**

```python
from contextlib import contextmanager
from typing import Generator

@contextmanager
def config_context(path: str) -> Generator[Config, None, None]:
    """Context manager for config loading with cleanup.

    Args:
        path: Path to configuration file

    Yields:
        Loaded configuration

    Raises:
        ConfigError: If configuration cannot be loaded
    """
    config = None
    try:
        config = load_config(path)
        yield config
    except ConfigError:
        raise
    except Exception as e:
        raise ConfigError(f"Unexpected error: {e}", path) from e
    finally:
        if config is not None:
            config.cleanup()
```

**Pattern 3: Handling Multiple Exceptions**

```python
def safe_load_config(path: str) -> Optional[Config]:
    """Attempt to load config, returning None on failure.

    Args:
        path: Path to configuration file

    Returns:
        Loaded config or None if loading fails
    """
    try:
        return load_config(path)
    except ConfigReadError:
        logger.warning(f"Config file not found at {path}, using defaults")
        return None
    except ConfigParseError as e:
        logger.error(f"Config parse error: {e}")
        return None
    except ValidationError as e:
        logger.error(f"Config validation error: {e}")
        return None
```

**Pattern 4: Never Use Bare except**

```python
# WRONG - catches everything including KeyboardInterrupt
try:
    result = risky_operation()
except:
    pass

# CORRECT - catch specific exceptions
try:
    result = risky_operation()
except (ValueError, TypeError) as e:
    logger.error(f"Operation failed: {e}")
    raise

# BETTER - be even more specific
try:
    result = risky_operation()
except ValueError as e:
    logger.error(f"Invalid value: {e}")
    raise OperationError("Invalid input") from e
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Error Patterns

**Pattern 1: Check Every Error**

```go
func LoadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, &ReadError{Path: path, Err: err}
    }

    var config Config
    if err := yaml.Unmarshal(data, &config); err != nil {
        return nil, &ConfigError{
            Path:    path,
            Reason:  "invalid YAML format",
            Wrapped: err,
        }
    }

    if err := config.Validate(); err != nil {
        return nil, &ValidationError{
            Field:  err.Field,
            Reason: err.Reason,
        }
    }

    return &config, nil
}
```

**Pattern 2: Wrapping Errors with Context**

```go
import "fmt"

func ProcessConfig(path string) error {
    config, err := LoadConfig(path)
    if err != nil {
        return fmt.Errorf("failed to load config from %s: %w", path, err)
    }

    if err := config.Apply(); err != nil {
        return fmt.Errorf("failed to apply config: %w", err)
    }

    return nil
}
```

**Pattern 3: Type Assertions for Specific Errors**

```go
import "errors"

func HandleConfigError(path string) error {
    config, err := LoadConfig(path)
    if err != nil {
        // Check for specific error types
        var notFoundErr *NotFoundError
        if errors.As(err, &notFoundErr) {
            // Use default config if file doesn't exist
            config = DefaultConfig()
        } else {
            return fmt.Errorf("config error: %w", err)
        }
    }

    return UseConfig(config)
}
```

**Pattern 4: Error Variables for Sentinel Errors**

```go
var (
    ErrConfigNotFound = errors.New("configuration not found")
    ErrInvalidConfig  = errors.New("invalid configuration")
    ErrEmptyConfig    = errors.New("empty configuration")
)

func ValidateConfig(config *Config) error {
    if config == nil {
        return ErrConfigNotFound
    }
    if config.IsEmpty() {
        return ErrEmptyConfig
    }
    if !config.IsValid() {
        return ErrInvalidConfig
    }
    return nil
}

// Usage
if err := ValidateConfig(config); err != nil {
    if errors.Is(err, ErrConfigNotFound) {
        // Handle missing config
    }
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Error Patterns

**Pattern 1: Try-Catch with Specific Errors**

```typescript
async function loadConfig(path: string): Promise<Config> {
  let data: string;

  try {
    data = await fs.readFile(path, "utf-8");
  } catch (err) {
    throw new ConfigReadError(path, err as Error);
  }

  let parsed: unknown;
  try {
    parsed = JSON.parse(data);
  } catch (err) {
    throw new ConfigParseError(
      `Invalid JSON syntax: ${(err as Error).message}`,
      path
    );
  }

  const config = validateConfig(parsed);
  return config;
}
```

**Pattern 2: Result Type Pattern**

```typescript
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

function safeLoadConfig(path: string): Result<Config, ConfigError> {
  try {
    const config = loadConfig(path);
    return { ok: true, value: config };
  } catch (err) {
    if (err instanceof ConfigError) {
      return { ok: false, error: err };
    }
    return {
      ok: false,
      error: new ConfigError(`Unexpected error: ${err}`),
    };
  }
}

// Usage
const result = safeLoadConfig("config.json");
if (result.ok) {
  console.log("Config loaded:", result.value);
} else {
  console.error("Failed to load config:", result.error);
}
```

**Pattern 3: Error Type Guards**

```typescript
function isConfigError(error: unknown): error is ConfigError {
  return error instanceof ConfigError;
}

function isConfigReadError(error: unknown): error is ConfigReadError {
  return error instanceof ConfigReadError;
}

async function handleConfig(path: string): Promise<void> {
  try {
    const config = await loadConfig(path);
    await applyConfig(config);
  } catch (err) {
    if (isConfigReadError(err)) {
      // Handle missing file
      await useDefaultConfig();
    } else if (isConfigError(err)) {
      // Handle other config errors
      throw new ApplicationError(`Config error: ${err.message}`);
    } else {
      // Unknown error
      throw new ApplicationError(`Unexpected error: ${err}`);
    }
  }
}
```

**Pattern 4: Never Ignore Errors**

```typescript
// WRONG - promise rejection ignored
loadConfig("config.json");

// WRONG - error swallowed
loadConfig("config.json").catch(() => {});

// CORRECT - handle the error
loadConfig("config.json")
  .then(config => applyConfig(config))
  .catch(err => {
    console.error("Failed to load config:", err);
    process.exit(1);
  });

// BETTER - use async/await
try {
  const config = await loadConfig("config.json");
  await applyConfig(config);
} catch (err) {
  console.error("Failed to load config:", err);
  throw err;
}
```

<!-- /LANG -->

---

## Context and Error Propagation

Always provide sufficient context when propagating errors up the call stack.

<!-- LANG:rust -->

### Rust Context Propagation

```rust
use anyhow::{Context, Result};

pub fn load_and_apply_config(env: &str) -> Result<()> {
    let config_path = find_config_file(env)
        .context(format!("No config file found for environment: {}", env))?;

    let config = load_config(&config_path)
        .context(format!("Failed to load config from {}", config_path))?;

    apply_config(&config)
        .context("Failed to apply configuration to system")?;

    Ok(())
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Context Propagation

```python
def load_and_apply_config(env: str) -> None:
    """Load and apply configuration for environment.

    Args:
        env: Environment name (e.g., 'production', 'staging')

    Raises:
        ConfigError: If config cannot be loaded or applied
    """
    try:
        config_path = find_config_file(env)
    except FileNotFoundError as e:
        raise ConfigError(
            f"No config file found for environment '{env}'",
            None
        ) from e

    try:
        config = load_config(config_path)
    except ConfigError as e:
        raise ConfigError(
            f"Failed to load config for '{env}'",
            config_path
        ) from e

    try:
        apply_config(config)
    except Exception as e:
        raise ConfigError(
            f"Failed to apply config for '{env}'",
            config_path
        ) from e
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Context Propagation

```go
func LoadAndApplyConfig(env string) error {
    configPath, err := FindConfigFile(env)
    if err != nil {
        return fmt.Errorf("no config file found for environment %s: %w", env, err)
    }

    config, err := LoadConfig(configPath)
    if err != nil {
        return fmt.Errorf("failed to load config from %s: %w", configPath, err)
    }

    if err := ApplyConfig(config); err != nil {
        return fmt.Errorf("failed to apply config for %s: %w", env, err)
    }

    return nil
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Context Propagation

```typescript
async function loadAndApplyConfig(env: string): Promise<void> {
  let configPath: string;
  try {
    configPath = await findConfigFile(env);
  } catch (err) {
    throw new ConfigError(
      `No config file found for environment '${env}'`,
      undefined,
      err as Error
    );
  }

  let config: Config;
  try {
    config = await loadConfig(configPath);
  } catch (err) {
    throw new ConfigError(
      `Failed to load config for environment '${env}'`,
      configPath,
      err as Error
    );
  }

  try {
    await applyConfig(config);
  } catch (err) {
    throw new ConfigError(
      `Failed to apply config for environment '${env}'`,
      configPath,
      err as Error
    );
  }
}
```

<!-- /LANG -->

---

## Testing Error Conditions

Always test error paths, not just happy paths.

<!-- LANG:rust -->

### Testing Errors in Rust

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_file_not_found() {
        let result = load_config("nonexistent.yaml");

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::ReadError { path, .. } => {
                assert_eq!(path, "nonexistent.yaml");
            }
            other => panic!("Expected ReadError, got {:?}", other),
        }
    }

    #[test]
    fn test_load_config_invalid_yaml() {
        let result = load_config("tests/fixtures/invalid.yaml");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::ParseError { .. }));
    }

    #[test]
    fn test_validation_error() {
        let config = Config { name: String::new() };
        let result = validate_config(&config);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("name"));
    }
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Testing Errors in Python

```python
import pytest
from mymodule import load_config, ConfigReadError, ConfigParseError

class TestErrorHandling:
    def test_load_config_file_not_found(self):
        """Test that ConfigReadError is raised for missing file."""
        with pytest.raises(ConfigReadError) as exc_info:
            load_config("nonexistent.yaml")

        assert "nonexistent.yaml" in str(exc_info.value)
        assert exc_info.value.path == "nonexistent.yaml"

    def test_load_config_invalid_yaml(self):
        """Test that ConfigParseError is raised for invalid YAML."""
        with pytest.raises(ConfigParseError) as exc_info:
            load_config("tests/fixtures/invalid.yaml")

        assert "invalid" in str(exc_info.value).lower()

    def test_validation_error(self):
        """Test that ValidationError is raised for invalid config."""
        from mymodule import ValidationError

        with pytest.raises(ValidationError) as exc_info:
            validate_config({"name": ""})

        assert exc_info.value.field == "name"
        assert "empty" in exc_info.value.reason.lower()
```

<!-- /LANG -->

<!-- LANG:golang -->

### Testing Errors in Go

```go
func TestLoadConfig_FileNotFound(t *testing.T) {
    _, err := LoadConfig("nonexistent.yaml")

    if err == nil {
        t.Fatal("expected error for nonexistent file")
    }

    var readErr *ReadError
    if !errors.As(err, &readErr) {
        t.Errorf("expected ReadError, got %T", err)
    }

    if readErr.Path != "nonexistent.yaml" {
        t.Errorf("expected path 'nonexistent.yaml', got %s", readErr.Path)
    }
}

func TestLoadConfig_InvalidYAML(t *testing.T) {
    _, err := LoadConfig("testdata/invalid.yaml")

    if err == nil {
        t.Fatal("expected error for invalid YAML")
    }

    var configErr *ConfigError
    if !errors.As(err, &configErr) {
        t.Errorf("expected ConfigError, got %T", err)
    }
}

func TestValidation_EmptyName(t *testing.T) {
    config := &Config{Name: ""}
    err := config.Validate()

    if err == nil {
        t.Fatal("expected validation error for empty name")
    }

    var valErr *ValidationError
    if !errors.As(err, &valErr) {
        t.Errorf("expected ValidationError, got %T", err)
    }

    if valErr.Field != "name" {
        t.Errorf("expected field 'name', got %s", valErr.Field)
    }
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### Testing Errors in TypeScript

```typescript
import { describe, it, expect } from '@jest/globals';
import { loadConfig, ConfigReadError, ConfigParseError } from './config';

describe('Error Handling', () => {
    it('should throw ConfigReadError for missing file', async () => {
        await expect(loadConfig('nonexistent.json'))
            .rejects
            .toThrow(ConfigReadError);

        try {
            await loadConfig('nonexistent.json');
            fail('Should have thrown');
        } catch (err) {
            expect(err).toBeInstanceOf(ConfigReadError);
            expect((err as ConfigReadError).path).toBe('nonexistent.json');
        }
    });

    it('should throw ConfigParseError for invalid JSON', async () => {
        await expect(loadConfig('tests/fixtures/invalid.json'))
            .rejects
            .toThrow(ConfigParseError);
    });

    it('should throw ValidationError for invalid config', () => {
        const validateConfig = (config: any) => {
            if (!config.name) {
                throw new ValidationError('name', 'Name is required');
            }
        };

        expect(() => validateConfig({}))
            .toThrow(ValidationError);

        try {
            validateConfig({});
            fail('Should have thrown');
        } catch (err) {
            expect(err).toBeInstanceOf(ValidationError);
            expect((err as ValidationError).field).toBe('name');
        }
    });
});
```

<!-- /LANG -->

---

## Best Practices Summary

1. **Define Clear Error Types**: Create domain-specific error types with meaningful names
2. **Include Context**: Always include relevant information (file paths, field names, etc.)
3. **Propagate Properly**: Use language-specific mechanisms to propagate errors with context
4. **Never Ignore Errors**: Handle or propagate every error explicitly
5. **Make Errors Actionable**: Error messages should help users understand and fix problems
6. **Test Error Paths**: Write tests for all error conditions
7. **Document Errors**: Document when and why functions can fail
8. **Use Type System**: Leverage your language's type system to make errors explicit
9. **Fail Fast**: Detect and report errors as early as possible
10. **Log Appropriately**: Log errors at the appropriate level before handling or propagating
