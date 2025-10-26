//! Size validation for components
//!
//! This module provides functionality for validating component sizes against
//! configured limits. It enforces size constraints per category and tier,
//! and provides detailed reporting on size violations.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::validator::size::{SizeValidator, SizeLimits};
//!
//! let limits = SizeLimits::default();
//! let validator = SizeValidator::new(limits);
//!
//! let result = validator.validate_content("core", None, "# Content\nLine 2\n");
//! assert!(result.is_ok());
//! ```

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Size limits configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SizeLimits {
    /// Maximum lines for core components
    pub core_max: usize,

    /// Maximum lines for general components
    pub general_max: usize,

    /// Maximum lines for language-specific components
    pub language_max: usize,

    /// Maximum lines for essential tool components
    pub tool_essential_max: usize,

    /// Maximum lines for comprehensive tool components
    pub tool_comprehensive_max: usize,

    /// Total maximum lines for complete rendered output
    pub total_max: usize,

    /// Warning threshold percentage (0.0-1.0)
    pub warn_threshold: f64,
}

impl Default for SizeLimits {
    fn default() -> Self {
        Self {
            core_max: 500,
            general_max: 800,
            language_max: 600,
            tool_essential_max: 300,
            tool_comprehensive_max: 800,
            total_max: 10000,
            warn_threshold: 0.8,
        }
    }
}

impl SizeLimits {
    /// Gets the size limit for a specific category and tier
    ///
    /// # Arguments
    ///
    /// * `category` - Component category (core, general, languages, tools)
    /// * `tier` - Optional tier (essential, comprehensive) for tools
    ///
    /// # Returns
    ///
    /// Returns the maximum line count for the given category/tier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::validator::size::SizeLimits;
    ///
    /// let limits = SizeLimits::default();
    /// assert_eq!(limits.get_limit("core", None), 500);
    /// assert_eq!(limits.get_limit("tools", Some("essential")), 300);
    /// assert_eq!(limits.get_limit("tools", Some("comprehensive")), 800);
    /// ```
    pub fn get_limit(&self, category: &str, tier: Option<&str>) -> usize {
        match category {
            "core" => self.core_max,
            "general" => self.general_max,
            "languages" => self.language_max,
            "tools" => match tier {
                Some("essential") => self.tool_essential_max,
                Some("comprehensive") => self.tool_comprehensive_max,
                _ => self.tool_comprehensive_max, // Default to comprehensive
            },
            _ => self.general_max, // Default to general limit
        }
    }

    /// Gets the warning threshold for a specific limit
    ///
    /// # Arguments
    ///
    /// * `limit` - The size limit
    ///
    /// # Returns
    ///
    /// Returns the warning threshold in lines
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::validator::size::SizeLimits;
    ///
    /// let limits = SizeLimits::default();
    /// assert_eq!(limits.warn_at(500), 400);
    /// ```
    pub fn warn_at(&self, limit: usize) -> usize {
        (limit as f64 * self.warn_threshold) as usize
    }
}

/// Size validation result
#[derive(Debug, Clone, PartialEq)]
pub struct SizeValidation {
    /// Component name or identifier
    pub name: String,

    /// Component category
    pub category: String,

    /// Component tier (if applicable)
    pub tier: Option<String>,

    /// Actual line count
    pub actual_lines: usize,

    /// Maximum allowed lines
    pub max_lines: usize,

    /// Warning threshold in lines
    pub warn_lines: usize,

    /// Whether the component is valid
    pub is_valid: bool,

    /// Whether the component exceeds warning threshold
    pub is_warning: bool,

    /// Percentage of limit used (0.0-1.0+)
    pub usage_ratio: f64,
}

impl SizeValidation {
    /// Creates a new size validation result
    pub fn new(
        name: String,
        category: String,
        tier: Option<String>,
        actual_lines: usize,
        max_lines: usize,
        warn_lines: usize,
    ) -> Self {
        let is_valid = actual_lines <= max_lines;
        let is_warning = actual_lines > warn_lines;
        let usage_ratio = actual_lines as f64 / max_lines as f64;

        Self {
            name,
            category,
            tier,
            actual_lines,
            max_lines,
            warn_lines,
            is_valid,
            is_warning,
            usage_ratio,
        }
    }

