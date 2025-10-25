//! CLI commands for prompt generation and management
//!
//! This module provides command-line interface for:
//! - Generating prompts from implementation plans
//! - Tracking progress through sections
//! - Managing prompt templates
//! - Verifying compliance

use crate::error::Result;
use crate::plans::parser::PlanParser;
use crate::prompts::{ContextExtractor, ProgressTracker, PromptGenerator, PromptGeneratorConfig};
use clap::{Args, Subcommand};
use std::path::{Path, PathBuf};

/// Prompt generation and management commands
#[derive(Debug, Args)]
pub struct PromptArgs {
    #[command(subcommand)]
    pub command: PromptCommand,
}

/// Prompt subcommands
#[derive(Debug, Subcommand)]
pub enum PromptCommand {
    /// Generate prompts from implementation plan
    Generate(GenerateArgs),

    /// Show a specific prompt without generating
    Show(ShowArgs),

    /// Mark a section as complete
    Complete(CompleteArgs),

    /// Show next section to work on
    Next(NextArgs),

    /// Show implementation progress
    Progress(ProgressArgs),

    /// Verify compliance with AGENTS.md rules
    Verify(VerifyArgs),

    /// Reset progress tracking
    Reset(ResetArgs),
}

/// Generate prompts
#[derive(Debug, Args)]
pub struct GenerateArgs {
    /// Path to implementation plan
    #[arg(
        short,
        long,
        default_value = "docs/explanations/implementation_plan.md"
    )]
    pub plan: PathBuf,

    /// Path to architecture plan (optional)
    #[arg(short, long)]
    pub architecture: Option<PathBuf>,

    /// Output directory for prompts
    #[arg(short, long, default_value = "prompts")]
    pub output: PathBuf,

    /// Generate all prompts (batch mode)
    #[arg(long)]
    pub all: bool,

    /// Interactive mode with preview
    #[arg(short, long)]
    pub interactive: bool,

    /// Specific phase to generate
    #[arg(long)]
    pub phase: Option<usize>,

    /// Specific section to generate (requires --phase)
    #[arg(long)]
    pub section: Option<String>,

    /// Force overwrite existing files
    #[arg(short, long)]
    pub force: bool,

    /// Don't create backups when overwriting
    #[arg(long)]
    pub no_backup: bool,

    /// JIRA issue for commit messages
    #[arg(long)]
    pub jira_issue: Option<String>,
}

/// Show prompt without generating
#[derive(Debug, Args)]
pub struct ShowArgs {
    /// Path to implementation plan
    #[arg(
        short,
        long,
        default_value = "docs/explanations/implementation_plan.md"
    )]
    pub plan: PathBuf,

    /// Phase number
    #[arg(long)]
    pub phase: usize,

    /// Section number (e.g., "1.1")
    #[arg(long)]
    pub section: String,
}

/// Mark section complete
#[derive(Debug, Args)]
pub struct CompleteArgs {
    /// Section number to mark complete (e.g., "1.1")
    pub section: String,

    /// Path to project root (for .implementation_progress file)
    #[arg(short, long, default_value = ".")]
    pub project_root: PathBuf,
}

/// Show next section
#[derive(Debug, Args)]
pub struct NextArgs {
    /// Path to implementation plan
    #[arg(
        short,
        long,
        default_value = "docs/explanations/implementation_plan.md"
    )]
    pub plan: PathBuf,

    /// Path to project root (for .implementation_progress file)
    #[arg(short = 'r', long, default_value = ".")]
    pub project_root: PathBuf,

    /// Generate prompt for next section
    #[arg(short, long)]
    pub generate: bool,
}

/// Show progress
#[derive(Debug, Args)]
pub struct ProgressArgs {
    /// Path to project root (for .implementation_progress file)
    #[arg(short, long, default_value = ".")]
    pub project_root: PathBuf,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    pub format: String,
}

/// Verify compliance
#[derive(Debug, Args)]
pub struct VerifyArgs {
    /// Path to project root
    #[arg(short, long, default_value = ".")]
    pub project_root: PathBuf,

    /// Specific phase to verify
    #[arg(long)]
    pub phase: Option<usize>,

    /// Output report file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Strict mode (fail on any violation)
    #[arg(short, long)]
    pub strict: bool,
}

/// Reset progress
#[derive(Debug, Args)]
pub struct ResetArgs {
    /// Path to project root (for .implementation_progress file)
    #[arg(short, long, default_value = ".")]
    pub project_root: PathBuf,

    /// Skip confirmation prompt
    #[arg(short, long)]
    pub yes: bool,
}

