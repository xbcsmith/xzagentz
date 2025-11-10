// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Resource resolution for component directories
//!
//! This module implements the resource resolution hierarchy for component directories:
//! 1. Environment variable overrides (XZAGENTZ_COMPONENTS_DIR, XDG_DATA_HOME)
//! 2. User configuration directory (~/.config/xzagentz/components)
//! 3. Embedded resources fallback
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::resolution::resolve_component_dir;
//!
//! let source = resolve_component_dir(None);
//! ```

use std::env;
use std::path::PathBuf;

/// Source of a resource (filesystem or embedded)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceSource {
    /// Resource loaded from filesystem
    Filesystem(PathBuf),
    /// Resource loaded from embedded binary
    Embedded,
}

impl ResourceSource {
    /// Returns true if the resource is from the filesystem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::resolution::ResourceSource;
    /// use std::path::PathBuf;
    ///
    /// let source = ResourceSource::Filesystem(PathBuf::from("/tmp/components"));
    /// assert!(source.is_filesystem());
    /// ```
    pub fn is_filesystem(&self) -> bool {
        matches!(self, ResourceSource::Filesystem(_))
    }

    /// Returns true if the resource is embedded
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::resolution::ResourceSource;
    ///
    /// let source = ResourceSource::Embedded;
    /// assert!(source.is_embedded());
    /// ```
    pub fn is_embedded(&self) -> bool {
        matches!(self, ResourceSource::Embedded)
    }
}

