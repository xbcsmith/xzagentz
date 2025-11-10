// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Component validator for structure and content validation
//!
//! This module provides the `ComponentValidator` which validates component
//! structure, markdown formatting, code block language identifiers, and
//! detects common issues like emoji usage.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::{Component, ComponentValidator};
//! use xzagentz::core::ComponentType;
//!
//! let component = Component::new(
//!     "test",
//!     ComponentType::Core,
//!     "# Test Component\n\n```rust\nfn main() {}\n```"
//! );
//!
//! let validator = ComponentValidator::new();
//! let result = validator.validate(&component);
//! ```

use crate::components::Component;
use crate::error::{Error, Result};
use regex::Regex;
use std::sync::OnceLock;

/// Validation result with details about issues found
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    /// Whether the component is valid
    pub valid: bool,

    /// List of errors found
    pub errors: Vec<String>,

    /// List of warnings found
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Creates a new ValidationResult
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Adds an error to the result
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.valid = false;
    }

    /// Adds a warning to the result
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Returns true if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns true if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Component validator
///
/// Validates component structure, content, and formatting according
/// to project standards defined in AGENTS.md.
#[derive(Debug, Default)]
pub struct ComponentValidator {
    /// Whether to check for emoji usage
    check_emojis: bool,

    /// Whether to check code block language identifiers
    check_code_blocks: bool,

    /// Whether to check placeholder format
    check_placeholders: bool,
}

