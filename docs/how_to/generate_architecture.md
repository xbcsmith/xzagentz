# How to Generate Architecture

## Overview

This guide explains how to use xzagentz to generate software architecture documents with LLM assistance. You will learn to create architecture documents, refine them interactively, and use templates for different architectural patterns.

## Prerequisites

- xzagentz installed and working
- LLM API access (OpenAI, Anthropic, or compatible)
- API key configured in environment
- Basic understanding of software architecture concepts

## Quick Start

### Generate Basic Architecture

```bash
xzagentz architecture \
  --project "My API Service" \
  --description "RESTful API for user management" \
  --output architecture.md
```

This creates a complete architecture document with:
- System overview
- Component breakdown
- Layer definitions
- Technology recommendations
- Implementation guidance

### Interactive Architecture Generation

```bash
xzagentz architecture --interactive
```

Prompts you for:
- Project name
- System description
- Architecture pattern
- Technology preferences
- Additional requirements

## Understanding Architecture Command

### Basic Command Structure

```bash
xzagentz architecture [OPTIONS]
```

### Common Options

- `--project <NAME>` - Project name
- `--description <TEXT>` - System description
- `--output <FILE>` - Output file path (default: architecture.md)
- `--template <NAME>` - Architecture template to use
- `--interactive` - Interactive mode
- `--language <LANG>` - Primary programming language
- `--pattern <PATTERN>` - Architecture pattern (layered, hexagonal, microservices)

### LLM Configuration

Set API key via environment variable:

```bash
# OpenAI
export OPENAI_API_KEY="your-api-key"

# Anthropic
export ANTHROPIC_API_KEY="your-api-key"

# Custom endpoint
export LLM_API_ENDPOINT="https://custom-endpoint.com/v1"
```

## Architecture Patterns

### Layered Architecture

Generate layered architecture:

```bash
xzagentz architecture \
  --project "E-commerce Platform" \
  --description "Online shopping with payment processing" \
  --pattern layered \
  --output architecture.md
```

Creates structure:
- Presentation Layer (API, UI)
- Application Layer (use cases, services)
- Domain Layer (business logic)
- Infrastructure Layer (database, external services)

### Hexagonal Architecture

Generate hexagonal (ports and adapters):

```bash
xzagentz architecture \
  --project "Payment Service" \
  --description "Payment processing with multiple providers" \
  --pattern hexagonal \
  --output architecture.md
```

Creates structure:
- Core domain (business logic)
- Ports (interfaces)
- Adapters (implementations)
- Infrastructure (external dependencies)

### Microservices Architecture

Generate microservices design:

```bash
xzagentz architecture \
  --project "E-commerce System" \
  --description "Distributed e-commerce platform" \
  --pattern microservices \
  --output architecture.md
```

Creates structure:
- Service boundaries
- Communication patterns
- Data management
- Deployment strategy

### Event-Driven Architecture

Generate event-driven system:

```bash
xzagentz architecture \
  --project "Real-time Analytics" \
  --description "Streaming data processing pipeline" \
  --pattern event-driven \
  --output architecture.md
```

Creates structure:
- Event producers
- Event consumers
- Message broker configuration
- Event schemas

## Using Templates

### List Available Templates

```bash
xzagentz list templates
```

Shows:
- Template names
- Descriptions
- Supported patterns
- Use cases

### Using Specific Template

```bash
xzagentz architecture \
  --project "My Service" \
  --template rust_web_service \
  --output architecture.md
```

Templates provide:
- Pre-configured components
- Technology stack recommendations
- Best practices for pattern
- Implementation examples

### Custom Templates

Create custom template:

```bash
# Extract templates
xzagentz init --templates-dir ./templates

# Create custom template
vim templates/my_template.md

# Use custom template
xzagentz architecture --template my_template --output architecture.md
```

## Interactive Mode

### Starting Interactive Session

```bash
xzagentz architecture --interactive
```

### Interactive Prompts

**Step 1: Project Information**
```
Project name: My API Service
Description: RESTful API for managing user accounts and authentication
```

