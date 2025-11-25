# Prompt Generation and Plan Management System

A Rust-based system for managing the complete project lifecycle from
architecture planning to phased implementation with compliance verification.

This document explains the integrated system that helps you plan and execute
projects systematically while ensuring compliance with AGENTS.md rules.

## Overview

The system provides a complete project workflow:

**Planning Phase**:

- **Create architecture plans** from templates
- **Generate implementation plans** from architecture plans
- **Verify plan completeness** and structure

**Execution Phase**:

- **Generate prompts** for each implementation section
- **Track progress** across multiple sessions with persistent state
- **Verify compliance** with AGENTS.md rules continuously
- **Review work** systematically before moving forward

**Workflow**:

```text
1. Architecture Plan → 2. Implementation Plan → 3. Prompt Generation → 4. Phased Implementation
```

## Directory Structure

```text
xzagentz/
├── src/
│   ├── plans/
│   │   ├── mod.rs                    # Module declaration
│   │   ├── architecture.rs           # Architecture plan management
│   │   ├── implementation.rs         # Implementation plan management
│   │   ├── templates.rs              # Plan templates
│   │   └── parser.rs                 # Plan parsers
│   ├── prompts/
│   │   ├── mod.rs                    # Module declaration
│   │   ├── generator.rs              # Prompt generation logic
│   │   ├── progress.rs               # Progress tracking
│   │   ├── compliance.rs             # AGENTS.md compliance verification
│   │   └── template.rs               # Prompt template rendering
│   └── main.rs                       # CLI integration
├── templates/
│   ├── plans/
│   │   ├── architecture_plan.md      # Architecture plan template
│   │   └── implementation_plan.md    # Implementation plan template
│   └── components/                   # AGENTS.md components
├── plans/                            # Generated/working plans
│   ├── architecture_plan.md
│   └── implementation_plan.md
├── prompts/                          # Generated prompt files
│   ├── prompt_1_1.md
│   ├── prompt_1_2.md
│   └── ...
├── .implementation_progress          # Progress state file
└── docs/
    └── explanation/
        └── implementation_plan.md    # Final implementation plan
```

## Quick Start

### Complete Project Workflow

**Step 1: Create Architecture Plan**

```bash
# Create from template
xzagentz plan architecture create --template rust_binary

# Create interactive
xzagentz plan architecture create --interactive

# View and edit the plan (optionally with IDE AI assistance)
cat plans/architecture_plan.md
```

**Step 2: Create Implementation Plan from Architecture**

```bash
# Generate implementation plan from architecture
xzagentz plan implementation create --from-architecture plans/architecture_plan.md

# View and edit the plan (optionally with IDE AI assistance)
cat plans/implementation_plan.md
```

**Step 3: Generate Section Prompts**

```bash
# Generate prompts from finalized implementation plan
xzagentz prompt generate --plan plans/implementation_plan.md

# Batch mode (generate all at once)
xzagentz prompt generate --batch

# Custom output directory
xzagentz prompt generate --output custom_prompts/
```

**Step 4: Execute Phased Implementation**

```bash
# Show next section to implement
xzagentz prompt next

# View section prompt
xzagentz prompt show 1.1

# Implement section...

# Mark complete
xzagentz prompt complete 1.1

# Verify compliance
xzagentz prompt verify
```

### Quick Commands Reference

**Plan Management**:

```bash
# Architecture plans
xzagentz plan architecture create [--template NAME] [--interactive]
xzagentz plan architecture verify PLAN_FILE

# Implementation plans
xzagentz plan implementation create --from-architecture ARCH_PLAN
xzagentz plan implementation verify PLAN_FILE

# Plan utilities
xzagentz plan list-templates
xzagentz plan validate PLAN_FILE
```

**Prompt Generation**:

````bash
# Generate prompts
xzagentz prompt generate [--plan FILE] [--batch] [--interactive]

### Show Specific Prompt

Display a specific section prompt:

```bash
# Show specific section
xzagentz prompt show 1.1

# Copy to clipboard (if supported)
xzagentz prompt show 1.1 --copy

# List all available prompts
xzagentz prompt list
````

### Track Progress

Manage implementation progress:

```bash
# Show current progress
xzagentz prompt progress

