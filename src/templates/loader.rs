// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Template loader with filesystem support and caching
//!
//! This module provides the `TemplateLoader` for loading templates from the filesystem
//! with an in-memory cache for performance.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::ComponentType;
use crate::{Error, Result};
use std::str::FromStr;

use super::Template;

/// Loads templates from filesystem with caching
///
/// The loader supports:
/// - Loading templates from a templates directory
/// - Caching loaded templates in memory
/// - Listing available templates
/// - Custom template directories
///
/// # Examples
///
/// ```no_run
/// use xzagentz::templates::TemplateLoader;
///
/// let loader = TemplateLoader::new();
/// let template = loader.load("rust-binary").unwrap();
/// assert_eq!(template.name, "rust-binary");
/// ```
#[derive(Debug)]
pub struct TemplateLoader {
    /// Base directory for templates
    template_dir: PathBuf,

    /// In-memory cache of loaded templates
    cache: RefCell<HashMap<String, Template>>,
}

impl TemplateLoader {
    /// Creates a new template loader with default directory
    ///
    /// Uses `templates/` in the current directory as the default location.
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateLoader;
    ///
    /// let loader = TemplateLoader::new();
    /// ```
    pub fn new() -> Self {
        Self {
            template_dir: PathBuf::from("templates"),
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Creates a template loader with a custom directory
    ///
    /// # Arguments
    ///
    /// * `template_dir` - Path to the templates directory
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = TemplateLoader::with_directory(PathBuf::from("/custom/templates"));
    /// ```
    pub fn with_directory<P: Into<PathBuf>>(template_dir: P) -> Self {
        Self {
            template_dir: template_dir.into(),
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Loads a template by name
    ///
    /// First checks the cache, then loads from filesystem if not cached.
    /// The template file should be named `{name}.toml` in the templates directory.
    ///
    /// # Arguments
    ///
    /// * `name` - Template name (without .toml extension)
    ///
    /// # Returns
    ///
    /// Returns a clone of the cached or loaded template
    ///
    /// # Errors
    ///
    /// Returns `Error::TemplateNotFound` if the template file doesn't exist
    /// Returns `Error::TemplateParse` if the TOML is invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::templates::TemplateLoader;
    ///
    /// let loader = TemplateLoader::new();
    /// let template = loader.load("rust-binary")?;
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn load(&self, name: &str) -> Result<Template> {
        // Check cache first
        if let Some(template) = self.cache.borrow().get(name) {
            return Ok(template.clone());
        }

        // Load from filesystem
        let template = self.load_from_file(name)?;

        // Cache it
        self.cache
            .borrow_mut()
            .insert(name.to_string(), template.clone());

        Ok(template)
    }

    /// Loads a template from the filesystem without caching
    ///
    /// # Arguments
    ///
    /// * `name` - Template name (without .toml extension)
    ///
    /// # Errors
    ///
    /// Returns `Error::TemplateNotFound` if the file doesn't exist
    /// Returns `Error::TemplateParse` if the TOML is invalid
    fn load_from_file(&self, name: &str) -> Result<Template> {
        // Try multiple extensions in a prioritized order: toml, yaml, md
        let candidates = vec![
            self.template_dir.join(format!("{}.toml", name)),
            self.template_dir.join(format!("{}.yaml", name)),
            self.template_dir.join(format!("{}.md", name)),
            self.template_dir.join(format!("{}.markdown", name)),
        ];

        let mut found: Option<PathBuf> = None;
        for path in candidates {
            if path.exists() {
                found = Some(path);
                break;
            }
        }

        let file_path = match found {
            Some(p) => p,
            None => {
                return Err(Error::TemplateNotFound(format!(
                    "Template '{}' not found in {}",
                    name,
                    self.template_dir.display()
                )))
            }
        };

        let contents = fs::read_to_string(&file_path).map_err(|e| {
            Error::Io(format!(
                "Failed to read template file {}: {}",
                file_path.display(),
                e
            ))
        })?;

        // Determine parsing strategy based on file extension
        match file_path.extension().and_then(|s| s.to_str()) {
            Some("toml") => {
                let template: Template = toml::from_str(&contents).map_err(|e| {
                    Error::TemplateParse(format!("Failed to parse template '{}': {}", name, e))
                })?;
                Ok(template)
            }
            Some("yaml") => {
                let template: Template = serde_yaml::from_str(&contents).map_err(|e| {
                    Error::TemplateParse(format!("Failed to parse YAML template '{}': {}", name, e))
                })?;
                Ok(template)
            }
            Some("md") | Some("markdown") => {
                // Try to extract YAML frontmatter between leading '---' markers
                let mut template = Template::new(name.to_string(), String::new());

                if contents.starts_with("---") {
                    // Split into lines and find the closing frontmatter delimiter on its own line
                    let lines: Vec<&str> = contents.lines().collect();
                    let mut end_line = None;
                    for (i, line) in lines.iter().enumerate().skip(1) {
                        if line.trim() == "---" {
                            end_line = Some(i);
                            break;
                        }
                    }

                    if let Some(idx) = end_line {
                        let fm = lines[1..idx].join("\n");
                        let meta: serde_yaml::Value = serde_yaml::from_str(&fm).map_err(|e| {
                            Error::TemplateParse(format!(
                                "Failed to parse frontmatter for '{}': {}",
                                name, e
                            ))
                        })?;

                        if let Some(n) = meta.get("name").and_then(|v| v.as_str()) {
                            template.name = n.to_string();
                        }

                        if let Some(d) = meta.get("description").and_then(|v| v.as_str()) {
                            template.description = d.to_string();
                        }

                        if let Some(v) = meta.get("version").and_then(|v| v.as_str()) {
                            template.version = Some(v.to_string());
                        }

                        if let Some(a) = meta.get("author").and_then(|v| v.as_str()) {
                            template.author = Some(a.to_string());
                        }

                        if let Some(map) = meta.get("metadata") {
                            if let Some(obj) = map.as_mapping() {
                                for (k, v) in obj {
                                    if let (Some(ks), Some(vs)) = (k.as_str(), v.as_str()) {
                                        template.metadata.insert(ks.to_string(), vs.to_string());
                                    }
                                }
                            }
                        }

                        // Optionally parse components list from frontmatter if present
                        if let Some(comps) = meta.get("components") {
                            if let Some(arr) = comps.as_sequence() {
                                for comp in arr {
                                    if let Some(obj) = comp.as_mapping() {
                                        let typ = obj
                                            .get(serde_yaml::Value::String("type".to_string()))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("core");
                                        let name = obj
                                            .get(serde_yaml::Value::String("name".to_string()))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();

                                        // best effort: try to parse component type
                                        if !name.is_empty() {
                                            if let Ok(component_type) = ComponentType::from_str(typ)
                                            {
                                                template.components.push(
                                                    crate::templates::TemplateComponent::new(
                                                        component_type,
                                                        name,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // If description is empty try to derive from following content
                        if template.description.is_empty() {
                            template.description = lines[idx + 1..]
                                .iter()
                                .find(|l| !l.trim().is_empty())
                                .map(|s| s.trim().to_string())
                                .unwrap_or_else(|| "".to_string());
                        }

                        Ok(template)
                    } else {
                        Err(Error::TemplateParse(format!(
                            "Markdown template '{}' missing closing frontmatter marker",
                            name
                        )))
                    }
                } else {
                    Err(Error::TemplateParse(format!(
                        "Markdown template '{}' has no frontmatter",
                        name
                    )))
                }
            }
            other => Err(Error::TemplateParse(format!(
                "Unsupported template file extension {:?} for '{}'",
                other, name
            ))),
        }
    }

    /// Lists all available template names
    ///
    /// Scans the templates directory for .toml files and returns their names
    /// (without the .toml extension).
    ///
    /// # Returns
    ///
    /// Returns a vector of template names, sorted alphabetically
    ///
    /// # Errors
    ///
    /// Returns `Error::Io` if the directory cannot be read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::templates::TemplateLoader;
    ///
    /// let loader = TemplateLoader::new();
    /// let templates = loader.list()?;
    /// for name in templates {
    ///     println!("Template: {}", name);
    /// }
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn list(&self) -> Result<Vec<String>> {
        if !self.template_dir.exists() {
            return Ok(Vec::new());
        }

        let mut templates = Vec::new();

        for entry in walkdir::WalkDir::new(&self.template_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            let path = entry.path().to_path_buf();

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                // derive a relative name (path without extension, relative to template_dir)
                if let Ok(rel) = path.strip_prefix(&self.template_dir) {
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let rel_name = if let Some(parent) = rel.parent() {
                        if parent.as_os_str().is_empty() {
                            stem.to_string()
                        } else {
                            format!("{}/{}", parent.to_string_lossy(), stem)
                        }
                    } else {
                        stem.to_string()
                    };

                    match ext {
                        "toml" | "yaml" => templates.push(rel_name),
                        "md" | "markdown" => {
                            if let Ok(contents) = fs::read_to_string(&path) {
                                if contents.starts_with("---") {
                                    templates.push(rel_name);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        templates.sort();
        Ok(templates)
    }

    /// Loads all available templates
    ///
    /// # Returns
    ///
    /// Returns a vector of all loaded templates
    ///
    /// # Errors
    ///
    /// Returns an error if any template fails to load
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::templates::TemplateLoader;
    ///
    /// let loader = TemplateLoader::new();
    /// let templates = loader.load_all()?;
    /// println!("Loaded {} templates", templates.len());
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn load_all(&self) -> Result<Vec<Template>> {
        let names = self.list()?;
        let mut templates = Vec::new();

        for name in names {
            templates.push(self.load(&name)?);
        }

        Ok(templates)
    }

    /// Clears the template cache
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateLoader;
    ///
    /// let loader = TemplateLoader::new();
    /// loader.clear_cache();
    /// assert_eq!(loader.cache_size(), 0);
    /// ```
    pub fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    /// Returns the number of cached templates
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateLoader;
    ///
    /// let loader = TemplateLoader::new();
    /// assert_eq!(loader.cache_size(), 0);
    /// ```
    pub fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }

    /// Returns the template directory path
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::TemplateLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = TemplateLoader::new();
    /// assert_eq!(loader.template_dir(), &PathBuf::from("templates"));
    /// ```
    pub fn template_dir(&self) -> &Path {
        &self.template_dir
    }
}

impl Default for TemplateLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_template_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(format!("{}.toml", name));
        fs::write(&file_path, content).unwrap();
        file_path
    }

    fn create_test_markdown_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(format!("{}.md", name));
        fs::write(&file_path, content).unwrap();
        file_path
    }

    fn create_valid_template_toml() -> String {
        r#"
name = "test-template"
description = "A test template"
version = "1.0.0"

[[components]]
type = "core"
name = "quick_reference"
required = true

[[components]]
type = "languages"
name = "rust"
required = true
"#
        .to_string()
    }

    #[test]
    fn test_template_loader_new() {
        let loader = TemplateLoader::new();
        assert_eq!(loader.template_dir(), Path::new("templates"));
        assert_eq!(loader.cache_size(), 0);
    }

    #[test]
    fn test_template_loader_with_directory() {
        let loader = TemplateLoader::with_directory("/custom/path");
        assert_eq!(loader.template_dir(), Path::new("/custom/path"));
    }

    #[test]
    fn test_template_loader_default() {
        let loader = TemplateLoader::default();
        assert_eq!(loader.template_dir(), Path::new("templates"));
    }

    #[test]
    fn test_load_template_success() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = create_valid_template_toml();
        create_test_template_file(temp_dir.path(), "test", &toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let template = loader.load("test").unwrap();

        assert_eq!(template.name, "test-template");
        assert_eq!(template.description, "A test template");
        assert_eq!(template.version, Some("1.0.0".to_string()));
        assert_eq!(template.components.len(), 2);
    }

    #[test]
    fn test_load_template_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let loader = TemplateLoader::with_directory(temp_dir.path());

        let result = loader.load("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::TemplateNotFound(_))));
    }

    #[test]
    fn test_load_template_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_toml = "this is not valid toml: : :";
        create_test_template_file(temp_dir.path(), "invalid", invalid_toml);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let result = loader.load("invalid");

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::TemplateParse(_))));
    }

    #[test]
    fn test_load_template_caching() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = create_valid_template_toml();
        create_test_template_file(temp_dir.path(), "cached", &toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());

        assert_eq!(loader.cache_size(), 0);

        // First load - from file
        let template1 = loader.load("cached").unwrap();
        assert_eq!(loader.cache_size(), 1);

        // Second load - from cache
        let template2 = loader.load("cached").unwrap();
        assert_eq!(loader.cache_size(), 1);

        assert_eq!(template1.name, template2.name);
    }

    #[test]
    fn test_list_templates() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = create_valid_template_toml();

        create_test_template_file(temp_dir.path(), "template1", &toml_content);
        create_test_template_file(temp_dir.path(), "template2", &toml_content);
        create_test_template_file(temp_dir.path(), "template3", &toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let templates = loader.list().unwrap();

        assert_eq!(templates.len(), 3);
        assert_eq!(templates[0], "template1");
        assert_eq!(templates[1], "template2");
        assert_eq!(templates[2], "template3");
    }

    #[test]
    fn test_list_templates_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let loader = TemplateLoader::with_directory(temp_dir.path());

        let templates = loader.list().unwrap();
        assert_eq!(templates.len(), 0);
    }

    #[test]
    fn test_list_templates_nonexistent_directory() {
        let loader = TemplateLoader::with_directory("/nonexistent/path");
        let templates = loader.list().unwrap();
        assert_eq!(templates.len(), 0);
    }

    #[test]
    fn test_list_templates_ignores_non_toml_files() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = create_valid_template_toml();

        create_test_template_file(temp_dir.path(), "template1", &toml_content);
        fs::write(temp_dir.path().join("readme.md"), "# Readme").unwrap();
        fs::write(temp_dir.path().join("data.json"), "{}").unwrap();

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let templates = loader.list().unwrap();

        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0], "template1");
    }

    #[test]
    fn test_list_templates_includes_markdown_with_frontmatter() {
        let temp_dir = TempDir::new().unwrap();

        let md_content = r#"---
name: md-template
description: A markdown template
---

# Md Template

This is a template in markdown with frontmatter.
"#;

        create_test_template_file(temp_dir.path(), "template1", &create_valid_template_toml());
        create_test_markdown_file(temp_dir.path(), "md_template", md_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let templates = loader.list().unwrap();

        // we expect both the toml and md template to be listed
        assert!(templates.contains(&"template1".to_string()));
        assert!(templates.contains(&"md_template".to_string()));
    }

    #[test]
    fn test_load_markdown_template_frontmatter() {
        let temp_dir = TempDir::new().unwrap();

        let md_content = r#"---
name: md-template
description: A markdown template
version: 0.1.0
author: Tester
metadata:
    category: prompts
    complexity: essential
---

# Md Template
"#;

        create_test_markdown_file(temp_dir.path(), "md_template", md_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let tmpl = loader.load("md_template").unwrap();

        assert_eq!(tmpl.name, "md-template");
        assert_eq!(tmpl.description, "A markdown template");
        assert_eq!(tmpl.version, Some("0.1.0".to_string()));
        assert_eq!(tmpl.author, Some("Tester".to_string()));
        assert_eq!(tmpl.metadata.get("category"), Some(&"prompts".to_string()));
        assert_eq!(tmpl.components.len(), 0);
    }

    #[test]
    fn test_load_all_templates() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = create_valid_template_toml();

        create_test_template_file(temp_dir.path(), "template1", &toml_content);
        create_test_template_file(temp_dir.path(), "template2", &toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let templates = loader.load_all().unwrap();

        assert_eq!(templates.len(), 2);
        assert_eq!(loader.cache_size(), 2);
    }

    #[test]
    fn test_clear_cache() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = create_valid_template_toml();
        create_test_template_file(temp_dir.path(), "cached", &toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());

        loader.load("cached").unwrap();
        assert_eq!(loader.cache_size(), 1);

        loader.clear_cache();
        assert_eq!(loader.cache_size(), 0);
    }

    #[test]
    fn test_template_with_optional_components() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = r#"
name = "mixed-template"
description = "Template with optional components"

[[components]]
type = "core"
name = "quick_reference"
required = true

[[components]]
type = "tools"
name = "docker"
required = false
"#;
        create_test_template_file(temp_dir.path(), "mixed", toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let template = loader.load("mixed").unwrap();

        assert_eq!(template.components.len(), 2);
        assert!(template.components[0].required);
        assert!(!template.components[1].required);
    }

    #[test]
    fn test_template_with_ordering() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = r#"
name = "ordered-template"
description = "Template with component ordering"

[[components]]
type = "core"
name = "first"
required = true
order = 1

[[components]]
type = "core"
name = "second"
required = true
order = 2
"#;
        create_test_template_file(temp_dir.path(), "ordered", toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let template = loader.load("ordered").unwrap();

        assert_eq!(template.components[0].order, Some(1));
        assert_eq!(template.components[1].order, Some(2));
    }

    #[test]
    fn test_template_with_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let toml_content = r#"
name = "metadata-template"
description = "Template with metadata"
version = "2.0.0"
author = "Test Author"

[metadata]
category = "development"
language = "rust"

[[components]]
type = "core"
name = "overview"
"#;
        create_test_template_file(temp_dir.path(), "metadata", toml_content);

        let loader = TemplateLoader::with_directory(temp_dir.path());
        let template = loader.load("metadata").unwrap();

        assert_eq!(template.author, Some("Test Author".to_string()));
        assert_eq!(
            template.metadata.get("category"),
            Some(&"development".to_string())
        );
        assert_eq!(template.metadata.get("language"), Some(&"rust".to_string()));
    }
}
