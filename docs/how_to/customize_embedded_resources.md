# How to Customize Embedded Resources

## Overview

This guide explains how to extract, customize, and manage xzagentz embedded resources. xzagentz includes all components and templates embedded in the binary, but you can extract and modify them to meet your specific needs.

## Prerequisites

- xzagentz installed and working
- Basic understanding of file systems and environment variables
- Text editor for modifying extracted resources

## Quick Start

### Extract All Resources to Default Location

```bash
xzagentz init
```

This extracts components and templates to `~/.config/xzagentz/`:
```
~/.config/xzagentz/
├── components/
│   ├── core/
│   ├── development/
│   ├── documentation/
│   └── tools/
└── templates/
    └── plans/
```

### Extract to Custom Location

```bash
xzagentz init --components-dir ./my-components --templates-dir ./my-templates
```

## Understanding Resource Resolution

xzagentz searches for resources in this priority order:

1. **CLI flag** (highest priority)
   ```bash
   xzagentz create --component-dir ./custom-components
   ```

2. **Environment variable**
   ```bash
   export XZAGENTZ_COMPONENT_DIR=/path/to/components
   ```

3. **XDG directories**
   - `$XDG_DATA_HOME/xzagentz/` (if XDG_DATA_HOME set)
   - `~/.local/share/xzagentz/`

4. **Home directory**
   - `~/.config/xzagentz/`

5. **Embedded resources** (lowest priority, always available)

This means custom resources automatically override embedded ones without configuration.

## Extracting Resources

### Initialize with Default Settings

Extract to default location:

```bash
xzagentz init
```

Output:
```
Successfully initialized xzagentz!
  Components: 42 files in /home/user/.config/xzagentz/components
  Templates:  8 files in /home/user/.config/xzagentz/templates
```

### Initialize with Custom Directories

Specify custom locations:

```bash
xzagentz init \
  --components-dir /opt/myteam/components \
  --templates-dir /opt/myteam/templates
```

### Dry Run Mode

Preview what would be extracted without making changes:

```bash
xzagentz init --dry-run
```

Shows:
- Which files would be created
- Directory structure
- Conflict detection
- Total file count

### Force Overwrite Existing Files

Overwrite existing files during extraction:

```bash
xzagentz init --force
```

**Warning**: This overwrites your customizations. Back up first:
```bash
cp -r ~/.config/xzagentz ~/.config/xzagentz.backup
xzagentz init --force
```

### Selective Extraction

Extract only components or only templates:

```bash
# Extract only components
xzagentz init --components-dir ./components --templates-dir /dev/null

# Extract only templates
xzagentz init --components-dir /dev/null --templates-dir ./templates
```

## Customizing Components

### Modifying Existing Components

Edit extracted component:

```bash
# Extract if not already done
xzagentz init

# Edit component
vim ~/.config/xzagentz/components/core/error_handling.md

# Validate changes
xzagentz validate ~/.config/xzagentz/components/core/error_handling.md

# Use customized component
xzagentz create --output AGENTS.md
```

xzagentz automatically uses your customized version.

### Adding New Components

Create new component in appropriate category:

```bash
# Create new component file
cat > ~/.config/xzagentz/components/general/api_guidelines.md << 'EOF'
---
component:
  name: "api_guidelines"
  category: "general"
  version: "1.0.0"
  description: "API design best practices"
---

# API Design Guidelines

## Overview

Best practices for designing RESTful APIs...
EOF

# Validate new component
xzagentz validate ~/.config/xzagentz/components/general/api_guidelines.md

# Use in project
xzagentz create --output AGENTS.md --component api_guidelines
```

### Removing Components

Remove unwanted components:

```bash
# Remove specific component
rm ~/.config/xzagentz/components/tools/kubernetes_comprehensive.md

# Remove entire category
rm -rf ~/.config/xzagentz/components/tools/
```

Removed components will not appear in listings or be available for use.

## Customizing Templates

### Modifying Existing Templates

Edit extracted template:

```bash
# Extract templates
xzagentz init

# Edit template
vim ~/.config/xzagentz/templates/plans/implementation_plan.md

# Use customized template
xzagentz architecture --template custom_plan
```

### Creating New Templates

Add custom template:

```bash
# Create new template
mkdir -p ~/.config/xzagentz/templates/custom
cat > ~/.config/xzagentz/templates/custom/my_template.md << 'EOF'
# Custom Template

Project: {{project_name}}
Architecture: {{architecture_type}}

## Components

{{#components}}
- {{name}}: {{description}}
{{/components}}
EOF

# Use custom template
xzagentz architecture --template custom/my_template
```