# Mark section complete
xzagentz prompt complete 1.1

# Reset progress
xzagentz prompt reset

# Resume from last section
xzagentz prompt next
```

### Verify Compliance

Check AGENTS.md compliance:

```bash
# Verify all rules
xzagentz prompt verify

# Verify specific rule category
xzagentz prompt verify --category markdown

# Generate verification report
xzagentz prompt verify --report compliance_report.md

# CI/CD mode (exit code on failure)
xzagentz prompt verify --strict
```

### Command-Line Options

**Plan Commands**:

```bash
xzagentz plan <subcommand> [options]

Subcommands:
  architecture              Manage architecture plans
    create                  Create new architecture plan
    verify                  Verify architecture plan structure

  implementation            Manage implementation plans
    create                  Create new implementation plan
    verify                  Verify implementation plan structure

  list-templates            List available plan templates
  validate PLAN_FILE        Validate plan file structure

Options:
  -t, --template NAME       Use predefined template
  -f, --from-architecture   Source architecture plan file
  -i, --interactive         Interactive mode with prompts
  -o, --output FILE         Output file path
  -v, --verbose             Verbose output
  -h, --help                Show help message
```

**Prompt Commands**:

```bash
xzagentz prompt <subcommand> [options]

Options:
  -p, --plan FILE           Implementation plan file (default: plans/implementation_plan.md)
  -o, --output DIR          Output directory for prompts (default: prompts/)
  -b, --batch               Generate all prompts without interaction
  -i, --interactive         Interactive mode with progress tracking
  -v, --verbose             Verbose output
  -h, --help                Show help message
```

## Architecture

### System Overview

```text
┌────────────────────────────────────────────────────────┐
│  CLI Layer (main.rs)                                   │
│  - Command routing (plan, prompt)                      │
│  - Argument parsing                                    │
├────────────────────────────────────────────────────────┤
│  Plan Module (src/plans/)                              │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Architecture Plan Manager                        │ │
│  │ - Create from templates                          │ │
│  │ - Verify structure and completeness              │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Implementation Plan Manager                      │ │
│  │ - Generate from architecture plans               │ │
│  │ - Verify phases and deliverables                 │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Plan Templates                                   │ │
│  │ - Architecture plan templates                    │ │
│  │ - Implementation plan templates                  │ │
│  └──────────────────────────────────────────────────┘ │
├────────────────────────────────────────────────────────┤
│  Prompt Module (src/prompts/)                         │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Generator                                        │ │
│  │ - Orchestrates prompt creation from plans        │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Progress Tracker                                 │ │
│  │ - Tracks section completion state                │ │
│  └──────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────┐ │
│  │ Compliance Checker                               │ │
│  │ - Verifies AGENTS.md rules                       │ │
│  └──────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────┘
```

### Data Flow

**Complete Workflow**:

```text
┌─────────────────────────────────────────────────────────┐
│ Phase 1: Architecture Planning                          │
├─────────────────────────────────────────────────────────┤
│ 1. Create Architecture Plan                             │
│    Template → architecture.rs → plans/architecture_plan.md│
│                                                          │
│ 2. Review and Edit (optionally with IDE AI)             │
│    User edits plans/architecture_plan.md                │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ Phase 2: Implementation Planning                        │
├─────────────────────────────────────────────────────────┤
│ 3. Generate Implementation Plan                         │
│    architecture_plan.md → implementation.rs →           │
│    plans/implementation_plan.md                         │
│                                                          │
│ 4. Review and Edit (optionally with IDE AI)             │
│    User edits plans/implementation_plan.md              │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ Phase 3: Phased Execution                               │
├─────────────────────────────────────────────────────────┤
│ 5. Generate Section Prompts                             │
│    implementation_plan.md → parser.rs →                 │
│    ImplementationPlan struct → generator.rs →           │
│    prompts/prompt_X_Y.md                                │
│                                                          │
│ 6. Track Progress                                       │
│    User marks complete → progress.rs →                  │
│    .implementation_progress                             │
│                                                          │
│ 7. Verify Compliance                                    │
│    Project files → compliance.rs → Violation reports    │
└─────────────────────────────────────────────────────────┘
```

## Core Components

### Plan Management Module (src/plans/)

#### 1. Architecture Plan Manager

**Module**: `src/plans/architecture.rs`

Manages architecture plan creation and enhancement.

**Key Structures**:

```rust
pub struct ArchitecturePlan {
    pub project_name: String,
    pub overview: String,
    pub goals: Vec<String>,
    pub architecture: ArchitectureDesign,
    pub technology_stack: TechnologyStack,
    pub system_components: Vec<Component>,
    pub data_flow: DataFlow,
    pub deployment: DeploymentStrategy,
}

