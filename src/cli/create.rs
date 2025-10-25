//! Create command implementation for generating AGENTS.md files
//!
//! This module provides functionality to create new AGENTS.md files from templates
//! and components, with support for interactive mode, placeholder replacement,
//! and README.md context parsing.
//!
//! # Features
//!
//! - Template-based file generation
//! - Component composition with ordering
//! - Placeholder rendering
//! - Interactive metadata collection
//! - README.md parsing for project context
//! - Backup creation before overwrite
//! - Force flag to overwrite existing files
//!
//! # Examples
//!
//! ```no_run
//! use xzagentz::cli::create::CreateCommand;
//! use std::path::PathBuf;
//!
//! let cmd = CreateCommand {
//!     output: PathBuf::from("AGENTS.md"),
//!     template: Some("rust-binary".to_string()),
//!     force: false,
//!     interactive: false,
//! };
//!
//! // cmd.execute(&config)?;
//! ```

use crate::components::{Component, ComponentLoader};
use crate::core::ComponentType;
use crate::error::{Error, Result};
use crate::templates::{PlaceholderRenderer, Template, TemplateLoader};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Configuration for the create command
#[derive(Debug, Clone)]
pub struct CreateConfig {
    /// Output file path
    pub output: PathBuf,
    /// Template name to use (optional)
    pub template: Option<String>,
    /// Force overwrite if file exists
    pub force: bool,
    /// Interactive mode
    pub interactive: bool,
    /// Template directory
    pub template_dir: PathBuf,
    /// Component directory
    pub component_dir: PathBuf,
    /// Enable verbose output
    pub verbose: bool,
}

/// Represents the create command
#[derive(Debug, Clone)]
pub struct CreateCommand {
    /// Output file path
    pub output: PathBuf,
    /// Template to use
    pub template: Option<String>,
    /// Force overwrite if file exists
    pub force: bool,
    /// Interactive mode
    pub interactive: bool,
}

impl CreateCommand {
    /// Creates a new CreateCommand
    pub fn new(output: PathBuf, template: Option<String>, force: bool, interactive: bool) -> Self {
        Self {
            output,
            template,
            force,
            interactive,
        }
    }

    /// Executes the create command
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the create operation
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or an Error on failure
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Output file already exists and force is false
    /// - Template not found
    /// - Component loading fails
    /// - File creation fails
    pub fn execute(&self, config: &CreateConfig) -> Result<()> {
        // Check if file exists
        if self.output.exists() && !self.force {
            return Err(Error::file_already_exists(self.output.clone()));
        }

        // Load project metadata from README if available
        let metadata = if Path::new("README.md").exists() {
            ReadmeParser::parse("README.md")?
        } else {
            ProjectMetadata::default()
        };

        // Collect additional metadata in interactive mode
        let metadata = if self.interactive {
            InteractivePrompt::collect_metadata(metadata)?
        } else {
            metadata
        };

        // Load template or use default
        let template = if let Some(template_name) = &self.template {
            let loader = TemplateLoader::with_directory(config.template_dir.clone());
            Some(loader.load(template_name)?)
        } else {
            None
        };

        // Create file creator
        let creator = FileCreator::new(config.clone());

        // Generate content
        let content = creator.generate_content(&template, &metadata)?;

        // Preview in interactive mode
        if self.interactive {
            InteractivePrompt::preview_content(&content)?;
            if !InteractivePrompt::confirm_write()? {
                println!("Create cancelled by user");
                return Ok(());
            }
        }

        // Create backup if file exists
        if self.output.exists() && self.force {
            creator.create_backup(&self.output)?;
        }

        // Write file
        creator.write_file(&self.output, &content)?;

        println!("Successfully created: {}", self.output.display());
        Ok(())
    }
}

/// Handles file creation with backups
#[derive(Debug, Clone)]
pub struct FileCreator {
    config: CreateConfig,
}

impl FileCreator {
    /// Creates a new FileCreator
    pub fn new(config: CreateConfig) -> Self {
        Self { config }
    }

    /// Generates content from template and metadata
    ///
    /// # Arguments
    ///
    /// * `template` - Optional template to use
    /// * `metadata` - Project metadata for placeholder replacement
    ///
    /// # Returns
    ///
    /// Returns the generated content as a String
    ///
    /// # Errors
    ///
    /// Returns an error if component loading or rendering fails
    pub fn generate_content(
        &self,
        template: &Option<Template>,
        metadata: &ProjectMetadata,
    ) -> Result<String> {
        let loader = ComponentLoader::new(Some(self.config.component_dir.clone()));

        let components = if let Some(tmpl) = template {
            // Load components from template
            self.load_template_components(&loader, tmpl)?
        } else {
            // Use default component set
            self.load_default_components(&loader)?
        };

        // Build placeholder map
        let placeholders = metadata.to_placeholder_map();

        // Compose components
        let content = ComponentComposer::compose(&components, &placeholders)?;

        Ok(content)
    }

