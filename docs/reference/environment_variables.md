# Environment Variables Reference

This document provides a complete reference for all environment variables supported by xzagentz.

## Overview

xzagentz uses environment variables to configure resource locations and behavior. Environment variables provide flexibility for different deployment scenarios without requiring code changes.

## Supported Environment Variables

### Resource Location Variables

#### XZAGENTZ_CONFIG_DIR

**Purpose**: Specifies the configuration directory location.

**Default**: `~/.config/xzagentz`

**Example**:

```shell
export XZAGENTZ_CONFIG_DIR="/custom/config/path"
```

**Usage**:

```shell
# Set custom config directory
export XZAGENTZ_CONFIG_DIR="/opt/xzagentz/config"
xzagentz create my-project --template rust_binary
```

**Notes**:
- Takes precedence over XDG_CONFIG_HOME
- Must be an absolute or relative path
- Directory will be created if it doesn't exist (with init command)

---

#### XZAGENTZ_COMPONENTS_DIR

**Purpose**: Specifies the components directory location.

**Default**: `~/.config/xzagentz/components`

**Example**:

```shell
export XZAGENTZ_COMPONENTS_DIR="/shared/components"
```

**Usage**:

```shell
# Use organization-wide components
export XZAGENTZ_COMPONENTS_DIR="/company/shared/xzagentz/components"
xzagentz list components
```

**Notes**:
- Components must be organized in category subdirectories
- Each component must be a YAML file with `.yaml` extension
- Takes precedence over XDG_DATA_HOME

---

#### XZAGENTZ_TEMPLATES_DIR

**Purpose**: Specifies the templates directory location.

**Default**: `~/.config/xzagentz/templates`

**Example**:

```shell
export XZAGENTZ_TEMPLATES_DIR="/custom/templates"
```

**Usage**:

```shell
# Use project-specific templates
export XZAGENTZ_TEMPLATES_DIR="./project-templates"
xzagentz create service --template rust_binary
```

**Notes**:
- Templates must be Markdown files with `.md` extension
- Typically organized in a `plans` subdirectory
- Takes precedence over XDG_DATA_HOME

---

### XDG Base Directory Variables

xzagentz follows the XDG Base Directory Specification for better system integration.

#### XDG_CONFIG_HOME

**Purpose**: Base directory for user-specific configuration files.

**Default**: `~/.config`

**xzagentz Usage**: `$XDG_CONFIG_HOME/xzagentz`

**Example**:

```shell
export XDG_CONFIG_HOME="$HOME/.config"
# xzagentz will use: $HOME/.config/xzagentz
```

**Notes**:
- Lower precedence than XZAGENTZ_CONFIG_DIR
- Standard XDG specification variable
- Shared with other XDG-compliant applications

---

#### XDG_DATA_HOME

**Purpose**: Base directory for user-specific data files.

**Default**: `~/.local/share`

**xzagentz Usage**:
- Components: `$XDG_DATA_HOME/xzagentz/components`
- Templates: `$XDG_DATA_HOME/xzagentz/templates`

**Example**:

```shell
export XDG_DATA_HOME="$HOME/.local/share"
# Components: $HOME/.local/share/xzagentz/components
# Templates: $HOME/.local/share/xzagentz/templates
```

**Notes**:
- Lower precedence than XZAGENTZ_COMPONENTS_DIR and XZAGENTZ_TEMPLATES_DIR
- Standard XDG specification variable
- Recommended for system-wide installations

---

#### HOME

**Purpose**: User's home directory.

**Default**: System-provided (typically `/home/username` on Linux, `/Users/username` on macOS)

**xzagentz Usage**: `$HOME/.config/xzagentz`

**Example**:

```shell
# Usually set by the system
echo $HOME
# Output: /home/username
```

**Notes**:
- Lowest precedence in resolution hierarchy
- Used as base for default paths
- Should rarely need manual override

---

## Resolution Hierarchy

xzagentz resolves resource locations using a hierarchical approach, checking locations in order and using the first match found.

### Configuration Directory Resolution

```text
Priority  Source                           Example Path
───────────────────────────────────────────────────────────────
1 (High)  XZAGENTZ_CONFIG_DIR             /custom/config
2         XDG_CONFIG_HOME/xzagentz        ~/.config/xzagentz
3         HOME/.config/xzagentz           ~/.config/xzagentz
4 (Low)   ./.config/xzagentz              ./.config/xzagentz
```

### Components Directory Resolution

```text
Priority  Source                                Example Path
─────────────────────────────────────────────────────────────────────
1 (High)  Custom flag (--components-dir)       ./custom/components
2         XZAGENTZ_COMPONENTS_DIR              /shared/components
3         XDG_DATA_HOME/xzagentz/components    ~/.local/share/xzagentz/components
4         HOME/.config/xzagentz/components     ~/.config/xzagentz/components
5 (Low)   Embedded resources                   (built into binary)
```

### Templates Directory Resolution