impl ComponentValidator {
    /// Creates a new ComponentValidator with all checks enabled
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::ComponentValidator;
    ///
    /// let validator = ComponentValidator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            check_emojis: true,
            check_code_blocks: true,
            check_placeholders: true,
        }
    }

    /// Creates a validator with custom settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::ComponentValidator;
    ///
    /// let validator = ComponentValidator::with_settings(true, false, true);
    /// ```
    pub fn with_settings(
        check_emojis: bool,
        check_code_blocks: bool,
        check_placeholders: bool,
    ) -> Self {
        Self {
            check_emojis,
            check_code_blocks,
            check_placeholders,
        }
    }

    /// Validates a component and returns the validation result
    ///
    /// # Arguments
    ///
    /// * `component` - The component to validate
    ///
    /// # Returns
    ///
    /// Returns a `ValidationResult` with details about any issues found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::{Component, ComponentValidator};
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = Component::new(
    ///     "test",
    ///     ComponentType::Core,
    ///     "# Test\n\n```rust\nfn main() {}\n```"
    /// );
    ///
    /// let validator = ComponentValidator::new();
    /// let result = validator.validate(&component);
    /// assert!(result.valid);
    /// ```
    pub fn validate(&self, component: &Component) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Check if content is empty
        if component.is_empty() {
            result.add_error("Component content is empty");
            return result;
        }

        // Validate structure
        self.validate_structure(component, &mut result);

        // Check for emojis
        if self.check_emojis {
            self.detect_emojis(component, &mut result);
        }

        // Check code blocks
        if self.check_code_blocks {
            self.validate_code_blocks(component, &mut result);
        }

        // Check placeholders
        if self.check_placeholders {
            self.validate_placeholders(component, &mut result);
        }

        result
    }

    /// Validates the basic structure of a component
    fn validate_structure(&self, component: &Component, result: &mut ValidationResult) {
        // Check for at least one heading
        if component.heading_level().is_none() {
            result.add_error("Component must contain at least one heading");
        }

        // Check for title
        if component.title().is_none() {
            result.add_warning("Component does not have a title");
        }

        // Check minimum content length
        if component.word_count() < 10 {
            result.add_warning("Component content is very short (less than 10 words)");
        }
    }

    /// Detects emoji usage in component content
    fn detect_emojis(&self, component: &Component, result: &mut ValidationResult) {
        static EMOJI_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = EMOJI_REGEX.get_or_init(|| {
            // Match common emoji ranges
            Regex::new(r"[\u{1F300}-\u{1F9FF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}]")
                .expect("Invalid emoji regex")
        });

        let mut line_number = 0;
        for line in component.content.lines() {
            line_number += 1;
            if regex.is_match(line) {
                result.add_error(format!(
                    "Emoji detected on line {}: emojis are not allowed per AGENTS.md",
                    line_number
                ));
            }
        }
    }

    /// Validates code block language identifiers
    fn validate_code_blocks(&self, component: &Component, result: &mut ValidationResult) {
        static CODE_BLOCK_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = CODE_BLOCK_REGEX
            .get_or_init(|| Regex::new(r"^```\s*$").expect("Invalid code block regex"));

        let mut line_number = 0;
        let mut in_code_block = false;

        for line in component.content.lines() {
            line_number += 1;
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                if !in_code_block {
                    // Opening code block
                    in_code_block = true;

                    // Check if it has a language identifier
                    if regex.is_match(trimmed) {
                        result.add_error(format!(
                            "Code block on line {} is missing language identifier",
                            line_number
                        ));
                    }
                } else {
                    // Closing code block
                    in_code_block = false;
                }
            }
        }

        if in_code_block {
            result.add_error("Unclosed code block detected");
        }
    }

    /// Validates placeholder format
    fn validate_placeholders(&self, component: &Component, result: &mut ValidationResult) {
        static PLACEHOLDER_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = PLACEHOLDER_REGEX.get_or_init(|| {
            // Match {{placeholder_name}} format (including empty placeholders)
            Regex::new(r"\{\{([^}]*)\}\}").expect("Invalid placeholder regex")
        });

        let mut line_number = 0;
        for line in component.content.lines() {
            line_number += 1;

            for capture in regex.captures_iter(line) {
                if let Some(placeholder) = capture.get(1) {
                    let name = placeholder.as_str().trim();

                    // Check placeholder naming convention (lowercase_snake_case)
                    if !name
                        .chars()
                        .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
                    {
                        result.add_error(format!(
                            "Invalid placeholder format '{{{{{}}}}}' on line {}: must use lowercase_snake_case",
                            name, line_number
                        ));
                    }

                    // Check for empty placeholders
                    if name.is_empty() {
                        result.add_error(format!(
                            "Empty placeholder '{{{{}}}}' on line {}",
                            line_number
                        ));
                    }
                }
            }
        }
    }

    /// Validates a component and returns an error if invalid
    ///
    /// # Arguments
    ///
    /// * `component` - The component to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if valid, or an `Error::ComponentValidation` if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::{Component, ComponentValidator};
    /// use xzagentz::core::ComponentType;
    ///
    /// let component = Component::new(
    ///     "test",
    ///     ComponentType::Core,
    ///     "# Valid Component\n\nSome content here."
    /// );
    ///
    /// let validator = ComponentValidator::new();
    /// assert!(validator.validate_or_error(&component).is_ok());
    /// ```
    pub fn validate_or_error(&self, component: &Component) -> Result<()> {
        let result = self.validate(component);

        if result.valid {
            Ok(())
        } else {
            let error_msg = result.errors.join("; ");
            Err(Error::ComponentValidation { reason: error_msg })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ComponentType;

    #[test]
    fn test_validate_component_structure() {
        let validator = ComponentValidator::new();

        // Valid component
        let valid = Component::new(
            "test",
            ComponentType::Core,
            "# Test Component\n\nThis is some content with enough words.",
        );
        let result = validator.validate(&valid);
        assert!(result.valid);
        assert!(!result.has_errors());

        // Empty component
        let empty = Component::new("test", ComponentType::Core, "");
        let result = validator.validate(&empty);
        assert!(!result.valid);
        assert!(result.has_errors());
        assert!(result.errors[0].contains("empty"));

        // No heading
        let no_heading = Component::new("test", ComponentType::Core, "Just some text");
        let result = validator.validate(&no_heading);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("heading")));
    }

    #[test]
    fn test_detect_missing_language_identifier() {
        let validator = ComponentValidator::new();

        // Code block without language
        let invalid = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\n```\nfn main() {}\n```",
        );
        let result = validator.validate(&invalid);
        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("missing language identifier")));

        // Code block with language
        let valid = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\n```rust\nfn main() {}\n```",
        );
        let result = validator.validate(&valid);
        // May have warnings but should not have code block errors
        assert!(!result
            .errors
            .iter()
            .any(|e| e.contains("missing language identifier")));
    }

    #[test]
    fn test_detect_emoji_usage() {
        let validator = ComponentValidator::new();

        // Content with emoji
        let with_emoji =
            Component::new("test", ComponentType::Core, "# Test 🚀\n\nContent here ✨");
        let result = validator.validate(&with_emoji);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Emoji detected")));

        // Content without emoji
        let without_emoji = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\nContent here without emojis",
        );
        let result = validator.validate(&without_emoji);
        assert!(!result.errors.iter().any(|e| e.contains("Emoji")));
    }

    #[test]
    fn test_validate_placeholder_format() {
        let validator = ComponentValidator::new();

        // Valid placeholders
        let valid = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\nUse {{project_name}} and {{version}} here.",
        );
        let result = validator.validate(&valid);
        assert!(!result
            .errors
            .iter()
            .any(|e| e.contains("Invalid placeholder")));

        // Invalid placeholders (uppercase)
        let invalid = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\nUse {{ProjectName}} here.",
        );
        let result = validator.validate(&invalid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("Invalid placeholder")));

        // Empty placeholder
        let empty_placeholder =
            Component::new("test", ComponentType::Core, "# Test\n\nUse {{}} here.");
        let result = validator.validate(&empty_placeholder);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("Empty placeholder")));
    }

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.valid);
        assert!(!result.has_errors());
        assert!(!result.has_warnings());

        result.add_warning("Test warning");
        assert!(result.valid);
        assert!(result.has_warnings());

        result.add_error("Test error");
        assert!(!result.valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validator_with_custom_settings() {
        // Disable emoji checking
        let validator = ComponentValidator::with_settings(false, true, true);

        let with_emoji = Component::new("test", ComponentType::Core, "# Test 🚀\n\nContent");
        let result = validator.validate(&with_emoji);
        // Should not report emoji errors
        assert!(!result.errors.iter().any(|e| e.contains("Emoji")));
    }

    #[test]
    fn test_validate_or_error() {
        let validator = ComponentValidator::new();

        let valid = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\nValid content with enough words.",
        );
        assert!(validator.validate_or_error(&valid).is_ok());

        let invalid = Component::new("test", ComponentType::Core, "");
        assert!(validator.validate_or_error(&invalid).is_err());
    }

    #[test]
    fn test_unclosed_code_block() {
        let validator = ComponentValidator::new();

        let unclosed = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\n```rust\nfn main() {}",
        );
        let result = validator.validate(&unclosed);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("Unclosed code block")));
    }

    #[test]
    fn test_multiple_code_blocks() {
        let validator = ComponentValidator::new();

        let multiple = Component::new(
            "test",
            ComponentType::Core,
            "# Test\n\n```rust\nlet x = 1;\n```\n\nMore text\n\n```python\nprint('hello')\n```",
        );
        let result = validator.validate(&multiple);
        // Should not have code block errors if all have language identifiers
        assert!(!result
            .errors
            .iter()
            .any(|e| e.contains("missing language identifier")));
    }

    #[test]
    fn test_short_content_warning() {
        let validator = ComponentValidator::new();

        let short = Component::new("test", ComponentType::Core, "# Test\n\nShort");
        let result = validator.validate(&short);
        assert!(result.has_warnings());
        assert!(result.warnings.iter().any(|w| w.contains("very short")));
    }
}
