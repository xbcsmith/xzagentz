# Template System Design

## Overview

The xzagentz template system provides a flexible, reliable, and user-friendly
way to manage templates for plans, prompts, and other generated content. The
system uses a hybrid approach that embeds templates directly in the binary while
allowing filesystem-based overrides for customization.

## Design Goals

1. **Zero Setup**: Tool works out-of-box with no installation or configuration
   required
2. **Offline Operation**: No network dependency - critical for developer tools
3. **Version Consistency**: Templates are version-locked to the binary to
   prevent mismatches
4. **Single Artifact**: Distribution is just one binary file
5. **Customizable**: Power users can override or extend templates
6. **Fast Access**: No network latency, minimal I/O operations

## Architecture

### Template Storage Strategy

The system uses a three-tier priority system for loading templates:

```text
Priority 1: Custom Directory (--template-dir flag)
    ↓ (if not found)
Priority 2: User Config Directory (~/.config/xzagentz/templates/)
    ↓ (if not found)
Priority 3: Embedded Templates (compiled into binary)
```

This approach provides:

- Default embedded templates ensure the tool always works
- User config directory allows permanent customization
- Command-line flag enables project-specific or team-shared templates

### Component Structure

```text
xzagentz/
├── templates/                          # Source templates (embedded at compile time)
│   ├── plans/
│   │   ├── architecture_plan_rust_binary.md
│   │   ├── architecture_plan_web_service.md
│   │   ├── architecture_plan_cli.md
│   │   ├── architecture_plan_library.md
│   │   └── implementation_plan.md
│   └── prompts/
│       ├── phase_prompt.md
│       ├── section_prompt.md
│       └── task_prompt.md
├── src/
│   └── templates/
│       ├── mod.rs                     # Public API
│       ├── loader.rs                  # Template loading logic
│       ├── embedded.rs                # Embedded template definitions
│       └── metadata.rs                # Template metadata and validation
└── ~/.config/xzagentz/templates/     # User customization directory (runtime)
    ├── plans/
    └── prompts/
```

## Implementation Details

### Embedded Templates

Templates are embedded at compile time using Rust's `include_str!` macro:

```rust
// src/templates/embedded.rs

/// Embedded template content (compiled into binary)
pub const EMBEDDED_TEMPLATES: &[(&str, &str)] = &[
    ("plans/architecture_plan_rust_binary.md",
     include_str!("../../templates/plans/architecture_plan_rust_binary.md")),
    ("plans/architecture_plan_web_service.md",
     include_str!("../../templates/plans/architecture_plan_web_service.md")),
    ("plans/implementation_plan.md",
     include_str!("../../templates/plans/implementation_plan.md")),
    ("prompts/phase_prompt.md",
     include_str!("../../templates/prompts/phase_prompt.md")),
    ("prompts/section_prompt.md",
     include_str!("../../templates/prompts/section_prompt.md")),
];

/// Get embedded template by name
pub fn get_embedded_template(name: &str) -> Option<&'static str> {
    EMBEDDED_TEMPLATES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, content)| *content)
}

/// List all embedded template names
pub fn list_embedded_templates() -> Vec<&'static str> {
    EMBEDDED_TEMPLATES
        .iter()
        .map(|(name, _)| *name)
        .collect()
}
```

### Template Loader

The `TemplateLoader` implements the priority-based loading strategy:

```rust
// src/templates/loader.rs

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

pub struct TemplateLoader {
    custom_dir: Option<PathBuf>,
    user_config_dir: Option<PathBuf>,
}

impl TemplateLoader {
    /// Create new loader with default user config directory
    pub fn new() -> Self {
        let user_config_dir = dirs::config_dir()
            .map(|p| p.join("xzagentz").join("templates"));

        Self {
            custom_dir: None,
            user_config_dir,
        }
    }

    /// Set custom template directory (highest priority)
    pub fn with_custom_dir(mut self, path: PathBuf) -> Self {
        self.custom_dir = Some(path);
        self
    }

    /// Load template with priority: custom > user config > embedded
    pub fn load(&self, template_name: &str) -> Result<String> {
        // Priority 1: Custom directory (--template-dir flag)
        if let Some(custom) = &self.custom_dir {
            if let Ok(content) = self.load_from_filesystem(custom, template_name) {
                return Ok(content);
            }
        }

        // Priority 2: User config directory
        if let Some(user_cfg) = &self.user_config_dir {
            if let Ok(content) = self.load_from_filesystem(user_cfg, template_name) {
                return Ok(content);
            }
        }

        // Priority 3: Embedded template
        self.load_embedded(template_name)
    }

    fn load_from_filesystem(&self, base_dir: &Path, template_name: &str) -> Result<String> {
        let path = base_dir.join(template_name);
        std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read template from {}", path.display()))
    }

    fn load_embedded(&self, template_name: &str) -> Result<String> {
        crate::templates::embedded::get_embedded_template(template_name)
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_name))
    }

    /// List all available templates
    pub fn list_templates(&self) -> Vec<String> {
        crate::templates::embedded::list_embedded_templates()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Export embedded template to filesystem
    pub fn export(&self, template_name: &str, output_path: &Path) -> Result<()> {
        let content = self.load_embedded(template_name)?;

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(output_path, content)
            .with_context(|| format!("Failed to write template to {}", output_path.display()))
    }

    /// Export all embedded templates to directory
    pub fn export_all(&self, output_dir: &Path) -> Result<usize> {
        let templates = self.list_templates();
        let count = templates.len();

        for template_name in templates {
            let output_path = output_dir.join(&template_name);
            self.export(&template_name, &output_path)?;
        }

        Ok(count)
    }
}
```

