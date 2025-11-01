//! Implementation command for generating implementation plans
//!
//! This module provides the CLI interface for generating implementation plans
//! from architecture documents using Ollama LLM services.

use crate::application::{
    ApplicationError, InteractiveConfig, InteractivePlanningSession, PlanningService,
};
use crate::domain::planning::PlanOptions;
use crate::infrastructure::fileio::{MarkdownArchitectureParser, MarkdownPlanWriter};
use crate::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};
use clap::Args;
use std::path::PathBuf;

/// Arguments for the implementation command
///
/// Generates implementation plans from architecture documents using Ollama LLM.
///
/// # Examples
///
/// ```bash
/// # Interactive mode
/// xzagentz implementation --interactive
///
/// # Non-interactive mode with specific model
/// xzagentz implementation --architecture docs/architecture.md \
///     --output plans/implementation.md \
///     --model llama3
///
/// # With custom Ollama URL
/// xzagentz implementation --architecture arch.md \
///     --ollama-url http://localhost:11434
/// ```
#[derive(Args, Debug)]
pub struct ImplementationArgs {
    /// Path to architecture document (markdown file)
    #[arg(short, long, value_name = "FILE")]
    pub architecture: Option<PathBuf>,

    /// Output path for generated implementation plan
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Ollama model to use for generation
    #[arg(short, long, default_value = "llama3")]
    pub model: String,

    /// Ollama service URL
    #[arg(long, default_value = "http://localhost:11434")]
    pub ollama_url: String,

    /// Run in interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Skip Ollama health check
    #[arg(long)]
    pub skip_health_check: bool,

    /// Number of phases to generate (auto-detect if not specified)
    #[arg(short = 'n', long, value_name = "NUM")]
    pub num_phases: Option<usize>,

    /// Force overwrite existing output file
    #[arg(short, long)]
    pub force: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Disable progress indicators
    #[arg(long)]
    pub no_progress: bool,

    /// Skip confirmation before saving (non-interactive mode only)
    #[arg(long)]
    pub yes: bool,
}

/// Executes the implementation command
///
/// # Arguments
///
/// * `args` - Command-line arguments
/// * `verbose` - Enable verbose output
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if generation fails
///
/// # Errors
///
/// Returns error if:
/// - Ollama service is unavailable
/// - Architecture file cannot be read or parsed
/// - Plan generation fails
/// - Output file cannot be written
pub async fn execute(
    args: ImplementationArgs,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Running implementation command...");
        println!("  Ollama URL: {}", args.ollama_url);
        println!("  Model: {}", args.model);
        println!("  Interactive: {}", args.interactive);
    }

    // Create Ollama configuration
    let ollama_config = OllamaConfig::new()
        .with_base_url(&args.ollama_url)
        .with_default_model(&args.model)
        .with_timeout_seconds(120)
        .with_max_retries(3);

    if verbose {
        println!("Creating Ollama client...");
    }

    // Create Ollama client
    let client = OllamaClient::new(ollama_config.clone()).map_err(|e| {
        format!(
            "Failed to create Ollama client: {}. Is Ollama running at {}?",
            e, args.ollama_url
        )
    })?;

    // Health check (unless skipped)
    if !args.skip_health_check {
        if verbose {
            println!("Checking Ollama service health...");
        }

        match client.health_check().await {
            Ok(true) => {
                if verbose {
                    println!("Ollama service is healthy");
                }
            }
            Ok(false) => {
                return Err(format!(
                    "Ollama service at {} is not responding properly",
                    args.ollama_url
                )
                .into());
            }
            Err(e) => {
                return Err(
                    format!("Failed to connect to Ollama at {}: {}", args.ollama_url, e).into(),
                );
            }
        }
    }

    // Create infrastructure components
    let generator = OllamaPlanGenerator::new(client);
    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();

    // Create planning service
    let service = PlanningService::new(generator, parser, writer);

    // Execute based on mode
    if args.interactive {
        execute_interactive(service, args, verbose)
    } else {
        execute_non_interactive(service, args, verbose)
    }
}