    /// Loads components specified in template
    fn load_template_components(
        &self,
        loader: &ComponentLoader,
        template: &Template,
    ) -> Result<Vec<Component>> {
        let mut components = Vec::new();
        let mut template_components = template.components.clone();

        // Sort by order if specified
        template_components.sort_by_key(|c| c.order.unwrap_or(usize::MAX));

        for tmpl_comp in &template_components {
            match loader.load(&tmpl_comp.name, tmpl_comp.component_type) {
                Ok(comp) => components.push(comp),
                Err(e) => {
                    if tmpl_comp.required {
                        return Err(e);
                    } else if self.config.verbose {
                        eprintln!("Warning: Optional component not found: {}", tmpl_comp.name);
                    }
                }
            }
        }

        Ok(components)
    }

    /// Loads default components
    fn load_default_components(&self, loader: &ComponentLoader) -> Result<Vec<Component>> {
        let mut components = Vec::new();

        // Load core components in order
        let core_names = vec![
            "quick_reference",
            "critical_rules",
            "project_overview",
            "development_workflow",
        ];

        for name in core_names {
            if let Ok(comp) = loader.load(name, ComponentType::Core) {
                components.push(comp);
            }
        }

        Ok(components)
    }

    /// Creates a backup of the existing file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to backup
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success
    ///
    /// # Errors
    ///
    /// Returns an error if the backup creation fails
    pub fn create_backup(&self, path: &Path) -> Result<()> {
        let backup_path = Self::get_backup_path(path);
        fs::copy(path, &backup_path).map_err(|e| {
            Error::file_create_error(
                backup_path.clone(),
                format!("Failed to create backup: {}", e),
            )
        })?;

        if self.config.verbose {
            println!("Created backup: {}", backup_path.display());
        }

        Ok(())
    }

    /// Gets the backup path for a file
    fn get_backup_path(path: &Path) -> PathBuf {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
        path.with_file_name(format!("{}.backup.{}", filename, timestamp))
    }

    /// Writes content to file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to write to
    /// * `content` - Content to write
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success
    ///
    /// # Errors
    ///
    /// Returns an error if the write fails
    pub fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        fs::write(path, content).map_err(|e| {
            Error::file_create_error(path.to_path_buf(), format!("Write failed: {}", e))
        })?;
        Ok(())
    }
}

/// Composes components into final content
pub struct ComponentComposer;

impl ComponentComposer {
    /// Composes components with placeholder replacement
    ///
    /// # Arguments
    ///
    /// * `components` - Components to compose
    /// * `placeholders` - Placeholder values
    ///
    /// # Returns
    ///
    /// Returns the composed content
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails
    pub fn compose(
        components: &[Component],
        placeholders: &HashMap<String, String>,
    ) -> Result<String> {
        let mut output = String::new();
        let renderer = PlaceholderRenderer::new(placeholders.clone());

        for (i, component) in components.iter().enumerate() {
            // Render placeholders in component content
            let rendered = renderer.render(&component.content)?;

            output.push_str(&rendered);

            // Add separator between components
            if i < components.len() - 1 {
                output.push_str("\n\n---\n\n");
            }
        }

        Ok(output)
    }
}

/// Parses README.md for project metadata
pub struct ReadmeParser;

