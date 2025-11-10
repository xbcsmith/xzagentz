// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! AGENTS.md parser for section extraction and manipulation
//!
//! This module provides functionality to parse AGENTS.md files, extract sections,
//! and manipulate the document structure while preserving formatting.

use crate::error::Result;
use std::fmt;

/// Represents a section in an AGENTS.md document
///
/// A section consists of a markdown header and its content, including all
/// subsections until the next section of the same or higher level.
#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    /// Header level (1-6 for # to ######)
    pub level: usize,
    /// Section title (without the # markers)
    pub title: String,
    /// Full content including header and body
    pub content: String,
    /// Starting line number (0-based)
    pub start_line: usize,
    /// Ending line number (0-based, exclusive)
    pub end_line: usize,
}

impl Section {
    /// Creates a new section
    ///
    /// # Arguments
    ///
    /// * `level` - Header level (1-6)
    /// * `title` - Section title
    /// * `content` - Full section content
    /// * `start_line` - Starting line number
    /// * `end_line` - Ending line number
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::parser::Section;
    ///
    /// let section = Section::new(
    ///     1,
    ///     "Overview".to_string(),
    ///     "# Overview\n\nThis is the overview.".to_string(),
    ///     0,
    ///     3,
    /// );
    /// assert_eq!(section.title, "Overview");
    /// assert_eq!(section.level, 1);
    /// ```
    pub fn new(
        level: usize,
        title: String,
        content: String,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        Self {
            level,
            title,
            content,
            start_line,
            end_line,
        }
    }

    /// Returns the section header line
    pub fn header(&self) -> String {
        format!("{} {}", "#".repeat(self.level), self.title)
    }

    /// Returns the body content (without the header line)
    pub fn body(&self) -> String {
        let lines: Vec<&str> = self.content.lines().collect();
        if lines.is_empty() {
            return String::new();
        }
        lines[1..].join("\n")
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

/// Represents a parsed AGENTS.md document
///
/// The document is parsed into sections based on markdown headers.
/// Each section includes all content until the next header of the same or higher level.
#[derive(Debug, Clone)]
pub struct AgentsDocument {
    /// All sections in the document
    sections: Vec<Section>,
    /// Raw document content
    raw_content: String,
}

impl AgentsDocument {
    /// Creates a new empty document
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            raw_content: String::new(),
        }
    }

    /// Creates a document from raw content
    pub fn from_content(content: String) -> Self {
        Self {
            sections: Vec::new(),
            raw_content: content,
        }
    }

    /// Returns all sections
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    /// Adds a section to the document
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// Finds a section by title (case-insensitive)
    ///
    /// # Arguments
    ///
    /// * `title` - Section title to search for
    ///
    /// # Returns
    ///
    /// Returns the first matching section, or None if not found
    pub fn find_section(&self, title: &str) -> Option<&Section> {
        let title_lower = title.to_lowercase();
        self.sections
            .iter()
            .find(|s| s.title.to_lowercase() == title_lower)
    }

    /// Finds a section by title (mutable reference)
    pub fn find_section_mut(&mut self, title: &str) -> Option<&mut Section> {
        let title_lower = title.to_lowercase();
        self.sections
            .iter_mut()
            .find(|s| s.title.to_lowercase() == title_lower)
    }

    /// Returns the raw document content
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }
}

