// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! README.md parser for project metadata extraction
//!
//! This module provides functionality to parse README.md files and extract
//! project metadata such as name, language, type, and description.

use crate::cli::create::ProjectMetadata;
use crate::error::{Error, Result};
use std::fs;
use std::path::Path;

/// Parses README.md for project metadata
pub struct ReadmeParser;

impl ReadmeParser {
    /// Parses README.md file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to README.md
    ///
    /// # Returns
    ///
    /// Returns ProjectMetadata extracted from README
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read
    pub fn parse(path: impl AsRef<Path>) -> Result<ProjectMetadata> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| Error::file_io(path.as_ref().to_path_buf(), e))?;

        let mut metadata = ProjectMetadata::default();

        // Extract project name from first heading
        if let Some(name) = Self::extract_first_heading(&content) {
            metadata.project_name = name;
        }

        // Detect language from code blocks or badges
        metadata.language = Self::detect_language(&content);

        // Detect project type from keywords
        metadata.project_type = Self::infer_project_type(&content);

        // Extract description from first paragraph
        if let Some(desc) = Self::extract_description(&content) {
            metadata.description = Some(desc);
        }

        Ok(metadata)
    }

    /// Extracts the first heading from content
    fn extract_first_heading(content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return Some(trimmed.trim_start_matches("# ").trim().to_string());
            }
        }
        None
    }

    /// Detects primary language from content
    fn detect_language(content: &str) -> String {
        let content_lower = content.to_lowercase();

        // Check for language badges or mentions
        if content_lower.contains("rust") || content_lower.contains("cargo.toml") {
            return "rust".to_string();
        }
        if content_lower.contains("python") || content_lower.contains("setup.py") {
            return "python".to_string();
        }
        if content_lower.contains("javascript") || content_lower.contains("package.json") {
            return "javascript".to_string();
        }
        if content_lower.contains("typescript") {
            return "typescript".to_string();
        }
        if content_lower.contains("go") || content_lower.contains("go.mod") {
            return "go".to_string();
        }

        "unknown".to_string()
    }

    /// Infers project type from content
    fn infer_project_type(content: &str) -> String {
        let content_lower = content.to_lowercase();

        if content_lower.contains("cli") || content_lower.contains("command-line") {
            return "cli".to_string();
        }
        if content_lower.contains("web")
            || content_lower.contains("api")
            || content_lower.contains("server")
        {
            return "web-service".to_string();
        }
        if content_lower.contains("library") || content_lower.contains("crate") {
            return "library".to_string();
        }

        "application".to_string()
    }

    /// Extracts description from first paragraph after heading
    fn extract_description(content: &str) -> Option<String> {
        let mut found_heading = false;
        let mut description = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('#') {
                found_heading = true;
                continue;
            }

            if found_heading && !trimmed.is_empty() && !trimmed.starts_with('[') {
                description.push_str(trimmed);
                description.push(' ');

                // Stop at next heading or empty line
                if description.len() > 200 {
                    break;
                }
            }

            if found_heading && !description.is_empty() && trimmed.is_empty() {
                break;
            }
        }

        if description.is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme_parser_detect_language() {
        let content = "# My Project\n\nA Rust project using Cargo.toml";
        assert_eq!(ReadmeParser::detect_language(content), "rust");

        let content2 = "# Python App\n\nInstall with setup.py";
        assert_eq!(ReadmeParser::detect_language(content2), "python");

        let content3 = "# Unknown Project";
        assert_eq!(ReadmeParser::detect_language(content3), "unknown");
    }

    #[test]
    fn test_readme_parser_infer_project_type() {
        let content = "# CLI Tool\n\nA command-line interface";
        assert_eq!(ReadmeParser::infer_project_type(content), "cli");

        let content2 = "# Web API\n\nA REST API server";
        assert_eq!(ReadmeParser::infer_project_type(content2), "web-service");

        let content3 = "# Library\n\nA reusable crate";
        assert_eq!(ReadmeParser::infer_project_type(content3), "library");
    }

    #[test]
    fn test_readme_parser_extract_first_heading() {
        let content = "# My Project\n\nDescription here";
        assert_eq!(
            ReadmeParser::extract_first_heading(content),
            Some("My Project".to_string())
        );

        let content2 = "Some text\n## Not First\n# First Heading";
        assert_eq!(
            ReadmeParser::extract_first_heading(content2),
            Some("First Heading".to_string())
        );
    }

    #[test]
    fn test_readme_parser_extract_description() {
        let content = "# Title\n\nDescription content.\nMore description.\n\n# Next Section";
        let desc = ReadmeParser::extract_description(content).unwrap();
        assert!(desc.contains("Description content"));
        assert!(desc.contains("More description"));
    }
}
