// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Architecture command for LLM-based architecture generation
//!
//! This module provides CLI commands for generating, refining, and validating
//! software architecture documents using Large Language Models via Ollama.
//!
//! # Commands
//!
//! - `generate` - Generate architecture from requirements or templates
//! - `list-templates` - List available architecture templates
//! - `refine` - Refine an existing architecture document
//! - `validate` - Validate an architecture document
//!
//! # Examples
//!
//! Generate from requirements:
//! ```bash
//! xzagentz architecture generate --requirements "E-commerce API" --output arch.md
//! ```
//!
//! Generate from template:
//! ```bash
//! xzagentz architecture generate --template microservices --output arch.md
//! ```
//!
//! Interactive mode:
//! ```bash
//! xzagentz architecture generate --interactive
//! ```

use crate::application::architecture_service::{ArchitectureService, ServiceError};
use crate::application::interactive_architecture_session::{
    InteractiveArchitectureSession, SessionError,
};
use crate::config::AppConfig;
use crate::domain::architecture::{
    ArchitecturePattern, ArchitectureValidator, ComplexityLevel, GenerationOptions,
    TemplateRepository, ValidationError,
};
use crate::infrastructure::{
    FileTemplateRepository, MarkdownArchitectureWriter, OllamaArchitectureGenerator, OllamaConfig,
};
use clap::{Args, Subcommand, ValueEnum};
use dialoguer::theme::ColorfulTheme;
use std::path::PathBuf;
use thiserror::Error;

/// Architecture generation and management commands
#[derive(Debug, Args)]
pub struct ArchitectureArgs {
    /// Architecture subcommand
    #[command(subcommand)]
    pub action: ArchitectureAction,
}

/// Architecture actions
#[derive(Debug, Subcommand)]
pub enum ArchitectureAction {
    /// Generate architecture document from requirements or template
    Generate(GenerateArgs),

    /// List available architecture templates
    ListTemplates(ListTemplatesArgs),

    /// Refine an existing architecture document
    Refine(RefineArgs),

    /// Validate an architecture document
    Validate(ValidateArgs),
}

/// Arguments for architecture generation
#[derive(Debug, Args)]
pub struct GenerateArgs {
    /// Architecture requirements description (e.g., "E-commerce API with payments")
    #[arg(
        short = 'r',
        long,
        conflicts_with = "template",
        required_unless_present = "template"
    )]
    pub requirements: Option<String>,

    /// Template name to use for generation
    #[arg(short = 't', long, conflicts_with = "requirements")]
    pub template: Option<String>,

    /// Template customization prompt (used with --template)
    #[arg(short = 'c', long, requires = "template")]
    pub customization: Option<String>,

    /// Architecture pattern to use
    #[arg(short = 'p', long, value_enum)]
    pub pattern: Option<PatternArg>,

    /// Complexity level
    #[arg(long, value_enum)]
    pub complexity: Option<ComplexityArg>,

    /// Include deployment architecture details
    #[arg(long, default_value_t = true)]
    pub include_deployment: bool,

    /// Include quality attributes section
    #[arg(long, default_value_t = true)]
    pub include_quality: bool,

    /// Maximum number of components to generate
    #[arg(long, default_value_t = 10)]
    pub max_components: usize,

    /// Technology stack preferences (comma-separated, e.g., "rust,postgres,redis")
    #[arg(long)]
    pub tech_stack: Option<String>,

    /// Output file path
    #[arg(short = 'o', long, default_value = "architecture.md")]
    pub output: PathBuf,

    /// Ollama model to use (defaults from config)
    #[arg(long)]
    pub model: Option<String>,

    /// Ollama base URL (defaults from config)
    #[arg(long)]
    pub ollama_url: Option<String>,

    /// Run in interactive mode
    #[arg(short = 'i', long)]
    pub interactive: bool,

    /// Skip Ollama health check
    #[arg(long)]
    pub skip_health_check: bool,

    /// Force overwrite if output file exists
    #[arg(short = 'f', long)]
    pub force: bool,
}

/// Architecture pattern argument
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum PatternArg {
    /// Microservices architecture
    Microservices,
    /// Monolithic architecture
    Monolithic,
    /// Event-driven architecture
    EventDriven,
    /// Layered architecture
    Layered,
    /// Hexagonal (ports and adapters) architecture
    Hexagonal,
    /// CQRS (Command Query Responsibility Segregation)
    Cqrs,
    /// Serverless architecture
    Serverless,
}

/// Complexity level argument
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ComplexityArg {
    /// Simple architecture (few components)
    Simple,
    /// Moderate complexity
    Moderate,
    /// Complex architecture (many components)
    Complex,
    /// Enterprise-grade complexity
    Enterprise,
}

