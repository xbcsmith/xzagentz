//! Add command for inserting new sections into AGENTS.md
//!
//! This module provides functionality to add new sections to an existing
//! AGENTS.md file while preventing duplicates and maintaining proper ordering.

use crate::components::ComponentLoader;
use crate::core::ComponentType;
use crate::error::{Error, Result};
use crate::parser::{AgentsDocument, AgentsParser, Section};
use crate::templates::PlaceholderRenderer;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Add command options
#[derive(Debug, Clone)]
pub struct AddOptions {
    /// Path to AGENTS.md file
    pub file: PathBuf,
    /// Section title to add
    pub section: String,
    /// Component to use for content
    pub component: Option<String>,
    /// Component type
    pub component_type: Option<ComponentType>,
    /// Custom content for the section
    pub content: Option<String>,
    /// Section level (1-6 for # to ######)
    pub level: usize,
    /// Position hint: "before" or "after" another section
    pub position: Option<Position>,
    /// Component directory path
    pub component_dir: Option<PathBuf>,
    /// Placeholder values
    pub placeholders: HashMap<String, String>,
    /// Allow duplicate sections
    pub allow_duplicates: bool,
}

/// Position specification for section insertion
#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    /// Insert before the specified section
    Before(String),
    /// Insert after the specified section
    After(String),
    /// Insert at the beginning
    Beginning,
    /// Insert at the end
    End,
}

impl AddOptions {
    /// Creates new add options
    ///
    /// # Arguments
    ///
    /// * `file` - Path to AGENTS.md file
    /// * `section` - Section title to add
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::cli::add::AddOptions;
    /// use std::path::PathBuf;
    ///
    /// let opts = AddOptions::new(PathBuf::from("AGENTS.md"), "New Section".to_string());
    /// assert_eq!(opts.section, "New Section");
    /// assert_eq!(opts.level, 1);
    /// ```
    pub fn new(file: PathBuf, section: String) -> Self {
        Self {
            file,
            section,
            component: None,
            component_type: None,
            content: None,
            level: 1,
            position: Some(Position::End),
            component_dir: None,
            placeholders: HashMap::new(),
            allow_duplicates: false,
        }
    }

    /// Sets the component to use for content
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

    /// Sets section level
    pub fn with_level(mut self, level: usize) -> Self {
        self.level = level.clamp(1, 6);
        self
    }

    /// Sets position for insertion
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
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

    /// Allows duplicate sections
    pub fn with_allow_duplicates(mut self, allow: bool) -> Self {
        self.allow_duplicates = allow;
        self
    }
}

/// Adds a new section to AGENTS.md
///
/// Inserts a new section with content from a component or custom text.
/// Prevents duplicate sections unless explicitly allowed.
///
/// # Arguments
///
/// * `options` - Add operation options
///
/// # Errors
///
/// Returns an error if:
/// - The file does not exist
/// - A duplicate section already exists
/// - The component cannot be loaded
/// - The file cannot be written
///
/// # Examples
///
/// ```no_run
/// use xzagentz::cli::add::{add_section, AddOptions};
/// use xzagentz::core::ComponentType;
/// use std::path::PathBuf;
///
/// let opts = AddOptions::new(PathBuf::from("AGENTS.md"), "New Section".to_string())
///     .with_component("quick_reference".to_string(), ComponentType::Core);
///
/// add_section(opts).unwrap();
/// ```
pub fn add_section(options: AddOptions) -> Result<()> {
    let content = if options.file.exists() {
        fs::read_to_string(&options.file).map_err(|e| Error::file_io(options.file.clone(), e))?
    } else {
        String::new()
    };

    let parser = AgentsParser::new();
    let mut document = if content.is_empty() {
        AgentsDocument::new()
    } else {
        parser.parse(&content)?
    };

    if !options.allow_duplicates && document.find_section(&options.section).is_some() {
        return Err(Error::validation_error(format!(
            "Section '{}' already exists",
            options.section
        )));
    }

    let section_content = get_section_content(&options)?;
    let rendered_content = render_content(&section_content, &options.placeholders)?;

    let new_section = create_section(&options.section, options.level, &rendered_content);

    insert_section(&mut document, new_section, &options.position)?;

    fs::write(&options.file, document.to_string())
        .map_err(|e| Error::file_io(options.file.clone(), e))?;

    Ok(())
}