    /// Gets a human-readable status message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::validator::size::SizeValidation;
    ///
    /// let validation = SizeValidation::new(
    ///     "test".to_string(),
    ///     "core".to_string(),
    ///     None,
    ///     450,
    ///     500,
    ///     400,
    /// );
    ///
    /// let status = validation.status_message();
    /// assert!(status.contains("WARNING"));
    /// ```
    pub fn status_message(&self) -> String {
        if !self.is_valid {
            format!(
                "ERROR: {} ({}) exceeds limit: {} > {} lines ({:.1}%)",
                self.name,
                self.display_category(),
                self.actual_lines,
                self.max_lines,
                self.usage_ratio * 100.0
            )
        } else if self.is_warning {
            format!(
                "WARNING: {} ({}) approaching limit: {} / {} lines ({:.1}%)",
                self.name,
                self.display_category(),
                self.actual_lines,
                self.max_lines,
                self.usage_ratio * 100.0
            )
        } else {
            format!(
                "OK: {} ({}) within limit: {} / {} lines ({:.1}%)",
                self.name,
                self.display_category(),
                self.actual_lines,
                self.max_lines,
                self.usage_ratio * 100.0
            )
        }
    }

    /// Displays category with tier if applicable
    fn display_category(&self) -> String {
        if let Some(ref tier) = self.tier {
            format!("{}/{}", self.category, tier)
        } else {
            self.category.clone()
        }
    }
}

/// Size validator for components
pub struct SizeValidator {
    /// Size limits configuration
    limits: SizeLimits,
}

impl Default for SizeValidator {
    fn default() -> Self {
        Self::new(SizeLimits::default())
    }
}

impl SizeValidator {
    /// Creates a new size validator with the given limits
    ///
    /// # Arguments
    ///
    /// * `limits` - Size limits configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::validator::size::{SizeValidator, SizeLimits};
    ///
    /// let limits = SizeLimits::default();
    /// let validator = SizeValidator::new(limits);
    /// ```
    pub fn new(limits: SizeLimits) -> Self {
        Self { limits }
    }

    /// Validates a component's content against size limits
    ///
    /// # Arguments
    ///
    /// * `category` - Component category
    /// * `tier` - Optional tier for tools
    /// * `content` - Component content to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(SizeValidation)` with validation results
    ///
    /// # Errors
    ///
    /// Returns error if content is invalid or exceeds limits
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::validator::size::SizeValidator;
    ///
    /// let validator = SizeValidator::default();
    /// let content = "# Title\nContent line\n";
    ///
    /// let validation = validator.validate_content("core", None, content)?;
    /// assert!(validation.is_valid);
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn validate_content(
        &self,
        category: &str,
        tier: Option<&str>,
        content: &str,
    ) -> Result<SizeValidation> {
        let line_count = self.count_lines(content);
        let max_lines = self.limits.get_limit(category, tier);
        let warn_lines = self.limits.warn_at(max_lines);

        let validation = SizeValidation::new(
            "content".to_string(),
            category.to_string(),
            tier.map(String::from),
            line_count,
            max_lines,
            warn_lines,
        );

        if !validation.is_valid {
            return Err(Error::SizeExceeded {
                name: category.to_string(),
                actual: line_count,
                limit: max_lines,
            });
        }

        Ok(validation)
    }

    /// Validates a component file against size limits
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the component file
    /// * `category` - Component category
    /// * `tier` - Optional tier for tools
    ///
    /// # Returns
    ///
    /// Returns `Ok(SizeValidation)` with validation results
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be read or exceeds limits
    pub fn validate_file<P: AsRef<Path>>(
        &self,
        path: P,
        category: &str,
        tier: Option<&str>,
    ) -> Result<SizeValidation> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(|e| {
            Error::io_error(format!("Failed to read file '{}': {}", path.display(), e))
        })?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let line_count = self.count_lines(&content);
        let max_lines = self.limits.get_limit(category, tier);
        let warn_lines = self.limits.warn_at(max_lines);

        let validation = SizeValidation::new(
            name.clone(),
            category.to_string(),
            tier.map(String::from),
            line_count,
            max_lines,
            warn_lines,
        );

        if !validation.is_valid {
            return Err(Error::SizeExceeded {
                name,
                actual: line_count,
                limit: max_lines,
            });
        }