/// Arguments for listing templates
#[derive(Debug, Args)]
pub struct ListTemplatesArgs {
    /// Show detailed template information
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Filter by architecture pattern
    #[arg(short = 'p', long, value_enum)]
    pub pattern: Option<PatternArg>,
}

/// Arguments for refining architecture
#[derive(Debug, Args)]
pub struct RefineArgs {
    /// Input architecture document to refine
    #[arg(short = 'i', long)]
    pub input: PathBuf,

    /// Refinement instructions (e.g., "add caching layer")
    #[arg(short = 'r', long)]
    pub refinement: String,

    /// Output file path (defaults to overwriting input)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Ollama model to use
    /// Ollama model to use for refinement (defaults from config)
    #[arg(long)]
    pub model: Option<String>,

    /// Ollama base URL (defaults from config)
    #[arg(long)]
    pub ollama_url: Option<String>,

    /// Create backup of original file
    #[arg(short = 'b', long, default_value_t = true)]
    pub backup: bool,
}

/// Arguments for validating architecture
#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Input architecture document to validate
    #[arg(short = 'i', long)]
    pub input: PathBuf,

    /// Show verbose validation output
    #[arg(short = 'v', long)]
    pub verbose: bool,
}

/// Execute architecture command
///
/// # Arguments
///
/// * `args` - Architecture command arguments
///
/// # Returns
///
/// Returns `Ok(())` on success or `CommandError` on failure
pub async fn execute(args: ArchitectureArgs) -> Result<(), CommandError> {
    match args.action {
        ArchitectureAction::Generate(gen_args) => execute_generate(gen_args).await,
        ArchitectureAction::ListTemplates(list_args) => execute_list_templates(list_args).await,
        ArchitectureAction::Refine(refine_args) => execute_refine(refine_args).await,
        ArchitectureAction::Validate(val_args) => execute_validate(val_args).await,
    }
}

/// Execute architecture generation command
async fn execute_generate(args: GenerateArgs) -> Result<(), CommandError> {
    // Load configuration
    let config = AppConfig::load().unwrap_or_default();

    // Check if output file exists
    if args.output.exists() && !args.force {
        return Err(CommandError::OutputFileExists(args.output.clone()));
    }

    // Determine Ollama URL and model from args or config
    let ollama_url = args.ollama_url.clone().unwrap_or_else(|| {
        config
            .architecture
            .ollama
            .base_url
            .clone()
            .unwrap_or_else(|| config.ollama.base_url.clone())
    });
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| config.architecture.ollama.default_model.clone());
    let templates_dir = PathBuf::from(config.architecture.templates.directory);

    // Health check Ollama unless skipped
    if !args.skip_health_check {
        check_ollama_health(&ollama_url).await?;
    }

    // Interactive mode
    if args.interactive {
        let service = create_architecture_service(&ollama_url, &model, &templates_dir)?;
        let theme = ColorfulTheme::default();
        let session = InteractiveArchitectureSession::new(service, theme);
        session.run().await.map_err(CommandError::SessionError)?;
        return Ok(());
    }

    // Non-interactive mode
    let service = create_architecture_service(&ollama_url, &model, &templates_dir)?;

    let document = if let Some(requirements) = &args.requirements {
        // Generate from requirements
        let options = GenerationOptions {
            pattern: args.pattern.map(convert_pattern_arg),
            complexity: args
                .complexity
                .map(convert_complexity_arg)
                .unwrap_or(ComplexityLevel::Moderate),
            include_deployment: args.include_deployment,
            include_quality_attributes: args.include_quality,
            max_components: Some(args.max_components),
            technology_preferences: args
                .tech_stack
                .as_ref()
                .map(|s| parse_tech_stack(s))
                .unwrap_or_default(),
        };

        println!("Generating architecture from requirements...");
        service
            .generate_from_requirements(requirements, options)
            .await
            .map_err(CommandError::ServiceError)?
    } else if let Some(template_name) = &args.template {
        // Generate from template
        println!(
            "Generating architecture from template '{}'...",
            template_name
        );
        let customization = args.customization.as_deref().unwrap_or("");
        service
            .generate_from_template(template_name, customization)
            .await
            .map_err(CommandError::ServiceError)?
    } else {
        return Err(CommandError::MissingRequirements);
    };

    // Save document
    println!("Saving architecture to {}...", args.output.display());
    service
        .save_architecture(&document, &args.output)
        .await
        .map_err(CommandError::ServiceError)?;

    println!("Architecture document generated successfully!");
    println!("Output: {}", args.output.display());

    Ok(())
}

