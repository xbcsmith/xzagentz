# SHIT THAT IS BROKEN AND NEEDS FIXING

Missing embedded templates for architecture, plans, and prompts. Templates should be embedded like components and be able to be extracted like components.

```bash
./target/debug/xzagentz list templates
```

```text
Available Templates (0)
==================================================
```

The list components output is horrible. Descriptions are not helpful and seem to
be a random line of text from the component.

```bash
./target/debug/xzagentz list components
```

```text
Available Components (22)
==================================================
  Core/error_handling: Error handling is critical for building robust, maintainable software. Follow these universal principles:
  Core/critical_rules: **CRITICAL**: These rules are mandatory. Non-compliance will result in rejected code.
  Core/header: **Version**: {{VERSION}}
  Core/testing_standards: **Core Principles:**
  Core/learning_resources: This section provides curated learning resources, references, and guides for AI agents working on this project.
  Languages/golang: This section provides Go-specific guidelines for projects that include Go components.
  Languages/python: This section provides Python-specific guidelines for projects that include Python components.
  Languages/rust: This section provides Rust-specific guidelines, patterns, and best practices for this project.
  Languages/typescript: This component provides TypeScript-specific guidelines, patterns, and best practices for robust application development.
  Tools/markdown: This section provides Markdown-specific guidelines for documentation in this project.
  Tools/git_essential: Core Git workflows and conventions for daily development tasks.
  Tools/docker_comprehensive: Complete Docker reference including advanced operations, optimization, and troubleshooting.
  Tools/git_comprehensive: Complete Git workflows including advanced operations, troubleshooting, and best practices.
  Tools/docker_essential: Core Docker commands and workflows for daily container operations.
  Tools/markdown_comprehensive: Complete Markdown reference including advanced formatting, best practices, and troubleshooting.
  Tools/git: This section provides Git-specific guidelines, workflows, and best practices for version control in this project.
  Tools/kubernetes_comprehensive: Complete Kubernetes reference including advanced operations, scaling, and production best practices.
  Tools/markdown_essential: Core Markdown guidelines for consistent documentation.
  Tools/kubernetes_essential: Core Kubernetes commands and workflows for daily container orchestration.
  General/testing: Follow these universal testing principles:
  General/documentation: Follow these universal documentation practices:
  General/development: Follow these universal development practices:
```

I want something that looks like this :

```text
Available Components
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Core
────
  acceptance_criteria         Acceptance criteria checklist for implementation tasks (m...
  context                     Context section template for implementation prompts
  critical_rules              Critical rules and guidelines that must never be violated
  deliverables                Deliverables checklist for implementation tasks (multi-la...
  header                      Project header with name, description, and key information
  implementation_checklist    Implementation checklist for development tasks (multi-lan...
  learning_resources          Learning resources and references for the project
  post_implementation_review  Post-implementation review checklist for validating code,...
  preflight_checklist         Pre-flight checklist for implementation tasks (multi-lang...
  task                        Task section template for implementation prompts

Languages
─────────
  bash        Bash scripting guidelines and best practices
  golang      Go-specific development guidelines and best practices
  nodejs      Node.js JavaScript runtime environment
  python      Python-specific development guidelines and best practices
  rust        Rust-specific development guidelines and best practices
  typescript  TypeScript-specific development guidelines and best pract...

Tools
─────
  airflow     airflow tool component
  cargo       Cargo package manager and build tool for Rust projects
  clippy      Clippy linter for catching common Rust mistakes and impro...
  docker      Docker containerization for building, shipping, and runni...
  eslint      ESLint JavaScript linter
  git         Git version control guidelines and workflows
  jest        jest tool component
  jupyter     jupyter tool component
  kubernetes  Kubernetes container orchestration platform
  markdown    Markdown writing standards and linting guidelines
  mlflow      mlflow tool component
  npm         NPM and Node.js package management guidelines
  pip         pip Python package installer and dependency management gu...
  poetry      Poetry Python package and dependency management guidelines
  postgresql  PostgreSQL relational database
  prettier    prettier tool component
  redis       Redis in-memory data store
  rustfmt     Rustfmt code formatter for consistent Rust code style

General
───────
  deployment       Deployment strategies, CI/CD pipelines, and release manag...
  development      Development workflow and environment setup guidelines
  diataxis         Diataxis Documentation Framework instructions
  documentation    Documentation standards and guidelines for code and proje...
  monitoring       Application monitoring, observability, and alerting strat...
  standard_layout  Standard documentation layout and organization guidelines
  testing          Testing strategies, patterns, and coverage requirements


Available Templates
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  bash_scripts               Template
  data_pipeline              Template
  event_driven_microservice  Template
  go_microservice            Template
  microservice               Template
  nodejs_monorepo            Template
  python_cli                 Template
  python_mcp_server          Template
  python_ml_pipeline         Template
  rust_cli                   Template
  rust_library               Template
  rust_microservice          Template
  typescript_web             Template
  vscode_extension           Template

Usage: agentz create --template <name> AGENTS.md


Architecture Templates
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  NAME                        COMPLEXITY    TECHNOLOGIES                    DESCRIPTION
  ────                        ──────────    ────────────                    ───────────
  Bash Scripts                beginner      Bash, ShellCheck, BATS          Template for Bash script projects wit...
  Data Pipeline               advanced      Python, Apache Airflow, Pan...  Template for data processing and ETL ...
  Event-Driven Microservice   advanced      Event-Driven Architecture, ...  Language-agnostic event-driven micros...
  Go Microservice             intermediate  Go, PostgreSQL, Redis, gRPC...  RESTful API microservice with databas...
  Microservice                intermediate  Docker, Kubernetes, API Gat...  Language-agnostic microservice templa...
  Node.js Monorepo            intermediate  Node.js, TypeScript, npm wo...  Monorepo with multiple packages, shar...
  Python CLI Tool             beginner      Python, Click, Typer, pytest    Template for Python CLI applications ...
  Python MCP Server           intermediate  Python, FastMCP, MCP Protoc...  Production-ready Model Context Protoc...
  Python ML Pipeline          advanced      Python, scikit-learn, Tenso...  End-to-end ML pipeline with data inge...
  Rust CLI Tool               beginner      Rust, Clap, Tokio, Serde, A...  Command-line tool with argument parsi...
  Rust Library                intermediate  Rust, Cargo, docs.rs            Template for Rust libraries with safe...
  Rust Microservice           advanced      Rust, Tokio, Actix-web, Pos...  Production-ready Rust microservice wi...
  TypeScript Web Application  intermediate  TypeScript, React, Vite, ES...  Template for TypeScript web applicati...
  VSCode Diataxis Extension   advanced      TypeScript, VSCode Extensio...  Template for VSCode extensions implem...
```

