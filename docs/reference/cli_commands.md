# CLI Commands Reference

## Overview

Complete reference for all xzagentz command-line interface commands, options, and usage patterns.

## Global Options

Available for all commands:

```bash
xzagentz [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

### --verbose, -v

Enable verbose output with detailed logging.

```bash
xzagentz --verbose list components
```

### --config-dir <PATH>

Specify custom configuration directory.

```bash
xzagentz --config-dir /path/to/config list components
```

### --component-dir <PATH>

Specify custom components directory.

```bash
xzagentz --component-dir ./my-components create
```

### --template-dir <PATH>

Specify custom templates directory.

```bash
xzagentz --template-dir ./my-templates architecture
```

### --format <FORMAT>

Set output format: `human` (default) or `json`.

```bash
xzagentz --format json list components
```

### --help, -h

Display help information.

```bash
xzagentz --help
xzagentz <command> --help
```

### --version, -V

Display version information.

```bash
xzagentz --version
```

## Commands

### init

Initialize xzagentz by extracting embedded resources.

**Syntax**:
```bash
xzagentz init [OPTIONS]
```

**Options**:

- `--components-dir <PATH>`, `-c <PATH>` - Custom components directory
- `--templates-dir <PATH>`, `-t <PATH>` - Custom templates directory
- `--force`, `-f` - Force overwrite existing files
- `--dry-run`, `-n` - Show what would be done without doing it

**Examples**:

```bash
# Extract to default location
xzagentz init

# Extract to custom location
xzagentz init --components-dir ./components --templates-dir ./templates

# Preview without extracting
xzagentz init --dry-run

# Force overwrite existing
xzagentz init --force
```

**Exit Codes**:
- `0` - Success
- `1` - Error during extraction

### list

List available components or templates.

**Syntax**:
```bash
xzagentz list <TARGET> [OPTIONS]
```

**Targets**:

#### list components

List available components.

```bash
xzagentz list components [OPTIONS]
```

**Options**:
- `--category <CATEGORY>`, `-c <CATEGORY>` - Filter by category

**Examples**:

```bash
# List all components
xzagentz list components

# List core components only
xzagentz list components --category core

# List in JSON format
xzagentz --format json list components
```

#### list templates

List available templates.

```bash
xzagentz list templates [OPTIONS]
```

**Options**:
- `--detailed`, `-d` - Show template details

**Examples**:

```bash
# List all templates
xzagentz list templates

# List with details
xzagentz list templates --detailed
```

**Exit Codes**:
- `0` - Success
- `1` - Error listing resources

### validate

Validate component or AGENTS.md file.

**Syntax**:
```bash
xzagentz validate [FILE] [OPTIONS]
```

**Arguments**:
- `FILE` - Path to file to validate (default: `AGENTS.md`)

**Options**:
- `--detailed`, `-d` - Show detailed validation report
- `--fix`, `-f` - Fix common issues automatically

**Examples**:

```bash
# Validate AGENTS.md
xzagentz validate

# Validate specific file
xzagentz validate components/core/error_handling.md

# Detailed validation report
xzagentz validate --detailed

# Fix issues automatically
xzagentz validate --fix

# JSON output
xzagentz --format json validate
```

**Validation Checks**:
- Valid YAML frontmatter
- Required fields present
- Size constraints met
- Language markers matched
- No emojis in content
- Code blocks have language specifiers

**Exit Codes**:
- `0` - Validation passed
- `1` - Validation failed

### create

Create new AGENTS.md file from components.

**Syntax**:
```bash
xzagentz create [OPTIONS]
```

**Options**:
- `--output <FILE>`, `-o <FILE>` - Output file path (default: `AGENTS.md`)
- `--template <NAME>`, `-t <NAME>` - Template to use
- `--force`, `-f` - Force overwrite if file exists
- `--interactive`, `-i` - Interactive mode

**Examples**:

```bash
# Create with defaults
xzagentz create

# Create with custom output
xzagentz create --output my_agents.md

# Create using template
xzagentz create --template rust_project

# Interactive mode
xzagentz create --interactive