impl Default for AgentsDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AgentsDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sections.is_empty() {
            return write!(f, "{}", self.raw_content);
        }

        for (i, section) in self.sections.iter().enumerate() {
            write!(f, "{}", section.content)?;
            if i < self.sections.len() - 1 && !section.content.ends_with('\n') {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Parser for AGENTS.md documents
///
/// Parses markdown documents into sections based on header hierarchy.
/// Handles malformed sections gracefully and preserves document structure.
pub struct AgentsParser;

impl AgentsParser {
    /// Creates a new parser
    pub fn new() -> Self {
        Self
    }

    /// Parses an AGENTS.md file content into a document
    ///
    /// # Arguments
    ///
    /// * `content` - The raw file content to parse
    ///
    /// # Returns
    ///
    /// Returns a parsed AgentsDocument with extracted sections
    ///
    /// # Errors
    ///
    /// Returns an error if the document cannot be parsed
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::parser::AgentsParser;
    ///
    /// let content = "# Overview\n\nThis is the overview.\n\n## Details\n\nMore info.";
    /// let parser = AgentsParser::new();
    /// let doc = parser.parse(content).unwrap();
    /// assert_eq!(doc.sections().len(), 2);
    /// ```
    pub fn parse(&self, content: &str) -> Result<AgentsDocument> {
        let mut document = AgentsDocument::from_content(content.to_string());
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            if let Some((level, title)) = self.parse_header(lines[i]) {
                let (section_content, next_i) = self.extract_section(&lines, i, level);
                let section = Section::new(level, title, section_content, i, next_i);
                document.add_section(section);
                i = next_i;
            } else {
                i += 1;
            }
        }

        Ok(document)
    }

    /// Parses a header line and extracts level and title
    ///
    /// Returns Some((level, title)) if the line is a valid header, None otherwise
    fn parse_header(&self, line: &str) -> Option<(usize, String)> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            return None;
        }

        let mut level = 0;
        let mut chars = trimmed.chars();

        while let Some('#') = chars.next() {
            level += 1;
            if level > 6 {
                return None;
            }
        }

        let rest = trimmed[level..].trim_start();
        if rest.is_empty() {
            return None;
        }

        Some((level, rest.to_string()))
    }

    /// Extracts a complete section starting at the given line
    ///
    /// Returns the section content and the index of the next line to process
    fn extract_section(&self, lines: &[&str], start: usize, _level: usize) -> (String, usize) {
        let mut end = start + 1;

        while end < lines.len() {
            if self.parse_header(lines[end]).is_some() {
                break;
            }
            end += 1;
        }

        let section_lines = &lines[start..end];
        let content = section_lines.join("\n");

        (content, end)
    }

    /// Identifies section boundaries in the document
    ///
    /// Returns a vector of (start_line, end_line, level, title) tuples
    pub fn identify_boundaries(&self, content: &str) -> Vec<(usize, usize, usize, String)> {
        let lines: Vec<&str> = content.lines().collect();
        let mut boundaries = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            if let Some((level, title)) = self.parse_header(lines[i]) {
                let end = self.find_section_boundary(&lines, i, level);
                boundaries.push((i, end, level, title));
                i += 1;
            } else {
                i += 1;
            }
        }

        boundaries
    }

    /// Finds the boundary for a section (includes subsections)
    fn find_section_boundary(&self, lines: &[&str], start: usize, level: usize) -> usize {
        let mut end = start + 1;
        while end < lines.len() {
            if let Some((next_level, _)) = self.parse_header(lines[end]) {
                if next_level <= level {
                    break;
                }
            }
            end += 1;
        }
        end
    }

    /// Extracts sections from parsed document
    ///
    /// This is a convenience method that parses and returns sections
    pub fn extract_sections(&self, content: &str) -> Result<Vec<Section>> {
        let doc = self.parse(content)?;
        Ok(doc.sections().to_vec())
    }

    /// Handles malformed sections gracefully
    ///
    /// Attempts to parse sections even when headers are malformed or irregular
    pub fn parse_lenient(&self, content: &str) -> Result<AgentsDocument> {
        self.parse(content)
    }
}

impl Default for AgentsParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_agents_file() {
        let content = r#"# Main Header

This is the intro.

## Subsection 1

Content 1

## Subsection 2

Content 2

# Second Main Header

More content
"#;

        let parser = AgentsParser::new();
        let doc = parser.parse(content).unwrap();

