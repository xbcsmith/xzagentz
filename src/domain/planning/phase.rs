// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Phase domain model
//!
//! This module defines the Phase entity, which represents a single implementation
//! phase within a larger plan. Phases contain tasks and can depend on other phases.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::{Phase, Task};
//!
//! let task = Task::new("Setup database", "Initialize PostgreSQL");
//! let phase = Phase::new(
//!     "phase-1",
//!     "Infrastructure Setup",
//!     "Set up core infrastructure components",
//! )
//! .with_tasks(vec![task])
//! .with_estimated_duration("2 days");
//!
//! assert_eq!(phase.name(), "Infrastructure Setup");
//! assert_eq!(phase.tasks().len(), 1);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

use super::task::Task;

/// Unique identifier for a phase
///
/// PhaseId is a type-safe wrapper around a String to prevent mixing
/// phase identifiers with other string types.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::PhaseId;
///
/// let id: PhaseId = "phase-1".into();
/// assert_eq!(id.as_str(), "phase-1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhaseId(String);

impl PhaseId {
    /// Creates a new PhaseId
    ///
    /// # Arguments
    ///
    /// * `id` - The phase identifier string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PhaseId;
    ///
    /// let id = PhaseId::new("phase-1");
    /// assert_eq!(id.as_str(), "phase-1");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the phase ID as a string slice
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::PhaseId;
    ///
    /// let id = PhaseId::new("phase-42");
    /// assert_eq!(id.as_str(), "phase-42");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PhaseId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for PhaseId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for PhaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for PhaseId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Represents a single implementation phase
///
/// A Phase is a logical grouping of tasks that should be completed together.
/// Phases can depend on other phases, creating a directed acyclic graph (DAG)
/// of work to be done.
///
/// # Invariants
///
/// - ID must not be empty
/// - Name must not be empty
/// - Dependencies must refer to valid phase IDs (validated at Plan level)
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::{Phase, Task};
///
/// let task1 = Task::new("Create models", "Define domain models");
/// let task2 = Task::new("Write tests", "Add unit tests");
///
/// let phase = Phase::new(
///     "phase-1",
///     "Domain Layer",
///     "Implement core domain logic",
/// )
/// .with_tasks(vec![task1, task2])
/// .with_dependencies(vec!["phase-0".into()])
/// .with_estimated_duration("3 days");
///
/// assert_eq!(phase.tasks().len(), 2);
/// assert_eq!(phase.dependencies().len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Phase {
    id: PhaseId,
    name: String,
    description: String,
    tasks: Vec<Task>,
    dependencies: Vec<PhaseId>,
    estimated_duration: Option<String>,
}

