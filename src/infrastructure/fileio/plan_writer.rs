//! Plan writer implementation for markdown format
//!
//! This module implements the PlanWriter trait to serialize and write
//! implementation plans to markdown files with atomic file operations.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::infrastructure::fileio::MarkdownPlanWriter;
//! use xzagentz::domain::planning::{PlanWriter, Plan, PlanMetadata};
//! use std::path::Path;
//! use chrono::Utc;
//! use std::path::PathBuf;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let writer = MarkdownPlanWriter::new();
//!
//! let metadata = PlanMetadata::new(
//!     Utc::now(),
//!     "llama3.2:3b",
//!     PathBuf::from("architecture.md"),
//! );
//! let plan = Plan::new(
//!     "My Implementation Plan",
//!     "A comprehensive plan",
//!     vec![],
//!     metadata,
//! );
//!
//! writer.write_plan(&plan, Path::new("output.md"))?;
//! # Ok(())
//! # }
//! ```

use super::error::FileIoError;
use crate::domain::planning::{Plan, PlanWriter};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Markdown plan writer
///
/// Implements the PlanWriter trait to serialize implementation plans
/// as markdown documents with proper structure and formatting.
///
/// # Features
///
/// - Atomic file writes (temp file + rename)
/// - Proper markdown formatting with headings and lists
/// - Automatic parent directory creation
/// - Safe error handling
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::infrastructure::fileio::MarkdownPlanWriter;
/// use xzagentz::domain::planning::{PlanWriter, Plan, PlanMetadata};
/// use std::path::Path;
/// use chrono::Utc;
/// use std::path::PathBuf;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let writer = MarkdownPlanWriter::new();
/// let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("arch.md"));
/// let plan = Plan::new("Title", "Description", vec![], metadata);
///
/// writer.write_plan(&plan, Path::new("plan.md"))?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct MarkdownPlanWriter;

impl MarkdownPlanWriter {
    /// Creates a new markdown plan writer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::MarkdownPlanWriter;
    ///
    /// let writer = MarkdownPlanWriter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Formats a plan as markdown
    ///
    /// # Arguments
    ///
    /// * `plan` - The plan to format
    ///
    /// # Returns
    ///
    /// Returns a formatted markdown string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::fileio::MarkdownPlanWriter;
    /// use xzagentz::domain::planning::{Plan, PlanMetadata};
    /// use chrono::Utc;
    /// use std::path::PathBuf;
    ///
    /// let writer = MarkdownPlanWriter::new();
    /// let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("arch.md"));
    /// let plan = Plan::new("Test Plan", "Description", vec![], metadata);
    ///
    /// let markdown = writer.format_plan(&plan);
    /// assert!(markdown.contains("# Test Plan"));
    /// ```
    pub fn format_plan(&self, plan: &Plan) -> String {
        let mut output = String::new();

        // Title
        output.push_str(&format!("# {}\n\n", plan.title()));

        // Description
        output.push_str(&format!("{}\n\n", plan.description()));

        // Metadata
        output.push_str("## Metadata\n\n");
        output.push_str(&format!(
            "- **Generated**: {}\n",
            plan.metadata().generated_at()
        ));
        output.push_str(&format!("- **Model**: {}\n", plan.metadata().model_used()));
        output.push_str(&format!(
            "- **Source**: {}\n",
            plan.metadata().source_document().display()
        ));
        output.push_str(&format!("- **Version**: {}\n\n", plan.metadata().version()));

        // Phases
        output.push_str("## Implementation Phases\n\n");
        output.push_str(&format!("Total phases: {}\n\n", plan.phases().len()));

        for phase in plan.phases() {
            output.push_str(&format!("### Phase: {}\n\n", phase.name()));
            output.push_str(&format!("**ID**: {}\n\n", phase.id()));
            output.push_str(&format!("**Description**: {}\n\n", phase.description()));

            if let Some(duration) = phase.estimated_duration() {
                output.push_str(&format!("**Estimated Duration**: {}\n\n", duration));
            }

            if !phase.dependencies().is_empty() {
                output.push_str("**Dependencies**:\n");
                for dep in phase.dependencies() {
                    output.push_str(&format!("- {}\n", dep));
                }
                output.push('\n');
            }

            output.push_str("**Tasks**:\n\n");
            for task in phase.tasks() {
                output.push_str(&format!("#### {}\n\n", task.name()));
                output.push_str(&format!("{}\n\n", task.description()));

                if !task.acceptance_criteria().is_empty() {
                    output.push_str("**Acceptance Criteria**:\n");
                    for criterion in task.acceptance_criteria() {
                        output.push_str(&format!("- {}\n", criterion));
                    }
                    output.push('\n');
                }

                if !task.components().is_empty() {
                    output.push_str("**Components**:\n");
                    for component in task.components() {
                        output.push_str(&format!("- `{}`\n", component));
                    }
                    output.push('\n');
                }
            }
        }

        output
    }