/// Executes the command in interactive mode
fn execute_interactive<G, P, W>(
    service: PlanningService<G, P, W>,
    args: ImplementationArgs,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    G: crate::domain::planning::PlanGenerator,
    P: crate::domain::planning::ArchitectureParser,
    W: crate::domain::planning::PlanWriter,
{
    if verbose {
        println!("Starting interactive session...");
    }

    // Create interactive configuration
    let config = InteractiveConfig {
        enable_colors: !args.no_color,
        show_progress: !args.no_progress,
        confirm_before_save: !args.yes,
        default_output_dir: Some("./plans".to_string()),
    };

    // Create and run interactive session
    let session = InteractivePlanningSession::new(service, config);

    match session.run() {
        Ok(plan) => {
            if verbose {
                println!("Plan generated successfully");
                println!("  Title: {}", plan.title());
                println!("  Phases: {}", plan.phases().len());
            }
            Ok(())
        }
        Err(ApplicationError::Cancelled) => {
            println!("Operation cancelled by user");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// Executes the command in non-interactive mode
fn execute_non_interactive<G, P, W>(
    service: PlanningService<G, P, W>,
    args: ImplementationArgs,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    G: crate::domain::planning::PlanGenerator,
    P: crate::domain::planning::ArchitectureParser,
    W: crate::domain::planning::PlanWriter,
{
    // Validate required arguments
    let architecture_path = args.architecture.ok_or(
        "Architecture file path is required in non-interactive mode. Use --architecture <FILE>",
    )?;

    let output_path = args
        .output
        .ok_or("Output file path is required in non-interactive mode. Use --output <FILE>")?;

    if verbose {
        println!("Architecture: {}", architecture_path.display());
        println!("Output: {}", output_path.display());
    }

    // Build plan options
    let mut options = PlanOptions::new().with_model(&args.model);

    if let Some(num_phases) = args.num_phases {
        options = options.with_num_phases(num_phases);
        if verbose {
            println!("Number of phases: {}", num_phases);
        }
    } else if verbose {
        println!("Number of phases: auto-detect");
    }

    // Generate plan
    if verbose {
        println!("Generating implementation plan...");
    }

    let plan = service.generate_implementation_plan(&architecture_path, &options)?;

    if verbose {
        println!("Plan generated:");
        println!("  Title: {}", plan.title());
        println!("  Phases: {}", plan.phases().len());
        let total_tasks: usize = plan.phases().iter().map(|p| p.tasks().len()).sum();
        println!("  Total Tasks: {}", total_tasks);
    }

    // Save plan
    if verbose {
        println!("Saving plan to {}...", output_path.display());
    }

    service.save_plan(&plan, &output_path, args.force)?;

    println!("Implementation plan saved to {}", output_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(flatten)]
        args: ImplementationArgs,
    }

    #[test]
    fn test_parse_interactive() {
        let cli = TestCli::parse_from(["test", "--interactive"]);
        assert!(cli.args.interactive);
        assert_eq!(cli.args.model, "llama3");
        assert_eq!(cli.args.ollama_url, "http://localhost:11434");
    }

    #[test]
    fn test_parse_non_interactive() {
        let cli = TestCli::parse_from([
            "test",
            "--architecture",
            "arch.md",
            "--output",
            "plan.md",
            "--model",
            "llama2",
        ]);
        assert!(!cli.args.interactive);
        assert_eq!(cli.args.architecture, Some(PathBuf::from("arch.md")));
        assert_eq!(cli.args.output, Some(PathBuf::from("plan.md")));
        assert_eq!(cli.args.model, "llama2");
    }

    #[test]
    fn test_parse_custom_ollama_url() {
        let cli = TestCli::parse_from([
            "test",
            "--interactive",
            "--ollama-url",
            "http://remote:8080",
        ]);
        assert_eq!(cli.args.ollama_url, "http://remote:8080");
    }

    #[test]
    fn test_parse_num_phases() {
        let cli = TestCli::parse_from(["test", "--interactive", "--num-phases", "7"]);
        assert_eq!(cli.args.num_phases, Some(7));
    }

    #[test]
    fn test_parse_force_flag() {
        let cli = TestCli::parse_from(["test", "--interactive", "--force"]);
        assert!(cli.args.force);
    }

    #[test]
    fn test_parse_skip_health_check() {
        let cli = TestCli::parse_from(["test", "--interactive", "--skip-health-check"]);
        assert!(cli.args.skip_health_check);
    }

    #[test]
    fn test_parse_no_color() {
        let cli = TestCli::parse_from(["test", "--interactive", "--no-color"]);
        assert!(cli.args.no_color);
    }

    #[test]
    fn test_parse_no_progress() {
        let cli = TestCli::parse_from(["test", "--interactive", "--no-progress"]);
        assert!(cli.args.no_progress);
    }

    #[test]
    fn test_parse_yes_flag() {
        let cli = TestCli::parse_from(["test", "--interactive", "--yes"]);
        assert!(cli.args.yes);
    }

    #[test]
    fn test_default_values() {
        let cli = TestCli::parse_from(["test", "--interactive"]);
        assert_eq!(cli.args.model, "llama3");
        assert_eq!(cli.args.ollama_url, "http://localhost:11434");
        assert!(!cli.args.force);
        assert!(!cli.args.skip_health_check);
        assert!(!cli.args.no_color);
        assert!(!cli.args.no_progress);
        assert!(!cli.args.yes);
        assert_eq!(cli.args.num_phases, None);
    }

    #[test]
    fn test_architecture_and_output_optional() {
        let cli = TestCli::parse_from(["test", "--interactive"]);
        assert_eq!(cli.args.architecture, None);
        assert_eq!(cli.args.output, None);
    }
}
