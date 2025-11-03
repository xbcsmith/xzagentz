//! Interactive Architecture Session
//!
//! This module provides an interactive CLI experience for architecture generation,
//! guiding users through template selection, requirements gathering, and refinement.
//!
//! # Examples
//!
//! ```no_run
//! use xzagentz::application::{ArchitectureService, InteractiveArchitectureSession};
//! use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
//! use xzagentz::infrastructure::templates::FileTemplateRepository;
//! use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
//! use xzagentz::infrastructure::OllamaConfig;
//! use dialoguer::theme::ColorfulTheme;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = OllamaConfig::default();
//! let generator = OllamaArchitectureGenerator::new(config)?;
//! let writer = MarkdownArchitectureWriter::new();
//! let template_repo = FileTemplateRepository::new("./templates".into());
//! template_repo.initialize().await?;
//! let service = ArchitectureService::new(generator, writer, template_repo);
//!
//! let session = InteractiveArchitectureSession::new(service, ColorfulTheme::default());
//! session.run().await?;
//! # Ok(())
//! # }
//! ```

use crate::application::architecture_service::{ArchitectureService, ServiceError};
use crate::domain::architecture::{
    ArchitectureDocument, ArchitectureGenerator, ArchitecturePattern, ArchitectureWriter,
    ComplexityLevel, GenerationOptions, TemplateRepository,
};
use dialoguer::{theme::Theme, Confirm, Input, Select};
use thiserror::Error;

/// Interactive session for architecture generation
///
/// Provides a guided CLI experience with prompts for:
/// - Generation mode selection (requirements vs template)
/// - Architecture pattern and complexity selection
/// - Requirements gathering
/// - Review and refinement loop
/// - Document saving
pub struct InteractiveArchitectureSession<G, W, T, Th>
where
    G: ArchitectureGenerator,
    W: ArchitectureWriter,
    T: TemplateRepository,
    Th: Theme,
{
    service: ArchitectureService<G, W, T>,
    theme: Th,
}

