// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Unit tests for embedded resources functionality
//!
//! This test file validates the embedded resources infrastructure including:
//! - Resource extraction to filesystem
//! - Resource listing and enumeration
//! - Resource content retrieval
//! - Error handling for edge cases

use std::fs;
use tempfile::TempDir;
use xzagentz::infrastructure::EmbeddedResources;

#[test]
fn test_embedded_resources_new_creates_instance() {
    let resources = EmbeddedResources::new();
    // Should create successfully
    assert_eq!(std::mem::size_of_val(&resources), std::mem::size_of::<()>());
}

#[test]
fn test_embedded_resources_default_creates_instance() {
    let resources = EmbeddedResources::default();
    // Should create successfully via Default trait
    assert_eq!(std::mem::size_of_val(&resources), std::mem::size_of::<()>());
}

#[test]
fn test_list_components_returns_paths() {
    let resources = EmbeddedResources::new();
    let components = resources.list_components();

    // Should have at least some components embedded
    assert!(!components.is_empty(), "Expected embedded components");

    // All paths should be relative
    for component in &components {
        assert!(
            !component.starts_with('/'),
            "Component path should be relative: {}",
            component
        );
    }
}

#[test]
fn test_list_templates_returns_paths() {
    let resources = EmbeddedResources::new();
    let templates = resources.list_templates();

    // Should have at least some templates embedded
    assert!(!templates.is_empty(), "Expected embedded templates");

    // All paths should be relative
    for template in &templates {
        assert!(
            !template.starts_with('/'),
            "Template path should be relative: {}",
            template
        );
    }
}

#[test]
fn test_get_component_existing_returns_content() {
    let resources = EmbeddedResources::new();
    let components = resources.list_components();

    // Get first component
    if let Some(first_component) = components.first() {
        let content = resources.get_component(first_component);
        assert!(
            content.is_some(),
            "Expected content for component: {}",
            first_component
        );

        // Content should not be empty
        let content = content.unwrap();
        assert!(!content.is_empty(), "Component content should not be empty");
    }
}

#[test]
fn test_get_template_existing_returns_content() {
    let resources = EmbeddedResources::new();
    let templates = resources.list_templates();

    // Get first template
    if let Some(first_template) = templates.first() {
        let content = resources.get_template(first_template);
        assert!(
            content.is_some(),
            "Expected content for template: {}",
            first_template
        );

        // Content should not be empty
        let content = content.unwrap();
        assert!(!content.is_empty(), "Template content should not be empty");
    }
}

#[test]
fn test_get_component_nonexistent_returns_none() {
    let resources = EmbeddedResources::new();
    let content = resources.get_component("nonexistent/component.yaml");
    assert!(content.is_none(), "Expected None for nonexistent component");
}

#[test]
fn test_get_template_nonexistent_returns_none() {
    let resources = EmbeddedResources::new();
    let content = resources.get_template("nonexistent/template.yaml");
    assert!(content.is_none(), "Expected None for nonexistent template");
}

#[test]
fn test_has_component_existing_returns_true() {
    let resources = EmbeddedResources::new();
    let components = resources.list_components();

    if let Some(first_component) = components.first() {
        assert!(
            resources.has_component(first_component),
            "Expected has_component to return true for existing component"
        );
    }
}

#[test]
fn test_has_component_nonexistent_returns_false() {
    let resources = EmbeddedResources::new();
    assert!(
        !resources.has_component("nonexistent/component.yaml"),
        "Expected has_component to return false for nonexistent component"
    );
}

#[test]
fn test_has_template_existing_returns_true() {
    let resources = EmbeddedResources::new();
    let templates = resources.list_templates();

    if let Some(first_template) = templates.first() {
        assert!(
            resources.has_template(first_template),
            "Expected has_template to return true for existing template"
        );
    }
}

#[test]
fn test_has_template_nonexistent_returns_false() {
    let resources = EmbeddedResources::new();
    assert!(
        !resources.has_template("nonexistent/template.yaml"),
        "Expected has_template to return false for nonexistent template"
    );
}

#[test]
fn test_extract_components_to_creates_files() {
    let resources = EmbeddedResources::new();
    let temp_dir = TempDir::new().unwrap();

    // Extract components
    let result = resources.extract_components_to(temp_dir.path());
    assert!(
        result.is_ok(),
        "Expected successful component extraction: {:?}",
        result.err()
    );

    // Verify files were created
    let components = resources.list_components();
    for component in components {
        let target_path = temp_dir.path().join(&component);
        assert!(
            target_path.exists(),
            "Expected component to be extracted: {}",
            target_path.display()
        );

        // Verify content matches
        let extracted_content = fs::read_to_string(&target_path).unwrap();
        let embedded_content = resources.get_component(&component).unwrap();
        assert_eq!(
            extracted_content, embedded_content,
            "Extracted content should match embedded content"
        );
    }
}

#[test]
fn test_extract_templates_to_creates_files() {
    let resources = EmbeddedResources::new();
    let temp_dir = TempDir::new().unwrap();

    // Extract templates
    let result = resources.extract_templates_to(temp_dir.path());
    assert!(
        result.is_ok(),
        "Expected successful template extraction: {:?}",
        result.err()
    );

    // Verify files were created
    let templates = resources.list_templates();
    for template in templates {
        let target_path = temp_dir.path().join(&template);
        assert!(
            target_path.exists(),
            "Expected template to be extracted: {}",
            target_path.display()
        );

        // Verify content matches
        let extracted_content = fs::read_to_string(&target_path).unwrap();
        let embedded_content = resources.get_template(&template).unwrap();
        assert_eq!(
            extracted_content, embedded_content,
            "Extracted content should match embedded content"
        );
    }
}