        Ok(validation)
    }

    /// Counts non-empty lines in content
    ///
    /// Ignores blank lines and lines with only whitespace
    ///
    /// # Arguments
    ///
    /// * `content` - Content to count lines in
    ///
    /// # Returns
    ///
    /// Returns the number of non-empty lines
    pub fn count_lines(&self, content: &str) -> usize {
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()
    }

    /// Validates total size across multiple components
    ///
    /// # Arguments
    ///
    /// * `total_lines` - Total line count across all components
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if within total limit
    ///
    /// # Errors
    ///
    /// Returns error if total exceeds the configured maximum
    pub fn validate_total(&self, total_lines: usize) -> Result<()> {
        if total_lines > self.limits.total_max {
            return Err(Error::SizeExceeded {
                name: "total".to_string(),
                actual: total_lines,
                limit: self.limits.total_max,
            });
        }
        Ok(())
    }

    /// Gets the configured size limits
    pub fn limits(&self) -> &SizeLimits {
        &self.limits
    }
}

/// Size report for a collection of components
#[derive(Debug, Clone)]
pub struct SizeReport {
    /// Individual validation results
    pub validations: Vec<SizeValidation>,

    /// Total lines across all components
    pub total_lines: usize,

    /// Total size limit
    pub total_limit: usize,

    /// Number of components with errors
    pub error_count: usize,

    /// Number of components with warnings
    pub warning_count: usize,

    /// Whether the entire report is valid
    pub is_valid: bool,
}

impl SizeReport {
    /// Creates a new size report
    pub fn new(validations: Vec<SizeValidation>, total_limit: usize) -> Self {
        let total_lines = validations.iter().map(|v| v.actual_lines).sum();
        let error_count = validations.iter().filter(|v| !v.is_valid).count();
        let warning_count = validations
            .iter()
            .filter(|v| v.is_valid && v.is_warning)
            .count();
        let is_valid = error_count == 0 && total_lines <= total_limit;

        Self {
            validations,
            total_lines,
            total_limit,
            error_count,
            warning_count,
            is_valid,
        }
    }

    /// Generates a formatted report string
    pub fn format_report(&self) -> String {
        let mut report = String::new();

        report.push_str("Component Size Report\n");
        report.push_str("====================\n\n");

        for validation in &self.validations {
            report.push_str(&format!("{}\n", validation.status_message()));
        }

        report.push_str("\nSummary\n");
        report.push_str("-------\n");
        report.push_str(&format!(
            "Total lines: {} / {}\n",
            self.total_lines, self.total_limit
        ));
        report.push_str(&format!("Components: {}\n", self.validations.len()));
        report.push_str(&format!("Errors: {}\n", self.error_count));
        report.push_str(&format!("Warnings: {}\n", self.warning_count));
        report.push_str(&format!(
            "Status: {}\n",
            if self.is_valid { "PASS" } else { "FAIL" }
        ));

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_limits_default() {
        let limits = SizeLimits::default();
        assert_eq!(limits.core_max, 500);
        assert_eq!(limits.general_max, 800);
        assert_eq!(limits.tool_essential_max, 300);
        assert_eq!(limits.tool_comprehensive_max, 800);
    }

    #[test]
    fn test_get_limit_for_category() {
        let limits = SizeLimits::default();
        assert_eq!(limits.get_limit("core", None), 500);
        assert_eq!(limits.get_limit("general", None), 800);
        assert_eq!(limits.get_limit("languages", None), 600);
        assert_eq!(limits.get_limit("tools", Some("essential")), 300);
        assert_eq!(limits.get_limit("tools", Some("comprehensive")), 800);
    }

    #[test]
    fn test_warn_at_threshold() {
        let limits = SizeLimits::default();
        assert_eq!(limits.warn_at(500), 400);
        assert_eq!(limits.warn_at(300), 240);
    }

    #[test]
    fn test_size_validation_ok() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 250, 500, 400);

