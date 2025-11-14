# How to Use Component Tiers

## Overview

This guide explains how to use xzagentz component tiers to manage complexity and provide appropriate guidance for different use cases. The tier system helps you choose between essential and comprehensive tool components based on project needs.

## Prerequisites

- xzagentz installed and working
- Understanding of component structure
- Familiarity with tool components

## What are Component Tiers?

Component tiers categorize tool-specific components by complexity and scope:

- **Essential**: Minimal, focused guidance for common use cases (100 lines max)
- **Comprehensive**: Detailed, advanced guidance for complex scenarios (200 lines max)

Tiers only apply to the `tools` category. Core, general, and language components do not use tiers.

## When to Use Each Tier

### Essential Tier

**Use for**:
- Getting started with a tool
- Common, straightforward use cases
- Quick reference
- Standard configurations
- Beginner to intermediate users

**Characteristics**:
- 100 line limit
- Focused on basics
- Minimal examples
- Common patterns only
- Quick to read and understand

**Examples**:
- Docker basics (containers, images, basic commands)
- Git essentials (commit, push, pull, branch)
- CI/CD fundamentals (basic pipeline setup)

### Comprehensive Tier

**Use for**:
- Advanced tool features
- Complex configurations
- Edge cases and troubleshooting
- Performance optimization
- Advanced users

**Characteristics**:
- 200 line limit
- In-depth coverage
- Multiple examples
- Advanced patterns
- Detailed explanations

**Examples**:
- Docker advanced (multi-stage builds, networking, orchestration)
- Git advanced (rebasing, submodules, hooks)
- CI/CD advanced (matrix builds, caching, secrets management)

## Listing Components by Tier

### List Essential Components

```bash
xzagentz list components --category tools --tier essential
```

Shows all essential tool components.

### List Comprehensive Components

```bash
xzagentz list components --category tools --tier comprehensive
```

Shows all comprehensive tool components.

### List All Tool Components

```bash
xzagentz list components --category tools
```

Shows both tiers with tier indicators.

## Creating Essential Component

### Structure

```yaml
---
component:
  name: "docker_essentials"
  category: "tools"
  version: "1.0.0"
  tier: "essential"
  description: "Docker basics for containerization"
---

# Docker Essentials

## Overview

Basic Docker usage for containerizing applications.

## Core Commands

### Building Images

```bash
docker build -t myapp:latest .
```

### Running Containers

```bash
docker run -d -p 8080:8080 myapp:latest
```

### Managing Containers

```bash
docker ps
docker stop <container-id>
docker rm <container-id>
```

## Best Practices

- Use .dockerignore to exclude files
- Keep images small with multi-stage builds
- Tag images with versions
- Run containers as non-root user

## Common Patterns

### Basic Dockerfile

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/myapp /usr/local/bin/
CMD ["myapp"]
```
```

**Key points**:
- Tier specified in frontmatter
- Focused on basics only
- Limited to 100 lines
- Simple examples

## Creating Comprehensive Component

### Structure

```yaml
---
component:
  name: "docker_advanced"
  category: "tools"
  version: "1.0.0"
  tier: "comprehensive"
  description: "Advanced Docker patterns and optimization"
---

# Docker Advanced

## Overview

Advanced Docker usage including optimization, networking, and orchestration.

## Multi-Stage Builds

### Optimized Build

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/myapp /
CMD ["/myapp"]
```

Benefits:
- Smaller final image
- Cached dependencies
- Secure runtime

## Docker Networking

### Custom Networks

```bash
docker network create mynetwork
docker run --network mynetwork --name db postgres
docker run --network mynetwork --name api myapp
```

### Network Inspection

```bash
docker network inspect mynetwork
docker network ls
```

## Volume Management

### Named Volumes

```bash
docker volume create mydata
docker run -v mydata:/data myapp
```

### Bind Mounts

```bash
docker run -v $(pwd)/config:/config myapp
```

## Docker Compose

### Complex Stack

```yaml
version: '3.8'
services:
  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgres://db:5432/mydb
    depends_on:
      - db
  db:
    image: postgres:15
    volumes:
      - pgdata:/var/lib/postgresql/data
volumes:
  pgdata:
```

## Performance Optimization

- Layer caching strategies
- BuildKit features
- Image size reduction
- Runtime resource limits

## Security Hardening

- Non-root users
- Read-only filesystems
- Secrets management
- Image scanning
```

**Key points**:
- Tier specified as comprehensive
- Advanced topics covered
- Multiple detailed examples
- Up to 200 lines allowed

## Using Tiered Components

### Including Essential Tier

```bash
xzagentz create --output AGENTS.md --component docker_essentials
```

