---
component:
    name: golang_essential
    category: languages
    version: "1.0"
    description: Essential Go guidelines and critical rules (concise version)
    tier: essential
    languages:
        - golang
---

# Go Development Guidelines (Essential)

## 1. Critical Rules

### File Extensions
- Use `.go` for all Go source files
- Use `.mod` for go.mod (module definition)
- Use `.sum` for go.sum (dependency checksums)

### Naming Conventions
- Packages: lowercase, single word (e.g., `component`, `config`, `handler`)
- Types/Interfaces: `PascalCase` (e.g., `ComponentLoader`, `ConfigReader`)
- Functions/Variables: `camelCase` or `PascalCase` (exported vs unexported)
- Constants: `PascalCase` or `camelCase` (e.g., `MaxRetries`, `defaultTimeout`)

### Quality Gates (MUST ALL PASS)
```bash
go fmt ./...                    # Format code
go vet ./...                    # Static analysis
go test ./...                   # Run tests
go test -race ./...             # Race condition detection
```

## 2. Error Handling (MANDATORY)

### Always Check Errors
```go
// BAD - ignoring error
data, _ := os.ReadFile("config.yaml")

// GOOD - handle errors
data, err := os.ReadFile("config.yaml")
if err != nil {
    return fmt.Errorf("failed to read config: %w", err)
}
```

### Use Error Wrapping
```go
// Wrap errors with context using %w
func loadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("load config from %s: %w", path, err)
    }
    
    var config Config
    if err := yaml.Unmarshal(data, &config); err != nil {
        return nil, fmt.Errorf("parse config: %w", err)
    }
    
    return &config, nil
}
```

### Custom Error Types
```go
type ValidationError struct {
    Field   string
    Message string
}

func (e *ValidationError) Error() string {
    return fmt.Sprintf("validation failed for %s: %s", e.Field, e.Message)
}
```

## 3. Testing Requirements

### Test Structure
```go
package component

import "testing"

func TestLoadComponent(t *testing.T) {
    component, err := LoadComponent("test.md")
    if err != nil {
        t.Fatalf("LoadComponent failed: %v", err)
    }
    
    if component.Name == "" {
        t.Error("component name is empty")
    }
}

func TestLoadComponentInvalidPath(t *testing.T) {
    _, err := LoadComponent("nonexistent.md")
    if err == nil {
        t.Error("expected error for invalid path")
    }
}
```

### Table-Driven Tests
```go
func TestValidateName(t *testing.T) {
    tests := []struct {
        name    string
        input   string
        wantErr bool
    }{
        {"valid name", "component", false},
        {"empty name", "", true},
        {"invalid chars", "comp@nent", true},
    }
    
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := ValidateName(tt.input)
            if (err != nil) != tt.wantErr {
                t.Errorf("ValidateName() error = %v, wantErr %v", err, tt.wantErr)
            }
        })
    }
}
```

### Test Requirements
- Write tests for ALL exported functions
- Test success cases, error cases, and edge cases
- Use table-driven tests for multiple scenarios
- Use descriptive test names: `TestFunctionName_Scenario`

## 4. Documentation Standards

### Package Documentation
```go
// Package component provides functionality for loading and managing
// markdown components.
package component
```

### Function Documentation
```go
// LoadComponent reads a component from the specified file path.
// It returns an error if the file cannot be read or parsed.
//
// Example:
//     comp, err := LoadComponent("components/core/header.md")
//     if err != nil {
//         log.Fatal(err)
//     }
func LoadComponent(path string) (*Component, error) {
    // Implementation
}
```

## 5. Common Patterns

### Use Struct Literals
```go
// PREFERRED
config := Config{
    Name:    "myapp",
    Timeout: 30 * time.Second,
    Retries: 3,
}

// AVOID
config := Config{}
config.Name = "myapp"
config.Timeout = 30 * time.Second
config.Retries = 3
```

### Return Early
```go
func process(data []byte) error {
    if len(data) == 0 {
        return errors.New("empty data")
    }
    
    if !isValid(data) {
        return errors.New("invalid data")
    }
    
    // Main logic here
    return nil
}
```

### Use defer for Cleanup
```go
func processFile(path string) error {
    f, err := os.Open(path)
    if err != nil {
        return err
    }
    defer f.Close()  // Ensures file is closed
    
    // Process file
    return nil
}
```

## 6. Concurrency Basics

### Use sync.WaitGroup
```go
var wg sync.WaitGroup

for _, item := range items {
    wg.Add(1)
    go func(i Item) {
        defer wg.Done()
        process(i)
    }(item)
}

wg.Wait()
```

### Protect Shared State
```go
type Cache struct {
    mu    sync.RWMutex
    items map[string]string
}

func (c *Cache) Get(key string) (string, bool) {
    c.mu.RLock()
    defer c.mu.RUnlock()
    val, ok := c.items[key]
    return val, ok
}

func (c *Cache) Set(key, value string) {
    c.mu.Lock()
    defer c.mu.Unlock()
    c.items[key] = value
}
```

## 7. Quick Command Reference

```bash
# Development
go build                      # Build current package
go build ./cmd/myapp          # Build specific package
go run main.go                # Run program

# Quality
go fmt ./...                  # Format all code
go vet ./...                  # Static analysis
go mod tidy                   # Clean up dependencies

# Testing
go test ./...                 # Run all tests
go test -v ./...              # Verbose output
go test -race ./...           # Race detection
go test -cover ./...          # Coverage report

# Dependencies
go get package@version        # Add dependency
go mod download               # Download dependencies
go list -m all                # List all dependencies
```

## 8. Essential Best Practices

```go
// Accept interfaces, return structs
func NewLoader(r io.Reader) *Loader {
    return &Loader{reader: r}
}

// Make zero value useful
type Config struct {
    Timeout time.Duration  // Zero value (0) is valid
}

// Use context for cancellation
func process(ctx context.Context, data []byte) error {
    select {
    case <-ctx.Done():
        return ctx.Err()
    default:
        // Process data
    }
    return nil
}
```

## 9. Validation Workflow

```text
1. Write code with package/function comments
2. Add tests (table-driven when applicable)
3. Run: go fmt ./...
4. Run: go vet ./...
5. Run: go test -race ./...
6. Run: go test -cover ./...
7. All checks MUST pass before committing
```
