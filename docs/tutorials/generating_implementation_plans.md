# Tutorial: Generating Implementation Plans with xzagentz

## Introduction

This tutorial will guide you through using xzagentz to automatically generate detailed implementation plans from architecture documents. The implementation planning feature uses Ollama with local LLMs to analyze your architecture documents and create phased, actionable implementation plans.

By the end of this tutorial, you will be able to:

- Set up Ollama for use with xzagentz
- Generate implementation plans from architecture documents
- Customize plan generation with various options
- Work in both interactive and automated modes

## Prerequisites

Before starting this tutorial, you need:

- xzagentz installed on your system
- Ollama installed and running
- An architecture document in Markdown format
- Basic familiarity with the command line

## Step 1: Install and Start Ollama

### Installing Ollama

First, install Ollama from the official website:

```bash
# macOS
curl -fsSL https://ollama.com/install.sh | sh

# Linux
curl -fsSL https://ollama.com/install.sh | sh

# Windows: Download from https://ollama.com
```

### Starting Ollama

Start the Ollama service:

```bash
ollama serve
```

By default, Ollama listens on `http://localhost:11434`.

### Downloading a Model

Download a model suitable for code and technical content generation:

```bash
# Recommended model (balance of speed and quality)
ollama pull llama3.2:3b

# Alternative: More capable but slower
ollama pull llama3.2:8b

# Alternative: Specialized for code
ollama pull codellama
```

Verify the model is available:

```bash
ollama list
```

## Step 2: Prepare Your Architecture Document

The implementation planner works best with well-structured architecture documents. Create or locate your architecture document in Markdown format.

### Example Architecture Document

Create a file named `docs/reference/architecture.md`:

```markdown
# Microservices E-Commerce Platform Architecture

## Overview

A scalable e-commerce platform built using microservices architecture,
designed to handle high traffic and provide reliable service to customers.

## Business Requirements

- Support 100,000 concurrent users
- 99.9% uptime SLA
- Process payments securely
- Real-time inventory management
- Order tracking and notifications

## System Components

### API Gateway

Entry point for all client requests. Handles authentication, rate limiting,
and request routing to appropriate microservices.

**Technology**: Kong or AWS API Gateway
**Responsibilities**:
- Request routing
- Authentication and authorization
- Rate limiting
- API versioning

### User Service

Manages user accounts, profiles, and authentication.

**Technology**: Node.js with PostgreSQL
**Responsibilities**:
- User registration and login
- Profile management
- Session management
- Password reset functionality

### Product Service

Manages product catalog, search, and inventory.

**Technology**: Java Spring Boot with MongoDB
**Responsibilities**:
- Product CRUD operations
- Search and filtering
- Inventory tracking
- Product recommendations

### Order Service

Handles order processing and management.

**Technology**: Python FastAPI with PostgreSQL
**Responsibilities**:
- Order creation and tracking
- Order status management
- Integration with payment service
- Order history

### Payment Service

Processes payments securely.

**Technology**: Node.js with Stripe API
**Responsibilities**:
- Payment processing
- Refund handling
- Payment method management
- PCI compliance

## Infrastructure Requirements

- Containerization with Docker
- Orchestration with Kubernetes
- Service mesh for inter-service communication
- Centralized logging with ELK stack
- Monitoring with Prometheus and Grafana
- CI/CD pipeline with GitHub Actions

## Security Requirements

- HTTPS for all communications
- JWT-based authentication
- Role-based access control (RBAC)
- Data encryption at rest and in transit
- Regular security audits
- PCI DSS compliance for payment processing

## Performance Requirements

- API response time under 200ms for 95th percentile
- Database queries under 50ms
- Support horizontal scaling
- Auto-scaling based on load
```

## Step 3: Generate Your First Implementation Plan (Interactive Mode)

Interactive mode guides you through the process with prompts and allows you to review the plan before saving.

### Run Interactive Mode

```bash
xzagentz implementation --interactive
```

### Follow the Prompts

The interactive session will guide you through:

1. **Architecture File Selection**
   ```
   ? Enter path to architecture document: docs/reference/architecture.md
   ```