impl ReadmeParser {
    /// Parses README.md file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to README.md
    ///
    /// # Returns
    ///
    /// Returns ProjectMetadata extracted from README
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read
    pub fn parse(path: impl AsRef<Path>) -> Result<ProjectMetadata> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| Error::file_io(path.as_ref().to_path_buf(), e))?;

        let mut metadata = ProjectMetadata::default();

        // Extract project name from first heading
        if let Some(name) = Self::extract_first_heading(&content) {
            metadata.project_name = name;
        }

        // Detect language from code blocks or badges
        metadata.language = Self::detect_language(&content);

        // Detect project type from keywords
        metadata.project_type = Self::infer_project_type(&content);

        // Extract description from first paragraph
        if let Some(desc) = Self::extract_description(&content) {
            metadata.description = Some(desc);
        }

        Ok(metadata)
    }

    /// Extracts the first heading from content
    fn extract_first_heading(content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return Some(trimmed.trim_start_matches("# ").trim().to_string());
            }
        }
        None
    }

    /// Detects primary language from content
    fn detect_language(content: &str) -> String {
        let content_lower = content.to_lowercase();

        // Check for language badges or mentions
        if content_lower.contains("rust") || content_lower.contains("cargo.toml") {
            return "rust".to_string();
        }
        if content_lower.contains("python") || content_lower.contains("setup.py") {
            return "python".to_string();
        }
        if content_lower.contains("javascript") || content_lower.contains("package.json") {
            return "javascript".to_string();
        }
        if content_lower.contains("typescript") {
            return "typescript".to_string();
        }
        if content_lower.contains("go") || content_lower.contains("go.mod") {
            return "go".to_string();
        }

        "unknown".to_string()
    }

    /// Infers project type from content
    fn infer_project_type(content: &str) -> String {
        let content_lower = content.to_lowercase();

        if content_lower.contains("cli") || content_lower.contains("command-line") {
            return "cli".to_string();
        }
        if content_lower.contains("web")
            || content_lower.contains("api")
            || content_lower.contains("server")
        {
            return "web-service".to_string();
        }
        if content_lower.contains("library") || content_lower.contains("crate") {
            return "library".to_string();
        }

        "application".to_string()
    }

    /// Extracts description from first paragraph after heading
    fn extract_description(content: &str) -> Option<String> {
        let mut found_heading = false;
        let mut description = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('#') {
                found_heading = true;
                continue;
            }

            if found_heading && !trimmed.is_empty() && !trimmed.starts_with('[') {
                description.push_str(trimmed);
                description.push(' ');

                // Stop at next heading or empty line
                if description.len() > 200 {
                    break;
                }
            }

            if found_heading && !description.is_empty() && trimmed.is_empty() {
                break;
            }
        }

        if description.is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        }
    }
}

/// Project metadata for placeholder replacement
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    /// Project name
    pub project_name: String,
    /// Primary programming language
    pub language: String,
    /// Project type (cli, web-service, library, etc.)
    pub project_type: String,
    /// Project description
    pub description: Option<String>,
    /// Project author
    pub author: Option<String>,
    /// Project version
    pub version: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            project_name: "my-project".to_string(),
            language: "rust".to_string(),
            project_type: "application".to_string(),
            description: None,
            author: None,
            version: "0.1.0".to_string(),
        }
    }
}

impl ProjectMetadata {
    /// Converts metadata to placeholder map
    pub fn to_placeholder_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("project_name".to_string(), self.project_name.clone());
        map.insert("PROJECT_NAME".to_string(), self.project_name.to_uppercase());
        map.insert("language".to_string(), self.language.clone());
        map.insert("project_type".to_string(), self.project_type.clone());
        map.insert("version".to_string(), self.version.clone());

        if let Some(desc) = &self.description {
            map.insert("description".to_string(), desc.clone());
        }

        if let Some(author) = &self.author {
            map.insert("author".to_string(), author.clone());
        }

        // Add current date
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        map.insert("date".to_string(), date);

        map
    }
}

/// Interactive prompt for metadata collection
pub struct InteractivePrompt;

impl InteractivePrompt {
    /// Collects metadata interactively
    ///
    /// # Arguments
    ///
    /// * `defaults` - Default metadata values
    ///
    /// # Returns
    ///
    /// Returns updated metadata
    ///
    /// # Errors
    ///
    /// Returns an error if input fails
    pub fn collect_metadata(defaults: ProjectMetadata) -> Result<ProjectMetadata> {
        println!("\n=== Interactive Mode: Project Metadata ===\n");

        let project_name = Self::prompt_with_default("Project name", &defaults.project_name)?;
        let language = Self::prompt_with_default("Language", &defaults.language)?;
        let project_type = Self::prompt_with_default("Project type", &defaults.project_type)?;
        let description = Self::prompt_optional("Description")?;
        let author = Self::prompt_optional("Author")?;
        let version = Self::prompt_with_default("Version", &defaults.version)?;

        Ok(ProjectMetadata {
            project_name,
            language,
            project_type,
            description,
            author,
            version,
        })
    }

    /// Prompts for input with default value
    fn prompt_with_default(prompt: &str, default: &str) -> Result<String> {
        print!("{} [{}]: ", prompt, default);
        io::stdout().flush().map_err(|e| Error::Io(e.to_string()))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| Error::Io(e.to_string()))?;

