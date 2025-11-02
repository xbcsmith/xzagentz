//! Phase 7: Comprehensive Unit Tests for Domain Models
//!
//! This module provides additional unit tests to ensure >80% coverage
//! for the core domain models and their validation logic.

use chrono::Utc;
use std::path::PathBuf;
use xzagentz::domain::planning::{
    ArchitectureDocument, Component, Phase, PhaseId, Plan, PlanMetadata, Requirement, Section, Task,
};

// ============================================================================
// Plan Model Tests
// ============================================================================

#[test]
fn test_plan_creation_with_valid_data() {
    let metadata = PlanMetadata::new(Utc::now(), "test-model", PathBuf::from("test.md"));
    let phase = Phase::new("phase-1", "Phase 1", "Test phase");
    let plan = Plan::new("Test Plan", "A test plan", vec![phase], metadata);

    assert_eq!(plan.title(), "Test Plan");
    assert_eq!(plan.description(), "A test plan");
    assert_eq!(plan.phases().len(), 1);
}

#[test]
fn test_plan_with_multiple_phases() {
    let metadata = PlanMetadata::new(Utc::now(), "test-model", PathBuf::from("test.md"));
    let phase1 = Phase::new("phase-1", "Phase 1", "First phase");
    let phase2 = Phase::new("phase-2", "Phase 2", "Second phase");
    let phase3 = Phase::new("phase-3", "Phase 3", "Third phase");

    let plan = Plan::new(
        "Multi-phase Plan",
        "Plan with multiple phases",
        vec![phase1, phase2, phase3],
        metadata,
    );

    assert_eq!(plan.phases().len(), 3);
    assert_eq!(plan.phases()[0].name(), "Phase 1");
    assert_eq!(plan.phases()[1].name(), "Phase 2");
    assert_eq!(plan.phases()[2].name(), "Phase 3");
}

#[test]
fn test_plan_metadata_accessors() {
    let now = Utc::now();
    let metadata = PlanMetadata::new(now, "llama3.2:3b", PathBuf::from("arch.md"));

    assert_eq!(metadata.model_used(), "llama3.2:3b");
    assert_eq!(metadata.source_document(), &PathBuf::from("arch.md"));
    assert_eq!(metadata.version(), "1.0.0");
}

#[test]
fn test_plan_with_empty_phases() {
    let metadata = PlanMetadata::new(Utc::now(), "test-model", PathBuf::from("test.md"));
    let plan = Plan::new("Empty Plan", "No phases", vec![], metadata);

    assert_eq!(plan.phases().len(), 0);
}

// ============================================================================
// Phase Model Tests
// ============================================================================

#[test]
fn test_phase_creation_with_valid_data() {
    let phase = Phase::new("phase-1", "Test Phase", "A test phase description");

    assert_eq!(phase.id().as_str(), "phase-1");
    assert_eq!(phase.name(), "Test Phase");
    assert_eq!(phase.description(), "A test phase description");
    assert_eq!(phase.tasks().len(), 0);
    assert_eq!(phase.dependencies().len(), 0);
}

#[test]
fn test_phase_with_tasks() {
    let task1 = Task::new("Task 1", "First task");
    let task2 = Task::new("Task 2", "Second task");
    let phase = Phase::new("phase-1", "Phase with Tasks", "Test").with_tasks(vec![task1, task2]);

    assert_eq!(phase.tasks().len(), 2);
    assert_eq!(phase.tasks()[0].name(), "Task 1");
    assert_eq!(phase.tasks()[1].name(), "Task 2");
}

#[test]
fn test_phase_with_dependencies() {
    let dep1 = PhaseId::new("phase-0");
    let dep2 = PhaseId::new("phase-00");
    let phase = Phase::new("phase-1", "Dependent Phase", "Depends on others")
        .with_dependencies(vec![dep1, dep2]);

    assert_eq!(phase.dependencies().len(), 2);
}

#[test]
fn test_phase_with_estimated_duration() {
    let phase =
        Phase::new("phase-1", "Timed Phase", "Has duration").with_estimated_duration("2-3 days");

    assert_eq!(phase.estimated_duration(), Some("2-3 days"));
}

