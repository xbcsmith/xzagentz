# How to Use Interactive Mode

## Overview

Interactive mode in `xzagentz create` allows you to customize your AGENTS.md file by selecting specific components instead of using a predefined template or accepting all defaults.

## Basic Usage

```bash
xzagentz create --interactive --output AGENTS.md
```

## Step-by-Step Walkthrough

### Step 1: Project Metadata

The tool will first collect basic information about your project:

```text
=== Interactive Mode: Project Metadata ===

Project name [xzagentz]: myproject
Language [rust]: python
Project type [binary]: service
Description (optional): My awesome service
Author (optional): Your Name
Version [0.1.0]: 1.0.0
```

**Tips:**

- Press Enter to accept the default value shown in brackets
- Leave optional fields empty by pressing Enter
- Common languages: rust, python, golang, typescript, bash
- Common project types: binary, library, service, cli, api

### Step 2: Component Selection

After metadata collection, you'll be asked to select components from different categories.

#### Core Components

```text
Include core components? [Y/n]: y

Available options:
  1. critical_rules
  2. error_handling
  3. header
  4. learning_resources
  5. testing_standards

Select core components (comma-separated numbers, or 'all'): 1,2
```

**Selection options:**

- `1,2,5` - Select specific components by number
- `all` - Select all available components
- (empty) - Skip this category entirely

**Core components typically include:**

- `critical_rules` - Mandatory coding rules (highly recommended)
- `error_handling` - Language-specific error handling patterns
- `testing_standards` - Testing requirements and patterns
- `header` - Document header and quick reference
- `learning_resources` - Learning materials and references

#### General Components

```text
Include general components? [Y/n]: y

Available options:
  1. development
  2. documentation
  3. testing

Select general components (comma-separated numbers, or 'all'): all
```

**General components typically include:**

- `development` - Development workflow and practices
- `documentation` - Documentation standards
- `testing` - General testing guidelines

#### Language-Specific Components

```text
Include language-specific components for 'python'? [Y/n]: y

Available options:
  1. python_standards
  2. common_patterns

Select python components (comma-separated numbers, or 'all'): 1
```

**Note:** Only components matching your selected language (or marked as "common") will be shown.

#### Tool-Specific Components

```text
Include tool-specific components? [y/N]: n
```

**Tool components typically include:**

- Tool configurations (clippy, black, prettier, etc.)
- CI/CD pipeline examples
- Linter configurations

**Tip:** Default is 'No' since these are usually project-specific.

### Step 3: Preview

After selection, you'll see a preview of the generated content:

```text
Selected 5 component(s)

=== Preview ===

# AGENTS.md - AI Agent Development Guidelines

**CRITICAL**: This file contains mandatory rules for AI agents...

[... first 20 lines ...]

... (245 more lines)
```

### Step 4: Confirm

```text
Write this file? [Y/n]: y

Successfully created: AGENTS.md
```

## Common Workflows

### Minimal Setup (Critical Rules Only)

For a lightweight AGENTS.md with just the essential rules:

```bash
xzagentz create --interactive --output AGENTS.md
```

Then:

- Include core components? → `y`
- Select: `1` (critical_rules only)
- Include general components? → `n`
- Include language-specific? → `n`
- Include tool-specific? → `n`

### Language-Focused Setup

For Python projects emphasizing language standards:

```bash
xzagentz create --interactive --output AGENTS.md
```

Then:

- Language: `python`
- Include core? → `y`, select: `1,2` (critical_rules, error_handling)
- Include general? → `y`, select: `all`
- Include language-specific? → `y`, select: `all`
- Include tool-specific? → `n`

### Comprehensive Setup

For complete documentation with all components:

```bash
xzagentz create --interactive --output AGENTS.md
```

Then select `all` for every category.

### Quick Non-Interactive

If you just want defaults without prompts:

```bash
xzagentz create --output AGENTS.md
```

No interactive prompts; uses default component set.

## Advanced Options

### Combine with Template

You can use a template AND interactive mode:

```bash
xzagentz create --template rust_project --interactive --output AGENTS.md
```

**Note:** When using a template, component selection is skipped and template-defined components are used. Interactive mode only affects metadata collection.

