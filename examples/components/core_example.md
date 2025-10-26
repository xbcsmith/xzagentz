---
component:
  name: code_review_practices
  category: core
  version: 2.0.0
  description: Code review standards and best practices for quality assurance
  languages:
    - rust
    - python
    - golang
    - typescript
  sections:
    - id: principles
      language_specific: false
      required: true
    - id: checklist
      language_specific: false
      required: true
    - id: review_patterns
      language_specific: true
      required: true
    - id: automation
      language_specific: true
      required: false
---

# Code Review Practices

## Core Principles

Code reviews are essential for maintaining code quality, sharing knowledge, and catching bugs early.

### Why Code Reviews Matter

1. **Catch bugs before production**: Multiple eyes spot issues that tests might miss
2. **Knowledge sharing**: Team members learn from each other
3. **Maintain standards**: Ensure code follows team conventions
4. **Improve design**: Discuss architectural decisions before merge
5. **Build trust**: Foster collaboration and collective ownership

### Review Mindset

**As a reviewer**:
- Focus on improving code, not criticizing the author
- Ask questions rather than making demands
- Provide specific, actionable feedback
- Recognize good work and patterns

**As an author**:
- Welcome feedback as learning opportunities
- Explain context and reasoning in PR description
- Keep changes focused and reasonably sized
- Respond promptly and professionally

---

## Review Checklist

Use this checklist for every code review:

### Functionality
- [ ] Code does what it claims to do
- [ ] Edge cases are handled
- [ ] Error conditions are managed appropriately
- [ ] No obvious bugs or logic errors

### Code Quality
- [ ] Code is readable and well-structured
- [ ] Functions and variables have clear names
- [ ] Complex logic is commented
- [ ] No unnecessary complexity

### Testing
- [ ] New code has appropriate tests
- [ ] Tests cover success and failure cases
- [ ] Existing tests still pass
- [ ] Test names are descriptive

### Documentation
- [ ] Public APIs are documented
- [ ] README updated if needed
- [ ] Breaking changes are noted
- [ ] Examples are provided for new features

### Security
- [ ] No sensitive data exposed
- [ ] Input validation present
- [ ] Authentication/authorization correct
- [ ] Dependencies are safe and up-to-date

### Performance
- [ ] No obvious performance issues
- [ ] Database queries are efficient
- [ ] Large data sets are handled properly
- [ ] Resource usage is reasonable

---

## Language-Specific Review Patterns

<!-- LANG:rust -->

### Rust Code Review Patterns

**Error Handling**:
```rust
// GOOD: Proper error handling with context
pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadFailed {
            path: path.to_string(),
            source: e,
        })?;

    let config: Config = serde_yaml::from_str(&contents)
        .map_err(|e| ConfigError::ParseFailed(e.to_string()))?;

    Ok(config)
}

// BAD: Using unwrap without justification
pub fn load_config(path: &str) -> Config {
    let contents = std::fs::read_to_string(path).unwrap(); // Panics on error!
    serde_yaml::from_str(&contents).unwrap()
}
```

**Ownership and Borrowing**:
```rust
// GOOD: Efficient borrowing
pub fn process_items(items: &[Item]) -> Vec<ProcessedItem> {
    items.iter().map(|item| process_item(item)).collect()
}

// BAD: Unnecessary cloning
pub fn process_items(items: Vec<Item>) -> Vec<ProcessedItem> {
    items.iter().map(|item| process_item(item.clone())).collect()
}
```

**Type Safety**:
```rust
// GOOD: Type-safe identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(u64);

pub fn get_user(id: UserId) -> Option<User> {
    // Cannot accidentally pass wrong ID type
}

// BAD: Raw primitive types
pub fn get_user(id: u64) -> Option<User> {
    // Could accidentally pass any u64
}
```

Review checklist for Rust:
- Avoid unnecessary `clone()` calls
- Use `?` operator for error propagation
- Prefer iterators over loops
- Use `&str` over `String` in function parameters
- Ensure lifetimes are minimal and clear
- Check for potential panics (`unwrap`, `expect`, index access)

<!-- /LANG:rust -->

<!-- LANG:python -->

### Python Code Review Patterns

**Type Hints**:
```python
# GOOD: Type hints for clarity
def process_data(items: list[dict[str, Any]]) -> list[ProcessedItem]:
    """Process raw data items into structured format."""
    return [ProcessedItem.from_dict(item) for item in items]

# BAD: No type information
def process_data(items):
    return [ProcessedItem.from_dict(item) for item in items]
```

**Error Handling**:
```python
# GOOD: Specific exception handling
def load_config(path: str) -> Config:
    try:
        with open(path, 'r') as f:
            data = yaml.safe_load(f)
    except FileNotFoundError:
        raise ConfigError(f"Config file not found: {path}")
    except yaml.YAMLError as e:
        raise ConfigError(f"Invalid YAML: {e}")

    return Config.from_dict(data)

# BAD: Catching all exceptions
def load_config(path: str) -> Config:
    try:
        with open(path, 'r') as f:
            data = yaml.safe_load(f)
        return Config.from_dict(data)
    except Exception:
        return None  # Silently fails!
```