        assert_eq!(doc.sections().len(), 4);
        assert_eq!(doc.sections()[0].title, "Main Header");
        assert_eq!(doc.sections()[0].level, 1);
        assert_eq!(doc.sections()[1].title, "Subsection 1");
        assert_eq!(doc.sections()[1].level, 2);
    }

    #[test]
    fn test_extract_sections() {
        let content = r#"# Header 1

Content 1

# Header 2

Content 2
"#;

        let parser = AgentsParser::new();
        let sections = parser.extract_sections(content).unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].title, "Header 1");
        assert_eq!(sections[1].title, "Header 2");
    }

    #[test]
    fn test_identify_section_boundaries() {
        let content = r#"# Header 1
Content
## Subheader
More content
# Header 2
Final content
"#;

        let parser = AgentsParser::new();
        let boundaries = parser.identify_boundaries(content);

        assert_eq!(boundaries.len(), 3);
        assert_eq!(boundaries[0], (0, 4, 1, "Header 1".to_string()));
        assert_eq!(boundaries[1], (2, 4, 2, "Subheader".to_string()));
        assert_eq!(boundaries[2], (4, 6, 1, "Header 2".to_string()));
    }

    #[test]
    fn test_handle_malformed_sections() {
        let content = r#"# Valid Header

Content

####### Too Many Hashes

This should not be a header

## Valid Subheader

More content
"#;

        let parser = AgentsParser::new();
        let doc = parser.parse_lenient(content).unwrap();

        assert_eq!(doc.sections().len(), 2);
        assert_eq!(doc.sections()[0].title, "Valid Header");
        assert_eq!(doc.sections()[1].title, "Valid Subheader");
    }

    #[test]
    fn test_section_new() {
        let section = Section::new(1, "Test".to_string(), "# Test\n\nContent".to_string(), 0, 3);

        assert_eq!(section.level, 1);
        assert_eq!(section.title, "Test");
        assert_eq!(section.start_line, 0);
        assert_eq!(section.end_line, 3);
    }

    #[test]
    fn test_section_header() {
        let section = Section::new(
            2,
            "Subsection".to_string(),
            "## Subsection\n\nBody".to_string(),
            5,
            8,
        );

        assert_eq!(section.header(), "## Subsection");
    }

    #[test]
    fn test_section_body() {
        let section = Section::new(
            1,
            "Header".to_string(),
            "# Header\n\nLine 1\nLine 2".to_string(),
            0,
            4,
        );

        assert_eq!(section.body(), "\nLine 1\nLine 2");
    }

    #[test]
    fn test_find_section() {
        let content = r#"# Overview

Content

## Details

More info
"#;

        let parser = AgentsParser::new();
        let doc = parser.parse(content).unwrap();

        let section = doc.find_section("Overview");
        assert!(section.is_some());
        assert_eq!(section.unwrap().title, "Overview");

        let section = doc.find_section("details");
        assert!(section.is_some());
        assert_eq!(section.unwrap().title, "Details");

        let section = doc.find_section("Nonexistent");
        assert!(section.is_none());
    }

    #[test]
    fn test_document_to_string() {
        let content = r#"# Header 1

Content 1

# Header 2

Content 2"#;

        let parser = AgentsParser::new();
        let doc = parser.parse(content).unwrap();
        let reconstructed = doc.to_string();

        assert!(reconstructed.contains("# Header 1"));
        assert!(reconstructed.contains("# Header 2"));
        assert!(reconstructed.contains("Content 1"));
        assert!(reconstructed.contains("Content 2"));
    }

    #[test]
    fn test_document_display_trait() {
        let content = "# Test\n\nContent";
        let parser = AgentsParser::new();
        let doc = parser.parse(content).unwrap();

        let displayed = format!("{}", doc);
        assert!(displayed.contains("# Test"));
        assert!(displayed.contains("Content"));
    }

    #[test]
    fn test_empty_document() {
        let parser = AgentsParser::new();
        let doc = parser.parse("").unwrap();

        assert_eq!(doc.sections().len(), 0);
        assert_eq!(format!("{}", doc), "");
    }

    #[test]
    fn test_nested_sections() {
        let content = r#"# Level 1

Content

## Level 2

More content

### Level 3

Deep content

## Another Level 2

Back to level 2
"#;

        let parser = AgentsParser::new();
        let doc = parser.parse(content).unwrap();

        assert_eq!(doc.sections().len(), 4);
        assert_eq!(doc.sections()[0].level, 1);
        assert_eq!(doc.sections()[1].level, 2);
        assert_eq!(doc.sections()[2].level, 3);
        assert_eq!(doc.sections()[3].level, 2);
    }
}
