// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Planning Service
//!
//! This module provides the `PlanningService` which orchestrates the workflow
//! of parsing architecture documents, generating implementation plans, and
//! writing plans to disk.

use crate::application::error::{ApplicationError, Result};
use crate::domain::planning::{
    ArchitectureDocument, ArchitectureParser, Plan, PlanGenerator, PlanOptions, PlanWriter,
};
use std::path::Path;

/// Planning Service
///
/// Orchestrates the complete planning workflow:
/// 1. Parse architecture document from file
/// 2. Generate implementation plan using LLM
/// 3. Write plan to output file
///
/// # Type Parameters
///
/// - `G`: Plan generator implementation (e.g., OllamaPlanGenerator)
/// - `P`: Architecture parser implementation (e.g., MarkdownArchitectureParser)
/// - `W`: Plan writer implementation (e.g., MarkdownPlanWriter)
///
/// # Examples
///
/// ```no_run
/// use xzagentz::application::PlanningService;
/// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaPlanGenerator, OllamaConfig};
/// use xzagentz::infrastructure::fileio::{MarkdownArchitectureParser, MarkdownPlanWriter};
/// use xzagentz::domain::planning::PlanOptions;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = OllamaConfig::default();
/// let client = OllamaClient::new(config)?;
/// let generator = OllamaPlanGenerator::new(client);
/// let parser = MarkdownArchitectureParser::new();
/// let writer = MarkdownPlanWriter::new();
///
/// let service = PlanningService::new(generator, parser, writer);
///
/// let options = PlanOptions::new()
///     .with_model("llama3")
///     .with_num_phases(5);
///
/// let plan = service.generate_implementation_plan(
///     "architecture.md",
///     &options,
/// )?;
///
/// service.save_plan(&plan, "implementation_plan.md", false)?;
/// # Ok(())
/// # }
/// ```
pub struct PlanningService<G, P, W>
where
    G: PlanGenerator,
    P: ArchitectureParser,
    W: PlanWriter,
{
    generator: G,
    parser: P,
    writer: W,
}

