# Implementation Prompt - Section {{section_number}}: {{section_name}}

## Context

- **Implementation Plan**: `docs/explanations/implementation_plan.md`
- **Section**: {{section_number}} - {{section_name}}
- **Phase**: Phase {{phase_number}} ({{phase_duration}})
- **Files to Reference**: AGENTS.md, existing code

**Project Overview**:

{{project_description}}

**Phase Goal**:

{{phase_goal}}

**Section Purpose**:

{{section_purpose}}

## Critical Rules to Follow

Based on AGENTS.md, these rules apply to this section:

### 1. Markdown File Naming (CRITICAL)

- ALL markdown files MUST use `snake_case` lowercase
- ONLY exception: `README.md`
- ✅ CORRECT: `distributed_tracing_implementation.md`
- ❌ WRONG: `DistributedTracing.md`, `distributed-tracing.md`

### 2. Fenced Code Blocks (CRITICAL - MD040)

- EVERY code block MUST specify language identifier
- ✅ CORRECT: ```rust
- ❌ WRONG: ```

### 3. YAML File Extensions (CRITICAL)

- ALL YAML files MUST use `.yaml` extension
- ❌ NEVER use `.yml` extension
- ✅ CORRECT: `config.yaml`, `docker-compose.yaml`
- ❌ WRONG: `config.yml`

### 4. No Emojis (CRITICAL)

- NO emojis in code, documentation, or commit messages
- Exception: AGENTS.md uses emojis as visual markers for rules

### 5. Error Handling (CRITICAL)

- Use `Result<T, E>` for ALL recoverable errors
- Use `?` operator for error propagation
- NO `unwrap()` without justification comment
- NO `expect()` without descriptive message

### 6. Documentation (MANDATORY)

- ALL public items MUST have `///` doc comments
- Include Examples section in doc comments
- Create `docs/explanations/{{documentation_file}}` for this section
- Doc comments MUST include runnable examples

### 7. Testing (MANDATORY)

- Write tests for ALL public functions
- Test success cases, failure cases, and edge cases
- Achieve >80% code coverage
- Test names: `test_{{function}}_{{condition}}_{{expected}}`

{{#if additional_rules}}
### Additional Rules for This Section

{{#each additional_rules}}
#### {{this.title}}

{{this.content}}
{{/each}}
{{/if}}

## Your Task

Implement section {{section_number}} following these steps:

1. **Read**: Review section {{section_number}} in implementation plan
2. **Plan**: Understand deliverables and acceptance criteria
3. **Implement**: Create files and write code
4. **Test**: Run tests and verify functionality
5. **Verify**: Check AGENTS.md compliance
6. **Document**: Update documentation as needed

### Tasks to Complete

{{#each tasks}}
{{@index}}. {{this}}
{{/each}}

## Pre-Flight Checklist

Before starting implementation:

- [ ] Read section {{section_number}} completely in implementation plan
- [ ] Understand all deliverables required
- [ ] Know which files to create/modify
- [ ] Understand acceptance criteria
- [ ] Have necessary tools installed (`rustup component add clippy rustfmt`)
- [ ] Review related sections in AGENTS.md

## Implementation Checklist

During implementation, verify each step:

- [ ] Created all required files with correct names (lowercase_with_underscores.md)
- [ ] All YAML files use `.yaml` extension (not `.yml`)
- [ ] All code blocks have language identifiers
- [ ] No emojis in any files
- [ ] All public items have doc comments with examples
- [ ] Error handling uses `Result<T, E>` (no unwrap without justification)
- [ ] Tests written for all public functions
- [ ] `cargo fmt --all` applied
- [ ] `cargo check --all-targets --all-features` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` shows zero warnings
- [ ] `cargo test --all-features` passes with >80% coverage

## Deliverables for Section {{section_number}}

You must create/modify the following:

### Code Files

{{#each deliverable_files}}
- `{{this.path}}` - {{this.description}}
{{/each}}

### Test Files

{{#each test_files}}
- `{{this.path}}` - {{this.description}}
{{/each}}

### Documentation

- `docs/explanations/{{documentation_file}}` - Implementation documentation for this section

## Acceptance Criteria

Your implementation will be validated against these criteria:

{{#each acceptance_criteria}}
- [ ] {{this}}
{{/each}}

## Testing Requirements

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    {{#each test_examples}}
    #[test]
    fn {{this.name}}() {
        // {{this.description}}
    }
    {{/each}}
}
```

### Expected Test Coverage

- Minimum 80% code coverage
- All public functions tested
- Success, failure, and edge cases covered
- Doc tests included and passing

## Post-Implementation Review

After completing implementation, verify:

### Code Quality

- [ ] `cargo fmt --all` - No formatting issues
- [ ] `cargo check --all-targets --all-features` - Compiles without errors
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` - Zero warnings
- [ ] `cargo test --all-features` - All tests pass

### Documentation Quality

- [ ] Documentation file created in `docs/explanations/`
- [ ] All filenames follow lowercase_with_underscores convention
- [ ] All code blocks specify language
- [ ] No emojis in documentation
- [ ] Documentation includes: Overview, Components, Implementation Details, Testing, Examples

### AGENTS.md Compliance

- [ ] All markdown files use `.md` extension
- [ ] All YAML files use `.yaml` extension
- [ ] All markdown filenames are lowercase_with_underscores (except README.md)
- [ ] No emojis anywhere (code, docs, comments)
- [ ] All public items have doc comments
- [ ] Error handling follows best practices
- [ ] Tests achieve >80% coverage

### Commit Quality

- [ ] Branch name: `pr-{{jira_issue}}`
- [ ] Commit message follows format: `<type>(<scope>): <description> ({{jira_issue}})`
- [ ] Commit message uses imperative mood
- [ ] First line ≤72 characters

## Verification Commands

Run these commands to verify your implementation:

```bash
# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Run linter (must show zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Verify documentation file exists
ls -la docs/explanations/{{documentation_file}}

# Check for emojis (should return no matches)
grep -r "[\x{1F600}-\x{1F64F}]" src/ docs/ --exclude=AGENTS.md || echo "No emojis found - OK"

# Verify YAML extensions
find . -name "*.yml" | grep -v target | grep -v .git || echo "No .yml files - OK"

# Verify markdown filenames (except README.md)
find docs/ -name "*.md" -exec basename {} \; | grep -v "^[a-z_]*\.md$" | grep -v "README.md" || echo "All filenames valid - OK"
```

## Next Steps

After completing this section:

1. Run all verification commands
2. Ensure all checklists are complete
3. Create documentation file
4. Commit changes with proper message format
5. Mark section complete: `xzagentz prompt complete {{section_number}}`
6. Generate next prompt: `xzagentz prompt next`

## References

- Implementation Plan: `docs/explanations/implementation_plan.md` (Section {{section_number}})
- AGENTS.md Rules: `AGENTS.md`
- Project Architecture: `docs/explanations/architecture_plan.md`
{{#if related_sections}}
- Related Sections: {{#each related_sections}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{/if}}

---

**Generated by**: xzagentz v{{version}}
**Timestamp**: {{timestamp}}
**Section**: {{section_number}} - {{section_name}}
