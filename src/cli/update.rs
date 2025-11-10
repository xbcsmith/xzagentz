// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Update command for modifying existing AGENTS.md sections
//!
//! This module provides functionality to update specific sections in an existing
//! AGENTS.md file while preserving other sections and creating backups.

use crate::components::ComponentLoader;
use crate::core::ComponentType;
use crate::error::{Error, Result};
use crate::parser::{AgentsDocument, AgentsParser};
use crate::templates::PlaceholderRenderer;
use chrono::Local;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Update command options
#[derive(Debug, Clone)]
pub struct UpdateOptions {
    /// Path to AGENTS.md file to update
    pub file: PathBuf,
    /// Section to update
    pub section: String,
    /// Component to use for replacement
    pub component: Option<String>,
    /// Component type
    pub component_type: Option<ComponentType>,
    /// Custom content to replace section with
    pub content: Option<String>,
    /// Whether to create a backup before updating
    pub backup: bool,
    /// Component directory path
    pub component_dir: Option<PathBuf>,
    /// Placeholder values
    pub placeholders: HashMap<String, String>,
}

impl UpdateOptions {
    /// Creates new update options
    ///
    /// # Arguments
    ///
    /// * `file` - Path to AGENTS.md file
    /// * `section` - Section title to update
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::cli::update::UpdateOptions;
    /// use std::path::PathBuf;
    ///
    /// let opts = UpdateOptions::new(PathBuf::from("AGENTS.md"), "Quick Reference".to_string());
    /// assert_eq!(opts.section, "Quick Reference");
    /// ```
    pub fn new(file: PathBuf, section: String) -> Self {
        Self {
            file,
            section,
            component: None,
            component_type: None,
            content: None,
            backup: true,
            component_dir: None,
            placeholders: HashMap::new(),
        }
    }

    /// Sets the component to use for replacement
    pub fn with_component(mut self, name: String, component_type: ComponentType) -> Self {
        self.component = Some(name);
        self.component_type = Some(component_type);
        self
    }

    /// Sets custom content
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets whether to create backup
    pub fn with_backup(mut self, backup: bool) -> Self {
        self.backup = backup;
        self
    }

    /// Sets component directory
    pub fn with_component_dir(mut self, dir: PathBuf) -> Self {
        self.component_dir = Some(dir);
        self
    }

    /// Adds a placeholder value
    pub fn add_placeholder(mut self, key: String, value: String) -> Self {
        self.placeholders.insert(key, value);
        self
    }
}

/// Updates an existing section in AGENTS.md
///
/// Replaces the specified section with new content from a component or custom text.
/// Creates a timestamped backup before modifying the file.
///
/// # Arguments
///
/// * `options` - Update operation options
///
/// # Returns
///
/// Returns the path to the backup file if created
///
/// # Errors
///
/// Returns an error if:
/// - The file does not exist
/// - The section is not found
/// - The component cannot be loaded
/// - The file cannot be written
///
/// # Examples
///
/// ```no_run
/// use xzagentz::cli::update::{update_section, UpdateOptions};
/// use xzagentz::core::ComponentType;
/// use std::path::PathBuf;
///
/// let opts = UpdateOptions::new(PathBuf::from("AGENTS.md"), "Quick Reference".to_string())
///     .with_component("quick_reference".to_string(), ComponentType::Core);
///
/// let backup_path = update_section(opts).unwrap();
/// println!("Created backup: {}", backup_path.display());
/// ```
pub fn update_section(options: UpdateOptions) -> Result<PathBuf> {
    if !options.file.exists() {
        return Err(Error::FileNotFound {
            path: options.file.clone(),
        });
    }

    let content =
        fs::read_to_string(&options.file).map_err(|e| Error::file_io(options.file.clone(), e))?;

    let parser = AgentsParser::new();
    let mut document = parser.parse(&content)?;

    let _section = document
        .find_section(&options.section)
        .ok_or_else(|| Error::section_not_found(&options.section))?
        .clone();

    let new_content = get_replacement_content(&options)?;
    let rendered_content = render_content(&new_content, &options.placeholders)?;

    let backup_path = if options.backup {
        create_backup(&options.file)?
    } else {
        PathBuf::new()
    };

    let updated_document =
        replace_section_content(&mut document, &options.section, &rendered_content)?;

    fs::write(&options.file, updated_document.to_string())
        .map_err(|e| Error::file_io(options.file.clone(), e))?;

    Ok(backup_path)
}

/// Gets replacement content from component or custom content
fn get_replacement_content(options: &UpdateOptions) -> Result<String> {
    if let Some(ref content) = options.content {
        return Ok(content.clone());
    }

    if let (Some(ref component_name), Some(component_type)) =
        (&options.component, &options.component_type)
    {
        let loader = ComponentLoader::new(options.component_dir.clone());
        let component = loader.load(component_name, *component_type)?;
        return Ok(component.content);
    }

    Err(Error::validation_error(
        "Either component or content must be specified",
    ))
}

/// Renders content with placeholders
fn render_content(content: &str, placeholders: &HashMap<String, String>) -> Result<String> {
    if placeholders.is_empty() {
        return Ok(content.to_string());
    }

    let renderer = PlaceholderRenderer::new(placeholders.clone());
    renderer.render(content)
}

/// Replaces a section's content in the document
fn replace_section_content(
    document: &mut AgentsDocument,
    section_title: &str,
    new_content: &str,
) -> Result<AgentsDocument> {
    let section = document
        .find_section_mut(section_title)
        .ok_or_else(|| Error::section_not_found(section_title))?;

    let header = section.header();
    let updated_content = if new_content.starts_with(&header) {
        new_content.to_string()
    } else {
        format!("{}\n\n{}", header, new_content.trim())
    };

    section.content = updated_content;

    Ok(document.clone())
}

