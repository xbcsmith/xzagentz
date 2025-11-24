// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! CLI init command implementation
//!
//! This module implements the `xzagentz init` command which extracts embedded
//! resources (components and templates) to the filesystem for customization.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::cli::init::{InitCommand, InitConfig};
//! use std::path::PathBuf;
//!
//! let config = InitConfig {
//!     components_dir: Some(PathBuf::from("~/.config/xzagentz/components")),
//!     templates_dir: Some(PathBuf::from("~/.config/xzagentz/templates")),
//!     force: false,
//!     dry_run: false,
//!     verbose: false,
//! };
//!
//! let cmd = InitCommand::new(config);
//! cmd.execute().unwrap();
//! ```

use crate::components::resolution::default_components_dir;
use crate::error::{Error, Result};
use crate::infrastructure::embedded::EmbeddedResources;
use crate::templates::resolution::default_templates_dir;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for the init command
#[derive(Debug, Clone, Default)]
pub struct InitConfig {
    /// Target directory for components (None = use default)
    pub components_dir: Option<PathBuf>,
    /// Target directory for templates (None = use default)
    pub templates_dir: Option<PathBuf>,
    /// Force overwrite of existing files
    pub force: bool,
    /// Dry run - show what would be done without doing it
    pub dry_run: bool,
    /// Verbose output
    pub verbose: bool,
}

/// Init command executor
#[derive(Debug)]
pub struct InitCommand {
    config: InitConfig,
    resources: EmbeddedResources,
}

impl InitCommand {
    /// Creates a new InitCommand with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the init operation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::cli::init::{InitCommand, InitConfig};
    ///
    /// let config = InitConfig::default();
    /// let cmd = InitCommand::new(config);
    /// ```
    pub fn new(config: InitConfig) -> Self {
        Self {
            config,
            resources: EmbeddedResources::new(),
        }
    }

    /// Executes the init command
    ///
    /// # Returns
    ///
    /// Returns `Ok(InitResult)` with extraction statistics, or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Directories cannot be created
    /// - Files cannot be written
    /// - File conflicts exist and force is not enabled
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::cli::init::{InitCommand, InitConfig};
    ///
    /// let config = InitConfig::default();
    /// let cmd = InitCommand::new(config);
    /// let result = cmd.execute().unwrap();
    /// println!("Extracted {} components and {} templates",
    ///          result.components_extracted, result.templates_extracted);
    /// ```
    pub fn execute(&self) -> Result<InitResult> {
        // Resolve directories with proper precedence: CLI args > env vars > defaults
        let components_dir = self
            .config
            .components_dir
            .clone()
            .or_else(|| {
                std::env::var("XZAGENTZ_COMPONENT_DIR")
                    .ok()
                    .map(PathBuf::from)
            })
            .unwrap_or_else(default_components_dir);

        let templates_dir = self
            .config
            .templates_dir
            .clone()
            .or_else(|| {
                std::env::var("XZAGENTZ_TEMPLATE_DIR")
                    .ok()
                    .map(PathBuf::from)
            })
            .unwrap_or_else(default_templates_dir);

        if self.config.verbose {
            eprintln!("Initializing xzagentz resources...");
            eprintln!("  Components directory: {}", components_dir.display());
            eprintln!("  Templates directory:  {}", templates_dir.display());
            eprintln!("  Force overwrite:      {}", self.config.force);
            eprintln!("  Dry run:              {}", self.config.dry_run);
        }

        // In dry-run mode, skip conflict checking and just show what would be done
        if self.config.dry_run {
            return self.dry_run(&components_dir, &templates_dir, &Vec::new());
        }

        // Check for conflicts before extraction (only when not in dry-run mode)
        let conflicts = self.check_conflicts(&components_dir, &templates_dir)?;

        if !conflicts.is_empty() && !self.config.force {
            return Err(Error::other(format!(
                "Found {} existing files. Use --force to overwrite:\n{}",
                conflicts.len(),
                conflicts.join("\n")
            )));
        }

        // Extract components
        let components_extracted = self.extract_components(&components_dir)?;

        // Extract templates
        let templates_extracted = self.extract_templates(&templates_dir)?;

        if self.config.verbose {
            eprintln!("\nSuccess!");
            eprintln!(
                "  Extracted {} components to {}",
                components_extracted,
                components_dir.display()
            );
            eprintln!(
                "  Extracted {} templates to {}",
                templates_extracted,
                templates_dir.display()
            );
        }

        Ok(InitResult {
            components_dir,
            templates_dir,
            components_extracted,
            templates_extracted,
            conflicts_overwritten: if self.config.force {
                conflicts.len()
            } else {
                0
            },
        })
    }

