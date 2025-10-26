//! Language-specific section filtering and extraction
//!
//! This module provides functionality for parsing and extracting language-specific
//! sections from component content using HTML-style comment markers.
//!
//! # Section Markers
//!
//! Language sections are delimited by HTML-style comments:
//! - `<!-- LANG:rust -->` ... `<!-- /LANG -->` - Rust-specific content
//! - `<!-- LANG:python -->` ... `<!-- /LANG -->` - Python-specific content
//! - `<!-- LANG:* -->` ... `<!-- /LANG -->` - Language-agnostic fallback
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::language_filter::LanguageFilter;
//!
//! let content = r#"
//! Common content here
//!
//! <!-- LANG:rust -->
//! Rust-specific content
//! <!-- /LANG -->
//!
//! <!-- LANG:python -->
//! Python-specific content
//! <!-- /LANG -->
//! "#;
//!
//! let filter = LanguageFilter::new("rust");
//! let filtered = filter.filter_content(content)?;
//! assert!(filtered.contains("Rust-specific"));
//! assert!(!filtered.contains("Python-specific"));
//! # Ok::<(), xzagentz::Error>(())
//! ```

use crate::error::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex pattern for detecting language section start markers
    /// Matches: <!-- LANG:rust -->, <!-- LANG:python -->, <!-- LANG:* -->, etc.
    static ref LANG_START_PATTERN: Regex = Regex::new(
        r"<!--\s*LANG:([a-z*]+)\s*-->"
    ).unwrap();

    /// Regex pattern for detecting language section end markers
    /// Matches: <!-- /LANG -->
    static ref LANG_END_PATTERN: Regex = Regex::new(
        r"<!--\s*/LANG\s*-->"
    ).unwrap();
}

/// Language-specific content section
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageSection {
    /// Language identifier (e.g., "rust", "python", "*" for agnostic)
    pub language: String,

    /// Content within the language section
    pub content: String,

    /// Start line number (0-indexed)
    pub start_line: usize,

    /// End line number (0-indexed)
    pub end_line: usize,
}

/// Filter for extracting language-specific content
#[derive(Debug, Clone)]
pub struct LanguageFilter {
    /// Target language to filter for
    target_language: String,

    /// Whether to include fallback content (LANG:*)
    include_fallback: bool,
}

impl LanguageFilter {
    /// Creates a new LanguageFilter for the specified target language
    ///
    /// # Arguments
    ///
    /// * `target_language` - Language to filter for (e.g., "rust", "python")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::language_filter::LanguageFilter;
    ///
    /// let filter = LanguageFilter::new("rust");
    /// ```
    pub fn new(target_language: impl Into<String>) -> Self {
        Self {
            target_language: target_language.into(),
            include_fallback: true,
        }
    }

    /// Creates a new LanguageFilter with fallback control
    ///
    /// # Arguments
    ///
    /// * `target_language` - Language to filter for
    /// * `include_fallback` - Whether to include LANG:* sections
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::language_filter::LanguageFilter;
    ///
    /// let filter = LanguageFilter::with_fallback("rust", false);
    /// ```
    pub fn with_fallback(target_language: impl Into<String>, include_fallback: bool) -> Self {
        Self {
            target_language: target_language.into(),
            include_fallback,
        }
    }