impl<G, W, T, Th> InteractiveArchitectureSession<G, W, T, Th>
where
    G: ArchitectureGenerator,
    W: ArchitectureWriter,
    T: TemplateRepository,
    Th: Theme,
{
    /// Creates a new interactive architecture session
    ///
    /// # Arguments
    ///
    /// * `service` - Architecture service for generation and persistence
    /// * `theme` - Dialoguer theme for styling prompts
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use xzagentz::application::{ArchitectureService, InteractiveArchitectureSession};
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// # use dialoguer::theme::ColorfulTheme;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// let session = InteractiveArchitectureSession::new(service, ColorfulTheme::default());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(service: ArchitectureService<G, W, T>, theme: Th) -> Self {
        Self { service, theme }
    }

    /// Runs the interactive session
    ///
    /// This is the main entry point that guides the user through:
    /// 1. Selecting generation mode
    /// 2. Gathering requirements or selecting template
    /// 3. Generating architecture
    /// 4. Review and refinement loop
    /// 5. Saving document
    ///
    /// # Returns
    ///
    /// Returns path to saved document on success, or `None` if cancelled
    ///
    /// # Errors
    ///
    /// Returns `SessionError` if any step fails or user cancels
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use xzagentz::application::{ArchitectureService, InteractiveArchitectureSession};
    /// # use xzagentz::infrastructure::ollama::OllamaArchitectureGenerator;
    /// # use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// # use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    /// # use xzagentz::infrastructure::OllamaConfig;
    /// # use dialoguer::theme::ColorfulTheme;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = OllamaConfig::default();
    /// # let generator = OllamaArchitectureGenerator::new(config)?;
    /// # let writer = MarkdownArchitectureWriter::new();
    /// # let template_repo = FileTemplateRepository::new("./templates".into());
    /// # template_repo.initialize().await?;
    /// # let service = ArchitectureService::new(generator, writer, template_repo);
    /// # let session = InteractiveArchitectureSession::new(service, ColorfulTheme::default());
    /// if let Some(output_path) = session.run().await? {
    ///     println!("Architecture saved to: {}", output_path);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run(&self) -> Result<Option<String>, SessionError> {
        println!("\n=== Architecture Generation Assistant ===\n");

        let mode = self.select_generation_mode()?;

        let mut document = match mode {
            GenerationMode::FromRequirements => {
                self.generate_from_requirements_interactive().await?
            }
            GenerationMode::FromTemplate => self.generate_from_template_interactive().await?,
        };

        document = self.review_and_refine_loop(document).await?;

        let output_path = self.save_document_interactive(&document).await?;

        Ok(Some(output_path))
    }

    fn select_generation_mode(&self) -> Result<GenerationMode, SessionError> {
        let options = vec!["Generate from requirements", "Generate from template"];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("How would you like to generate your architecture?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        match selection {
            0 => Ok(GenerationMode::FromRequirements),
            1 => Ok(GenerationMode::FromTemplate),
            _ => Err(SessionError::InteractionError(
                "Invalid selection".to_string(),
            )),
        }
    }

    async fn generate_from_requirements_interactive(
        &self,
    ) -> Result<ArchitectureDocument, SessionError> {
        println!("\n--- Generate from Requirements ---\n");

        let requirements: String = Input::with_theme(&self.theme)
            .with_prompt("Describe your system requirements")
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        let pattern = self.select_architecture_pattern()?;
        let complexity = self.select_complexity_level()?;

        let include_deployment = self.prompt_yes_no("Include deployment architecture?")?;
        let include_quality = self.prompt_yes_no("Include quality attributes?")?;

        let tech_prefs = self.prompt_technology_preferences()?;

        let options = GenerationOptions {
            pattern: Some(pattern),
            complexity,
            include_deployment,
            include_quality_attributes: include_quality,
            max_components: Some(20),
            technology_preferences: tech_prefs,
        };

        println!("\nGenerating architecture...");
        self.service
            .generate_from_requirements(&requirements, options)
            .await
            .map_err(SessionError::ServiceError)
    }

    async fn generate_from_template_interactive(
        &self,
    ) -> Result<ArchitectureDocument, SessionError> {
        println!("\n--- Generate from Template ---\n");

        let templates = self
            .service
            .list_templates()
            .await
            .map_err(SessionError::ServiceError)?;

        if templates.is_empty() {
            return Err(SessionError::NoTemplatesAvailable);
        }

        let template_names: Vec<String> = templates
            .iter()
            .map(|t| format!("{} - {}", t.name, t.description))
            .collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select a template")
            .items(&template_names)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        let template_name = &templates[selection].name;

        let customization: String = Input::with_theme(&self.theme)
            .with_prompt("Customization instructions (optional)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        println!("\nGenerating architecture from template...");
        self.service
            .generate_from_template(template_name, &customization)
            .await
            .map_err(SessionError::ServiceError)
    }

    async fn review_and_refine_loop(
        &self,
        mut document: ArchitectureDocument,
    ) -> Result<ArchitectureDocument, SessionError> {
        loop {
            println!("\n--- Architecture Preview ---\n");
            self.display_document_summary(&document);

            let action = self.select_review_action()?;

            match action {
                ReviewAction::Accept => {
                    println!("\nArchitecture accepted!");
                    return Ok(document);
                }
                ReviewAction::Refine => {
                    let feedback: String = Input::with_theme(&self.theme)
                        .with_prompt("What would you like to refine?")
                        .interact_text()
                        .map_err(|e| SessionError::InteractionError(e.to_string()))?;

                    println!("\nRefining architecture...");
                    document = self
                        .service
                        .refine_architecture(&document, &feedback)
                        .await
                        .map_err(SessionError::ServiceError)?;
                }
                ReviewAction::Cancel => {
                    println!("\nCancelled.");
                    return Err(SessionError::Cancelled);
                }
            }
        }
    }

    fn display_document_summary(&self, document: &ArchitectureDocument) {
        println!("Title: {}", document.metadata.title);
        println!("Pattern: {}", document.metadata.pattern);
        println!("Description: {}", document.overview.description);
        println!("\nLayers: {}", document.layers.len());
        println!("Components: {}", document.components.len());
        println!("Integrations: {}", document.integrations.len());
    }

    fn select_review_action(&self) -> Result<ReviewAction, SessionError> {
        let options = vec!["Accept and save", "Refine further", "Cancel"];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        match selection {
            0 => Ok(ReviewAction::Accept),
            1 => Ok(ReviewAction::Refine),
            2 => Ok(ReviewAction::Cancel),
            _ => Err(SessionError::InteractionError(
                "Invalid selection".to_string(),
            )),
        }
    }

    async fn save_document_interactive(
        &self,
        document: &ArchitectureDocument,
    ) -> Result<String, SessionError> {
        let default_name = format!(
            "{}_architecture.md",
            document.metadata.title.to_lowercase().replace(' ', "_")
        );

        let output_path: String = Input::with_theme(&self.theme)
            .with_prompt("Output file path")
            .default(default_name)
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        if std::path::Path::new(&output_path).exists() {
            let overwrite = self.prompt_yes_no(&format!(
                "File '{}' already exists. Overwrite?",
                output_path
            ))?;

            if !overwrite {
                return Err(SessionError::Cancelled);
            }
        }

        self.service
            .save_architecture(document, &output_path)
            .await
            .map_err(SessionError::ServiceError)?;

        println!("\nArchitecture saved to: {}", output_path);
        Ok(output_path)
    }

    fn select_architecture_pattern(&self) -> Result<ArchitecturePattern, SessionError> {
        let patterns = [
            ("Microservices", ArchitecturePattern::Microservices),
            ("Monolithic", ArchitecturePattern::Monolithic),
            ("Event-Driven", ArchitecturePattern::EventDriven),
            ("Layered", ArchitecturePattern::Layered),
            ("Hexagonal", ArchitecturePattern::Hexagonal),
            ("CQRS", ArchitecturePattern::CQRS),
            ("Serverless", ArchitecturePattern::Serverless),
        ];

        let pattern_names: Vec<&str> = patterns.iter().map(|(name, _)| *name).collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select architecture pattern")
            .items(&pattern_names)
            .default(0)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(patterns[selection].1.clone())
    }

    fn select_complexity_level(&self) -> Result<ComplexityLevel, SessionError> {
        let levels = [
            ("Simple", ComplexityLevel::Simple),
            ("Moderate", ComplexityLevel::Moderate),
            ("Complex", ComplexityLevel::Complex),
            ("Enterprise", ComplexityLevel::Enterprise),
        ];

        let level_names: Vec<&str> = levels.iter().map(|(name, _)| *name).collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select complexity level")
            .items(&level_names)
            .default(1)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(levels[selection].1)
    }

    fn prompt_yes_no(&self, prompt: &str) -> Result<bool, SessionError> {
        Confirm::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(true)
            .interact()
            .map_err(|e| SessionError::InteractionError(e.to_string()))
    }

    fn prompt_technology_preferences(&self) -> Result<Vec<String>, SessionError> {
        let tech_input: String = Input::with_theme(&self.theme)
            .with_prompt("Technology preferences (comma-separated, optional)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| SessionError::InteractionError(e.to_string()))?;

        Ok(tech_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }
}

/// Generation mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenerationMode {
    /// Generate from natural language requirements
    FromRequirements,
    /// Generate from existing template
    FromTemplate,
}

/// Review action selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReviewAction {
    /// Accept current architecture
    Accept,
    /// Request refinement
    Refine,
    /// Cancel session
    Cancel,
}

/// Errors that can occur during interactive session
#[derive(Error, Debug)]
pub enum SessionError {
    /// Error during user interaction
    #[error("Interaction error: {0}")]
    InteractionError(String),

    /// Error from architecture service
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// No templates available
    #[error("No templates available")]
    NoTemplatesAvailable,

    /// User cancelled the session
    #[error("Session cancelled by user")]
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_mode_variants() {
        let mode1 = GenerationMode::FromRequirements;
        let mode2 = GenerationMode::FromTemplate;
        assert_ne!(mode1, mode2);
    }

    #[test]
    fn test_review_action_variants() {
        let action1 = ReviewAction::Accept;
        let action2 = ReviewAction::Refine;
        let action3 = ReviewAction::Cancel;
        assert_ne!(action1, action2);
        assert_ne!(action2, action3);
    }

    #[test]
    fn test_session_error_display() {
        let error = SessionError::NoTemplatesAvailable;
        assert_eq!(error.to_string(), "No templates available");

        let error = SessionError::Cancelled;
        assert_eq!(error.to_string(), "Session cancelled by user");

        let error = SessionError::InteractionError("test".to_string());
        assert_eq!(error.to_string(), "Interaction error: test");
    }
}