    /// Checks for existing files that would conflict with extraction
    ///
    /// # Arguments
    ///
    /// * `components_dir` - Target components directory
    /// * `templates_dir` - Target templates directory
    ///
    /// # Returns
    ///
    /// Returns a vector of conflicting file paths
    fn check_conflicts(&self, components_dir: &Path, templates_dir: &Path) -> Result<Vec<String>> {
        let mut conflicts = Vec::new();

        // Check component conflicts
        if components_dir.exists() {
            for component_path in self.resources.list_components() {
                let target = components_dir.join(&component_path);
                if target.exists() {
                    conflicts.push(format!("  components/{}", component_path));
                }
            }
        }

        // Check template conflicts
        if templates_dir.exists() {
            for template_path in self.resources.list_templates() {
                let target = templates_dir.join(&template_path);
                if target.exists() {
                    conflicts.push(format!("  templates/{}", template_path));
                }
            }
        }

        Ok(conflicts)
    }

    /// Performs a dry run, showing what would be extracted
    ///
    /// # Arguments
    ///
    /// * `components_dir` - Target components directory
    /// * `templates_dir` - Target templates directory
    /// * `conflicts` - List of conflicting files
    fn dry_run(
        &self,
        components_dir: &Path,
        templates_dir: &Path,
        conflicts: &[String],
    ) -> Result<InitResult> {
        let components = self.resources.list_components();
        let templates = self.resources.list_templates();

        println!("Dry run - would perform the following actions:");
        println!("\nComponents directory: {}", components_dir.display());
        if !components_dir.exists() {
            println!("  [CREATE] {}", components_dir.display());
        }
        for component in &components {
            let target = components_dir.join(component);
            if target.exists() {
                if self.config.force {
                    println!("  [OVERWRITE] {}", component);
                } else {
                    println!("  [SKIP] {} (exists)", component);
                }
            } else {
                println!("  [EXTRACT] {}", component);
            }
        }

        println!("\nTemplates directory: {}", templates_dir.display());
        if !templates_dir.exists() {
            println!("  [CREATE] {}", templates_dir.display());
        }
        for template in &templates {
            let target = templates_dir.join(template);
            if target.exists() {
                if self.config.force {
                    println!("  [OVERWRITE] {}", template);
                } else {
                    println!("  [SKIP] {} (exists)", template);
                }
            } else {
                println!("  [EXTRACT] {}", template);
            }
        }

        if !conflicts.is_empty() && !self.config.force {
            println!(
                "\nWould skip {} existing files (use --force to overwrite)",
                conflicts.len()
            );
        }

        Ok(InitResult {
            components_dir: components_dir.to_path_buf(),
            templates_dir: templates_dir.to_path_buf(),
            components_extracted: components.len(),
            templates_extracted: templates.len(),
            conflicts_overwritten: 0,
        })
    }

    /// Extracts components to the target directory
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Directory where components will be extracted
    ///
    /// # Returns
    ///
    /// Returns the number of components extracted
    fn extract_components(&self, target_dir: &Path) -> Result<usize> {
        if self.config.verbose {
            eprintln!("\nExtracting components to {}...", target_dir.display());
        }

        // Create directory if it doesn't exist
        if !target_dir.exists() {
            fs::create_dir_all(target_dir).map_err(|e| {
                Error::other(format!(
                    "Failed to create components directory '{}': {}",
                    target_dir.display(),
                    e
                ))
            })?;
        }

        let count = self.resources.extract_components_to(target_dir)?;

        if self.config.verbose {
            eprintln!("  Extracted {} component files", count);
        }

        Ok(count)
    }