#[test]
fn test_extract_all_to_creates_both_components_and_templates() {
    let resources = EmbeddedResources::new();
    let temp_dir = TempDir::new().unwrap();

    // Extract all resources
    let result = resources.extract_all_to(temp_dir.path());
    assert!(
        result.is_ok(),
        "Expected successful extraction: {:?}",
        result.err()
    );

    // Verify components directory
    let components_dir = temp_dir.path().join("components");
    assert!(
        components_dir.exists(),
        "Expected components directory to be created"
    );

    // Verify templates directory
    let templates_dir = temp_dir.path().join("templates");
    assert!(
        templates_dir.exists(),
        "Expected templates directory to be created"
    );

    // Verify at least one component was extracted
    let components = resources.list_components();
    for component in components {
        let target_path = components_dir.join(&component);
        assert!(
            target_path.exists(),
            "Expected component to be extracted: {}",
            target_path.display()
        );
    }

    // Verify at least one template was extracted
    let templates = resources.list_templates();
    for template in templates {
        let target_path = templates_dir.join(&template);
        assert!(
            target_path.exists(),
            "Expected template to be extracted: {}",
            target_path.display()
        );
    }
}

#[test]
fn test_extract_preserves_directory_structure() {
    let resources = EmbeddedResources::new();
    let temp_dir = TempDir::new().unwrap();

    // Extract components
    resources.extract_components_to(temp_dir.path()).unwrap();

    // Check that directory structure is preserved
    let components = resources.list_components();
    for component in components {
        let target_path = temp_dir.path().join(&component);
        assert!(
            target_path.exists(),
            "Expected nested path to exist: {}",
            target_path.display()
        );

        // Verify parent directories were created
        if let Some(parent) = target_path.parent() {
            assert!(
                parent.exists(),
                "Expected parent directory to exist: {}",
                parent.display()
            );
        }
    }
}

#[test]
fn test_extract_creates_parent_directories() {
    let resources = EmbeddedResources::new();
    let temp_dir = TempDir::new().unwrap();
    let nested_target = temp_dir.path().join("level1").join("level2").join("level3");

    // Extract to deeply nested path
    let result = resources.extract_components_to(&nested_target);
    assert!(
        result.is_ok(),
        "Expected successful extraction with parent directory creation: {:?}",
        result.err()
    );

    // Verify nested directories were created
    assert!(nested_target.exists(), "Expected nested target to exist");

    // Verify components were extracted
    let components = resources.list_components();
    if let Some(first_component) = components.first() {
        let target_path = nested_target.join(first_component);
        assert!(
            target_path.exists(),
            "Expected component to be extracted to nested path"
        );
    }
}

#[test]
fn test_extract_to_existing_directory_succeeds() {
    let resources = EmbeddedResources::new();
    let temp_dir = TempDir::new().unwrap();

    // Extract once
    resources.extract_components_to(temp_dir.path()).unwrap();

    // Extract again to same location (should succeed, overwriting)
    let result = resources.extract_components_to(temp_dir.path());
    assert!(
        result.is_ok(),
        "Expected successful extraction to existing directory: {:?}",
        result.err()
    );

    // Verify files still exist and have correct content
    let components = resources.list_components();
    if let Some(first_component) = components.first() {
        let target_path = temp_dir.path().join(first_component);
        assert!(
            target_path.exists(),
            "Expected component to exist after re-extraction"
        );

        let extracted_content = fs::read_to_string(&target_path).unwrap();
        let embedded_content = resources.get_component(first_component).unwrap();
        assert_eq!(
            extracted_content, embedded_content,
            "Content should match after re-extraction"
        );
    }
}

#[test]
fn test_list_components_consistent_across_calls() {
    let resources = EmbeddedResources::new();

    let first_list = resources.list_components();
    let second_list = resources.list_components();

    assert_eq!(
        first_list.len(),
        second_list.len(),
        "Component list should be consistent"
    );

    for (first, second) in first_list.iter().zip(second_list.iter()) {
        assert_eq!(
            first, second,
            "Component paths should be consistent across calls"
        );
    }
}

#[test]
fn test_list_templates_consistent_across_calls() {
    let resources = EmbeddedResources::new();

    let first_list = resources.list_templates();
    let second_list = resources.list_templates();

    assert_eq!(
        first_list.len(),
        second_list.len(),
        "Template list should be consistent"
    );

    for (first, second) in first_list.iter().zip(second_list.iter()) {
        assert_eq!(
            first, second,
            "Template paths should be consistent across calls"
        );
    }
}

#[test]
fn test_component_content_immutable() {
    let resources = EmbeddedResources::new();
    let components = resources.list_components();

    if let Some(first_component) = components.first() {
        let first_read = resources.get_component(first_component).unwrap();
        let second_read = resources.get_component(first_component).unwrap();

        assert_eq!(
            first_read, second_read,
            "Component content should be immutable"
        );
    }
}

#[test]
fn test_template_content_immutable() {
    let resources = EmbeddedResources::new();
    let templates = resources.list_templates();

    if let Some(first_template) = templates.first() {
        let first_read = resources.get_template(first_template).unwrap();
        let second_read = resources.get_template(first_template).unwrap();

        assert_eq!(
            first_read, second_read,
            "Template content should be immutable"
        );
    }
}
