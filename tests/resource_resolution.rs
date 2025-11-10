// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Unit tests for resource resolution hierarchy
//!
//! This test file validates the resource resolution system including:
//! - Environment variable precedence
//! - XDG directory resolution
//! - Custom directory overrides
//! - Fallback to embedded resources
//! - Resolution hierarchy correctness

use serial_test::serial;
use std::env;
use std::fs;
use tempfile::TempDir;
use xzagentz::components::resolution::{
    default_components_dir, resolve_component_dir, ResourceSource,
};
use xzagentz::config::resolution::{default_config_dir, resolve_config_dir};
use xzagentz::templates::resolution::{default_templates_dir, resolve_template_dir};

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

    fn clear_all(&self) {
        for (name, _) in &self.vars {
            env::remove_var(name);
        }
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

// Component resolution tests

#[test]
#[serial]
fn test_resolve_component_custom_dir_takes_precedence() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    let temp1 = TempDir::new().unwrap();
    let temp2 = TempDir::new().unwrap();

    env::set_var("XZAGENTZ_COMPONENTS_DIR", temp2.path());

    let source = resolve_component_dir(Some(temp1.path().to_path_buf()));

    assert!(source.is_filesystem());
    if let ResourceSource::Filesystem(path) = source {
        assert_eq!(path, temp1.path());
    }
}

#[test]
#[serial]
fn test_resolve_component_env_var_precedence() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    let temp = TempDir::new().unwrap();

    env::set_var("XZAGENTZ_COMPONENTS_DIR", temp.path());

    let source = resolve_component_dir(None);

    assert!(source.is_filesystem());
    if let ResourceSource::Filesystem(path) = source {
        assert_eq!(path, temp.path());
    }
}

#[test]
#[serial]
fn test_resolve_component_embedded_fallback() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    _guard.clear_all();

    let source = resolve_component_dir(None);

    assert!(source.is_embedded());
}

#[test]
#[serial]
fn test_default_components_dir_respects_home() {
    let _guard = EnvGuard::new(&["HOME"]);
    let temp = TempDir::new().unwrap();

    env::set_var("HOME", temp.path());

    let dir = default_components_dir();
    let expected = temp
        .path()
        .join(".config")
        .join("xzagentz")
        .join("components");

    assert_eq!(dir, expected);
}

// Template resolution tests

#[test]
#[serial]
fn test_resolve_template_env_var_precedence() {
    let _guard = EnvGuard::new(&["XZAGENTZ_TEMPLATES_DIR", "XDG_DATA_HOME", "HOME"]);
    let temp = TempDir::new().unwrap();

    env::set_var("XZAGENTZ_TEMPLATES_DIR", temp.path());

    let source = resolve_template_dir(None);

    assert!(source.is_filesystem());
    if let ResourceSource::Filesystem(path) = source {
        assert_eq!(path, temp.path());
    }
}

#[test]
#[serial]
fn test_resolve_template_embedded_fallback() {
    let _guard = EnvGuard::new(&["XZAGENTZ_TEMPLATES_DIR", "XDG_DATA_HOME", "HOME"]);
    _guard.clear_all();

    let source = resolve_template_dir(None);

    assert!(source.is_embedded());
}

#[test]
#[serial]
fn test_default_templates_dir_respects_home() {
    let _guard = EnvGuard::new(&["HOME"]);
    let temp = TempDir::new().unwrap();

    env::set_var("HOME", temp.path());

    let dir = default_templates_dir();
    let expected = temp
        .path()
        .join(".config")
        .join("xzagentz")
        .join("templates");

    assert_eq!(dir, expected);
}

// Config resolution tests

#[test]
#[serial]
fn test_resolve_config_env_var_precedence() {
    let _guard = EnvGuard::new(&["XZAGENTZ_CONFIG_DIR", "XDG_CONFIG_HOME", "HOME"]);
    let temp = TempDir::new().unwrap();

    env::set_var("XZAGENTZ_CONFIG_DIR", temp.path());

    let resolved = resolve_config_dir(None);

    assert_eq!(resolved, temp.path());
}

#[test]
#[serial]
fn test_resolve_config_xdg_config_home_fallback() {
    let _guard = EnvGuard::new(&["XZAGENTZ_CONFIG_DIR", "XDG_CONFIG_HOME", "HOME"]);

    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("xzagentz");

    env::remove_var("XZAGENTZ_CONFIG_DIR");
    env::remove_var("HOME");
    env::set_var("XDG_CONFIG_HOME", temp.path());

    let resolved = resolve_config_dir(None);

    assert_eq!(resolved, config_dir);
}

#[test]
#[serial]
fn test_default_config_dir_respects_home() {
    let _guard = EnvGuard::new(&["HOME"]);
    let temp = TempDir::new().unwrap();

    env::set_var("HOME", temp.path());

    let dir = default_config_dir();
    let expected = temp.path().join(".config").join("xzagentz");

    assert_eq!(dir, expected);
}