    /// Extracts templates to the target directory
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Directory where templates will be extracted
    ///
    /// # Returns
    ///
    /// Returns the number of templates extracted
    fn extract_templates(&self, target_dir: &Path) -> Result<usize> {
        if self.config.verbose {
            eprintln!("\nExtracting templates to {}...", target_dir.display());
        }

        // Create directory if it doesn't exist
        if !target_dir.exists() {
            fs::create_dir_all(target_dir).map_err(|e| {
                Error::other(format!(
                    "Failed to create templates directory '{}': {}",
                    target_dir.display(),
                    e
                ))
            })?;
        }

        let count = self.resources.extract_templates_to(target_dir)?;

        if self.config.verbose {
            eprintln!("  Extracted {} template files", count);
        }

        Ok(count)
    }
}

/// Result of the init command execution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitResult {
    /// Directory where components were extracted
    pub components_dir: PathBuf,
    /// Directory where templates were extracted
    pub templates_dir: PathBuf,
    /// Number of components extracted
    pub components_extracted: usize,
    /// Number of templates extracted
    pub templates_extracted: usize,
    /// Number of conflicts that were overwritten
    pub conflicts_overwritten: usize,
}

impl InitResult {
    /// Returns true if any resources were extracted
    pub fn has_extractions(&self) -> bool {
        self.components_extracted > 0 || self.templates_extracted > 0
    }

    /// Returns the total number of files extracted
    pub fn total_extracted(&self) -> usize {
        self.components_extracted + self.templates_extracted
    }
}

