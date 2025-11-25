# Documentation Cleanup Prompt

## Purpose

This prompt guides AI agents or human contributors to audit and reorganize
project documentation according to the Diataxis Framework, removing cruft while
establishing clear, maintainable structure.

## When to Use This Prompt

Use this prompt when:

- Documentation has accumulated over time without clear organization
- Historical implementation notes are mixed with user-facing documentation
- Documents are misclassified or hard to discover
- You need to reorganize documentation to follow Diataxis principles
- Documentation debt has grown and needs systematic cleanup

## The Prompt

```
Analyze all documentation in the docs/ directory and create a comprehensive cleanup plan following these steps:

## Step 1: Inventory Current Documentation

List all documentation files organized by directory. For each file, identify:
- File name and location
- Apparent purpose
- Target audience (developer, user, contributor)
- Classification (tutorial, how-to, explanation, reference, or unclear)
- Status (current, outdated, historical cruft, misclassified)

## Step 2: Identify Problems

Document these categories of issues:

### Historical Cruft
- Implementation summaries generated during development
- Phase completion reports
- Temporary planning documents
- AI agent output not intended for end users
- Development logs and progress tracking

### Misclassified Documents
Use Diataxis Framework to identify:
- Tutorials in wrong location (should be learning-oriented, step-by-step)
- How-To guides misplaced (should be task-oriented, problem-solving)
- Explanations miscategorized (should be understanding-oriented, conceptual)
- Reference material misfiled (should be information-oriented, specifications)

### Quality Issues
- Outdated content referencing removed features
- Redundant documents covering same topics
- Incomplete or stub documents
- Broken internal links
- Inconsistent formatting or naming

## Step 3: Analyze Current Codebase

Before making recommendations:
1. Examine package manifest (package.json, Cargo.toml, setup.py, etc.)
2. Review README.md to understand actual project purpose
3. Search codebase for public APIs, commands, or interfaces
4. Identify core features that need documentation
5. Verify which documented features still exist
6. Check for AGENTS.md or similar development guidelines

## Step 4: Create Cleanup Plan

Organize cleanup into phases:

### Phase 1: Delete Historical Content
List files to delete with rationale:
- Why they're not user-facing
- What value (if any) they provided during development
- Where information is preserved (git history, actual implementation)

### Phase 2: Reclassify Documents
For each misclassified document:
- Current location and classification
- Correct Diataxis category
- New location path
- Any content splitting needed

Apply Diataxis Framework rigorously:

**Tutorials** (docs/tutorials/):
- Learning-oriented: Teaching concepts
- Takes user by the hand through series of steps
- Gets learner started
- Example: "Getting Started with [Project]"

**How-To Guides** (docs/how_to/):
- Task-oriented: Solving specific problems
- Assumes some knowledge
- Shows how to accomplish goal
- Example: "How to Configure Authentication"

**Explanation** (docs/explanation/):
- Understanding-oriented: Clarifying concepts
- Discusses why, not how
- Provides context and background
- Example: "Understanding the Component System"

**Reference** (docs/reference/):
- Information-oriented: Technical specifications
- Dry, factual description
- Accurate and complete
- Example: "API Reference" or "Configuration Options"

Note: If project has AGENTS.md, follow its Diataxis categorization rules.

### Phase 3: Identify Documentation Gaps
List missing essential documentation:
- Core features without how-to guides
- Getting started tutorial if missing
- Command/API reference if incomplete
- Architecture explanation if complex system

### Phase 4: Create Documentation Index
Design new docs/README.md with:
- Quick links to common tasks
- Documentation organized by Diataxis type
- Documentation organized by feature area
- Clear navigation structure

## Step 5: Provide Implementation Checklist

Create actionable checklist with:
- [ ] Specific files to delete
- [ ] Specific files to move with source → destination
- [ ] Specific files to create with purpose
- [ ] Links to verify
- [ ] Validation steps

Include time estimates for each phase.

## Step 6: Define Success Criteria

Specify measurable outcomes:
- Target file count reduction
- Zero misclassified documents
- Complete coverage of core features
- All links functional
- Clear navigation path for common tasks

## Deliverables

1. **Comprehensive cleanup plan** saved as `docs/explanation/document_cleanup_implementation_plan.md`
   - Current state analysis
   - Problems identified
   - Phase-by-phase cleanup strategy
   - Implementation checklist
   - Success criteria
   - Timeline estimates

## Quality Standards

Ensure the plan follows these standards:
- File names use lowercase_with_underscores.md (except README.md)
- No emojis in documentation
- All code blocks specify language
- Internal links use relative paths
- Follows project's documentation conventions

For projects with AGENTS.md or similar development guidelines:
- Read AGENTS.md first to understand project-specific rules
- Follow file naming conventions (e.g., lowercase_with_underscores.md)
- Apply code quality requirements (e.g., cargo fmt, clippy for Rust)
- Use project-specific commit message formats
- Follow project's Diataxis implementation if documented
- Run validation commands specified in guidelines
```