2. **Model Selection**
   ```
   ? Select Ollama model:
   > llama3.2:3b (recommended)
     llama3.2:8b
     codellama
   ```

3. **Plan Options**
   ```
   ? Number of phases to generate (or press Enter for auto-detect): 5
   ? Temperature (creativity) [0.0-1.0]: 0.7
   ```

4. **Generation Progress**
   ```
   Parsing architecture document...
   Generating implementation plan with llama3.2:3b...
   [===================>] Generating...
   ```

5. **Plan Review**
   ```
   Generated Plan:

   Title: E-Commerce Platform Implementation Plan
   Phases: 5
   Estimated Duration: 12 weeks

   Phase 1: Infrastructure Setup
   Phase 2: Core Services Development
   Phase 3: Integration and Testing
   Phase 4: Security and Compliance
   Phase 5: Deployment and Monitoring

   ? Review complete plan? (y/n):
   ```

6. **Save Location**
   ```
   ? Enter output file path: plans/ecommerce_implementation.md
   ? Confirm save? (y/n): y

   Plan saved successfully to plans/ecommerce_implementation.md
   ```

## Step 4: Generate Plans in Non-Interactive Mode

For automation or CI/CD integration, use non-interactive mode with command-line arguments.

### Basic Non-Interactive Usage

```bash
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation.md \
  --model llama3.2:3b
```

### With All Options

```bash
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation_detailed.md \
  --model llama3.2:8b \
  --num-phases 7 \
  --force \
  --yes
```

### Using Custom Ollama URL

If Ollama is running on a different host or port:

```bash
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation.md \
  --ollama-url http://192.168.1.100:11434
```

### Skip Health Check

For faster execution when you know Ollama is running:

```bash
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation.md \
  --skip-health-check
```

## Step 5: Understanding the Generated Plan

The generated implementation plan will be a comprehensive Markdown document with the following structure:

### Plan Structure

```markdown
# E-Commerce Platform Implementation Plan

**Generated**: 2024-11-02 08:00:00 UTC
**Model**: llama3.2:3b
**Source**: docs/reference/architecture.md
**Version**: 1.0.0

## Overview

Brief summary of the implementation approach and key considerations.

## Phase 1: Infrastructure Setup

**Duration**: 2 weeks
**Dependencies**: None

### Tasks

#### Task 1.1: Set up Kubernetes cluster

**Description**: Configure production-ready Kubernetes cluster

**Acceptance Criteria**:
- Kubernetes cluster deployed and accessible
- Namespaces configured for each environment
- RBAC policies implemented
- Network policies configured

**Components**: Infrastructure

#### Task 1.2: Configure CI/CD pipeline

**Description**: Set up GitHub Actions for automated builds and deployments

**Acceptance Criteria**:
- GitHub Actions workflows created
- Build and test stages configured
- Deployment stages for each environment
- Secret management implemented

**Components**: CI/CD, Infrastructure

## Phase 2: Core Services Development

**Duration**: 4 weeks
**Dependencies**: Phase 1

### Tasks

[Detailed tasks for Phase 2...]
```

### Key Sections

1. **Metadata**: Information about when and how the plan was generated
2. **Overview**: High-level summary of the implementation strategy
3. **Phases**: Organized implementation phases with clear progression
4. **Tasks**: Specific, actionable tasks with acceptance criteria
5. **Dependencies**: Phase dependencies to understand execution order
6. **Components**: System components affected by each task

## Step 6: Customize Generation with Configuration File

Create a configuration file to set default options and avoid repetitive command-line arguments.

### Create Configuration File

Create `~/.config/xzagentz/config.yaml`:

```yaml
# Ollama configuration
ollama:
  base_url: "http://localhost:11434"
  default_model: "llama3.2:3b"
  timeout_seconds: 120
  max_retries: 3

# Planning defaults
planning:
  default_phases: 5
  default_output_dir: "plans"
  output_format: "markdown"

# Interactive mode preferences
interactive:
  enable_colors: true
  show_progress: true
  confirm_before_save: true
```