**Context Managers**:
```python
# GOOD: Proper resource management
def write_data(path: str, data: str) -> None:
    with open(path, 'w') as f:
        f.write(data)
    # File automatically closed

# BAD: Manual file handling
def write_data(path: str, data: str) -> None:
    f = open(path, 'w')
    f.write(data)
    f.close()  # Might not execute if error occurs
```

Review checklist for Python:
- Type hints on all public functions
- Use context managers for resources
- Avoid bare `except:` clauses
- Follow PEP 8 style guidelines
- Use list comprehensions appropriately
- Check for mutable default arguments

<!-- /LANG:python -->

<!-- LANG:golang -->

### Go Code Review Patterns

**Error Handling**:
```go
// GOOD: Proper error handling with context
func LoadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("failed to read config file %s: %w", path, err)
    }

    var config Config
    if err := yaml.Unmarshal(data, &config); err != nil {
        return nil, fmt.Errorf("failed to parse config: %w", err)
    }

    return &config, nil
}

// BAD: Ignoring errors
func LoadConfig(path string) *Config {
    data, _ := os.ReadFile(path)  // Ignoring error!
    var config Config
    yaml.Unmarshal(data, &config)  // Ignoring error!
    return &config
}
```

**Concurrency**:
```go
// GOOD: Proper synchronization
type Cache struct {
    mu    sync.RWMutex
    items map[string]Item
}

func (c *Cache) Get(key string) (Item, bool) {
    c.mu.RLock()
    defer c.mu.RUnlock()
    item, ok := c.items[key]
    return item, ok
}

// BAD: Race condition
type Cache struct {
    items map[string]Item  // Concurrent access unsafe!
}

func (c *Cache) Get(key string) (Item, bool) {
    item, ok := c.items[key]
    return item, ok
}
```

Review checklist for Go:
- Check all error returns
- Use `defer` for cleanup
- Avoid goroutine leaks
- Ensure proper synchronization
- Follow Go conventions (gofmt, golint)
- Use interfaces appropriately

<!-- /LANG:golang -->

<!-- LANG:typescript -->

### TypeScript Code Review Patterns

**Type Safety**:
```typescript
// GOOD: Strong typing
interface Config {
    host: string;
    port: number;
    timeout: number;
}

function loadConfig(path: string): Config {
    const data = readFileSync(path, 'utf-8');
    const config = JSON.parse(data) as Config;
    return config;
}

// BAD: Using any
function loadConfig(path: string): any {
    const data = readFileSync(path, 'utf-8');
    return JSON.parse(data);  // No type safety
}
```

**Error Handling**:
```typescript
// GOOD: Explicit error types
class ConfigError extends Error {
    constructor(message: string, public readonly cause?: Error) {
        super(message);
        this.name = 'ConfigError';
    }
}

async function loadConfig(path: string): Promise<Config> {
    try {
        const data = await readFile(path, 'utf-8');
        return JSON.parse(data) as Config;
    } catch (error) {
        throw new ConfigError(
            `Failed to load config from ${path}`,
            error as Error
        );
    }
}

// BAD: Swallowing errors
async function loadConfig(path: string): Promise<Config | null> {
    try {
        const data = await readFile(path, 'utf-8');
        return JSON.parse(data);
    } catch {
        return null;  // Lost error information
    }
}
```

Review checklist for TypeScript:
- Avoid `any` type
- Use strict TypeScript configuration
- Prefer `unknown` over `any` when needed
- Handle promises properly
- Use optional chaining and nullish coalescing
- Check for proper async/await usage

<!-- /LANG:typescript -->

---

## Common Review Feedback Examples

### Constructive Feedback

**Good feedback**:
- "Consider extracting this logic into a separate function for better testability"
- "This could panic if the slice is empty. What about adding a bounds check?"
- "Great use of the builder pattern here! Makes the API very clear"

**Feedback to avoid**:
- "This is wrong" (not specific or helpful)
- "Why didn't you do it this way?" (sounds confrontational)
- "I would have done X" (not actionable)

### Asking Questions

Use questions to guide discussion:
- "What happens if this value is null?"
- "Have you considered the case where...?"
- "Could we simplify this by...?"
- "What's the reasoning behind this approach?"

### Suggesting Changes

Be specific and provide examples:
```
Instead of:
"This function is too complex"

Try:
"This function has several responsibilities. Consider splitting it:
- `validate_input()` for validation
- `process_data()` for processing
- `format_output()` for formatting

This would make each piece easier to test and understand."
```

## Summary

Effective code reviews require:
- Clear communication and mutual respect
- Focus on code quality over personal preferences
- Constructive feedback with specific examples
- Understanding of language-specific best practices
- Balance between thoroughness and pragmatism

Code reviews are opportunities for learning and improvement for both reviewers and authors.