/// Execute prompt command
pub fn execute(args: PromptArgs) -> Result<()> {
    match args.command {
        PromptCommand::Generate(gen_args) => generate_prompts(gen_args),
        PromptCommand::Show(show_args) => show_prompt(show_args),
        PromptCommand::Complete(complete_args) => complete_section(complete_args),
        PromptCommand::Next(next_args) => show_next(next_args),
        PromptCommand::Progress(progress_args) => show_progress(progress_args),
        PromptCommand::Verify(verify_args) => verify_compliance(verify_args),
        PromptCommand::Reset(reset_args) => reset_progress(reset_args),
    }
}

/// Generate prompts command
fn generate_prompts(args: GenerateArgs) -> Result<()> {
    println!("Generating prompts from: {}", args.plan.display());

    // Parse implementation plan
    let parser = PlanParser::new();
    let plan_content =
        std::fs::read_to_string(&args.plan).map_err(|e| crate::error::Error::FileIo {
            path: args.plan.clone(),
            source: e,
        })?;
    let plan = parser.parse_implementation_plan(&plan_content)?;

    // Load architecture plan if provided
    let arch_plan = if let Some(arch_path) = &args.architecture {
        let arch_content =
            std::fs::read_to_string(arch_path).map_err(|e| crate::error::Error::FileIo {
                path: arch_path.clone(),
                source: e,
            })?;
        Some(parser.parse_architecture_plan(&arch_content)?)
    } else {
        None
    };

    // Configure generator
    let config = PromptGeneratorConfig {
        output_dir: args.output.clone(),
        force_overwrite: args.force,
        create_backup: !args.no_backup,
        include_full_architecture: arch_plan.is_some(),
        jira_issue: args.jira_issue.unwrap_or_default(),
        project_name: plan.metadata.name.clone(),
        project_description: plan.metadata.description.clone().unwrap_or_default(),
    };

    let generator = PromptGenerator::with_config(config)?;

    // Load or create progress tracker
    let tracker = ProgressTracker::new(Path::new("."))?;

    // Generate based on mode
    let generated = if args.interactive {
        generator.generate_interactive(&plan, Some(&tracker))?
    } else if args.all {
        generator.generate_all(&plan, Some(&tracker))?
    } else if let (Some(phase_num), Some(section_num)) = (args.phase, args.section) {
        // Generate specific section
        let phase = plan
            .phases
            .iter()
            .find(|p| p.number == phase_num)
            .ok_or_else(|| crate::error::Error::Other {
                message: format!("Phase {} not found", phase_num),
            })?;

        let section = phase
            .sections
            .iter()
            .find(|s| s.number == section_num)
            .ok_or_else(|| crate::error::Error::Other {
                message: format!("Section {} not found", section_num),
            })?;

        let extractor = ContextExtractor::new();
        let context = extractor.extract_context(&plan, arch_plan.as_ref(), phase, section)?;
        let path = generator.generate_section_prompt(&context)?;
        vec![path]
    } else {
        return Err(crate::error::Error::Other {
            message: "Must specify --all, --interactive, or --phase and --section".to_string(),
        });
    };

    println!("\n✓ Generated {} prompts:", generated.len());
    for path in generated {
        println!("  - {}", path.display());
    }

    Ok(())
}

/// Show prompt command
fn show_prompt(args: ShowArgs) -> Result<()> {
    let parser = PlanParser::new();
    let plan_content =
        std::fs::read_to_string(&args.plan).map_err(|e| crate::error::Error::FileIo {
            path: args.plan.clone(),
            source: e,
        })?;
    let plan = parser.parse_implementation_plan(&plan_content)?;

    let phase = plan
        .phases
        .iter()
        .find(|p| p.number == args.phase)
        .ok_or_else(|| crate::error::Error::Other {
            message: format!("Phase {} not found", args.phase),
        })?;

    let section = phase
        .sections
        .iter()
        .find(|s| s.number == args.section)
        .ok_or_else(|| crate::error::Error::Other {
            message: format!("Section {} not found", args.section),
        })?;

    let extractor = ContextExtractor::new();
    let context = extractor.extract_context(&plan, None, phase, section)?;

    let template = crate::prompts::template::PromptTemplate::load_embedded("section_prompt")?;
    let rendered = template.render(&context)?;

    println!("{}", rendered);

    Ok(())
}

/// Complete section command
fn complete_section(args: CompleteArgs) -> Result<()> {
    let mut tracker = ProgressTracker::new(&args.project_root)?;
    tracker.complete(&args.section)?;

    let stats = tracker.stats();
    println!("✓ Section {} marked complete", args.section);
    println!(
        "Progress: {:.1}% ({}/{})",
        stats.percentage, stats.completed, stats.total_sections
    );

    Ok(())
}

