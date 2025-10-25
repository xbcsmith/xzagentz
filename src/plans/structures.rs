//! Data structures for architecture and implementation plans
//!
//! This module defines the core data structures used to represent
//! architecture plans and implementation plans in the xzagentz system.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::plans::structures::{ArchitecturePlan, PlanMetadata};
//!
//! let metadata = PlanMetadata {
//!     name: "My Project".to_string(),
//!     description: Some("A Rust project".to_string()),
//!     version: "1.0.0".to_string(),
//!     author: Some("Developer".to_string()),
//!     created_at: None,
//! };
//!
//! let plan = ArchitecturePlan {
//!     metadata,
//!     overview: "Project overview".to_string(),
//!     requirements: vec![],
//!     architecture: vec![],
//!     components: vec![],
//!     sections: vec![],
//! };
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a plan document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanMetadata {
    /// Name of the project
    pub name: String,

    /// Brief description of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Version of the plan
    pub version: String,

    /// Author of the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl PlanMetadata {
    /// Creates a new PlanMetadata with required fields
    ///
    /// # Arguments
    ///
    /// * `name` - The project name
    /// * `version` - The plan version
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::structures::PlanMetadata;
    ///
    /// let metadata = PlanMetadata::new("My Project", "1.0.0");
    /// assert_eq!(metadata.name, "My Project");
    /// assert_eq!(metadata.version, "1.0.0");
    /// ```
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: None,
            author: None,
            created_at: None,
        }
    }

    /// Sets the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the creation timestamp
    pub fn with_created_at(mut self, created_at: impl Into<String>) -> Self {
        self.created_at = Some(created_at.into());
        self
    }
}

/// A section within a plan document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanSection {
    /// Section number (e.g., "1.1", "2.3")
    pub number: String,

    /// Section title
    pub title: String,

    /// Section content (markdown)
    pub content: String,

    /// Tasks in this section
    #[serde(default)]
    pub tasks: Vec<String>,

    /// Deliverables for this section
    #[serde(default)]
    pub deliverables: Vec<String>,

    /// Acceptance criteria
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,

    /// Testing requirements
    #[serde(default)]
    pub tests: Vec<String>,
}

impl PlanSection {
    /// Creates a new section with minimal fields
    ///
    /// # Arguments
    ///
    /// * `number` - Section number
    /// * `title` - Section title
    /// * `content` - Section content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::structures::PlanSection;
    ///
    /// let section = PlanSection::new("1.1", "Foundation", "Setup project");
    /// assert_eq!(section.number, "1.1");
    /// assert_eq!(section.title, "Foundation");
    /// ```
    pub fn new(
        number: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            number: number.into(),
            title: title.into(),
            content: content.into(),
            tasks: Vec::new(),
            deliverables: Vec::new(),
            acceptance_criteria: Vec::new(),
            tests: Vec::new(),
        }
    }

    /// Adds a task to the section
    pub fn add_task(mut self, task: impl Into<String>) -> Self {
        self.tasks.push(task.into());
        self
    }

    /// Adds a deliverable to the section
    pub fn add_deliverable(mut self, deliverable: impl Into<String>) -> Self {
        self.deliverables.push(deliverable.into());
        self
    }

    /// Adds acceptance criteria to the section
    pub fn add_acceptance_criteria(mut self, criteria: impl Into<String>) -> Self {
        self.acceptance_criteria.push(criteria.into());
        self
    }

    /// Adds a test requirement to the section
    pub fn add_test(mut self, test: impl Into<String>) -> Self {
        self.tests.push(test.into());
        self
    }
}

/// A phase in an implementation plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phase {
    /// Phase number
    pub number: usize,

    /// Phase title
    pub title: String,

    /// Duration estimate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// Goal of this phase
    pub goal: String,

    /// Sections in this phase
    #[serde(default)]
    pub sections: Vec<PlanSection>,

    /// Dependencies on other phases
    #[serde(default)]
    pub dependencies: Vec<usize>,
}

