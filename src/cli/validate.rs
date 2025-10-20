//! Validate command implementation
//!
//! This module implements the validate command for checking
//! AGENTS.md files for compliance with project standards.

use crate::error::Error;
use regex::Regex;
use serde::Serialize;
use std::fmt;
use std::fs;
use std::path::Path;

use super::output;
use super::OutputFormat;

/// Validation issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Error level issue (must fix)
    Error,
    /// Warning level issue (should fix)
    Warning,
    /// Info level (suggestion)
    Info,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "ERROR"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

/// A single validation issue
#[derive(Debug, Serialize)]
pub struct ValidationIssue {
    /// Severity level
    pub severity: Severity,
    /// Line number (if applicable)
    pub line: Option<usize>,
    /// Issue description
    pub message: String,
    /// Suggested fix (if available)
    pub suggestion: Option<String>,
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(line) = self.line {
            write!(f, "[{}] Line {}: {}", self.severity, line, self.message)?;
        } else {
            write!(f, "[{}] {}", self.severity, self.message)?;
        }
        if let Some(ref suggestion) = self.suggestion {
            write!(f, "\n  Suggestion: {}", suggestion)?;
        }
        Ok(())
    }
}

/// Validation report
#[derive(Debug, Serialize)]
pub struct ValidationReport {
    /// File path validated
    pub file: String,
    /// Total issues found
    pub total_issues: usize,
    /// Number of errors
    pub errors: usize,
    /// Number of warnings
    pub warnings: usize,
    /// Number of info messages
    pub infos: usize,
    /// List of issues
    pub issues: Vec<ValidationIssue>,
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Validation Report: {}", self.file)?;
        writeln!(f, "{}", "=".repeat(60))?;
        writeln!(
            f,
            "Total: {} (Errors: {}, Warnings: {}, Info: {})",
            self.total_issues, self.errors, self.warnings, self.infos
        )?;
        writeln!(f)?;

        if self.issues.is_empty() {
            writeln!(f, "No issues found. File is valid.")?;
        } else {
            writeln!(f, "Issues:")?;
            for issue in &self.issues {
                writeln!(f, "  {}", issue)?;
            }
        }

        Ok(())
    }
}

impl ValidationReport {
    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors == 0
    }
}

/// Execute the validate command
///
/// # Arguments
///
/// * `file` - Path to file to validate
/// * `detailed` - Show detailed report
/// * `fix` - Attempt to fix issues automatically
/// * `format` - Output format
/// * `verbose` - Enable verbose output
///
/// # Errors
///
/// Returns an error if validation cannot be performed
pub fn execute(
    file: &Path,
    detailed: bool,
    fix: bool,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Error> {
    output::print_verbose(verbose, &format!("Validating file: {:?}", file));

    if fix {
        output::print_warning("Auto-fix is not yet implemented");
    }

    let report = validate_file(file, detailed)?;

    output::print_output(format, &report)?;

    if !report.is_valid() {
        return Err(Error::ValidationError(format!(
            "Validation failed with {} errors",
            report.errors
        )));
    }

    Ok(())
}

/// Validate a file and generate a report
fn validate_file(file: &Path, detailed: bool) -> Result<ValidationReport, Error> {
    if !file.exists() {
        return Err(Error::component_not_found(format!(
            "File not found: {:?}",
            file
        )));
    }

    let content = fs::read_to_string(file)
        .map_err(|e| Error::ComponentLoadError(format!("Failed to read file: {}", e)))?;

    let mut issues = Vec::new();

    // Check for emojis
    issues.extend(check_emojis(&content));

    // Check for uppercase filenames in markdown links
    issues.extend(check_filename_case(file, &content));

    // Check for .yml extensions
    issues.extend(check_yaml_extensions(&content));

    // Check for missing code block languages
    issues.extend(check_code_blocks(&content));

    // Only include detailed checks if requested
    if detailed {
        issues.extend(check_section_structure(&content));
    }

    let errors = issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .count();
    let warnings = issues
        .iter()
        .filter(|i| i.severity == Severity::Warning)
        .count();
    let infos = issues
        .iter()
        .filter(|i| i.severity == Severity::Info)
        .count();

    Ok(ValidationReport {
        file: file.to_string_lossy().to_string(),
        total_issues: issues.len(),
        errors,
        warnings,
        infos,
        issues,
    })
}

/// Check for emoji usage
fn check_emojis(content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let emoji_regex = Regex::new(r"[\u{1F600}-\u{1F64F}\u{1F300}-\u{1F5FF}\u{1F680}-\u{1F6FF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}]").unwrap();

    for (line_num, line) in content.lines().enumerate() {
        if emoji_regex.is_match(line) {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                line: Some(line_num + 1),
                message: "Emoji usage detected (not allowed per AGENTS.md rules)".to_string(),
                suggestion: Some("Remove all emojis from documentation".to_string()),
            });
        }
    }

    issues
}

