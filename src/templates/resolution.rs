//! Resource resolution for template directories
//!
//! This module implements the resource resolution hierarchy for template directories:
//! 1. Environment variable overrides (XZAGENTZ_TEMPLATES_DIR, XDG_DATA_HOME)
//! 2. User configuration directory (~/.config/xzagentz/templates)
//! 3. Embedded resources fallback
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::templates::resolution::resolve_template_dir;
//!
//! let source = resolve_template_dir(None);
//! ```

use crate::components::resolution::ResourceSource;
use std::env;
use std::path::PathBuf;

/// Resolves the template directory using the resource resolution hierarchy
///
/// # Resolution Order
///
/// 1. Custom directory if provided
/// 2. XZAGENTZ_TEMPLATES_DIR environment variable
/// 3. XDG_DATA_HOME/xzagentz/templates
/// 4. ~/.config/xzagentz/templates
/// 5. Embedded resources (fallback)
///
/// # Arguments
///
/// * `custom_dir` - Optional custom directory override
///
/// # Returns
///
/// Returns a `ResourceSource` indicating where templates should be loaded from
///
/// # Examples
///
/// ```rust
/// use xzagentz::templates::resolution::resolve_template_dir;
/// use xzagentz::components::resolution::ResourceSource;
/// use std::path::PathBuf;
///
/// // With custom directory (may return embedded if dir doesn't exist)
/// let source = resolve_template_dir(Some(PathBuf::from("/custom/templates")));
///
/// // Using resolution hierarchy (falls back to embedded if no dirs found)
/// let source = resolve_template_dir(None);
/// // Embedded resources are always available as fallback
/// ```
pub fn resolve_template_dir(custom_dir: Option<PathBuf>) -> ResourceSource {
    // 1. Custom directory takes precedence
    if let Some(dir) = custom_dir {
        if dir.exists() {
            return ResourceSource::Filesystem(dir);
        }
    }

    // 2. Check XZAGENTZ_TEMPLATES_DIR environment variable
    if let Ok(dir) = env::var("XZAGENTZ_TEMPLATES_DIR") {
        let path = PathBuf::from(dir);
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 3. Check XDG_DATA_HOME/xzagentz/templates
    if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data).join("xzagentz").join("templates");
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 4. Check ~/.config/xzagentz/templates
    if let Some(home) = env::var_os("HOME") {
        let path = PathBuf::from(home)
            .join(".config")
            .join("xzagentz")
            .join("templates");
        if path.exists() {
            return ResourceSource::Filesystem(path);
        }
    }

    // 5. Fall back to embedded resources
    ResourceSource::Embedded
}

/// Returns the default templates directory path
///
/// This is the preferred location where users should extract templates
/// for customization: ~/.config/xzagentz/templates
///
/// # Returns
///
/// Returns the default templates directory path
///
/// # Examples
///
/// ```rust
/// use xzagentz::templates::resolution::default_templates_dir;
///
/// let dir = default_templates_dir();
/// assert!(dir.ends_with(".config/xzagentz/templates"));
/// ```
pub fn default_templates_dir() -> PathBuf {
    if let Some(home) = env::var_os("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("xzagentz")
            .join("templates")
    } else {
        // Fallback for systems without HOME
        PathBuf::from(".config").join("xzagentz").join("templates")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_with_custom_dir_existing() {
        let temp = TempDir::new().unwrap();
        let source = resolve_template_dir(Some(temp.path().to_path_buf()));
        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, temp.path());
        }
    }

    #[test]
    fn test_resolve_with_custom_dir_nonexistent() {
        let source = resolve_template_dir(Some(PathBuf::from("/nonexistent/path")));
        // Should fall back to embedded since custom dir doesn't exist
        assert!(source.is_embedded());
    }

    #[test]
    fn test_resolve_with_env_var() {
        let temp = TempDir::new().unwrap();
        env::set_var("XZAGENTZ_TEMPLATES_DIR", temp.path());

        let source = resolve_template_dir(None);

        env::remove_var("XZAGENTZ_TEMPLATES_DIR");

        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, temp.path());
        }
    }

    #[test]
    fn test_resolve_fallback_to_embedded() {
        // Ensure no env vars are set
        env::remove_var("XZAGENTZ_TEMPLATES_DIR");
        env::remove_var("XDG_DATA_HOME");

        // Use a nonexistent custom dir
        let source = resolve_template_dir(Some(PathBuf::from("/definitely/does/not/exist")));
        assert!(source.is_embedded());
    }

    #[test]
    fn test_default_templates_dir_with_home() {
        if env::var_os("HOME").is_some() {
            let dir = default_templates_dir();
            assert!(dir.to_string_lossy().contains(".config/xzagentz/templates"));
        }
    }

    #[test]
    fn test_default_templates_dir_without_home() {
        let old_home = env::var_os("HOME");
        env::remove_var("HOME");

        let dir = default_templates_dir();
        assert!(dir.to_string_lossy().contains(".config/xzagentz/templates"));

        // Restore HOME
        if let Some(home) = old_home {
            env::set_var("HOME", home);
        }
    }

    #[test]
    fn test_resolve_with_xdg_data_home() {
        let temp = TempDir::new().unwrap();
        let templates_dir = temp.path().join("xzagentz").join("templates");
        std::fs::create_dir_all(&templates_dir).unwrap();

        env::set_var("XDG_DATA_HOME", temp.path());
        let source = resolve_template_dir(None);
        env::remove_var("XDG_DATA_HOME");

        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, templates_dir);
        }
    }

    #[test]
    fn test_custom_dir_takes_precedence() {
        let temp1 = TempDir::new().unwrap();
        let temp2 = TempDir::new().unwrap();

        env::set_var("XZAGENTZ_TEMPLATES_DIR", temp2.path());

        let source = resolve_template_dir(Some(temp1.path().to_path_buf()));

        env::remove_var("XZAGENTZ_TEMPLATES_DIR");

        assert!(source.is_filesystem());
        if let ResourceSource::Filesystem(path) = source {
            assert_eq!(path, temp1.path());
        }
    }
}
