# Creating Custom Components

## Overview

This tutorial teaches you how to create custom reusable components for xzagentz. You will learn the component structure, validation rules, language-specific sections, and best practices. By the end of this 30-minute tutorial, you will have created, tested, and validated your own component.

## Prerequisites

- Completed the Getting Started tutorial
- xzagentz installed and working
- Understanding of Markdown syntax
- Text editor with Markdown support

## What is a Component?

Components are reusable documentation units that provide development guidelines. Each component:
- Focuses on a specific topic (error handling, testing, security, etc.)
- Contains YAML frontmatter with metadata
- Supports multiple programming languages
- Enforces size limits based on category
- Can be composed into larger documents

## Step 1: Understanding Component Structure

### Basic Component Anatomy

A component has three main parts:

```markdown
---
component:
  name: "my_component"
  category: "general"
  version: "1.0.0"
  description: "Brief description"
  languages: ["rust", "python"]
---

# Component Title

## Universal Content

This content applies to all languages.

## Language-Specific Sections

<!-- RUST_START -->
### Rust Implementation

Rust-specific guidance here.
<!-- RUST_END -->

<!-- PYTHON_START -->
### Python Implementation

Python-specific guidance here.
<!-- PYTHON_END -->
```

### YAML Frontmatter

The frontmatter defines component metadata:

```yaml
---
component:
  name: "component_identifier"       # Lowercase with underscores
  category: "core"                   # core, general, languages, tools
  version: "1.0.0"                   # Semantic versioning
  description: "Short description"   # One-line summary
  languages: ["rust", "python"]      # Supported languages (optional)
  tier: "essential"                  # For tools category only
---
```

### Required Fields

- `name`: Unique identifier (lowercase, underscores only)
- `category`: Must be one of: core, general, languages, tools
- `version`: Semantic version (MAJOR.MINOR.PATCH)

### Optional Fields

- `description`: Brief summary of component purpose
- `languages`: Array of supported languages
- `tier`: For tools category - "essential" or "comprehensive"

## Step 2: Choosing Component Category

Category determines size limits and purpose.

### Core Components

Purpose: Essential development practices

Size limit: 200 lines

Examples:
- Error handling strategies
- Testing standards
- Security fundamentals

```yaml
component:
  category: "core"
```

### General Components

Purpose: Common development topics

Size limit: 150 lines

Examples:
- Code review guidelines
- Documentation standards
- Performance optimization tips

```yaml
component:
  category: "general"
```

### Language Components

Purpose: Language-specific best practices

Size limit: 300 lines

Examples:
- Rust ownership patterns
- Python async/await guidelines
- Go concurrency patterns

```yaml
component:
  category: "languages"
```

### Tool Components

Purpose: Specific tool usage and configuration

Size limit: 100 lines (essential), 200 lines (comprehensive)

Examples:
- Docker setup (essential)
- Kubernetes advanced patterns (comprehensive)
- CI/CD pipeline configuration

```yaml
component:
  category: "tools"
  tier: "essential"  # Required for tools
```

## Step 3: Creating Your First Component

Let's create a component for API design guidelines.

### Create the File

```bash
# If using custom components directory
mkdir -p ~/.config/xzagentz/components/general
cd ~/.config/xzagentz/components/general
touch api_design_guidelines.md
```

### Write the Frontmatter

```yaml
---
component:
  name: "api_design_guidelines"
  category: "general"
  version: "1.0.0"
  description: "Best practices for designing RESTful APIs"
  languages: ["rust", "python", "go", "typescript"]
---
```

### Add Universal Content

```markdown
# API Design Guidelines

## Overview

This component provides best practices for designing RESTful APIs that are consistent, maintainable, and developer-friendly.

## Core Principles

### 1. Use Consistent Naming

- Use nouns for resources (not verbs)
- Use plural forms for collections
- Use lowercase with hyphens for multi-word resources

Examples:
- Good: `/api/users`, `/api/order-items`
- Bad: `/api/getUser`, `/api/OrderItem`

### 2. HTTP Methods

- GET: Retrieve resources (read-only, idempotent)
- POST: Create new resources
- PUT: Update entire resource (idempotent)
- PATCH: Update partial resource
- DELETE: Remove resource (idempotent)

### 3. Status Codes

Use appropriate HTTP status codes:
- 200 OK: Success
- 201 Created: Resource created successfully
- 400 Bad Request: Invalid request data
- 401 Unauthorized: Authentication required
- 403 Forbidden: Authenticated but not authorized
- 404 Not Found: Resource does not exist
- 500 Internal Server Error: Server-side error

### 4. Versioning

Include API version in URL or header:
- URL: `/api/v1/users`
- Header: `Accept: application/vnd.myapi.v1+json`

## Best Practices

- Always return JSON for data responses
- Use pagination for large collections
- Implement filtering, sorting, and searching
- Provide comprehensive error messages
- Document all endpoints with OpenAPI/Swagger
```

