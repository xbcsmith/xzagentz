// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Interactive Planning Session
//!
//! This module provides an interactive CLI interface for generating
//! implementation plans from architecture documents.

use crate::application::error::{ApplicationError, Result};
use crate::application::planning_service::PlanningService;
use crate::domain::planning::{ArchitectureParser, Plan, PlanGenerator, PlanOptions, PlanWriter};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;

/// Configuration for interactive session
#[derive(Debug, Clone)]
pub struct InteractiveConfig {
    /// Enable colored output
    pub enable_colors: bool,
    /// Show progress indicators
    pub show_progress: bool,
    /// Confirm before saving
    pub confirm_before_save: bool,
    /// Default output directory
    pub default_output_dir: Option<String>,
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            enable_colors: true,
            show_progress: true,
            confirm_before_save: true,
            default_output_dir: Some("./plans".to_string()),
        }
    }
}

/// Interactive Planning Session
///
/// Provides a user-friendly interactive interface for:
/// - Selecting architecture files
/// - Choosing Ollama models
/// - Configuring generation options
/// - Reviewing and confirming plans
/// - Saving plans to disk
///
/// # Examples
///
/// ```no_run
/// use xzagentz::application::{InteractivePlanningSession, InteractiveConfig};
/// use xzagentz::application::PlanningService;
/// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaPlanGenerator, OllamaConfig};
/// use xzagentz::infrastructure::fileio::{MarkdownArchitectureParser, MarkdownPlanWriter};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = OllamaConfig::default();
/// let client = OllamaClient::new(config)?;
/// let generator = OllamaPlanGenerator::new(client);
/// let parser = MarkdownArchitectureParser::new();
/// let writer = MarkdownPlanWriter::new();
///
/// let service = PlanningService::new(generator, parser, writer);
/// let config = InteractiveConfig::default();
///
/// let session = InteractivePlanningSession::new(service, config);
/// let plan = session.run()?;
/// # Ok(())
/// # }
/// ```
pub struct InteractivePlanningSession<G, P, W>
where
    G: PlanGenerator,
    P: ArchitectureParser,
    W: PlanWriter,
{
    service: PlanningService<G, P, W>,
    config: InteractiveConfig,
}