// Cross-resource consistency tests

#[test]
#[serial]
fn test_all_defaults_use_consistent_base_path() {
    let _guard = EnvGuard::new(&["HOME"]);
    let temp = TempDir::new().unwrap();
    let temp_path = temp.path().to_path_buf();

    env::set_var("HOME", &temp_path);

    let config_dir = default_config_dir();
    let components_dir = default_components_dir();
    let templates_dir = default_templates_dir();

    // All should use the same structure pattern (.config/xzagentz)
    // Note: Each module reads HOME independently, so verify structure not exact paths
    assert!(config_dir.ends_with(".config/xzagentz"));
    assert!(components_dir.ends_with(".config/xzagentz/components"));
    assert!(templates_dir.ends_with(".config/xzagentz/templates"));

    // Verify consistent parent structure
    assert_eq!(config_dir.file_name().unwrap(), "xzagentz");
    assert_eq!(
        components_dir.parent().unwrap().file_name().unwrap(),
        "xzagentz"
    );
    assert_eq!(
        templates_dir.parent().unwrap().file_name().unwrap(),
        "xzagentz"
    );
}

#[test]
#[serial]
fn test_resolution_hierarchy_independence() {
    let _guard = EnvGuard::new(&[
        "XZAGENTZ_COMPONENTS_DIR",
        "XZAGENTZ_TEMPLATES_DIR",
        "XZAGENTZ_CONFIG_DIR",
        "XDG_DATA_HOME",
        "XDG_CONFIG_HOME",
        "HOME",
    ]);

    let temp1 = TempDir::new().unwrap();
    let temp2 = TempDir::new().unwrap();
    let temp3 = TempDir::new().unwrap();

    let temp1_path = temp1.path().to_path_buf();
    let temp2_path = temp2.path().to_path_buf();
    let temp3_path = temp3.path().to_path_buf();

    // Set different paths for each resource type
    env::set_var("XZAGENTZ_COMPONENTS_DIR", &temp1_path);
    env::set_var("XZAGENTZ_TEMPLATES_DIR", &temp2_path);
    env::set_var("XZAGENTZ_CONFIG_DIR", &temp3_path);

    let component_source = resolve_component_dir(None);
    let template_source = resolve_template_dir(None);
    let config_dir = resolve_config_dir(None);

    // Each should resolve independently
    assert!(component_source.is_filesystem());
    if let ResourceSource::Filesystem(path) = component_source {
        assert_eq!(path, temp1_path);
    }

    assert!(template_source.is_filesystem());
    if let ResourceSource::Filesystem(path) = template_source {
        assert_eq!(path, temp2_path);
    }

    assert_eq!(config_dir, temp3_path);
}

#[test]
#[serial]
fn test_resolution_with_xdg_data_home() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    _guard.clear_all();

    let temp = TempDir::new().unwrap();
    let components_dir = temp.path().join("xzagentz").join("components");
    fs::create_dir_all(&components_dir).unwrap();

    // Set XDG_DATA_HOME to temp directory with components subdirectory
    env::set_var("XDG_DATA_HOME", temp.path());

    let source = resolve_component_dir(None);

    assert!(source.is_filesystem());
    if let ResourceSource::Filesystem(path) = source {
        assert_eq!(path, components_dir);
    }
}

#[test]
#[serial]
fn test_resolution_deterministic() {
    let _guard = EnvGuard::new(&["XZAGENTZ_COMPONENTS_DIR", "XDG_DATA_HOME", "HOME"]);
    _guard.clear_all();

    let temp = TempDir::new().unwrap();
    let temp_path = temp.path().to_path_buf();

    // Ensure temp directory exists (TempDir creates it, but verify)
    assert!(
        temp_path.exists(),
        "Temp directory should exist: {:?}",
        temp_path
    );

    env::set_var("XZAGENTZ_COMPONENTS_DIR", &temp_path);

    // Call multiple times - should be deterministic
    let first = resolve_component_dir(None);
    let second = resolve_component_dir(None);
    let third = resolve_component_dir(None);

    // All should return the same result
    assert_eq!(
        first, second,
        "First and second resolution should be identical"
    );
    assert_eq!(
        second, third,
        "Second and third resolution should be identical"
    );

    // All should resolve to the same temp path, not embedded
    assert!(
        first.is_filesystem(),
        "Should resolve to filesystem, not embedded. Got: {:?}",
        first
    );

    if let ResourceSource::Filesystem(path) = first {
        assert_eq!(path, temp_path, "Resolved path should match temp directory");
    } else {
        panic!("Expected Filesystem variant, got: {:?}", first);
    }
}
