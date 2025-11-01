//! Core domain models for implementation planning
//!
//! This module contains the primary domain entities for representing
//! implementation plans, including the Plan aggregate root and associated
//! metadata.
//!
//! # Models
//!
//! - [`Plan`]: The aggregate root representing a complete implementation plan
//! - [`PlanMetadata`]: Metadata about plan generation and provenance
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::{Plan, PlanMetadata, Phase};
//! use chrono::Utc;
//! use std::path::PathBuf;
//!
//! let metadata = PlanMetadata::new(
//!     Utc::now(),
//!     "llama3.2:3b",
//!     PathBuf::from("architecture.md"),
//! );
//!
//! let plan = Plan::new(
//!     "Authentication System",
//!     "Implementation plan for authentication",
//!     vec![],
//!     metadata,
//! );
//!
//! assert_eq!(plan.title(), "Authentication System");
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::phase::Phase;

/// Represents a complete implementation plan
///
/// A Plan is the aggregate root that contains all phases, tasks, and metadata
/// for implementing a feature or system based on an architecture document.
///
/// # Invariants
///
/// - Title must not be empty
/// - Must contain at least one phase (validated at creation)
/// - Phase dependencies must form a valid DAG (no cycles)
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::{Plan, PlanMetadata, Phase};
/// use chrono::Utc;
/// use std::path::PathBuf;
///
/// let metadata = PlanMetadata::new(
///     Utc::now(),
///     "llama3.2:3b",
///     PathBuf::from("docs/architecture.md"),
/// );
///
/// let phase = Phase::new(
///     "phase-1",
///     "Setup Infrastructure",
///     "Initialize project infrastructure",
/// );
///
/// let plan = Plan::new(
///     "E-commerce Platform",
///     "Implementation plan for e-commerce system",
///     vec![phase],
///     metadata,
/// );
///
/// assert_eq!(plan.title(), "E-commerce Platform");
/// assert_eq!(plan.phases().len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Plan {
    title: String,
    description: String,
    phases: Vec<Phase>,
    metadata: PlanMetadata,
}