Component language templates are ridiculously long.

A good Rust @AGENTS.md should look something like this:

````markdown
# AGENTS.md - AI Agent Development Guidelines

**CRITICAL**: This file contains mandatory rules for AI agents working on
XZardgz. Non-compliance will result in rejected code.

---

## 1. Identity & Purpose

- **Name**: XZardgz
- **Purpose**: Autonomous AI agent CLI for repository documentation generation.
- **Stack**: Rust (stable), Tokio, Clap, GitHub Copilot/Ollama.

---

## 2. Critical Rules (The "Golden Rules")

**VIOLATION OF THESE RULES = IMMEDIATE REJECTION**

### Rule 1: File Extensions

- **MUST** use `.yaml` (NOT `.yml`)
- **MUST** use `.md` (NOT `.MD` or `.markdown`)
- **MUST** use `.rs` for Rust code

### Rule 2: Filenames

- **MUST** use `lowercase_with_underscores` for ALL files (docs, code, config).
- **EXCEPTION**: `README.md` is the ONLY uppercase filename allowed.
- **EXAMPLE**: `docs/explanation/implementation_plan.md` (✅),
  `docs/explanation/ImplementationPlan.md` (❌)

### Rule 3: No Emojis

- **NEVER** use emojis in code, documentation, or commit messages.
- **REASON**: Encoding issues and professional standards.

### Rule 4: Code Quality Gates

**ALL** of the following must pass before you claim a task is done:

1. `cargo fmt --all` (Formatting)
2. `cargo check --all-targets --all-features` (Compilation)
3. `cargo clippy --all-targets --all-features -- -D warnings` (Linting - ZERO
   warnings allowed)
4. `cargo test --all-features` (Testing - >80% coverage)

### Rule 5: Dependency Management

- **MUST** use `cargo add <crate>` to add dependencies.
- **NEVER** edit `Cargo.toml` manually to add dependencies.
- **REASON**: Ensures compatible versions are selected.

---

## 3. Development Workflow

Follow this exact sequence for every task:

1.  **Implement**: Write code with `///` doc comments.
2.  **Test**: Add unit tests covering success, failure, and edge cases.
3.  **Verify**: Run the 4 Quality Gates (fmt, check, clippy, test).
4.  **Document**: Create/Update `docs/explanation/{feature}_implementation.md`.

---

## 4. Documentation Standards (Diataxis)

We follow the [Diataxis Framework](https://diataxis.fr/).

| Category        | Path                | Purpose                                       | Example                 |
| :-------------- | :------------------ | :-------------------------------------------- | :---------------------- |
| **Tutorials**   | `docs/tutorials/`   | Learning-oriented, step-by-step               | `getting_started.md`    |
| **How-To**      | `docs/how_to/`      | Task-oriented, specific goals                 | `configure_provider.md` |
| **Explanation** | `docs/explanation/` | Understanding-oriented, design/implementation | `architecture.md`       |
| **Reference**   | `docs/reference/`   | Information-oriented, specs                   | `api_spec.md`           |

**Implementation Summaries**: Always create a summary in `docs/explanation/` for
your work.

- **Filename**: `{feature}_implementation.md`
- **Content**: Overview, Components, Implementation Details, Testing Results.

---

## 5. Rust Coding Standards

### Error Handling

- Use `Result<T, E>` for recoverable errors.
- Use `thiserror` for custom error enums.
- **NEVER** use `unwrap()` or `expect()` without a `// SAFETY:` comment
  explaining why it cannot fail.

### Testing

- **Coverage**: >80% required.
- **Structure**:

  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_success_case() { ... }

      #[test]
      fn test_failure_case() { ... }
  }
  ```

---

## 6. Quick Reference

```bash
# Quality Check Loop
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
````

The other languages should follow suit. We should break the break the components
into minimal sets that do not bloat the context window. We should have a flag to
enable the language sections of the components.
