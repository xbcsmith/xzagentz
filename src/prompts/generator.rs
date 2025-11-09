//! Prompt generation orchestrator
//!
//! This module provides the main prompt generation functionality, orchestrating
//! template rendering, file writing, and progress tracking.

use crate::error::{Error, Result};
use crate::plans::structures::{ImplementationPlan, Phase, PlanSection};
use crate::prompts::progress::ProgressTracker;
use crate::prompts::template::{
    DeliverableFile, PromptContext, PromptTemplate, TestExample, TestFile,
};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Prompt generator configuration
#[derive(Debug, Clone)]
pub struct PromptGeneratorConfig {
    /// Output directory for generated prompts
    pub output_dir: PathBuf,
    /// Overwrite existing prompts without asking
    pub force_overwrite: bool,
    /// Create backup before overwriting
    pub create_backup: bool,
    /// Include full architecture in context
    pub include_full_architecture: bool,
    /// JIRA issue prefix for commit messages
    pub jira_issue: String,
    /// Project name
    pub project_name: String,
    /// Project description
    pub project_description: String,
}

impl Default for PromptGeneratorConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("prompts"),
            force_overwrite: false,
            create_backup: true,
            include_full_architecture: false,
            jira_issue: String::new(),
            project_name: String::new(),
            project_description: String::new(),
        }
    }
}

/// Main prompt generator
pub struct PromptGenerator {
    config: PromptGeneratorConfig,
    template: PromptTemplate,
}

impl PromptGenerator {
    /// Create new prompt generator
    ///
    /// # Arguments
    ///
    /// * `output_dir` - Directory to write prompts to
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::PromptGenerator;
    /// use std::path::Path;
    ///
    /// let generator = PromptGenerator::new(Path::new("prompts")).unwrap();
    /// ```
    pub fn new(output_dir: &Path) -> Result<Self> {
        let config = PromptGeneratorConfig {
            output_dir: output_dir.to_path_buf(),
            ..Default::default()
        };

        Self::with_config(config)
    }

    /// Create prompt generator with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Generator configuration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::{PromptGenerator, PromptGeneratorConfig};
    /// use std::path::PathBuf;
    ///
    /// let config = PromptGeneratorConfig {
    ///     output_dir: PathBuf::from("prompts"),
    ///     force_overwrite: true,
    ///     ..Default::default()
    /// };
    /// let generator = PromptGenerator::with_config(config).unwrap();
    /// ```
    pub fn with_config(config: PromptGeneratorConfig) -> Result<Self> {
        // Create output directory if it doesn't exist
        if !config.output_dir.exists() {
            fs::create_dir_all(&config.output_dir).map_err(|e| Error::FileIo {
                path: config.output_dir.clone(),
                source: e,
            })?;
        }

        // Load embedded template
        let template = PromptTemplate::load_embedded("section_prompt")?;

        Ok(Self { config, template })
    }

    /// Generate all prompts from implementation plan
    ///
    /// # Arguments
    ///
    /// * `plan` - Implementation plan to generate prompts from
    /// * `tracker` - Progress tracker (optional, for marking generated sections)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::PromptGenerator;
    /// use xzagentz::plans::parser::PlanParser;
    /// use std::path::Path;
    ///
    /// let generator = PromptGenerator::new(Path::new("prompts")).unwrap();
    /// let parser = PlanParser::new();
    /// let plan_content = std::fs::read_to_string("docs/explanation/implementation_plan.md").unwrap();
    /// let plan = parser.parse_implementation_plan(&plan_content).unwrap();
    ///
    /// let generated = generator.generate_all(&plan, None).unwrap();
    /// println!("Generated {} prompts", generated.len());
    /// ```
    pub fn generate_all(
        &self,
        plan: &ImplementationPlan,
        tracker: Option<&ProgressTracker>,
    ) -> Result<Vec<PathBuf>> {
        let mut generated = Vec::new();

        for phase in &plan.phases {
            for section in &phase.sections {
                let context = self.build_context(plan, phase, section)?;
                let path = self.generate_section_prompt(&context)?;
                generated.push(path);
            }
        }

        if let Some(tracker) = tracker {
            println!(
                "\nGenerated {} prompts. Current progress: {:.1}%",
                generated.len(),
                tracker.completion_percentage()
            );
        }

        Ok(generated)
    }