    /// Filters content to include only target language sections
    ///
    /// Extracts all sections matching the target language or fallback (LANG:*)
    /// and returns the filtered content with language markers removed.
    ///
    /// # Arguments
    ///
    /// * `content` - Content containing language sections
    ///
    /// # Returns
    ///
    /// Returns filtered content with only relevant language sections
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentValidation` if:
    /// - Language section markers are unclosed
    /// - Nested language sections are detected
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::language_filter::LanguageFilter;
    ///
    /// let content = r#"
    /// Common content
    /// <!-- LANG:rust -->
    /// Rust content
    /// <!-- /LANG -->
    /// "#;
    ///
    /// let filter = LanguageFilter::new("rust");
    /// let filtered = filter.filter_content(content)?;
    /// assert!(filtered.contains("Rust content"));
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn filter_content(&self, content: &str) -> Result<String> {
        let sections = self.parse_sections(content)?;
        let mut result = String::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_line = 0;

        for section in &sections {
            // Add content before this section
            if current_line < section.start_line {
                for line in &lines[current_line..section.start_line] {
                    result.push_str(line);
                    result.push('\n');
                }
            }

            // Include section if it matches target language or is fallback
            if section.language == self.target_language
                || (self.include_fallback && section.language == "*")
            {
                result.push_str(&section.content);
                if !section.content.ends_with('\n') {
                    result.push('\n');
                }
            }

            current_line = section.end_line + 1;
        }

        // Add remaining content after last section
        if current_line < lines.len() {
            for line in &lines[current_line..] {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result.trim_end().to_string())
    }

    /// Parses all language sections from content
    ///
    /// # Arguments
    ///
    /// * `content` - Content to parse
    ///
    /// # Returns
    ///
    /// Returns a vector of all detected LanguageSection structs
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentValidation` if sections are malformed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::language_filter::LanguageFilter;
    ///
    /// let content = "<!-- LANG:rust -->\nRust\n<!-- /LANG -->";
    /// let filter = LanguageFilter::new("rust");
    /// let sections = filter.parse_sections(content)?;
    /// assert_eq!(sections.len(), 1);
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn parse_sections(&self, content: &str) -> Result<Vec<LanguageSection>> {
        let mut sections = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // Check for language section start
            if let Some(caps) = LANG_START_PATTERN.captures(line) {
                let language = caps.get(1).unwrap().as_str().to_string();
                let start_line = i;

                // Find matching end marker
                let mut end_line = None;
                let mut section_content = String::new();

                for (j, line_content) in lines.iter().enumerate().skip(i + 1) {
                    if LANG_END_PATTERN.is_match(line_content) {
                        end_line = Some(j);
                        break;
                    }

                    // Check for nested sections (not allowed)
                    if LANG_START_PATTERN.is_match(line_content) {
                        return Err(Error::ComponentValidation {
                            reason: format!("Nested language section detected at line {}", j + 1),
                        });
                    }

                    section_content.push_str(line_content);
                    section_content.push('\n');
                }

                match end_line {
                    Some(end) => {
                        sections.push(LanguageSection {
                            language,
                            content: section_content.trim_end().to_string(),
                            start_line,
                            end_line: end,
                        });
                        i = end + 1;
                    }
                    None => {
                        return Err(Error::ComponentValidation {
                            reason: format!(
                                "Unclosed language section starting at line {}",
                                start_line + 1
                            ),
                        });
                    }
                }
            } else {
                i += 1;
            }
        }

