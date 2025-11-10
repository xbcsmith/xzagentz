// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! List command implementation

use crate::components::ComponentLoader;
use crate::core::ComponentType;
use crate::error::Error;
use crate::templates::TemplateLoader;
use serde::Serialize;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use super::output;
use super::{ListTarget, OutputFormat};

/// Information about a component for display
#[derive(Debug, Serialize)]
pub struct ComponentInfo {
    pub name: String,
    pub component_type: String,
    pub summary: String,
}

impl fmt::Display for ComponentInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}: {}", self.component_type, self.name, self.summary)
    }
}

/// List of components for display
#[derive(Debug, Serialize)]
pub struct ComponentList {
    pub total: usize,
    pub components: Vec<ComponentInfo>,
}

impl fmt::Display for ComponentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Available Components ({})", self.total)?;
        writeln!(f, "{}", "=".repeat(50))?;
        for comp in &self.components {
            writeln!(f, "  {}", comp)?;
        }
        Ok(())
    }
}

/// Information about a template for display
#[derive(Debug, Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub component_count: usize,
}

impl fmt::Display for TemplateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} ({} components)",
            self.name, self.description, self.component_count
        )
    }
}

/// List of templates for display
#[derive(Debug, Serialize)]
pub struct TemplateList {
    pub total: usize,
    pub templates: Vec<TemplateInfo>,
}

impl fmt::Display for TemplateList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Available Templates ({})", self.total)?;
        writeln!(f, "{}", "=".repeat(50))?;
        for tmpl in &self.templates {
            writeln!(f, "  {}", tmpl)?;
        }
        Ok(())
    }
}

/// Execute the list command
pub fn execute(
    target: &ListTarget,
    component_dir: &Path,
    template_dir: &Path,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Error> {
    output::print_verbose(verbose, &format!("Component dir: {:?}", component_dir));
    output::print_verbose(verbose, &format!("Template dir: {:?}", template_dir));

    match target {
        ListTarget::Components { category } => {
            list_components(component_dir, category.as_deref(), format, verbose)
        }
        ListTarget::Templates { detailed } => {
            list_templates(template_dir, *detailed, format, verbose)
        }
    }
}

/// List available components
fn list_components(
    component_dir: &Path,
    category_filter: Option<&str>,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Error> {
    output::print_verbose(verbose, "Loading components...");

    let loader = ComponentLoader::new(Some(component_dir.to_path_buf()));

    // Determine which component types to load
    let types_to_load = if let Some(cat) = category_filter {
        vec![ComponentType::from_str(cat)
            .map_err(|e| Error::other(format!("Invalid category: {}", e)))?]
    } else {
        vec![
            ComponentType::Core,
            ComponentType::Languages,
            ComponentType::Tools,
            ComponentType::General,
        ]
    };

    // Load all components
    let mut all_components = Vec::new();
    for comp_type in types_to_load {
        match loader.load_all(comp_type) {
            Ok(mut components) => all_components.append(&mut components),
            Err(e) if verbose => {
                eprintln!("Warning: Could not load {:?} components: {}", comp_type, e);
            }
            Err(_) => {} // Silently skip missing directories if not verbose
        }
    }

    output::print_verbose(
        verbose,
        &format!("Found {} components", all_components.len()),
    );

    // Convert to display format
    let component_infos: Vec<ComponentInfo> = all_components
        .iter()
        .map(|c| {
            let summary = c
                .content
                .lines()
                .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
                .unwrap_or("")
                .trim()
                .to_string();

            ComponentInfo {
                name: c.name.clone(),
                component_type: format!("{:?}", c.component_type),
                summary,
            }
        })
        .collect();

    let list = ComponentList {
        total: component_infos.len(),
        components: component_infos,
    };

    output::print_output(format, &list)?;

    Ok(())
}

/// List available templates
fn list_templates(
    template_dir: &Path,
    detailed: bool,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Error> {
    output::print_verbose(verbose, "Loading templates...");

    let loader = TemplateLoader::with_directory(template_dir);

    // List all templates
    let template_names = match loader.list() {
        Ok(names) => names,
        Err(e) if verbose => {
            eprintln!("Warning: Could not list templates: {}", e);
            vec![]
        }
        Err(_) => vec![], // Return empty list if templates dir doesn't exist
    };

    output::print_verbose(
        verbose,
        &format!("Found {} templates", template_names.len()),
    );

    // Load template details if requested
    let template_infos: Vec<TemplateInfo> = if detailed {
        template_names
            .iter()
            .filter_map(|name| {
                loader.load(name).ok().map(|tmpl| TemplateInfo {
                    name: tmpl.name.clone(),
                    description: tmpl.description.clone(),
                    component_count: tmpl.components.len(),
                })
            })
            .collect()
    } else {
        template_names
            .iter()
            .map(|name| TemplateInfo {
                name: name.clone(),
                description: String::new(),
                component_count: 0,
            })
            .collect()
    };

    let list = TemplateList {
        total: template_infos.len(),
        templates: template_infos,
    };

    output::print_output(format, &list)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_component(dir: &Path, type_name: &str, name: &str, content: &str) {
        let type_dir = dir.join(type_name);
        fs::create_dir_all(&type_dir).unwrap();
        let file_path = type_dir.join(format!("{}.md", name));
        fs::write(file_path, content).unwrap();
    }

    fn create_test_template(dir: &Path, name: &str) {
        let content = format!(
            r#"
name = "{}"
description = "Test template"

[[components]]
category = "core"
name = "test"
required = true
"#,
            name
        );
        let file_path = dir.join(format!("{}.toml", name));
        fs::write(file_path, content).unwrap();
    }

    #[test]
    fn test_list_components() {
        let temp_dir = TempDir::new().unwrap();
        create_test_component(
            temp_dir.path(),
            "core",
            "test1",
            "# Test 1\n\nTest component 1",
        );

        let result = list_components(temp_dir.path(), None, OutputFormat::Human, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_templates() {
        let temp_dir = TempDir::new().unwrap();
        create_test_template(temp_dir.path(), "rust_binary");

        let result = list_templates(temp_dir.path(), false, OutputFormat::Human, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_component_info_display() {
        let info = ComponentInfo {
            name: "test".to_string(),
            component_type: "Core".to_string(),
            summary: "Test summary".to_string(),
        };
        let display = format!("{}", info);
        assert!(display.contains("Core/test"));
    }
}