#[test]
fn test_phase_builder_pattern() {
    let task = Task::new("Test Task", "Description");
    let dep = PhaseId::new("phase-0");

    let phase = Phase::new("phase-1", "Complete Phase", "Full phase")
        .with_tasks(vec![task])
        .with_dependencies(vec![dep])
        .with_estimated_duration("1 week");

    assert_eq!(phase.tasks().len(), 1);
    assert_eq!(phase.dependencies().len(), 1);
    assert_eq!(phase.estimated_duration(), Some("1 week"));
}

// ============================================================================
// PhaseId Tests
// ============================================================================

#[test]
fn test_phase_id_creation() {
    let id = PhaseId::new("phase-1");
    assert_eq!(id.as_str(), "phase-1");
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
fn test_phase_id_from_str() {
    let id = PhaseId::from("phase-1");
    assert_eq!(id.as_str(), "phase-1");
}

// ============================================================================
// Task Model Tests
// ============================================================================

#[test]
fn test_task_creation_with_valid_data() {
    let task = Task::new("Test Task", "A test task description");

    assert_eq!(task.name(), "Test Task");
    assert_eq!(task.description(), "A test task description");
    assert_eq!(task.acceptance_criteria().len(), 0);
    assert_eq!(task.components().len(), 0);
}

#[test]
fn test_task_with_acceptance_criteria() {
    let criteria = vec![
        "Criterion 1".to_string(),
        "Criterion 2".to_string(),
        "Criterion 3".to_string(),
    ];
    let task = Task::new("Test Task", "Description").with_acceptance_criteria(criteria);

    assert_eq!(task.acceptance_criteria().len(), 3);
    assert_eq!(task.acceptance_criteria()[0], "Criterion 1");
}

#[test]
fn test_task_with_components() {
    let components = vec!["Component A".to_string(), "Component B".to_string()];
    let task = Task::new("Test Task", "Description").with_components(components);

    assert_eq!(task.components().len(), 2);
    assert_eq!(task.components()[0], "Component A");
    assert_eq!(task.components()[1], "Component B");
}

#[test]
fn test_task_builder_pattern() {
    let task = Task::new("Complete Task", "Full task")
        .with_acceptance_criteria(vec!["AC1".to_string(), "AC2".to_string()])
        .with_components(vec!["Comp1".to_string()]);

    assert_eq!(task.acceptance_criteria().len(), 2);
    assert_eq!(task.components().len(), 1);
}

// ============================================================================
// ArchitectureDocument Tests
// ============================================================================

#[test]
fn test_architecture_document_creation() {
    let doc = ArchitectureDocument::new("Test Architecture");

    assert_eq!(doc.title(), "Test Architecture");
    assert_eq!(doc.description(), None);
    assert_eq!(doc.sections().len(), 0);
    assert_eq!(doc.components().len(), 0);
    assert_eq!(doc.requirements().len(), 0);
}

#[test]
fn test_architecture_document_with_description() {
    let doc =
        ArchitectureDocument::new("Test Architecture").with_description("Detailed description");

    assert_eq!(doc.description(), Some("Detailed description"));
}

#[test]
fn test_architecture_document_add_section() {
    let mut doc = ArchitectureDocument::new("Test Architecture");
    let section = Section::new("Overview", 1).with_content("Test content");
    doc.add_section(section);

    assert_eq!(doc.sections().len(), 1);
    assert_eq!(doc.sections()[0].heading(), "Overview");
}

#[test]
fn test_architecture_document_add_component() {
    let mut doc = ArchitectureDocument::new("Test Architecture");
    let component =
        Component::new("TestComponent", "test-comp").with_description("A test component");
    doc.add_component(component);

    assert_eq!(doc.components().len(), 1);
    assert_eq!(doc.components()[0].name(), "TestComponent");
}

#[test]
fn test_architecture_document_add_requirement() {
    let mut doc = ArchitectureDocument::new("Test Architecture");
    let requirement = Requirement::new("REQ-1", "Test requirement").with_priority("High");
    doc.add_requirement(requirement);

    assert_eq!(doc.requirements().len(), 1);
    assert_eq!(doc.requirements()[0].id(), "REQ-1");
}

#[test]
fn test_architecture_document_complex_structure() {
    let mut doc =
        ArchitectureDocument::new("Complex Architecture").with_description("A complex system");

    let section1 = Section::new("Section 1", 1).with_content("Content 1");
    let section2 = Section::new("Section 2", 1).with_content("Content 2");
    doc.add_section(section1);
    doc.add_section(section2);

    let comp1 = Component::new("Component1", "comp-1");
    let comp2 = Component::new("Component2", "comp-2");
    doc.add_component(comp1);
    doc.add_component(comp2);

    let req1 = Requirement::new("REQ-1", "Requirement 1");
    let req2 = Requirement::new("REQ-2", "Requirement 2");
    doc.add_requirement(req1);
    doc.add_requirement(req2);

    assert_eq!(doc.sections().len(), 2);
    assert_eq!(doc.components().len(), 2);
    assert_eq!(doc.requirements().len(), 2);
}

// ============================================================================
// Section Tests
// ============================================================================

#[test]
fn test_section_creation() {
    let section = Section::new("Test Section", 1);

    assert_eq!(section.heading(), "Test Section");
    assert_eq!(section.level(), 1);
    assert_eq!(section.content(), "");
    assert_eq!(section.subsections().len(), 0);
}

#[test]
fn test_section_with_content() {
    let section = Section::new("Test Section", 1).with_content("Test content here");

    assert_eq!(section.content(), "Test content here");
}

#[test]
fn test_section_with_subsections() {
    let subsection1 = Section::new("Subsection 1", 2);
    let subsection2 = Section::new("Subsection 2", 2);
    let section =
        Section::new("Parent Section", 1).with_subsections(vec![subsection1, subsection2]);

    assert_eq!(section.subsections().len(), 2);
    assert_eq!(section.subsections()[0].heading(), "Subsection 1");
    assert_eq!(section.subsections()[0].level(), 2);
}

#[test]
fn test_section_hierarchy() {
    let subsub = Section::new("Subsub", 3);
    let sub = Section::new("Sub", 2).with_subsections(vec![subsub]);
    let section = Section::new("Parent", 1).with_subsections(vec![sub]);

    assert_eq!(section.level(), 1);
    assert_eq!(section.subsections()[0].level(), 2);
    assert_eq!(section.subsections()[0].subsections()[0].level(), 3);
}

// ============================================================================
// Component Tests
// ============================================================================

#[test]
fn test_component_creation() {
    let component = Component::new("TestComponent", "test-comp");

    assert_eq!(component.name(), "TestComponent");
    assert_eq!(component.id(), "test-comp");
    assert_eq!(component.description(), None);
    assert_eq!(component.technology(), None);
}

#[test]
fn test_component_with_description() {
    let component = Component::new("TestComponent", "test-comp")
        .with_description("A test component for testing");

    assert_eq!(
        component.description(),
        Some("A test component for testing")
    );
}

#[test]
fn test_component_with_type() {
    let component = Component::new("TestComponent", "test-comp").with_technology("Service");

    assert_eq!(component.technology(), Some("Service"));
}

#[test]
fn test_component_full_builder() {
    let component = Component::new("FullComponent", "full-comp")
        .with_description("Complete component")
        .with_technology("Module");

    assert_eq!(component.name(), "FullComponent");
    assert_eq!(component.id(), "full-comp");
    assert_eq!(component.description(), Some("Complete component"));
    assert_eq!(component.technology(), Some("Module"));
}

// ============================================================================
// Requirement Tests
// ============================================================================

#[test]
fn test_requirement_creation() {
    let requirement = Requirement::new("REQ-001", "Test requirement");

    assert_eq!(requirement.id(), "REQ-001");
    assert_eq!(requirement.description(), "Test requirement");
    assert_eq!(requirement.priority(), None);
    assert_eq!(requirement.category(), None);
}

#[test]
fn test_requirement_with_priority() {
    let requirement =
        Requirement::new("REQ-001", "High priority requirement").with_priority("High");

    assert_eq!(requirement.priority(), Some("High"));
}

#[test]
fn test_requirement_with_category() {
    let requirement =
        Requirement::new("REQ-001", "Functional requirement").with_category("Functional");

    assert_eq!(requirement.category(), Some("Functional"));
}

#[test]
fn test_requirement_full_builder() {
    let requirement = Requirement::new("REQ-001", "Complete requirement")
        .with_priority("Critical")
        .with_category("Security");

    assert_eq!(requirement.id(), "REQ-001");
    assert_eq!(requirement.description(), "Complete requirement");
    assert_eq!(requirement.priority(), Some("Critical"));
    assert_eq!(requirement.category(), Some("Security"));
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_plan_with_long_title() {
    let long_title = "A".repeat(1000);
    let metadata = PlanMetadata::new(Utc::now(), "test-model", PathBuf::from("test.md"));
    let plan = Plan::new(&long_title, "Description", vec![], metadata);

    assert_eq!(plan.title().len(), 1000);
}

#[test]
fn test_phase_with_empty_strings() {
    let phase = Phase::new("", "", "");

    assert_eq!(phase.name(), "");
    assert_eq!(phase.description(), "");
}

#[test]
fn test_task_with_many_acceptance_criteria() {
    let criteria: Vec<String> = (0..100).map(|i| format!("Criterion {}", i)).collect();
    let task = Task::new("Task", "Description").with_acceptance_criteria(criteria);

    assert_eq!(task.acceptance_criteria().len(), 100);
}

#[test]
fn test_architecture_document_with_many_sections() {
    let mut doc = ArchitectureDocument::new("Large Doc");

    for i in 0..50 {
        let section = Section::new(format!("Section {}", i), 1);
        doc.add_section(section);
    }

    assert_eq!(doc.sections().len(), 50);
}

#[test]
fn test_phase_dependencies_uniqueness() {
    let dep1 = PhaseId::new("phase-0");
    let dep2 = PhaseId::new("phase-0");
    let phase = Phase::new("phase-1", "Test", "Test").with_dependencies(vec![dep1, dep2]);

    // Both dependencies should be present (no automatic deduplication)
    assert_eq!(phase.dependencies().len(), 2);
}

// ============================================================================
// Integration Tests - Model Composition
// ============================================================================

#[test]
fn test_complete_plan_structure() {
    // Create a complete plan with all components
    let metadata = PlanMetadata::new(Utc::now(), "llama3.2:3b", PathBuf::from("architecture.md"));

    let task1 = Task::new("Implement feature", "Implement the core feature")
        .with_acceptance_criteria(vec!["Feature works".to_string(), "Tests pass".to_string()])
        .with_components(vec!["Module A".to_string(), "Module B".to_string()]);

    let task2 = Task::new("Write tests", "Add comprehensive tests")
        .with_acceptance_criteria(vec!["Coverage > 80%".to_string()])
        .with_components(vec!["Test Suite".to_string()]);

    let phase1 = Phase::new("phase-1", "Implementation", "Build the feature")
        .with_tasks(vec![task1])
        .with_estimated_duration("2-3 days");

    let phase2 = Phase::new("phase-2", "Testing", "Test the feature")
        .with_tasks(vec![task2])
        .with_dependencies(vec![PhaseId::new("phase-1")])
        .with_estimated_duration("1 day");

    let plan = Plan::new(
        "Feature Development Plan",
        "Plan for implementing a new feature",
        vec![phase1, phase2],
        metadata,
    );

    assert_eq!(plan.title(), "Feature Development Plan");
    assert_eq!(plan.phases().len(), 2);
    assert_eq!(plan.phases()[0].tasks().len(), 1);
    assert_eq!(plan.phases()[1].dependencies().len(), 1);
}

#[test]
fn test_architecture_document_full_workflow() {
    let mut doc = ArchitectureDocument::new("Microservices Architecture")
        .with_description("A scalable microservices system");

    // Add sections
    let overview =
        Section::new("Overview", 1).with_content("This system uses microservices architecture");
    let design =
        Section::new("Design", 1).with_content("The design follows hexagonal architecture");
    doc.add_section(overview);
    doc.add_section(design);

    // Add components
    let auth_service = Component::new("AuthService", "auth-svc")
        .with_description("Handles authentication")
        .with_technology("Microservice");
    let api_gateway = Component::new("APIGateway", "api-gw")
        .with_description("Routes requests")
        .with_technology("Gateway");
    doc.add_component(auth_service);
    doc.add_component(api_gateway);

    // Add requirements
    let req1 = Requirement::new("REQ-001", "Must support OAuth2")
        .with_priority("High")
        .with_category("Security");
    let req2 = Requirement::new("REQ-002", "Must scale to 10k RPS")
        .with_priority("Medium")
        .with_category("Performance");
    doc.add_requirement(req1);
    doc.add_requirement(req2);

    assert_eq!(doc.title(), "Microservices Architecture");
    assert_eq!(doc.sections().len(), 2);
    assert_eq!(doc.components().len(), 2);
    assert_eq!(doc.requirements().len(), 2);
}
