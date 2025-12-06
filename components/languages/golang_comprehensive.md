---
component:
  name: golang_comprehensive
  category: languages
  version: "1.0"
  description: Comprehensive Go development guidelines and best practices
  tier: comprehensive
  languages:
    - golang
---

# Go Language Guidelines

This section provides Go-specific guidelines for projects that include Go components.

---

## Go Version

**Recommended**: Go 1.19 or later

Check your version:

```bash
go version
```

Update Go:

```bash
# Using package manager (e.g., brew on macOS)
brew upgrade go

# Or download from https://go.dev/dl/
```

---

## Code Style and Formatting

### Use gofmt

**Auto-format code:**

```bash
gofmt -w .
```

**Or use goimports (preferred):**

```bash
goimports -w .
```

### Standard Formatting Rules

- Indentation: Tabs (not spaces)
- Brace style: K&R style (opening brace on same line)
- Line length: No hard limit, but keep reasonable
- Import grouping: stdlib, third-party, local

---

## Project Structure

### Standard Go Project Layout

```
myproject/
├── cmd/
│   └── myapp/
│       └── main.go
├── internal/
│   ├── component/
│   │   ├── component.go
│   │   └── component_test.go
│   └── config/
│       └── config.go
├── pkg/
│   └── public/
│       └── api.go
├── go.mod
├── go.sum
├── Makefile
└── README.md
```

**Directory purposes:**

- `cmd/` - Main applications
- `internal/` - Private application code
- `pkg/` - Public library code
- `api/` - API definitions (Protocol Buffers, OpenAPI)
- `web/` - Web assets, templates

---

## Naming Conventions

### Packages

- Use lowercase, single-word names
- No underscores or mixed caps
- Examples: `component`, `config`, `handler`

### Variables and Functions

- Use camelCase for unexported: `componentName`, `loadConfig`
- Use PascalCase for exported: `ComponentLoader`, `ParseTemplate`
- Acronyms should be uppercase: `HTTPServer`, `URLPath`, `IDToken`

### Constants

- Use camelCase or PascalCase (based on export)
- Group related constants in const blocks

```go
const (
    MaxRetries     = 3
    DefaultTimeout = 30 * time.Second
)
```

---

## Error Handling

### Return Errors, Don't Panic

```go
func LoadComponent(path string) (*Component, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("failed to read component %s: %w", path, err)
    }

    component, err := ParseComponent(data)
    if err != nil {
        return nil, fmt.Errorf("failed to parse component: %w", err)
    }

    return component, nil
}
```

### Custom Error Types

```go
type ComponentError struct {
    Path string
    Msg  string
    Err  error
}

func (e *ComponentError) Error() string {
    return fmt.Sprintf("component error at %s: %s", e.Path, e.Msg)
}

func (e *ComponentError) Unwrap() error {
    return e.Err
}

// Usage
func LoadComponent(path string) error {
    if _, err := os.Stat(path); err != nil {
        return &ComponentError{
            Path: path,
            Msg:  "file not found",
            Err:  err,
        }
    }
    return nil
}
```

### Error Wrapping

**Use fmt.Errorf with %w:**

```go
if err := validateComponent(comp); err != nil {
    return fmt.Errorf("validation failed: %w", err)
}
```

**Check wrapped errors:**

```go
if errors.Is(err, ErrNotFound) {
    // Handle not found
}

var compErr *ComponentError
if errors.As(err, &compErr) {
    // Handle ComponentError
}
```

---

## Interfaces

### Small Interfaces

**Prefer small, focused interfaces:**

```go
// Good
type Reader interface {
    Read(p []byte) (n int, err error)
}

type Closer interface {
    Close() error
}

type ReadCloser interface {
    Reader
    Closer
}

// Less ideal - too broad
type FileHandler interface {
    Open() error
    Read() ([]byte, error)
    Write([]byte) error
    Close() error
    Validate() error
}
```

### Accept Interfaces, Return Structs

```go
// Good
func ProcessData(r io.Reader) (*Result, error) {
    // Implementation
}

// Less flexible
func ProcessData(f *os.File) (*Result, error) {
    // Implementation
}
```

---

## Structs and Methods

### Struct Definition

```go
type Component struct {
    Name     string
    Content  string
    Category string
    metadata map[string]string // unexported
}

// Constructor function
func NewComponent(name, content string) *Component {
    return &Component{
        Name:     name,
        Content:  content,
        metadata: make(map[string]string),
    }
}
```

### Method Receivers

**Use pointer receivers when:**

- Method modifies the receiver
- Receiver is a large struct
- Consistency (if some methods need pointer, use for all)

```go
// Pointer receiver - modifies state
func (c *Component) SetCategory(category string) {
    c.Category = category
}

// Value receiver - doesn't modify
func (c Component) IsValid() bool {
    return c.Name != "" && c.Content != ""
}
```

---

## Concurrency

### Goroutines

**Use goroutines for concurrent operations:**