impl Plan {
    /// Creates a new Plan
    ///
    /// # Arguments
    ///
    /// * `title` - The plan title (must not be empty)
    /// * `description` - Detailed description of what the plan implements
    /// * `phases` - List of implementation phases
    /// * `metadata` - Plan generation metadata
    ///
    /// # Returns
    ///
    /// Returns a new Plan instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{Plan, PlanMetadata, Phase};
    /// use chrono::Utc;
    /// use std::path::PathBuf;
    ///
    /// let metadata = PlanMetadata::new(
    ///     Utc::now(),
    ///     "llama3.2:3b",
    ///     PathBuf::from("arch.md"),
    /// );
    ///
    /// let phase = Phase::new("phase-1", "Setup", "Initial setup");
    /// let plan = Plan::new("My Plan", "Description", vec![phase], metadata);
    ///
    /// assert_eq!(plan.title(), "My Plan");
    /// ```
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        phases: Vec<Phase>,
        metadata: PlanMetadata,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            phases,
            metadata,
        }
    }

    /// Returns the plan title
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xzagentz::domain::planning::{Plan, PlanMetadata};
    /// # use chrono::Utc;
    /// # use std::path::PathBuf;
    /// # let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("test.md"));
    /// let plan = Plan::new("Test Plan", "Description", vec![], metadata);
    /// assert_eq!(plan.title(), "Test Plan");
    /// ```
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the plan description
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xzagentz::domain::planning::{Plan, PlanMetadata};
    /// # use chrono::Utc;
    /// # use std::path::PathBuf;
    /// # let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("test.md"));
    /// let plan = Plan::new("Title", "My description", vec![], metadata);
    /// assert_eq!(plan.description(), "My description");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns a reference to the phases
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xzagentz::domain::planning::{Plan, PlanMetadata, Phase};
    /// # use chrono::Utc;
    /// # use std::path::PathBuf;
    /// # let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("test.md"));
    /// let phase = Phase::new("phase-1", "Test", "Description");
    /// let plan = Plan::new("Title", "Description", vec![phase], metadata);
    /// assert_eq!(plan.phases().len(), 1);
    /// ```
    pub fn phases(&self) -> &[Phase] {
        &self.phases
    }

    /// Returns a reference to the metadata
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xzagentz::domain::planning::{Plan, PlanMetadata};
    /// # use chrono::Utc;
    /// # use std::path::PathBuf;
    /// # let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("test.md"));
    /// let plan = Plan::new("Title", "Description", vec![], metadata.clone());
    /// assert_eq!(plan.metadata().model_used(), "llama3.2:3b");
    /// ```
    pub fn metadata(&self) -> &PlanMetadata {
        &self.metadata
    }

    /// Validates the plan structure
    ///
    /// Checks for:
    /// - Non-empty title
    /// - At least one phase
    /// - Valid phase dependencies (no cycles)
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, Err with reason if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xzagentz::domain::planning::{Plan, PlanMetadata, Phase};
    /// # use chrono::Utc;
    /// # use std::path::PathBuf;
    /// # let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("test.md"));
    /// let phase = Phase::new("phase-1", "Test", "Description");
    /// let plan = Plan::new("Title", "Description", vec![phase], metadata);
    /// assert!(plan.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Plan title cannot be empty".to_string());
        }

        if self.phases.is_empty() {
            return Err("Plan must contain at least one phase".to_string());
        }

        // Validate each phase
        for phase in &self.phases {
            phase.validate()?;
        }

        // Check for circular dependencies
        self.validate_phase_dependencies()?;

        Ok(())
    }

    /// Validates that phase dependencies form a valid DAG
    fn validate_phase_dependencies(&self) -> Result<(), String> {
        use std::collections::{HashSet, VecDeque};

        let phase_ids: HashSet<String> = self.phases.iter().map(|p| p.id().to_string()).collect();

        // Check all dependencies exist
        for phase in &self.phases {
            for dep in phase.dependencies() {
                if !phase_ids.contains(dep.as_str()) {
                    return Err(format!(
                        "Phase '{}' depends on non-existent phase '{}'",
                        phase.id(),
                        dep
                    ));
                }
            }
        }

        // Check for cycles using topological sort
        // in_degree[phase] = number of phases that this phase depends on
        let mut in_degree: std::collections::HashMap<String, usize> = self
            .phases
            .iter()
            .map(|p| (p.id().to_string(), p.dependencies().len()))
            .collect();

        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut processed = 0;

        while let Some(phase_id) = queue.pop_front() {
            processed += 1;

            // Find phases that depend on the current phase
            // When we process a phase, we decrement in_degree for phases that depend on it
            for phase in &self.phases {
                if phase
                    .dependencies()
                    .iter()
                    .any(|dep| dep.as_str() == phase_id)
                {
                    let count = in_degree.get_mut(phase.id().as_str()).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(phase.id().to_string());
                    }
                }
            }
        }

        if processed != phase_ids.len() {
            return Err("Circular dependency detected in phase dependencies".to_string());
        }

        Ok(())
    }

    /// Returns the total number of tasks across all phases
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xzagentz::domain::planning::{Plan, PlanMetadata, Phase, Task};
    /// # use chrono::Utc;
    /// # use std::path::PathBuf;
    /// # let metadata = PlanMetadata::new(Utc::now(), "model", PathBuf::from("test.md"));
    /// let task = Task::new("Task 1", "Description");
    /// let phase = Phase::new("phase-1", "Test", "Description").with_tasks(vec![task]);
    /// let plan = Plan::new("Title", "Description", vec![phase], metadata);
    /// assert_eq!(plan.total_tasks(), 1);
    /// ```
    pub fn total_tasks(&self) -> usize {
        self.phases.iter().map(|p| p.tasks().len()).sum()
    }
}

