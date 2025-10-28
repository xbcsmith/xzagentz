//! Markdown cleaner module for normalizing generated content
//!
//! This module provides utilities to clean and normalize markdown content
//! to ensure it passes markdown linting rules without requiring external
//! dependencies like markdownlint.
//!
//! # Features
//!
//! - Fix ordered list numbering (sequential 1, 2, 3, ...)
//! - Add language specifiers to fenced code blocks
//! - Remove trailing whitespace
//! - Normalize line endings
//!
//! # Examples
//!
//! ```
//! use xzagentz::markdown::cleaner::MarkdownCleaner;
//!
//! let content = "Some markdown content";
//! let cleaned = MarkdownCleaner::clean(content);
//! ```

use crate::error::Result;

/// Cleans and normalizes markdown content
pub struct MarkdownCleaner;

impl MarkdownCleaner {
    /// Cleans markdown content to pass linting rules
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown content to clean
    ///
    /// # Returns
    ///
    /// Returns cleaned markdown content
    ///
    /// # Errors
    ///
    /// Currently returns Ok always, but signature allows for future error handling
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::markdown::cleaner::MarkdownCleaner;
    ///
    /// let content = "1. First\n3. Third\n";
    /// let cleaned = MarkdownCleaner::clean(content).unwrap();
    /// assert!(cleaned.contains("1. First"));
    /// assert!(cleaned.contains("2. Third"));
    /// ```
    pub fn clean(content: &str) -> Result<String> {
        let mut output = content.to_string();

        // Apply cleaning operations in order
        output = Self::fix_ordered_lists(&output);
        output = Self::add_code_fence_languages(&output);
        output = Self::remove_trailing_whitespace(&output);
        output = Self::normalize_line_endings(&output);

        Ok(output)
    }

    /// Fixes ordered list numbering to be sequential
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown content with potentially incorrect list numbering
    ///
    /// # Returns
    ///
    /// Returns content with corrected list numbering
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::markdown::cleaner::MarkdownCleaner;
    ///
    /// let content = "1. First\n3. Third\n5. Fifth";
    /// let fixed = MarkdownCleaner::fix_ordered_lists(content);
    /// assert!(fixed.contains("1. First"));
    /// assert!(fixed.contains("2. Third"));
    /// assert!(fixed.contains("3. Fifth"));
    /// ```
    pub fn fix_ordered_lists(content: &str) -> String {
        let mut output = String::new();
        let mut list_counters: Vec<(usize, usize)> = Vec::new(); // (indent_level, counter)

        for line in content.lines() {
            let trimmed = line.trim_start();
            let indent = line.len() - trimmed.len();

            // Check if this is an ordered list item
            if let Some(rest) = Self::parse_ordered_list_item(trimmed) {
                // Find the appropriate counter for this indent level
                let counter_idx = list_counters.iter().position(|(lvl, _)| *lvl == indent);

                let number = if let Some(idx) = counter_idx {
                    // Increment existing counter at this level
                    list_counters[idx].1 += 1;
                    // Remove any deeper nested counters
                    list_counters.truncate(idx + 1);
                    list_counters[idx].1
                } else {
                    // New indent level - start at 1
                    // Remove counters at deeper levels
                    list_counters.retain(|(lvl, _)| *lvl < indent);
                    list_counters.push((indent, 1));
                    1
                };

                // Reconstruct line with correct number
                let spaces = " ".repeat(indent);
                output.push_str(&format!("{}{}. {}\n", spaces, number, rest));
            } else {
                // Not an ordered list item
                if !trimmed.is_empty() && !trimmed.starts_with('-') && !trimmed.starts_with('*') {
                    // Non-list content - reset list state
                    list_counters.clear();
                }

                output.push_str(line);
                output.push('\n');
            }
        }

        // Remove trailing newline if original didn't have one
        if !content.ends_with('\n') {
            output.pop();
        }

        output
    }

    /// Parses an ordered list item and returns the content after the number
    ///
    /// # Arguments
    ///
    /// * `line` - Line to parse (already trimmed of leading whitespace)
    ///
    /// # Returns
    ///
    /// Returns Some(content) if line is an ordered list item, None otherwise
    fn parse_ordered_list_item(line: &str) -> Option<String> {
        // Match patterns like "1. ", "2. ", "10. ", etc.
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        // Skip leading digits
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }

        // Must have at least one digit, followed by period and space
        if i > 0 && i < chars.len() && chars[i] == '.' {
            // Check for space or direct content after period
            if i + 1 < chars.len() && chars[i + 1] == ' ' {
                return Some(chars[i + 2..].iter().collect());
            } else if i + 1 < chars.len() {
                // No space after period, but there's content
                return Some(chars[i + 1..].iter().collect());
            }
        }

