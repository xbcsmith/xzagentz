// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Architecture document parser for markdown format
//!
//! This module implements the ArchitectureParser trait to read and parse
//! markdown architecture documents into structured domain models.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::infrastructure::fileio::MarkdownArchitectureParser;
//! use xzagentz::domain::planning::ArchitectureParser;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = MarkdownArchitectureParser::new();
//! let content = "# My Architecture\n\nThis is the architecture document.";
//! let architecture = parser.parse(content)?;
//!
//! assert_eq!(architecture.title(), "My Architecture");
//! # Ok(())
//! # }
//! ```

use super::error::FileIoError;
use crate::domain::planning::{
    ArchitectureDocument, ArchitectureParser, Component, Requirement, Section,
};
use regex::Regex;

/// Markdown architecture parser
///
/// Implements the ArchitectureParser trait to parse markdown documents
/// into structured ArchitectureDocument models.
///
/// # Parsing Rules
///
/// - First level-1 heading becomes the title
/// - Subsequent headings become sections
/// - Lines starting with "- Component:" or "## Components" section are parsed as components
/// - Lines starting with "- Requirement:" or "## Requirements" section are parsed as requirements
///
/// # Examples
///
/// ```rust
/// use xzagentz::infrastructure::fileio::MarkdownArchitectureParser;
/// use xzagentz::domain::planning::ArchitectureParser;
///
/// let parser = MarkdownArchitectureParser::new();
/// let markdown = r#"
/// # Project Architecture
///
/// This is the overview.
///
/// ## Components
///
/// - API Server
/// - Database
///
/// ## Requirements
///
/// - Must be scalable
/// - Must be secure
/// "#;
///
/// let result = parser.parse(markdown);
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct MarkdownArchitectureParser {
    /// Regex for matching markdown headings
    heading_regex: Regex,
}

impl Default for MarkdownArchitectureParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownArchitectureParser {
    /// Creates a new markdown architecture parser
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::MarkdownArchitectureParser;
    ///
    /// let parser = MarkdownArchitectureParser::new();
    /// ```
    pub fn new() -> Self {
        Self {
            heading_regex: Regex::new(r"^(#{1,6})\s+(.+)$").expect("Valid regex"),
        }
    }

    /// Parses markdown headings and content into sections
    ///
    /// Skips the first level-1 heading (title) and parses level-2+ headings as sections
    fn parse_sections(&self, content: &str) -> Vec<Section> {
        let mut sections = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if let Some(captures) = self.heading_regex.captures(line) {
                let level = captures.get(1).unwrap().as_str().len() as u8;
                let heading = captures.get(2).unwrap().as_str().trim().to_string();

                // Skip level-1 headings (title) - only parse sections (level 2+)
                if level > 1 {
                    // Collect content until next heading of same or higher level
                    let mut content_lines = Vec::new();
                    i += 1;

                    while i < lines.len() {
                        let next_line = lines[i].trim();
                        if let Some(next_captures) = self.heading_regex.captures(next_line) {
                            let next_level = next_captures.get(1).unwrap().as_str().len() as u8;
                            if next_level <= level {
                                break;
                            }
                        }
                        if !next_line.is_empty() {
                            content_lines.push(next_line);
                        }
                        i += 1;
                    }

                    let content = content_lines.join("\n");
                    let section = Section::new(heading, level).with_content(content);
                    sections.push(section);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        sections
    }

    /// Extracts components from content
    fn extract_components(&self, content: &str) -> Vec<Component> {
        let mut components = Vec::new();
        let mut in_components_section = false;
        let mut component_counter = 0;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check for components section header
            if trimmed.starts_with("## Components")
                || trimmed.starts_with("# Components")
                || trimmed.eq_ignore_ascii_case("components:")
            {
                in_components_section = true;
                continue;
            }

            // Check for end of components section
            if in_components_section
                && (trimmed.starts_with("##") || trimmed.starts_with('#'))
                && !trimmed.to_lowercase().contains("component")
            {
                in_components_section = false;
                continue;
            }

            // Parse component lines
            if in_components_section && trimmed.starts_with('-') {
                let component_text = trimmed.trim_start_matches('-').trim();
                if !component_text.is_empty() {
                    component_counter += 1;
                    let id = format!("component-{}", component_counter);

                    // Try to split name and description by colon or dash
                    if let Some(colon_pos) = component_text.find(':') {
                        let name = component_text[..colon_pos].trim();
                        let description = component_text[colon_pos + 1..].trim();
                        components.push(Component::new(name, &id).with_description(description));
                    } else if let Some(dash_pos) = component_text.find(" - ") {
                        let name = component_text[..dash_pos].trim();
                        let description = component_text[dash_pos + 3..].trim();
                        components.push(Component::new(name, &id).with_description(description));
                    } else {
                        components.push(Component::new(component_text, &id));
                    }
                }
            }

            // Also look for inline component markers
            if trimmed.to_lowercase().starts_with("- component:") {
                let component_text = trimmed[12..].trim();
                if !component_text.is_empty() {
                    component_counter += 1;
                    let id = format!("component-{}", component_counter);
                    components.push(Component::new(component_text, &id));
                }
            }
        }

        components
    }

    /// Extracts requirements from content
    fn extract_requirements(&self, content: &str) -> Vec<Requirement> {
        let mut requirements = Vec::new();
        let mut in_requirements_section = false;
        let mut requirement_counter = 0;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check for requirements section header
            if trimmed.starts_with("## Requirements")
                || trimmed.starts_with("# Requirements")
                || trimmed.eq_ignore_ascii_case("requirements:")
            {
                in_requirements_section = true;
                continue;
            }

            // Check for end of requirements section
            if in_requirements_section
                && (trimmed.starts_with("##") || trimmed.starts_with('#'))
                && !trimmed.to_lowercase().contains("requirement")
            {
                in_requirements_section = false;
                continue;
            }

            // Parse requirement lines
            if in_requirements_section && trimmed.starts_with('-') {
                let requirement_text = trimmed.trim_start_matches('-').trim();
                if !requirement_text.is_empty() {
                    requirement_counter += 1;
                    let id = format!("req-{}", requirement_counter);

                    // Try to extract priority
                    let mut priority = None;
                    let mut description = requirement_text;

                    if let Some(prio_start) = requirement_text.to_lowercase().find("(priority:") {
                        if let Some(prio_end) = requirement_text[prio_start..].find(')') {
                            let prio_text =
                                &requirement_text[prio_start + 10..prio_start + prio_end];
                            priority = Some(prio_text.trim().to_string());
                            description = requirement_text[..prio_start].trim();
                        }
                    }

                    let mut req = Requirement::new(&id, description);
                    if let Some(p) = priority {
                        req = req.with_priority(p);
                    }
                    requirements.push(req);
                }
            }

            // Also look for inline requirement markers
            if trimmed.to_lowercase().starts_with("- requirement:") {
                let requirement_text = trimmed[14..].trim();
                if !requirement_text.is_empty() {
                    requirement_counter += 1;
                    let id = format!("req-{}", requirement_counter);
                    requirements.push(Requirement::new(&id, requirement_text));
                }
            }
        }

        requirements
    }