/// Metadata about plan generation
///
/// Contains provenance information about how and when the plan was generated,
/// including the source architecture document and the model used.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::PlanMetadata;
/// use chrono::Utc;
/// use std::path::PathBuf;
///
/// let metadata = PlanMetadata::new(
///     Utc::now(),
///     "llama3.2:3b",
///     PathBuf::from("docs/reference/architecture.md"),
/// );
///
/// assert_eq!(metadata.model_used(), "llama3.2:3b");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanMetadata {
    generated_at: DateTime<Utc>,
    model_used: String,
    source_document: PathBuf,
    version: String,
}

impl PlanMetadata {
    /// Creates new plan metadata
    ///
    /// # Arguments
    ///
    /// * `generated_at` - Timestamp when the plan was generated
    /// * `model_used` - Name/identifier of the LLM model used
    /// * `source_document` - Path to the source architecture document
    ///
    /// # Returns
    ///
    /// Returns a new PlanMetadata instance with default version "1.0.0"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanMetadata;
    /// use chrono::Utc;
    /// use std::path::PathBuf;
    ///
    /// let metadata = PlanMetadata::new(
    ///     Utc::now(),
    ///     "llama3.2:3b",
    ///     PathBuf::from("architecture.md"),
    /// );
    ///
    /// assert_eq!(metadata.version(), "1.0.0");
    /// ```
    pub fn new(
        generated_at: DateTime<Utc>,
        model_used: impl Into<String>,
        source_document: PathBuf,
    ) -> Self {
        Self {
            generated_at,
            model_used: model_used.into(),
            source_document,
            version: "1.0.0".to_string(),
        }
    }

    /// Creates metadata with a custom version
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PlanMetadata;
    /// use chrono::Utc;
    /// use std::path::PathBuf;
    ///
    /// let metadata = PlanMetadata::with_version(
    ///     Utc::now(),
    ///     "llama3.2:3b",
    ///     PathBuf::from("architecture.md"),
    ///     "2.0.0",
    /// );
    ///
    /// assert_eq!(metadata.version(), "2.0.0");
    /// ```
    pub fn with_version(
        generated_at: DateTime<Utc>,
        model_used: impl Into<String>,
        source_document: PathBuf,
        version: impl Into<String>,
    ) -> Self {
        Self {
            generated_at,
            model_used: model_used.into(),
            source_document,
            version: version.into(),
        }
    }

    /// Returns the generation timestamp
    pub fn generated_at(&self) -> DateTime<Utc> {
        self.generated_at
    }

    /// Returns the model identifier
    pub fn model_used(&self) -> &str {
        &self.model_used
    }

    /// Returns the source document path
    pub fn source_document(&self) -> &PathBuf {
        &self.source_document
    }

    /// Returns the plan version
    pub fn version(&self) -> &str {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::planning::Task;

    fn create_test_metadata() -> PlanMetadata {
        PlanMetadata::new(Utc::now(), "test-model", PathBuf::from("test.md"))
    }

    #[test]
    fn test_plan_new() {
        let metadata = create_test_metadata();
        let plan = Plan::new("Test Plan", "Test description", vec![], metadata);

        assert_eq!(plan.title(), "Test Plan");
        assert_eq!(plan.description(), "Test description");
        assert_eq!(plan.phases().len(), 0);
    }

    #[test]
    fn test_plan_with_phases() {
        let metadata = create_test_metadata();
        let phase = Phase::new("phase-1", "Phase 1", "First phase");
        let plan = Plan::new("Test Plan", "Description", vec![phase], metadata);

        assert_eq!(plan.phases().len(), 1);
        assert_eq!(plan.phases()[0].name(), "Phase 1");
    }

    #[test]
    fn test_plan_validation_empty_title() {
        let metadata = create_test_metadata();
        let plan = Plan::new("", "Description", vec![], metadata);

        let result = plan.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("title cannot be empty"));
    }

    #[test]
    fn test_plan_validation_no_phases() {
        let metadata = create_test_metadata();
        let plan = Plan::new("Title", "Description", vec![], metadata);

        let result = plan.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least one phase"));
    }

