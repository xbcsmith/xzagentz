//! Plan parser for extracting architecture and implementation plans from markdown
//!
//! This module provides parsers for extracting structured plan data from
//! markdown documents, including phases, sections, tasks, deliverables, and
//! acceptance criteria.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::plans::parser::PlanParser;
//!
//! let markdown = r#"
//! # Architecture Plan: My Project
//!
//! ## Overview
//!
//! This is the project overview.
//! "#;
//!
//! let parser = PlanParser::new();
//! let result = parser.parse_architecture_plan(markdown);
//! assert!(result.is_ok());
//! ```

use crate::error::{Error, Result};
use crate::plans::structures::{
    ArchitecturePlan, ImplementationPlan, Phase, PlanMetadata, PlanSection,
};
use regex::Regex;

/// Parser for plan documents
pub struct PlanParser {
    /// Regex for matching headers
    header_regex: Regex,
    /// Regex for matching phase headers (e.g., "## Phase 1: Foundation")
    phase_regex: Regex,
    /// Regex for matching section numbers (e.g., "### 1.1 Core Setup")
    section_number_regex: Regex,
}

impl PlanParser {
    /// Creates a new plan parser
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::parser::PlanParser;
    ///
    /// let parser = PlanParser::new();
    /// ```
    pub fn new() -> Self {
        Self {
            header_regex: Regex::new(r"^(#{1,6})\s+(.+)$").expect("Valid regex"),
            phase_regex: Regex::new(r"^##\s+Phase\s+(\d+):?\s*(.+)$").expect("Valid regex"),
            section_number_regex: Regex::new(r"^###\s+(\d+\.\d+)\s+(.+)$").expect("Valid regex"),
        }
    }

    /// Parses an architecture plan from markdown content
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to parse
    ///
    /// # Returns
    ///
    /// Returns Ok(ArchitecturePlan) if successful, Err otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::parser::PlanParser;
    ///
    /// let markdown = r#"
    /// # Architecture Plan: My Project
    ///
    /// ## Overview
    ///
    /// Project overview content.
    /// "#;
    ///
    /// let parser = PlanParser::new();
    /// let plan = parser.parse_architecture_plan(markdown);
    /// assert!(plan.is_ok());
    /// ```
    pub fn parse_architecture_plan(&self, content: &str) -> Result<ArchitecturePlan> {
        let lines: Vec<&str> = content.lines().collect();

        // Extract project name from title
        let name = self.extract_plan_title(&lines)?;

        // Create metadata
        let metadata = PlanMetadata::new(name, "1.0.0");

        // Extract overview
        let overview = self.extract_section_content(&lines, "Overview")?;

        // Extract requirements
        let requirements = self.extract_list_items(&lines, "Requirements");

        // Extract architecture
        let architecture = self.extract_list_items(&lines, "Architecture");

        // Extract components
        let components = self.extract_list_items(&lines, "Components");

        // Extract additional sections
        let sections = self.extract_sections(&lines)?;

        Ok(ArchitecturePlan {
            metadata,
            overview,
            requirements,
            architecture,
            components,
            sections,
        })
    }

    /// Parses an implementation plan from markdown content
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to parse
    ///
    /// # Returns
    ///
    /// Returns Ok(ImplementationPlan) if successful, Err otherwise
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::plans::parser::PlanParser;
    ///
    /// let markdown = r#"
    /// # Implementation Plan: My Project
    ///
    /// ## Overview
    ///
    /// Implementation overview.
    ///
    /// ## Phase 1: Foundation
    ///
    /// Setup project structure.
    ///
    /// ### 1.1 Core Setup
    ///
    /// Initialize the project.
    /// "#;
    ///
    /// let parser = PlanParser::new();
    /// let _plan = parser.parse_implementation_plan(markdown);
    /// ```
    pub fn parse_implementation_plan(&self, content: &str) -> Result<ImplementationPlan> {
        let lines: Vec<&str> = content.lines().collect();

        // Extract project name
        let name = self.extract_plan_title(&lines)?;

        // Create metadata
        let metadata = PlanMetadata::new(name, "1.0.0");

        // Extract overview
        let overview = self.extract_section_content(&lines, "Overview")?;

        // Extract architecture summary if present
        let architecture_summary = self
            .extract_section_content(&lines, "Architecture Summary")
            .ok();

        // Extract phases
        let phases = self.extract_phases(&lines)?;

        // Extract success metrics
        let success_metrics = self.extract_list_items(&lines, "Success Metrics");

        // Extract risks
        let risks = self.extract_list_items(&lines, "Risk Mitigation");

        Ok(ImplementationPlan {
            metadata,
            overview,
            architecture_summary,
            phases,
            success_metrics,
            risks,
        })
    }