**Step 2: Architecture Pattern**
```
Select architecture pattern:
1. Layered (traditional n-tier)
2. Hexagonal (ports and adapters)
3. Microservices (distributed services)
4. Event-driven (message-based)
5. Custom

Choice [1-5]: 1
```

**Step 3: Technology Stack**
```
Primary language:
1. Rust
2. Python
3. Go
4. TypeScript
5. Other

Choice [1-5]: 1
```

**Step 4: Additional Components**
```
Select components to include:
[x] Database (PostgreSQL)
[x] Message Queue (Redis)
[ ] Cache Layer
[x] Authentication
[x] API Documentation

Press Enter to continue...
```

**Step 5: Review and Generate**
```
Architecture Summary:
- Project: My API Service
- Pattern: Layered
- Language: Rust
- Components: Database, Message Queue, Authentication, API Documentation

Generate architecture? [Y/n]: Y
```

## Refining Architecture

### Iterative Refinement

Generate initial architecture:

```bash
xzagentz architecture --project "My Service" --output architecture.md
```

Review and request refinements:

```bash
xzagentz architecture \
  --input architecture.md \
  --refine "Add security layer with JWT authentication" \
  --output architecture_v2.md
```

### Adding Components

Add specific components to existing architecture:

```bash
xzagentz architecture \
  --input architecture.md \
  --add-component "Caching layer with Redis" \
  --output architecture_updated.md
```

### Removing Components

Simplify architecture by removing components:

```bash
xzagentz architecture \
  --input architecture.md \
  --remove-component "Message Queue" \
  --output architecture_simplified.md
```

## Language-Specific Architecture

### Rust Architecture

```bash
xzagentz architecture \
  --project "Rust Web API" \
  --language rust \
  --description "High-performance REST API" \
  --output architecture.md
```

Includes:
- Actix-web or Axum recommendations
- Tokio async runtime
- Diesel or SQLx for database
- Error handling with thiserror
- Testing with cargo test

### Python Architecture

```bash
xzagentz architecture \
  --project "Python API" \
  --language python \
  --description "FastAPI microservice" \
  --output architecture.md
```

Includes:
- FastAPI framework
- Pydantic for validation
- SQLAlchemy for ORM
- Pytest for testing
- Uvicorn for ASGI server

### Go Architecture

```bash
xzagentz architecture \
  --project "Go Service" \
  --language go \
  --description "Microservice with gRPC" \
  --output architecture.md
```

Includes:
- Standard library patterns
- gRPC for communication
- GORM for database
- Structured logging
- Testing with standard library

## Working with Output

### Generated Architecture Structure

Typical output includes:

```markdown
# Project Architecture: My API Service

## System Overview
High-level system description...

## Architecture Pattern
Layered architecture explanation...

## Components

### Presentation Layer
- REST API endpoints
- Request/response handling
- Input validation

### Application Layer
- Use cases
- Business workflows
- Service orchestration

### Domain Layer
- Business entities
- Domain logic
- Validation rules

### Infrastructure Layer
- Database access
- External API clients
- Message queue integration

## Technology Stack
- Language: Rust
- Framework: Axum
- Database: PostgreSQL
- Message Queue: Redis

## Implementation Guide
Step-by-step implementation recommendations...

## Testing Strategy
Unit, integration, and end-to-end testing approach...

## Deployment
Containerization and deployment recommendations...
```

### Validating Generated Architecture

```bash
# Validate markdown structure
xzagentz validate architecture.md

# Check for completeness
grep "TODO\|FIXME" architecture.md
```

### Converting to Implementation Plan

Generate implementation plan from architecture:

```bash
xzagentz implementation \
  --input architecture.md \
  --output implementation_plan.md
```

## Advanced Usage

### Batch Architecture Generation

Generate multiple architectures:

```bash
#!/bin/bash
# generate-architectures.sh

PATTERNS=("layered" "hexagonal" "microservices")

for pattern in "${PATTERNS[@]}"; do
  xzagentz architecture \
    --project "My Service" \
    --pattern "$pattern" \
    --output "architecture_${pattern}.md"
done
```

### Architecture Comparison