/// Gets section content from component or custom content
fn get_section_content(options: &AddOptions) -> Result<String> {
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

/// Creates a new section with header and content
fn create_section(title: &str, level: usize, content: &str) -> Section {
    let header = format!("{} {}", "#".repeat(level), title);
    let full_content = if content.starts_with(&header) {
        content.to_string()
    } else {
        format!("{}\n\n{}", header, content.trim())
    };

    Section::new(level, title.to_string(), full_content, 0, 0)
}

/// Inserts a section into the document at the specified position
fn insert_section(
    document: &mut AgentsDocument,
    section: Section,
    position: &Option<Position>,
) -> Result<()> {
    match position {
        Some(Position::Beginning) => {
            let mut sections = vec![section];
            sections.extend(document.sections().to_vec());
            *document = rebuild_document(sections);
        }
        Some(Position::End) | None => {
            document.add_section(section);
        }
        Some(Position::Before(ref_section)) => {
            let sections = document.sections().to_vec();
            let index = sections
                .iter()
                .position(|s| s.title.to_lowercase() == ref_section.to_lowercase())
                .ok_or_else(|| Error::section_not_found(ref_section))?;

            let mut new_sections = sections[..index].to_vec();
            new_sections.push(section);
            new_sections.extend_from_slice(&sections[index..]);
            *document = rebuild_document(new_sections);
        }
        Some(Position::After(ref_section)) => {
            let sections = document.sections().to_vec();
            let index = sections
                .iter()
                .position(|s| s.title.to_lowercase() == ref_section.to_lowercase())
                .ok_or_else(|| Error::section_not_found(ref_section))?;

            let mut new_sections = sections[..=index].to_vec();
            new_sections.push(section);
            new_sections.extend_from_slice(&sections[index + 1..]);
            *document = rebuild_document(new_sections);
        }
    }

    Ok(())
}

/// Rebuilds a document from a list of sections
fn rebuild_document(sections: Vec<Section>) -> AgentsDocument {
    let mut doc = AgentsDocument::new();
    for section in sections {
        doc.add_section(section);
    }
    doc
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_add_options_new() {
        let opts = AddOptions::new(PathBuf::from("AGENTS.md"), "New Section".to_string());

        assert_eq!(opts.file, PathBuf::from("AGENTS.md"));
        assert_eq!(opts.section, "New Section");
        assert_eq!(opts.level, 1);
        assert!(!opts.allow_duplicates);
    }

    #[test]
    fn test_add_options_with_component() {
        let opts = AddOptions::new(PathBuf::from("AGENTS.md"), "Test".to_string())
            .with_component("comp".to_string(), ComponentType::Core);

        assert_eq!(opts.component, Some("comp".to_string()));
        assert_eq!(opts.component_type, Some(ComponentType::Core));
    }

    #[test]
    fn test_add_options_with_level() {
        let opts = AddOptions::new(PathBuf::from("AGENTS.md"), "Test".to_string()).with_level(3);

        assert_eq!(opts.level, 3);
    }

    #[test]
    fn test_add_new_section() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        let initial_content = r#"# Overview

This is the overview.

# Details

Some details.
"#;

        fs::write(&file_path, initial_content).unwrap();

        let opts = AddOptions::new(file_path.clone(), "New Section".to_string())
            .with_content("New section content.".to_string())
            .with_position(Position::End);

        add_section(opts).unwrap();

        let updated_content = fs::read_to_string(&file_path).unwrap();
        assert!(updated_content.contains("# New Section"));
        assert!(updated_content.contains("New section content"));
        assert!(updated_content.contains("# Overview"));
        assert!(updated_content.contains("# Details"));
    }

    #[test]
    fn test_add_maintains_order() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        fs::write(&file_path, "# First\n\nContent 1\n\n# Second\n\nContent 2").unwrap();

        let opts = AddOptions::new(file_path.clone(), "Middle".to_string())
            .with_content("Middle content".to_string())
            .with_position(Position::After("First".to_string()));

        add_section(opts).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let first_pos = content.find("# First").unwrap();
        let middle_pos = content.find("# Middle").unwrap();
        let second_pos = content.find("# Second").unwrap();

        assert!(first_pos < middle_pos);
        assert!(middle_pos < second_pos);
    }

    #[test]
    fn test_add_prevents_duplicates() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        fs::write(&file_path, "# Existing\n\nContent").unwrap();

        let opts = AddOptions::new(file_path.clone(), "Existing".to_string())
            .with_content("Duplicate".to_string());

        let result = add_section(opts);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_to_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        let opts = AddOptions::new(file_path.clone(), "First Section".to_string())
            .with_content("First content".to_string());

        add_section(opts).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("# First Section"));
        assert!(content.contains("First content"));
    }

    #[test]
    fn test_add_with_placeholders() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        let opts = AddOptions::new(file_path.clone(), "Test".to_string())
            .with_content("Project: {{project_name}}".to_string())
            .add_placeholder("project_name".to_string(), "my-project".to_string());

        add_section(opts).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("my-project"));
    }

    #[test]
    fn test_add_at_beginning() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        fs::write(&file_path, "# Existing\n\nContent").unwrap();

        let opts = AddOptions::new(file_path.clone(), "First".to_string())
            .with_content("First content".to_string())
            .with_position(Position::Beginning);

        add_section(opts).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let first_pos = content.find("# First").unwrap();
        let existing_pos = content.find("# Existing").unwrap();

        assert!(first_pos < existing_pos);
    }

    #[test]
    fn test_add_before_section() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        fs::write(&file_path, "# Target\n\nContent").unwrap();

        let opts = AddOptions::new(file_path.clone(), "Before".to_string())
            .with_content("Before content".to_string())
            .with_position(Position::Before("Target".to_string()));

        add_section(opts).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let before_pos = content.find("# Before").unwrap();
        let target_pos = content.find("# Target").unwrap();

        assert!(before_pos < target_pos);
    }

    #[test]
    fn test_create_section() {
        let section = create_section("Test", 2, "Content here");

        assert_eq!(section.title, "Test");
        assert_eq!(section.level, 2);
        assert!(section.content.contains("## Test"));
        assert!(section.content.contains("Content here"));
    }

    #[test]
    fn test_position_enum() {
        let pos1 = Position::Before("Section".to_string());
        let pos2 = Position::After("Section".to_string());
        let pos3 = Position::Beginning;
        let pos4 = Position::End;

        assert!(matches!(pos1, Position::Before(_)));
        assert!(matches!(pos2, Position::After(_)));
        assert_eq!(pos3, Position::Beginning);
        assert_eq!(pos4, Position::End);
    }

    #[test]
    fn test_get_section_content_requires_input() {
        let opts = AddOptions::new(PathBuf::from("test.md"), "Section".to_string());

        let result = get_section_content(&opts);
        assert!(result.is_err());
    }

    #[test]
    fn test_allow_duplicates() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("AGENTS.md");

        fs::write(&file_path, "# Existing\n\nContent").unwrap();

        let opts = AddOptions::new(file_path.clone(), "Existing".to_string())
            .with_content("Duplicate".to_string())
            .with_allow_duplicates(true);

        let result = add_section(opts);
        assert!(result.is_ok());
    }
}
