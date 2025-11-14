# Configuration Reference

## Overview

Complete reference for xzagentz configuration options, environment variables, and resource resolution.

## Configuration Sources

xzagentz supports multiple configuration sources in priority order:

1. Command-line flags (highest priority)
2. Environment variables
3. Configuration files
4. Default values (lowest priority)

## Environment Variables

### XZAGENTZ_CONFIG_DIR

**Description**: Base configuration directory

**Default**: `~/.config/xzagentz`

**Example**:
```bash
export XZAGENTZ_CONFIG_DIR=/path/to/config
```

**Usage**: Base directory for components and templates subdirectories

### XZAGENTZ_COMPONENT_DIR

**Description**: Custom components directory

**Default**: `$XZAGENTZ_CONFIG_DIR/components` or `~/.config/xzagentz/components`

**Example**:
```bash
export XZAGENTZ_COMPONENT_DIR=/path/to/components
```

**Usage**: Directory containing component markdown files

### XZAGENTZ_TEMPLATE_DIR

**Description**: Custom templates directory

**Default**: `$XZAGENTZ_CONFIG_DIR/templates` or `~/.config/xzagentz/templates`

**Example**:
```bash
export XZAGENTZ_TEMPLATE_DIR=/path/to/templates
```

**Usage**: Directory containing template files

### XDG_CONFIG_HOME

**Description**: XDG base directory for configuration

**Default**: `~/.config`

**Example**:
```bash
export XDG_CONFIG_HOME=/custom/config
```

**Usage**: xzagentz uses `$XDG_CONFIG_HOME/xzagentz` if set

### XDG_DATA_HOME

**Description**: XDG base directory for data

**Default**: `~/.local/share`

**Example**:
```bash
export XDG_DATA_HOME=/custom/data
```

**Usage**: Alternative location for xzagentz data

### OPENAI_API_KEY

**Description**: OpenAI API key for LLM operations

**Required For**: `architecture` and `implementation` commands

**Example**:
```bash
export OPENAI_API_KEY=sk-proj-...
```

**Usage**: Authentication for OpenAI API calls

### ANTHROPIC_API_KEY

**Description**: Anthropic (Claude) API key for LLM operations

**Required For**: `architecture` and `implementation` commands

**Example**:
```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

**Usage**: Authentication for Anthropic API calls

### LLM_API_ENDPOINT

**Description**: Custom LLM API endpoint

**Default**: Provider-specific default endpoint

**Example**:
```bash
export LLM_API_ENDPOINT=https://api.custom-llm.com/v1
```

**Usage**: Override default LLM endpoint

## Command-Line Flags

### Global Flags

Available for all commands:

**--config-dir <PATH>**
- Override XZAGENTZ_CONFIG_DIR
- Example: `xzagentz --config-dir /custom/config list components`

**--component-dir <PATH>**
- Override XZAGENTZ_COMPONENT_DIR
- Example: `xzagentz --component-dir ./components create`

**--template-dir <PATH>**
- Override XZAGENTZ_TEMPLATE_DIR
- Example: `xzagentz --template-dir ./templates architecture`

**--verbose, -v**
- Enable verbose logging
- Example: `xzagentz -v list components`

**--format <FORMAT>**
- Set output format: `human` or `json`
- Example: `xzagentz --format json list components`

## Configuration Files

### Project Configuration

**File**: `xzagentz.yaml` or `.xzagentz.yaml`

**Location**: Project root directory

**Format**: YAML

**Schema**:
```yaml
project:
  name: string
  version: string
  description: string