### Add Language-Specific Sections

```markdown
## Implementation Examples

<!-- RUST_START -->
### Rust Implementation

Use Axum or Actix-web for robust API servers:

```rust
use axum::{Router, Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

async fn get_user(Path(id): Path<u64>) -> Json<User> {
    // Fetch user from database
    Json(User {
        id,
        name: "Example".to_string(),
        email: "user@example.com".to_string(),
    })
}

fn app() -> Router {
    Router::new()
        .route("/api/v1/users/:id", axum::routing::get(get_user))
}
```

Error handling pattern:

```rust
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ApiError {
    NotFound,
    BadRequest(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, &msg),
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        (status, message).into_response()
    }
}
```
<!-- RUST_END -->

<!-- PYTHON_START -->
### Python Implementation

Use FastAPI for modern Python APIs:

```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class User(BaseModel):
    id: int
    name: str
    email: str

@app.get("/api/v1/users/{user_id}")
async def get_user(user_id: int) -> User:
    # Fetch user from database
    user = fetch_user_from_db(user_id)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return user

@app.post("/api/v1/users", status_code=201)
async def create_user(user: User) -> User:
    # Validate and save user
    saved_user = save_user_to_db(user)
    return saved_user
```

Error handling pattern:

```python
from fastapi import Request
from fastapi.responses import JSONResponse

@app.exception_handler(ValueError)
async def value_error_handler(request: Request, exc: ValueError):
    return JSONResponse(
        status_code=400,
        content={"error": str(exc)},
    )
```
<!-- PYTHON_END -->

<!-- GO_START -->
### Go Implementation

Use standard library or Gin framework:

```go
package main

import (
    "github.com/gin-gonic/gin"
    "net/http"
)

type User struct {
    ID    uint64 `json:"id"`
    Name  string `json:"name"`
    Email string `json:"email"`
}

func getUser(c *gin.Context) {
    id := c.Param("id")
    // Fetch user from database
    user := User{ID: 1, Name: "Example", Email: "user@example.com"}
    c.JSON(http.StatusOK, user)
}

func createUser(c *gin.Context) {
    var user User
    if err := c.ShouldBindJSON(&user); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
        return
    }
    // Save user to database
    c.JSON(http.StatusCreated, user)
}

func main() {
    r := gin.Default()
    r.GET("/api/v1/users/:id", getUser)
    r.POST("/api/v1/users", createUser)
    r.Run()
}
```
<!-- GO_END -->
```

## Step 4: Using Language Markers

Language markers enable language-specific content extraction.

### Marker Format

```markdown
<!-- LANGUAGE_START -->
Content for specific language
<!-- LANGUAGE_END -->
```

### Supported Languages

- `RUST_START` / `RUST_END`
- `PYTHON_START` / `PYTHON_END`
- `GO_START` / `GO_END`
- `TYPESCRIPT_START` / `TYPESCRIPT_END`
- `BASH_START` / `BASH_END`

### Rules for Language Markers

1. Markers must be on their own line
2. Use uppercase for language names
3. Every START must have matching END
4. Markers cannot be nested
5. Content outside markers is universal

### Universal vs Language-Specific Content

**Universal content** (outside markers):
- Applies to all languages
- Included in all renders
- Use for language-agnostic concepts

**Language-specific content** (inside markers):
- Only included when rendering for that language
- Use for code examples, syntax, idioms
- Fallback to universal if language section missing

## Step 5: Validating Your Component

### Run Validation

```bash
xzagentz validate ~/.config/xzagentz/components/general/api_design_guidelines.md
```

### Common Validation Errors