    /// Generate prompt for a specific section
    ///
    /// # Arguments
    ///
    /// * `context` - Prompt context with section data
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::{PromptGenerator, PromptContext};
    /// use std::path::Path;
    ///
    /// let generator = PromptGenerator::new(Path::new("prompts")).unwrap();
    /// let mut context = PromptContext::default();
    /// context.section_number = "1.1".to_string();
    /// context.section_name = "Foundation".to_string();
    ///
    /// let path = generator.generate_section_prompt(&context).unwrap();
    /// println!("Generated: {}", path.display());
    /// ```
    pub fn generate_section_prompt(&self, context: &PromptContext) -> Result<PathBuf> {
        let filename = self.format_filename(context);
        let output_path = self.config.output_dir.join(&filename);

        // Check if file exists and handle overwrite
        if output_path.exists() && !self.config.force_overwrite {
            if !self.confirm_overwrite(&output_path)? {
                return Ok(output_path);
            }

            if self.config.create_backup {
                self.create_backup(&output_path)?;
            }
        }

        // Render template
        let content = self.template.render(context)?;

        // Write to file
        fs::write(&output_path, content).map_err(|e| Error::FileIo {
            path: output_path.clone(),
            source: e,
        })?;

        Ok(output_path)
    }

    /// Generate prompts interactively with preview
    ///
    /// # Arguments
    ///
    /// * `plan` - Implementation plan
    /// * `tracker` - Progress tracker (optional)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::PromptGenerator;
    /// use xzagentz::plans::parser::PlanParser;
    /// use std::path::Path;
    ///
    /// let generator = PromptGenerator::new(Path::new("prompts")).unwrap();
    /// let parser = PlanParser::new();
    /// let plan_content = std::fs::read_to_string("docs/explanation/implementation_plan.md").unwrap();
    /// let plan = parser.parse_implementation_plan(&plan_content).unwrap();
    ///
    /// let generated = generator.generate_interactive(&plan, None).unwrap();
    /// ```
    pub fn generate_interactive(
        &self,
        plan: &ImplementationPlan,
        tracker: Option<&ProgressTracker>,
    ) -> Result<Vec<PathBuf>> {
        let mut generated = Vec::new();

        println!("Interactive Prompt Generation");
        println!("============================\n");

        for phase in &plan.phases {
            println!("Phase {}: {}", phase.number, phase.title);
            println!();

            for section in &phase.sections {
                let context = self.build_context(plan, phase, section)?;

                println!(
                    "Section {}: {}",
                    context.section_number, context.section_name
                );
                println!("Tasks: {}", context.tasks.len());
                println!("Deliverables: {}", context.deliverable_files.len());
                println!();

                print!("Generate this prompt? [y/n/p(review)/q(quit)]: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => {
                        let path = self.generate_section_prompt(&context)?;
                        println!("Generated: {}\n", path.display());
                        generated.push(path);
                    }
                    "p" | "preview" => {
                        let content = self.template.render(&context)?;
                        println!("\n--- Preview ---");
                        println!("{}", &content[..content.len().min(500)]);
                        println!("... (truncated)\n");

                        print!("Save this prompt? [y/n]: ");
                        io::stdout().flush().unwrap();

                        let mut save_input = String::new();
                        io::stdin().read_line(&mut save_input).unwrap();

                        if save_input.trim().to_lowercase().starts_with('y') {
                            let path = self.generate_section_prompt(&context)?;
                            println!("Generated: {}\n", path.display());
                            generated.push(path);
                        }
                    }
                    "q" | "quit" => {
                        println!("Exiting interactive generation.");
                        break;
                    }
                    _ => {
                        println!("Skipped.\n");
                    }
                }
            }
        }

        if let Some(tracker) = tracker {
            println!(
                "\nGenerated {} prompts. Current progress: {:.1}%",
                generated.len(),
                tracker.completion_percentage()
            );
        }