impl<G, P, W> PlanningService<G, P, W>
where
    G: PlanGenerator,
    P: ArchitectureParser,
    W: PlanWriter,
{
    /// Creates a new planning service
    ///
    /// # Arguments
    ///
    /// * `generator` - Implementation of PlanGenerator trait
    /// * `parser` - Implementation of ArchitectureParser trait
    /// * `writer` - Implementation of PlanWriter trait
    ///
    /// # Returns
    ///
    /// A new `PlanningService` instance
    pub fn new(generator: G, parser: P, writer: W) -> Self {
        Self {
            generator,
            parser,
            writer,
        }
    }

    /// Parses an architecture document from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the architecture markdown file
    ///
    /// # Returns
    ///
    /// Returns the parsed `ArchitectureDocument`
    ///
    /// # Errors
    ///
    /// Returns `ApplicationError::ReadArchitectureError` if file cannot be read
    /// Returns `ApplicationError::ParseError` if document cannot be parsed
    pub fn parse_architecture_file(&self, path: impl AsRef<Path>) -> Result<ArchitectureDocument> {
        let path = path.as_ref();

        // Check if file exists
        if !path.exists() {
            return Err(ApplicationError::ArchitectureNotFound(
                path.display().to_string(),
            ));
        }

        // Read file contents
        let content =
            std::fs::read_to_string(path).map_err(|e| ApplicationError::ReadArchitectureError {
                path: path.display().to_string(),
                source: e,
            })?;

        // Parse the content
        self.parser
            .parse(&content)
            .map_err(|e| ApplicationError::ParseError(e.to_string()))
    }

    /// Generates an implementation plan from an architecture document
    ///
    /// # Arguments
    ///
    /// * `architecture_path` - Path to the architecture markdown file
    /// * `options` - Configuration options for plan generation
    ///
    /// # Returns
    ///
    /// Returns the generated `Plan`
    ///
    /// # Errors
    ///
    /// Returns `ApplicationError` if any step fails:
    /// - File reading
    /// - Parsing
    /// - Plan generation
    /// - Plan validation
    pub fn generate_implementation_plan(
        &self,
        architecture_path: impl AsRef<Path>,
        options: &PlanOptions,
    ) -> Result<Plan> {
        // Parse architecture document
        let architecture = self.parse_architecture_file(architecture_path)?;

        // Validate architecture has content
        if architecture.title().is_empty() && architecture.sections().is_empty() {
            return Err(ApplicationError::InvalidArchitecture(
                "Architecture document is empty".to_string(),
            ));
        }

        // Generate plan using the generator
        let plan = self
            .generator
            .generate_plan(&architecture, options)
            .map_err(|e| ApplicationError::GenerationError(e.to_string()))?;

        // Validate plan has phases
        if plan.phases().is_empty() {
            return Err(ApplicationError::ValidationError(
                "Generated plan has no phases".to_string(),
            ));
        }

        Ok(plan)
    }

    /// Saves a plan to a file
    ///
    /// # Arguments
    ///
    /// * `plan` - The plan to write
    /// * `output_path` - Path where the plan should be written
    /// * `force` - If true, overwrite existing file; if false, return error if file exists
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success
    ///
    /// # Errors
    ///
    /// Returns `ApplicationError::OutputExists` if file exists and force is false
    /// Returns `ApplicationError::WriteError` if writing fails
    pub fn save_plan(&self, plan: &Plan, output_path: impl AsRef<Path>, force: bool) -> Result<()> {
        let path = output_path.as_ref();

        // Check if output file exists
        if path.exists() && !force {
            return Err(ApplicationError::OutputExists(path.display().to_string()));
        }

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    ApplicationError::WriteError(format!(
                        "Failed to create parent directory: {}",
                        e
                    ))
                })?;
            }
        }

        // Write the plan
        self.writer
            .write_plan(plan, path)
            .map_err(|e| ApplicationError::WriteError(e.to_string()))?;

        Ok(())
    }

    /// Generates and saves an implementation plan in one operation
    ///
    /// This is a convenience method that combines `generate_implementation_plan`
    /// and `save_plan`.
    ///
    /// # Arguments
    ///
    /// * `architecture_path` - Path to the architecture markdown file
    /// * `options` - Configuration options for plan generation
    /// * `output_path` - Path where the plan should be written
    /// * `force` - If true, overwrite existing file
    ///
    /// # Returns
    ///
    /// Returns the generated `Plan`
    ///
    /// # Errors
    ///
    /// Returns `ApplicationError` if generation or writing fails
    pub fn generate_and_save_plan(
        &self,
        architecture_path: impl AsRef<Path>,
        options: &PlanOptions,
        output_path: impl AsRef<Path>,
        force: bool,
    ) -> Result<Plan> {
        let plan = self.generate_implementation_plan(architecture_path, options)?;
        self.save_plan(&plan, output_path, force)?;
        Ok(plan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::planning::{Phase, PlanMetadata, Section, Task};
    use chrono::Utc;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    // Mock implementations for testing
    struct MockParser {
        should_fail: bool,
    }

    impl ArchitectureParser for MockParser {
        type Error = std::io::Error;

        fn parse(&self, content: &str) -> std::result::Result<ArchitectureDocument, Self::Error> {
            if self.should_fail {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "mock parse error",
                ));
            }

            let mut doc = ArchitectureDocument::new("Test Architecture");
            doc.add_section(Section::new("Introduction", 2).with_content(content));
            Ok(doc)
        }
    }

    struct MockGenerator {
        should_fail: bool,
    }

    impl PlanGenerator for MockGenerator {
        type Error = std::io::Error;

        fn generate_plan(
            &self,
            _architecture: &ArchitectureDocument,
            options: &PlanOptions,
        ) -> std::result::Result<Plan, Self::Error> {
            if self.should_fail {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "mock generation error",
                ));
            }

            let task = Task::new("task-1", "Test task description");

            let phase = Phase::new("phase-1", "Test Phase", "Test phase description")
                .with_tasks(vec![task]);

            let model = options.model().unwrap_or("test-model");
            let metadata = PlanMetadata::new(Utc::now(), model, PathBuf::from("test.md"));

            Ok(Plan::new(
                "Test Plan",
                "Test plan description",
                vec![phase],
                metadata,
            ))
        }
    }

    struct MockWriter {
        should_fail: bool,
    }

    impl PlanWriter for MockWriter {
        type Error = std::io::Error;

        fn write_plan(&self, _plan: &Plan, _path: &Path) -> std::result::Result<(), Self::Error> {
            if self.should_fail {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "mock write error",
                ));
            }
            Ok(())
        }
    }

    #[test]
    fn test_new_planning_service() {
        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let _service = PlanningService::new(generator, parser, writer);
    }

    #[test]
    fn test_parse_architecture_file_not_found() {
        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let result = service.parse_architecture_file("/nonexistent/file.md");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::ArchitectureNotFound(_)
        ));
    }

    #[test]
    fn test_parse_architecture_file_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test Architecture").unwrap();
        writeln!(temp_file, "Some content").unwrap();
        temp_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let result = service.parse_architecture_file(temp_file.path());
        assert!(result.is_ok());

        let arch = result.unwrap();
        assert_eq!(arch.title(), "Test Architecture");
    }

    #[test]
    fn test_parse_architecture_file_parse_error() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test Architecture").unwrap();
        temp_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: true };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let result = service.parse_architecture_file(temp_file.path());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::ParseError(_)
        ));
    }

    #[test]
    fn test_generate_implementation_plan_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test Architecture").unwrap();
        writeln!(temp_file, "Content").unwrap();
        temp_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);
        let options = PlanOptions::new().with_model("test-model");

        let result = service.generate_implementation_plan(temp_file.path(), &options);

        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.title(), "Test Plan");
        assert_eq!(plan.phases().len(), 1);
    }

    #[test]
    fn test_generate_implementation_plan_parse_error() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test Architecture").unwrap();
        temp_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: true };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);
        let options = PlanOptions::new();

        let result = service.generate_implementation_plan(temp_file.path(), &options);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_implementation_plan_generation_error() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test Architecture").unwrap();
        writeln!(temp_file, "Content").unwrap();
        temp_file.flush().unwrap();

        let generator = MockGenerator { should_fail: true };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);
        let options = PlanOptions::new();

        let result = service.generate_implementation_plan(temp_file.path(), &options);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::GenerationError(_)
        ));
    }

    #[test]
    fn test_generate_implementation_plan_with_num_phases() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Test Architecture").unwrap();
        writeln!(temp_file, "Content").unwrap();
        temp_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);
        let options = PlanOptions::new().with_num_phases(3);

        let result = service.generate_implementation_plan(temp_file.path(), &options);

        assert!(result.is_ok());
    }

    #[test]
    fn test_save_plan_success() {
        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("source.md"));
        let plan = Plan::new("Test Plan", "Description", vec![], metadata);

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("plan.md");

        let result = service.save_plan(&plan, &output_path, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_plan_file_exists_no_force() {
        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("source.md"));
        let plan = Plan::new("Test Plan", "Description", vec![], metadata);

        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "existing content").unwrap();
        temp_file.flush().unwrap();

        let result = service.save_plan(&plan, temp_file.path(), false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::OutputExists(_)
        ));
    }

    #[test]
    fn test_save_plan_file_exists_with_force() {
        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("source.md"));
        let plan = Plan::new("Test Plan", "Description", vec![], metadata);

        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "existing content").unwrap();
        temp_file.flush().unwrap();

        let result = service.save_plan(&plan, temp_file.path(), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_plan_write_error() {
        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: true };

        let service = PlanningService::new(generator, parser, writer);

        let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("source.md"));
        let plan = Plan::new("Test Plan", "Description", vec![], metadata);

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("plan.md");

        let result = service.save_plan(&plan, &output_path, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::WriteError(_)
        ));
    }

    #[test]
    fn test_generate_and_save_plan_success() {
        let mut arch_file = NamedTempFile::new().unwrap();
        writeln!(arch_file, "# Test Architecture").unwrap();
        writeln!(arch_file, "Content").unwrap();
        arch_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("plan.md");
        let options = PlanOptions::new().with_model("test-model");

        let result =
            service.generate_and_save_plan(arch_file.path(), &options, &output_path, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_and_save_plan_generation_fails() {
        let mut arch_file = NamedTempFile::new().unwrap();
        writeln!(arch_file, "# Test Architecture").unwrap();
        writeln!(arch_file, "Content").unwrap();
        arch_file.flush().unwrap();

        let generator = MockGenerator { should_fail: true };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: false };

        let service = PlanningService::new(generator, parser, writer);

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("plan.md");
        let options = PlanOptions::new();

        let result =
            service.generate_and_save_plan(arch_file.path(), &options, &output_path, false);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_and_save_plan_write_fails() {
        let mut arch_file = NamedTempFile::new().unwrap();
        writeln!(arch_file, "# Test Architecture").unwrap();
        writeln!(arch_file, "Content").unwrap();
        arch_file.flush().unwrap();

        let generator = MockGenerator { should_fail: false };
        let parser = MockParser { should_fail: false };
        let writer = MockWriter { should_fail: true };

        let service = PlanningService::new(generator, parser, writer);

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("plan.md");
        let options = PlanOptions::new();

        let result =
            service.generate_and_save_plan(arch_file.path(), &options, &output_path, false);

        assert!(result.is_err());
    }
}