    /// Extracts the title from content (first level-1 heading)
    fn extract_title(&self, content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(captures) = self.heading_regex.captures(trimmed) {
                let level = captures.get(1).unwrap().as_str().len();
                if level == 1 {
                    return Some(captures.get(2).unwrap().as_str().trim().to_string());
                }
            }
        }
        None
    }

    /// Extracts description (content before first heading or after title)
    fn extract_description(&self, content: &str) -> Option<String> {
        let mut description_lines = Vec::new();
        let mut found_title = false;
        let mut in_description = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if let Some(captures) = self.heading_regex.captures(trimmed) {
                let level = captures.get(1).unwrap().as_str().len();
                if level == 1 {
                    found_title = true;
                    in_description = true;
                    continue;
                } else if found_title {
                    // Stop at next heading
                    break;
                }
            }

            if in_description && !trimmed.is_empty() {
                description_lines.push(trimmed);
            }
        }

        if description_lines.is_empty() {
            None
        } else {
            Some(description_lines.join("\n"))
        }
    }
}

impl ArchitectureParser for MarkdownArchitectureParser {
    type Error = FileIoError;

    fn parse(&self, content: &str) -> Result<ArchitectureDocument, Self::Error> {
        if content.trim().is_empty() {
            return Err(FileIoError::invalid_format("content", "Content is empty"));
        }

        // Extract title
        let title = self
            .extract_title(content)
            .ok_or_else(|| FileIoError::missing_section("content", "title (level-1 heading)"))?;

        // Create document
        let mut document = ArchitectureDocument::new(title);

        // Extract description
        if let Some(description) = self.extract_description(content) {
            document = document.with_description(description);
        }

        // Parse sections
        let sections = self.parse_sections(content);
        for section in sections {
            document.add_section(section);
        }

        // Extract components
        let components = self.extract_components(content);
        for component in components {
            document.add_component(component);
        }

        // Extract requirements
        let requirements = self.extract_requirements(content);
        for requirement in requirements {
            document.add_requirement(requirement);
        }

        // Validate the document
        document
            .validate()
            .map_err(|e| FileIoError::invalid_format("content", e))?;

        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_parser() {
        let parser = MarkdownArchitectureParser::new();
        assert!(format!("{:?}", parser).contains("MarkdownArchitectureParser"));
    }

    #[test]
    fn test_parse_empty_content() {
        let parser = MarkdownArchitectureParser::new();
        let result = parser.parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_title() {
        let parser = MarkdownArchitectureParser::new();
        let content = "This is just text without a title.";
        let result = parser.parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_title() {
        let parser = MarkdownArchitectureParser::new();
        let content = "# My Architecture\n\nSome description.";
        let result = parser.parse(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title(), "My Architecture");
    }

    #[test]
    fn test_parse_description() {
        let parser = MarkdownArchitectureParser::new();
        let content = "# Architecture\n\nThis is the description.\nIt has multiple lines.";
        let result = parser.parse(content).unwrap();
        assert_eq!(
            result.description(),
            Some("This is the description.\nIt has multiple lines.")
        );
    }

    #[test]
    fn test_parse_sections() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

## Overview
This is an overview.

## Design
This is the design.
"#;
        let result = parser.parse(content).unwrap();
        assert_eq!(result.sections().len(), 2);
        assert_eq!(result.sections()[0].heading(), "Overview");
        assert_eq!(result.sections()[1].heading(), "Design");
    }

    #[test]
    fn test_parse_components() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

## Components

- API Server: Handles HTTP requests
- Database: Stores data
- Cache
"#;
        let result = parser.parse(content).unwrap();
        assert_eq!(result.components().len(), 3);
        assert_eq!(result.components()[0].name(), "API Server");
        assert_eq!(
            result.components()[0].description(),
            Some("Handles HTTP requests")
        );
        assert_eq!(result.components()[2].name(), "Cache");
    }

    #[test]
    fn test_parse_requirements() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

## Requirements

- Must be scalable
- Must be secure (Priority: High)
- Should be fast
"#;
        let result = parser.parse(content).unwrap();
        assert_eq!(result.requirements().len(), 3);
        assert_eq!(result.requirements()[0].description(), "Must be scalable");
        assert_eq!(result.requirements()[1].priority(), Some("High"));
    }