### Use Configuration

Now you can run commands with fewer arguments:

```bash
# Uses defaults from config file
xzagentz implementation --architecture docs/reference/architecture.md

# Override specific options
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --model llama3.2:8b  # Overrides config file
```

## Step 7: Advanced Usage Patterns

### Generate Multiple Plans with Different Models

Compare plans generated by different models:

```bash
# Generate with smaller, faster model
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation_3b.md \
  --model llama3.2:3b

# Generate with larger, more detailed model
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation_8b.md \
  --model llama3.2:8b

# Generate with code-specialized model
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation_code.md \
  --model codellama
```

### Generate Plans with Different Phase Counts

Experiment with different levels of granularity:

```bash
# High-level plan (fewer phases)
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/highlevel_plan.md \
  --num-phases 3

# Detailed plan (more phases)
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/detailed_plan.md \
  --num-phases 10
```

### Batch Processing Multiple Architecture Documents

Create a script to generate plans for multiple documents:

```bash
#!/bin/bash
# generate_all_plans.sh

for arch_file in docs/architectures/*.md; do
  base_name=$(basename "$arch_file" .md)
  echo "Generating plan for $base_name..."

  xzagentz implementation \
    --architecture "$arch_file" \
    --output "plans/${base_name}_implementation.md" \
    --yes \
    --no-progress

  echo "Completed: plans/${base_name}_implementation.md"
done
```

Make it executable and run:

```bash
chmod +x generate_all_plans.sh
./generate_all_plans.sh
```

## Step 8: Troubleshooting Common Issues

### Issue: "Failed to connect to Ollama"

**Cause**: Ollama service is not running or unreachable.

**Solution**:
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# If not running, start it
ollama serve

# If running on different port, specify URL
xzagentz implementation \
  --architecture docs/arch.md \
  --ollama-url http://localhost:YOUR_PORT
```

### Issue: "Model not found"

**Cause**: The specified model hasn't been downloaded.

**Solution**:
```bash
# List available models
ollama list

# Download the model
ollama pull llama3.2:3b

# Or use a different available model
xzagentz implementation \
  --architecture docs/arch.md \
  --model YOUR_AVAILABLE_MODEL
```

### Issue: "Failed to parse architecture document"

**Cause**: Architecture document is not valid Markdown or is empty.

**Solution**:
```bash
# Verify file exists and has content
cat docs/reference/architecture.md

# Check for common issues:
# - File is not empty
# - File has at least one heading (# Title)
# - File is valid Markdown
```

### Issue: "Generation timeout"

**Cause**: The model is taking too long to generate the plan.

**Solution**:

1. Use a smaller, faster model:
   ```bash
   xzagentz implementation \
     --architecture docs/arch.md \
     --model llama3.2:3b
   ```

2. Increase timeout in config file:
   ```yaml
   ollama:
     timeout_seconds: 300  # 5 minutes
   ```

3. Simplify your architecture document

### Issue: "Plan quality is poor"

**Cause**: Model may not be suitable or architecture document lacks detail.

**Solution**:

1. Use a larger, more capable model:
   ```bash
   xzagentz implementation \
     --architecture docs/arch.md \
     --model llama3.2:8b
   ```

2. Improve your architecture document:
   - Add more detail about components
   - Include clear requirements
   - Specify technologies and constraints
   - Add section headings for structure

3. Adjust the number of phases:
   ```bash
   xzagentz implementation \
     --architecture docs/arch.md \
     --num-phases 7  # More detailed breakdown
   ```

## Step 9: Best Practices

### Architecture Document Structure

For best results, structure your architecture documents with:

1. **Clear Overview**: Summarize the system purpose and goals
2. **Components Section**: List and describe each major component
3. **Requirements**: Functional and non-functional requirements
4. **Technology Choices**: Specify frameworks, languages, databases
5. **Integration Points**: How components interact
6. **Constraints**: Technical, business, or regulatory constraints

### Model Selection

- **llama3.2:3b**: Fast, good for iterative development and quick plans
- **llama3.2:8b**: Better quality, more detailed plans, slower
- **codellama**: Specialized for code, good for technical implementations
- **mixtral**: High quality, requires more resources

### Phase Planning

- **3-5 phases**: High-level overview, good for executive summaries
- **5-7 phases**: Balanced detail, good for project planning
- **8-10 phases**: Detailed breakdown, good for sprint planning
- **Auto-detect**: Let the LLM decide based on document complexity

### Version Control

Always commit generated plans to version control:

```bash
git add plans/implementation.md
git commit -m "docs: add generated implementation plan for microservices platform"
```

Track changes over time as architecture evolves:

```bash
# Regenerate plan
xzagentz implementation \
  --architecture docs/reference/architecture.md \
  --output plans/implementation.md \
  --force

