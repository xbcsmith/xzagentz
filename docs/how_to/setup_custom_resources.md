# Setup and Customize Resources

This guide explains how to set up, customize, and manage resources (components and templates) in xzagentz.

## Overview

xzagentz includes embedded resources that work out of the box, but you can customize them by extracting and modifying them on your filesystem. The system automatically detects and uses your customized versions.

## Quick Start

### First-Time Setup

Initialize your configuration directory and extract resources:

```shell
# Initialize with default locations (~/.config/xzagentz)
xzagentz init

# Or specify a custom directory
xzagentz init --config-dir ./my-config
```

This creates:

```text
~/.config/xzagentz/
├── components/
│   ├── core/
│   ├── development/
│   ├── documentation/
│   └── testing/
└── templates/
    └── plans/
```

### Verify Installation

List available components and templates:

```shell
# List all components
xzagentz list components

# List all templates
xzagentz list templates
```

## Understanding Resource Resolution

xzagentz uses a resolution hierarchy to find resources. It checks locations in order and uses the first match found.

### Resolution Order

For components:

1. Custom directory (if specified via `--components-dir` flag)
2. `XZAGENTZ_COMPONENTS_DIR` environment variable
3. `$XDG_DATA_HOME/xzagentz/components`
4. `~/.config/xzagentz/components`
5. Embedded resources (built into the binary)

For templates:

1. Custom directory (if specified via `--templates-dir` flag)
2. `XZAGENTZ_TEMPLATES_DIR` environment variable
3. `$XDG_DATA_HOME/xzagentz/templates`
4. `~/.config/xzagentz/templates`
5. Embedded resources (built into the binary)

For configuration:

1. Custom directory (if specified via `--config-dir` flag)
2. `XZAGENTZ_CONFIG_DIR` environment variable
3. `$XDG_CONFIG_HOME/xzagentz`
4. `~/.config/xzagentz`
5. `./.config/xzagentz` (current directory fallback)

## Customizing Components

### Extract Components

Extract components to your filesystem for customization:

```shell
# Extract all components
xzagentz init

# Or extract to a specific location
xzagentz init --config-dir /path/to/config
```

### Modify a Component

1. Navigate to the components directory:

```shell
cd ~/.config/xzagentz/components
```

2. Edit the component file (YAML format):

```shell
# Edit the error handling component
vim core/error_handling.yaml
```

3. Modify the content while preserving the YAML frontmatter:

```yaml
---
name: "error_handling"
category: "core"
version: "1.0.0"
tier: "critical"
---

# Error Handling

Your customized content here...
```

4. Test your changes:

```shell
# Create a new project using your customized component
xzagentz create my-project --template rust_binary --language rust
```

### Create a New Component

1. Create a new YAML file in the appropriate category:

```shell
touch ~/.config/xzagentz/components/custom/my_component.yaml
```

2. Add the required frontmatter and content:

```yaml
---
name: "my_component"
category: "custom"
version: "1.0.0"
tier: "general"
---

# My Custom Component

Add your component content here.

This can include:
- Guidelines
- Code examples
- Best practices
```

3. Reference it in templates or use with the add command:

```shell
xzagentz add --component my_component --file README.md
```

## Customizing Templates

### Extract Templates

Templates are extracted automatically with `xzagentz init`:

```shell
xzagentz init
```

### Modify a Template

1. Navigate to the templates directory:

```shell
cd ~/.config/xzagentz/templates/plans
```

2. Edit a template file (Markdown format):

```shell
vim rust_binary.md
```

3. Modify the template while preserving placeholders:

```markdown
# {{PROJECT_NAME}}

{{PROJECT_DESCRIPTION}}

## Setup

Your customized setup instructions...

## Components

{{COMPONENT:error_handling}}
{{COMPONENT:testing}}
```

4. Test your changes:

```shell
xzagentz create test-project --template rust_binary --language rust
```

### Create a New Template

1. Create a new template file:

```shell
touch ~/.config/xzagentz/templates/plans/my_template.md
```

2. Add content with placeholders:

```markdown
# {{PROJECT_NAME}}

Version: {{PROJECT_VERSION}}
Language: {{LANGUAGE}}

## Overview

{{PROJECT_DESCRIPTION}}

## Required Components

{{COMPONENT:error_handling}}
{{COMPONENT:testing}}

## Optional Components

{{COMPONENT:documentation}}
```

3. Use your new template:

```shell
xzagentz create my-project --template my_template --language rust
```

## Using Environment Variables

Environment variables provide a flexible way to configure resource locations without modifying code or configuration files.

### Setting Environment Variables

#### Temporary (Current Session)

```shell
# Set components directory
export XZAGENTZ_COMPONENTS_DIR=/path/to/components

# Set templates directory
export XZAGENTZ_TEMPLATES_DIR=/path/to/templates

# Set config directory
export XZAGENTZ_CONFIG_DIR=/path/to/config
```

#### Permanent (Shell Profile)

Add to `~/.bashrc`, `~/.zshrc`, or equivalent:

```shell
# xzagentz configuration
export XZAGENTZ_CONFIG_DIR="$HOME/.config/xzagentz"
export XZAGENTZ_COMPONENTS_DIR="$HOME/.config/xzagentz/components"
export XZAGENTZ_TEMPLATES_DIR="$HOME/.config/xzagentz/templates"
```

Reload your shell configuration:

```shell
source ~/.bashrc  # or ~/.zshrc
```

#### Project-Specific (.env File)

Create a `.env` file in your project:

```shell
XZAGENTZ_CONFIG_DIR=./config
XZAGENTZ_COMPONENTS_DIR=./config/components
XZAGENTZ_TEMPLATES_DIR=./config/templates
```

Load it before running commands:

```shell
export $(cat .env | xargs)
xzagentz create my-project --template rust_binary
```

### XDG Base Directory Specification

xzagentz supports XDG Base Directory specification for better system integration:

```shell
# Set XDG directories
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"

# xzagentz will automatically use:
# - $XDG_CONFIG_HOME/xzagentz for configuration
# - $XDG_DATA_HOME/xzagentz/components for components
# - $XDG_DATA_HOME/xzagentz/templates for templates
```

### Environment Variable Precedence

When multiple environment variables are set, precedence is:

1. Specific variables (`XZAGENTZ_*_DIR`)
2. XDG variables (`XDG_CONFIG_HOME`, `XDG_DATA_HOME`)
3. HOME variable fallback
4. Current directory fallback

Example:

```shell
# If all are set:
export HOME=/home/user
export XDG_CONFIG_HOME=/custom/config
export XZAGENTZ_CONFIG_DIR=/override/config

# xzagentz uses: /override/config (highest precedence)
```

## Advanced Usage Scenarios

### Organization-Wide Templates

Share custom templates across your team:

1. Create a shared repository:

```shell
mkdir -p /shared/xzagentz/templates
```

2. Add organization templates:

```shell
cp templates/* /shared/xzagentz/templates/
```

3. Configure team members to use shared templates:

```shell
export XZAGENTZ_TEMPLATES_DIR=/shared/xzagentz/templates
```

### Per-Project Customization

Use project-specific resources:

1. Initialize resources in project directory:

```shell
cd my-project
xzagentz init --config-dir ./config
```

2. Customize for the project:

```shell
vim ./config/components/core/error_handling.yaml
```

3. Use project resources:

```shell
xzagentz create sub-project --config-dir ./config
```

### CI/CD Integration

Use embedded resources in CI/CD without filesystem setup:

```shell
# No init needed - uses embedded resources automatically
xzagentz create service --template rust_binary --language rust
```

Or extract once and cache:

```shell
# In CI setup phase
xzagentz init --config-dir ./.xzagentz-cache

# In build phase
export XZAGENTZ_CONFIG_DIR=./.xzagentz-cache
xzagentz create service --template rust_binary
```

### Multiple Versions

Maintain multiple resource versions:

```shell
# Version 1 resources
export XZAGENTZ_COMPONENTS_DIR=/resources/v1/components

# Version 2 resources
export XZAGENTZ_COMPONENTS_DIR=/resources/v2/components

# Switch between versions as needed
```

## Troubleshooting

### Resources Not Found

If xzagentz cannot find your resources:

1. Check the resolution order and verify paths:

```shell
# Print current environment
echo $XZAGENTZ_CONFIG_DIR
echo $XZAGENTZ_COMPONENTS_DIR
echo $XZAGENTZ_TEMPLATES_DIR
```

2. Verify directory structure:

```shell
# Components should be in subdirectories by category
ls -la ~/.config/xzagentz/components/

# Templates should be in plans subdirectory
ls -la ~/.config/xzagentz/templates/plans/
```

3. Check file permissions:

```shell
# Ensure files are readable
chmod -R u+r ~/.config/xzagentz/
```

4. Verify file format:

```shell
# Components must be YAML with .yaml extension
file ~/.config/xzagentz/components/core/error_handling.yaml

# Templates must be Markdown with .md extension
file ~/.config/xzagentz/templates/plans/rust_binary.md
```

### Init Command Fails

If `xzagentz init` fails:

1. Check if directory already exists:

```shell
ls -la ~/.config/xzagentz/
```

2. Use force flag to overwrite:

```shell
xzagentz init --force
```

3. Try a different directory:

```shell
xzagentz init --config-dir ~/my-xzagentz-config
```

4. Check available disk space:

```shell
df -h ~
```

### Modified Resources Not Used

If your modifications are not being used:

1. Clear any conflicting environment variables:

```shell
unset XZAGENTZ_COMPONENTS_DIR
unset XZAGENTZ_TEMPLATES_DIR
```

2. Verify file was actually modified:

```shell
stat ~/.config/xzagentz/components/core/error_handling.yaml
cat ~/.config/xzagentz/components/core/error_handling.yaml
```

3. Check for typos in component/template names

4. Ensure YAML frontmatter is valid:

```shell
# Use a YAML validator
yamllint ~/.config/xzagentz/components/core/error_handling.yaml
```

### Environment Variables Not Working

If environment variables are not taking effect:

1. Verify variables are exported:

```shell
env | grep XZAGENTZ
```

2. Check for shell escaping issues:

```shell
# Use quotes for paths with spaces
export XZAGENTZ_CONFIG_DIR="/path/with spaces/config"
```

3. Ensure variables are set before running command:

```shell
# Same command line
XZAGENTZ_CONFIG_DIR=/custom/path xzagentz create project
```

4. Check for conflicting aliases or functions:

```shell
type xzagentz
```

### Performance Issues

If resource loading is slow:

1. Reduce number of components in directories

2. Use embedded resources for read-only scenarios:

```shell
# Don't initialize - use embedded
xzagentz create project --template rust_binary
```

3. Clear any caching issues by reinitializing:

```shell
rm -rf ~/.config/xzagentz/
xzagentz init
```

## Best Practices

### Version Control

Track your customizations:

```shell
cd ~/.config/xzagentz
git init
git add .
git commit -m "Initial xzagentz customizations"
```

### Backup Resources

Before modifying, create backups:

```shell
cp -r ~/.config/xzagentz ~/.config/xzagentz.backup
```

### Test Changes

Test customizations before deploying:

```shell
# Use a test directory
xzagentz init --config-dir ./test-config
# Modify resources in ./test-config
# Test with: xzagentz create test-project --config-dir ./test-config
```

### Document Customizations

Add comments to explain custom modifications:

```yaml
---
name: "error_handling"
category: "core"
version: "1.0.0"
tier: "critical"
# Modified: 2024-01-15
# Reason: Added project-specific error handling patterns
---
```

### Use Consistent Naming

Follow naming conventions:

- Components: `lowercase_with_underscores.yaml`
- Templates: `lowercase_with_underscores.md`
- Categories: `lowercase` (no underscores)

### Validate Before Committing

Always validate changes:

```shell
# Check YAML syntax
yamllint ~/.config/xzagentz/components/**/*.yaml

# Test template rendering
xzagentz create test-project --template rust_binary --dry-run
```

## Related Documentation

- Environment Variables Reference: `docs/reference/environment_variables.md`
- Embedded Resources Architecture: `docs/explanations/embedded_resources_architecture.md`
- CLI Commands Reference: `docs/reference/cli_commands.md`

## Support

For additional help:

- Check the FAQ: `docs/reference/faq.md`
- Report issues: GitHub Issues
- Community discussions: GitHub Discussions