        None
    }

    /// Adds language specifiers to fenced code blocks that lack them
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown content with code blocks
    ///
    /// # Returns
    ///
    /// Returns content with language specifiers added to bare code fences
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::markdown::cleaner::MarkdownCleaner;
    ///
    /// let content = "```\ncode\n```";
    /// let fixed = MarkdownCleaner::add_code_fence_languages(content);
    /// assert!(fixed.contains("```text"));
    /// ```
    pub fn add_code_fence_languages(content: &str) -> String {
        let mut output = String::new();
        let mut in_code_block = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                if !in_code_block {
                    // Opening fence
                    let language = trimmed.trim_start_matches('`').trim();

                    if language.is_empty() {
                        // No language specified - add "text" as default
                        let indent = line.len() - trimmed.len();
                        output.push_str(&format!("{}```text\n", " ".repeat(indent)));
                    } else {
                        // Language already specified
                        output.push_str(line);
                        output.push('\n');
                    }

                    in_code_block = true;
                } else {
                    // Closing fence
                    output.push_str(line);
                    output.push('\n');
                    in_code_block = false;
                }
            } else {
                output.push_str(line);
                output.push('\n');
            }
        }

        // Remove trailing newline if original didn't have one
        if !content.ends_with('\n') {
            output.pop();
        }

        output
    }

    /// Removes trailing whitespace from each line
    ///
    /// # Arguments
    ///
    /// * `content` - Content with potential trailing whitespace
    ///
    /// # Returns
    ///
    /// Returns content with trailing whitespace removed from each line
    pub fn remove_trailing_whitespace(content: &str) -> String {
        content
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Normalizes line endings to Unix style (LF)
    ///
    /// # Arguments
    ///
    /// * `content` - Content with potentially mixed line endings
    ///
    /// # Returns
    ///
    /// Returns content with normalized line endings
    pub fn normalize_line_endings(content: &str) -> String {
        content.replace("\r\n", "\n").replace('\r', "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_ordered_lists_simple() {
        let content = "1. First\n3. Third\n5. Fifth";
        let fixed = MarkdownCleaner::fix_ordered_lists(content);

        assert!(fixed.contains("1. First"));
        assert!(fixed.contains("2. Third"));
        assert!(fixed.contains("3. Fifth"));
    }

    #[test]
    fn test_fix_ordered_lists_with_text_between() {
        let content = "1. First\n\nSome text\n\n1. Second list first\n3. Second list third";
        let fixed = MarkdownCleaner::fix_ordered_lists(content);

        assert!(fixed.contains("1. First"));
        assert!(fixed.contains("1. Second list first"));
        assert!(fixed.contains("2. Second list third"));
    }

    #[test]
    fn test_fix_ordered_lists_nested() {
        let content = "1. First\n   1. Nested first\n   5. Nested second\n2. Second";
        let fixed = MarkdownCleaner::fix_ordered_lists(content);

        assert!(fixed.contains("1. First"));
        assert!(fixed.contains("   1. Nested first"));
        assert!(fixed.contains("   2. Nested second"));
        assert!(fixed.contains("2. Second"));
    }

    #[test]
    fn test_fix_ordered_lists_correct_already() {
        let content = "1. First\n2. Second\n3. Third";
        let fixed = MarkdownCleaner::fix_ordered_lists(content);

        assert_eq!(content, fixed.trim());
    }

    #[test]
    fn test_add_code_fence_languages_bare_fence() {
        let content = "```\ncode here\n```";
        let fixed = MarkdownCleaner::add_code_fence_languages(content);

        assert!(fixed.contains("```text"));
        assert!(fixed.contains("code here"));
    }

    #[test]
    fn test_add_code_fence_languages_with_language() {
        let content = "```rust\nfn main() {}\n```";
        let fixed = MarkdownCleaner::add_code_fence_languages(content);

        assert!(fixed.contains("```rust"));
        assert!(!fixed.contains("```text"));
    }

    #[test]
    fn test_add_code_fence_languages_multiple_blocks() {
        let content = "```\nfirst\n```\n\n```python\nsecond\n```\n\n```\nthird\n```";
        let fixed = MarkdownCleaner::add_code_fence_languages(content);

        let text_count = fixed.matches("```text").count();
        assert_eq!(text_count, 2);
        assert!(fixed.contains("```python"));
    }

    #[test]
    fn test_remove_trailing_whitespace() {
        let content = "line1  \nline2\t\nline3\n";
        let fixed = MarkdownCleaner::remove_trailing_whitespace(content);

        assert_eq!(fixed, "line1\nline2\nline3");
    }

    #[test]
    fn test_normalize_line_endings_crlf() {
        let content = "line1\r\nline2\r\nline3";
        let fixed = MarkdownCleaner::normalize_line_endings(content);

        assert_eq!(fixed, "line1\nline2\nline3");
    }

    #[test]
    fn test_normalize_line_endings_cr() {
        let content = "line1\rline2\rline3";
        let fixed = MarkdownCleaner::normalize_line_endings(content);

        assert_eq!(fixed, "line1\nline2\nline3");
    }

    #[test]
    fn test_clean_full_pipeline() {
        let content = "1. First\n3. Third\n\n```\ncode\n```\nline  ";
        let cleaned = MarkdownCleaner::clean(content).unwrap();

        assert!(cleaned.contains("1. First"));
        assert!(cleaned.contains("2. Third"));
        assert!(cleaned.contains("```text"));
        assert!(!cleaned.contains("line  "));
        // Line endings are normalized, so check without trailing newline
        assert!(cleaned.contains("line"));
    }

    #[test]
    fn test_parse_ordered_list_item_valid() {
        assert_eq!(
            MarkdownCleaner::parse_ordered_list_item("1. First item"),
            Some("First item".to_string())
        );
        assert_eq!(
            MarkdownCleaner::parse_ordered_list_item("10. Tenth item"),
            Some("Tenth item".to_string())
        );
        assert_eq!(
            MarkdownCleaner::parse_ordered_list_item("5.No space"),
            Some("No space".to_string())
        );
    }

    #[test]
    fn test_parse_ordered_list_item_invalid() {
        assert_eq!(
            MarkdownCleaner::parse_ordered_list_item("- Unordered"),
            None
        );
        assert_eq!(MarkdownCleaner::parse_ordered_list_item("Not a list"), None);
        assert_eq!(
            MarkdownCleaner::parse_ordered_list_item("1 No period"),
            None
        );
    }
}