**Missing Required Fields**:
```
Error: Missing required field 'name' in frontmatter
```
Fix: Add `name` field to frontmatter

**Invalid Category**:
```
Error: Invalid category 'custom'. Must be: core, general, languages, tools
```
Fix: Use valid category

**Size Limit Exceeded**:
```
Error: Component exceeds size limit (250 lines, limit 150)
```
Fix: Split component or move to appropriate category

**Mismatched Markers**:
```
Error: RUST_START marker without matching RUST_END
```
Fix: Ensure every START has matching END

**Invalid Version Format**:
```
Error: Version must follow semantic versioning (MAJOR.MINOR.PATCH)
```
Fix: Use format like "1.0.0"

### Detailed Validation Report

```bash
xzagentz validate api_design_guidelines.md --detailed
```

Shows:
- Line count analysis
- Marker validation results
- Size constraint status
- Frontmatter validation
- Warnings for potential issues

## Step 6: Testing Your Component

### Manual Testing

Create test document using your component:

```bash
xzagentz create --output test_api.md --component api_design_guidelines
```

Verify:
1. Component content appears in output
2. Frontmatter is correct
3. Language-specific sections work
4. Size is appropriate

### Language-Specific Rendering

Test with different languages:

```bash
# Render for Rust
xzagentz create --output test_rust.md --language rust

# Render for Python
xzagentz create --output test_python.md --language python
```

Verify:
- Correct language sections appear
- Other language sections are excluded
- Universal content is always present

### Automated Testing

If you have test suite:

```bash
cargo test component_validation -- api_design_guidelines
```

## Step 7: Component Best Practices

### Writing Effective Components

**Do**:
- Focus on single responsibility
- Use clear, concise language
- Include practical examples
- Keep within size limits
- Provide language-specific guidance where needed

**Do Not**:
- Mix multiple unrelated topics
- Use emojis (violates AGENTS.md rules)
- Exceed category size limits
- Forget frontmatter validation
- Leave markers unmatched

### Size Management

If component exceeds limits:

1. **Split into multiple components**:
   - `api_design_basics.md`
   - `api_authentication.md`
   - `api_versioning.md`

2. **Move to larger category**:
   - General (150) → Languages (300)
   - Tools Essential (100) → Comprehensive (200)

3. **Remove redundant content**:
   - Eliminate repeated examples
   - Reference external docs
   - Focus on essentials

### Documentation Standards

Follow AGENTS.md rules:
- No emojis anywhere
- Lowercase filenames with underscores
- Code blocks specify language
- Proper Markdown formatting

## Step 8: Sharing Your Component

### Local Use

Component is immediately available after creation:

```bash
xzagentz create --output project.md --component api_design_guidelines
```

### Team Distribution

Share custom components directory:

```bash
# Create team components repository
git init team-xzagentz-components
cp -r ~/.config/xzagentz/components/* team-xzagentz-components/

# Team members clone and use
git clone <repo-url>
export XZAGENTZ_COMPONENT_DIR=./team-xzagentz-components
```

### Contributing Upstream

To contribute components to xzagentz:

1. Fork xzagentz repository
2. Add component to `components/` directory
3. Run validation and tests
4. Submit pull request

## Summary

You have learned to:
- Understand component structure and anatomy
- Choose appropriate component categories
- Write YAML frontmatter correctly
- Create universal and language-specific content
- Use language markers properly
- Validate components thoroughly
- Test components with different languages
- Follow best practices for maintainable components

## What You Have Learned

- **Component Structure**: Frontmatter, universal content, language sections
- **Categories**: Core, general, languages, tools with size limits
- **Language Markers**: Creating language-specific sections
- **Validation**: Running checks and fixing errors
- **Best Practices**: Writing effective, maintainable components
- **Distribution**: Sharing components with teams

## Time Invested

Approximately 30 minutes

## Next Steps

- Read `docs/how_to/authoring_components.md` for advanced techniques
- Explore `docs/explanation/component_system_design.md` for architecture details
- Review `docs/reference/component_format.md` for complete specification
- Try creating components for your specific needs

## Additional Resources

- Component Format Reference: `docs/reference/component_format.md`
- Validation Guide: `docs/how_to/validate_components.md`
- Component Configuration: `docs/reference/component_configuration.md`