```text
Priority  Source                                Example Path
─────────────────────────────────────────────────────────────────────
1 (High)  Custom flag (--templates-dir)        ./custom/templates
2         XZAGENTZ_TEMPLATES_DIR               /shared/templates
3         XDG_DATA_HOME/xzagentz/templates     ~/.local/share/xzagentz/templates
4         HOME/.config/xzagentz/templates      ~/.config/xzagentz/templates
5 (Low)   Embedded resources                   (built into binary)
```

## Configuration Examples

### Development Environment

```shell
# Use project-local resources
export XZAGENTZ_CONFIG_DIR="./config"
export XZAGENTZ_COMPONENTS_DIR="./config/components"
export XZAGENTZ_TEMPLATES_DIR="./config/templates"
```

### Production Environment

```shell
# Use system-wide resources
export XZAGENTZ_CONFIG_DIR="/etc/xzagentz"
export XZAGENTZ_COMPONENTS_DIR="/usr/share/xzagentz/components"
export XZAGENTZ_TEMPLATES_DIR="/usr/share/xzagentz/templates"
```

### Multi-User Environment

```shell
# Shared components, user-specific templates
export XZAGENTZ_COMPONENTS_DIR="/shared/components"
export XZAGENTZ_TEMPLATES_DIR="$HOME/.config/xzagentz/templates"
```

### CI/CD Environment

```shell
# Use embedded resources (no environment variables needed)
# OR extract to cache directory
export XZAGENTZ_CONFIG_DIR="./.cache/xzagentz"
```

### Testing Environment

```shell
# Isolate test resources
export XZAGENTZ_CONFIG_DIR="/tmp/xzagentz-test"
export XZAGENTZ_COMPONENTS_DIR="/tmp/xzagentz-test/components"
export XZAGENTZ_TEMPLATES_DIR="/tmp/xzagentz-test/templates"
```

## Setting Environment Variables

### Temporary (Current Shell Session)

```shell
# Set for current session only
export XZAGENTZ_CONFIG_DIR="/custom/config"

# Verify it's set
echo $XZAGENTZ_CONFIG_DIR

# Run command with variable
xzagentz create my-project --template rust_binary
```

### Permanent (Shell Profile)

Add to `~/.bashrc`, `~/.zshrc`, or `~/.profile`:

```shell
# xzagentz configuration
export XZAGENTZ_CONFIG_DIR="$HOME/.config/xzagentz"
export XZAGENTZ_COMPONENTS_DIR="$HOME/.config/xzagentz/components"
export XZAGENTZ_TEMPLATES_DIR="$HOME/.config/xzagentz/templates"
```

Reload configuration:

```shell
source ~/.bashrc  # or ~/.zshrc
```

### Per-Command (One-Time)

```shell
# Set variable for single command
XZAGENTZ_CONFIG_DIR=/tmp/config xzagentz create project
```

### Project-Specific (.env File)

Create `.env` in project root:

```shell
XZAGENTZ_CONFIG_DIR=./config
XZAGENTZ_COMPONENTS_DIR=./config/components
XZAGENTZ_TEMPLATES_DIR=./config/templates
```

Load and use:

```shell
# Load variables
export $(cat .env | xargs)

# Or use with direnv
echo 'dotenv' > .envrc
direnv allow
```

### System-Wide (All Users)

Add to `/etc/environment` or `/etc/profile.d/xzagentz.sh`:

```shell
# /etc/profile.d/xzagentz.sh
export XZAGENTZ_CONFIG_DIR=/etc/xzagentz
export XZAGENTZ_COMPONENTS_DIR=/usr/share/xzagentz/components
export XZAGENTZ_TEMPLATES_DIR=/usr/share/xzagentz/templates
```

## Verification and Troubleshooting

### Check Current Values

```shell
# Display all xzagentz-related variables
env | grep XZAGENTZ

# Check specific variables
echo "Config: $XZAGENTZ_CONFIG_DIR"
echo "Components: $XZAGENTZ_COMPONENTS_DIR"
echo "Templates: $XZAGENTZ_TEMPLATES_DIR"
echo "XDG Config: $XDG_CONFIG_HOME"
echo "XDG Data: $XDG_DATA_HOME"
echo "Home: $HOME"
```

### Test Resolution

```shell
# Create a test project to see which resources are used
xzagentz create test-project --template rust_binary --dry-run

# List components to verify directory
xzagentz list components

# List templates to verify directory
xzagentz list templates
```

### Clear Variables

```shell
# Unset specific variables
unset XZAGENTZ_CONFIG_DIR
unset XZAGENTZ_COMPONENTS_DIR
unset XZAGENTZ_TEMPLATES_DIR

# Verify they're cleared
env | grep XZAGENTZ
```

### Common Issues

**Issue**: Variables not taking effect

**Solution**: Ensure variables are exported, not just set:

```shell
# Wrong - not exported
XZAGENTZ_CONFIG_DIR=/custom/path

# Correct - exported
export XZAGENTZ_CONFIG_DIR=/custom/path
```

**Issue**: Path with spaces not working

**Solution**: Use quotes:

```shell
export XZAGENTZ_CONFIG_DIR="/path with spaces/config"
```

**Issue**: Variables not persisting

