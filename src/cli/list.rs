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
        writeln!(f, "{}", "━".repeat(60))?;

        use std::collections::HashMap;

        // Group components by their category string
        let mut groups: HashMap<String, Vec<&ComponentInfo>> = HashMap::new();
        for comp in &self.components {
            groups
                .entry(comp.component_type.clone())
                .or_default()
                .push(comp);
        }

        // Preferred display order
        let order = vec!["Core", "Languages", "Tools", "General"];

        for key in order {
            if let Some(items) = groups.get(key) {
                writeln!(f, "\n{}", key)?;
                writeln!(f, "{}", "─".repeat(4))?;

                // sort items by name
                let mut items_sorted = items.clone();
                items_sorted.sort_by(|a, b| a.name.cmp(&b.name));

                // compute max name length
                let max_name = items_sorted.iter().map(|c| c.name.len()).max().unwrap_or(0);

                for item in items_sorted {
                    writeln!(
                        f,
                        "  {name:<width$}  {desc}",
                        name = item.name,
                        width = max_name,
                        desc = item.summary
                    )?;
                }
            }
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

        use std::collections::HashMap;

        let mut groups: HashMap<String, Vec<&TemplateInfo>> = HashMap::new();
        for tmpl in &self.templates {
            let key = tmpl.name.split('/').next().unwrap_or("root").to_string();
            groups.entry(key).or_default().push(tmpl);
        }

        let mut keys: Vec<String> = groups.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(items) = groups.get(&key) {
                writeln!(f, "\n{}", key)?;
                writeln!(f, "{}", "-".repeat(4))?;

                // sort items by display name (last segment)
                let mut items_sorted = items.clone();
                items_sorted.sort_by(|a, b| {
                    let an = a.name.rsplit_once('/').map(|(_, s)| s).unwrap_or(&a.name);
                    let bn = b.name.rsplit_once('/').map(|(_, s)| s).unwrap_or(&b.name);
                    an.cmp(bn)
                });

                let max_name = items_sorted
                    .iter()
                    .map(|c| {
                        c.name
                            .rsplit_once('/')
                            .map(|(_, s)| s.len())
                            .unwrap_or(c.name.len())
                    })
                    .max()
                    .unwrap_or(0);

                for item in items_sorted {
                    let display = item
                        .name
                        .rsplit_once('/')
                        .map(|(_, s)| s)
                        .unwrap_or(&item.name);

                    writeln!(
                        f,
                        "  {name:<width$}  {desc}  ({count} components)",
                        name = display,
                        width = max_name,
                        desc = item.description,
                        count = item.component_count
                    )?;
                }
            }
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
        ListTarget::Components { category, tier } => {
            list_components(component_dir, category.as_deref(), *tier, format, verbose)
        }
        ListTarget::Templates {
            detailed,
            filter,
            tech,
        } => list_templates(
            template_dir,
            *detailed,
            filter,
            tech.as_deref(),
            format,
            verbose,
        ),
    }
}