    #[test]
    fn test_plan_validation_success() {
        let metadata = create_test_metadata();
        let phase = Phase::new("phase-1", "Phase 1", "Description");
        let plan = Plan::new("Title", "Description", vec![phase], metadata);

        assert!(plan.validate().is_ok());
    }

    #[test]
    fn test_plan_validation_invalid_dependency() {
        let metadata = create_test_metadata();
        let phase = Phase::new("phase-1", "Phase 1", "Description")
            .with_dependencies(vec!["non-existent".into()]);
        let plan = Plan::new("Title", "Description", vec![phase], metadata);

        let result = plan.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("non-existent"));
    }

    #[test]
    fn test_plan_validation_circular_dependency() {
        let metadata = create_test_metadata();
        let phase1 = Phase::new("phase-1", "Phase 1", "Description")
            .with_dependencies(vec!["phase-2".into()]);
        let phase2 = Phase::new("phase-2", "Phase 2", "Description")
            .with_dependencies(vec!["phase-1".into()]);
        let plan = Plan::new("Title", "Description", vec![phase1, phase2], metadata);

        let result = plan.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular dependency"));
    }

    #[test]
    fn test_plan_validation_valid_dependencies() {
        let metadata = create_test_metadata();
        let phase1 = Phase::new("phase-1", "Phase 1", "Description");
        let phase2 = Phase::new("phase-2", "Phase 2", "Description")
            .with_dependencies(vec!["phase-1".into()]);
        let plan = Plan::new("Title", "Description", vec![phase1, phase2], metadata);

        assert!(plan.validate().is_ok());
    }

    #[test]
    fn test_plan_total_tasks() {
        let metadata = create_test_metadata();
        let task1 = Task::new("Task 1", "Description");
        let task2 = Task::new("Task 2", "Description");
        let phase = Phase::new("phase-1", "Phase 1", "Description").with_tasks(vec![task1, task2]);
        let plan = Plan::new("Title", "Description", vec![phase], metadata);

        assert_eq!(plan.total_tasks(), 2);
    }

    #[test]
    fn test_plan_metadata_accessors() {
        let metadata = create_test_metadata();
        let plan = Plan::new("Title", "Description", vec![], metadata.clone());

        assert_eq!(plan.metadata().model_used(), metadata.model_used());
        assert_eq!(
            plan.metadata().source_document(),
            metadata.source_document()
        );
    }

    #[test]
    fn test_metadata_new() {
        let now = Utc::now();
        let metadata = PlanMetadata::new(now, "llama3.2:3b", PathBuf::from("arch.md"));

        assert_eq!(metadata.model_used(), "llama3.2:3b");
        assert_eq!(metadata.source_document(), &PathBuf::from("arch.md"));
        assert_eq!(metadata.version(), "1.0.0");
    }

    #[test]
    fn test_metadata_with_version() {
        let now = Utc::now();
        let metadata =
            PlanMetadata::with_version(now, "llama3.2:3b", PathBuf::from("arch.md"), "2.1.0");

        assert_eq!(metadata.version(), "2.1.0");
    }

    #[test]
    fn test_metadata_generated_at() {
        let now = Utc::now();
        let metadata = PlanMetadata::new(now, "model", PathBuf::from("test.md"));

        // Compare timestamps within a reasonable delta
        let diff = (metadata.generated_at() - now).num_seconds().abs();
        assert!(diff < 1);
    }

    #[test]
    fn test_plan_serialization() {
        let metadata = create_test_metadata();
        let phase = Phase::new("phase-1", "Phase 1", "Description");
        let plan = Plan::new("Title", "Description", vec![phase], metadata);

        let json = serde_json::to_string(&plan).unwrap();
        let deserialized: Plan = serde_json::from_str(&json).unwrap();

        assert_eq!(plan.title(), deserialized.title());
        assert_eq!(plan.phases().len(), deserialized.phases().len());
    }
}