# Review changes
git diff plans/implementation.md

# Commit if changes are valuable
git commit -am "docs: update implementation plan with new requirements"
```

## Step 10: Integration with Development Workflow

### CI/CD Integration

Add plan generation to your CI/CD pipeline:

```yaml
# .github/workflows/generate-plan.yml
name: Generate Implementation Plan

on:
  push:
    paths:
      - 'docs/reference/architecture.md'
  workflow_dispatch:

jobs:
  generate-plan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Ollama
        run: |
          curl -fsSL https://ollama.com/install.sh | sh
          ollama serve &
          sleep 10
          ollama pull llama3.2:3b

      - name: Install xzagentz
        run: cargo install xzagentz

      - name: Generate Plan
        run: |
          xzagentz implementation \
            --architecture docs/reference/architecture.md \
            --output plans/implementation.md \
            --yes

      - name: Commit Plan
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add plans/implementation.md
          git commit -m "docs: regenerate implementation plan" || echo "No changes"
          git push
```

### Pre-commit Hook

Generate plan automatically before commits:

```bash
# .git/hooks/pre-commit
#!/bin/bash

if git diff --cached --name-only | grep -q "docs/reference/architecture.md"; then
  echo "Architecture changed, regenerating implementation plan..."

  xzagentz implementation \
    --architecture docs/reference/architecture.md \
    --output plans/implementation.md \
    --yes

  git add plans/implementation.md
fi
```

## Conclusion

You now know how to:

- Set up and configure Ollama for plan generation
- Generate implementation plans in interactive and automated modes
- Customize plan generation with various options
- Troubleshoot common issues
- Integrate plan generation into your development workflow

### Next Steps

- Explore other xzagentz features for AGENTS.md file generation
- Experiment with different LLM models for varied output styles
- Create templates for your architecture documents
- Integrate plan generation into your team's workflow

### Additional Resources

- xzagentz Documentation: `docs/`
- Implementation Plan Format: `docs/reference/plan_format.md`
- Architecture Document Examples: `docs/examples/`
- Ollama Documentation: https://ollama.com/docs

## Appendix: Command Reference

### Quick Command Examples

```bash
# Interactive mode
xzagentz implementation --interactive

# Basic non-interactive
xzagentz implementation -a docs/arch.md -o plans/plan.md

# With specific model
xzagentz implementation -a docs/arch.md -o plans/plan.md -m llama3.2:8b

# With custom phases
xzagentz implementation -a docs/arch.md -o plans/plan.md -n 7

# Force overwrite
xzagentz implementation -a docs/arch.md -o plans/plan.md --force

# Skip confirmations
xzagentz implementation -a docs/arch.md -o plans/plan.md --yes

# Custom Ollama URL
xzagentz implementation -a docs/arch.md --ollama-url http://remote:11434

# Verbose output
xzagentz implementation -a docs/arch.md -o plans/plan.md -v

# Disable colors and progress
xzagentz implementation -a docs/arch.md --no-color --no-progress
```

### Configuration File Options

```yaml
# Complete configuration example
ollama:
  base_url: "http://localhost:11434"
  default_model: "llama3.2:3b"
  timeout_seconds: 120
  max_retries: 3

planning:
  default_phases: 5
  default_output_dir: "plans"
  output_format: "markdown"

interactive:
  enable_colors: true
  show_progress: true
  confirm_before_save: true
```