/// List available components
fn list_components(
    component_dir: &Path,
    category_filter: Option<&str>,
    tier_filter: Option<super::Tier>,
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
    // Instead of load_all (which applies tier resolution), enumerate names and apply tier filtering
    let mut all_components = Vec::new();
    for comp_type in types_to_load {
        match loader.list(comp_type) {
            Ok(names) => {
                for name in names {
                    // Skip unwanted tier variants if a tier filter is applied
                    if let Some(tfilter) = tier_filter {
                        let is_comprehensive = name.ends_with("_comprehensive");
                        let is_essential = name.ends_with("_essential");

                        let keep = match tfilter {
                            super::Tier::Essential => is_essential || !is_comprehensive,
                            super::Tier::Comprehensive => is_comprehensive || !is_essential,
                        };

                        if !keep {
                            continue;
                        }
                    }

                    // Try to load component to extract metadata/summary
                    if let Ok(c) = loader.load(&name, comp_type) {
                        all_components.push(c);
                    }
                }
            }
            Err(e) if verbose => eprintln!(
                "Warning: Could not read directory for {:?}: {}",
                comp_type, e
            ),
            Err(_) => {}
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
            // Prefer structured description from metadata if present
            let raw_summary = c.metadata.get("description").cloned().unwrap_or_else(|| {
                c.content
                    .lines()
                    .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
                    .unwrap_or("")
                    .trim()
                    .to_string()
            });

            let summary = if raw_summary.len() > 60 {
                format!("{}...", &raw_summary[..57])
            } else {
                raw_summary
            };

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
    filters: &Vec<String>,
    tech: Option<&str>,
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
        let mut infos = Vec::new();
        for name in &template_names {
            if let Ok(tmpl) = loader.load(name) {
                // apply filters: each filter must match key=value in tmpl.metadata
                let mut skip = false;
                for f in filters {
                    if let Some((k, v)) = f.split_once('=') {
                        if let Some(val) = tmpl.metadata.get(k) {
                            if val.to_lowercase() != v.to_lowercase() {
                                skip = true;
                                break;
                            }
                        } else {
                            skip = true;
                            break;
                        }
                    }
                }

                if skip {
                    continue;
                }

                // tech filter checks 'technologies' metadata substring
                if let Some(tech_q) = tech {
                    let tech_q = tech_q.to_lowercase();
                    if let Some(t) = tmpl.metadata.get("technologies") {
                        if !t.to_lowercase().contains(&tech_q) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                let mut desc = tmpl.description.clone();
                if let Some(c) = tmpl.metadata.get("complexity") {
                    if !c.is_empty() {
                        desc = format!("{} [{}]", desc, c);
                    }
                }
                if let Some(t) = tmpl.metadata.get("technologies") {
                    if !t.is_empty() {
                        desc = format!("{} ({})", desc, t);
                    }
                }

                infos.push(TemplateInfo {
                    name: name.clone(),
                    description: desc,
                    component_count: tmpl.components.len(),
                });
            }
        }

        infos
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

        let result = list_components(temp_dir.path(), None, None, OutputFormat::Human, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_templates() {
        let temp_dir = TempDir::new().unwrap();
        create_test_template(temp_dir.path(), "rust_binary");

        let result = list_templates(
            temp_dir.path(),
            false,
            &Vec::new(),
            None,
            OutputFormat::Human,
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_components_tier_filter() {
        let temp_dir = TempDir::new().unwrap();
        // create language component variants
        create_test_component(
            temp_dir.path(),
            "languages",
            "rust_essential",
            "# Rust Essential\n\nEssential content",
        );
        create_test_component(
            temp_dir.path(),
            "languages",
            "rust_comprehensive",
            "# Rust Comprehensive\n\nComprehensive content",
        );

        // Essential tier should include the essential variant
        let res1 = list_components(
            temp_dir.path(),
            Some("languages"),
            Some(crate::cli::Tier::Essential),
            OutputFormat::Human,
            false,
        );
        assert!(res1.is_ok());

        // Comprehensive tier should include the comprehensive variant
        let res2 = list_components(
            temp_dir.path(),
            Some("languages"),
            Some(crate::cli::Tier::Comprehensive),
            OutputFormat::Human,
            false,
        );
        assert!(res2.is_ok());
    }

    #[test]
    fn test_list_templates_filter_and_tech() {
        let temp_dir = TempDir::new().unwrap();

        // create a toml template with metadata
        let content = r#"
name = "filter_test"
description = "A template with metadata"

[metadata]
complexity = "advanced"
technologies = "Rust,Tokio"

[[components]]
type = "core"
name = "quick_reference"
required = true
"#;

        let file_path = temp_dir.path().join("filter_test.toml");
        fs::write(&file_path, content).unwrap();

        // detailed with matching filter should succeed
        let res = list_templates(
            temp_dir.path(),
            true,
            &vec!["complexity=advanced".to_string()],
            Some("Rust"),
            OutputFormat::Human,
            false,
        );

        assert!(res.is_ok());
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