components:
  directory: string
  include:
    - category/component_name
    - category/*
  exclude:
    - category/component_name

templates:
  directory: string
  default: string

output:
  format: human | json
  verbose: boolean
```

**Example**:
```yaml
project:
  name: "My Project"
  version: "1.0.0"
  description: "Project description"

components:
  directory: "./custom-components"
  include:
    - core/*
    - languages/rust
  exclude:
    - tools/kubernetes_comprehensive

templates:
  directory: "./custom-templates"
  default: "rust_project"

output:
  format: human
  verbose: false
```

### User Configuration

**File**: `config.yaml`

**Location**: `$XZAGENTZ_CONFIG_DIR/config.yaml` or `~/.config/xzagentz/config.yaml`

**Format**: YAML

**Schema**:
```yaml
defaults:
  component_dir: string
  template_dir: string
  output_format: human | json
  verbose: boolean

llm:
  provider: openai | anthropic | custom
  api_key: string
  endpoint: string
  model: string

validation:
  strict: boolean
  check_emojis: boolean
  check_code_blocks: boolean
```

**Example**:
```yaml
defaults:
  component_dir: "~/.config/xzagentz/components"
  template_dir: "~/.config/xzagentz/templates"
  output_format: human
  verbose: false

llm:
  provider: openai
  model: gpt-4
  endpoint: https://api.openai.com/v1

validation:
  strict: true
  check_emojis: true
  check_code_blocks: true
```

## Resource Resolution

### Component Resolution

xzagentz searches for components in this order:

1. **CLI flag**: `--component-dir <PATH>`
2. **Environment variable**: `$XZAGENTZ_COMPONENT_DIR`
3. **Project config**: `xzagentz.yaml` → `components.directory`
4. **User config**: `~/.config/xzagentz/config.yaml` → `defaults.component_dir`
5. **XDG directories**: `$XDG_DATA_HOME/xzagentz/components`
6. **Home directory**: `~/.config/xzagentz/components`
7. **Embedded resources**: Built-in components

First match wins. Embedded resources always available as fallback.

### Template Resolution

xzagentz searches for templates in this order:

1. **CLI flag**: `--template-dir <PATH>`
2. **Environment variable**: `$XZAGENTZ_TEMPLATE_DIR`
3. **Project config**: `xzagentz.yaml` → `templates.directory`
4. **User config**: `~/.config/xzagentz/config.yaml` → `defaults.template_dir`
5. **XDG directories**: `$XDG_DATA_HOME/xzagentz/templates`
6. **Home directory**: `~/.config/xzagentz/templates`
7. **Embedded resources**: Built-in templates

First match wins. Embedded resources always available as fallback.

## Default Values

### Directory Defaults

| Configuration | Default Value |
|---------------|---------------|
| Config directory | `~/.config/xzagentz` |
| Components directory | `~/.config/xzagentz/components` |
| Templates directory | `~/.config/xzagentz/templates` |

### Output Defaults

| Configuration | Default Value |
|---------------|---------------|
| Output format | `human` |
| Verbose mode | `false` |
| Output file | `AGENTS.md` (for create command) |

### LLM Defaults

| Configuration | Default Value |
|---------------|---------------|
| Provider | `openai` |
| Model | `gpt-4` |
| Endpoint | Provider-specific default |

## Configuration Examples

### Minimal Setup

Use defaults with embedded resources:

```bash
# No configuration needed
xzagentz create --output AGENTS.md
```

### Custom Component Directory

Use custom components:

```bash
# Via environment variable
export XZAGENTZ_COMPONENT_DIR=./my-components
xzagentz create

# Via CLI flag
xzagentz --component-dir ./my-components create
```

### Team Configuration

Shared team resources:

```bash
# .bashrc or .zshrc
export XZAGENTZ_COMPONENT_DIR=/team/shared/components
export XZAGENTZ_TEMPLATE_DIR=/team/shared/templates
```

### Project Configuration

Project-specific settings:

**xzagentz.yaml**:
```yaml
project:
  name: "My API Service"

components:
  directory: "./.xzagentz/components"
  include:
    - core/*
    - languages/rust
    - tools/docker_essentials

output:
  verbose: true
```

**Usage**:
```bash
# Automatically uses xzagentz.yaml in current directory
xzagentz create
```

### Multi-Environment Setup

Different configurations per environment:

```bash
# Development
export XZAGENTZ_CONFIG_DIR=./config/dev

# Staging
export XZAGENTZ_CONFIG_DIR=./config/staging

# Production
export XZAGENTZ_CONFIG_DIR=./config/prod
```

## Configuration Precedence

When multiple configuration sources exist:

1. **CLI flags** override everything
2. **Environment variables** override config files and defaults
3. **Project config** (`xzagentz.yaml`) overrides user config
4. **User config** (`~/.config/xzagentz/config.yaml`) overrides defaults
5. **Defaults** are used if nothing else specified

**Example**:
```bash
# CLI flag takes precedence over environment variable
export XZAGENTZ_COMPONENT_DIR=/env/components
xzagentz --component-dir /cli/components create
# Uses: /cli/components
```

## Validation Configuration

### Validation Strictness

Control validation behavior:

```yaml
validation:
  strict: true  # Fail on warnings
  check_emojis: true  # Check for emoji usage
  check_code_blocks: true  # Verify code block languages
  check_placeholders: true  # Verify placeholder format
```

### Custom Validation Rules

Project-specific validation:

```yaml
validation:
  rules:
    - name: "no_todos"
      pattern: "TODO|FIXME"
      message: "Remove TODO/FIXME before commit"
    - name: "line_length"
      max: 120
      message: "Line exceeds 120 characters"
```

## Troubleshooting

### Configuration Not Found

**Problem**: xzagentz ignores configuration

**Check**:
```bash
# Verify file location
ls -la ~/.config/xzagentz/config.yaml

# Check syntax
xzagentz --verbose list components
```

### Environment Variables Not Working

**Problem**: Environment variables not applied

**Check**:
```bash
# Verify variables are set
echo $XZAGENTZ_COMPONENT_DIR
env | grep XZAGENTZ

# Reload shell configuration
source ~/.bashrc
```

### Resource Not Found

**Problem**: Custom components not found

**Check**:
```bash
# Verify directory exists
ls -la $XZAGENTZ_COMPONENT_DIR

# Check resolution order
xzagentz --verbose list components
```

### Permission Issues

**Problem**: Cannot read/write configuration

**Fix**:
```bash
# Ensure directory is writable
chmod u+w ~/.config/xzagentz

# Check file permissions
ls -la ~/.config/xzagentz/config.yaml
```

## Best Practices

### Team Configuration

1. Use project-level `xzagentz.yaml` for team standards
2. Commit configuration to version control
3. Document custom configurations in README
4. Use environment variables for paths, not in config files

### Security

1. Never commit API keys to version control
2. Use environment variables for sensitive data
3. Set restrictive file permissions on config files
4. Rotate API keys regularly

### Organization

1. Keep configurations simple and focused
2. Use defaults when possible
3. Override only what is necessary
4. Document non-standard configurations

### Maintenance

1. Review configurations periodically
2. Remove unused settings
3. Update for new xzagentz versions
4. Validate configuration syntax

## See Also

- Environment Variables Reference: `docs/reference/environment_variables.md`
- CLI Commands Reference: `docs/reference/cli_commands.md`
- Customizing Resources Guide: `docs/how_to/customize_embedded_resources.md`
- Getting Started Tutorial: `docs/tutorials/getting_started.md`