Compare different patterns:

```bash
# Generate alternatives
xzagentz architecture --pattern layered --output arch_layered.md
xzagentz architecture --pattern hexagonal --output arch_hexagonal.md

# Review differences
diff arch_layered.md arch_hexagonal.md
```

### Version Control Integration

Track architecture evolution:

```bash
# Initial architecture
xzagentz architecture --project "My Service" --output architecture.md
git add architecture.md
git commit -m "docs: add initial architecture"

# Refine architecture
xzagentz architecture --input architecture.md --refine "Add caching" --output architecture.md
git add architecture.md
git commit -m "docs: add caching layer to architecture"
```

## Troubleshooting

### Issue: LLM API Key Not Found

**Error**: "API key not configured"

**Solution**:
```bash
# Set API key
export OPENAI_API_KEY="your-key-here"

# Or use config file
echo "OPENAI_API_KEY=your-key-here" > ~/.config/xzagentz/config.env
```

### Issue: Generation Takes Too Long

**Problem**: Architecture generation hangs or times out

**Solutions**:
1. Simplify project description
2. Use specific template
3. Check LLM API status
4. Increase timeout setting

### Issue: Generated Architecture Incomplete

**Problem**: Missing sections or components

**Solutions**:
1. Provide more detailed description
2. Use interactive mode for clarity
3. Refine output with additional context
4. Review template used

### Issue: Incorrect Technology Recommendations

**Problem**: LLM suggests inappropriate technologies

**Solutions**:
1. Specify language explicitly: `--language rust`
2. Use template matching tech stack
3. Refine with specific requirements
4. Edit generated output manually

## Best Practices

### Before Generation

1. Clearly define project scope
2. Identify key requirements
3. Choose appropriate pattern
4. Select target technology stack
5. Review available templates

### During Generation

1. Use interactive mode for complex projects
2. Provide detailed descriptions
3. Specify language and frameworks
4. Review each section as generated
5. Request refinements as needed

### After Generation

1. Validate markdown structure
2. Review technical accuracy
3. Verify component relationships
4. Check technology compatibility
5. Share with team for feedback
6. Version control architecture document

### Iterative Approach

```bash
# 1. Generate initial draft
xzagentz architecture --project "My Service" --output arch_v1.md

# 2. Review and identify gaps
vim arch_v1.md

# 3. Refine specific areas
xzagentz architecture --input arch_v1.md --refine "Add security details" --output arch_v2.md

# 4. Validate and commit
xzagentz validate arch_v2.md
git add arch_v2.md
git commit -m "docs: refined architecture with security"
```

## Integration with Workflow

### Architecture-Driven Development

```bash
# 1. Generate architecture
xzagentz architecture --project "My Service" --output architecture.md

# 2. Generate implementation plan
xzagentz implementation --input architecture.md --output plan.md

# 3. Create project structure
mkdir -p src/{api,domain,infrastructure}

# 4. Generate AGENTS.md
xzagentz create --output AGENTS.md

# 5. Start implementation
# Follow plan.md with AGENTS.md guidance
```

### Team Collaboration

```bash
# Share architecture for review
xzagentz architecture --project "Team Service" --output architecture.md
git add architecture.md
git commit -m "docs: initial architecture for review"
git push origin feature/architecture

# Team reviews and suggests changes via PR comments

# Incorporate feedback
xzagentz architecture --input architecture.md --refine "$(cat feedback.txt)" --output architecture.md
git commit -am "docs: incorporate team feedback"
```

## Summary

Architecture generation with xzagentz enables:
- Quick bootstrapping of architecture documents
- Multiple pattern support
- Language-specific recommendations
- Iterative refinement
- Template-based consistency
- LLM-assisted design

Use architecture command to accelerate design phase and ensure comprehensive documentation.

## Related Documentation

- Architecture Patterns Explanation: `docs/explanation/architecture_patterns.md`
- Implementation Plan Generation: `docs/how_to/using_implementation_command.md`
- Interactive Mode Guide: `docs/how_to/use_interactive_mode.md`
- Template System: `docs/explanation/template_system.md`
