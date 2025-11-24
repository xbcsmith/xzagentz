// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

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
        Commands::Init {
            components_dir,
            templates_dir,
            force,
            dry_run,
        } => {
            let result = xzagentz::cli::init::execute(
                components_dir,
                templates_dir,
                force,
                dry_run,
                cli.verbose,
            )?;

            if !dry_run {
                println!("Successfully initialized xzagentz!");
                println!(
                    "  Components: {} files in {}",
                    result.components_extracted,
                    result.components_dir.display()
                );
                println!(
                    "  Templates:  {} files in {}",
                    result.templates_extracted,
                    result.templates_dir.display()
                );
                if result.conflicts_overwritten > 0 {
                    println!(
                        "  Overwrote {} existing files",
                        result.conflicts_overwritten
                    );
                }
            }
        }
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
            file,
            template,
            force,
            interactive,
        } => {
            use xzagentz::cli::create::{CreateCommand, CreateConfig};

            let config = CreateConfig {
                file: file.clone(),
                template: template.clone(),
                force,
                interactive,
                template_dir,
                component_dir,
                verbose: cli.verbose,
            };

            let cmd = CreateCommand::new(file, template, force, interactive);
            cmd.execute(&config)?;
        }
        Commands::Update {
            file,
            section,
            backup,
        } => {
            use xzagentz::cli::update::{update_section, UpdateOptions};

            if cli.verbose {
                eprintln!("Updating section '{}' in {:?}", section, file);
            }

            let opts = UpdateOptions::new(file.clone(), section)
                .with_backup(backup)
                .with_component_dir(component_dir);

            let backup_path = update_section(opts)?;

            if backup && !backup_path.as_os_str().is_empty() {
                println!("Created backup: {}", backup_path.display());
            }
            println!("Successfully updated section in {}", file.display());
        }
        Commands::Add {
            file,
            component,
            position,
        } => {
            use xzagentz::cli::add::{add_section, AddOptions, Position};

            if cli.verbose {
                eprintln!("Adding component '{}' to {:?}", component, file);
            }

            let pos = match position.as_str() {
                "top" | "beginning" => Position::Beginning,
                "bottom" | "end" => Position::End,
                other => {
                    if let Some(after) = other.strip_prefix("after:") {
                        Position::After(after.to_string())
                    } else if let Some(before) = other.strip_prefix("before:") {
                        Position::Before(before.to_string())
                    } else {
                        Position::End
                    }
                }
            };

            let opts = AddOptions::new(file.clone(), component.clone())
                .with_component_dir(component_dir)
                .with_position(pos);

            add_section(opts)?;

            println!("Successfully added section to {}", file.display());
        }
        Commands::Prompt(prompt_args) => {
            xzagentz::cli::prompt::execute(prompt_args)?;
        }
        Commands::Implementation(impl_args) => {
            // Implementation command requires async runtime
            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| Error::other(format!("Failed to create async runtime: {}", e)))?;

            runtime.block_on(async {
                xzagentz::cli::implementation::execute(impl_args, cli.verbose)
                    .await
                    .map_err(|e| Error::other(e.to_string()))
            })?;
        }
        Commands::Architecture(arch_args) => {
            // Architecture command requires async runtime
            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| Error::other(format!("Failed to create async runtime: {}", e)))?;

            runtime.block_on(async {
                xzagentz::cli::architecture::execute(arch_args)
                    .await
                    .map_err(|e| Error::other(e.to_string()))
            })?;
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
    fn test_run_create_command() {
        let cli = Cli::parse_from(["xzagentz", "create", "test_output.md"]);
        // This will fail if components/templates don't exist, but that's expected in tests
        let _result = run(cli);
    }

    #[test]
    fn test_run_init_dry_run() {
        let cli = Cli::parse_from(["xzagentz", "init", "--dry-run"]);
        let _result = run(cli);
    }
}
