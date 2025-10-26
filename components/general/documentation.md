---
component:
  name: documentation_standards
  category: general
  version: 2.0.0
  description: Documentation standards and best practices for maintainable docs
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
    - id: doc_comments
      language_specific: true
      required: true
    - id: examples
      language_specific: true
      required: true
    - id: organization
      language_specific: false
      required: true
---

# Documentation Standards

## Core Principles

Follow these universal documentation practices:

1. **Documentation is Code**: Treat docs with same care as code
2. **Write for Users**: Focus on user needs, not implementation details
3. **Keep Docs Close**: Documentation should live near the code it describes
4. **Update with Code**: Documentation changes accompany code changes
5. **Show, Don't Tell**: Use examples to demonstrate usage
6. **Make it Discoverable**: Organize docs for easy navigation
7. **Keep it Current**: Remove outdated information promptly

---

## Doc Comments

Write clear, comprehensive documentation for all public APIs.

<!-- LANG:rust -->

### Rust Doc Comments

````rust
/// Loads a component from the specified file path
///
/// This function reads the component file, validates its structure,
/// and returns a parsed Component instance.
///
/// # Arguments
///
/// * `path` - Path to the component file
///
/// # Returns
///
/// Returns `Ok(Component)` on success
///
/// # Errors
///
/// Returns error if:
/// - File doesn't exist
/// - Invalid file format
/// - Validation fails
///
/// # Examples
///
/// ```
/// use xzagentz::component::load_component;
/// use std::path::Path;
///
/// let component = load_component(Path::new("component.md"))?;
/// assert!(component.is_valid());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Panics
///
/// Does not panic under normal circumstances
pub fn load_component(path: &Path) -> Result<Component> {
    // Implementation
}
````

**Required Sections:**

- Brief description (first line)
- `# Arguments` - Function parameters
- `# Returns` - Return value description
- `# Errors` - Error conditions
- `# Examples` - Runnable code examples

**Optional Sections:**

- `# Panics` - Panic conditions
- `# Safety` - For unsafe code
- Extended description

**Module Documentation:**

````rust
//! Component loading module
//!
//! Provides functionality for loading and validating component files.
//!
//! # Examples
//!
//! ```
//! use xzagentz::component::ComponentLoader;
//!
//! let loader = ComponentLoader::new();
//! let component = loader.load("component.md")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub struct ComponentLoader {
    // Fields
}
````

<!-- /LANG -->

<!-- LANG:python -->

### Python Doc Comments

```python
def load_component(path: str) -> Component:
    """Load a component from the specified file path.

    This function reads the component file, validates its structure,
    and returns a parsed Component instance.

    Args:
        path: Path to the component file

    Returns:
        Component: The loaded component instance

    Raises:
        FileNotFoundError: If file doesn't exist
        ParseError: If file format is invalid
        ValidationError: If validation fails

    Examples:
        >>> component = load_component("component.md")
        >>> assert component.is_valid()

        Load with error handling:
        >>> try:
        ...     component = load_component("missing.md")
        ... except FileNotFoundError:
        ...     print("File not found")
    """
    # Implementation
    pass
```

**Required Sections:**

- Brief description (first line)
- `Args:` - Function parameters with types
- `Returns:` - Return value and type
- `Raises:` - Exception types and conditions
- `Examples:` - Usage examples

**Class Documentation:**