### Force Overwrite

If the file already exists:

```bash
xzagentz create --interactive --force --output AGENTS.md
```

A backup will be created automatically (e.g., `AGENTS.md.backup.20240101_120000`).

### Verbose Mode

See detailed information during creation:

```bash
xzagentz create --interactive --verbose --output AGENTS.md
```

### Custom Component Directory

Use components from a different location:

```bash
xzagentz --component-dir ./my-components create --interactive --output AGENTS.md
```

## Troubleshooting

### No Components Available

**Problem:** "Available options: (none)"

**Solution:**

- Verify components directory exists: `ls components/core/`
- Check component directory path: `xzagentz --verbose create --interactive`
- Ensure components have `.md` or `.yaml` extensions

### Language Components Not Showing

**Problem:** No language-specific components appear

**Solution:**

- Check component naming: files should contain the language name (e.g., `rust_standards.md`)
- Use "common" in filename for language-agnostic components
- Verify components are in `components/languages/` directory

### File Already Exists Error

**Problem:** "Error: File already exists: AGENTS.md"

**Solution:**

- Use `--force` flag to overwrite
- Specify a different output filename
- Remove or rename the existing file

### Empty File Created

**Problem:** Generated file has no content or minimal content

**Solution:**

- Ensure you selected at least one component
- Check that component files are not empty
- Enable verbose mode to see warnings about missing components

### Placeholder Not Found Error

**Problem:** "Error: Placeholder not found: Placeholder 'VERSION' not found" (or similar for PROJECT_NAME, PRIMARY_LANGUAGE, etc.)

**Cause:** Component templates contain placeholders like `{{VERSION}}` that need to be replaced with your project metadata.

**Solution:**

- This was a bug fixed in version 0.1.0+
- Update to the latest version of xzagentz
- If you see this error, the metadata mapping is incomplete
- Verify all metadata was collected during interactive mode
- File an issue if you still see this error with the latest version

## Tips and Best Practices

1. **Start Small**: Begin with critical_rules only, add more as needed
2. **Language First**: Always include language-specific components for your primary language
3. **Preview Carefully**: Review the preview before confirming write
4. **Use Templates for Teams**: Create templates for common project types in your team
5. **Version Control**: Commit your AGENTS.md to track changes over time
6. **Update Regularly**: Re-run with new components as your project evolves

## Example Session

Complete example of creating a Rust project AGENTS.md:

```bash
$ xzagentz create --interactive --output AGENTS.md

=== Interactive Mode: Project Metadata ===

Project name [xzagentz]: my-rust-project
Language [rust]: rust
Project type [binary]: library
Description (optional): High-performance data processing library
Author (optional): Jane Developer
Version [0.1.0]: 0.1.0

=== Interactive Mode: Component Selection ===

Include core components? [Y/n]: y

Available options:
  1. critical_rules
  2. error_handling
  3. testing_standards

Select core components (comma-separated numbers, or 'all'): all

Include general components? [Y/n]: y

Available options:
  1. development
  2. documentation

Select general components (comma-separated numbers, or 'all'): all

Include language-specific components for 'rust'? [Y/n]: y

Available options:
  1. rust_standards
  2. common_patterns

Select rust components (comma-separated numbers, or 'all'): all

Include tool-specific components? [y/N]: n

Selected 7 component(s)

=== Preview ===

# AGENTS.md - AI Agent Development Guidelines
...

Write this file? [Y/n]: y

Successfully created: AGENTS.md
```

## Related Commands

- `xzagentz list components` - See all available components
- `xzagentz validate AGENTS.md` - Validate generated file
- `xzagentz update --section <name>` - Update specific sections later
- `xzagentz add --component <name>` - Add new sections to existing file

## Next Steps

After creating your AGENTS.md:

1. Review the generated file
2. Customize placeholders and project-specific details
3. Commit to version control
4. Share with your team or AI agent
5. Run `xzagentz validate` to verify correctness

For more information:

- See `docs/explanation/interactive_component_selection_implementation.md` for technical details
- See `docs/tutorials/getting_started.md` for complete walkthrough
- Run `xzagentz create --help` for all available options