    /// Extracts the plan title from the first level 1 header
    fn extract_plan_title(&self, lines: &[&str]) -> Result<String> {
        for line in lines {
            if let Some(caps) = self.header_regex.captures(line) {
                if caps.get(1).is_some_and(|m| m.as_str() == "#") {
                    let title = caps.get(2).map_or("", |m| m.as_str());
                    // Remove prefix like "Architecture Plan: " or "Implementation Plan: "
                    let name = title.split(':').nth(1).unwrap_or(title).trim().to_string();
                    return Ok(name);
                }
            }
        }
        Err(Error::PlanParse {
            name: "unknown".to_string(),
            reason: "No plan title found".to_string(),
        })
    }

    /// Extracts content from a specific section
    fn extract_section_content(&self, lines: &[&str], section_name: &str) -> Result<String> {
        let mut in_section = false;
        let mut content = Vec::new();
        let mut section_level = 0;

        for line in lines {
            if let Some(caps) = self.header_regex.captures(line) {
                let level = caps.get(1).map_or(0, |m| m.as_str().len());
                let title = caps.get(2).map_or("", |m| m.as_str());

                if title.contains(section_name) {
                    in_section = true;
                    section_level = level;
                    continue;
                } else if in_section && level <= section_level {
                    // Found next section at same or higher level
                    break;
                }
            }

            if in_section {
                content.push(*line);
            }
        }

        if content.is_empty() {
            Err(Error::PlanParse {
                name: section_name.to_string(),
                reason: "Section not found".to_string(),
            })
        } else {
            Ok(content.join("\n").trim().to_string())
        }
    }

    /// Extracts list items from a section
    fn extract_list_items(&self, lines: &[&str], section_name: &str) -> Vec<String> {
        let mut in_section = false;
        let mut items = Vec::new();
        let mut section_level = 0;

        for line in lines {
            if let Some(caps) = self.header_regex.captures(line) {
                let level = caps.get(1).map_or(0, |m| m.as_str().len());
                let title = caps.get(2).map_or("", |m| m.as_str());

                if title.contains(section_name) {
                    in_section = true;
                    section_level = level;
                    continue;
                } else if in_section && level <= section_level {
                    break;
                }
            }

            if in_section {
                let trimmed = line.trim();
                if trimmed.starts_with('-') || trimmed.starts_with('*') {
                    let item = trimmed.trim_start_matches(['-', '*']).trim();
                    if !item.is_empty() {
                        items.push(item.to_string());
                    }
                } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) {
                    // Numbered list item
                    if let Some(idx) = trimmed.find('.') {
                        let item = trimmed[idx + 1..].trim();
                        if !item.is_empty() {
                            items.push(item.to_string());
                        }
                    }
                }
            }
        }

