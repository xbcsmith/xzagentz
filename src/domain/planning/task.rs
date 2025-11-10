// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Task domain model
//!
//! This module defines the Task entity, which represents a single unit of work
//! within an implementation phase. Tasks include acceptance criteria and
//! associated components/files.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::Task;
//!
//! let task = Task::new(
//!     "Implement user authentication",
//!     "Add JWT-based authentication with token refresh",
//! )
//! .with_acceptance_criteria(vec![
//!     "Users can log in with username and password".to_string(),
//!     "JWT tokens expire after 1 hour".to_string(),
//!     "Refresh tokens work correctly".to_string(),
//! ])
//! .with_components(vec![
//!     "src/auth/mod.rs".to_string(),
//!     "src/auth/jwt.rs".to_string(),
//! ]);
//!
//! assert_eq!(task.name(), "Implement user authentication");
//! assert_eq!(task.acceptance_criteria().len(), 3);
//! ```

use serde::{Deserialize, Serialize};

/// Represents a single task within an implementation phase
///
/// A Task is the smallest unit of work in the planning system. It includes
/// a name, description, acceptance criteria for validation, and a list of
/// components (files/modules) that need to be created or modified.
///
/// # Invariants
///
/// - Name must not be empty
/// - Description must not be empty
/// - Acceptance criteria should be specific and testable
/// - Component paths should be valid project paths
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::planning::Task;
///
/// let task = Task::new(
///     "Create database schema",
///     "Design and implement the initial database schema for users",
/// )
/// .with_acceptance_criteria(vec![
///     "Schema includes users table".to_string(),
///     "Schema includes appropriate indexes".to_string(),
/// ])
/// .with_components(vec![
///     "migrations/001_create_users.sql".to_string(),
/// ]);
///
/// assert_eq!(task.components().len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    name: String,
    description: String,
    acceptance_criteria: Vec<String>,
    components: Vec<String>,
}