pub struct ArchitectureDesign {
    pub architectural_style: String,  // e.g., "Layered", "Microservices"
    pub layers: Vec<Layer>,
    pub patterns: Vec<DesignPattern>,
}
```

**Functionality**:

````rust
impl ArchitecturePlanManager {
    pub struct ArchitecturePlanManager {
        pub fn create_from_template(template_name: &str) -> Result<ArchitecturePlan>;

        pub fn create_interactive() -> Result<ArchitecturePlan>;

        pub fn verify_completeness(plan: &ArchitecturePlan) -> Result<VerificationReport>;
    }
    ```

    **Usage**:

    Users can review and enhance plans using their IDE's AI assistant by opening the generated plan file and requesting improvements directly in their development environment.

#### 2. Implementation Plan Manager

**Module**: `src/plans/implementation.rs`

Generates implementation plans from architecture plans.

**Key Structures**:

```rust
pub struct ImplementationPlanGenerator {
    architecture: ArchitecturePlan,
    config: GeneratorConfig,
}

pub struct GeneratorConfig {
    pub phase_duration_weeks: usize,
    pub include_testing: bool,
    pub include_documentation: bool,
    pub risk_mitigation: bool,
}
````

**Functionality**:

```rust
impl ImplementationPlanGenerator {
    pub fn new(architecture: ArchitecturePlan) -> Self;

    pub fn generate_plan(&self) -> Result<ImplementationPlan>;

    pub fn verify_plan_structure(plan: &ImplementationPlan) -> Result<VerificationReport>;
}
```

**Plan Generation Strategy**:

```rust
// Converts architecture to implementation phases
pub fn architecture_to_phases(arch: &ArchitecturePlan) -> Vec<Phase> {
    // 1. Foundation phase (always first)
    // 2. Layer-by-layer implementation
    // 3. Integration phase
    // 4. Testing phase
    // 5. Documentation phase
    // 6. Deployment phase
}
```

**Usage**:

After generation, users can review and refine the implementation plan using
their IDE's AI assistant for improvements to phase structure, timeline
estimates, and risk assessment.

#### 3. Plan Templates

**Module**: `src/plans/templates.rs`

Provides templates for architecture and implementation plans.

**Architecture Plan Templates**:

```rust
pub enum ArchitectureTemplate {
    RustBinary,      // Single binary application
    RustLibrary,     // Reusable library
    WebService,      // HTTP/REST service
    Microservices,   // Microservices architecture
    CLI,             // Command-line tool
}
```

**Template Structure**:

Each template includes:

- Project overview section
- Goals and requirements
- Architecture design (layered by default)
- Technology stack recommendations
- Component structure
- Data flow diagrams (ASCII)
- Deployment strategy

#### 4. Plan Parser

**Module**: `src/plans/parser.rs`

Parses both architecture and implementation plans.

**Functionality**:

```rust
pub struct PlanParser;

impl PlanParser {
    pub fn parse_architecture_plan(content: &str) -> Result<ArchitecturePlan>;

    pub fn parse_implementation_plan(content: &str) -> Result<ImplementationPlan>;

    fn extract_sections(content: &str) -> Vec<Section>;

    fn parse_phases(content: &str) -> Vec<Phase>;
}
```

### Prompt Management Module (src/prompts/)

#### 1. Implementation Plan Parser (Extended)

**Module**: `src/prompts/parser.rs` (now extends `src/plans/parser.rs`)

Parses implementation plan markdown to extract detailed section information:

```text
## Phase X: Name (Week Y-Z)        → Phase header
### X.Y Section Name                → Subsection (generates prompt)
**Deliverables**:                   → Section deliverables
**Acceptance Criteria**:            → Section criteria
**Testing**:                        → Test requirements
```

**Key Structures**:

```rust
pub struct ImplementationPlan {
    pub phases: Vec<Phase>,
    pub metadata: PlanMetadata,
}

