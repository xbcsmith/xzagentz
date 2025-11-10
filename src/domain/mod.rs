// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Domain layer for xzagentz
//!
//! This module contains the core domain models and business logic for xzagentz.
//! It follows Domain-Driven Design principles with no infrastructure dependencies.
//!
//! # Architecture
//!
//! The domain layer is organized into bounded contexts:
//!
//! - [`planning`]: Implementation planning domain models and contracts
//!
//! # Design Principles
//!
//! 1. **No Infrastructure Dependencies**: Domain layer is pure business logic
//! 2. **Rich Domain Models**: Models contain validation and business rules
//! 3. **Trait-Based Contracts**: Infrastructure implements domain traits
//! 4. **Testability**: All domain logic is easily testable without infrastructure
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::planning::{Plan, Phase, Task, PlanMetadata};
//! use chrono::Utc;
//! use std::path::PathBuf;
//!
//! // Create a complete implementation plan
//! let task = Task::new(
//!     "Setup project structure",
//!     "Initialize Rust project with cargo",
//! );
//!
//! let phase = Phase::new(
//!     "phase-1",
//!     "Project Initialization",
//!     "Set up the basic project structure",
//! )
//! .with_tasks(vec![task]);
//!
//! let metadata = PlanMetadata::new(
//!     Utc::now(),
//!     "llama3.2:3b",
//!     PathBuf::from("docs/architecture.md"),
//! );
//!
//! let plan = Plan::new(
//!     "Project Setup",
//!     "Initial project setup plan",
//!     vec![phase],
//!     metadata,
//! );
//!
//! assert!(plan.validate().is_ok());
//! ```

pub mod architecture;
pub mod planning;

// Re-export main types from planning for convenience
pub use planning::{
    ArchitectureDocument as PlanningArchitectureDocument, ArchitectureParser, Component, Phase,
    PhaseId, Plan, PlanGenerator, PlanMetadata, PlanOptions, PlanWriter, Requirement, Section,
    Task,
};

// Re-export main types from architecture for convenience
pub use architecture::{
    ArchitectureDocument, ArchitectureGenerator, ArchitectureMetadata, ArchitecturePattern,
    ArchitectureTemplate, ArchitectureValidator, ArchitectureWriter, AvailabilityDesign,
    ComplexityLevel, ComponentTemplate, DeploymentArchitecture, DeploymentStrategy,
    GenerationOptions, InfrastructureComponent, Integration, IntegrationType, Interface, Layer,
    Overview, QualityAttribute, ScalingStrategy, TemplateInfo, TemplateRepository,
    TemplateStructure, ValidationError,
};