impl Phase {
    /// Creates a new Phase
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the phase
    /// * `name` - Human-readable name
    /// * `description` - Detailed description of the phase
    ///
    /// # Returns
    ///
    /// Returns a new Phase with no tasks, dependencies, or duration estimate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Phase;
    ///
    /// let phase = Phase::new(
    ///     "phase-1",
    ///     "Setup",
    ///     "Initialize project structure",
    /// );
    ///
    /// assert_eq!(phase.id().as_str(), "phase-1");
    /// assert_eq!(phase.name(), "Setup");
    /// ```
    pub fn new(
        id: impl Into<PhaseId>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            tasks: Vec::new(),
            dependencies: Vec::new(),
            estimated_duration: None,
        }
    }

    /// Adds tasks to the phase (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{Phase, Task};
    ///
    /// let task = Task::new("Task 1", "Description");
    /// let phase = Phase::new("phase-1", "Name", "Description")
    ///     .with_tasks(vec![task]);
    ///
    /// assert_eq!(phase.tasks().len(), 1);
    /// ```
    pub fn with_tasks(mut self, tasks: Vec<Task>) -> Self {
        self.tasks = tasks;
        self
    }

    /// Adds dependencies to the phase (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Phase;
    ///
    /// let phase = Phase::new("phase-2", "Name", "Description")
    ///     .with_dependencies(vec!["phase-1".into()]);
    ///
    /// assert_eq!(phase.dependencies().len(), 1);
    /// ```
    pub fn with_dependencies(mut self, dependencies: Vec<PhaseId>) -> Self {
        self.dependencies = dependencies;
        self
    }

    /// Sets the estimated duration (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Phase;
    ///
    /// let phase = Phase::new("phase-1", "Name", "Description")
    ///     .with_estimated_duration("2 days");
    ///
    /// assert_eq!(phase.estimated_duration(), Some("2 days"));
    /// ```
    pub fn with_estimated_duration(mut self, duration: impl Into<String>) -> Self {
        self.estimated_duration = Some(duration.into());
        self
    }

    /// Returns the phase ID
    pub fn id(&self) -> &PhaseId {
        &self.id
    }

    /// Returns the phase name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the phase description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns a reference to the tasks
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Returns a reference to the dependencies
    pub fn dependencies(&self) -> &[PhaseId] {
        &self.dependencies
    }

    /// Returns the estimated duration if set
    pub fn estimated_duration(&self) -> Option<&str> {
        self.estimated_duration.as_deref()
    }

    /// Adds a task to the phase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{Phase, Task};
    ///
    /// let mut phase = Phase::new("phase-1", "Name", "Description");
    /// let task = Task::new("New task", "Description");
    /// phase.add_task(task);
    ///
    /// assert_eq!(phase.tasks().len(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Adds a dependency to the phase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Phase;
    ///
    /// let mut phase = Phase::new("phase-2", "Name", "Description");
    /// phase.add_dependency("phase-1".into());
    ///
    /// assert_eq!(phase.dependencies().len(), 1);
    /// ```
    pub fn add_dependency(&mut self, dependency: PhaseId) {
        self.dependencies.push(dependency);
    }

    /// Validates the phase structure
    ///
    /// Checks for:
    /// - Non-empty ID
    /// - Non-empty name
    /// - Valid tasks
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, Err with reason if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Phase;
    ///
    /// let phase = Phase::new("phase-1", "Setup", "Description");
    /// assert!(phase.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.id.as_str().trim().is_empty() {
            return Err("Phase ID cannot be empty".to_string());
        }

        if self.name.trim().is_empty() {
            return Err("Phase name cannot be empty".to_string());
        }

        if self.description.trim().is_empty() {
            return Err("Phase description cannot be empty".to_string());
        }

        // Validate each task
        for task in &self.tasks {
            task.validate()?;
        }

        Ok(())
    }

    /// Returns true if the phase has no dependencies
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Phase;
    ///
    /// let phase1 = Phase::new("phase-1", "First", "Description");
    /// assert!(phase1.is_root());
    ///
    /// let phase2 = Phase::new("phase-2", "Second", "Description")
    ///     .with_dependencies(vec!["phase-1".into()]);
    /// assert!(!phase2.is_root());
    /// ```
    pub fn is_root(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Returns the number of tasks in this phase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::{Phase, Task};
    ///
    /// let task = Task::new("Task 1", "Description");
    /// let phase = Phase::new("phase-1", "Name", "Description")
    ///     .with_tasks(vec![task]);
    ///
    /// assert_eq!(phase.task_count(), 1);
    /// ```
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_id_new() {
        let id = PhaseId::new("phase-1");
        assert_eq!(id.as_str(), "phase-1");
    }

    #[test]
    fn test_phase_id_from_str() {
        let id: PhaseId = "phase-2".into();
        assert_eq!(id.as_str(), "phase-2");
    }

    #[test]
    fn test_phase_id_from_string() {
        let id: PhaseId = String::from("phase-3").into();
        assert_eq!(id.as_str(), "phase-3");
    }

    #[test]
    fn test_phase_id_display() {
        let id = PhaseId::new("phase-test");
        assert_eq!(format!("{}", id), "phase-test");
    }

    #[test]
    fn test_phase_id_equality() {
        let id1 = PhaseId::new("phase-1");
        let id2 = PhaseId::new("phase-1");
        let id3 = PhaseId::new("phase-2");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_phase_new() {
        let phase = Phase::new("phase-1", "Setup", "Initial setup phase");

        assert_eq!(phase.id().as_str(), "phase-1");
        assert_eq!(phase.name(), "Setup");
        assert_eq!(phase.description(), "Initial setup phase");
        assert_eq!(phase.tasks().len(), 0);
        assert_eq!(phase.dependencies().len(), 0);
        assert_eq!(phase.estimated_duration(), None);
    }

    #[test]
    fn test_phase_with_tasks() {
        let task1 = Task::new("Task 1", "Description 1");
        let task2 = Task::new("Task 2", "Description 2");

        let phase = Phase::new("phase-1", "Name", "Description").with_tasks(vec![task1, task2]);

        assert_eq!(phase.tasks().len(), 2);
        assert_eq!(phase.task_count(), 2);
    }

    #[test]
    fn test_phase_with_dependencies() {
        let phase = Phase::new("phase-2", "Name", "Description")
            .with_dependencies(vec!["phase-1".into(), "phase-0".into()]);

        assert_eq!(phase.dependencies().len(), 2);
        assert!(!phase.is_root());
    }

    #[test]
    fn test_phase_with_estimated_duration() {
        let phase = Phase::new("phase-1", "Name", "Description").with_estimated_duration("3 days");

        assert_eq!(phase.estimated_duration(), Some("3 days"));
    }

    #[test]
    fn test_phase_is_root() {
        let root_phase = Phase::new("phase-1", "Root", "Description");
        assert!(root_phase.is_root());

        let dependent_phase = Phase::new("phase-2", "Dependent", "Description")
            .with_dependencies(vec!["phase-1".into()]);
        assert!(!dependent_phase.is_root());
    }

    #[test]
    fn test_phase_add_task() {
        let mut phase = Phase::new("phase-1", "Name", "Description");
        assert_eq!(phase.tasks().len(), 0);

        let task = Task::new("New task", "Description");
        phase.add_task(task);

        assert_eq!(phase.tasks().len(), 1);
    }

    #[test]
    fn test_phase_add_dependency() {
        let mut phase = Phase::new("phase-2", "Name", "Description");
        assert_eq!(phase.dependencies().len(), 0);

        phase.add_dependency("phase-1".into());
        assert_eq!(phase.dependencies().len(), 1);
    }

    #[test]
    fn test_phase_validation_success() {
        let phase = Phase::new("phase-1", "Valid Name", "Valid description");
        assert!(phase.validate().is_ok());
    }

    #[test]
    fn test_phase_validation_empty_id() {
        let phase = Phase::new("", "Name", "Description");
        let result = phase.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ID cannot be empty"));
    }

    #[test]
    fn test_phase_validation_empty_name() {
        let phase = Phase::new("phase-1", "", "Description");
        let result = phase.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("name cannot be empty"));
    }

    #[test]
    fn test_phase_validation_empty_description() {
        let phase = Phase::new("phase-1", "Name", "");
        let result = phase.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("description cannot be empty"));
    }

    #[test]
    fn test_phase_validation_with_invalid_task() {
        let invalid_task = Task::new("", "Description");
        let phase = Phase::new("phase-1", "Name", "Description").with_tasks(vec![invalid_task]);

        let result = phase.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_phase_builder_pattern() {
        let task = Task::new("Task", "Description");
        let phase = Phase::new("phase-1", "Name", "Description")
            .with_tasks(vec![task])
            .with_dependencies(vec!["phase-0".into()])
            .with_estimated_duration("2 days");

        assert_eq!(phase.tasks().len(), 1);
        assert_eq!(phase.dependencies().len(), 1);
        assert_eq!(phase.estimated_duration(), Some("2 days"));
    }

    #[test]
    fn test_phase_serialization() {
        let phase =
            Phase::new("phase-1", "Test Phase", "Description").with_estimated_duration("1 day");

        let json = serde_json::to_string(&phase).unwrap();
        let deserialized: Phase = serde_json::from_str(&json).unwrap();

        assert_eq!(phase.id(), deserialized.id());
        assert_eq!(phase.name(), deserialized.name());
        assert_eq!(
            phase.estimated_duration(),
            deserialized.estimated_duration()
        );
    }
}