        let trimmed = input.trim();
        if trimmed.is_empty() {
            Ok(default.to_string())
        } else {
            Ok(trimmed.to_string())
        }
    }

    /// Prompts for optional input
    fn prompt_optional(prompt: &str) -> Result<Option<String>> {
        print!("{} (optional): ", prompt);
        io::stdout().flush().map_err(|e| Error::Io(e.to_string()))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| Error::Io(e.to_string()))?;

        let trimmed = input.trim();
        if trimmed.is_empty() {
            Ok(None)
        } else {
            Ok(Some(trimmed.to_string()))
        }
    }

    /// Shows content preview
    pub fn preview_content(content: &str) -> Result<()> {
        println!("\n=== Preview ===\n");

        // Show first 20 lines
        let lines: Vec<&str> = content.lines().take(20).collect();
        for line in lines {
            println!("{}", line);
        }

        let total_lines = content.lines().count();
        if total_lines > 20 {
            println!("\n... ({} more lines)", total_lines - 20);
        }

        Ok(())
    }

    /// Confirms write operation
    pub fn confirm_write() -> Result<bool> {
        print!("\nWrite this file? [Y/n]: ");
        io::stdout().flush().map_err(|e| Error::Io(e.to_string()))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| Error::Io(e.to_string()))?;

        let trimmed = input.trim().to_lowercase();
        Ok(trimmed.is_empty() || trimmed == "y" || trimmed == "yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_command_new() {
        let cmd = CreateCommand::new(
            PathBuf::from("AGENTS.md"),
            Some("rust-binary".to_string()),
            false,
            false,
        );

        assert_eq!(cmd.output, PathBuf::from("AGENTS.md"));
        assert_eq!(cmd.template, Some("rust-binary".to_string()));
        assert!(!cmd.force);
        assert!(!cmd.interactive);
    }

    #[test]
    fn test_project_metadata_default() {
        let metadata = ProjectMetadata::default();
        assert_eq!(metadata.project_name, "my-project");
        assert_eq!(metadata.language, "rust");
        assert_eq!(metadata.version, "0.1.0");
    }

    #[test]
    fn test_metadata_to_placeholder_map() {
        let metadata = ProjectMetadata {
            project_name: "test-project".to_string(),
            language: "rust".to_string(),
            project_type: "cli".to_string(),
            description: Some("A test project".to_string()),
            author: Some("Test Author".to_string()),
            version: "1.0.0".to_string(),
        };

        let map = metadata.to_placeholder_map();

        assert_eq!(map.get("project_name").unwrap(), "test-project");
        assert_eq!(map.get("PROJECT_NAME").unwrap(), "TEST-PROJECT");
        assert_eq!(map.get("language").unwrap(), "rust");
        assert_eq!(map.get("version").unwrap(), "1.0.0");
        assert_eq!(map.get("description").unwrap(), "A test project");
        assert_eq!(map.get("author").unwrap(), "Test Author");
        assert!(map.contains_key("date"));
    }

    #[test]
    fn test_readme_parser_detect_language() {
        let content = "# My Project\n\nA Rust project using Cargo.toml";
        assert_eq!(ReadmeParser::detect_language(content), "rust");

        let content2 = "# Python App\n\nInstall with setup.py";
        assert_eq!(ReadmeParser::detect_language(content2), "python");

        let content3 = "# Unknown Project";
        assert_eq!(ReadmeParser::detect_language(content3), "unknown");
    }

    #[test]
    fn test_readme_parser_infer_project_type() {
        let content = "# CLI Tool\n\nA command-line interface";
        assert_eq!(ReadmeParser::infer_project_type(content), "cli");

        let content2 = "# Web API\n\nA REST API server";
        assert_eq!(ReadmeParser::infer_project_type(content2), "web-service");

        let content3 = "# Library\n\nA reusable crate";
        assert_eq!(ReadmeParser::infer_project_type(content3), "library");
    }

    #[test]
    fn test_readme_parser_extract_first_heading() {
        let content = "# My Project\n\nDescription here";
        assert_eq!(
            ReadmeParser::extract_first_heading(content),
            Some("My Project".to_string())
        );

        let content2 = "Some text\n## Not First\n# First Heading";
        assert_eq!(
            ReadmeParser::extract_first_heading(content2),
            Some("First Heading".to_string())
        );
    }

    #[test]
    fn test_file_creator_get_backup_path() {
        let path = Path::new("AGENTS.md");
        let backup = FileCreator::get_backup_path(path);

        let backup_str = backup.to_string_lossy();
        assert!(backup_str.contains("AGENTS.md.backup"));
    }

    #[test]
    fn test_component_composer_compose() {
        let component1 = Component {
            name: "test1".to_string(),
            component_type: ComponentType::Core,
            content: "Hello {{name}}".to_string(),
            metadata: Default::default(),
        };

        let component2 = Component {
            name: "test2".to_string(),
            component_type: ComponentType::Core,
            content: "Version {{version}}".to_string(),
            metadata: Default::default(),
        };

        let mut placeholders = HashMap::new();
        placeholders.insert("name".to_string(), "World".to_string());
        placeholders.insert("version".to_string(), "1.0.0".to_string());

        let result = ComponentComposer::compose(&[component1, component2], &placeholders);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("Hello World"));
        assert!(content.contains("Version 1.0.0"));
        assert!(content.contains("---"));
    }
}