pub struct Phase {
    pub number: u32,
    pub name: String,
    pub duration: String,
    pub goal: String,
    pub sections: Vec<Section>,
}

pub struct Section {
    pub number: String,        // e.g., "1.1"
    pub name: String,
    pub tasks: Vec<String>,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub tests: Vec<String>,
}
```

**Usage**:

```rust
use xzagentz::prompts::parser::ImplementationPlanParser;

let parser = ImplementationPlanParser::new();
let plan = parser.parse_file("docs/explanation/implementation_plan.md")?;

for phase in plan.phases {
    for section in phase.sections {
        println!("Section {}: {}", section.number, section.name);
    }
}
```

### 2. Prompt Generator

**Module**: `src/prompts/generator.rs`

Orchestrates prompt generation from parsed implementation plan.

**Key Functions**:

```rust
pub struct PromptGenerator {
    template: PromptTemplate,
    compliance_rules: ComplianceRules,
}

impl PromptGenerator {
    pub fn new() -> Result<Self>;

    pub fn generate_all(&self, plan: &ImplementationPlan, output_dir: &Path) -> Result<Vec<PathBuf>>;

    pub fn generate_section(&self, section: &Section, output_path: &Path) -> Result<()>;

    pub fn generate_interactive(&self, plan: &ImplementationPlan) -> Result<()>;
}
```

**Usage**:

```rust
use xzagentz::prompts::generator::PromptGenerator;

let generator = PromptGenerator::new()?;
let plan = parser.parse_file("docs/explanation/implementation_plan.md")?;

// Batch generation
let generated = generator.generate_all(&plan, Path::new("prompts/"))?;

// Interactive generation
generator.generate_interactive(&plan)?;
```

### 3. Prompt Template

**Module**: `src/prompts/template.rs`

Renders prompt structure with section-specific content.

**Template Structure**:

```rust
pub struct PromptTemplate {
    sections: Vec<TemplateSection>,
}

pub enum TemplateSection {
    Context,
    CriticalRules,
    Task,
    PreFlightChecklist,
    ImplementationChecklist,
    Deliverables,
    AcceptanceCriteria,
    PostImplementationReview,
}
```

**Generated Prompt Format**:

````markdown
# Implementation Prompt - Section X.Y: Name

## Context

- **Implementation Plan**: `docs/explanation/implementation_plan.md`
- **Section**: X.Y - Section Name
- **Phase**: Phase X (Week Y-Z)
- **Files to Reference**: AGENTS.md, existing code

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

[Additional relevant rules based on section type]

## Your Task

Implement section X.Y following these steps:

1. **Read**: Review section X.Y in implementation plan
2. **Plan**: Understand deliverables and acceptance criteria
3. **Implement**: Create files and write code
4. **Test**: Run tests and verify functionality
5. **Verify**: Check AGENTS.md compliance
6. **Document**: Update documentation as needed

## Pre-Flight Checklist

Before starting implementation:

- [ ] Read section X.Y completely in implementation plan
- [ ] Understand all deliverables required
- [ ] Know which files to create/modify
- [ ] Understand acceptance criteria
- [ ] Have test plan in mind

## Implementation Checklist

During implementation:

- [ ] Files created with correct naming (snake_case for .md)
- [ ] All code blocks have language identifiers
- [ ] YAML files use `.yaml` extension (NOT `.yml`)
- [ ] Tests written for all deliverables
- [ ] Documentation follows Diataxis framework
- [ ] Code follows Rust conventions (cargo fmt, clippy)

## Deliverables for Section X.Y

[Extracted from implementation plan]

- Deliverable 1
- Deliverable 2
- ...

## Acceptance Criteria

[Extracted from implementation plan]

- Criterion 1
- Criterion 2
- ...

## Testing Requirements

[Extracted from implementation plan]

```rust
#[test]
fn test_example()
```
````

## Post-Implementation Review

After completing this section, verify:

1. **Summary**: What was implemented?
2. **Files Changed**: List all files created/modified
3. **Tests**: Do all tests pass? Coverage >80%?
4. **Compliance**: Run `xzagentz prompt verify` - all checks pass?
5. **Documentation**: Is documentation complete?
6. **Next Steps**: Ready for next section?

## Verification Commands

```bash
# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Lint (zero warnings required)
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Verify AGENTS.md compliance
xzagentz prompt verify
```

````

**Usage**:

```rust
use xzagentz::prompts::template::PromptTemplate;