### Template Metadata

Templates include metadata for validation and documentation:

```rust
// src/templates/metadata.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub variables: Vec<TemplateVariable>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateCategory {
    Plan,
    Prompt,
    Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default: Option<String>,
}

impl TemplateMetadata {
    /// Extract metadata from template content (frontmatter or header comments)
    pub fn parse_from_template(content: &str) -> Option<Self> {
        // Implementation parses YAML frontmatter or special comment blocks
        // Example:
        // ---
        // name: "Architecture Plan - Rust Binary"
        // description: "Template for planning a Rust binary application"
        // category: Plan
        // variables:
        //   - name: project_name
        //     description: "Name of the project"
        //     required: true
        // ---
        todo!()
    }

    /// Validate that all required variables are provided
    pub fn validate_variables(&self, provided: &HashMap<String, String>) -> Result<(), Vec<String>> {
        let missing: Vec<String> = self.variables
            .iter()
            .filter(|v| v.required && !provided.contains_key(&v.name))
            .map(|v| v.name.clone())
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}
```

## CLI Integration

### Commands

The template system integrates with the CLI through several commands:

#### List Templates

```bash
# List all available templates
xzagentz templates list

# List templates in specific category
xzagentz templates list --category plans

# Show template details
xzagentz templates show architecture_plan_rust_binary
```

#### Export Templates

```bash
# Export all templates to user config directory
xzagentz templates export

# Export to specific directory
xzagentz templates export --output ~/my-templates

# Export single template
xzagentz templates export architecture_plan_rust_binary --output custom.md
```

#### Using Custom Templates

```bash
# Use custom template directory for all commands
xzagentz --template-dir ~/my-templates plan create architecture

# Set in config file (persistent)
# ~/.config/xzagentz/config.toml
[templates]
custom_dir = "/home/user/my-templates"
```

### CLI Implementation

```rust
// src/cli/templates.rs

use clap::{Args, Subcommand};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Args)]
pub struct TemplatesArgs {
    #[command(subcommand)]
    command: TemplatesCommand,
}

#[derive(Subcommand)]
enum TemplatesCommand {
    /// List available templates
    List {
        /// Filter by category (plans, prompts)
        #[arg(long)]
        category: Option<String>,
    },

    /// Show template details
    Show {
        /// Template name
        name: String,
    },

    /// Export templates to filesystem
    Export {
        /// Template name (exports all if not specified)
        name: Option<String>,

        /// Output directory (defaults to user config)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

pub fn handle_templates_command(args: TemplatesArgs) -> Result<()> {
    match args.command {
        TemplatesCommand::List { category } => handle_list(category),
        TemplatesCommand::Show { name } => handle_show(name),
        TemplatesCommand::Export { name, output } => handle_export(name, output),
    }
}

fn handle_list(category: Option<String>) -> Result<()> {
    let loader = crate::templates::TemplateLoader::new();
    let templates = loader.list_templates();

    println!("Available templates:");
    for template in templates {
        if let Some(ref cat) = category {
            if !template.starts_with(cat) {
                continue;
            }
        }
        println!("  - {}", template);
    }

    Ok(())
}

fn handle_export(name: Option<String>, output: Option<PathBuf>) -> Result<()> {
    let loader = crate::templates::TemplateLoader::new();
    let output_dir = output.unwrap_or_else(|| {
        dirs::config_dir()
            .unwrap()
            .join("xzagentz")
            .join("templates")
    });

    match name {
        Some(template_name) => {
            let output_path = output_dir.join(&template_name);
            loader.export(&template_name, &output_path)?;
            println!("Exported {} to {}", template_name, output_path.display());
        }
        None => {
            let count = loader.export_all(&output_dir)?;
            println!("Exported {} templates to {}", count, output_dir.display());
        }
    }

    Ok(())
}
```

