//! Resource resolution for configuration directories
//!
//! This module implements the resource resolution hierarchy for configuration directories:
//! 1. Environment variable overrides (XZAGENTZ_CONFIG_DIR, XDG_CONFIG_HOME)
//! 2. User configuration directory (~/.config/xzagentz)
//! 3. Current working directory fallback
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::config::resolution::default_config_dir;
//!
//! let dir = default_config_dir();
//! ```

use std::env;
use std::path::PathBuf;

/// Returns the default configuration directory path
///
/// This is the preferred location where project configuration files
/// should be stored: ~/.config/xzagentz
///
/// # Resolution Order
///
/// 1. XZAGENTZ_CONFIG_DIR environment variable (if set)
/// 2. XDG_CONFIG_HOME/xzagentz (if XDG_CONFIG_HOME is set)
/// 3. ~/.config/xzagentz (default)
/// 4. ./.config/xzagentz (fallback if HOME not available)
///
/// # Returns
///
/// Returns the default configuration directory path
///
/// # Examples
///
/// ```rust
/// use xzagentz::config::resolution::default_config_dir;
///
/// let dir = default_config_dir();
/// assert!(dir.ends_with("xzagentz") || dir.ends_with(".config/xzagentz"));
/// ```
pub fn default_config_dir() -> PathBuf {
    // 1. Check XZAGENTZ_CONFIG_DIR environment variable
    if let Ok(config_dir) = env::var("XZAGENTZ_CONFIG_DIR") {
        return PathBuf::from(config_dir);
    }

    // 2. Check XDG_CONFIG_HOME/xzagentz
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg_config).join("xzagentz");
    }

    // 3. Use ~/.config/xzagentz (default)
    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".config").join("xzagentz");
    }

    // 4. Fallback for systems without HOME
    PathBuf::from(".config").join("xzagentz")
}

/// Returns the default components directory path
///
/// This delegates to the components module's default_components_dir function
/// for consistency, but provides a unified API from the config module.
///
/// # Returns
///
/// Returns the default components directory path
///
/// # Examples
///
/// ```rust
/// use xzagentz::config::resolution::default_components_dir;
///
/// let dir = default_components_dir();
/// assert!(dir.ends_with(".config/xzagentz/components"));
/// ```
pub fn default_components_dir() -> PathBuf {
    crate::components::resolution::default_components_dir()
}

/// Returns the default templates directory path
///
/// This delegates to the templates module's default_templates_dir function
/// for consistency, but provides a unified API from the config module.
///
/// # Returns
///
/// Returns the default templates directory path
///
/// # Examples
///
/// ```rust
/// use xzagentz::config::resolution::default_templates_dir;
///
/// let dir = default_templates_dir();
/// assert!(dir.ends_with(".config/xzagentz/templates"));
/// ```
pub fn default_templates_dir() -> PathBuf {
    crate::templates::resolution::default_templates_dir()
}

/// Returns all default resource directories as a tuple
///
/// This is a convenience function to get all default directories at once.
///
/// # Returns
///
/// Returns (config_dir, components_dir, templates_dir)
///
/// # Examples
///
/// ```rust
/// use xzagentz::config::resolution::default_resource_dirs;
///
/// let (config, components, templates) = default_resource_dirs();
/// assert!(config.ends_with("xzagentz") || config.ends_with(".config/xzagentz"));
/// assert!(components.ends_with(".config/xzagentz/components"));
/// assert!(templates.ends_with(".config/xzagentz/templates"));
/// ```
pub fn default_resource_dirs() -> (PathBuf, PathBuf, PathBuf) {
    (
        default_config_dir(),
        default_components_dir(),
        default_templates_dir(),
    )
}