impl Task {
    /// Creates a new Task
    ///
    /// # Arguments
    ///
    /// * `name` - The task name (must not be empty)
    /// * `description` - Detailed description of what needs to be done
    ///
    /// # Returns
    ///
    /// Returns a new Task with empty acceptance criteria and components
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new(
    ///     "Setup logging",
    ///     "Configure application logging with tracing",
    /// );
    ///
    /// assert_eq!(task.name(), "Setup logging");
    /// assert!(task.acceptance_criteria().is_empty());
    /// ```
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            acceptance_criteria: Vec::new(),
            components: Vec::new(),
        }
    }

    /// Adds acceptance criteria (builder pattern)
    ///
    /// # Arguments
    ///
    /// * `criteria` - Vector of acceptance criteria strings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "Description")
    ///     .with_acceptance_criteria(vec![
    ///         "Criterion 1".to_string(),
    ///         "Criterion 2".to_string(),
    ///     ]);
    ///
    /// assert_eq!(task.acceptance_criteria().len(), 2);
    /// ```
    pub fn with_acceptance_criteria(mut self, criteria: Vec<String>) -> Self {
        self.acceptance_criteria = criteria;
        self
    }

    /// Adds components (builder pattern)
    ///
    /// # Arguments
    ///
    /// * `components` - Vector of component/file paths
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "Description")
    ///     .with_components(vec![
    ///         "src/main.rs".to_string(),
    ///         "src/lib.rs".to_string(),
    ///     ]);
    ///
    /// assert_eq!(task.components().len(), 2);
    /// ```
    pub fn with_components(mut self, components: Vec<String>) -> Self {
        self.components = components;
        self
    }

    /// Returns the task name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("My Task", "Description");
    /// assert_eq!(task.name(), "My Task");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the task description
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "My description");
    /// assert_eq!(task.description(), "My description");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns a reference to the acceptance criteria
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "Description")
    ///     .with_acceptance_criteria(vec!["Criterion".to_string()]);
    ///
    /// assert_eq!(task.acceptance_criteria().len(), 1);
    /// ```
    pub fn acceptance_criteria(&self) -> &[String] {
        &self.acceptance_criteria
    }

    /// Returns a reference to the components
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "Description")
    ///     .with_components(vec!["src/main.rs".to_string()]);
    ///
    /// assert_eq!(task.components().len(), 1);
    /// ```
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Adds a single acceptance criterion
    ///
    /// # Arguments
    ///
    /// * `criterion` - The acceptance criterion to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let mut task = Task::new("Task", "Description");
    /// task.add_acceptance_criterion("Tests pass");
    ///
    /// assert_eq!(task.acceptance_criteria().len(), 1);
    /// ```
    pub fn add_acceptance_criterion(&mut self, criterion: impl Into<String>) {
        self.acceptance_criteria.push(criterion.into());
    }

    /// Adds a single component
    ///
    /// # Arguments
    ///
    /// * `component` - The component/file path to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let mut task = Task::new("Task", "Description");
    /// task.add_component("src/auth/mod.rs");
    ///
    /// assert_eq!(task.components().len(), 1);
    /// ```
    pub fn add_component(&mut self, component: impl Into<String>) {
        self.components.push(component.into());
    }

    /// Validates the task structure
    ///
    /// Checks for:
    /// - Non-empty name
    /// - Non-empty description
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, Err with reason if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Valid Task", "Valid description");
    /// assert!(task.validate().is_ok());
    ///
    /// let invalid_task = Task::new("", "Description");
    /// assert!(invalid_task.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }

        if self.description.trim().is_empty() {
            return Err("Task description cannot be empty".to_string());
        }

        // Validate acceptance criteria are not empty strings
        for criterion in &self.acceptance_criteria {
            if criterion.trim().is_empty() {
                return Err("Acceptance criteria cannot contain empty strings".to_string());
            }
        }

        // Validate components are not empty strings
        for component in &self.components {
            if component.trim().is_empty() {
                return Err("Component paths cannot be empty strings".to_string());
            }
        }

        Ok(())
    }

    /// Returns true if the task has acceptance criteria defined
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task1 = Task::new("Task", "Description");
    /// assert!(!task1.has_acceptance_criteria());
    ///
    /// let task2 = Task::new("Task", "Description")
    ///     .with_acceptance_criteria(vec!["Criterion".to_string()]);
    /// assert!(task2.has_acceptance_criteria());
    /// ```
    pub fn has_acceptance_criteria(&self) -> bool {
        !self.acceptance_criteria.is_empty()
    }

    /// Returns true if the task has components defined
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task1 = Task::new("Task", "Description");
    /// assert!(!task1.has_components());
    ///
    /// let task2 = Task::new("Task", "Description")
    ///     .with_components(vec!["src/main.rs".to_string()]);
    /// assert!(task2.has_components());
    /// ```
    pub fn has_components(&self) -> bool {
        !self.components.is_empty()
    }

    /// Returns the number of acceptance criteria
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "Description")
    ///     .with_acceptance_criteria(vec![
    ///         "Criterion 1".to_string(),
    ///         "Criterion 2".to_string(),
    ///     ]);
    ///
    /// assert_eq!(task.criteria_count(), 2);
    /// ```
    pub fn criteria_count(&self) -> usize {
        self.acceptance_criteria.len()
    }

    /// Returns the number of components
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::planning::Task;
    ///
    /// let task = Task::new("Task", "Description")
    ///     .with_components(vec!["src/main.rs".to_string()]);
    ///
    /// assert_eq!(task.component_count(), 1);
    /// ```
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = Task::new("Test Task", "Test description");

        assert_eq!(task.name(), "Test Task");
        assert_eq!(task.description(), "Test description");
        assert_eq!(task.acceptance_criteria().len(), 0);
        assert_eq!(task.components().len(), 0);
    }

    #[test]
    fn test_task_with_acceptance_criteria() {
        let task = Task::new("Task", "Description").with_acceptance_criteria(vec![
            "Criterion 1".to_string(),
            "Criterion 2".to_string(),
            "Criterion 3".to_string(),
        ]);

        assert_eq!(task.acceptance_criteria().len(), 3);
        assert_eq!(task.criteria_count(), 3);
        assert!(task.has_acceptance_criteria());
    }

    #[test]
    fn test_task_with_components() {
        let task = Task::new("Task", "Description")
            .with_components(vec!["src/main.rs".to_string(), "src/lib.rs".to_string()]);

        assert_eq!(task.components().len(), 2);
        assert_eq!(task.component_count(), 2);
        assert!(task.has_components());
    }

    #[test]
    fn test_task_add_acceptance_criterion() {
        let mut task = Task::new("Task", "Description");
        assert!(!task.has_acceptance_criteria());

        task.add_acceptance_criterion("New criterion");
        assert_eq!(task.acceptance_criteria().len(), 1);
        assert!(task.has_acceptance_criteria());
    }

    #[test]
    fn test_task_add_component() {
        let mut task = Task::new("Task", "Description");
        assert!(!task.has_components());

        task.add_component("src/auth/mod.rs");
        assert_eq!(task.components().len(), 1);
        assert!(task.has_components());
    }

    #[test]
    fn test_task_validation_success() {
        let task = Task::new("Valid Task", "Valid description");
        assert!(task.validate().is_ok());
    }

    #[test]
    fn test_task_validation_empty_name() {
        let task = Task::new("", "Description");
        let result = task.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("name cannot be empty"));
    }

    #[test]
    fn test_task_validation_whitespace_name() {
        let task = Task::new("   ", "Description");
        let result = task.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("name cannot be empty"));
    }

    #[test]
    fn test_task_validation_empty_description() {
        let task = Task::new("Name", "");
        let result = task.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("description cannot be empty"));
    }

    #[test]
    fn test_task_validation_empty_criterion() {
        let task = Task::new("Name", "Description")
            .with_acceptance_criteria(vec!["Valid criterion".to_string(), "".to_string()]);

        let result = task.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Acceptance criteria cannot contain empty strings"));
    }

    #[test]
    fn test_task_validation_empty_component() {
        let task = Task::new("Name", "Description")
            .with_components(vec!["src/main.rs".to_string(), "".to_string()]);

        let result = task.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Component paths cannot be empty strings"));
    }

    #[test]
    fn test_task_builder_pattern() {
        let task = Task::new("Complete Task", "Description")
            .with_acceptance_criteria(vec!["Criterion 1".to_string()])
            .with_components(vec!["src/main.rs".to_string()]);

        assert_eq!(task.name(), "Complete Task");
        assert_eq!(task.acceptance_criteria().len(), 1);
        assert_eq!(task.components().len(), 1);
    }

    #[test]
    fn test_task_has_acceptance_criteria() {
        let task1 = Task::new("Task", "Description");
        assert!(!task1.has_acceptance_criteria());

        let task2 = Task::new("Task", "Description")
            .with_acceptance_criteria(vec!["Criterion".to_string()]);
        assert!(task2.has_acceptance_criteria());
    }

    #[test]
    fn test_task_has_components() {
        let task1 = Task::new("Task", "Description");
        assert!(!task1.has_components());

        let task2 =
            Task::new("Task", "Description").with_components(vec!["src/main.rs".to_string()]);
        assert!(task2.has_components());
    }

    #[test]
    fn test_task_serialization() {
        let task = Task::new("Serialization Test", "Test serialization")
            .with_acceptance_criteria(vec!["Works correctly".to_string()])
            .with_components(vec!["src/test.rs".to_string()]);

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(task.name(), deserialized.name());
        assert_eq!(task.description(), deserialized.description());
        assert_eq!(
            task.acceptance_criteria().len(),
            deserialized.acceptance_criteria().len()
        );
        assert_eq!(task.components().len(), deserialized.components().len());
    }

    #[test]
    fn test_task_equality() {
        let task1 = Task::new("Task", "Description")
            .with_acceptance_criteria(vec!["Criterion".to_string()]);
        let task2 = Task::new("Task", "Description")
            .with_acceptance_criteria(vec!["Criterion".to_string()]);
        let task3 = Task::new("Different", "Description");

        assert_eq!(task1, task2);
        assert_ne!(task1, task3);
    }

    #[test]
    fn test_task_counts() {
        let task = Task::new("Task", "Description")
            .with_acceptance_criteria(vec!["C1".to_string(), "C2".to_string(), "C3".to_string()])
            .with_components(vec!["src/a.rs".to_string(), "src/b.rs".to_string()]);

        assert_eq!(task.criteria_count(), 3);
        assert_eq!(task.component_count(), 2);
    }
}