## User Workflows

### Workflow 1: Default Usage (Embedded Templates)

```bash
# User installs binary
curl -L https://github.com/xbcsmith/xzagentz/releases/latest/download/xzagentz -o xzagentz
chmod +x xzagentz
sudo mv xzagentz /usr/local/bin/

# Tool works immediately with embedded templates
cd my-project/
xzagentz plan create architecture --template rust_binary

# All templates available, no network required
# Templates match the binary version exactly
```

### Workflow 2: Customization (User Config Directory)

```bash
# Export templates to customize
xzagentz templates export

# Edit templates
vim ~/.config/xzagentz/templates/plans/architecture_plan_rust_binary.md

# Add custom sections, modify structure, etc.

# CLI automatically uses customized templates
cd my-project/
xzagentz plan create architecture --template rust_binary
# Uses customized version from ~/.config/xzagentz/templates/
```

### Workflow 3: Team Templates (Custom Directory)

```bash
# Team maintains shared templates in version control
git clone company/xzagentz-templates ~/team-templates

# Use team templates via flag
xzagentz --template-dir ~/team-templates plan create architecture

# Or configure permanently
echo 'custom_dir = "/home/user/team-templates"' >> ~/.config/xzagentz/config.toml

# Updates to team templates via git pull
cd ~/team-templates
git pull
```

### Workflow 4: Project-Specific Templates

```bash
# Project maintains its own templates
my-project/
├── .xzagentz/
│   └── templates/
│       └── plans/
│           └── custom_architecture.md
└── README.md

# Use project templates
cd my-project/
xzagentz --template-dir .xzagentz/templates plan create architecture
```

## Comparison with Alternatives

### Embedded vs Remote URL

| Aspect                  | Embedded (Chosen) | Remote URL             |
| ----------------------- | ----------------- | ---------------------- |
| **Works offline**       | ✅ Yes            | ❌ No                  |
| **Zero setup**          | ✅ Yes            | ✅ Yes                 |
| **Version consistency** | ✅ Perfect        | ❌ Can mismatch        |
| **Network dependency**  | ✅ None           | ❌ Required            |
| **Security**            | ✅ No remote code | ⚠️ Trust remote source |
| **Speed**               | ✅ Instant        | ⚠️ Network latency     |
| **Binary size**         | ⚠️ +50KB          | ✅ Small               |
| **Update templates**    | Recompile         | Automatic              |

### Embedded vs Filesystem Only

| Aspect                      | Embedded (Chosen)  | Filesystem Only       |
| --------------------------- | ------------------ | --------------------- |
| **Works out-of-box**        | ✅ Yes             | ❌ Requires setup     |
| **Installation complexity** | ✅ Single file     | ⚠️ Multiple files     |
| **Version consistency**     | ✅ Perfect         | ⚠️ Manual sync        |
| **User customization**      | ✅ Via override    | ✅ Direct edit        |
| **Distribution**            | ✅ Single artifact | ❌ Multiple artifacts |

### Why Hybrid Approach Wins

The hybrid embedded + filesystem override approach provides:

1. **Best default experience**: Works immediately with no setup
2. **Version safety**: Templates always match binary capabilities
3. **Flexibility**: Power users can customize without limits
4. **Team collaboration**: Shared template directories via config or flags
5. **Offline reliability**: No network failures or connectivity issues
6. **Security**: No remote code execution risks

## Binary Size Impact

Estimated template sizes:

- Architecture plan templates: ~5KB each × 4 templates = 20KB
- Implementation plan template: ~8KB
- Prompt templates: ~3KB each × 3 templates = 9KB
- Component templates: ~2KB each × 5 templates = 10KB

Total embedded template size: ~50KB

This is negligible compared to:

- Rust runtime and stdlib: ~500KB
- Dependencies (clap, serde, etc.): ~2-3MB
- Total binary size: ~3-5MB

Template embedding adds ~1-2% to binary size.

## Template Format

Templates use a consistent format with metadata and placeholders:

```markdown
---
name: "Architecture Plan - Rust Binary"
description: "Template for planning a Rust binary application"
category: plan
version: "1.0.0"
variables:
  - name: project_name
    description: "Name of the project"
    required: true
  - name: project_description
    description: "Brief description of the project"
    required: true
  - name: author
    description: "Project author or organization"
    required: false
    default: "Unknown"
---

# {{project_name}} Architecture Plan

## Project Overview

{{project_description}}

**Author**: {{author}}

## System Context

[User provides system context]

## Architecture Components

[Template continues...]
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_templates_exist() {
        let templates = crate::templates::embedded::list_embedded_templates();
        assert!(!templates.is_empty());
        assert!(templates.contains(&"plans/architecture_plan_rust_binary.md"));
    }

    #[test]
    fn test_load_embedded_template() {
        let loader = TemplateLoader::new();
        let content = loader.load("plans/architecture_plan_rust_binary.md");
        assert!(content.is_ok());
        assert!(content.unwrap().contains("Architecture Plan"));
    }

    #[test]
    fn test_priority_custom_over_embedded() {
        let temp_dir = tempfile::tempdir().unwrap();
        let custom_path = temp_dir.path().join("plans/test.md");
        std::fs::create_dir_all(custom_path.parent().unwrap()).unwrap();
        std::fs::write(&custom_path, "CUSTOM").unwrap();

        let loader = TemplateLoader::new()
            .with_custom_dir(temp_dir.path().to_path_buf());

        let content = loader.load("plans/test.md").unwrap();
        assert_eq!(content, "CUSTOM");
    }

    #[test]
    fn test_export_template() {
        let loader = TemplateLoader::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("exported.md");

        loader.export("plans/architecture_plan_rust_binary.md", &output_path).unwrap();
        assert!(output_path.exists());
    }

    #[test]
    fn test_export_all_templates() {
        let loader = TemplateLoader::new();
        let temp_dir = tempfile::tempdir().unwrap();

        let count = loader.export_all(temp_dir.path()).unwrap();
        assert!(count > 0);
        assert!(temp_dir.path().join("plans").exists());
    }
}
```

### Integration Tests

```rust
// tests/template_integration.rs

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_list_templates_command() {
    Command::cargo_bin("xzagentz")
        .unwrap()
        .arg("templates")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("architecture_plan_rust_binary"));
}

#[test]
fn test_export_templates_command() {
    let temp_dir = tempfile::tempdir().unwrap();

    Command::cargo_bin("xzagentz")
        .unwrap()
        .arg("templates")
        .arg("export")
        .arg("--output")
        .arg(temp_dir.path())
        .assert()
        .success();

    assert!(temp_dir.path().join("plans/architecture_plan_rust_binary.md").exists());
}

#[test]
fn test_use_custom_template_dir() {
    let temp_dir = tempfile::tempdir().unwrap();
    let custom_template = temp_dir.path().join("plans/custom.md");
    std::fs::create_dir_all(custom_template.parent().unwrap()).unwrap();
    std::fs::write(&custom_template, "# Custom Template").unwrap();

    Command::cargo_bin("xzagentz")
        .unwrap()
        .arg("--template-dir")
        .arg(temp_dir.path())
        .arg("plan")
        .arg("create")
        .arg("architecture")
        .arg("--template")
        .arg("custom")
        .assert()
        .success();
}
```

## Dependencies

Required Cargo dependencies:

```toml
[dependencies]
dirs = "5.0"           # Cross-platform config directory location
anyhow = "1.0"         # Error handling (already in project)
serde = "1.0"          # Serialization (already in project)

[dev-dependencies]
tempfile = "3.8"       # Temporary directories for tests
assert_cmd = "2.0"     # CLI testing
predicates = "3.0"     # Test assertions
```

## Documentation Requirements

The following documentation must be created:

1. **User Guide** (`docs/how_to/customize_templates.md`)

   - How to export templates
   - How to modify templates
   - Template variable reference
   - Custom template directory setup

2. **Template Reference** (`docs/reference/template_format.md`)

   - Template metadata format
   - Available placeholders
   - Template validation rules
   - Template categories

3. **Examples** (`docs/examples/custom_templates.md`)
   - Example custom templates
   - Team template repository setup
   - Project-specific templates

## Future Enhancements

Potential future improvements (not in initial implementation):

1. **Template Registry**: Remote registry for community templates
2. **Template Validation**: Schema validation for template format
3. **Template Versioning**: Version compatibility checking
4. **Template Preview**: Preview rendered template before creating
5. **Template Variables UI**: Interactive variable collection
6. **Template Inheritance**: Templates extending other templates
7. **Template Bundles**: Collections of related templates

## References

- Rust `include_str!` macro:
  <https://doc.rust-lang.org/std/macro.include_str.html>
- XDG Base Directory Specification:
  <https://specifications.freedesktop.org/basedir-spec/latest/>
- Template design patterns: Similar to tools like `cargo-generate`,
  `cookiecutter`

---

**Created**: 2024 **Author**: xzagentz development team **Status**: Design
approved, ready for implementation