/// Show next section command
fn show_next(args: NextArgs) -> Result<()> {
    let tracker = ProgressTracker::new(&args.project_root)?;
    let parser = PlanParser::new();
    let plan_content =
        std::fs::read_to_string(&args.plan).map_err(|e| crate::error::Error::FileIo {
            path: args.plan.clone(),
            source: e,
        })?;
    let plan = parser.parse_implementation_plan(&plan_content)?;

    // Build list of all sections
    let all_sections: Vec<String> = plan
        .phases
        .iter()
        .flat_map(|p| p.sections.iter().map(|s| s.number.clone()))
        .collect();

    let section_refs: Vec<&str> = all_sections.iter().map(|s| s.as_str()).collect();

    if let Some(next) = tracker.next_section(&section_refs) {
        println!("Next section: {}", next);

        // Find the phase and section
        for phase in &plan.phases {
            if let Some(section) = phase.sections.iter().find(|s| s.number == next) {
                println!("\nSection: {} - {}", section.number, section.title);
                println!("Phase: {} - {}", phase.number, phase.title);
                println!("Tasks: {}", section.tasks.len());
                println!("Deliverables: {}", section.deliverables.len());

                if args.generate {
                    println!("\nGenerating prompt...");
                    let extractor = ContextExtractor::new();
                    let context = extractor.extract_context(&plan, None, phase, section)?;

                    let config = PromptGeneratorConfig {
                        output_dir: PathBuf::from("prompts"),
                        project_name: plan.metadata.name.clone(),
                        project_description: plan.metadata.description.clone().unwrap_or_default(),
                        ..Default::default()
                    };

                    let generator = PromptGenerator::with_config(config)?;
                    let path = generator.generate_section_prompt(&context)?;
                    println!("Generated: {}", path.display());
                }

                break;
            }
        }
    } else {
        println!("All sections complete!");
    }

    Ok(())
}

/// Show progress command
fn show_progress(args: ProgressArgs) -> Result<()> {
    let tracker = ProgressTracker::new(&args.project_root)?;
    let stats = tracker.stats();

    match args.format.as_str() {
        "json" => {
            println!(
                r#"{{"total": {}, "completed": {}, "percentage": {:.1}, "current": "{}"}}"#,
                stats.total_sections, stats.completed, stats.percentage, stats.current_section
            );
        }
        _ => {
            println!("Implementation Progress");
            println!("======================\n");
            println!("Current Section: {}", stats.current_section);
            println!("Completed: {} / {}", stats.completed, stats.total_sections);
            println!("Remaining: {}", stats.remaining);
            println!("Progress: {:.1}%", stats.percentage);

            // Progress bar
            let bar_width = 50;
            let filled = (stats.percentage / 100.0 * bar_width as f64) as usize;
            let empty = bar_width - filled;
            println!(
                "\n[{}{}] {:.1}%",
                "=".repeat(filled),
                " ".repeat(empty),
                stats.percentage
            );
        }
    }

    Ok(())
}

/// Verify compliance command
fn verify_compliance(_args: VerifyArgs) -> Result<()> {
    println!("Compliance verification not yet implemented");
    println!("This will check:");
    println!("  - Markdown file naming conventions");
    println!("  - YAML file extensions (.yaml not .yml)");
    println!("  - Code block language identifiers");
    println!("  - No emojis in code/docs");
    println!("  - Error handling patterns");
    println!("  - Documentation completeness");

    Ok(())
}

/// Reset progress command
fn reset_progress(args: ResetArgs) -> Result<()> {
    if !args.yes {
        println!("This will reset all progress tracking. Are you sure? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let mut tracker = ProgressTracker::new(&args.project_root)?;
    tracker.reset()?;
    println!("✓ Progress reset to start");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_args_structure() {
        // Verify args structure compiles
        let _args = PromptArgs {
            command: PromptCommand::Progress(ProgressArgs {
                project_root: PathBuf::from("."),
                format: "text".to_string(),
            }),
        };
    }

    #[test]
    fn test_generate_args_defaults() {
        let args = GenerateArgs {
            plan: PathBuf::from("plan.md"),
            architecture: None,
            output: PathBuf::from("prompts"),
            all: false,
            interactive: false,
            phase: None,
            section: None,
            force: false,
            no_backup: false,
            jira_issue: None,
        };

        assert_eq!(args.output, PathBuf::from("prompts"));
        assert!(!args.force);
    }
}