    #[test]
    fn test_parse_inline_components() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

- Component: Web Server
- Component: Database
"#;
        let result = parser.parse(content).unwrap();
        assert_eq!(result.components().len(), 2);
    }

    #[test]
    fn test_parse_inline_requirements() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

- Requirement: High availability
- Requirement: Low latency
"#;
        let result = parser.parse(content).unwrap();
        assert_eq!(result.requirements().len(), 2);
    }

    #[test]
    fn test_parse_nested_sections() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

## Section 1
Content 1

### Subsection 1.1
Content 1.1

## Section 2
Content 2
"#;
        let result = parser.parse(content).unwrap();
        assert!(result.sections().len() >= 2);
    }

    #[test]
    fn test_extract_title() {
        let parser = MarkdownArchitectureParser::new();
        let content = "# My Title\n\nSome content.";
        let title = parser.extract_title(content);
        assert_eq!(title, Some("My Title".to_string()));
    }

    #[test]
    fn test_extract_title_none() {
        let parser = MarkdownArchitectureParser::new();
        let content = "## Not a level 1 heading";
        let title = parser.extract_title(content);
        assert_eq!(title, None);
    }

    #[test]
    fn test_extract_components() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
## Components
- Service A
- Service B: Does something
"#;
        let components = parser.extract_components(content);
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].name(), "Service A");
        assert_eq!(components[1].name(), "Service B");
    }

    #[test]
    fn test_extract_requirements() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
## Requirements
- Must be fast
- Should be reliable
"#;
        let requirements = parser.extract_requirements(content);
        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].description(), "Must be fast");
    }

    #[test]
    fn test_default_trait() {
        let parser = MarkdownArchitectureParser::default();
        assert!(format!("{:?}", parser).contains("MarkdownArchitectureParser"));
    }

    #[test]
    fn test_clone() {
        let parser1 = MarkdownArchitectureParser::new();
        let parser2 = parser1.clone();
        assert!(format!("{:?}", parser2).contains("MarkdownArchitectureParser"));
    }

    #[test]
    fn test_parse_complex_document() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Microservices Architecture

This document describes our microservices architecture.
It is designed for scalability and maintainability.

## Overview

The system consists of multiple independent services.

## Components

- API Gateway: Routes requests to services
- User Service: Manages user accounts
- Payment Service: Processes payments
- Notification Service

## Requirements

- Must handle 10,000 requests per second (Priority: High)
- Must be highly available (Priority: Critical)
- Should support multiple payment methods

## Design Principles

We follow these principles:
- Loose coupling
- High cohesion
"#;
        let result = parser.parse(content).unwrap();

        assert_eq!(result.title(), "Microservices Architecture");
        assert!(result.description().is_some());
        assert!(result.sections().len() >= 3);
        assert_eq!(result.components().len(), 4);
        assert_eq!(result.requirements().len(), 3);
    }

    #[test]
    fn test_parse_with_dash_separator() {
        let parser = MarkdownArchitectureParser::new();
        let content = r#"
# Architecture

## Components

- Frontend - Web application
- Backend - REST API
"#;
        let result = parser.parse(content).unwrap();
        assert_eq!(result.components().len(), 2);
        assert_eq!(result.components()[0].name(), "Frontend");
        assert_eq!(
            result.components()[0].description(),
            Some("Web application")
        );
    }
}
