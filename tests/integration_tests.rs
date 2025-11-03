//! Integration tests for xzagentz embedded resources system
//!
//! This module contains integration tests that validate complete workflows
//! and end-to-end functionality of the embedded resources system.

use serial_test::serial;
use std::env;
use std::fs;
use tempfile::TempDir;
use xzagentz::components::resolution::{resolve_component_dir, ResourceSource};
use xzagentz::config::resolution::{
    default_components_dir, default_config_dir, default_templates_dir,
};
use xzagentz::infrastructure::EmbeddedResources;
use xzagentz::templates::resolution::resolve_template_dir;

/// Helper to save and restore environment variables for test isolation
struct EnvGuard {
    vars: Vec<(String, Option<String>)>,
}

impl EnvGuard {
    fn new(var_names: &[&str]) -> Self {
        let vars = var_names
            .iter()
            .map(|name| (name.to_string(), env::var(name).ok()))
            .collect();
        Self { vars }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (name, value) in &self.vars {
            match value {
                Some(val) => env::set_var(name, val),
                None => env::remove_var(name),
            }
        }
    }
}

#[test]
#[serial]
fn test_init_workflow_first_time_initialization() {
    let _guard = EnvGuard::new(&["HOME", "XDG_CONFIG_HOME", "XDG_DATA_HOME"]);
    let temp_home = TempDir::new().unwrap();
    env::set_var("HOME", temp_home.path());

    let resources = EmbeddedResources::new();

    // Get default directories
    let config_dir = default_config_dir();
    let components_dir = default_components_dir();
    let templates_dir = default_templates_dir();

    // Create directories
    fs::create_dir_all(&config_dir).unwrap();
    fs::create_dir_all(&components_dir).unwrap();
    fs::create_dir_all(&templates_dir).unwrap();

    // Extract resources
    let result = resources.extract_all_to(&config_dir);
    assert!(
        result.is_ok(),
        "Resource extraction should succeed: {:?}",
        result.err()
    );

    // Verify components were extracted
    let extracted_components_dir = config_dir.join("components");
    let component_list = resources.list_components();
    assert!(!component_list.is_empty(), "Should have components");

    for component in &component_list {
        let component_path = extracted_components_dir.join(component);
        assert!(
            component_path.exists(),
            "Component should be extracted: {}",
            component
        );
    }

    // Verify templates were extracted
    let extracted_templates_dir = config_dir.join("templates");
    let template_list = resources.list_templates();
    assert!(!template_list.is_empty(), "Should have templates");

    for template in &template_list {
        let template_path = extracted_templates_dir.join(template);
        assert!(
            template_path.exists(),
            "Template should be extracted: {}",
            template
        );
    }
}

#[test]
#[serial]
fn test_resource_loading_from_embedded() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    env::remove_var("XZAGENTZ_COMPONENTS_DIR");
    env::remove_var("XDG_DATA_HOME");
    env::remove_var("HOME");

    let resources = EmbeddedResources::new();

    // Resolve should return embedded
    let source = resolve_component_dir(None);
    assert!(
        source.is_embedded(),
        "Should use embedded resources when no filesystem resources exist"
    );

    // Load components from embedded
    let components = resources.list_components();
    assert!(
        !components.is_empty(),
        "Embedded resources should contain components"
    );

    for component in &components {
        let content = resources.get_component(component);
        assert!(
            content.is_some(),
            "Should be able to load component from embedded: {}",
            component
        );
        assert!(
            !content.unwrap().is_empty(),
            "Component content should not be empty"
        );
    }
}

#[test]
#[serial]
fn test_resource_loading_from_filesystem() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    let temp_dir = TempDir::new().unwrap();
    let components_dir = temp_dir.path().join("components");

    let resources = EmbeddedResources::new();

    // Extract resources to filesystem
    resources.extract_components_to(&components_dir).unwrap();

    // Set environment to use filesystem
    env::set_var("XZAGENTZ_COMPONENTS_DIR", &components_dir);

    // Resolve should return filesystem
    let source = resolve_component_dir(None);
    assert!(
        source.is_filesystem(),
        "Should use filesystem when resources are extracted"
    );

    if let ResourceSource::Filesystem(path) = source {
        assert_eq!(path, components_dir);

        // Verify we can load from filesystem
        let components = resources.list_components();
        for component in &components {
            let component_path = path.join(component);
            assert!(
                component_path.exists(),
                "Component should exist on filesystem: {}",
                component
            );

            let content = fs::read_to_string(&component_path).unwrap();
            assert!(
                !content.is_empty(),
                "Filesystem content should not be empty"
            );
        }
    }
}

