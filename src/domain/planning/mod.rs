//! Planning domain module
//!
//! This module contains the core domain models and traits for implementation planning.
//! It follows Domain-Driven Design principles with no infrastructure dependencies.
//!
//! # Architecture
//!
//! The planning domain is organized into:
//! - Models: Core business entities (Plan, Phase, Task)
//! - Traits: Behavior contracts for plan generation and parsing
//! - Value Objects: Architecture documents and metadata
//!
//! # Design Principles
//!
//! 1. **No Infrastructure Dependencies**: This domain layer is pure business logic
//! 2. **Trait-Based Contracts**: Infrastructure implements domain traits
//! 3. **Rich Domain Models**: Models contain validation and business rules
//! 4. **Immutability Preferred**: Models should be created valid and remain consistent
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::{Plan, Phase, Task, PlanMetadata};
//! use chrono::Utc;
//! use std::path::PathBuf;
//!
//! // Create a task
//! let task = Task::new(
//!     "Implement user authentication",
//!     "Add JWT-based authentication system",
//! )
//! .with_acceptance_criteria(vec![
//!     "Users can log in with credentials".to_string(),
//!     "JWT tokens are issued on successful login".to_string(),
//! ])
//! .with_components(vec!["src/auth/mod.rs".to_string()]);
//!
//! // Create a phase
//! let phase = Phase::new(
//!     "phase-1",
//!     "Authentication Implementation",
//!     "Implement core authentication features",
//! )
//! .with_tasks(vec![task]);
//!
//! // Create metadata
//! let metadata = PlanMetadata::new(
//!     Utc::now(),
//!     "llama3.2:3b",
//!     PathBuf::from("docs/reference/architecture.md"),
//! );
//!
//! // Create a plan
//! let plan = Plan::new(
//!     "User Authentication System",
//!     "Implementation plan for authentication system",
//!     vec![phase],
//!     metadata,
//! );
//!
//! assert_eq!(plan.phases().len(), 1);
//! ```

pub mod architecture;
pub mod models;
pub mod phase;
pub mod task;
pub mod traits;

// Re-export main types for convenience
pub use architecture::{ArchitectureDocument, Component, Requirement, Section};
pub use models::{Plan, PlanMetadata};
pub use phase::{Phase, PhaseId};
pub use task::Task;
pub use traits::{ArchitectureParser, PlanGenerator, PlanOptions, PlanWriter};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;

    #[test]
    fn test_create_complete_plan() {
        let task = Task::new("Test task", "Description");
        let phase = Phase::new("phase-1", "Test Phase", "Description").with_tasks(vec![task]);
        let metadata = PlanMetadata::new(Utc::now(), "test-model", PathBuf::from("test.md"));
        let plan = Plan::new("Test Plan", "Description", vec![phase], metadata);

        assert_eq!(plan.title(), "Test Plan");
        assert_eq!(plan.phases().len(), 1);
    }

    #[test]
    fn test_phase_ordering() {
        let phase1 = Phase::new("phase-1", "First", "Description");
        let phase2 = Phase::new("phase-2", "Second", "Description")
            .with_dependencies(vec!["phase-1".into()]);

        assert!(phase1.dependencies().is_empty());
        assert_eq!(phase2.dependencies().len(), 1);
    }
}