        items
    }

    /// Extracts phases from implementation plan
    fn extract_phases(&self, lines: &[&str]) -> Result<Vec<Phase>> {
        let mut phases = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            if let Some(caps) = self.phase_regex.captures(line) {
                let number = caps
                    .get(1)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(0);
                let title = caps.get(2).map_or("", |m| m.as_str()).to_string();

                // Extract phase content
                let phase_end = self.find_next_phase(lines, i + 1);
                let phase_lines = &lines[i + 1..phase_end];

                // Extract goal (first paragraph after phase header)
                let goal = self.extract_phase_goal(phase_lines);

                // Extract duration if present
                let duration = self.extract_phase_duration(phase_lines);

                // Extract sections within this phase
                let sections = self.extract_phase_sections(phase_lines)?;

                let phase = Phase {
                    number,
                    title,
                    duration,
                    goal,
                    sections,
                    dependencies: Vec::new(),
                };

                phases.push(phase);
                i = phase_end;
            } else {
                i += 1;
            }
        }

        if phases.is_empty() {
            Err(Error::PlanParse {
                name: "implementation".to_string(),
                reason: "No phases found".to_string(),
            })
        } else {
            Ok(phases)
        }
    }

    /// Finds the index of the next phase header
    fn find_next_phase(&self, lines: &[&str], start: usize) -> usize {
        for (i, line) in lines[start..].iter().enumerate() {
            if self.phase_regex.is_match(line) {
                return start + i;
            }
        }
        lines.len()
    }

    /// Extracts the goal from phase content (first paragraph)
    fn extract_phase_goal(&self, lines: &[&str]) -> String {
        let mut goal_lines = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() && !goal_lines.is_empty() {
                // End of first paragraph
                break;
            }
            if !trimmed.is_empty() && !self.header_regex.is_match(line) {
                goal_lines.push(trimmed);
            }
        }

        goal_lines.join(" ")
    }

    /// Extracts duration from phase content
    fn extract_phase_duration(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            let lower = line.to_lowercase();
            if lower.contains("duration") || lower.contains("week") || lower.contains("day") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 1 {
                    return Some(parts[1].trim().to_string());
                }
            }
        }
        None
    }

    /// Extracts sections within a phase
    fn extract_phase_sections(&self, lines: &[&str]) -> Result<Vec<PlanSection>> {
        let mut sections = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            if let Some(caps) = self.section_number_regex.captures(line) {
                let number = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let title = caps.get(2).map_or("", |m| m.as_str()).to_string();

                // Find end of this section
                let section_end = self.find_next_section(lines, i + 1);
                let section_lines = &lines[i + 1..section_end];

                // Extract content
                let content = section_lines.join("\n").trim().to_string();

                // Extract tasks
                let tasks = self.extract_subsection_items(section_lines, "Tasks");

                // Extract deliverables
                let deliverables = self.extract_subsection_items(section_lines, "Deliverables");

                // Extract acceptance criteria
                let acceptance_criteria =
                    self.extract_subsection_items(section_lines, "Acceptance Criteria");

                // Extract tests
                let tests = self.extract_subsection_items(section_lines, "Testing");

                sections.push(PlanSection {
                    number,
                    title,
                    content,
                    tasks,
                    deliverables,
                    acceptance_criteria,
                    tests,
                });

                i = section_end;
            } else {
                i += 1;
            }
        }

        Ok(sections)
    }

    /// Finds the index of the next section header
    fn find_next_section(&self, lines: &[&str], start: usize) -> usize {
        for (i, line) in lines[start..].iter().enumerate() {
            if self.section_number_regex.is_match(line)
                || self.header_regex.is_match(line) && line.starts_with("##")
            {
                return start + i;
            }
        }
        lines.len()
    }

    /// Extracts items from a subsection within a section
    fn extract_subsection_items(&self, lines: &[&str], subsection_name: &str) -> Vec<String> {
        let mut in_subsection = false;
        let mut items = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            // Check for subsection header
            if trimmed.starts_with("**") && trimmed.contains(subsection_name) {
                in_subsection = true;
                continue;
            }

            // Check if we've left the subsection
            if in_subsection && trimmed.starts_with("**") && !trimmed.contains(subsection_name) {
                break;
            }

            if in_subsection {
                if trimmed.starts_with('-') || trimmed.starts_with('*') {
                    let item = trimmed.trim_start_matches(['-', '*']).trim();
                    if !item.is_empty() {
                        items.push(item.to_string());
                    }
                } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) {
                    if let Some(idx) = trimmed.find('.') {
                        let item = trimmed[idx + 1..].trim();
                        if !item.is_empty() {
                            items.push(item.to_string());
                        }
                    }
                }
            }
        }

        items
    }

    /// Extracts generic sections from content
    fn extract_sections(&self, lines: &[&str]) -> Result<Vec<PlanSection>> {
        let mut sections = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            if let Some(caps) = self.section_number_regex.captures(line) {
                let number = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let title = caps.get(2).map_or("", |m| m.as_str()).to_string();

                let section_end = self.find_next_section(lines, i + 1);
                let section_lines = &lines[i + 1..section_end];
                let content = section_lines.join("\n").trim().to_string();

                sections.push(PlanSection::new(number, title, content));

                i = section_end;
            } else {
                i += 1;
            }
        }

        Ok(sections)
    }
}