let template = PromptTemplate::default();
let prompt = template.render(&section, &compliance_rules)?;
````

### 4. Progress Tracking

**Module**: `src/prompts/progress.rs`

Manages implementation progress with persistent state.

**Key Structures**:

```rust
pub struct ProgressTracker {
    state_file: PathBuf,
    state: ProgressState,
}

pub struct ProgressState {
    pub current_section: Option<String>,
    pub completed_sections: Vec<String>,
    pub total_sections: usize,
    pub last_updated: DateTime<Utc>,
}

impl ProgressTracker {
    pub fn new(state_file: PathBuf) -> Result<Self>;

    pub fn current(&self) -> Option<&str>;

    pub fn complete(&mut self, section: &str) -> Result<()>;

    pub fn next_section(&self, plan: &ImplementationPlan) -> Option<&Section>;

    pub fn completion_percentage(&self) -> f64;

    pub fn reset(&mut self) -> Result<()>;

    pub fn save(&self) -> Result<()>;
}
```

**State File Format** (`.implementation_progress`):

```toml
current_section = "1.2"
last_updated = "2024-01-15T10:30:00Z"

completed_sections = [
    "1.1",
]

[statistics]
total_sections = 27
completed = 1
percentage = 3.7
```

**Usage**:

```rust
use xzagentz::prompts::progress::ProgressTracker;

let mut tracker = ProgressTracker::new(".implementation_progress".into())?;

// Mark section complete
tracker.complete("1.1")?;
tracker.save()?;

// Get next section
if let Some(next) = tracker.next_section(&plan) {
    println!("Next: Section {} - {}", next.number, next.name);
}

// Show progress
println!("Progress: {:.1}%", tracker.completion_percentage());
```

### 5. Compliance Verification

**Module**: `src/prompts/compliance.rs`

Verifies project compliance with AGENTS.md rules.

**Key Structures**:

```rust
pub struct ComplianceChecker {
    rules: Vec<ComplianceRule>,
}

pub struct ComplianceRule {
    pub id: String,
    pub name: String,
    pub category: RuleCategory,
    pub check: Box<dyn Fn(&Path) -> Result<Vec<Violation>>>,
}

pub enum RuleCategory {
    Markdown,
    Yaml,
    Rust,
    Git,
    Shell,
}

pub struct Violation {
    pub rule_id: String,
    pub file: PathBuf,
    pub line: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}

pub struct ComplianceReport {
    pub violations: Vec<Violation>,
    pub passed_rules: Vec<String>,
    pub summary: ReportSummary,
}
```

**Rules Checked**:

1. **Markdown File Naming**: All `.md` files use snake_case (except README.md)
2. **Fenced Code Blocks**: All code blocks have language identifiers
3. **YAML Extensions**: All YAML files use `.yaml` (not `.yml`)
4. **No Emojis**: No emoji characters in documentation
5. **Shell Script Shebang**: Scripts use `#!/usr/bin/env bash`
6. **Shell Script Safety**: Scripts include `set -euo pipefail`
7. **Rust Formatting**: Code passes `cargo fmt --check`
8. **Rust Linting**: Code passes `cargo clippy -- -D warnings`

**Usage**:

```rust
use xzagentz::prompts::compliance::ComplianceChecker;

let checker = ComplianceChecker::new()?;
let report = checker.verify_all(Path::new("."))?;

if !report.violations.is_empty() {
    eprintln!("Compliance violations found:");
    for violation in &report.violations {
        eprintln!("  [{}] {}: {}",
            violation.rule_id,
            violation.file.display(),
            violation.message);
        if let Some(suggestion) = &violation.suggestion {
            eprintln!("    Suggestion: {}", suggestion);
        }
    }
    std::process::exit(1);
}

println!("✓ All compliance checks passed!");
```

## CLI Integration

### Command Structure

