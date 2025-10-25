//! Plan management system for architecture and implementation plans
//!
//! This module provides functionality for creating, parsing, and managing
//! architecture and implementation plans for software projects.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::plans::structures::{ArchitecturePlan, PlanMetadata};
//! use xzagentz::plans::embedded::EmbeddedTemplates;
//!
//! // Create a plan from metadata
//! let metadata = PlanMetadata::new("My Project", "1.0.0");
//! let plan = ArchitecturePlan::new(metadata, "Project overview");
//!
//! // Load embedded templates
//! let templates = EmbeddedTemplates::new();
//! let template = templates.get_plan_template("architecture_plan_rust_binary");
//! ```

pub mod embedded;
pub mod parser;
pub mod structures;

// Re-export commonly used types
pub use embedded::{EmbeddedTemplate, EmbeddedTemplates, TemplateCategory};
pub use parser::PlanParser;
pub use structures::{
    ArchitecturePlan, ImplementationPlan, Phase, PlanMetadata, PlanSection, PlanTemplate,
};