        assert!(validation.is_valid);
        assert!(!validation.is_warning);
        assert_eq!(validation.usage_ratio, 0.5);
    }

    #[test]
    fn test_size_validation_warning() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 450, 500, 400);

        assert!(validation.is_valid);
        assert!(validation.is_warning);
    }

    #[test]
    fn test_size_validation_error() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 600, 500, 400);

        assert!(!validation.is_valid);
        assert!(validation.is_warning);
        assert!(validation.usage_ratio > 1.0);
    }

    #[test]
    fn test_status_message_ok() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 250, 500, 400);

        let message = validation.status_message();
        assert!(message.contains("OK"));
        assert!(message.contains("250 / 500"));
    }

    #[test]
    fn test_status_message_warning() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 450, 500, 400);

        let message = validation.status_message();
        assert!(message.contains("WARNING"));
    }

    #[test]
    fn test_status_message_error() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 600, 500, 400);

        let message = validation.status_message();
        assert!(message.contains("ERROR"));
        assert!(message.contains("exceeds limit"));
    }

    #[test]
    fn test_display_category_with_tier() {
        let validation = SizeValidation::new(
            "test".to_string(),
            "tools".to_string(),
            Some("essential".to_string()),
            200,
            300,
            240,
        );

        assert_eq!(validation.display_category(), "tools/essential");
    }

    #[test]
    fn test_display_category_without_tier() {
        let validation =
            SizeValidation::new("test".to_string(), "core".to_string(), None, 250, 500, 400);

        assert_eq!(validation.display_category(), "core");
    }

    #[test]
    fn test_size_validator_default() {
        let validator = SizeValidator::default();
        assert_eq!(validator.limits().core_max, 500);
    }

    #[test]
    fn test_size_validator_implements_default_trait() {
        let validator: SizeValidator = Default::default();
        assert_eq!(validator.limits().core_max, 500);
    }

    #[test]
    fn test_count_lines_non_empty() {
        let validator = SizeValidator::default();
        let content = "Line 1\nLine 2\n\nLine 3\n  \nLine 4";
        assert_eq!(validator.count_lines(content), 4);
    }

    #[test]
    fn test_count_lines_empty() {
        let validator = SizeValidator::default();
        assert_eq!(validator.count_lines(""), 0);
        assert_eq!(validator.count_lines("\n\n\n"), 0);
        assert_eq!(validator.count_lines("   \n  \n  "), 0);
    }

    #[test]
    fn test_validate_content_success() {
        let validator = SizeValidator::default();
        let content = (0..250)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let result = validator.validate_content("core", None, &content);
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.actual_lines, 250);
    }

    #[test]
    fn test_validate_content_warning() {
        let validator = SizeValidator::default();
        let content = (0..450)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let result = validator.validate_content("core", None, &content);
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert!(validation.is_valid);
        assert!(validation.is_warning);
    }

    #[test]
    fn test_validate_content_error() {
        let validator = SizeValidator::default();
        let content = (0..600)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let result = validator.validate_content("core", None, &content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tool_essential() {
        let validator = SizeValidator::default();
        let content = (0..250)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let result = validator.validate_content("tools", Some("essential"), &content);
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.max_lines, 300);
    }

    #[test]
    fn test_validate_tool_comprehensive() {
        let validator = SizeValidator::default();
        let content = (0..700)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let result = validator.validate_content("tools", Some("comprehensive"), &content);
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.max_lines, 800);
    }

    #[test]
    fn test_validate_total_success() {
        let validator = SizeValidator::default();
        let result = validator.validate_total(5000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_total_error() {
        let validator = SizeValidator::default();
        let result = validator.validate_total(15000);
        assert!(result.is_err());
    }

    #[test]
    fn test_size_report_creation() {
        let validations = vec![
            SizeValidation::new("comp1".to_string(), "core".to_string(), None, 250, 500, 400),
            SizeValidation::new(
                "comp2".to_string(),
                "general".to_string(),
                None,
                450,
                800,
                640,
            ),
            SizeValidation::new("comp3".to_string(), "core".to_string(), None, 600, 500, 400),
        ];

        let report = SizeReport::new(validations, 10000);
        assert_eq!(report.total_lines, 1300);
        assert_eq!(report.error_count, 1);
        assert_eq!(report.warning_count, 0); // comp2 is 450/800 with warn at 640, so no warning
        assert!(!report.is_valid);
    }

    #[test]
    fn test_size_report_format() {
        let validations = vec![SizeValidation::new(
            "comp1".to_string(),
            "core".to_string(),
            None,
            250,
            500,
            400,
        )];

        let report = SizeReport::new(validations, 10000);
        let formatted = report.format_report();

        assert!(formatted.contains("Component Size Report"));
        assert!(formatted.contains("Summary"));
        assert!(formatted.contains("Total lines: 250 / 10000"));
    }
}