/// Check for filename case issues
fn check_filename_case(_file: &Path, content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^\)]+\.md)\)").unwrap();

    for (line_num, line) in content.lines().enumerate() {
        for cap in link_regex.captures_iter(line) {
            if let Some(path) = cap.get(2) {
                let path_str = path.as_str();
                // Check if filename has uppercase (except README.md)
                if path_str.contains(char::is_uppercase) && !path_str.contains("README.md") {
                    issues.push(ValidationIssue {
                        severity: Severity::Error,
                        line: Some(line_num + 1),
                        message: format!("Markdown file uses uppercase: {}", path_str),
                        suggestion: Some(
                            "Use lowercase with underscores (e.g., my_file.md)".to_string(),
                        ),
                    });
                }
                // Check for kebab-case
                if path_str.contains('-') && path_str.ends_with(".md") {
                    issues.push(ValidationIssue {
                        severity: Severity::Warning,
                        line: Some(line_num + 1),
                        message: format!("Markdown file uses kebab-case: {}", path_str),
                        suggestion: Some("Use snake_case instead (e.g., my_file.md)".to_string()),
                    });
                }
            }
        }
    }

    issues
}

/// Check for .yml extensions (should be .yaml)
fn check_yaml_extensions(content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if line.contains(".yml") && !line.trim().starts_with("//") && !line.trim().starts_with('#')
        {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                line: Some(line_num + 1),
                message: "Reference to .yml file (should use .yaml extension)".to_string(),
                suggestion: Some("Change .yml to .yaml".to_string()),
            });
        }
    }

    issues
}

/// Check code blocks have language identifiers
fn check_code_blocks(content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let code_block_regex = Regex::new(r"^```\s*$").unwrap();

    for (line_num, line) in content.lines().enumerate() {
        if code_block_regex.is_match(line) {
            issues.push(ValidationIssue {
                severity: Severity::Warning,
                line: Some(line_num + 1),
                message: "Code block missing language identifier".to_string(),
                suggestion: Some("Add language after backticks (e.g., ```rust)".to_string()),
            });
        }
    }

    issues
}

/// Check section structure
fn check_section_structure(content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // Check for required sections
    let required_sections = vec![
        "Quick Reference",
        "CRITICAL RULES",
        "Development Workflow",
        "Validation Checklist",
    ];

    for section in required_sections {
        if !content.contains(section) {
            issues.push(ValidationIssue {
                severity: Severity::Info,
                line: None,
                message: format!("Missing recommended section: {}", section),
                suggestion: Some(format!("Consider adding a '{}' section", section)),
            });
        }
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_valid_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "# Test\n\nValid content.").unwrap();

        let report = validate_file(&file_path, false);
        assert!(report.is_ok());
    }

    #[test]
    fn test_validate_missing_file() {
        let file_path = Path::new("nonexistent.md");
        let report = validate_file(file_path, false);
        assert!(report.is_err());
    }

    #[test]
    fn test_check_emojis() {
        let content = "This has an emoji: 😀";
        let issues = check_emojis(content);
        assert!(!issues.is_empty());
        assert_eq!(issues[0].severity, Severity::Error);
    }

    #[test]
    fn test_check_yaml_extensions() {
        let content = "config.yml is wrong\nconfig.yaml is correct";
        let issues = check_yaml_extensions(content);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains(".yml"));
    }

    #[test]
    fn test_check_code_blocks() {
        let content = "```\ncode without language\n```";
        let issues = check_code_blocks(content);
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_validation_report_display() {
        let report = ValidationReport {
            file: "test.md".to_string(),
            total_issues: 1,
            errors: 1,
            warnings: 0,
            infos: 0,
            issues: vec![ValidationIssue {
                severity: Severity::Error,
                line: Some(10),
                message: "Test error".to_string(),
                suggestion: None,
            }],
        };

        let display = format!("{}", report);
        assert!(display.contains("test.md"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_validation_report_is_valid() {
        let report = ValidationReport {
            file: "test.md".to_string(),
            total_issues: 1,
            errors: 0,
            warnings: 1,
            infos: 0,
            issues: vec![],
        };
        assert!(report.is_valid());

        let report_with_errors = ValidationReport {
            file: "test.md".to_string(),
            total_issues: 1,
            errors: 1,
            warnings: 0,
            infos: 0,
            issues: vec![],
        };
        assert!(!report_with_errors.is_valid());
    }
}
