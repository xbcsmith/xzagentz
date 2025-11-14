# Getting Started with xzagentz

## Overview

This tutorial will guide you through installing xzagentz, creating your first component-based project, and validating your work. By the end of this 15-minute tutorial, you will understand the core workflow and be ready to explore advanced features.

## Prerequisites

- Rust 1.70 or later installed
- Cargo package manager
- Basic familiarity with command-line tools
- A text editor (VS Code, Vim, etc.)

## Step 1: Installation

### Building from Source

Clone the repository and build xzagentz:

```bash
git clone https://github.com/xbcsmith/xzagentz.git
cd xzagentz
cargo build --release
```

The compiled binary will be at `target/release/xzagentz`.

### Installing Globally

For convenient access from any directory:

```bash
cargo install --path .
```

This installs xzagentz to your Cargo bin directory (typically `~/.cargo/bin`).

### Verifying Installation

Check that xzagentz is installed correctly:

```bash
xzagentz --version
```

You should see output like: `xzagentz 2.0.0`

## Step 2: Understanding Embedded Resources

xzagentz includes all components and templates embedded in the binary. This means you can start using it immediately without any setup.

### Exploring Available Resources

List embedded components:

```bash
xzagentz list components
```

You will see components organized by category:
- `core` - Essential development components (error handling, testing standards)
- `development` - Development workflow components (git conventions, CI/CD)
- `documentation` - Documentation standards
- `tools` - Language and tool-specific components

List embedded templates:

```bash
xzagentz list templates
```

Templates provide starting points for common project types and architecture patterns.

## Step 3: Creating Your First Project

Let's create a simple AGENTS.md file for a Rust project.

### Using the Create Command

Create a basic AGENTS.md file:

```bash
xzagentz create --output AGENTS.md
```

This creates a file with default components suitable for most projects.

### Viewing the Result

Open the generated `AGENTS.md` file. You will see:
- YAML frontmatter with project configuration
- Core components (error handling, testing, etc.)
- Development guidelines
- Language-specific sections

The file is ready to use as-is or customize for your needs.

## Step 4: Validating Your Work

xzagentz includes powerful validation to ensure your AGENTS.md file follows best practices.

### Running Basic Validation

Validate your AGENTS.md file:

```bash
xzagentz validate AGENTS.md
```

If validation passes, you will see:
```
Validation passed: AGENTS.md
```

### Understanding Validation Rules

xzagentz checks for:
- Valid YAML frontmatter
- Proper component structure
- Size constraints per component category
- Markdown formatting issues
- Missing required sections

### Running Detailed Validation

Get a comprehensive validation report:

```bash
xzagentz validate AGENTS.md --detailed
```

This shows:
- Total components found
- Size analysis per component
- Warnings for components approaching size limits
- Specific issues that need attention

### Fixing Common Issues Automatically

Some validation issues can be fixed automatically:

```bash
xzagentz validate AGENTS.md --fix
```

This corrects:
- Malformed frontmatter
- Inconsistent marker formatting
- Whitespace issues

## Step 5: Customizing Components

As you become comfortable with xzagentz, you may want to customize the embedded components.

### Extracting Resources

Extract embedded resources to your config directory:

```bash
xzagentz init
```

This creates:
```
~/.config/xzagentz/
├── components/
└── templates/
```

### Modifying Components

Edit extracted components to match your team's needs:

```bash
# Edit the error handling component
vim ~/.config/xzagentz/components/core/error_handling.md

# xzagentz will automatically use your customized version
xzagentz create --output AGENTS.md
```

## Step 6: Working with Languages

xzagentz supports language-specific content within components.

### Specifying a Language

Create a Rust-specific AGENTS.md:

```bash
xzagentz create --output AGENTS.md --language rust
```

This extracts Rust-specific sections from components and includes Rust best practices.

### Supported Languages

xzagentz currently supports:
- `rust` - Rust language
- `python` - Python language
- `go` - Go language
- `typescript` - TypeScript/JavaScript
- `bash` - Shell scripting

### Language Fallback

If a component does not have language-specific content, xzagentz automatically falls back to universal content, ensuring you always get complete guidance.

## Step 7: Next Steps

Congratulations! You have completed the getting started tutorial. Here are recommended next steps:

### Learn More

- **Creating Custom Components**: See `creating_custom_component.md` tutorial
- **Component Authoring Guide**: See `docs/how_to/authoring_components.md`
- **Validation Guide**: See `docs/how_to/validate_components.md`

### Explore Advanced Features

- **Architecture Generation**: Use `xzagentz architecture` to generate system architecture documents
- **Implementation Plans**: Use `xzagentz implementation` to create detailed implementation plans
- **Interactive Mode**: Use `xzagentz create --interactive` for guided project creation

### Common Workflows

**Daily Development Workflow**:
```bash
# Validate your work regularly
xzagentz validate AGENTS.md

# Update specific sections
xzagentz update AGENTS.md --section testing_standards

# Add new components as needs evolve
xzagentz add AGENTS.md --component security_guidelines
```

**Team Onboarding Workflow**:
```bash
# Create standardized AGENTS.md for team
xzagentz create --template team_standards --output AGENTS.md

# Customize for project specifics
xzagentz update AGENTS.md --section project_overview

# Validate before committing
xzagentz validate AGENTS.md --detailed
```

## Troubleshooting

### xzagentz Command Not Found

If you get "command not found" after installation:

1. Ensure `~/.cargo/bin` is in your PATH:
   ```bash
   echo $PATH | grep cargo
   ```

2. Add to PATH if missing (add to `~/.bashrc` or `~/.zshrc`):
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

3. Reload your shell:
   ```bash
   source ~/.bashrc  # or source ~/.zshrc
   ```

### Validation Errors

If validation fails with errors:

1. Read error messages carefully - they indicate exactly what is wrong
2. Use `--detailed` flag for comprehensive diagnostics
3. Try `--fix` flag for automatic corrections
4. See `docs/how_to/troubleshooting.md` for common issues

### Permission Denied

If you get permission errors during `xzagentz init`:

```bash
# Ensure config directory is writable
chmod u+w ~/.config/xzagentz

# Or use custom directory
xzagentz init --components-dir ./my-components
```

## Summary

You have learned to:
- Install xzagentz and verify the installation
- Understand embedded resources and list available components
- Create your first AGENTS.md file
- Validate your work with detailed reporting
- Extract and customize components
- Work with language-specific content
- Access resources for further learning

xzagentz empowers you to create consistent, validated development guidelines across all your projects. Explore the documentation to unlock its full potential.

## What You Have Learned

- **Installation**: Building and installing xzagentz globally
- **Embedded Resources**: Using built-in components and templates
- **Project Creation**: Generating AGENTS.md files
- **Validation**: Checking files for correctness and best practices
- **Customization**: Extracting and modifying components
- **Language Support**: Creating language-specific guidance

## Time Invested

Approximately 15 minutes

## Next Tutorial

Proceed to `creating_custom_component.md` to learn how to author your own reusable components.
