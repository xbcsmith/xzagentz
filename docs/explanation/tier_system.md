# Tier System Explanation

## Overview

This document explains the design and rationale behind xzagentz's tier system for tool components. The tier system provides flexibility in component complexity while maintaining focused, maintainable documentation.

## What is the Tier System?

The tier system categorizes tool-specific components by complexity and target audience:

- **Essential**: Minimal, focused guidance for common use cases (100 lines)
- **Comprehensive**: Detailed, advanced guidance for complex scenarios (200 lines)

Tiers apply only to components in the `tools` category. Other categories (core, general, languages) do not use tiers.

## Why Tiers Exist

### The Problem

Tool components have highly variable complexity requirements:

- **Docker basics**: Container concepts, basic commands, simple Dockerfile (fits in 100 lines)
- **Docker advanced**: Multi-stage builds, networking, volumes, security, optimization (needs 200 lines)
- **Git basics**: Clone, commit, push, pull, branch (fits in 100 lines)
- **Git advanced**: Rebasing, submodules, hooks, bisect, reflog (needs 200 lines)

A single size limit for all tool components would either:
- Be too small (can't cover advanced tools adequately)
- Be too large (allows bloated basic tool documentation)

### The Solution

Two tiers with different size limits allow appropriate documentation depth:

**Essential Tier** (100 lines):
- Getting started quickly
- Common patterns only
- Standard configurations
- Quick reference
- 80% use cases

**Comprehensive Tier** (200 lines):
- Advanced features
- Edge cases
- Performance tuning
- Troubleshooting
- Complex configurations

## Design Principles

### 1. Progressive Disclosure

Users start with essential components and graduate to comprehensive as expertise grows:

```
Day 1: docker_essentials (basic containers)
  ↓
Week 2: docker_essentials (mastered basics)
  ↓
Month 3: docker_advanced (need optimization)
```

This prevents information overload for beginners while supporting advanced users.

### 2. Audience Targeting

Different tiers serve different audiences:

**Essential**: Developers learning the tool, infrequent users, quick reference
**Comprehensive**: DevOps engineers, power users, production optimization

Teams can select appropriate tier based on role and expertise.

### 3. Opt-In Complexity

Complexity is opt-in rather than forced:

```bash
# Start simple
xzagentz create --component docker_essentials

# Add complexity when needed
xzagentz add --component docker_advanced
```

Users consciously choose complexity level rather than getting overwhelmed by default.

### 4. Forced Focus

Size limits enforce quality:

**Essential (100 lines)**: Must focus on absolute essentials
- Forces prioritization
- Eliminates nice-to-have content
- Results in lean, focused documentation

**Comprehensive (200 lines)**: Allows depth but still constrained
- Prevents documentation bloat
- Encourages splitting into multiple components
- Maintains readability

## Implementation Details

### Tier Field in Frontmatter

```yaml
---
component:
  name: "docker_essentials"
  category: "tools"
  version: "1.0.0"
  tier: "essential"  # Required for tools category
---
```

Tier field is:
- **Required** for tools category
- **Invalid** for other categories
- **Validated** during component validation
- **Must be** either "essential" or "comprehensive"

### Size Validation

```rust
let limit = match component.category {
    "tools" => match component.tier {
        Some("essential") => 100,
        Some("comprehensive") => 200,
        _ => return Err(ValidationError::InvalidTier),
    },
    "core" => 200,
    "general" => 150,
    "languages" => 300,
    _ => return Err(ValidationError::InvalidCategory),
};

if component.line_count() > limit {
    return Err(ValidationError::SizeExceeded { actual, limit });
}
```

Tier determines size limit dynamically based on component metadata.

### Component Naming Convention

```
Essential tier: {tool}_essentials.md
Comprehensive tier: {tool}_advanced.md or {tool}_comprehensive.md

Examples:
- docker_essentials.md (100 lines)
- docker_advanced.md (200 lines)
- kubernetes_essentials.md (100 lines)
- kubernetes_advanced.md (200 lines)
```

Naming makes tier immediately obvious from filename.

## Tier Selection Guidelines

### Choose Essential When

- **Tool is straightforward**: Basic usage covers most needs
- **Audience is beginners**: Learning the tool for first time
- **Use case is common**: Standard patterns apply
- **Quick reference needed**: Users need fast lookup
- **Documentation goal**: Get productive quickly

**Examples of essential-tier tools**:
- Docker basics (run, build, push)
- Git fundamentals (clone, commit, push)
- Make basics (targets, variables)
- CI/CD essentials (basic pipeline)

### Choose Comprehensive When

- **Tool is complex**: Many advanced features exist
- **Audience is experienced**: Already know basics
- **Use case is advanced**: Edge cases, optimization, troubleshooting
- **Deep reference needed**: Complete feature coverage
- **Documentation goal**: Master the tool

**Examples of comprehensive-tier tools**:
- Docker advanced (networking, volumes, security)
- Git advanced (rebase, submodules, internals)
- Kubernetes (complex deployments, operators)
- CI/CD advanced (matrix builds, caching, security)

### Provide Both Tiers When

- **Tool has broad usage**: Beginners to experts use it
- **Progressive learning path**: Clear advancement from basic to advanced
- **Team has mixed skills**: Some beginners, some experts
- **Documentation completeness**: Want full coverage

**Examples of tools benefiting from both tiers**:
- Docker (very common, wide skill range)
- Git (universal tool, deep complexity)
- Kubernetes (starts simple, grows complex)

## Comparison with Other Categories

### Why Core/General/Languages Don't Use Tiers

**Core components** (200 lines):
- Always essential to development
- Complexity is inherent to topic
- No "basic" vs "advanced" error handling
- All developers need full guidance

**General components** (150 lines):
- Moderate complexity naturally
- Topics are inherently focused
- Single limit sufficient
- Splitting by tier would be artificial

**Language components** (300 lines):
- Language-specific content needs examples
- Idioms require demonstration
- Single comprehensive treatment better
- Language docs naturally detailed

**Tool components** (100-200 lines):
- Complexity varies dramatically by tool
- Clear basic vs advanced distinction
- Users opt into complexity
- Two tiers match usage patterns

## Evolution and Maintenance

### When to Split Essential Component

If essential component approaches 100 lines:

1. Review content for non-essential material
2. Remove advanced patterns
3. If still too large, split into multiple components:
   - `docker_containers_essentials.md`
   - `docker_images_essentials.md`

### When to Split Comprehensive Component

If comprehensive component approaches 200 lines:

1. Review for redundancy
2. Extract topics to separate components:
   - `docker_networking_advanced.md`
   - `docker_security_advanced.md`
3. Cross-reference between components

### Upgrading Essential to Comprehensive

When essential component covers basics well but users need more:

1. Create comprehensive component
2. Keep essential component unchanged
3. Add reference from essential to comprehensive
4. Users opt into advanced content

```markdown
# Docker Essentials

...basic content...

## Next Steps

For advanced Docker usage including networking, volumes, and security,
see `docker_advanced.md`.
```

## Trade-offs and Alternatives

### Alternative Considered: Single Size Limit

**Approach**: All tool components have same limit (e.g., 150 lines)

**Rejected because**:
- Too restrictive for complex tools
- Too permissive for simple tools
- No guidance for appropriate depth
- Doesn't match user needs

### Alternative Considered: No Size Limits

**Approach**: Let tool components be any size

**Rejected because**:
- Components become unwieldy
- Hard to maintain
- Information overload
- No forcing function for quality

### Alternative Considered: Three Tiers

**Approach**: Essential, intermediate, comprehensive

**Rejected because**:
- Added complexity without clear benefit
- Intermediate tier poorly defined
- Two tiers cover usage patterns
- Users prefer binary choice

### Chosen Approach: Two Tiers

**Benefits**:
- Simple binary choice
- Clear size constraints
- Matches user mental models
- Enforces quality through limits
- Flexible enough for all tools

## Usage Patterns

### Pattern 1: Starter Kit

Project includes only essential tier components:

```bash
xzagentz create --output AGENTS.md \
  --component docker_essentials \
  --component git_essentials \
  --component ci_essentials
```

**Use case**: New project, learning phase, getting started quickly

### Pattern 2: Production Grade

Project includes comprehensive tier components:

```bash
xzagentz create --output AGENTS.md \
  --component docker_advanced \
  --component kubernetes_advanced \
  --component ci_advanced
```

**Use case**: Production system, experienced team, optimization focus

### Pattern 3: Progressive Enhancement

Start with essential, add comprehensive over time:

```bash
# Sprint 1: Basics
xzagentz create --component docker_essentials

# Sprint 5: Team mastered basics
xzagentz add --component docker_advanced

# Sprint 10: Remove redundant essential
xzagentz remove --component docker_essentials
```

**Use case**: Long-lived project, team skill growth, evolving needs

### Pattern 4: Role-Based

Different tiers for different roles:

```bash
# Developer guide
xzagentz create --output DEV_GUIDE.md \
  --component docker_essentials \
  --component git_essentials

# DevOps guide
xzagentz create --output OPS_GUIDE.md \
  --component docker_advanced \
  --component kubernetes_advanced
```

**Use case**: Multiple team roles, different responsibility levels

## Metrics and Success Criteria

### Component Quality Metrics

**Essential tier quality**:
- Covers 80% of use cases
- User can complete task in 5 minutes
- No more than 3 examples
- All beginners can understand

**Comprehensive tier quality**:
- Covers 95% of use cases
- Includes troubleshooting
- Multiple detailed examples
- Experts find it useful

### Usage Metrics

Track which tier is used more:
- Essential tier dominates → Good (appropriate defaults)
- Comprehensive tier dominates → May indicate essential insufficient
- Both used equally → Good (meeting diverse needs)

## Summary

The tier system provides:
- **Flexibility**: Appropriate complexity for different needs
- **Clarity**: Clear distinction between basic and advanced
- **Quality**: Size constraints enforce focused content
- **Usability**: Users opt into complexity
- **Maintainability**: Smaller components easier to update

Tiers recognize that tools vary in complexity and users vary in expertise, providing the right level of guidance for each situation.

## Related Documentation

- Creating Components Tutorial: `docs/tutorials/creating_custom_component.md`
- Using Component Tiers Guide: `docs/how_to/use_component_tiers.md`
- Component System Design: `docs/explanation/component_system_design.md`
- Component Format Reference: `docs/reference/component_format.md`