/// Executes the init command with the given configuration
///
/// This is a convenience function for the CLI command handler.
///
/// # Arguments
///
/// * `components_dir` - Optional custom components directory
/// * `templates_dir` - Optional custom templates directory
/// * `force` - Force overwrite of existing files
/// * `dry_run` - Perform a dry run without extracting
/// * `verbose` - Enable verbose output
///
/// # Returns
///
/// Returns `Ok(InitResult)` with extraction statistics, or an error.
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::cli::init::execute;
/// use std::path::PathBuf;
///
/// let result = execute(None, None, false, false, true).unwrap();
/// println!("Extracted {} total files", result.total_extracted());
/// ```
pub fn execute(
    components_dir: Option<PathBuf>,
    templates_dir: Option<PathBuf>,
    force: bool,
    dry_run: bool,
    verbose: bool,
) -> Result<InitResult> {
    let config = InitConfig {
        components_dir,
        templates_dir,
        force,
        dry_run,
        verbose,
    };

    let cmd = InitCommand::new(config);
    cmd.execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_config_default() {
        let config = InitConfig::default();
        assert!(config.components_dir.is_none());
        assert!(config.templates_dir.is_none());
        assert!(!config.force);
        assert!(!config.dry_run);
        assert!(!config.verbose);
    }

    #[test]
    fn test_init_command_new() {
        let config = InitConfig::default();
        let cmd = InitCommand::new(config);
        assert!(!cmd.config.force);
    }

    #[test]
    fn test_execute_with_temp_dirs() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        let config = InitConfig {
            components_dir: Some(components_dir.clone()),
            templates_dir: Some(templates_dir.clone()),
            force: false,
            dry_run: false,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let result = cmd.execute().unwrap();

        assert!(result.components_extracted > 0);
        assert!(result.templates_extracted > 0);
        assert!(components_dir.exists());
        assert!(templates_dir.exists());
        assert_eq!(result.conflicts_overwritten, 0);
    }

    #[test]
    fn test_execute_dry_run() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        let config = InitConfig {
            components_dir: Some(components_dir.clone()),
            templates_dir: Some(templates_dir.clone()),
            force: false,
            dry_run: true,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let result = cmd.execute().unwrap();

        assert!(result.components_extracted > 0);
        assert!(result.templates_extracted > 0);
        assert!(!components_dir.exists());
        assert!(!templates_dir.exists());
    }

    #[test]
    fn test_execute_with_conflicts_no_force() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        fs::create_dir_all(&components_dir).unwrap();
        fs::create_dir_all(&templates_dir).unwrap();

        let resources = EmbeddedResources::new();
        resources.extract_components_to(&components_dir).unwrap();
        resources.extract_templates_to(&templates_dir).unwrap();

        let config = InitConfig {
            components_dir: Some(components_dir.clone()),
            templates_dir: Some(templates_dir.clone()),
            force: false,
            dry_run: false,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let result = cmd.execute();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Use --force"));
    }

    #[test]
    fn test_execute_with_conflicts_force() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        fs::create_dir_all(&components_dir).unwrap();
        fs::create_dir_all(&templates_dir).unwrap();

        let resources = EmbeddedResources::new();
        resources.extract_components_to(&components_dir).unwrap();
        resources.extract_templates_to(&templates_dir).unwrap();

        let config = InitConfig {
            components_dir: Some(components_dir.clone()),
            templates_dir: Some(templates_dir.clone()),
            force: true,
            dry_run: false,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let result = cmd.execute().unwrap();

        assert!(result.components_extracted > 0);
        assert!(result.templates_extracted > 0);
        assert!(result.conflicts_overwritten > 0);
    }

    #[test]
    fn test_check_conflicts_empty() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        let config = InitConfig::default();
        let cmd = InitCommand::new(config);

        let conflicts = cmd
            .check_conflicts(&components_dir, &templates_dir)
            .unwrap();
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_check_conflicts_with_existing() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        fs::create_dir_all(&components_dir).unwrap();
        fs::create_dir_all(&templates_dir).unwrap();

        let resources = EmbeddedResources::new();
        resources.extract_components_to(&components_dir).unwrap();
        resources.extract_templates_to(&templates_dir).unwrap();

        let config = InitConfig::default();
        let cmd = InitCommand::new(config);

        let conflicts = cmd
            .check_conflicts(&components_dir, &templates_dir)
            .unwrap();
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_init_result_has_extractions() {
        let result = InitResult {
            components_dir: PathBuf::from("/tmp/components"),
            templates_dir: PathBuf::from("/tmp/templates"),
            components_extracted: 5,
            templates_extracted: 3,
            conflicts_overwritten: 0,
        };

        assert!(result.has_extractions());
        assert_eq!(result.total_extracted(), 8);
    }

    #[test]
    fn test_init_result_no_extractions() {
        let result = InitResult {
            components_dir: PathBuf::from("/tmp/components"),
            templates_dir: PathBuf::from("/tmp/templates"),
            components_extracted: 0,
            templates_extracted: 0,
            conflicts_overwritten: 0,
        };

        assert!(!result.has_extractions());
        assert_eq!(result.total_extracted(), 0);
    }

    #[test]
    fn test_execute_function() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("components");
        let templates_dir = temp.path().join("templates");

        let result = execute(
            Some(components_dir.clone()),
            Some(templates_dir.clone()),
            false,
            false,
            false,
        )
        .unwrap();

        assert!(result.components_extracted > 0);
        assert!(result.templates_extracted > 0);
        assert!(components_dir.exists());
        assert!(templates_dir.exists());
    }

    #[test]
    fn test_execute_uses_defaults() {
        let temp = TempDir::new().unwrap();

        std::env::set_var("HOME", temp.path().to_str().unwrap());

        let config = InitConfig {
            components_dir: None,
            templates_dir: None,
            force: false,
            dry_run: true,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let result = cmd.execute().unwrap();

        assert!(result.has_extractions());
        assert!(result.components_dir.to_string_lossy().contains(".config"));
        assert!(result.templates_dir.to_string_lossy().contains(".config"));
    }

    #[test]
    fn test_extract_components_creates_directory() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("new_components");

        assert!(!components_dir.exists());

        let config = InitConfig {
            components_dir: Some(components_dir.clone()),
            templates_dir: None,
            force: false,
            dry_run: false,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let count = cmd.extract_components(&components_dir).unwrap();

        assert!(components_dir.exists());
        assert!(count > 0);
    }

    #[test]
    fn test_extract_templates_creates_directory() {
        let temp = TempDir::new().unwrap();
        let templates_dir = temp.path().join("new_templates");

        assert!(!templates_dir.exists());

        let config = InitConfig {
            components_dir: None,
            templates_dir: Some(templates_dir.clone()),
            force: false,
            dry_run: false,
            verbose: false,
        };

        let cmd = InitCommand::new(config);
        let count = cmd.extract_templates(&templates_dir).unwrap();

        assert!(templates_dir.exists());
        assert!(count > 0);
    }
}