## Environment Variables

### Component Directory

Control component location:

```bash
# Set custom component directory
export XZAGENTZ_COMPONENT_DIR=/path/to/components

# Verify
xzagentz list components
```

### Template Directory

Control template location:

```bash
# Set custom template directory
export XZAGENTZ_TEMPLATE_DIR=/path/to/templates

# Verify
xzagentz list templates
```

### Config Directory

Set base configuration directory:

```bash
# Set config directory (components and templates subdirectories expected)
export XZAGENTZ_CONFIG_DIR=/path/to/config

# xzagentz looks for:
# - /path/to/config/components/
# - /path/to/config/templates/
```

### Making Variables Permanent

Add to shell configuration file:

**Bash** (`~/.bashrc`):
```bash
export XZAGENTZ_COMPONENT_DIR="$HOME/myteam/xzagentz/components"
export XZAGENTZ_TEMPLATE_DIR="$HOME/myteam/xzagentz/templates"
```

**Zsh** (`~/.zshrc`):
```bash
export XZAGENTZ_COMPONENT_DIR="$HOME/myteam/xzagentz/components"
export XZAGENTZ_TEMPLATE_DIR="$HOME/myteam/xzagentz/templates"
```

Reload shell configuration:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

## Team Workflows

### Shared Team Resources

Create team resource repository:

```bash
# Initialize team resources repository
mkdir team-xzagentz-resources
cd team-xzagentz-resources
git init

# Extract resources
xzagentz init --components-dir ./components --templates-dir ./templates

# Customize for team
vim components/core/error_handling.md
vim templates/plans/sprint_plan.md

# Commit and push
git add .
git commit -m "feat: add team xzagentz resources"
git push origin main
```

### Team Members Using Shared Resources

Clone and configure:

```bash
# Clone team resources
git clone https://github.com/myteam/xzagentz-resources.git
cd xzagentz-resources

# Set environment variables
export XZAGENTZ_COMPONENT_DIR="$(pwd)/components"
export XZAGENTZ_TEMPLATE_DIR="$(pwd)/templates"

# Verify
xzagentz list components
xzagentz list templates
```

### Updating Team Resources

Pull latest changes:

```bash
# Update team resources
cd xzagentz-resources
git pull origin main

# xzagentz automatically uses updated resources
xzagentz create --output AGENTS.md
```

### Per-Project Customization

Project-specific resources:

```bash
# Project directory structure
myproject/
├── .xzagentz/
│   ├── components/
│   └── templates/
├── src/
└── AGENTS.md

# Extract to project
cd myproject
xzagentz init --components-dir .xzagentz/components --templates-dir .xzagentz/templates

# Use project-specific resources
xzagentz create --component-dir .xzagentz/components --output AGENTS.md
```

## Resource Management Patterns

### Pattern 1: Global Defaults

Use home directory for personal defaults:

```bash
# One-time setup
xzagentz init

# Resources in ~/.config/xzagentz/ used automatically
xzagentz create --output AGENTS.md
```

**Use case**: Personal productivity, consistent setup across projects

### Pattern 2: Team Standards

Environment variables pointing to team repository:

```bash
# In ~/.bashrc or ~/.zshrc
export XZAGENTZ_COMPONENT_DIR="$HOME/team-resources/components"
export XZAGENTZ_TEMPLATE_DIR="$HOME/team-resources/templates"
```

**Use case**: Team consistency, centralized updates

### Pattern 3: Project-Specific

Per-project directories with CLI flags:

```bash
# In project Makefile
COMPONENT_DIR := .xzagentz/components
TEMPLATE_DIR := .xzagentz/templates

generate:
	xzagentz create \
		--component-dir $(COMPONENT_DIR) \
		--template-dir $(TEMPLATE_DIR) \
		--output AGENTS.md
```

**Use case**: Project-specific requirements, version control

### Pattern 4: Hybrid Approach

Team defaults with project overrides:

```bash
# Team defaults via environment variables
export XZAGENTZ_COMPONENT_DIR="$HOME/team-resources/components"

# Project-specific overrides
xzagentz create \
  --component-dir ./project-components \
  --output AGENTS.md
```

**Use case**: Team standards with project flexibility

## Validation and Quality Assurance

### Validate Customized Components

After customization, always validate:

```bash
# Validate single component
xzagentz validate ~/.config/xzagentz/components/core/error_handling.md

# Validate all customized components
for file in ~/.config/xzagentz/components/**/*.md; do
  echo "Validating: $file"
  xzagentz validate "$file"
done
```