Use when:
- Project is straightforward
- Team is learning Docker
- Need quick reference
- Avoiding information overload

### Including Comprehensive Tier

```bash
xzagentz create --output AGENTS.md --component docker_advanced
```

Use when:
- Project has complex requirements
- Team is experienced
- Need detailed guidance
- Optimizing for production

### Including Both Tiers

```bash
xzagentz create --output AGENTS.md \
  --component docker_essentials \
  --component docker_advanced
```

Use when:
- Supporting diverse skill levels
- Progressive learning path
- Complete reference needed

## Choosing the Right Tier

### Decision Matrix

| Factor | Essential | Comprehensive |
|--------|-----------|---------------|
| Team experience | Beginner/Intermediate | Advanced |
| Project complexity | Simple | Complex |
| Time available | Limited | Ample |
| Learning goal | Quick start | Deep understanding |
| Use case | Common patterns | Edge cases |

### Questions to Ask

**Choose Essential if**:
- Do we need basic functionality only?
- Is team learning this tool?
- Do we want concise reference?
- Are we using standard patterns?

**Choose Comprehensive if**:
- Do we need advanced features?
- Is team experienced with tool?
- Do we need troubleshooting guidance?
- Are we optimizing for production?

## Tier-Specific Validation

### Validate Essential Component

```bash
xzagentz validate components/tools/docker_essentials.md
```

Checks:
- Tier field present and set to "essential"
- Content within 100 line limit
- Basic requirements met

### Validate Comprehensive Component

```bash
xzagentz validate components/tools/docker_advanced.md
```

Checks:
- Tier field present and set to "comprehensive"
- Content within 200 line limit
- Advanced content appropriate

## Common Patterns

### Progressive Enhancement

Start with essential, add comprehensive as needed:

```bash
# Phase 1: Basic setup
xzagentz create --output AGENTS.md --component docker_essentials

# Phase 2: Team gains experience
xzagentz add AGENTS.md --component docker_advanced
```

### Role-Based Selection

Different tiers for different roles:

```bash
# For developers (essentials)
xzagentz create --output DEV_GUIDE.md \
  --component docker_essentials \
  --component kubernetes_essentials

# For DevOps (comprehensive)
xzagentz create --output OPS_GUIDE.md \
  --component docker_advanced \
  --component kubernetes_advanced
```

### Project Lifecycle

Tier selection changes with project maturity:

```bash
# Early development
--component ci_essentials

# Pre-production
--component ci_comprehensive

# Production
--component ci_advanced
```

## Tier Upgrade Path

### Moving from Essential to Comprehensive

When to upgrade:
- Team mastered basics
- Hitting tool limitations
- Need advanced features
- Optimizing for scale

Process:
```bash
# Remove essential
xzagentz remove AGENTS.md --component docker_essentials

# Add comprehensive
xzagentz add AGENTS.md --component docker_advanced

# Validate result
xzagentz validate AGENTS.md
```

## Best Practices

### Component Design

**Essential tier**:
- Focus on 80% use case
- Minimal viable guidance
- Clear, simple examples
- Quick reference format

**Comprehensive tier**:
- Cover edge cases
- Multiple approaches
- Troubleshooting included
- Performance considerations

### Documentation Strategy

**Team onboarding**: Start with essential components
**Production systems**: Use comprehensive components
**Mixed experience**: Provide both tiers
**External contributors**: Essential for contributors, comprehensive for maintainers

## Troubleshooting

### Issue: Component Exceeds Tier Limit

**Error**: "Component exceeds size limit (150 lines, limit 100 for tier 'essential')"

**Solutions**:
1. Remove non-essential content
2. Change tier to comprehensive
3. Split into multiple components

### Issue: Wrong Tier Selected

**Problem**: Essential component too basic for needs

**Solution**:
```bash
# Replace with comprehensive version
xzagentz update AGENTS.md --section docker_essentials --component docker_advanced
```

### Issue: Tier Field Missing

**Error**: "Tools category component missing required 'tier' field"

**Solution**: Add tier to frontmatter
```yaml
component:
  category: "tools"
  tier: "essential"  # Add this
```

## Summary

Component tiers enable:
- Complexity management
- Appropriate guidance levels
- Progressive learning paths
- Focused documentation
- Size constraint enforcement

Choose essential for basics, comprehensive for advanced needs, or provide both for flexibility.

## Related Documentation

- Creating Custom Components: `docs/tutorials/creating_custom_component.md`
- Component Validation: `docs/how_to/validate_components.md`
- Tier System Explanation: `docs/explanation/tier_system.md`
- Component Format Reference: `docs/reference/component_format.md`