        Ok(generated)
    }

    /// Build prompt context from plan, phase, and section
    fn build_context(
        &self,
        _plan: &ImplementationPlan,
        phase: &Phase,
        section: &PlanSection,
    ) -> Result<PromptContext> {
        let context = PromptContext {
            project_name: self.config.project_name.clone(),
            project_description: self.config.project_description.clone(),
            phase_number: phase.number,
            phase_duration: phase.duration.clone().unwrap_or_default(),
            phase_goal: phase.goal.clone(),
            section_number: section.number.clone(),
            section_name: section.title.clone(),
            section_purpose: section
                .tasks
                .first()
                .cloned()
                .unwrap_or_else(|| "Implement this section".to_string()),
            tasks: section.tasks.clone(),
            deliverable_files: section
                .deliverables
                .iter()
                .map(|d| DeliverableFile {
                    path: d.clone(),
                    description: "Implementation file".to_string(),
                })
                .collect(),
            test_files: section
                .tests
                .iter()
                .map(|t| TestFile {
                    path: format!("tests/{}", t),
                    description: "Test function".to_string(),
                })
                .collect(),
            documentation_file: format!(
                "phase{}_{}_implementation.md",
                phase.number,
                section.number.replace('.', "_")
            ),
            acceptance_criteria: section.acceptance_criteria.clone(),
            test_examples: section
                .tests
                .iter()
                .map(|t| TestExample {
                    name: t.clone(),
                    description: format!("Test for {}", t),
                })
                .collect(),
            jira_issue: self.config.jira_issue.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        };

        Ok(context)
    }

    /// Format filename from context
    fn format_filename(&self, context: &PromptContext) -> String {
        format!(
            "prompt_{}_{}.md",
            context.phase_number,
            context.section_number.replace('.', "_")
        )
    }

    /// Confirm overwrite with user
    fn confirm_overwrite(&self, path: &Path) -> Result<bool> {
        print!("File {} already exists. Overwrite? [y/n]: ", path.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        Ok(input.trim().to_lowercase().starts_with('y'))
    }

    /// Create backup of existing file
    fn create_backup(&self, path: &Path) -> Result<PathBuf> {
        let backup_path = path.with_extension("md.bak");
        fs::copy(path, &backup_path).map_err(|e| Error::FileIo {
            path: backup_path.clone(),
            source: e,
        })?;
        Ok(backup_path)
    }

    /// Get configuration reference
    pub fn config(&self) -> &PromptGeneratorConfig {
        &self.config
    }

    /// Set JIRA issue
    pub fn set_jira_issue(&mut self, jira_issue: String) {
        self.config.jira_issue = jira_issue;
    }

    /// Set project metadata
    pub fn set_project_metadata(&mut self, name: String, description: String) {
        self.config.project_name = name;
        self.config.project_description = description;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generator_config_default() {
        let config = PromptGeneratorConfig::default();
        assert_eq!(config.output_dir, PathBuf::from("prompts"));
        assert!(!config.force_overwrite);
        assert!(config.create_backup);
    }

    #[test]
    fn test_generator_new_creates_output_dir() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("prompts");

        let generator = PromptGenerator::new(&output_dir).unwrap();
        assert!(output_dir.exists());
        assert_eq!(generator.config.output_dir, output_dir);
    }

    #[test]
    fn test_format_filename() {
        let temp_dir = tempdir().unwrap();
        let generator = PromptGenerator::new(temp_dir.path()).unwrap();

        let context = PromptContext {
            phase_number: 2,
            section_number: "2.3".to_string(),
            ..Default::default()
        };

        let filename = generator.format_filename(&context);
        assert_eq!(filename, "prompt_2_2_3.md");
    }

    #[test]
    fn test_set_jira_issue() {
        let temp_dir = tempdir().unwrap();
        let mut generator = PromptGenerator::new(temp_dir.path()).unwrap();

        generator.set_jira_issue("PROJ-123".to_string());
        assert_eq!(generator.config.jira_issue, "PROJ-123");
    }

    #[test]
    fn test_set_project_metadata() {
        let temp_dir = tempdir().unwrap();
        let mut generator = PromptGenerator::new(temp_dir.path()).unwrap();

        generator.set_project_metadata("test_project".to_string(), "A test project".to_string());
        assert_eq!(generator.config.project_name, "test_project");
        assert_eq!(generator.config.project_description, "A test project");
    }

    #[test]
    fn test_build_context_from_section() {
        let temp_dir = tempdir().unwrap();
        let mut generator = PromptGenerator::new(temp_dir.path()).unwrap();
        generator.set_project_metadata("test".to_string(), "Test project".to_string());

        let phase = Phase {
            number: 1,
            title: "Foundation".to_string(),
            duration: Some("Week 1-2".to_string()),
            goal: "Setup project".to_string(),
            sections: vec![],
            dependencies: vec![],
        };

        let section = PlanSection {
            number: "1.1".to_string(),
            title: "Init".to_string(),
            content: "Content".to_string(),
            tasks: vec!["Task 1".to_string()],
            deliverables: vec!["file.rs".to_string()],
            acceptance_criteria: vec!["Works".to_string()],
            tests: vec!["test_fn".to_string()],
        };

        let metadata = crate::plans::structures::PlanMetadata::new("test", "1.0.0");
        let plan = ImplementationPlan {
            metadata,
            overview: "Overview".to_string(),
            architecture_summary: None,
            phases: vec![],
            success_metrics: vec![],
            risks: vec![],
        };

        let context = generator.build_context(&plan, &phase, &section).unwrap();
        assert_eq!(context.phase_number, 1);
        assert_eq!(context.section_number, "1.1");
        assert_eq!(context.tasks.len(), 1);
    }
}