### Version Control Best Practices

Track customizations in version control:

```bash
# Initialize git repository
cd ~/.config/xzagentz
git init
git add components/ templates/
git commit -m "feat: initial custom resources"

# Track changes
vim components/core/error_handling.md
git add components/core/error_handling.md
git commit -m "feat: customize error handling for team standards"
```

### Testing Customizations

Test before rolling out to team:

```bash
# Create test output
xzagentz create --output test_agents.md

# Validate result
xzagentz validate test_agents.md --detailed

# Review content
less test_agents.md

# Clean up
rm test_agents.md
```

## Troubleshooting

### Issue: Custom Resources Not Used

**Symptom**: xzagentz still uses embedded resources after extraction

**Diagnosis**:
```bash
# Check if resources exist
ls -la ~/.config/xzagentz/components/
ls -la ~/.config/xzagentz/templates/

# Verify xzagentz can find them
xzagentz list components
xzagentz list templates --detailed
```

**Solutions**:
1. Verify extraction completed successfully
2. Check file permissions (must be readable)
3. Ensure correct directory structure
4. Verify environment variables if set

### Issue: Permission Denied During Init

**Symptom**: Cannot write to config directory

**Solutions**:
```bash
# Create directory manually
mkdir -p ~/.config/xzagentz
chmod u+w ~/.config/xzagentz

# Or use custom location
xzagentz init --components-dir ~/my-components --templates-dir ~/my-templates
```

### Issue: Customizations Lost After Update

**Symptom**: Customizations overwritten by new xzagentz version

**Prevention**:
```bash
# Always back up before updating
cp -r ~/.config/xzagentz ~/.config/xzagentz.backup.$(date +%Y%m%d)

# After update, merge changes
diff -r ~/.config/xzagentz.backup.* ~/.config/xzagentz/
```

**Solution**:
```bash
# Restore backup
rm -rf ~/.config/xzagentz
mv ~/.config/xzagentz.backup.* ~/.config/xzagentz
```

### Issue: Component Not Found

**Symptom**: Error "Component not found: custom_component"

**Diagnosis**:
```bash
# List available components
xzagentz list components

# Check component file exists
ls -la ~/.config/xzagentz/components/*/custom_component.md

# Verify component name in frontmatter
grep "name:" ~/.config/xzagentz/components/*/custom_component.md
```

**Solutions**:
1. Ensure component file exists in correct directory
2. Verify component name matches frontmatter
3. Check file permissions
4. Validate component syntax

## Advanced Customization

### Using Symbolic Links

Link to shared resources:

```bash
# Link to team resources
ln -s /shared/team/xzagentz/components ~/.config/xzagentz/components
ln -s /shared/team/xzagentz/templates ~/.config/xzagentz/templates

# xzagentz follows symlinks automatically
```

### Multiple Resource Locations

Combine multiple sources:

```bash
# Core resources from team
export XZAGENTZ_COMPONENT_DIR="/team/resources/components"

# Project-specific overrides via CLI
xzagentz create \
  --component-dir ./project-components \
  --output AGENTS.md
```

xzagentz uses project-specific components if found, falls back to team resources.

### Programmatic Resource Management

Script resource updates:

```bash
#!/bin/bash
# update-resources.sh

set -e

RESOURCE_DIR="$HOME/.config/xzagentz"
BACKUP_DIR="$HOME/.config/xzagentz.backup.$(date +%Y%m%d)"

# Backup existing
if [ -d "$RESOURCE_DIR" ]; then
  echo "Backing up to $BACKUP_DIR"
  cp -r "$RESOURCE_DIR" "$BACKUP_DIR"
fi

# Extract latest
echo "Extracting resources..."
xzagentz init --force

# Validate all components
echo "Validating components..."
for file in "$RESOURCE_DIR/components"/**/*.md; do
  xzagentz validate "$file" || exit 1
done

echo "Resources updated successfully"
```

## Summary

Customizing embedded resources allows you to:
- Adapt xzagentz to team standards
- Create project-specific components
- Share resources across teams
- Version control customizations
- Override defaults without modifying binary

The resource resolution order ensures customizations take precedence while embedded resources remain available as fallbacks.

## Related Documentation

- Environment Variables Reference: `docs/reference/environment_variables.md`
- Component Creation Tutorial: `docs/tutorials/creating_custom_component.md`
- Component Validation Guide: `docs/how_to/validate_components.md`
- Architecture Explanation: `docs/explanation/embedded_resources_architecture.md`