/// Creates a timestamped backup of a file
///
/// # Arguments
///
/// * `file_path` - Path to file to backup
///
/// # Returns
///
/// Returns the path to the created backup file
///
/// # Errors
///
/// Returns an error if the backup cannot be created
pub fn create_backup(file_path: &Path) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!(
        "{}.backup.{}",
        file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file"),
        timestamp
    );

    let backup_path = if let Some(parent) = file_path.parent() {
        parent.join(backup_name)
    } else {
        PathBuf::from(backup_name)
    };

    fs::copy(file_path, &backup_path).map_err(|e| Error::file_io(backup_path.clone(), e))?;

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Section;
    use tempfile::TempDir;

    #[test]
    fn test_update_options_new() {
        let opts = UpdateOptions::new(PathBuf::from("AGENTS.md"), "Test Section".to_string());

        assert_eq!(opts.file, PathBuf::from("AGENTS.md"));
        assert_eq!(opts.section, "Test Section");
        assert!(opts.backup);
        assert!(opts.component.is_none());
        assert!(opts.content.is_none());
    }

    #[test]
    fn test_update_options_with_component() {
        let opts = UpdateOptions::new(PathBuf::from("AGENTS.md"), "Test".to_string())
            .with_component("quick_ref".to_string(), ComponentType::Core);

        assert_eq!(opts.component, Some("quick_ref".to_string()));
        assert_eq!(opts.component_type, Some(ComponentType::Core));
    }

    #[test]
    fn test_update_options_with_content() {
        let opts = UpdateOptions::new(PathBuf::from("AGENTS.md"), "Test".to_string())
            .with_content("Custom content".to_string());

        assert_eq!(opts.content, Some("Custom content".to_string()));
    }

    #[test]
    fn test_update_existing_section() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        let initial_content = r#"# Overview

This is the overview.

# Quick Reference

Old content here.

# Details

Some details.
"#;

        fs::write(&file_path, initial_content).unwrap();

        let opts = UpdateOptions::new(file_path.clone(), "Quick Reference".to_string())
            .with_content("New updated content.".to_string())
            .with_backup(true);

        let backup_path = update_section(opts).unwrap();

        assert!(backup_path.exists());
        assert!(backup_path.to_string_lossy().contains("backup"));

        let updated_content = fs::read_to_string(&file_path).unwrap();
        assert!(updated_content.contains("# Quick Reference"));
        assert!(updated_content.contains("New updated content"));
        assert!(!updated_content.contains("Old content here"));
        assert!(updated_content.contains("# Overview"));
        assert!(updated_content.contains("# Details"));
    }

    #[test]
    fn test_update_creates_backup() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");

        fs::write(&file_path, "Original content").unwrap();

        let backup_path = create_backup(&file_path).unwrap();

        assert!(backup_path.exists());
        assert_ne!(backup_path, file_path);

        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, "Original content");
    }

    #[test]
    fn test_update_preserves_other_sections() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        let initial = r#"# Section 1

Content 1

# Section 2

Content 2

# Section 3

Content 3
"#;

        fs::write(&file_path, initial).unwrap();

        let opts = UpdateOptions::new(file_path.clone(), "Section 2".to_string())
            .with_content("Updated content 2".to_string())
            .with_backup(false);

        update_section(opts).unwrap();

        let updated = fs::read_to_string(&file_path).unwrap();
        assert!(updated.contains("Content 1"));
        assert!(updated.contains("Updated content 2"));
        assert!(updated.contains("Content 3"));
        assert!(!updated.contains("Content 2"));
    }

    #[test]
    fn test_update_nonexistent_section() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        fs::write(&file_path, "# Only Section\n\nContent").unwrap();

        let opts = UpdateOptions::new(file_path.clone(), "Missing Section".to_string())
            .with_content("New content".to_string());

        let result = update_section(opts);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_content_with_placeholders() {
        let mut placeholders = HashMap::new();
        placeholders.insert("project_name".to_string(), "my-project".to_string());
        placeholders.insert("version".to_string(), "1.0.0".to_string());

        let content = "Project: {{project_name}}\nVersion: {{version}}";
        let rendered = render_content(content, &placeholders).unwrap();

        assert!(rendered.contains("my-project"));
        assert!(rendered.contains("1.0.0"));
    }

    #[test]
    fn test_render_content_without_placeholders() {
        let content = "Plain content";
        let rendered = render_content(content, &HashMap::new()).unwrap();

        assert_eq!(rendered, "Plain content");
    }

    #[test]
    fn test_replace_section_content() {
        let mut doc = AgentsDocument::new();
        doc.add_section(Section::new(
            1,
            "Test Section".to_string(),
            "# Test Section\n\nOld content".to_string(),
            0,
            2,
        ));

        let updated = replace_section_content(&mut doc, "Test Section", "New content").unwrap();

        let section = updated.find_section("Test Section").unwrap();
        assert!(section.content.contains("New content"));
        assert!(!section.content.contains("Old content"));
    }

    #[test]
    fn test_get_replacement_content_from_custom() {
        let opts = UpdateOptions::new(PathBuf::from("test.md"), "Section".to_string())
            .with_content("Custom content".to_string());

        let content = get_replacement_content(&opts).unwrap();
        assert_eq!(content, "Custom content");
    }

    #[test]
    fn test_get_replacement_content_requires_input() {
        let opts = UpdateOptions::new(PathBuf::from("test.md"), "Section".to_string());

        let result = get_replacement_content(&opts);
        assert!(result.is_err());
    }
}