    /// Writes content to a file atomically
    ///
    /// Uses a temporary file and atomic rename to ensure the file is either
    /// fully written or not written at all, preventing partial writes.
    fn write_atomic(&self, path: &Path, content: &str) -> Result<(), FileIoError> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| FileIoError::DirectoryCreationError {
                    path: parent.display().to_string(),
                    message: e.to_string(),
                })?;
            }
        }

        // Create temporary file path
        let temp_path = path.with_extension("tmp");

        // Write to temporary file
        let mut file = fs::File::create(&temp_path).map_err(|e| {
            FileIoError::write_error(temp_path.display().to_string(), e.to_string())
        })?;

        file.write_all(content.as_bytes()).map_err(|e| {
            FileIoError::write_error(temp_path.display().to_string(), e.to_string())
        })?;

        // Sync to disk
        file.sync_all().map_err(|e| {
            FileIoError::write_error(temp_path.display().to_string(), e.to_string())
        })?;

        // Atomic rename
        fs::rename(&temp_path, path)
            .map_err(|e| FileIoError::write_error(path.display().to_string(), e.to_string()))?;

        Ok(())
    }
}

impl PlanWriter for MarkdownPlanWriter {
    type Error = FileIoError;

    fn write_plan(&self, plan: &Plan, output_path: &Path) -> Result<(), Self::Error> {
        // Validate plan before writing
        plan.validate().map_err(|e| {
            FileIoError::invalid_format(
                output_path.display().to_string(),
                format!("Invalid plan: {:?}", e),
            )
        })?;

        // Format as markdown
        let markdown = self.format_plan(plan);

        // Write atomically
        self.write_atomic(output_path, &markdown)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::planning::{Phase, PlanMetadata, Task};
    use chrono::Utc;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_plan() -> Plan {
        let metadata =
            PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("architecture.md"));

        let task = Task::new("Setup database", "Initialize PostgreSQL")
            .with_acceptance_criteria(vec!["DB running".to_string()])
            .with_components(vec!["src/db.rs".to_string()]);

        let phase = Phase::new("phase-1", "Setup", "Initial setup").with_tasks(vec![task]);

        Plan::new(
            "Test Plan",
            "A test implementation plan",
            vec![phase],
            metadata,
        )
    }

    #[test]
    fn test_new_writer() {
        let writer = MarkdownPlanWriter::new();
        assert!(format!("{:?}", writer).contains("MarkdownPlanWriter"));
    }

    #[test]
    fn test_format_plan_title() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("# Test Plan"));
    }

    #[test]
    fn test_format_plan_description() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("A test implementation plan"));
    }

    #[test]
    fn test_format_plan_metadata() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("## Metadata"));
        assert!(markdown.contains("**Model**: llama3.2:3b"));
        assert!(markdown.contains("**Source**: architecture.md"));
    }

    #[test]
    fn test_format_plan_phases() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("## Implementation Phases"));
        assert!(markdown.contains("Total phases: 1"));
        assert!(markdown.contains("### Phase: Setup"));
    }

    #[test]
    fn test_format_plan_tasks() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("#### Setup database"));
        assert!(markdown.contains("Initialize PostgreSQL"));
    }

    #[test]
    fn test_format_plan_acceptance_criteria() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("**Acceptance Criteria**:"));
        assert!(markdown.contains("- DB running"));
    }

    #[test]
    fn test_format_plan_components() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("**Components**:"));
        assert!(markdown.contains("- `src/db.rs`"));
    }

    #[test]
    fn test_write_plan_creates_file() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("plan.md");

        let result = writer.write_plan(&plan, &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_write_plan_content() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("plan.md");

        writer.write_plan(&plan, &output_path).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("# Test Plan"));
        assert!(content.contains("Setup database"));
    }

    #[test]
    fn test_write_plan_creates_parent_directories() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir
            .path()
            .join("subdir")
            .join("nested")
            .join("plan.md");

        let result = writer.write_plan(&plan, &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_write_plan_overwrites_existing() {
        let writer = MarkdownPlanWriter::new();
        let plan = create_test_plan();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("plan.md");

        // Write first time
        writer.write_plan(&plan, &output_path).unwrap();

        // Write second time (should overwrite)
        let result = writer.write_plan(&plan, &output_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_trait() {
        let writer = MarkdownPlanWriter;
        assert!(format!("{:?}", writer).contains("MarkdownPlanWriter"));
    }

    #[test]
    fn test_clone() {
        let writer1 = MarkdownPlanWriter::new();
        let writer2 = writer1.clone();
        assert!(format!("{:?}", writer2).contains("MarkdownPlanWriter"));
    }

    #[test]
    fn test_format_plan_with_dependencies() {
        let writer = MarkdownPlanWriter::new();
        let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("arch.md"));

        let phase =
            Phase::new("phase-2", "Build", "Build phase").with_dependencies(vec!["phase-1".into()]);

        let plan = Plan::new("Plan", "Description", vec![phase], metadata);
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("**Dependencies**:"));
        assert!(markdown.contains("- phase-1"));
    }

    #[test]
    fn test_format_plan_with_estimated_duration() {
        let writer = MarkdownPlanWriter::new();
        let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("arch.md"));

        let phase =
            Phase::new("phase-1", "Setup", "Setup phase").with_estimated_duration("2 weeks");

        let plan = Plan::new("Plan", "Description", vec![phase], metadata);
        let markdown = writer.format_plan(&plan);

        assert!(markdown.contains("**Estimated Duration**: 2 weeks"));
    }
}