```python
class ComponentLoader:
    """Component loading and validation.

    Provides functionality for loading component files from
    the filesystem and validating their structure.

    Attributes:
        cache_enabled: Whether to cache loaded components
        validate: Whether to validate on load

    Examples:
        >>> loader = ComponentLoader()
        >>> component = loader.load("component.md")
    """

    def __init__(self, cache_enabled: bool = True):
        """Initialize the component loader.

        Args:
            cache_enabled: Enable component caching
        """
        self.cache_enabled = cache_enabled
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Doc Comments

```go
// LoadComponent loads a component from the specified file path.
//
// This function reads the component file, validates its structure,
// and returns a parsed Component instance.
//
// Parameters:
//   - path: Path to the component file
//
// Returns:
//   - *Component: The loaded component instance
//   - error: Error if loading or validation fails
//
// Errors:
//   - ErrNotFound: File doesn't exist
//   - ErrParse: Invalid file format
//   - ErrValidation: Validation failed
//
// Example:
//
//	component, err := LoadComponent("component.md")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	fmt.Println(component.Name)
func LoadComponent(path string) (*Component, error) {
    // Implementation
    return nil, nil
}
```

**Required Sections:**

- Brief description (first line)
- Extended description
- Parameters list
- Returns list
- Errors list
- Example with code block

**Package Documentation:**

```go
// Package component provides component loading and validation.
//
// This package offers functionality for loading component files
// from the filesystem and validating their structure.
//
// Example:
//
//	loader := component.NewLoader()
//	comp, err := loader.Load("component.md")
//	if err != nil {
//	    log.Fatal(err)
//	}
package component
```

**Type Documentation:**

```go
// ComponentLoader handles loading and caching of components.
//
// The loader can optionally cache loaded components and validate
// them on load.
type ComponentLoader struct {
    // CacheEnabled indicates whether caching is active
    CacheEnabled bool

    cache map[string]*Component
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Doc Comments

````typescript
/**
 * Loads a component from the specified file path.
 *
 * This function reads the component file, validates its structure,
 * and returns a parsed Component instance.
 *
 * @param path - Path to the component file
 * @returns Promise resolving to the loaded component
 * @throws {FileNotFoundError} If file doesn't exist
 * @throws {ParseError} If file format is invalid
 * @throws {ValidationError} If validation fails
 *
 * @example
 * ```typescript
 * const component = await loadComponent("component.md");
 * console.log(component.name);
 * ```
 *
 * @example Error handling
 * ```typescript
 * try {
 *   const component = await loadComponent("missing.md");
 * } catch (error) {
 *   console.error("Failed to load:", error);
 * }
 * ```
 */
export async function loadComponent(path: string): Promise<Component> {
  // Implementation
  throw new Error("Not implemented");
}
````

**Required Sections:**

- Brief description (first line)
- `@param` - Parameters with types
- `@returns` - Return value description
- `@throws` - Exception types
- `@example` - Usage examples

**Class Documentation:**

````typescript
/**
 * Component loading and validation.
 *
 * Provides functionality for loading component files from
 * the filesystem and validating their structure.
 *
 * @example
 * ```typescript
 * const loader = new ComponentLoader();
 * const component = await loader.load("component.md");
 * ```
 */
export class ComponentLoader {
  /**
   * Whether to cache loaded components.
   */
  cacheEnabled: boolean;

  /**
   * Creates a new component loader.
   *
   * @param options - Loader configuration options
   */
  constructor(options?: LoaderOptions) {
    this.cacheEnabled = options?.cache ?? true;
  }
}
````

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Doc Comments

```bash
#!/usr/bin/env bash

#######################################
# Load a component from the specified file path.
#
# This function reads the component file, validates its structure,
# and returns the component name.
#
# Arguments:
#   path - Path to the component file
#
# Returns:
#   0 on success, non-zero on error
#
# Outputs:
#   Writes component name to stdout
#
# Examples:
#   load_component "component.md"
#   name=$(load_component "component.md")
#######################################
load_component() {
    local path="$1"

    # Implementation
    echo "component_name"
    return 0
}
```

**Required Sections:**

- Brief description
- `Arguments:` - Function parameters
- `Returns:` - Exit codes
- `Outputs:` - What is written to stdout/stderr
- `Examples:` - Usage examples

**Script Header:**

```bash
#!/usr/bin/env bash
#
# component_loader.sh - Component loading utilities
#
# Description:
#   Provides functions for loading and validating component files.
#
# Usage:
#   source component_loader.sh
#   load_component "path/to/component.md"
#
# Requirements:
#   - bash 4.0+
#   - jq
#   - yq

set -euo pipefail
```

<!-- /LANG -->

---

## Examples

Write clear, runnable examples in documentation.

<!-- LANG:rust -->

### Rust Examples

````rust
/// Parse configuration
///
/// # Examples
///
/// Basic usage:
/// ```
/// use myapp::config::parse;
///
/// let config = parse("key = value")?;
/// assert_eq!(config.get("key"), Some("value"));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Hide setup with `#`:
/// ```
/// # use myapp::config::parse;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = parse("key = value")?;
/// println!("{:?}", config);
/// # Ok(())
/// # }
/// ```
///
/// Non-executing example:
/// ```no_run
/// let server = start_server()?;
/// server.listen(); // Won't actually run
/// ```
pub fn parse(input: &str) -> Result<Config> {
    // Implementation
    Ok(Config::new())
}
````

**Run doc tests:**

```bash
cargo test --doc
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Examples

```python
def parse(input_str: str) -> Config:
    """Parse configuration string.

    Examples:
        Basic usage:
        >>> config = parse("key = value")
        >>> config.get("key")
        'value'

        Error handling:
        >>> try:
        ...     config = parse("invalid")
        ... except ParseError as e:
        ...     print(f"Error: {e}")

        Skip test:
        >>> parse("skip")  # doctest: +SKIP
    """
    return Config()
```

**Run doctests:**

```bash
python -m doctest module.py
pytest --doctest-modules
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Examples

```go
// Parse parses a configuration string.
//
// Example:
//
//	config, err := Parse("key = value")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	fmt.Println(config.Get("key"))
//	// Output: value
func Parse(input string) (*Config, error) {
    return &Config{}, nil
}
```

**Example Tests:**

```go
func ExampleParse() {
    config, _ := Parse("key = value")
    fmt.Println(config.Get("key"))
    // Output: value
}

func ExampleParse_error() {
    _, err := Parse("invalid")
    fmt.Println(err)
    // Output: parse error
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Examples

````typescript
/**
 * Parse configuration string.
 *
 * @example Basic usage
 * ```typescript
 * const config = parse("key = value");
 * console.log(config.get("key")); // "value"
 * ```
 *
 * @example Error handling
 * ```typescript
 * try {
 *   const config = parse("invalid");
 * } catch (error) {
 *   console.error("Parse error:", error);
 * }
 * ```
 */
export function parse(input: string): Config {
  return new Config();
}
````

**Test examples:**

```typescript
import { describe, it, expect } from "vitest";

describe("parse examples", () => {
  it("should match documentation example", () => {
    const config = parse("key = value");
    expect(config.get("key")).toBe("value");
  });
});
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Examples

```bash
#######################################
# Parse configuration string.
#
# Arguments:
#   input - Configuration string
#
# Examples:
#   config=$(parse_config "key=value")
#   echo "$config"
#
#   if parse_config "key=value" > /dev/null; then
#       echo "Valid config"
#   fi
#######################################
parse_config() {
    local input="$1"
    # Implementation
    echo "parsed_config"
}
```

**Test examples:**

```bash
test_parse_config_example() {
    local result
    result=$(parse_config "key=value")
    assertEquals "Expected parsed config" "parsed_config" "$result"
}
```

<!-- /LANG -->

---

## Documentation Organization

Organize documentation using the Diataxis framework.

### Tutorials (Learning-Oriented)

Step-by-step lessons for learning.

**Location:** `docs/tutorials/`

**Contents:**

- Getting started guides
- Learning paths
- Hands-on exercises
- Progressive examples

**Example:**

```
docs/tutorials/
├── getting_started.md
├── first_component.md
└── advanced_workflows.md
```

### How-To Guides (Task-Oriented)

Practical guides for accomplishing specific tasks.

**Location:** `docs/how_to/`

**Contents:**

- Installation steps
- Configuration guides
- Troubleshooting procedures
- Task-specific recipes

**Example:**

```
docs/how_to/
├── setup_environment.md
├── configure_logging.md
└── deploy_production.md
```

### Explanations (Understanding-Oriented)

Conceptual documentation explaining why and how things work.

**Location:** `docs/explanations/`

**Contents:**

- Architecture discussions
- Design decisions
- Implementation summaries
- Concept clarifications

**Example:**

```
docs/explanations/
├── architecture.md
├── design_decisions.md
└── phase1_implementation.md
```

### Reference (Information-Oriented)

Technical specifications and API documentation.

**Location:** `docs/reference/`

**Contents:**

- API specifications
- Configuration reference
- CLI command reference
- Error codes

**Example:**

```
docs/reference/
├── api_specification.md
├── configuration.md
└── cli_reference.md
```

### Decision Tree

```
Is it teaching a skill?
├─ YES → Tutorials
└─ NO
   ├─ Is it solving a specific task?
   │  ├─ YES → How-To Guides
   │  └─ NO
   │     ├─ Is it explaining concepts?
   │     │  ├─ YES → Explanations
   │     │  └─ NO
   │     │     └─ Is it reference material?
   │     │        └─ YES → Reference
```

---

## File Organization

### Markdown Files

**Naming Convention:**

- Use lowercase with underscores: `implementation_plan.md`
- Exception: `README.md` only

**Structure:**

```markdown
# Title

Brief description of document contents.

## Section 1

Content with code examples.

## Section 2

More content.

---

## Additional Resources

- [Link 1](url)
- [Link 2](url)
```

### Code Blocks

Always specify language:

````markdown
    ```rust
    fn main() {
        println!("Hello!");
    }
    ```
````

### Links

**Internal Links:**

```markdown
See [Installation](#installation) section.
See [API Reference](./reference/api.md).
```

**External Links:**

```markdown
[Rust Book](https://doc.rust-lang.org/book/)

[Project Repository][repo]

[repo]: https://github.com/user/project
```

---

## Maintenance

### When to Update

Update documentation when:

- Adding new features
- Changing public APIs
- Fixing bugs affecting behavior
- Making architectural changes
- Updating dependencies with breaking changes

### Version Documentation

Track changes in CHANGELOG.md:

```markdown
# Changelog

## [Unreleased]

### Added

- New feature X

### Changed

- Updated Y behavior

### Fixed

- Bug in feature Z

## [1.0.0] - 2024-01-01

Initial release
```

### Documentation in Pull Requests

**Checklist:**

- [ ] Doc comments updated
- [ ] README updated if needed
- [ ] CHANGELOG entry added
- [ ] Migration guide if breaking
- [ ] Examples updated
- [ ] All doc tests pass

---

## Style Guidelines

### Writing Style

- Use clear, concise language
- Write in present tense
- Use active voice
- Be direct and specific
- Avoid jargon
- Define acronyms on first use

### Formatting

- One sentence per line (source control friendly)
- Use bullet points for lists
- Use tables for comparisons
- Use code blocks for code
- Use blockquotes for important notes

### Good vs Bad Examples

**Good:**

```markdown
The loader validates component structure. It checks format,
extracts metadata, and reports errors.
```

**Bad:**

```markdown
The loader is responsible for the validation of component
structure, and it will check format and extract metadata,
and also report any errors that might occur.
```

---

## Quality Checklist

### For New Features

- [ ] Public API has doc comments
- [ ] Doc comments include examples
- [ ] Examples compile and run
- [ ] Module documentation updated
- [ ] README updated if user-facing
- [ ] CHANGELOG entry added
- [ ] How-to guide if complex

### Before Release

- [ ] All doc tests pass
- [ ] README is current
- [ ] CHANGELOG complete
- [ ] Migration guide if breaking
- [ ] Version numbers updated
- [ ] Links are valid
- [ ] No broken examples
