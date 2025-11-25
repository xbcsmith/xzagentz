# Modular AGENTS.md Template System

A component-based system for building project-specific AGENTS.md files using
Rust.

This document explains a component-based system for building project-specific
AGENTS.md using Rust.

## Directory Structure

```
xzagentz/
├── README.md                          # This file
├── components/
│   ├── core/
│   │   ├── header.md                 # Project overview section
│   │   ├── critical_rules.md         # Universal critical rules
│   │   └── learning_resources.md    # External references
│   ├── languages/
│   │   ├── python.md                 # Python-specific guidelines
│   │   ├── rust.md                   # Rust-specific guidelines
│   │   ├── golang.md                 # Go-specific guidelines
│   │   ├── typescript.md             # Typescript-specific guidelines
│   │   └── bash.md                   # Bash scripting guidelines
│   ├── tools/
│   │   ├── git.md                    # Git conventions
│   │   ├── npm.md                    # NPM/Node.js guidelines
│   │   └── markdown.md               # Markdown style guide
│   └── general/
│       ├── development.md            # General dev practices
│       ├── testing.md                # Testing guidelines
│       └── documentation.md          # Documentation framework
├── templates/
│   ├── python_cli.toml              # Python CLI project recipe
│   ├── rust_binary.toml              # Rust binary project recipe
│   ├── golang_service.toml           # Go microservice recipe
│   ├── npm_webapp.toml               # NPM web app recipe
│   └── bash_scripts.toml             # Bash tooling recipe
├── src/
│   ├── templates/                    # Rust Templates code
│   └── agents/                     # Rust Agents logic code for create, update, add functionality
└── examples/
    ├── python_cli_example.md         # Example output
    ├── rust_binary_example.md        # Example output
    └── golang_service_example.md     # Example output
```

## Quick Start

### Install

```bash
go install <path to repo>
```

### Create an AGENTS.md File

Create an AGENTS.md for your project using the README.md for context.

```bash
xzagentz create --python --git --markdown --project README.md AGENTS.md
```

Create an AGENTS.md for your project using an existing config for context.

```bash
xzagentz create --python --git --markdown --config python_cli.toml AGENTS.md
```

Or Create an AGENTS.md from your project using input from the user.

```bash
xzagentz create --python --git --markdown --interactive AGENTS.md
```

### Update an AGENTS.md File

Update an AGENTS.md section with new rules.

```bash
xzagentz update --python AGENTS.md
```

### Add a new section to an AGENTS.md File

```bash
xzagentz add --markdown AGENTS.md
```

### List available AGENTS.md template sections

```bash
xzagentz list
```

### Validate AGENTS.md sections

```bash
xzagentz validate AGENTS.md
```

### Command-Line Options

```bash
Options:
  -t, --template NAME        Use predefined template (e.g., python_cli.toml)
  -i, --interactive          Interactive mode with prompts
  -d, --dry-run              Show output without writing file
  -v, --verbose              Verbose output
  -h, --help                 Show help message
```

## Sections

1. CRITICAL RULES - READ FIRST
2. PRE-FLIGHT CHECKLIST
3. PROJECT OVERVIEW
4. DEVELOPMENT GUIDELINES
5. DOCUMENTATION STYLE GUIDE
6. GIT CONVENTIONS
7. COMMON MISTAKES & SOLUTIONS
8. TROUBLESHOOTING
9. QUICK REFERENCE COMMANDS
10. FINAL CHECKLIST
11. LEARNING RESOURCES

### CRITICAL RULES - READ FIRST

**8 Critical Rules:**

- Markdown file naming (snake_case)
- Fenced code blocks (must have language identifiers)
- YAML file extensions (.yaml not .yml)
- No emojis in documentation
- Commit message format
- Shell script conventions
- Template file naming
- Markdown line length

### PRE-FLIGHT CHECKLIST

- Environment Setup
- Before Creating Files
- Before Committing

### PROJECT OVERVIEW

- Basic Information
- Key Features
- Core Components
- Project Structure (MANDATORY)

### DEVELOPMENT GUIDELINES

- Language Best Practices
- Language Content Guidelines
- Language Specific Testing Guidelines
- Language Quality Checks

### DOCUMENTATION STYLE GUIDE

- File Creation Decision Tree
- Template Categories
- Markdown File Naming (CRITICAL)
- Fenced Code Blocks (CRITICAL - MD040)
- Emoji Usage (CRITICAL)
- YAML File Extensions (CRITICAL)
- Markdownlint Compliance
- Documentation Pre-Commit Checklist

### GIT CONVENTIONS

- Branch Naming (MANDATORY)
- Commit Messages (MANDATORY)