```go
func ProcessComponents(paths []string) ([]*Component, error) {
    results := make(chan *Component, len(paths))
    errors := make(chan error, len(paths))

    for _, path := range paths {
        go func(p string) {
            comp, err := LoadComponent(p)
            if err != nil {
                errors <- err
                return
            }
            results <- comp
        }(path)
    }

    // Collect results
    components := make([]*Component, 0, len(paths))
    for i := 0; i < len(paths); i++ {
        select {
        case comp := <-results:
            components = append(components, comp)
        case err := <-errors:
            return nil, err
        }
    }

    return components, nil
}
```

### Context for Cancellation

```go
func LoadComponentWithContext(ctx context.Context, path string) (*Component, error) {
    done := make(chan *Component)
    errChan := make(chan error)

    go func() {
        comp, err := LoadComponent(path)
        if err != nil {
            errChan <- err
            return
        }
        done <- comp
    }()

    select {
    case comp := <-done:
        return comp, nil
    case err := <-errChan:
        return nil, err
    case <-ctx.Done():
        return nil, ctx.Err()
    }
}
```

---

## Testing

### Test Function Naming

```go
package component

import "testing"

func TestComponentCreation(t *testing.T) {
    comp := NewComponent("test", "content")
    if comp.Name != "test" {
        t.Errorf("expected name 'test', got %s", comp.Name)
    }
}

func TestComponentValidation(t *testing.T) {
    tests := []struct {
        name    string
        comp    *Component
        wantErr bool
    }{
        {
            name:    "valid component",
            comp:    NewComponent("test", "content"),
            wantErr: false,
        },
        {
            name:    "empty name",
            comp:    NewComponent("", "content"),
            wantErr: true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := tt.comp.Validate()
            if (err != nil) != tt.wantErr {
                t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
            }
        })
    }
}
```

### Table-Driven Tests

```go
func TestParseComponent(t *testing.T) {
    tests := []struct {
        name    string
        input   string
        want    *Component
        wantErr bool
    }{
        {
            name:  "valid input",
            input: "name: test\ncontent: data",
            want:  &Component{Name: "test", Content: "data"},
        },
        {
            name:    "invalid input",
            input:   "invalid",
            wantErr: true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got, err := ParseComponent(tt.input)
            if (err != nil) != tt.wantErr {
                t.Errorf("ParseComponent() error = %v, wantErr %v", err, tt.wantErr)
                return
            }
            if !tt.wantErr && !reflect.DeepEqual(got, tt.want) {
                t.Errorf("ParseComponent() = %v, want %v", got, tt.want)
            }
        })
    }
}
```

### Test Coverage

```bash
go test -cover ./...
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out
```

---

## Documentation

### Package Documentation

```go
// Package component provides functionality for loading and validating
// component files from the filesystem.
//
// Basic usage:
//
//     loader := component.NewLoader()
//     comp, err := loader.Load("components/header.md")
//     if err != nil {
//         log.Fatal(err)
//     }
package component
```

### Function Documentation

```go
// LoadComponent loads a component from the specified file path.
//
// The file must exist and contain valid component data. Returns an error
// if the file cannot be read or if the content is invalid.
//
// Example:
//
//     comp, err := LoadComponent("components/core/header.md")
//     if err != nil {
//         return err
//     }
func LoadComponent(path string) (*Component, error) {
    // Implementation
}
```

---

## Dependency Management

### Go Modules

**Initialize module:**

```bash
go mod init github.com/username/project
```

**Add dependency:**

```bash
go get github.com/spf13/cobra
```

**Tidy dependencies:**

```bash
go mod tidy
```

**Vendor dependencies:**

```bash
go mod vendor
```

---

## Common Tools

### Formatting and Linting

```bash
# Format code
gofmt -w .
goimports -w .

# Lint code
golangci-lint run

# Vet code
go vet ./...

# Static analysis
staticcheck ./...
```

### Build and Run

```bash
# Build
go build -o myapp cmd/myapp/main.go

# Run
go run cmd/myapp/main.go

# Install
go install ./cmd/myapp
```

---

## Best Practices

### Defer for Cleanup

```go
func ProcessFile(path string) error {
    f, err := os.Open(path)
    if err != nil {
        return err
    }
    defer f.Close()

    // Process file
    return nil
}
```

### Zero Values

**Design structs to have useful zero values:**

```go
type Config struct {
    MaxRetries int    // 0 is reasonable default
    Timeout    int    // 0 is reasonable default
    Enabled    bool   // false is reasonable default
}

// Usage - no constructor needed
var config Config
```

### Avoid Init Functions

**Prefer explicit initialization:**

```go
// Less ideal
func init() {
    // Setup code
}

// Preferred
func NewLoader() *Loader {
    // Setup code
    return &Loader{}
}
```

---

## Additional Resources

- **Effective Go**: https://go.dev/doc/effective_go
- **Go Code Review Comments**: https://github.com/golang/go/wiki/CodeReviewComments
- **Go by Example**: https://gobyexample.com/
- **Standard Library**: https://pkg.go.dev/std
