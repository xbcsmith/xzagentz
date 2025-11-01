//! CLI module for xzagentz
//!
//! This module provides the command-line interface using clap v4 with type-safe argument parsing.
//!
//! # Architecture
//!
//! The CLI is structured as:
//! - Main `Cli` struct with global options
//! - `Commands` enum for subcommands (list, validate, create, etc.)
//! - Each command has its own implementation module

pub mod add;
pub mod create;
pub mod implementation;
pub mod init;
pub mod list;
pub mod output;
pub mod prompt;
pub mod update;
pub mod validate;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// xzagentz - AI Agent Development Guidelines and Template Manager
#[derive(Parser, Debug)]
#[command(name = "xzagentz")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Path to configuration directory
    #[arg(long, global = true, env = "XZAGENTZ_CONFIG_DIR")]
    pub config_dir: Option<PathBuf>,

    /// Path to templates directory
    #[arg(long, global = true, env = "XZAGENTZ_TEMPLATE_DIR")]
    pub template_dir: Option<PathBuf>,

    /// Path to components directory
    #[arg(long, global = true, env = "XZAGENTZ_COMPONENT_DIR")]
    pub component_dir: Option<PathBuf>,

    /// Output format
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Human)]
    pub format: OutputFormat,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output (default)
    Human,
    /// JSON output for machine parsing
    Json,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize xzagentz by extracting embedded resources
    Init {
        /// Custom components directory (default: ~/.config/xzagentz/components)
        #[arg(short, long)]
        components_dir: Option<PathBuf>,

        /// Custom templates directory (default: ~/.config/xzagentz/templates)
        #[arg(short, long)]
        templates_dir: Option<PathBuf>,

        /// Force overwrite existing files
        #[arg(short, long)]
        force: bool,

        /// Dry run - show what would be done without doing it
        #[arg(short = 'n', long)]
        dry_run: bool,
    },

    /// List available components or templates
    List {
        /// What to list
        #[command(subcommand)]
        target: ListTarget,
    },

    /// Validate an existing AGENTS.md file
    Validate {
        /// Path to AGENTS.md file to validate
        #[arg(default_value = "AGENTS.md")]
        file: PathBuf,

        /// Show detailed validation report
        #[arg(short, long)]
        detailed: bool,

        /// Fix common issues automatically
        #[arg(short, long)]
        fix: bool,
    },

    /// Create a new AGENTS.md file
    Create {
        /// Output file path
        #[arg(short, long, default_value = "AGENTS.md")]
        output: PathBuf,

        /// Template to use
        #[arg(short, long)]
        template: Option<String>,

        /// Force overwrite if file exists
        #[arg(short, long)]
        force: bool,

        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Update an existing AGENTS.md file
    Update {
        /// Path to AGENTS.md file to update
        #[arg(default_value = "AGENTS.md")]
        file: PathBuf,

        /// Section to update
        #[arg(short, long)]
        section: String,

        /// Create backup before updating
        #[arg(short, long, default_value_t = true)]
        backup: bool,
    },

    /// Add new sections to an AGENTS.md file
    Add {
        /// Path to AGENTS.md file
        #[arg(default_value = "AGENTS.md")]
        file: PathBuf,

        /// Component or section to add
        #[arg(short, long)]
        component: String,

        /// Position to insert
        #[arg(short, long, default_value = "bottom")]
        position: String,
    },

    /// Prompt generation and management commands
    Prompt(prompt::PromptArgs),

    /// Generate implementation plans from architecture documents
    Implementation(implementation::ImplementationArgs),
}

/// List command targets
#[derive(Subcommand, Debug)]
pub enum ListTarget {
    /// List available components
    Components {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// List available templates
    Templates {
        /// Show template details
        #[arg(short, long)]
        detailed: bool,
    },
}

impl Cli {
    /// Get the effective template directory path
    pub fn get_template_dir(&self) -> PathBuf {
        self.template_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from("templates"))
    }

    /// Get the effective component directory path
    pub fn get_component_dir(&self) -> PathBuf {
        self.component_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from("components"))
    }

    /// Get the effective config directory path
    pub fn get_config_dir(&self) -> Option<PathBuf> {
        self.config_dir
            .clone()
            .or_else(|| dirs::config_dir().map(|dir| dir.join("xzagentz")))
    }
}

impl OutputFormat {
    /// Check if format is JSON
    pub fn is_json(&self) -> bool {
        matches!(self, OutputFormat::Json)
    }

    /// Check if format is human-readable
    pub fn is_human(&self) -> bool {
        matches!(self, OutputFormat::Human)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_components() {
        let cli = Cli::parse_from(["xzagentz", "list", "components"]);
        assert!(matches!(cli.command, Commands::List { .. }));
    }

    #[test]
    fn test_parse_validate() {
        let cli = Cli::parse_from(["xzagentz", "validate"]);
        if let Commands::Validate {
            file,
            detailed,
            fix,
        } = cli.command
        {
            assert_eq!(file, PathBuf::from("AGENTS.md"));
            assert!(!detailed);
            assert!(!fix);
        } else {
            panic!("Expected Validate command");
        }
    }

    #[test]
    fn test_verbose_flag() {
        let cli = Cli::parse_from(["xzagentz", "-v", "list", "components"]);
        assert!(cli.verbose);
    }

    #[test]
    fn test_json_format() {
        let cli = Cli::parse_from(["xzagentz", "--format", "json", "list", "components"]);
        assert_eq!(cli.format, OutputFormat::Json);
        assert!(cli.format.is_json());
    }
}