### COMMON MISTAKES & SOLUTIONS

**10 Common Mistakes:**

1. Wrong Markdown Filename Format
2. Missing Language in Code Blocks
3. Using .yml Extension
4. Shell Scripts Not Executable
5. Missing Error Handling in Scripts
6. Unquoted Variables in Shell Scripts
7. Template Placeholders Not Clear
8. Invalid Commit Messages
9. Files in Wrong Location
10. Not Validating Before Commit

### TROUBLESHOOTING

- Markdownlint Errors
- Shell Script Syntax Errors
- Template Not Working
- Files Not Being Tracked

### QUICK REFERENCE COMMANDS

- Daily Workflow
- File Validation
- Shell Script Testing

### FINAL CHECKLIST

- Template Changes
- Documentation Changes
- Git Changes
- Before Push

### LEARNING RESOURCES

- Essential Reading
- Project-Specific

## Component System

### Core Components (Always Include)

- **header.md**: Project overview with placeholders
- **critical_rules.md**: Top 10 universal rules for AI agents
- **learning_resources.md**: External documentation links

### Language Components (Choose One or More)

- **python.md**: PEP 8, type hints, docstrings, virtual environments
- **rust.md**: Cargo conventions, ownership rules, error handling
- **golang.md**: Go formatting, module structure, idioms
- **bash.md**: Script safety, portability, error handling

### Tool Components (Optional)

- **git.md**: Conventional Commits, branch naming, PR workflow
- **npm.md**: Package.json standards, dependency management
- **markdown.md**: Diataxis framework, formatting rules

### General Components (Recommended)

- **development.md**: Code review, architecture, security
- **testing.md**: Unit tests, integration tests, coverage
- **documentation.md**: Documentation as code, API docs

## Template Recipe Format

Templates are toml files.

### Example: `templates/python_cli.toml`

```toml
[project]
name = "Pipeline CLI"
description = "Command-line tool built with Python"
type = "Python CLI Application"
package_manager = "pip/setuptools"

[components]
core = [ "header","critical_rules", "learning_resources" ]
languages = [ "python" ]
tools = [ "git", "markdown" ]
general = ["development", "testing", "documentation" ]

```

### Example: `templates/rust_binary.toml`

```toml
[project]
name = "Pipeline Doberman CLI"
description = "Compiled binary built with Rust"
type = "Rust Binary Application"
package_manager = "cargo"
main_language = "rust"

[components]
core = [ "header","critical_rules", "learning_resources" ]
languages = [ "rust" ]
tools = [ "git", "markdown" ]
general = ["development", "testing", "documentation" ]

```

## Component Development Guidelines

### File Structure

Each component should:

```markdown
## Section Name

Brief introduction to this section.

### Subsection 1

Content with examples...

### Subsection 2

More content...
```

### Standard Placeholders

- `@project-name@`: Python module name (snake_case)
- `@project-description@`: One-line description
- `@main-language@`: Primary programming language
- `@package-manager@`: Package/dependency manager
- `@project-type@`: Type of project (CLI, web app, etc.)

### Markdown Rules

- Start with `## Section Name`
- Use proper heading hierarchy (##, ###, ####)
- Always specify language for code blocks
- No trailing whitespace
- Use UTF-8 encoding
- Include practical examples

### Example Component

````markdown
## Python Development Guidelines

### Version Requirements

- **Python**: @main-language@
- **Virtual Environment**: `.venv/@project-name@/`

### Code Style

Follow PEP 8:

```python
def process_data(input_path: Path, limit: int = 100) -> list[dict[str, Any]]:
    """Process data from input file.

    Args:
        input_path: Path to input file
        limit: Maximum records to process

    Returns:
        List of processed records
    """
    pass
```
````

## Listing

```bash
xzagentz list

# Output:
Available Components:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Core Components:
  header                Project overview
  critical_rules        Universal rules
  learning_resources    External links

Language Components:
  python                Python guidelines
  rust                  Rust guidelines
  golang                Go guidelines
  bash                  Bash guidelines

Tool Components:
  git                   Git conventions
  npm                   NPM guidelines
  markdown              Markdown style

General Components:
  development           Dev practices
  testing               Testing guidelines
  documentation         Docs framework


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Available Templates:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
python_cli       Python CLI Application
rust_binary      Rust Binary Application
golang_service   Go Microservice
npm_webapp       NPM Web Application
bash_scripts     Bash Script Collection
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━


```

## License

Apache-2.0 (same as original AGENTS.md template)

## Credits

- **Based On**: AGENTS.md
- **Author**: Brett Smith
- **Standards**: PEP 8, PEP 257, Conventional Commits, Diataxis

---