/// Execute list templates command
async fn execute_list_templates(args: ListTemplatesArgs) -> Result<(), CommandError> {
    // Load configuration
    let config = AppConfig::load().unwrap_or_default();
    let templates_dir = PathBuf::from(config.architecture.templates.directory);

    let repo = FileTemplateRepository::new(templates_dir);
    repo.initialize()
        .await
        .map_err(|e| CommandError::ServiceError(ServiceError::TemplateError(e.to_string())))?;

    let templates = repo
        .list_templates()
        .await
        .map_err(|e| CommandError::ServiceError(ServiceError::TemplateError(e.to_string())))?;

    // Filter by pattern if specified
    let filtered: Vec<_> = if let Some(pattern) = args.pattern {
        let domain_pattern = convert_pattern_arg(pattern);
        templates
            .into_iter()
            .filter(|t| t.pattern == domain_pattern)
            .collect()
    } else {
        templates
    };

    if filtered.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    println!("Available architecture templates:\n");

    for template in filtered {
        if args.verbose {
            println!("Name: {}", template.name);
            println!("Pattern: {:?}", template.pattern);
            println!("Description: {}", template.description);
            println!("Use cases:");
            for use_case in &template.use_cases {
                println!("  - {}", use_case);
            }
            println!();
        } else {
            println!("  {} - {:?}", template.name, template.pattern);
        }
    }

    Ok(())
}

/// Execute refine architecture command
async fn execute_refine(args: RefineArgs) -> Result<(), CommandError> {
    // Load configuration
    let config = AppConfig::load().unwrap_or_default();

    // Check if input file exists
    if !args.input.exists() {
        return Err(CommandError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file not found: {}", args.input.display()),
        )));
    }

    // Read input document
    let content = tokio::fs::read_to_string(&args.input)
        .await
        .map_err(CommandError::IoError)?;

    let document = parse_architecture_document(&content)?;

    // Create backup if requested or configured
    let should_backup = args.backup || config.architecture.output.create_backup;
    if should_backup {
        let backup_path = args.input.with_extension("md.bak");
        tokio::fs::copy(&args.input, &backup_path)
            .await
            .map_err(CommandError::IoError)?;
        println!("Backup created: {}", backup_path.display());
    }

    // Determine Ollama URL and model from args or config
    let ollama_url = args.ollama_url.clone().unwrap_or_else(|| {
        config
            .architecture
            .ollama
            .base_url
            .clone()
            .unwrap_or_else(|| config.ollama.base_url.clone())
    });
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| config.architecture.ollama.default_model.clone());
    let templates_dir = PathBuf::from(config.architecture.templates.directory);

    // Health check Ollama
    check_ollama_health(&ollama_url).await?;

    // Refine architecture
    let service = create_architecture_service(&ollama_url, &model, &templates_dir)?;
    println!("Refining architecture...");
    let refined = service
        .refine_architecture(&document, &args.refinement)
        .await
        .map_err(CommandError::ServiceError)?;

    // Save refined document
    let output_path = args.output.unwrap_or_else(|| args.input.clone());
    println!(
        "Saving refined architecture to {}...",
        output_path.display()
    );
    service
        .save_architecture(&refined, &output_path)
        .await
        .map_err(CommandError::ServiceError)?;

    println!("Architecture refined successfully!");
    println!("Output: {}", output_path.display());

    Ok(())
}

/// Execute validate architecture command
async fn execute_validate(args: ValidateArgs) -> Result<(), CommandError> {
    // Check if input file exists
    if !args.input.exists() {
        return Err(CommandError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file not found: {}", args.input.display()),
        )));
    }

    // Read and parse document
    let content = tokio::fs::read_to_string(&args.input)
        .await
        .map_err(CommandError::IoError)?;

    let document = parse_architecture_document(&content)?;

    // Validate
    let validator = ArchitectureValidator;
    match validator.validate(&document) {
        Ok(_) => {
            println!("Architecture document is valid!");
            if args.verbose {
                println!("Title: {}", document.metadata.title);
                println!("Layers: {}", document.layers.len());
                println!("Components: {}", document.components.len());
                println!("Integrations: {}", document.integrations.len());
            }
            Ok(())
        }
        Err(error) => {
            println!("Validation failed:\n");
            println!("  - {:?}", error);
            Err(CommandError::ValidationError(error))
        }
    }
}

/// Create architecture service with configured generator and writer
fn create_architecture_service(
    ollama_url: &str,
    model: &str,
    templates_dir: &std::path::Path,
) -> Result<
    ArchitectureService<
        OllamaArchitectureGenerator,
        MarkdownArchitectureWriter,
        FileTemplateRepository,
    >,
    CommandError,
