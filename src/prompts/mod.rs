// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Prompt generation and progress tracking module
//!
//! This module provides functionality for generating implementation prompts from
//! implementation plans, tracking progress through sections, and verifying compliance
//! with AGENTS.md rules.
//!
//! # Overview
//!
//! The prompt system helps manage the implementation workflow by:
//! - Generating structured prompts for each implementation section
//! - Tracking progress through sections with persistent state
//! - Verifying compliance with AGENTS.md rules
//!
//! # Example Workflow
//!
//! ```no_run
//! use xzagentz::prompts::{PromptGenerator, ProgressTracker};
//! use xzagentz::plans::parser::PlanParser;
//! use std::path::Path;
//!
//! // Parse implementation plan
//! let parser = PlanParser::new();
//! let plan_content = std::fs::read_to_string("docs/explanation/implementation_plan.md").unwrap();
//! let plan = parser.parse_implementation_plan(&plan_content).unwrap();
//!
//! // Initialize progress tracking
//! let mut tracker = ProgressTracker::new(Path::new(".")).unwrap();
//! tracker.initialize("docs/explanation/implementation_plan.md", 42, "xzagentz").unwrap();
//!
//! // Generate prompts
//! let generator = PromptGenerator::new(Path::new("prompts")).unwrap();
//! generator.generate_all(&plan, Some(&tracker)).unwrap();
//!
//! // Track completion
//! tracker.complete("1.1").unwrap();
//! let stats = tracker.stats();
//! println!("Progress: {:.1}%", stats.percentage);
//! ```

pub mod context;
pub mod generator;
pub mod progress;
pub mod template;

// Re-export main types
pub use context::ContextExtractor;
pub use generator::{PromptGenerator, PromptGeneratorConfig};
pub use progress::{ProgressState, ProgressStats, ProgressTracker};
pub use template::{
    DeliverableFile, PromptContext, PromptTemplate, Rule, TemplateSection, TestExample, TestFile,
};