```bash
xzagentz prompt <subcommand> [options]

Subcommands:
  generate     Generate prompts from implementation plan
  show         Show specific section prompt
  list         List all available prompts
  progress     Show implementation progress
  complete     Mark section as complete
  next         Show next section to implement
  reset        Reset progress tracking
  verify       Verify AGENTS.md compliance
```

### Example Workflow

```bash
# 1. Generate all prompts
xzagentz prompt generate --batch

# 2. Show current progress
xzagentz prompt progress
# Output: Progress: 0.0% (0/27 sections)

# 3. Get next section
xzagentz prompt next
# Output: Next: Section 1.1 - Project Initialization

# 4. Show section prompt
xzagentz prompt show 1.1 > current_task.md

# 5. Implement section
# (create files, write code, tests)

# 6. Verify compliance
xzagentz prompt verify

# 7. Mark complete
xzagentz prompt complete 1.1

# 8. Continue to next
xzagentz prompt next
# Output: Next: Section 1.2 - Core Data Structures
```

## Workflow Integration

### Standard Development Flow

```bash
# 1. Generate prompts (once at project start)
xzagentz prompt generate

# 2. Get next section to work on
xzagentz prompt next

# 3. View section details
xzagentz prompt show 1.1

# 4. Implement section
# (create files, write code, tests)

# 5. Run quality checks
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

# 6. Verify AGENTS.md compliance
xzagentz prompt verify

# 7. Review changes
git status
git diff

# 8. Commit
git add .
git commit -m "feat(component): implement section 1.1"

# 9. Mark section complete
xzagentz prompt complete 1.1

# 10. Continue to next section
# Repeat from step 2
```

### CI/CD Integration

```yaml
name: Compliance Check

on: [push, pull_request]

jobs:
  compliance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install xzagentz
        run: cargo install --path .

      - name: Verify AGENTS.md compliance
        run: xzagentz prompt verify --strict --report compliance_report.md

      - name: Upload compliance report
        if: failure()
        uses: actions/upload-artifact@v3
        with:
          name: compliance-report
          path: compliance_report.md
```

## Compliance Verification Details

### Verification Categories

#### 1. Markdown Compliance

**Rules**:

- File naming: snake_case (except README.md)
- Fenced code blocks: language identifiers required
- No emojis in documentation
- Line length ≤ 120 characters (configurable)

**Commands**:

```bash
# Check specific file
xzagentz prompt verify --file docs/explanation/example.md

# Check all markdown
xzagentz prompt verify --category markdown
```

#### 2. YAML Compliance

**Rules**:

- Extension must be `.yaml` (not `.yml`)
- Valid YAML syntax
- No tabs (spaces only)

**Commands**:

```bash
# Check YAML files
xzagentz prompt verify --category yaml
```

#### 3. Rust Compliance

**Rules**:

- Code formatted with `cargo fmt`
- No clippy warnings (`-D warnings`)
- Tests pass
- Documentation complete

**Commands**:

```bash
# Check Rust compliance
xzagentz prompt verify --category rust
```

#### 4. Git Compliance

**Rules**:

- Branch naming: `pr-{jira-issue}` format
- Commit messages: Conventional Commits format
- No large files committed

**Commands**:

```bash
# Check Git compliance
xzagentz prompt verify --category git
```

### Verification Report Format

```markdown
# AGENTS.md Compliance Report

Generated: 2024-01-15 10:30:00 UTC

## Summary

- Total Rules Checked: 15
- Passed: 13
- Failed: 2
- Overall Status: FAIL

## Violations

### [MD-001] Markdown File Naming

**File**: `docs/explanation/DistributedTracing.md` **Issue**: Filename uses
CamelCase instead of snake_case **Suggestion**: Rename to
`distributed_tracing.md`

### [MD-040] Missing Language Identifier

**File**: `docs/how_to/setup_guide.md:45` **Issue**: Code block missing language
identifier **Suggestion**: Change `to`bash

## Passed Rules

- ✓ [YAML-001] YAML file extensions
- ✓ [RUST-001] Code formatting
- ✓ [RUST-002] Clippy warnings
- ✓ [GIT-001] Branch naming ...

## Recommendations

1. Fix markdown filenames to use snake_case
2. Add language identifiers to all code blocks
3. Run `cargo fmt --all` before committing

---

Generated by xzagentz v0.1.0
```