#[test]
#[serial]
fn test_config_resolution_integration() {
    let _guard = EnvGuard::new(&["HOME", "XDG_CONFIG_HOME", "XZAGENTZ_CONFIG_DIR"]);
    let temp_home = TempDir::new().unwrap();
    env::set_var("HOME", temp_home.path());

    let resources = EmbeddedResources::new();

    // Get default config directory
    let config_dir = default_config_dir();

    // Create directory structure and extract resources
    fs::create_dir_all(&config_dir).unwrap();
    let result = resources.extract_all_to(&config_dir);
    assert!(result.is_ok(), "Resource extraction should succeed");

    // Verify structure was created
    assert!(config_dir.exists());
    assert!(config_dir.join("components").exists());
    assert!(config_dir.join("templates").exists());

    // Verify resources were extracted
    let components = resources.list_components();
    for component in &components {
        let component_path = config_dir.join("components").join(component);
        assert!(
            component_path.exists(),
            "Component should be extracted: {}",
            component
        );
    }
}

#[test]
#[serial]
fn test_mixed_sources_components_and_templates() {
    let _guard = EnvGuard::new(&[
        "XZAGENTZ_COMPONENTS_DIR",
        "XZAGENTZ_TEMPLATES_DIR",
        "XDG_DATA_HOME",
        "HOME",
    ]);

    // Clear all environment variables first for proper isolation
    env::remove_var("XZAGENTZ_COMPONENTS_DIR");
    env::remove_var("XZAGENTZ_TEMPLATES_DIR");
    env::remove_var("XDG_DATA_HOME");
    env::remove_var("HOME");

    // Use completely isolated temp directory
    let temp_dir = TempDir::new().unwrap();
    let components_dir = temp_dir.path().join("components");

    let resources = EmbeddedResources::new();

    // Extract only components to filesystem
    resources.extract_components_to(&components_dir).unwrap();

    // Set environment variables to use isolated paths
    env::set_var("XZAGENTZ_COMPONENTS_DIR", &components_dir);
    // Don't set templates dir, XDG, or HOME - let them be unset for embedded fallback

    // Components should resolve to filesystem
    let component_source = resolve_component_dir(None);
    assert!(
        component_source.is_filesystem(),
        "Components should use filesystem"
    );

    // Templates should resolve to embedded (not extracted, all paths nonexistent)
    let template_source = resolve_template_dir(None);
    assert!(
        template_source.is_embedded(),
        "Templates should use embedded"
    );

    // Verify we can load from both sources
    let components = resources.list_components();
    assert!(!components.is_empty(), "Should have components");

    let templates = resources.list_templates();
    assert!(!templates.is_empty(), "Should have templates");
}

#[test]
fn test_content_integrity_after_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let resources = EmbeddedResources::new();

    resources.extract_all_to(temp_dir.path()).unwrap();

    // Verify extracted content matches embedded content
    let components_dir = temp_dir.path().join("components");
    let component_list = resources.list_components();

    for component in &component_list {
        let component_path = components_dir.join(component);
        let extracted_content = fs::read_to_string(&component_path).unwrap();
        let embedded_content = resources.get_component(component).unwrap();

        assert_eq!(
            extracted_content, embedded_content,
            "Extracted content should match embedded content for: {}",
            component
        );
    }
}

#[test]
#[serial]
fn test_default_resource_dirs_consistency() {
    let _guard = EnvGuard::new(&["HOME"]);
    let temp_home = TempDir::new().unwrap();
    env::set_var("HOME", temp_home.path());

    let config_dir = default_config_dir();
    let components_dir = default_components_dir();
    let templates_dir = default_templates_dir();

    // All should use the same base pattern (.config/xzagentz)
    // Note: Each module reads HOME independently, so paths share structure but not temp dir
    assert!(config_dir.ends_with(".config/xzagentz"));
    assert!(components_dir.ends_with(".config/xzagentz/components"));
    assert!(templates_dir.ends_with(".config/xzagentz/templates"));

    // Verify they all have the same parent structure
    let config_parent = config_dir.file_name().unwrap();
    let components_grandparent = components_dir.parent().unwrap().file_name().unwrap();
    let templates_grandparent = templates_dir.parent().unwrap().file_name().unwrap();

    assert_eq!(config_parent, "xzagentz");
    assert_eq!(components_grandparent, "xzagentz");
    assert_eq!(templates_grandparent, "xzagentz");
}