impl Default for PlanParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_architecture_plan() {
        let markdown = r#"
# Architecture Plan: Test Project

## Overview

This is a test project overview.

## Requirements

- Requirement 1
- Requirement 2

## Architecture

- Layered architecture
- Microservices

## Components

- API Layer
- Database Layer
"#;

        let parser = PlanParser::new();
        let result = parser.parse_architecture_plan(markdown);
        assert!(result.is_ok());

        let plan = result.unwrap();
        assert_eq!(plan.metadata.name, "Test Project");
        assert!(plan.overview.contains("test project"));
        assert!(plan.requirements.len() >= 2);
        assert!(plan.architecture.len() >= 2);
        assert!(plan.components.len() >= 2);
    }

    #[test]
    fn test_parse_implementation_plan() {
        let markdown = r#"
# Implementation Plan: Test Project

## Overview

Implementation overview.

## Phase 1: Foundation

Setup project structure.

Duration: 1 week

### 1.1 Core Setup

Initialize the project.

**Tasks**:

- Create Cargo project
- Setup dependencies

**Deliverables**:

- Cargo.toml
- README.md

**Acceptance Criteria**:

- Project compiles
- Tests pass

## Phase 2: Development

Build core features.
"#;

        let parser = PlanParser::new();
        let result = parser.parse_implementation_plan(markdown);
        assert!(result.is_ok());

        let plan = result.unwrap();
        assert_eq!(plan.metadata.name, "Test Project");
        assert_eq!(plan.phases.len(), 2);

        let phase1 = &plan.phases[0];
        assert_eq!(phase1.number, 1);
        assert_eq!(phase1.title, "Foundation");
        assert_eq!(phase1.duration, Some("1 week".to_string()));
        assert_eq!(phase1.sections.len(), 1);

        let section = &phase1.sections[0];
        assert_eq!(section.number, "1.1");
        assert_eq!(section.title, "Core Setup");
        assert_eq!(section.tasks.len(), 2);
        assert_eq!(section.deliverables.len(), 2);
        assert_eq!(section.acceptance_criteria.len(), 2);
    }

    #[test]
    fn test_extract_phases() {
        let markdown = r#"
## Phase 1: Foundation

Setup project.

## Phase 2: Core Development

Build features.
"#;

        let lines: Vec<&str> = markdown.lines().collect();
        let parser = PlanParser::new();
        let result = parser.extract_phases(&lines);
        assert!(result.is_ok());

        let phases = result.unwrap();
        assert_eq!(phases.len(), 2);
        assert_eq!(phases[0].number, 1);
        assert_eq!(phases[1].number, 2);
    }

    #[test]
    fn test_extract_sections() {
        let markdown = r#"
### 1.1 Setup

Initialize project.

### 1.2 Configuration

Configure settings.
"#;

        let lines: Vec<&str> = markdown.lines().collect();
        let parser = PlanParser::new();
        let result = parser.extract_sections(&lines);
        assert!(result.is_ok());

        let sections = result.unwrap();
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].number, "1.1");
        assert_eq!(sections[1].number, "1.2");
    }

    #[test]
    fn test_parse_deliverables() {
        let lines = vec!["**Deliverables**:", "", "- File 1", "- File 2", "- File 3"];

        let parser = PlanParser::new();
        let items = parser.extract_subsection_items(&lines, "Deliverables");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "File 1");
    }

    #[test]
    fn test_parse_acceptance_criteria() {
        let lines = vec![
            "**Acceptance Criteria**:",
            "",
            "- All tests pass",
            "- Code coverage > 80%",
        ];

        let parser = PlanParser::new();
        let items = parser.extract_subsection_items(&lines, "Acceptance Criteria");
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_handle_malformed_markdown() {
        let markdown = r#"
# Malformed Plan

This has no proper structure.
"#;

        let parser = PlanParser::new();
        let result = parser.parse_implementation_plan(markdown);
        assert!(result.is_err());
    }
}