## Best Practices

### 1. One Section at a Time

Complete each section fully before moving to the next:

```bash
# Generate prompts
xzagentz prompt generate

# Work through sections sequentially
xzagentz prompt next    # Shows next section
# Implement section
xzagentz prompt verify  # Check compliance
xzagentz prompt complete 1.1
```

### 2. Verify Before Committing

Always run compliance checks before committing:

```bash
# Pre-commit verification
xzagentz prompt verify --strict

# If passed, commit
git add .
git commit -m "feat(prompts): implement section 1.1"
```

### 3. Track Progress Consistently

Keep progress state up to date:

```bash
# After completing a section
xzagentz prompt complete 1.1

# Check progress regularly
xzagentz prompt progress
```

### 4. Use Interactive Mode

For guided implementation:

```bash
# Interactive generation with prompts
xzagentz prompt generate --interactive

# Interactive progress tracking
xzagentz prompt next --interactive
```

### 5. Generate Reports

Document compliance status:

```bash
# Generate report for review
xzagentz prompt verify --report compliance_report.md

# Share with team or include in PR
git add compliance_report.md
```

## Customization

### Custom Rules

Add project-specific rules to `src/prompts/compliance.rs`:

```rust
impl ComplianceChecker {
    pub fn add_custom_rule(&mut self, rule: ComplianceRule) {
        self.rules.push(rule);
    }
}

// Usage
let custom_rule = ComplianceRule {
    id: "CUSTOM-001".to_string(),
    name: "Custom naming convention".to_string(),
    category: RuleCategory::Custom,
    check: Box::new(|path| {
        // Custom check logic
        Ok(vec![])
    }),
};

checker.add_custom_rule(custom_rule);
```

### Custom Templates

Modify prompt template structure:

```rust
// src/prompts/template.rs
impl PromptTemplate {
    pub fn with_custom_sections(sections: Vec<TemplateSection>) -> Self {
        Self { sections }
    }
}
```

### Custom Progress Format

Change progress file format:

```rust
// src/prompts/progress.rs
impl ProgressTracker {
    pub fn set_format(&mut self, format: ProgressFormat) {
        self.format = format;
    }
}
```

## Testing

### Unit Tests

Test individual components:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_implementation_plan() {
        let content = r#"
## Phase 1: Foundation (Week 1-2)

### 1.1 Project Setup

**Deliverables**:
- Cargo.toml
- src/main.rs
        "#;

        let parser = ImplementationPlanParser::new();
        let plan = parser.parse_str(content).unwrap();

        assert_eq!(plan.phases.len(), 1);
        assert_eq!(plan.phases[0].sections.len(), 1);
        assert_eq!(plan.phases[0].sections[0].number, "1.1");
    }

    #[test]
    fn test_generate_prompt() {
        let section = Section {
            number: "1.1".to_string(),
            name: "Project Setup".to_string(),
            tasks: vec![],
            deliverables: vec!["Cargo.toml".to_string()],
            acceptance_criteria: vec![],
            tests: vec![],
        };

        let template = PromptTemplate::default();
        let prompt = template.render(&section, &ComplianceRules::default()).unwrap();

        assert!(prompt.contains("Section 1.1"));
        assert!(prompt.contains("Cargo.toml"));
    }

    #[test]
    fn test_progress_tracking() {
        let mut tracker = ProgressTracker::new_in_memory();

        tracker.complete("1.1").unwrap();
        assert!(tracker.is_complete("1.1"));
        assert_eq!(tracker.completed_sections.len(), 1);
    }

    #[test]
    fn test_compliance_verification() {
        let checker = ComplianceChecker::new().unwrap();
        let report = checker.verify_all(Path::new("test_fixtures")).unwrap();

        assert!(report.violations.is_empty());
    }
}
```

### Integration Tests

Test full workflows:

```rust
#[test]
fn test_generate_and_verify_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Generate prompts
    let generator = PromptGenerator::new().unwrap();
    let plan = parse_test_plan();
    generator.generate_all(&plan, temp_dir.path()).unwrap();

    // Verify prompts exist
    assert!(temp_dir.path().join("prompts/prompt_1_1.md").exists());

    // Track progress
    let mut tracker = ProgressTracker::new(
        temp_dir.path().join(".implementation_progress")
    ).unwrap();
    tracker.complete("1.1").unwrap();

    // Verify compliance
    let checker = ComplianceChecker::new().unwrap();
    let report = checker.verify_all(temp_dir.path()).unwrap();
    assert!(report.violations.is_empty());
}
```

## Troubleshooting

### Prompts Not Generated

**Issue**: `xzagentz prompt generate` produces no output

**Solutions**:

```bash
# Check implementation plan exists
ls docs/explanation/implementation_plan.md