impl<G, P, W> InteractivePlanningSession<G, P, W>
where
    G: PlanGenerator,
    P: ArchitectureParser,
    W: PlanWriter,
{
    /// Creates a new interactive planning session
    ///
    /// # Arguments
    ///
    /// * `service` - The planning service to use
    /// * `config` - Configuration for interactive behavior
    pub fn new(service: PlanningService<G, P, W>, config: InteractiveConfig) -> Self {
        Self { service, config }
    }

    /// Runs the interactive planning session
    ///
    /// This method orchestrates the complete interactive workflow:
    /// 1. Prompt for architecture file
    /// 2. Parse and display architecture summary
    /// 3. Select Ollama model
    /// 4. Configure generation options
    /// 5. Generate plan with progress indicator
    /// 6. Display plan summary
    /// 7. Prompt for output path
    /// 8. Save plan
    ///
    /// # Returns
    ///
    /// Returns the generated `Plan` on success
    ///
    /// # Errors
    ///
    /// Returns `ApplicationError` if any step fails or user cancels
    pub fn run(&self) -> Result<Plan> {
        self.print_welcome();

        // Step 1: Prompt for architecture file
        let architecture_path = self.prompt_architecture_file()?;

        // Step 2: Parse and validate architecture
        self.print_step("Parsing architecture document...");
        let architecture = self.service.parse_architecture_file(&architecture_path)?;
        self.print_architecture_summary(architecture.title(), architecture.sections().len());

        // Step 3: Select model and configure options
        let model = self.prompt_model_selection()?;
        let num_phases = self.prompt_num_phases()?;

        let mut options = PlanOptions::new().with_model(&model);
        if let Some(phases) = num_phases {
            options = options.with_num_phases(phases);
        }

        // Step 4: Generate plan
        self.print_step("Generating implementation plan...");
        let plan = self.generate_with_progress(&architecture_path, &options)?;

        // Step 5: Display plan summary
        self.print_plan_summary(&plan);

        // Step 6: Review and confirm
        if self.config.confirm_before_save && !self.confirm_generation()? {
            return Err(ApplicationError::Cancelled);
        }

        // Step 7: Prompt for output path and save
        let output_path = self.prompt_output_path(plan.title())?;
        self.print_step(&format!("Saving plan to {}...", output_path));

        let force = if Path::new(&output_path).exists() {
            self.confirm_overwrite(&output_path)?
        } else {
            false
        };

        self.service.save_plan(&plan, &output_path, force)?;
        self.print_success(&format!("Plan saved successfully to {}", output_path));

        Ok(plan)
    }

    /// Prompts user for architecture file path
    fn prompt_architecture_file(&self) -> Result<String> {
        let path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Architecture document path")
            .default("architecture.md".to_string())
            .interact_text()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))?;

        // Validate file exists
        if !Path::new(&path).exists() {
            return Err(ApplicationError::ArchitectureNotFound(path));
        }

        Ok(path)
    }

    /// Prompts user to select an Ollama model
    fn prompt_model_selection(&self) -> Result<String> {
        let models = vec![
            "llama3",
            "llama3.2:3b",
            "mistral",
            "mixtral",
            "codellama",
            "phi",
            "gemma",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select Ollama model")
            .default(0)
            .items(&models)
            .interact()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))?;

        Ok(models[selection].to_string())
    }

    /// Prompts user for number of phases
    fn prompt_num_phases(&self) -> Result<Option<usize>> {
        let use_default = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Use automatic phase detection?")
            .default(true)
            .interact()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))?;

        if use_default {
            return Ok(None);
        }

        let num: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Number of phases")
            .default("5".to_string())
            .validate_with(|input: &String| -> std::result::Result<(), &str> {
                input
                    .parse::<usize>()
                    .map(|_| ())
                    .map_err(|_| "Please enter a valid number")
            })
            .interact_text()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))?;

        Ok(Some(num.parse().unwrap()))
    }

    /// Prompts user for output path
    fn prompt_output_path(&self, plan_title: &str) -> Result<String> {
        let default_name = plan_title
            .to_lowercase()
            .replace(' ', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();

        let default_path = if let Some(dir) = &self.config.default_output_dir {
            format!("{}/{}.md", dir, default_name)
        } else {
            format!("{}.md", default_name)
        };

        let path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Output file path")
            .default(default_path)
            .interact_text()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))?;

        Ok(path)
    }

    /// Confirms plan generation
    fn confirm_generation(&self) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Save this plan?")
            .default(true)
            .interact()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))
    }

    /// Confirms overwriting existing file
    fn confirm_overwrite(&self, path: &str) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("File '{}' exists. Overwrite?", path))
            .default(false)
            .interact()
            .map_err(|e| ApplicationError::InteractiveError(e.to_string()))
    }

    /// Generates plan with progress indicator
    fn generate_with_progress(
        &self,
        architecture_path: &str,
        options: &PlanOptions,
    ) -> Result<Plan> {
        if self.config.show_progress {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner:.green} {msg}")
                    .expect("Invalid progress template"),
            );
            pb.set_message("Calling Ollama API...");
            pb.enable_steady_tick(Duration::from_millis(100));

            let result = self
                .service
                .generate_implementation_plan(architecture_path, options);

            pb.finish_and_clear();
            result
        } else {
            self.service
                .generate_implementation_plan(architecture_path, options)
        }
    }

    /// Prints welcome message
    fn print_welcome(&self) {
        if self.config.enable_colors {
            println!("\n{}", style("Implementation Plan Generator").bold().cyan());
            println!("{}\n", style("Interactive Mode").dim());
        } else {
            println!("\nImplementation Plan Generator");
            println!("Interactive Mode\n");
        }
    }

    /// Prints a step message
    fn print_step(&self, message: &str) {
        if self.config.enable_colors {
            println!("{} {}", style("→").cyan().bold(), message);
        } else {
            println!("→ {}", message);
        }
    }

    /// Prints success message
    fn print_success(&self, message: &str) {
        if self.config.enable_colors {
            println!("{} {}\n", style("✓").green().bold(), message);
        } else {
            println!("✓ {}\n", message);
        }
    }

    /// Prints architecture summary
    fn print_architecture_summary(&self, title: &str, num_sections: usize) {
        if self.config.enable_colors {
            println!("  {} {}", style("Title:").dim(), style(title).bold());
            println!(
                "  {} {}\n",
                style("Sections:").dim(),
                style(num_sections).bold()
            );
        } else {
            println!("  Title: {}", title);
            println!("  Sections: {}\n", num_sections);
        }
    }

    /// Prints plan summary
    fn print_plan_summary(&self, plan: &Plan) {
        let total_tasks: usize = plan.phases().iter().map(|p| p.tasks().len()).sum();

        if self.config.enable_colors {
            println!("\n{}", style("Generated Plan Summary").bold().green());
            println!("  {} {}", style("Title:").dim(), style(plan.title()).bold());
            println!(
                "  {} {}",
                style("Phases:").dim(),
                style(plan.phases().len()).bold()
            );
            println!(
                "  {} {}",
                style("Total Tasks:").dim(),
                style(total_tasks).bold()
            );
            println!(
                "  {} {}\n",
                style("Model:").dim(),
                style(plan.metadata().model_used()).bold()
            );
        } else {
            println!("\nGenerated Plan Summary");
            println!("  Title: {}", plan.title());
            println!("  Phases: {}", plan.phases().len());
            println!("  Total Tasks: {}", total_tasks);
            println!("  Model: {}\n", plan.metadata().model_used());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::planning::{ArchitectureDocument, Phase, PlanMetadata, Section, Task};
    use chrono::Utc;
    use std::path::PathBuf;

    // Mock implementations for testing
    struct MockParser;

    impl ArchitectureParser for MockParser {
        type Error = std::io::Error;

        fn parse(&self, _content: &str) -> std::result::Result<ArchitectureDocument, Self::Error> {
            let mut doc = ArchitectureDocument::new("Test Architecture");
            doc.add_section(Section::new("Section 1", 2).with_content("Content"));
            doc.add_section(Section::new("Section 2", 2).with_content("Content"));
            Ok(doc)
        }
    }

    struct MockGenerator;

    impl PlanGenerator for MockGenerator {
        type Error = std::io::Error;

        fn generate_plan(
            &self,
            _architecture: &ArchitectureDocument,
            options: &PlanOptions,
        ) -> std::result::Result<Plan, Self::Error> {
            let task1 = Task::new("task-1", "Description");
            let task2 = Task::new("task-2", "Description");

            let phase =
                Phase::new("phase-1", "Test Phase", "Description").with_tasks(vec![task1, task2]);

            let model = options.model().unwrap_or("test-model");
            let metadata = PlanMetadata::new(Utc::now(), model, PathBuf::from("test.md"));

            Ok(Plan::new(
                "Test Implementation Plan",
                "Generated plan",
                vec![phase],
                metadata,
            ))
        }
    }

    struct MockWriter;

    impl PlanWriter for MockWriter {
        type Error = std::io::Error;

        fn write_plan(&self, _plan: &Plan, _path: &Path) -> std::result::Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn test_new_interactive_session() {
        let generator = MockGenerator;
        let parser = MockParser;
        let writer = MockWriter;

        let service = PlanningService::new(generator, parser, writer);
        let config = InteractiveConfig::default();

        let _session = InteractivePlanningSession::new(service, config);
    }

    #[test]
    fn test_interactive_config_default() {
        let config = InteractiveConfig::default();
        assert!(config.enable_colors);
        assert!(config.show_progress);
        assert!(config.confirm_before_save);
        assert_eq!(config.default_output_dir, Some("./plans".to_string()));
    }

    #[test]
    fn test_print_methods_with_colors() {
        let generator = MockGenerator;
        let parser = MockParser;
        let writer = MockWriter;

        let service = PlanningService::new(generator, parser, writer);
        let config = InteractiveConfig {
            enable_colors: true,
            show_progress: false,
            confirm_before_save: false,
            default_output_dir: None,
        };

        let session = InteractivePlanningSession::new(service, config);

        session.print_welcome();
        session.print_step("Test step");
        session.print_success("Test success");
        session.print_architecture_summary("Test Title", 5);
    }

    #[test]
    fn test_print_methods_without_colors() {
        let generator = MockGenerator;
        let parser = MockParser;
        let writer = MockWriter;

        let service = PlanningService::new(generator, parser, writer);
        let config = InteractiveConfig {
            enable_colors: false,
            show_progress: false,
            confirm_before_save: false,
            default_output_dir: None,
        };

        let session = InteractivePlanningSession::new(service, config);

        session.print_welcome();
        session.print_step("Test step");
        session.print_success("Test success");
        session.print_architecture_summary("Test Title", 5);
    }

    #[test]
    fn test_print_plan_summary() {
        let generator = MockGenerator;
        let parser = MockParser;
        let writer = MockWriter;

        let service = PlanningService::new(generator, parser, writer);
        let config = InteractiveConfig::default();

        let session = InteractivePlanningSession::new(service, config);

        let task1 = Task::new("task-1", "Description");
        let task2 = Task::new("task-2", "Description");

        let phase = Phase::new("phase-1", "Phase 1", "Description").with_tasks(vec![task1, task2]);

        let metadata = PlanMetadata::new(Utc::now(), "llama3", PathBuf::from("test.md"));
        let plan = Plan::new("Test Plan", "Description", vec![phase], metadata);

        session.print_plan_summary(&plan);
    }
}
