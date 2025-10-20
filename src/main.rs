//! xzagentz - AI Agent Development Guidelines and Template Manager
//!
//! A CLI tool for managing AI agent development guidelines, project templates,
//! and implementation plans.

use clap::Parser;
use std::process;
use xzagentz::cli::{Cli, Commands};
use xzagentz::error::Error;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Main application entry point
///
/// # Arguments
///
/// * `cli` - Parsed CLI arguments
///
/// # Errors
///
/// Returns an error if command execution fails
fn run(cli: Cli) -> Result<(), Error> {
    // Get effective directories
    let component_dir = cli.get_component_dir();
    let template_dir = cli.get_template_dir();

    if cli.verbose {
        eprintln!("xzagentz v{}", env!("CARGO_PKG_VERSION"));
        eprintln!("Component dir: {:?}", component_dir);
        eprintln!("Template dir: {:?}", template_dir);
        eprintln!("Output format: {:?}", cli.format);
    }

    // Dispatch to command handlers
    match cli.command {
        Commands::List { target } => {
            xzagentz::cli::list::execute(
                &target,
                &component_dir,
                &template_dir,
                cli.format,
                cli.verbose,
            )?;
        }
        Commands::Validate {
            file,
            detailed,
            fix,
        } => {
            xzagentz::cli::validate::execute(&file, detailed, fix, cli.format, cli.verbose)?;
        }
        Commands::Create {
            output,
            template,
            force,
            interactive,
        } => {
            eprintln!("Create command not yet implemented");
            eprintln!("  Output: {:?}", output);
            eprintln!("  Template: {:?}", template);
            eprintln!("  Force: {}", force);
            eprintln!("  Interactive: {}", interactive);
            return Err(Error::NotImplemented(
                "Create command will be implemented in Phase 5".to_string(),
            ));
        }
        Commands::Update {
            file,
            section,
            backup,
        } => {
            eprintln!("Update command not yet implemented");
            eprintln!("  File: {:?}", file);
            eprintln!("  Section: {}", section);
            eprintln!("  Backup: {}", backup);
            return Err(Error::NotImplemented(
                "Update command will be implemented in Phase 6".to_string(),
            ));
        }
        Commands::Add {
            file,
            component,
            position,
        } => {
            eprintln!("Add command not yet implemented");
            eprintln!("  File: {:?}", file);
            eprintln!("  Component: {}", component);
            eprintln!("  Position: {}", position);
            return Err(Error::NotImplemented(
                "Add command will be implemented in Phase 6".to_string(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_list_components() {
        let cli = Cli::parse_from(["xzagentz", "list", "components"]);
        // This will fail if components directory doesn't exist, but that's expected
        let _result = run(cli);
    }

    #[test]
    fn test_run_list_templates() {
        let cli = Cli::parse_from(["xzagentz", "list", "templates"]);
        // This will fail if templates directory doesn't exist, but that's expected
        let _result = run(cli);
    }

    #[test]
    fn test_run_create_not_implemented() {
        let cli = Cli::parse_from(["xzagentz", "create"]);
        let result = run(cli);
        assert!(result.is_err());
        if let Err(Error::NotImplemented(_)) = result {
            // Expected
        } else {
            panic!("Expected NotImplemented error");
        }
    }
}