/// Resolves the component directory using the resource resolution hierarchy
///
/// # Resolution Order
///
/// 1. Custom directory if provided
/// 2. XZAGENTZ_COMPONENTS_DIR environment variable
/// 3. XDG_DATA_HOME/xzagentz/components
/// 4. ~/.config/xzagentz/components
/// 5. Embedded resources (fallback)
///
/// # Arguments
///
/// * `custom_dir` - Optional custom directory override
///
/// # Returns
///
/// Returns a `ResourceSource` indicating where components should be loaded from
///
/// # Examples
///
/// ```rust
/// use xzagentz::components::resolution::{resolve_component_dir, ResourceSource};
/// use std::path::PathBuf;
///
/// // With custom directory (may return embedded if dir doesn't exist)
/// let source = resolve_component_dir(Some(PathBuf::from("/custom/components")));
///
/// // Using resolution hierarchy (falls back to embedded if no dirs found)
/// let source = resolve_component_dir(None);
/// // Embedded resources are always available as fallback
/// ```
pub fn resolve_component_dir(custom_dir: Option<PathBuf>) -> ResourceSource {
    // 1. Custom directory takes precedence
    if let Some(dir) = custom_dir {
        if dir.exists() {
            return ResourceSource::Filesystem(dir);
        }
    }

    // 2. Check XZAGENTZ_COMPONENTS_DIR environment variable
    if let Ok(dir) = env::var("XZAGENTZ_COMPONENTS_DIR") {
        let path = PathBuf::from(dir);
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 3. Check XDG_DATA_HOME/xzagentz/components
    if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data).join("xzagentz").join("components");
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 4. Check ~/.config/xzagentz/components
    if let Some(home) = env::var_os("HOME") {
        let path = PathBuf::from(home)
            .join(".config")
            .join("xzagentz")
            .join("components");
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 5. Fall back to embedded resources
    ResourceSource::Embedded
}

/// Returns the default components directory path
///
/// This is the preferred location where users should extract components
/// for customization: ~/.config/xzagentz/components
///
/// # Returns
///
/// Returns the default components directory path
///
/// # Examples
///
/// ```rust
/// use xzagentz::components::resolution::default_components_dir;
///
/// let dir = default_components_dir();
/// assert!(dir.ends_with(".config/xzagentz/components"));
/// ```
pub fn default_components_dir() -> PathBuf {
    if let Some(home) = env::var_os("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("xzagentz")
            .join("components")
    } else {
        // Fallback for systems without HOME
        PathBuf::from(".config").join("xzagentz").join("components")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_resource_source_filesystem() {
        let source = ResourceSource::Filesystem(PathBuf::from("/tmp/components"));
        assert!(source.is_filesystem());
        assert!(!source.is_embedded());
    }

    #[test]
    fn test_resource_source_embedded() {
        let source = ResourceSource::Embedded;
        assert!(source.is_embedded());
        assert!(!source.is_filesystem());
    }

    #[test]
    fn test_resolve_with_custom_dir_existing() {
        let temp = TempDir::new().unwrap();
        let source = resolve_component_dir(Some(temp.path().to_path_buf()));
        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, temp.path());
        }
    }

    #[test]
    fn test_resolve_with_custom_dir_nonexistent() {
        // Clear all environment variables to ensure proper fallback to embedded
        let old_components = env::var_os("XZAGENTZ_COMPONENTS_DIR");
        let old_xdg_data = env::var_os("XDG_DATA_HOME");
        let old_home = env::var_os("HOME");

        env::remove_var("XZAGENTZ_COMPONENTS_DIR");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("HOME");

        let source = resolve_component_dir(Some(PathBuf::from("/nonexistent/path")));

        // Restore environment variables
        if let Some(val) = old_components {
            env::set_var("XZAGENTZ_COMPONENTS_DIR", val);
        }
        if let Some(val) = old_xdg_data {
            env::set_var("XDG_DATA_HOME", val);
        }
        if let Some(val) = old_home {
            env::set_var("HOME", val);
        }

        // Should fall back to embedded since custom dir doesn't exist and no env vars
        assert!(source.is_embedded());
    }

    #[test]
    fn test_resolve_with_env_var() {
        let temp = TempDir::new().unwrap();
        env::set_var("XZAGENTZ_COMPONENTS_DIR", temp.path());

        let source = resolve_component_dir(None);

        env::remove_var("XZAGENTZ_COMPONENTS_DIR");

        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, temp.path());
        }
    }

    #[test]
    fn test_resolve_fallback_to_embedded() {
        // Save and clear all env vars for proper test isolation
        let old_components = env::var_os("XZAGENTZ_COMPONENTS_DIR");
        let old_xdg_data = env::var_os("XDG_DATA_HOME");
        let old_home = env::var_os("HOME");

        env::remove_var("XZAGENTZ_COMPONENTS_DIR");
        env::remove_var("XDG_DATA_HOME");
        env::remove_var("HOME");

        // Use a nonexistent custom dir
        let source = resolve_component_dir(Some(PathBuf::from("/definitely/does/not/exist")));

        // Restore environment variables
        if let Some(val) = old_components {
            env::set_var("XZAGENTZ_COMPONENTS_DIR", val);
        }
        if let Some(val) = old_xdg_data {
            env::set_var("XDG_DATA_HOME", val);
        }
        if let Some(val) = old_home {
            env::set_var("HOME", val);
        }

        assert!(source.is_embedded());
    }

    #[test]
    fn test_default_components_dir_with_home() {
        if env::var_os("HOME").is_some() {
            let dir = default_components_dir();
            assert!(dir
                .to_string_lossy()
                .contains(".config/xzagentz/components"));
        }
    }

    #[test]
    fn test_default_components_dir_without_home() {
        let old_home = env::var_os("HOME");
        env::remove_var("HOME");

        let dir = default_components_dir();
        assert!(dir
            .to_string_lossy()
            .contains(".config/xzagentz/components"));

        // Restore HOME
        if let Some(home) = old_home {
            env::set_var("HOME", home);
        }
    }

    #[test]
    fn test_resolve_with_xdg_data_home() {
        let temp = TempDir::new().unwrap();
        let components_dir = temp.path().join("xzagentz").join("components");
        std::fs::create_dir_all(&components_dir).unwrap();

        env::set_var("XDG_DATA_HOME", temp.path());
        let source = resolve_component_dir(None);
        env::remove_var("XDG_DATA_HOME");

        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, components_dir);
        }
    }

    #[test]
    fn test_custom_dir_takes_precedence() {
        let temp1 = TempDir::new().unwrap();
        let temp2 = TempDir::new().unwrap();

        env::set_var("XZAGENTZ_COMPONENTS_DIR", temp2.path());

        let source = resolve_component_dir(Some(temp1.path().to_path_buf()));

        env::remove_var("XZAGENTZ_COMPONENTS_DIR");

        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, temp1.path());
        }
    }

    #[test]
    fn test_resource_source_equality() {
        let fs1 = ResourceSource::Filesystem(PathBuf::from("/tmp"));
        let fs2 = ResourceSource::Filesystem(PathBuf::from("/tmp"));
        let fs3 = ResourceSource::Filesystem(PathBuf::from("/other"));
        let embed1 = ResourceSource::Embedded;
        let embed2 = ResourceSource::Embedded;

        assert_eq!(fs1, fs2);
        assert_ne!(fs1, fs3);
        assert_eq!(embed1, embed2);
        assert_ne!(fs1, embed1);
    }
}