**Solution**: Add to shell profile and reload:

```shell
echo 'export XZAGENTZ_CONFIG_DIR=/custom/path' >> ~/.bashrc
source ~/.bashrc
```

## Best Practices

### Use Absolute Paths

```shell
# Preferred - absolute path
export XZAGENTZ_CONFIG_DIR="/home/user/.config/xzagentz"

# Avoid - relative paths may be ambiguous
export XZAGENTZ_CONFIG_DIR="./config"
```

### Document Custom Configurations

Add comments when setting variables:

```shell
# Team-wide shared components
export XZAGENTZ_COMPONENTS_DIR="/shared/company/xzagentz/components"

# User-specific templates for client projects
export XZAGENTZ_TEMPLATES_DIR="$HOME/clients/xzagentz-templates"
```

### Use XDG Variables When Possible

```shell
# Preferred - follows XDG specification
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"

# Let xzagentz derive paths from XDG variables
```

### Separate Concerns

```shell
# Keep config and data separate (XDG pattern)
export XZAGENTZ_CONFIG_DIR="$HOME/.config/xzagentz"
export XZAGENTZ_COMPONENTS_DIR="$HOME/.local/share/xzagentz/components"
export XZAGENTZ_TEMPLATES_DIR="$HOME/.local/share/xzagentz/templates"
```

### Test Before Deploying

```shell
# Test with temporary variables
XZAGENTZ_CONFIG_DIR=/tmp/test-config xzagentz init
XZAGENTZ_CONFIG_DIR=/tmp/test-config xzagentz create test --template rust_binary

# If successful, make permanent
export XZAGENTZ_CONFIG_DIR=/tmp/test-config
```

## Security Considerations

### File Permissions

Ensure resource directories have appropriate permissions:

```shell
# Recommended permissions
chmod 755 ~/.config/xzagentz
chmod 644 ~/.config/xzagentz/components/**/*.yaml
chmod 644 ~/.config/xzagentz/templates/**/*.md
```

### Avoid Untrusted Sources

```shell
# Dangerous - don't use untrusted directories
export XZAGENTZ_COMPONENTS_DIR="/untrusted/source/components"

# Safe - use known, controlled locations
export XZAGENTZ_COMPONENTS_DIR="$HOME/.config/xzagentz/components"
```

### Environment Variable Injection

Be cautious in multi-user environments:

```shell
# In scripts, validate environment variables
if [ -z "$XZAGENTZ_CONFIG_DIR" ]; then
    export XZAGENTZ_CONFIG_DIR="$HOME/.config/xzagentz"
fi
```

## Migration Guide

### From Embedded to Filesystem

```shell
# Step 1: Initialize and extract resources
xzagentz init

# Step 2: Set environment variables
export XZAGENTZ_COMPONENTS_DIR="$HOME/.config/xzagentz/components"
export XZAGENTZ_TEMPLATES_DIR="$HOME/.config/xzagentz/templates"

# Step 3: Test
xzagentz list components
xzagentz create test-project --template rust_binary
```

### From Filesystem to Custom Location

```shell
# Step 1: Copy existing resources
cp -r ~/.config/xzagentz /custom/location/

# Step 2: Update environment variables
export XZAGENTZ_CONFIG_DIR="/custom/location"
export XZAGENTZ_COMPONENTS_DIR="/custom/location/components"
export XZAGENTZ_TEMPLATES_DIR="/custom/location/templates"

# Step 3: Verify
xzagentz list components
```

### From Custom to XDG Standard

```shell
# Step 1: Set XDG variables
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"

# Step 2: Move resources to XDG locations
mkdir -p "$XDG_DATA_HOME/xzagentz"
mv /custom/components "$XDG_DATA_HOME/xzagentz/components"
mv /custom/templates "$XDG_DATA_HOME/xzagentz/templates"

# Step 3: Remove custom variables
unset XZAGENTZ_COMPONENTS_DIR
unset XZAGENTZ_TEMPLATES_DIR

# Step 4: Verify
xzagentz list components
```

## Related Documentation

- Setup and Customization Guide: `docs/how_to/setup_custom_resources.md`
- Embedded Resources Architecture: `docs/explanations/embedded_resources_architecture.md`
- CLI Reference: `docs/reference/cli_commands.md`

## Appendix: Quick Reference Table

| Variable | Default | Purpose | Precedence |
|----------|---------|---------|------------|
| XZAGENTZ_CONFIG_DIR | `~/.config/xzagentz` | Configuration directory | High |
| XZAGENTZ_COMPONENTS_DIR | `~/.config/xzagentz/components` | Components directory | High |
| XZAGENTZ_TEMPLATES_DIR | `~/.config/xzagentz/templates` | Templates directory | High |
| XDG_CONFIG_HOME | `~/.config` | XDG config base | Medium |
| XDG_DATA_HOME | `~/.local/share` | XDG data base | Medium |
| HOME | `/home/username` | User home directory | Low |

## Version History

- v1.0.0 - Initial environment variables support
- v1.1.0 - Added XDG Base Directory specification support
- v1.2.0 - Added embedded resources fallback