/// Resolves the configuration directory path
///
/// # Resolution Order
///
/// 1. Custom directory if provided
/// 2. XZAGENTZ_CONFIG_DIR environment variable
/// 3. XDG_CONFIG_HOME/xzagentz
/// 4. ~/.config/xzagentz
/// 5. ./.config/xzagentz (fallback)
///
/// # Arguments
///
/// * `custom_dir` - Optional custom directory override
///
/// # Returns
///
/// Returns the resolved configuration directory path
///
/// # Examples
///
/// ```rust
/// use xzagentz::config::resolution::resolve_config_dir;
/// use std::path::PathBuf;
///
/// // With custom directory
/// let dir = resolve_config_dir(Some(PathBuf::from("/custom/config")));
/// assert_eq!(dir, PathBuf::from("/custom/config"));
///
/// // Using resolution hierarchy
/// let dir = resolve_config_dir(None);
/// ```
pub fn resolve_config_dir(custom_dir: Option<PathBuf>) -> PathBuf {
    // 1. Custom directory takes precedence
    if let Some(dir) = custom_dir {
        return dir;
    }

    // 2-5. Use default resolution
    default_config_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config_dir_with_home() {
        let old_xzagentz = env::var_os("XZAGENTZ_CONFIG_DIR");
        let old_xdg = env::var_os("XDG_CONFIG_HOME");

        env::remove_var("XZAGENTZ_CONFIG_DIR");
        env::remove_var("XDG_CONFIG_HOME");

        if env::var_os("HOME").is_some() {
            let dir = default_config_dir();
            let dir_str = dir.to_string_lossy();
            assert!(
                dir_str.contains(".config/xzagentz") || dir_str.ends_with("xzagentz"),
                "Expected config dir to contain .config/xzagentz or end with xzagentz, got: {}",
                dir_str
            );
        }

        // Restore
        if let Some(xzagentz) = old_xzagentz {
            env::set_var("XZAGENTZ_CONFIG_DIR", xzagentz);
        }
        if let Some(xdg) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        }
    }

    #[test]
    fn test_default_config_dir_without_home() {
        let old_home = env::var_os("HOME");
        let old_xdg = env::var_os("XDG_CONFIG_HOME");
        let old_xzagentz = env::var_os("XZAGENTZ_CONFIG_DIR");

        env::remove_var("HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XZAGENTZ_CONFIG_DIR");

        let dir = default_config_dir();
        assert!(dir.to_string_lossy().contains(".config/xzagentz"));

        // Restore environment variables
        if let Some(home) = old_home {
            env::set_var("HOME", home);
        }
        if let Some(xdg) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        }
        if let Some(xzagentz) = old_xzagentz {
            env::set_var("XZAGENTZ_CONFIG_DIR", xzagentz);
        }
    }

    #[test]
    fn test_default_config_dir_with_xdg() {
        let old_xdg = env::var_os("XDG_CONFIG_HOME");
        let old_xzagentz = env::var_os("XZAGENTZ_CONFIG_DIR");

        env::remove_var("XZAGENTZ_CONFIG_DIR");
        env::set_var("XDG_CONFIG_HOME", "/custom/xdg/config");

        let dir = default_config_dir();
        assert_eq!(dir, PathBuf::from("/custom/xdg/config/xzagentz"));

        // Restore
        env::remove_var("XDG_CONFIG_HOME");
        if let Some(xdg) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        }
        if let Some(xzagentz) = old_xzagentz {
            env::set_var("XZAGENTZ_CONFIG_DIR", xzagentz);
        }
    }

    #[test]
    fn test_default_config_dir_with_env_var() {
        let old_var = env::var_os("XZAGENTZ_CONFIG_DIR");

        env::set_var("XZAGENTZ_CONFIG_DIR", "/custom/config/dir");

        let dir = default_config_dir();
        assert_eq!(dir, PathBuf::from("/custom/config/dir"));

        // Restore
        env::remove_var("XZAGENTZ_CONFIG_DIR");
        if let Some(val) = old_var {
            env::set_var("XZAGENTZ_CONFIG_DIR", val);
        }
    }

    #[test]
    fn test_default_components_dir() {
        let old_xzagentz_comp = env::var_os("XZAGENTZ_COMPONENTS_DIR");
        let old_xdg = env::var_os("XDG_DATA_HOME");

        env::remove_var("XZAGENTZ_COMPONENTS_DIR");
        env::remove_var("XDG_DATA_HOME");

        let dir = default_components_dir();
        assert!(dir
            .to_string_lossy()
            .contains(".config/xzagentz/components"));

        // Restore
        if let Some(comp) = old_xzagentz_comp {
            env::set_var("XZAGENTZ_COMPONENTS_DIR", comp);
        }
        if let Some(xdg) = old_xdg {
            env::set_var("XDG_DATA_HOME", xdg);
        }
    }

    #[test]
    fn test_default_templates_dir() {
        let old_xzagentz_temp = env::var_os("XZAGENTZ_TEMPLATES_DIR");
        let old_xdg = env::var_os("XDG_DATA_HOME");

        env::remove_var("XZAGENTZ_TEMPLATES_DIR");
        env::remove_var("XDG_DATA_HOME");

        let dir = default_templates_dir();
        assert!(dir.to_string_lossy().contains(".config/xzagentz/templates"));

        // Restore
        if let Some(temp) = old_xzagentz_temp {
            env::set_var("XZAGENTZ_TEMPLATES_DIR", temp);
        }
        if let Some(xdg) = old_xdg {
            env::set_var("XDG_DATA_HOME", xdg);
        }
    }

    #[test]
    fn test_default_resource_dirs() {
        let old_xzagentz = env::var_os("XZAGENTZ_CONFIG_DIR");
        let old_xzagentz_comp = env::var_os("XZAGENTZ_COMPONENTS_DIR");
        let old_xzagentz_temp = env::var_os("XZAGENTZ_TEMPLATES_DIR");
        let old_xdg_config = env::var_os("XDG_CONFIG_HOME");
        let old_xdg_data = env::var_os("XDG_DATA_HOME");

        env::remove_var("XZAGENTZ_CONFIG_DIR");
        env::remove_var("XZAGENTZ_COMPONENTS_DIR");
        env::remove_var("XZAGENTZ_TEMPLATES_DIR");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");

        let (config, components, templates) = default_resource_dirs();

        let config_str = config.to_string_lossy();
        assert!(
            config_str.contains(".config/xzagentz") || config_str.ends_with("xzagentz"),
            "Config dir should contain .config/xzagentz or end with xzagentz"
        );
        assert!(components
            .to_string_lossy()
            .contains(".config/xzagentz/components"));
        assert!(templates
            .to_string_lossy()
            .contains(".config/xzagentz/templates"));

        // Restore
        if let Some(cfg) = old_xzagentz {
            env::set_var("XZAGENTZ_CONFIG_DIR", cfg);
        }
        if let Some(comp) = old_xzagentz_comp {
            env::set_var("XZAGENTZ_COMPONENTS_DIR", comp);
        }
        if let Some(temp) = old_xzagentz_temp {
            env::set_var("XZAGENTZ_TEMPLATES_DIR", temp);
        }
        if let Some(xdg_cfg) = old_xdg_config {
            env::set_var("XDG_CONFIG_HOME", xdg_cfg);
        }
        if let Some(xdg_data) = old_xdg_data {
            env::set_var("XDG_DATA_HOME", xdg_data);
        }
    }

    #[test]
    fn test_resolve_config_dir_with_custom() {
        let custom = PathBuf::from("/my/custom/path");
        let resolved = resolve_config_dir(Some(custom.clone()));
        assert_eq!(resolved, custom);
    }

    #[test]
    fn test_resolve_config_dir_without_custom() {
        let resolved = resolve_config_dir(None);
        let expected = default_config_dir();
        assert_eq!(resolved, expected);
    }

    #[test]
    fn test_env_var_precedence() {
        let old_xdg = env::var_os("XDG_CONFIG_HOME");
        let old_xzagentz = env::var_os("XZAGENTZ_CONFIG_DIR");

        // Set both environment variables
        env::set_var("XDG_CONFIG_HOME", "/xdg/config");
        env::set_var("XZAGENTZ_CONFIG_DIR", "/xzagentz/config");

        // XZAGENTZ_CONFIG_DIR should take precedence
        let dir = default_config_dir();
        assert_eq!(dir, PathBuf::from("/xzagentz/config"));

        // Restore
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XZAGENTZ_CONFIG_DIR");
        if let Some(xdg) = old_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        }
        if let Some(xzagentz) = old_xzagentz {
            env::set_var("XZAGENTZ_CONFIG_DIR", xzagentz);
        }
    }

    #[test]
    fn test_custom_dir_precedence_in_resolve() {
        let old_xzagentz = env::var_os("XZAGENTZ_CONFIG_DIR");

        env::set_var("XZAGENTZ_CONFIG_DIR", "/env/config");

        let custom = PathBuf::from("/custom/config");
        let resolved = resolve_config_dir(Some(custom.clone()));

        // Custom dir should take precedence over environment variable
        assert_eq!(resolved, custom);

        // Restore
        env::remove_var("XZAGENTZ_CONFIG_DIR");
        if let Some(xzagentz) = old_xzagentz {
            env::set_var("XZAGENTZ_CONFIG_DIR", xzagentz);
        }
    }
}