# Force overwrite
xzagentz create --force
```

**Exit Codes**:
- `0` - Success
- `1` - Error during creation

### update

Update existing AGENTS.md file section.

**Syntax**:
```bash
xzagentz update [FILE] [OPTIONS]
```

**Arguments**:
- `FILE` - Path to file to update (default: `AGENTS.md`)

**Options**:
- `--section <NAME>`, `-s <NAME>` - Section to update (required)
- `--backup`, `-b` - Create backup before updating (default: true)

**Examples**:

```bash
# Update section
xzagentz update --section testing_standards

# Update specific file
xzagentz update my_agents.md --section error_handling

# Update without backup
xzagentz update --section git_conventions --backup false
```

**Exit Codes**:
- `0` - Success
- `1` - Error during update

### add

Add new section to AGENTS.md file.

**Syntax**:
```bash
xzagentz add [FILE] [OPTIONS]
```

**Arguments**:
- `FILE` - Path to file (default: `AGENTS.md`)

**Options**:
- `--component <NAME>`, `-c <NAME>` - Component to add (required)
- `--position <POS>`, `-p <POS>` - Position to insert (default: `bottom`)

**Position Values**:
- `top` or `beginning` - Add at beginning
- `bottom` or `end` - Add at end
- `after:<section>` - Add after specific section
- `before:<section>` - Add before specific section

**Examples**:

```bash
# Add component at end
xzagentz add --component security_guidelines

# Add at beginning
xzagentz add --component project_overview --position top

# Add after specific section
xzagentz add --component logging_standards --position after:error_handling

# Add before specific section
xzagentz add --component pre_commit_hooks --position before:git_conventions
```

**Exit Codes**:
- `0` - Success
- `1` - Error during add

### prompt

Generate and manage prompts for LLM interactions.

**Syntax**:
```bash
xzagentz prompt [SUBCOMMAND] [OPTIONS]
```

**Subcommands**:

#### prompt generate

Generate prompt from template.

```bash
xzagentz prompt generate [OPTIONS]
```

**Options**:
- `--template <NAME>`, `-t <NAME>` - Prompt template to use
- `--output <FILE>`, `-o <FILE>` - Output file path
- `--variables <JSON>`, `-v <JSON>` - Template variables as JSON

**Examples**:

```bash
# Generate prompt
xzagentz prompt generate --template code_review --output prompt.txt

# With variables
xzagentz prompt generate --template architecture --variables '{"language":"rust"}'
```

#### prompt list

List available prompt templates.

```bash
xzagentz prompt list
```

**Exit Codes**:
- `0` - Success
- `1` - Error during prompt operation

### implementation

Generate implementation plans from architecture documents.

**Syntax**:
```bash
xzagentz implementation [OPTIONS]
```

**Options**:
- `--input <FILE>`, `-i <FILE>` - Input architecture file (required)
- `--output <FILE>`, `-o <FILE>` - Output implementation plan file
- `--phases <NUMBER>`, `-p <NUMBER>` - Number of implementation phases
- `--verbose`, `-v` - Verbose output

**Examples**:

```bash
# Generate implementation plan
xzagentz implementation --input architecture.md --output plan.md

# Generate with specific phases
xzagentz implementation --input architecture.md --phases 5

# Verbose mode
xzagentz implementation --input architecture.md --verbose
```

**Exit Codes**:
- `0` - Success
- `1` - Error during generation

**Requirements**:
- LLM API key configured (OPENAI_API_KEY or ANTHROPIC_API_KEY)
- Valid architecture document as input

### architecture

Generate and manage software architecture documents with LLM.

**Syntax**:
```bash
xzagentz architecture [OPTIONS]
```

**Options**:
- `--project <NAME>`, `-p <NAME>` - Project name
- `--description <TEXT>`, `-d <TEXT>` - System description
- `--output <FILE>`, `-o <FILE>` - Output file path (default: `architecture.md`)
- `--template <NAME>`, `-t <NAME>` - Architecture template to use
- `--pattern <PATTERN>` - Architecture pattern
- `--language <LANG>`, `-l <LANG>` - Primary programming language
- `--interactive`, `-i` - Interactive mode
- `--input <FILE>` - Input architecture file for refinement
- `--refine <TEXT>` - Refinement instructions

**Architecture Patterns**:
- `layered` - Traditional n-tier architecture
- `hexagonal` - Ports and adapters architecture
- `microservices` - Distributed services architecture
- `event-driven` - Event-based architecture

**Examples**:

```bash
# Generate basic architecture
xzagentz architecture --project "My API" --description "REST API service"