> {
    let config = OllamaConfig::new()
        .with_base_url(ollama_url)
        .with_default_model(model);

    let generator = OllamaArchitectureGenerator::new(config)
        .map_err(|e| CommandError::OllamaUnavailable(e.to_string()))?;
    let writer = MarkdownArchitectureWriter::new();
    let template_repo = FileTemplateRepository::new(templates_dir.to_path_buf());

    Ok(ArchitectureService::new(generator, writer, template_repo))
}

/// Check if Ollama service is available
async fn check_ollama_health(base_url: &str) -> Result<(), CommandError> {
    let client = reqwest::Client::new();
    let health_url = format!("{}/api/tags", base_url);

    match client.get(&health_url).send().await {
        Ok(response) if response.status().is_success() => Ok(()),
        Ok(response) => Err(CommandError::OllamaUnavailable(format!(
            "Ollama service returned status: {}",
            response.status()
        ))),
        Err(e) => Err(CommandError::OllamaUnavailable(format!(
            "Cannot connect to Ollama at {}: {}",
            base_url, e
        ))),
    }
}

/// Convert PatternArg to domain ArchitecturePattern
fn convert_pattern_arg(pattern: PatternArg) -> ArchitecturePattern {
    match pattern {
        PatternArg::Microservices => ArchitecturePattern::Microservices,
        PatternArg::Monolithic => ArchitecturePattern::Monolithic,
        PatternArg::EventDriven => ArchitecturePattern::EventDriven,
        PatternArg::Layered => ArchitecturePattern::Layered,
        PatternArg::Hexagonal => ArchitecturePattern::Hexagonal,
        PatternArg::Cqrs => ArchitecturePattern::CQRS,
        PatternArg::Serverless => ArchitecturePattern::Serverless,
    }
}

/// Convert ComplexityArg to domain ComplexityLevel
fn convert_complexity_arg(complexity: ComplexityArg) -> ComplexityLevel {
    match complexity {
        ComplexityArg::Simple => ComplexityLevel::Simple,
        ComplexityArg::Moderate => ComplexityLevel::Moderate,
        ComplexityArg::Complex => ComplexityLevel::Complex,
        ComplexityArg::Enterprise => ComplexityLevel::Enterprise,
    }
}

/// Parse comma-separated technology stack string
fn parse_tech_stack(tech_stack: &str) -> Vec<String> {
    tech_stack
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parse architecture document from markdown content
fn parse_architecture_document(
    _content: &str,
) -> Result<crate::domain::architecture::ArchitectureDocument, CommandError> {
    // TODO: Implement proper markdown parsing
    // For now, return error as parsing is not implemented
    Err(CommandError::IoError(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Markdown parsing not yet implemented",
    )))
}

/// Command execution errors
#[derive(Debug, Error)]
pub enum CommandError {
    /// Output file already exists
    #[error("Output file already exists: {0}. Use --force to overwrite")]
    OutputFileExists(PathBuf),

    /// Missing requirements or template
    #[error("Either --requirements or --template must be provided")]
    MissingRequirements,

    /// Ollama service unavailable
    #[error("Ollama service unavailable: {0}")]
    OllamaUnavailable(String),

    /// Service error
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),

    /// Session error
    #[error("Interactive session error: {0}")]
    SessionError(#[from] SessionError),

    /// Validation error
    #[error("Validation failed: {0}")]
    ValidationError(#[from] ValidationError),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_pattern_arg() {
        assert_eq!(
            convert_pattern_arg(PatternArg::Microservices),
            ArchitecturePattern::Microservices
        );
        assert_eq!(
            convert_pattern_arg(PatternArg::Layered),
            ArchitecturePattern::Layered
        );
        assert_eq!(
            convert_pattern_arg(PatternArg::Cqrs),
            ArchitecturePattern::CQRS
        );
    }

    #[test]
    fn test_convert_complexity_arg() {
        assert_eq!(
            convert_complexity_arg(ComplexityArg::Simple),
            ComplexityLevel::Simple
        );
        assert_eq!(
            convert_complexity_arg(ComplexityArg::Enterprise),
            ComplexityLevel::Enterprise
        );
    }

    #[test]
    fn test_parse_tech_stack() {
        let result = parse_tech_stack("rust,postgres,redis");
        assert_eq!(result, vec!["rust", "postgres", "redis"]);

        let result = parse_tech_stack("rust, postgres , redis ");
        assert_eq!(result, vec!["rust", "postgres", "redis"]);

        let result = parse_tech_stack("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_tech_stack_with_spaces() {
        let result = parse_tech_stack("rust,  ,postgres");
        assert_eq!(result, vec!["rust", "postgres"]);
    }
}