impl Phase {
    /// Creates a new phase
    ///
    /// # Arguments
    ///
    /// * `number` - Phase number
    /// * `title` - Phase title
    /// * `goal` - Phase goal
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::structures::Phase;
    ///
    /// let phase = Phase::new(1, "Foundation", "Setup project structure");
    /// assert_eq!(phase.number, 1);
    /// assert_eq!(phase.title, "Foundation");
    /// ```
    pub fn new(number: usize, title: impl Into<String>, goal: impl Into<String>) -> Self {
        Self {
            number,
            title: title.into(),
            duration: None,
            goal: goal.into(),
            sections: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Sets the duration estimate
    pub fn with_duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Adds a section to this phase
    pub fn add_section(mut self, section: PlanSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Adds a dependency on another phase
    pub fn add_dependency(mut self, phase_number: usize) -> Self {
        self.dependencies.push(phase_number);
        self
    }
}

/// Architecture plan structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchitecturePlan {
    /// Plan metadata
    pub metadata: PlanMetadata,

    /// Project overview section
    pub overview: String,

    /// Requirements section
    #[serde(default)]
    pub requirements: Vec<String>,

    /// Architecture design section
    #[serde(default)]
    pub architecture: Vec<String>,

    /// Components section
    #[serde(default)]
    pub components: Vec<String>,

    /// Additional sections
    #[serde(default)]
    pub sections: Vec<PlanSection>,
}

impl ArchitecturePlan {
    /// Creates a new architecture plan
    ///
    /// # Arguments
    ///
    /// * `metadata` - Plan metadata
    /// * `overview` - Project overview
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::structures::{ArchitecturePlan, PlanMetadata};
    ///
    /// let metadata = PlanMetadata::new("My Project", "1.0.0");
    /// let plan = ArchitecturePlan::new(metadata, "Overview of the project");
    /// assert_eq!(plan.overview, "Overview of the project");
    /// ```
    pub fn new(metadata: PlanMetadata, overview: impl Into<String>) -> Self {
        Self {
            metadata,
            overview: overview.into(),
            requirements: Vec::new(),
            architecture: Vec::new(),
            components: Vec::new(),
            sections: Vec::new(),
        }
    }

    /// Adds a requirement
    pub fn add_requirement(mut self, requirement: impl Into<String>) -> Self {
        self.requirements.push(requirement.into());
        self
    }

    /// Adds an architecture item
    pub fn add_architecture(mut self, architecture: impl Into<String>) -> Self {
        self.architecture.push(architecture.into());
        self
    }

    /// Adds a component
    pub fn add_component(mut self, component: impl Into<String>) -> Self {
        self.components.push(component.into());
        self
    }

    /// Adds a section
    pub fn add_section(mut self, section: PlanSection) -> Self {
        self.sections.push(section);
        self
    }
}

/// Implementation plan structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplementationPlan {
    /// Plan metadata
    pub metadata: PlanMetadata,

    /// Project overview
    pub overview: String,

    /// Architecture summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_summary: Option<String>,

    /// Implementation phases
    #[serde(default)]
    pub phases: Vec<Phase>,

    /// Success metrics
    #[serde(default)]
    pub success_metrics: Vec<String>,

    /// Risk mitigation strategies
    #[serde(default)]
    pub risks: Vec<String>,
}

impl ImplementationPlan {
    /// Creates a new implementation plan
    ///
    /// # Arguments
    ///
    /// * `metadata` - Plan metadata
    /// * `overview` - Project overview
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::structures::{ImplementationPlan, PlanMetadata};
    ///
    /// let metadata = PlanMetadata::new("My Project", "1.0.0");
    /// let plan = ImplementationPlan::new(metadata, "Implementation overview");
    /// assert_eq!(plan.overview, "Implementation overview");
    /// ```
    pub fn new(metadata: PlanMetadata, overview: impl Into<String>) -> Self {
        Self {
            metadata,
            overview: overview.into(),
            architecture_summary: None,
            phases: Vec::new(),
            success_metrics: Vec::new(),
            risks: Vec::new(),
        }
    }

    /// Sets the architecture summary
    pub fn with_architecture_summary(mut self, summary: impl Into<String>) -> Self {
        self.architecture_summary = Some(summary.into());
        self
    }

    /// Adds a phase
    pub fn add_phase(mut self, phase: Phase) -> Self {
        self.phases.push(phase);
        self
    }

    /// Adds a success metric
    pub fn add_success_metric(mut self, metric: impl Into<String>) -> Self {
        self.success_metrics.push(metric.into());
        self
    }

    /// Adds a risk
    pub fn add_risk(mut self, risk: impl Into<String>) -> Self {
        self.risks.push(risk.into());
        self
    }

    /// Gets a phase by number
    ///
    /// # Arguments
    ///
    /// * `number` - The phase number to find
    ///
    /// # Returns
    ///
    /// Returns a reference to the phase if found, None otherwise
    pub fn get_phase(&self, number: usize) -> Option<&Phase> {
        self.phases.iter().find(|p| p.number == number)
    }

    /// Gets a mutable phase by number
    pub fn get_phase_mut(&mut self, number: usize) -> Option<&mut Phase> {
        self.phases.iter_mut().find(|p| p.number == number)
    }
}

/// Template configuration for plan generation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanTemplate {
    /// Template name
    pub name: String,

    /// Template category (e.g., "rust_binary", "web_service")
    pub category: String,

    /// Template description
    pub description: String,

    /// Template version
    pub version: String,

    /// Required placeholders
    #[serde(default)]
    pub required_placeholders: Vec<String>,