# Specify pattern
xzagentz architecture --project "My API" --pattern layered --output architecture.md

# Interactive mode
xzagentz architecture --interactive

# Refine existing architecture
xzagentz architecture --input architecture.md --refine "Add caching layer"

# Language-specific
xzagentz architecture --project "My Service" --language rust --pattern hexagonal
```

**Exit Codes**:
- `0` - Success
- `1` - Error during generation

**Requirements**:
- LLM API key configured (OPENAI_API_KEY or ANTHROPIC_API_KEY)

## Environment Variables

### XZAGENTZ_CONFIG_DIR

Base configuration directory.

```bash
export XZAGENTZ_CONFIG_DIR=/path/to/config
```

### XZAGENTZ_COMPONENT_DIR

Custom components directory.

```bash
export XZAGENTZ_COMPONENT_DIR=/path/to/components
```

### XZAGENTZ_TEMPLATE_DIR

Custom templates directory.

```bash
export XZAGENTZ_TEMPLATE_DIR=/path/to/templates
```

### OPENAI_API_KEY

OpenAI API key for LLM operations.

```bash
export OPENAI_API_KEY=sk-...
```

### ANTHROPIC_API_KEY

Anthropic API key for LLM operations.

```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

### LLM_API_ENDPOINT

Custom LLM API endpoint.

```bash
export LLM_API_ENDPOINT=https://custom-endpoint.com/v1
```

## Exit Codes

All commands use standard exit codes:

- `0` - Success
- `1` - Error (validation failure, file not found, etc.)
- `2` - Invalid arguments or options

## Output Formats

### Human Format (Default)

Human-readable output:

```bash
xzagentz list components
```

Output:
```
Available Components:

Core:
  - error_handling: Error handling best practices
  - testing_standards: Testing guidelines

General:
  - code_review: Code review process
  - documentation: Documentation standards
```

### JSON Format

Machine-readable output:

```bash
xzagentz --format json list components
```

Output:
```json
{
  "total": 4,
  "components": [
    {
      "name": "error_handling",
      "category": "core",
      "description": "Error handling best practices"
    },
    {
      "name": "testing_standards",
      "category": "core",
      "description": "Testing guidelines"
    }
  ]
}
```

## Common Workflows

### Initial Setup

```bash
# Install xzagentz
cargo install --path .

# Initialize resources
xzagentz init

# List available components
xzagentz list components
```

### Creating Project Documentation

```bash
# Create AGENTS.md
xzagentz create --output AGENTS.md

# Validate result
xzagentz validate AGENTS.md --detailed

# Add additional components
xzagentz add AGENTS.md --component security_guidelines
```

### Architecture-Driven Development

```bash
# Generate architecture
xzagentz architecture --project "My Service" --output architecture.md

# Generate implementation plan
xzagentz implementation --input architecture.md --output plan.md

# Create development guidelines
xzagentz create --output AGENTS.md
```

### Validation Workflow

```bash
# Validate all components
for file in components/**/*.md; do
  xzagentz validate "$file"
done

# Fix issues automatically
xzagentz validate AGENTS.md --fix

# Detailed validation report
xzagentz validate AGENTS.md --detailed
```

## Troubleshooting

### Command Not Found

Ensure `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Permission Denied

Check file permissions:

```bash
chmod +x ~/.cargo/bin/xzagentz
```

### API Key Not Found

Set required environment variable:

```bash
export OPENAI_API_KEY=your-key-here
```

### Validation Fails

Use detailed mode to see specific errors:

```bash
xzagentz validate --detailed
```

## See Also

- Environment Variables: `docs/reference/environment_variables.md`
- Component Format: `docs/reference/component_format.md`
- Getting Started Tutorial: `docs/tutorials/getting_started.md`
- Validation Guide: `docs/how_to/validate_components.md`
