# Phase {{phase_number}}: {{phase_title}}

## Overview

You are implementing **Phase {{phase_number}}: {{phase_title}}** of the {{project_name}} project.

**Phase Goal**: {{phase_goal}}

**Duration**: {{phase_duration}}

**Total Sections**: {{section_count}}

## Context

### Project Overview

{{project_description}}

### Phase Objectives

This phase focuses on:

{{#each phase_objectives}}
- {{this}}
{{/each}}

### Architecture Context

{{architecture_overview}}

### Dependencies

{{#if dependencies}}
This phase depends on:

{{#each dependencies}}
- {{this}}
{{/each}}
{{/if}}

## Critical Rules from AGENTS.md

### 1. File Extensions (CRITICAL)

- ALL YAML files MUST use `.yaml` extension (NEVER `.yml`)
- ALL Markdown files MUST use `.md` extension (NOT `.markdown`)

**Examples**:
```text
✅ CORRECT: config.yaml, README.md
❌ WRONG: config.yml, README.markdown
```

### 2. Markdown File Naming (CRITICAL)

- ALL markdown filenames MUST use `lowercase_with_underscores`
- ONLY exception: `README.md`

**Examples**:
```text
✅ CORRECT: distributed_tracing_implementation.md
❌ WRONG: DistributedTracing.md, distributed-tracing.md
```

### 3. No Emojis (CRITICAL)

- NO emojis in code, documentation, or commit messages
- Exception: AGENTS.md uses emojis as visual markers for rules

### 4. Code Quality Gates (MUST ALL PASS)

Every section completion requires:

```bash
# 1. Format code
cargo fmt --all

# 2. Compile check
cargo check --all-targets --all-features

# 3. Lint (zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# 4. Tests (>80% coverage)
cargo test --all-features
```

### 5. Error Handling (MANDATORY)

- Use `Result<T, E>` for ALL recoverable errors
- Use `?` operator for error propagation
- NO `unwrap()` without justification comment
- NO `expect()` without descriptive message

### 6. Documentation (MANDATORY)

- ALL public items MUST have `///` doc comments
- Doc comments MUST include Examples section
- Create `docs/explanations/phase{{phase_number}}_*.md` files
- All code blocks MUST specify language

### 7. Testing (MANDATORY)

- Write tests for ALL public functions
- Test success cases, failure cases, and edge cases
- Achieve >80% code coverage
- Test names: `test_{{function}}_{{condition}}_{{expected}}`

## Phase {{phase_number}} Sections

{{#each sections}}
### Section {{this.number}}: {{this.name}}

**Tasks**: {{this.task_count}} tasks
**Deliverables**: {{this.deliverable_count}} files
**Tests**: {{this.test_count}} tests

**Purpose**: {{this.purpose}}

**Status**: {{this.status}}

---

{{/each}}

## Phase Implementation Strategy

### Step 1: Review Complete Phase

Before starting any section:

1. Read all sections in Phase {{phase_number}}
2. Understand dependencies between sections
3. Identify common patterns and shared code
4. Plan data structures and interfaces

### Step 2: Implement Sections in Order

For each section {{section_start}} through {{section_end}}:

1. Generate section prompt: `xzagentz prompt generate --phase {{phase_number}} --section X`
2. Read and understand the section prompt completely
3. Implement the section following the prompt checklist
4. Run all quality gates (fmt, check, clippy, test)
5. Mark section complete: `xzagentz prompt complete {{phase_number}}.X`
6. Commit with proper format: `<type>(<scope>): <description> ({{jira_issue}})`

### Step 3: Phase Integration

After completing all sections:

1. Run integration tests for the phase
2. Verify all sections work together
3. Check for code duplication and refactor
4. Update phase documentation
5. Run full quality check suite

### Step 4: Phase Review

Before moving to next phase:

1. Review all acceptance criteria met
2. Verify test coverage >80%
3. Check all documentation complete
4. Run compliance verification: `xzagentz prompt verify`
5. Generate phase summary report

## Pre-Flight Checklist

Before starting Phase {{phase_number}}:

- [ ] Previous phase (if any) is complete
- [ ] All dependencies satisfied
- [ ] Development environment set up
- [ ] Necessary tools installed (`rustup component add clippy rustfmt`)
- [ ] Project structure understood
- [ ] Architecture plan reviewed
- [ ] Implementation plan for Phase {{phase_number}} read completely

## Phase Quality Checklist

During Phase {{phase_number}} implementation:

- [ ] All sections completed in order
- [ ] Each section passed all quality gates
- [ ] No clippy warnings across entire codebase
- [ ] All tests passing with >80% coverage
- [ ] All public APIs documented with examples
- [ ] All filenames follow conventions
- [ ] No emojis anywhere
- [ ] All YAML files use `.yaml` extension
- [ ] All markdown files use `.md` extension
- [ ] Code blocks specify language identifiers
- [ ] Error handling uses `Result<T, E>`
- [ ] Commits follow conventional format

## Expected Deliverables for Phase {{phase_number}}

### Code Files

{{#each deliverable_files}}
- `{{this.path}}` - {{this.description}}
{{/each}}

### Test Files

{{#each test_files}}
- `{{this.path}}` - {{this.description}}
{{/each}}

### Documentation Files

{{#each documentation_files}}
- `{{this.path}}` - {{this.description}}
{{/each}}

## Acceptance Criteria for Phase {{phase_number}}

The phase is complete when:

{{#each phase_acceptance_criteria}}
- [ ] {{this}}
{{/each}}

### Technical Requirements

- [ ] All sections implemented according to specifications
- [ ] All quality gates pass for entire phase
- [ ] Integration tests added and passing
- [ ] Phase documentation complete
- [ ] No technical debt introduced
- [ ] Code review ready

### Documentation Requirements

- [ ] Phase summary document created
- [ ] All section implementations documented
- [ ] API documentation complete
- [ ] Examples provided and tested
- [ ] README updated if needed

## Phase Verification Commands

Run these commands to verify Phase {{phase_number}} completion:

```bash
# Format all code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Lint with zero warnings
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all-features

# Check test coverage (if tarpaulin installed)
cargo tarpaulin --all-features

# Verify no emojis (should return no matches)
grep -r "[\x{1F600}-\x{1F64F}]" src/ docs/ --exclude=AGENTS.md || echo "No emojis - OK"

# Verify YAML extensions (should return no matches)
find . -name "*.yml" | grep -v target | grep -v .git || echo "No .yml files - OK"

# Verify markdown filenames (except README.md)
find docs/ -name "*.md" -exec basename {} \; | grep -v "^[a-z_]*\.md$" | grep -v "README.md" || echo "All valid - OK"

# Generate compliance report
xzagentz prompt verify --phase {{phase_number}}

# Check progress
xzagentz prompt progress
```

## Common Pitfalls for Phase {{phase_number}}

### Pitfall 1: Skipping Quality Gates

**Issue**: Implementing multiple sections before running quality checks

**Why It Fails**: Errors accumulate and become harder to fix

**Prevention**:
- Run quality gates after EACH section
- Fix ALL warnings immediately
- Don't proceed to next section with failing tests

### Pitfall 2: Using Wrong File Extensions

**Issue**: Using `.yml` instead of `.yaml`, `.markdown` instead of `.md`

**Why It Fails**: CI/CD expects specific extensions

**Prevention**:
- Always use `.yaml` for YAML files
- Always use `.md` for Markdown files
- Check with `find` commands above

### Pitfall 3: Forgetting Documentation

**Issue**: Implementing code without documentation

**Why It Fails**: Code is hard to understand and maintain

**Prevention**:
- Write doc comments as you code
- Create section documentation file immediately
- Include runnable examples in doc comments

### Pitfall 4: Inadequate Testing

**Issue**: Only testing happy path

**Why It Fails**: Code breaks with edge cases

**Prevention**:
- Test success, failure, and edge cases
- Aim for >80% coverage (verify with tarpaulin)
- Add integration tests between sections

## Progress Tracking

Track your progress through Phase {{phase_number}}:

```bash
# View current progress
xzagentz prompt progress

# Mark section complete
xzagentz prompt complete {{phase_number}}.X

# View next section
xzagentz prompt next

# Generate section prompt
xzagentz prompt generate --phase {{phase_number}} --section X
```

## Next Steps

1. Review this phase overview completely
2. Generate prompt for first section: `xzagentz prompt generate --phase {{phase_number}} --section {{first_section}}`
3. Implement section following section prompt
4. Mark section complete and move to next
5. After all sections, run phase verification
6. Generate phase summary document

## References

- Implementation Plan: `docs/explanations/implementation_plan.md` (Phase {{phase_number}})
- Architecture Plan: `docs/explanations/architecture_plan.md`
- AGENTS.md Rules: `AGENTS.md`
{{#if related_phases}}
- Related Phases: {{#each related_phases}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{/if}}

---

**Generated by**: xzagentz v{{version}}
**Timestamp**: {{timestamp}}
**Phase**: {{phase_number}} - {{phase_title}}
**Total Sections**: {{section_count}}