    /// Optional placeholders
    #[serde(default)]
    pub optional_placeholders: Vec<String>,

    /// Template content (markdown with placeholders)
    pub content: String,
}

impl PlanTemplate {
    /// Creates a new plan template
    ///
    /// # Arguments
    ///
    /// * `name` - Template name
    /// * `category` - Template category
    /// * `description` - Template description
    /// * `version` - Template version
    /// * `content` - Template content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::plans::structures::PlanTemplate;
    ///
    /// let template = PlanTemplate::new(
    ///     "rust_binary",
    ///     "plans",
    ///     "Rust binary application",
    ///     "1.0.0",
    ///     "# Architecture Plan\n..."
    /// );
    /// assert_eq!(template.name, "rust_binary");
    /// ```
    pub fn new(
        name: impl Into<String>,
        category: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            category: category.into(),
            description: description.into(),
            version: version.into(),
            required_placeholders: Vec::new(),
            optional_placeholders: Vec::new(),
            content: content.into(),
        }
    }

    /// Adds a required placeholder
    pub fn add_required_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.required_placeholders.push(placeholder.into());
        self
    }

    /// Adds an optional placeholder
    pub fn add_optional_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.optional_placeholders.push(placeholder.into());
        self
    }

    /// Validates that all required placeholders are present in values
    ///
    /// # Arguments
    ///
    /// * `values` - Placeholder values to validate
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if all required placeholders are present, Err with missing placeholders otherwise
    pub fn validate_placeholders(
        &self,
        values: &HashMap<String, String>,
    ) -> Result<(), Vec<String>> {
        let missing: Vec<String> = self
            .required_placeholders
            .iter()
            .filter(|p| !values.contains_key(*p))
            .cloned()
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_metadata_new() {
        let metadata = PlanMetadata::new("Test Project", "1.0.0");
        assert_eq!(metadata.name, "Test Project");
        assert_eq!(metadata.version, "1.0.0");
        assert!(metadata.description.is_none());
        assert!(metadata.author.is_none());
    }

    #[test]
    fn test_plan_metadata_with_description() {
        let metadata = PlanMetadata::new("Test", "1.0.0")
            .with_description("A test project")
            .with_author("Developer");
        assert_eq!(metadata.description, Some("A test project".to_string()));
        assert_eq!(metadata.author, Some("Developer".to_string()));
    }

    #[test]
    fn test_plan_section_new() {
        let section = PlanSection::new("1.1", "Foundation", "Setup project");
        assert_eq!(section.number, "1.1");
        assert_eq!(section.title, "Foundation");
        assert_eq!(section.content, "Setup project");
        assert!(section.tasks.is_empty());
    }

    #[test]
    fn test_plan_section_with_tasks() {
        let section = PlanSection::new("1.1", "Setup", "Initialize")
            .add_task("Create project")
            .add_deliverable("Project structure")
            .add_acceptance_criteria("Compiles successfully");
        assert_eq!(section.tasks.len(), 1);
        assert_eq!(section.deliverables.len(), 1);
        assert_eq!(section.acceptance_criteria.len(), 1);
    }

    #[test]
    fn test_phase_new() {
        let phase = Phase::new(1, "Foundation", "Setup project");
        assert_eq!(phase.number, 1);
        assert_eq!(phase.title, "Foundation");
        assert_eq!(phase.goal, "Setup project");
        assert!(phase.duration.is_none());
    }

    #[test]
    fn test_phase_with_sections() {
        let section = PlanSection::new("1.1", "Setup", "Initialize");
        let phase = Phase::new(1, "Foundation", "Setup")
            .with_duration("1 week")
            .add_section(section)
            .add_dependency(0);
        assert_eq!(phase.duration, Some("1 week".to_string()));
        assert_eq!(phase.sections.len(), 1);
        assert_eq!(phase.dependencies.len(), 1);
    }

    #[test]
    fn test_architecture_plan_new() {
        let metadata = PlanMetadata::new("Test", "1.0.0");
        let plan = ArchitecturePlan::new(metadata.clone(), "Overview");
        assert_eq!(plan.metadata.name, "Test");
        assert_eq!(plan.overview, "Overview");
    }

    #[test]
    fn test_architecture_plan_add_items() {
        let metadata = PlanMetadata::new("Test", "1.0.0");
        let plan = ArchitecturePlan::new(metadata, "Overview")
            .add_requirement("Must be fast")
            .add_architecture("Layered architecture")
            .add_component("API Layer");
        assert_eq!(plan.requirements.len(), 1);
        assert_eq!(plan.architecture.len(), 1);
        assert_eq!(plan.components.len(), 1);
    }

    #[test]
    fn test_implementation_plan_new() {
        let metadata = PlanMetadata::new("Test", "1.0.0");
        let plan = ImplementationPlan::new(metadata.clone(), "Overview");
        assert_eq!(plan.metadata.name, "Test");
        assert_eq!(plan.overview, "Overview");
    }

    #[test]
    fn test_implementation_plan_add_phases() {
        let metadata = PlanMetadata::new("Test", "1.0.0");
        let phase = Phase::new(1, "Foundation", "Setup");
        let plan = ImplementationPlan::new(metadata, "Overview")
            .add_phase(phase)
            .add_success_metric("80% coverage")
            .add_risk("Technical debt");
        assert_eq!(plan.phases.len(), 1);
        assert_eq!(plan.success_metrics.len(), 1);
        assert_eq!(plan.risks.len(), 1);
    }

    #[test]
    fn test_implementation_plan_get_phase() {
        let metadata = PlanMetadata::new("Test", "1.0.0");
        let phase1 = Phase::new(1, "Foundation", "Setup");
        let phase2 = Phase::new(2, "Core", "Implement");
        let plan = ImplementationPlan::new(metadata, "Overview")
            .add_phase(phase1)
            .add_phase(phase2);
        assert!(plan.get_phase(1).is_some());
        assert!(plan.get_phase(2).is_some());
        assert!(plan.get_phase(3).is_none());
    }

    #[test]
    fn test_plan_template_new() {
        let template = PlanTemplate::new(
            "rust_binary",
            "plans",
            "Rust binary template",
            "1.0.0",
            "# Template",
        );
        assert_eq!(template.name, "rust_binary");
        assert_eq!(template.category, "plans");
    }

    #[test]
    fn test_plan_template_validate_placeholders() {
        let template = PlanTemplate::new("test", "plans", "desc", "1.0", "content")
            .add_required_placeholder("name")
            .add_required_placeholder("version");

        let mut values = HashMap::new();
        values.insert("name".to_string(), "Test".to_string());

        let result = template.validate_placeholders(&values);
        assert!(result.is_err());

        values.insert("version".to_string(), "1.0.0".to_string());
        let result = template.validate_placeholders(&values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plan_section_serde() {
        let section = PlanSection::new("1.1", "Test", "Content")
            .add_task("Task 1")
            .add_deliverable("Deliverable 1");

        let json = serde_json::to_string(&section).unwrap();
        let deserialized: PlanSection = serde_json::from_str(&json).unwrap();
        assert_eq!(section, deserialized);
    }

    #[test]
    fn test_architecture_plan_structure() {
        let metadata = PlanMetadata::new("MyProject", "1.0.0")
            .with_description("A test project")
            .with_author("Developer");

        let section = PlanSection::new("1.1", "Foundation", "Setup the project");

        let plan = ArchitecturePlan::new(metadata, "Project overview")
            .add_requirement("Must be performant")
            .add_architecture("Layered architecture")
            .add_component("CLI Layer")
            .add_section(section);

        assert_eq!(plan.requirements.len(), 1);
        assert_eq!(plan.architecture.len(), 1);
        assert_eq!(plan.components.len(), 1);
        assert_eq!(plan.sections.len(), 1);
    }

    #[test]
    fn test_implementation_plan_structure() {
        let metadata = PlanMetadata::new("MyProject", "1.0.0");

        let section = PlanSection::new("1.1", "Core Setup", "Initialize project")
            .add_task("Create Cargo project")
            .add_deliverable("Cargo.toml")
            .add_acceptance_criteria("Project compiles");

        let phase = Phase::new(1, "Foundation", "Setup project structure")
            .with_duration("1 week")
            .add_section(section);

        let plan = ImplementationPlan::new(metadata, "Implementation overview")
            .with_architecture_summary("Layered architecture")
            .add_phase(phase)
            .add_success_metric("All tests pass")
            .add_risk("Dependency conflicts");

        assert_eq!(plan.phases.len(), 1);
        assert_eq!(plan.success_metrics.len(), 1);
        assert_eq!(plan.risks.len(), 1);
        assert!(plan.architecture_summary.is_some());
    }

    #[test]
    fn test_template_rust_binary() {
        let template = PlanTemplate::new(
            "rust_binary",
            "plans",
            "Architecture plan for Rust binary applications",
            "1.0.0",
            "# Architecture Plan: @project-name@\n\n@project-description@",
        )
        .add_required_placeholder("project-name")
        .add_required_placeholder("project-description")
        .add_optional_placeholder("author");

        assert_eq!(template.required_placeholders.len(), 2);
        assert_eq!(template.optional_placeholders.len(), 1);
    }

    #[test]
    fn test_template_serialization() {
        let template = PlanTemplate::new(
            "test",
            "plans",
            "Test template",
            "1.0.0",
            "# Test\n\n@placeholder@",
        )
        .add_required_placeholder("placeholder");

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: PlanTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(template, deserialized);
    }
}