        Ok(sections)
    }

    /// Checks if content contains any language sections
    ///
    /// # Arguments
    ///
    /// * `content` - Content to check
    ///
    /// # Returns
    ///
    /// Returns `true` if content contains language section markers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::language_filter::LanguageFilter;
    ///
    /// let filter = LanguageFilter::new("rust");
    /// assert!(filter.has_language_sections("<!-- LANG:rust -->test<!-- /LANG -->"));
    /// assert!(!filter.has_language_sections("No sections here"));
    /// ```
    pub fn has_language_sections(&self, content: &str) -> bool {
        LANG_START_PATTERN.is_match(content)
    }

    /// Extracts available languages from content
    ///
    /// # Arguments
    ///
    /// * `content` - Content to analyze
    ///
    /// # Returns
    ///
    /// Returns a vector of language identifiers found in the content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::language_filter::LanguageFilter;
    ///
    /// let content = "<!-- LANG:rust -->\nRust content\n<!-- /LANG -->\n<!-- LANG:python -->\nPython content\n<!-- /LANG -->";
    /// let filter = LanguageFilter::new("rust");
    /// let langs = filter.available_languages(content)?;
    /// assert!(langs.contains(&"rust".to_string()));
    /// assert!(langs.contains(&"python".to_string()));
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn available_languages(&self, content: &str) -> Result<Vec<String>> {
        let sections = self.parse_sections(content)?;
        let mut languages: Vec<String> = sections
            .into_iter()
            .map(|s| s.language)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        languages.sort();
        Ok(languages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_rust_content() {
        let content = r#"
Common content

<!-- LANG:rust -->
Rust-specific content
<!-- /LANG -->

<!-- LANG:python -->
Python-specific content
<!-- /LANG -->

More common content
"#;

        let filter = LanguageFilter::new("rust");
        let result = filter.filter_content(content).unwrap();

        assert!(result.contains("Common content"));
        assert!(result.contains("Rust-specific content"));
        assert!(!result.contains("Python-specific content"));
        assert!(result.contains("More common content"));
    }

    #[test]
    fn test_filter_with_fallback() {
        let content = r#"
<!-- LANG:* -->
Fallback content
<!-- /LANG -->

<!-- LANG:rust -->
Rust content
<!-- /LANG -->
"#;

        let filter = LanguageFilter::new("rust");
        let result = filter.filter_content(content).unwrap();

        assert!(result.contains("Fallback content"));
        assert!(result.contains("Rust content"));
    }

    #[test]
    fn test_filter_without_fallback() {
        let content = r#"
<!-- LANG:* -->
Fallback content
<!-- /LANG -->

<!-- LANG:rust -->
Rust content
<!-- /LANG -->
"#;

        let filter = LanguageFilter::with_fallback("rust", false);
        let result = filter.filter_content(content).unwrap();

        assert!(!result.contains("Fallback content"));
        assert!(result.contains("Rust content"));
    }

    #[test]
    fn test_parse_sections_success() {
        let content = r#"
<!-- LANG:rust -->
Rust content
<!-- /LANG -->

<!-- LANG:python -->
Python content
<!-- /LANG -->
"#;

        let filter = LanguageFilter::new("rust");
        let sections = filter.parse_sections(content).unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].language, "rust");
        assert_eq!(sections[1].language, "python");
        assert!(sections[0].content.contains("Rust content"));
    }

    #[test]
    fn test_parse_sections_unclosed() {
        let content = r#"
<!-- LANG:rust -->
Rust content without closing
"#;

        let filter = LanguageFilter::new("rust");
        let result = filter.parse_sections(content);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed"));
    }

    #[test]
    fn test_parse_sections_nested() {
        let content = r#"
<!-- LANG:rust -->
Content
<!-- LANG:python -->
Nested not allowed
<!-- /LANG -->
<!-- /LANG -->
"#;

        let filter = LanguageFilter::new("rust");
        let result = filter.parse_sections(content);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Nested"));
    }

    #[test]
    fn test_has_language_sections() {
        let filter = LanguageFilter::new("rust");

        assert!(filter.has_language_sections("<!-- LANG:rust -->test<!-- /LANG -->"));
        assert!(filter.has_language_sections("text\n<!-- LANG:python -->test<!-- /LANG -->"));
        assert!(!filter.has_language_sections("No sections here"));
        assert!(!filter.has_language_sections("Just <!-- comments -->"));
    }

    #[test]
    fn test_available_languages() {
        let content = r#"
<!-- LANG:rust -->
Rust
<!-- /LANG -->

<!-- LANG:python -->
Python
<!-- /LANG -->

<!-- LANG:golang -->
Go
<!-- /LANG -->
"#;

        let filter = LanguageFilter::new("rust");
        let langs = filter.available_languages(content).unwrap();

        assert_eq!(langs.len(), 3);
        assert!(langs.contains(&"rust".to_string()));
        assert!(langs.contains(&"python".to_string()));
        assert!(langs.contains(&"golang".to_string()));
    }

    #[test]
    fn test_available_languages_with_fallback() {
        let content = r#"
<!-- LANG:* -->
Fallback
<!-- /LANG -->

<!-- LANG:rust -->
Rust
<!-- /LANG -->
"#;

        let filter = LanguageFilter::new("rust");
        let langs = filter.available_languages(content).unwrap();

        assert_eq!(langs.len(), 2);
        assert!(langs.contains(&"*".to_string()));
        assert!(langs.contains(&"rust".to_string()));
    }

    #[test]
    fn test_filter_content_no_sections() {
        let content = "Just plain content without sections";

        let filter = LanguageFilter::new("rust");
        let result = filter.filter_content(content).unwrap();

        assert_eq!(result, content);
    }

    #[test]
    fn test_filter_content_empty() {
        let filter = LanguageFilter::new("rust");
        let result = filter.filter_content("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_language_section_equality() {
        let section1 = LanguageSection {
            language: "rust".to_string(),
            content: "test".to_string(),
            start_line: 0,
            end_line: 2,
        };

        let section2 = LanguageSection {
            language: "rust".to_string(),
            content: "test".to_string(),
            start_line: 0,
            end_line: 2,
        };

        assert_eq!(section1, section2);
    }

    #[test]
    fn test_marker_patterns() {
        assert!(LANG_START_PATTERN.is_match("<!-- LANG:rust -->"));
        assert!(LANG_START_PATTERN.is_match("<!-- LANG:python -->"));
        assert!(LANG_START_PATTERN.is_match("<!-- LANG:* -->"));
        assert!(LANG_START_PATTERN.is_match("<!--LANG:rust-->"));
        assert!(!LANG_START_PATTERN.is_match("<!-- LANG:Rust -->"));
        assert!(!LANG_START_PATTERN.is_match("<!-- LANG -->"));

        assert!(LANG_END_PATTERN.is_match("<!-- /LANG -->"));
        assert!(LANG_END_PATTERN.is_match("<!--/LANG-->"));
        assert!(!LANG_END_PATTERN.is_match("<!-- LANG -->"));
    }

    #[test]
    fn test_filter_multiple_same_language() {
        let content = r#"
<!-- LANG:rust -->
First rust section
<!-- /LANG -->

Some text

<!-- LANG:rust -->
Second rust section
<!-- /LANG -->
"#;

        let filter = LanguageFilter::new("rust");
        let result = filter.filter_content(content).unwrap();

        assert!(result.contains("First rust section"));
        assert!(result.contains("Second rust section"));
    }
}