# Verify plan format
xzagentz prompt generate --verbose

# Check for parsing errors
xzagentz prompt generate --dry-run
```

### Progress Not Saving

**Issue**: Progress resets between runs

**Solutions**:

```bash
# Check progress file
cat .implementation_progress

# Verify file permissions
ls -la .implementation_progress

# Manually set progress
xzagentz prompt complete 1.1
```

### Compliance Checks Failing

**Issue**: `xzagentz prompt verify` reports violations

**Solutions**:

```bash
# Get detailed report
xzagentz prompt verify --verbose --report report.md

# Check specific category
xzagentz prompt verify --category markdown

# Auto-fix if possible
cargo fmt --all  # For Rust formatting
```

### Template Rendering Issues

**Issue**: Generated prompts have missing sections

**Solutions**:

```bash
# Regenerate prompts
xzagentz prompt generate --force

# Verify template integrity
xzagentz prompt generate --validate-template

# Check for template customizations
git diff src/prompts/template.rs
```

## Complete Workflow Example

### End-to-End Project Setup

**Day 1: Architecture Planning**

```bash
# Create architecture plan
xzagentz plan architecture create --template rust_binary

# Review and edit architecture plan (optionally ask IDE AI for improvements)
vim plans/architecture_plan.md

# Verify architecture completeness
xzagentz plan architecture verify plans/architecture_plan.md
```

**Day 2: Implementation Planning**

```bash
# Generate implementation plan from architecture
xzagentz plan implementation create \
  --from-architecture plans/architecture_plan.md

# Review and edit implementation plan (optionally ask IDE AI for improvements)
vim plans/implementation_plan.md

# Verify implementation plan
xzagentz plan implementation verify plans/implementation_plan.md
```

**Day 3+: Phased Implementation**

```bash
# Generate prompts for all sections
xzagentz prompt generate --plan plans/implementation_plan.md --batch

# Start implementation workflow
xzagentz prompt next
# Output: Next: Section 1.1 - Project Initialization

# View section prompt
xzagentz prompt show 1.1

# Implement section 1.1...
# (Create files, write code, write tests)

# Verify compliance
xzagentz prompt verify

# Mark section complete
xzagentz prompt complete 1.1

# Continue to next section
xzagentz prompt next
# Output: Next: Section 1.2 - Core Data Structures
```

## Summary

The integrated plan management and prompt generation system provides:

**Planning Phase**:

- **Architecture Templates**: Quick-start architecture plans
- **Plan Generation**: Automated implementation plan from architecture
- **Plan Validation**: Verify completeness and structure

**Execution Phase**:

- **Prompt Generation**: Detailed implementation prompts per section
- **Progress Tracking**: Persistent state across sessions
- **Compliance Verification**: Automated AGENTS.md rule checking

**Benefits**:

- **Complete Lifecycle**: Architecture → Implementation → Execution
- **IDE Integration**: Use your IDE's AI assistant to review and improve plans
- **Type-Safe**: Rust-based implementation with compile-time guarantees
- **Quality**: Built-in compliance checking throughout
- **Efficiency**: Reduced planning overhead, focused execution
- **Consistency**: Uniform approach from planning to completion

By using this integrated system, you ensure well-planned, high-quality
implementation with automated plan generation and compliance verification
throughout the entire project lifecycle.

## See Also

- [AGENTS.md](../../AGENTS.md) - Project rules and conventions
- [Template System](./template_system.md) - Template generation system
- [Document Cleanup Implementation Plan](./document_cleanup_implementation_plan.md) -
  Documentation cleanup strategy