## Usage Instructions

### For AI Agents

1. Copy the prompt above
2. Check if project has AGENTS.md or similar guidelines
3. Add project-specific context: "This project is [description]"
4. Point to docs directory: "Analyze all documentation in [path]"
5. Request deliverables: "Create the two deliverable documents"
6. Review output and iterate if needed

### For Human Contributors

1. Use prompt as checklist for manual audit
2. Work through each step systematically
3. Document findings in the plan template
4. Get review before executing deletions
5. Implement changes in phases with commits per phase

## Diataxis Decision Tree

Use this decision tree when classifying documentation:

```text
Is it a step-by-step tutorial?
├─ YES → docs/tutorials/
└─ NO
   ├─ Is it solving a specific task?
   │  ├─ YES → docs/how_to/
   │  └─ NO
   │     ├─ Is it explaining concepts/architecture?
   │     │  ├─ YES → docs/explanation/
   │     │  └─ NO
   │     │     └─ Is it reference material?
   │     │        └─ YES → docs/reference/
```

Note: If project has AGENTS.md with documented Diataxis rules, use that decision
tree instead as it contains project-specific guidance.

Reference: See AGENTS.md section "Documentation Organization (Diataxis
Framework)" for canonical project guidance.

## Common Patterns

### Historical Cruft Indicators

Files likely to be historical cruft:

- Names containing "phase", "summary", "completion"
- Implementation details from development
- Files in `implementation/` or `summaries/` subdirectories
- AI agent output logs
- Temporary planning documents
- Progress tracking documents

### Misclassification Indicators

**Wrongly in Reference**:

- Documents explaining "why" instead of "what"
- Design decision rationale
- Architecture overviews with diagrams
- System concept explanations

**Wrongly in Tutorials**:

- Task-specific recipes without teaching context
- Command reference material
- Troubleshooting guides

**Wrongly in How-To**:

- Conceptual explanations
- Getting started guides for complete beginners
- Architecture documentation

**Wrongly in Explanation**:

- Step-by-step procedures
- Command syntax documentation
- Configuration option lists

## Example Reclassifications

### Before and After

```text
BEFORE:
docs/reference/architecture.md (explains design)

AFTER:
docs/explanation/architecture.md

REASON: Architecture document explains concepts and design
decisions, not technical specifications.
```

```text
BEFORE:
docs/tutorials/troubleshooting.md (solves specific problems)

AFTER:
docs/how_to/troubleshooting.md

REASON: Troubleshooting is task-oriented problem-solving,
not learning-oriented teaching.
```

```text
BEFORE:
docs/explanation/api_endpoints.md (lists endpoints and parameters)

AFTER:
docs/reference/api_reference.md

REASON: API endpoints are technical specifications,
not conceptual explanations.
```

## Validation Checklist

After implementing cleanup plan, verify:

- [ ] No historical implementation summaries remain in docs/
- [ ] Each document is in correct Diataxis category
- [ ] File names follow project conventions
- [ ] All internal links work (no broken references)
- [ ] Every core feature has at least one how-to guide
- [ ] Getting started tutorial exists and covers installation
- [ ] Complete reference documentation for public APIs/commands
- [ ] docs/README.md provides clear navigation
- [ ] Documentation passes linting (if configured)
- [ ] New user can find answers to common questions in 2 clicks

For projects with AGENTS.md, also verify:

- [ ] File naming follows AGENTS.md rules (lowercase_with_underscores.md)
- [ ] No emojis anywhere in documentation
- [ ] All code blocks specify language
- [ ] Quality gates pass (e.g., cargo fmt, clippy for Rust projects)

## Maintenance Guidelines

After cleanup, maintain quality with:

### Documentation PR Checklist

```markdown
- [ ] Documentation in correct Diataxis category
- [ ] File name follows conventions (lowercase_with_underscores.md)
- [ ] Internal links tested
- [ ] Code examples tested
- [ ] Added to docs/README.md if new file
- [ ] No emojis or special characters in text
```

For Rust projects with AGENTS.md:

```markdown
- [ ] cargo fmt --all passed
- [ ] cargo clippy --all-targets --all-features -- -D warnings shows zero
      warnings
- [ ] All code examples compile
- [ ] Follows AGENTS.md documentation standards
```

### Quarterly Documentation Audit

Every 3 months:

1. Review each document for accuracy
2. Check for outdated references
3. Verify code examples still work
4. Update for new features
5. Archive or delete obsolete content

### New Feature Documentation Requirements

When adding features:

1. **How-To Guide**: Required for user-facing features
2. **Reference Entry**: Required for APIs, commands, configs
3. **Tutorial Update**: If affects getting started flow
4. **Explanation**: If introduces new concepts

## Pitfalls to Avoid

### Common Mistakes

1. **Keeping Everything**: Historical value ≠ user value

   - Delete implementation summaries confidently
   - Preserve in git history if needed
   - Focus on user needs, not development artifacts

2. **Inconsistent Categorization**: One document type per file

   - Do not mix tutorial and reference in same doc
   - Split documents that serve multiple purposes
   - Use Diataxis decision tree religiously

3. **Over-Documentation**: More docs ≠ better docs

   - Consolidate redundant content
   - Remove stub documents
   - Focus on quality over quantity

4. **Broken Links**: Update all references when moving files

   - Search codebase for old paths
   - Check README links
   - Test navigation flows

5. **Loss of Context**: Document why you deleted things

   - Explain in commit messages
   - Note in cleanup plan
   - Preserve institutional knowledge where relevant

6. **Ignoring Project Standards**: Each project has conventions
   - Read AGENTS.md or similar guidelines first
   - Follow project-specific file naming
   - Apply project-specific quality gates
   - Use project-specific commit formats

## Success Stories

Well-organized documentation has:

- Clear entry points for new users
- Quick reference for experienced users
- Conceptual depth for understanding
- Task-focused guides for common operations
- Easy navigation between related topics
- Minimal maintenance burden

## Project-Specific Guidelines

If the project has an AGENTS.md file or similar development guidelines:

### Critical Steps

1. **Read AGENTS.md first**: Check for documentation standards and conventions
2. **Follow file naming rules**: Use lowercase_with_underscores.md (except
   README.md)
3. **Apply quality gates**: Run specified validation commands before committing
4. **Use commit format**: Follow conventional commits or project-specific format
5. **Reference decision tree**: Use project's Diataxis guidance if provided

### Example: Rust Projects with AGENTS.md

Required validation commands:

```bash
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Required conventions:

- File names: lowercase_with_underscores.md
- No emojis anywhere in documentation
- All code blocks must specify language
- Documentation files go in docs/explanation/ by default

Commit message format:

```text
docs(category): description (JIRA-ISSUE)
```

Examples:

```text
docs: remove historical implementation summaries
docs: reorganize per diataxis framework
docs(tutorial): add getting_started guide
docs(reference): add cli_commands reference
```

## References

- Diataxis Framework: <https://diataxis.fr/>
- Write the Docs: <https://www.writethedocs.org/>
- The Documentation System: <https://documentation.divio.com/>
- AGENTS.md: Project-specific development guidelines (if present)

## Meta

This prompt document itself is:

- **Type**: How-To Guide (task-oriented: cleaning up documentation)
- **Audience**: AI agents and human contributors
- **Purpose**: Provide reusable methodology for documentation cleanup
- **Usage**: Copy prompt section and adapt to specific project
- **Following**: AGENTS.md rules for xzagentz project

Save this file as `docs/explanation/doc_cleanup_prompt.md` for reuse across
projects.
