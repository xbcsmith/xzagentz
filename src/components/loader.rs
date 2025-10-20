//! Component loader with caching and filesystem support
//!
//! This module provides the `ComponentLoader` which loads components from
//! the filesystem with support for caching. Future versions will support
//! embedded resources with override priority.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::ComponentLoader;
//! use xzagentz::core::ComponentType;
//!
//! let loader = ComponentLoader::new(None);
//! // Load components from default location
//! ```

use crate::components::Component;
use crate::core::ComponentType;
use crate::error::{Error, Result};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[cfg(test)]
use std::path::Path;

/// Component loader with caching support
///
/// Loads components from the filesystem with an in-memory cache
/// to avoid repeated disk reads.
#[derive(Debug)]
pub struct ComponentLoader {
    /// Optional custom components directory
    custom_dir: Option<PathBuf>,

    /// Cache of loaded components (name -> Component)
    cache: RefCell<HashMap<String, Component>>,
}

impl ComponentLoader {
    /// Creates a new ComponentLoader
    ///
    /// # Arguments
    ///
    /// * `custom_dir` - Optional path to custom components directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::ComponentLoader;
    /// use std::path::PathBuf;
    ///
    /// // Use default location
    /// let loader = ComponentLoader::new(None);
    ///
    /// // Use custom directory
    /// let custom_loader = ComponentLoader::new(Some(PathBuf::from("./components")));
    /// ```
    pub fn new(custom_dir: Option<PathBuf>) -> Self {
        Self {
            custom_dir,
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Loads a component by name and type
    ///
    /// # Arguments
    ///
    /// * `name` - Component name (without .md extension)
    /// * `component_type` - Component category
    ///
    /// # Returns
    ///
    /// Returns the loaded `Component` or an error if not found
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentNotFound` if the component does not exist
    /// Returns `Error::FileIo` if there are filesystem errors
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::components::ComponentLoader;
    /// use xzagentz::core::ComponentType;
    ///
    /// let loader = ComponentLoader::new(None);
    /// let component = loader.load("rust_standards", ComponentType::Core)?;
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn load(&self, name: &str, component_type: ComponentType) -> Result<Component> {
        // Check cache first
        let cache_key = format!("{}:{}", component_type.as_str(), name);
        if let Some(cached) = self.cache.borrow().get(&cache_key) {
            return Ok(cached.clone());
        }

        // Build file path
        let path = self.component_path(name, component_type);

        // Check if file exists
        if !path.exists() {
            return Err(Error::component_not_found(format!(
                "{} (type: {})",
                name,
                component_type.as_str()
            )));
        }

        // Read file content
        let content = fs::read_to_string(&path).map_err(|e| Error::file_io(path.clone(), e))?;

        // Create component
        let component = Component::new(name, component_type, content);

        // Cache the component
        self.cache.borrow_mut().insert(cache_key, component.clone());

        Ok(component)
    }

    /// Loads all components in a specific category
    ///
    /// # Arguments
    ///
    /// * `component_type` - Component category to load
    ///
    /// # Returns
    ///
    /// Returns a vector of all components in the category
    ///
    /// # Errors
    ///
    /// Returns `Error::DirectoryNotFound` if the category directory does not exist
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::components::ComponentLoader;
    /// use xzagentz::core::ComponentType;
    ///
    /// let loader = ComponentLoader::new(None);
    /// let core_components = loader.load_all(ComponentType::Core)?;
    /// println!("Found {} core components", core_components.len());
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn load_all(&self, component_type: ComponentType) -> Result<Vec<Component>> {
        let dir = self.component_dir(component_type);

        if !dir.exists() {
            return Err(Error::DirectoryNotFound { path: dir });
        }

        let mut components = Vec::new();

        for entry in WalkDir::new(&dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip directories and non-markdown files
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            // Extract component name (filename without extension)
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                // Try to load the component
                match self.load(name, component_type) {
                    Ok(component) => components.push(component),
                    Err(_) => continue, // Skip components that fail to load
                }
            }
        }

        Ok(components)
    }

    /// Lists all available component names in a category
    ///
    /// # Arguments
    ///
    /// * `component_type` - Component category to list
    ///
    /// # Returns
    ///
    /// Returns a vector of component names (without .md extension)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::components::ComponentLoader;
    /// use xzagentz::core::ComponentType;
    ///
    /// let loader = ComponentLoader::new(None);
    /// let names = loader.list(ComponentType::Core)?;
    /// for name in names {
    ///     println!("Component: {}", name);
    /// }
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn list(&self, component_type: ComponentType) -> Result<Vec<String>> {
        let dir = self.component_dir(component_type);

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut names = Vec::new();

        for entry in WalkDir::new(&dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                names.push(name.to_string());
            }
        }

        names.sort();
        Ok(names)
    }

    /// Clears the component cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::ComponentLoader;
    ///
    /// let loader = ComponentLoader::new(None);
    /// loader.clear_cache();
    /// ```
    pub fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    /// Returns the number of cached components
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::ComponentLoader;
    ///
    /// let loader = ComponentLoader::new(None);
    /// assert_eq!(loader.cache_size(), 0);
    /// ```
    pub fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }

    /// Builds the path to a component file
    fn component_path(&self, name: &str, component_type: ComponentType) -> PathBuf {
        let dir = self.component_dir(component_type);
        dir.join(format!("{}.md", name))
    }

    /// Returns the directory path for a component type
    fn component_dir(&self, component_type: ComponentType) -> PathBuf {
        if let Some(ref custom) = self.custom_dir {
            custom.join(component_type.as_str())
        } else {
            PathBuf::from("components").join(component_type.as_str())
        }
    }
}

impl Default for ComponentLoader {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_components(base_dir: &Path) -> Result<()> {
        // Create directory structure
        let core_dir = base_dir.join("core");
        fs::create_dir_all(&core_dir)?;

        // Create test components
        fs::write(
            core_dir.join("rust_standards.md"),
            "# Rust Coding Standards\n\nContent here",
        )?;
        fs::write(
            core_dir.join("git_conventions.md"),
            "# Git Conventions\n\nCommit message format",
        )?;

        let languages_dir = base_dir.join("languages");
        fs::create_dir_all(&languages_dir)?;
        fs::write(
            languages_dir.join("rust.md"),
            "# Rust Language Guide\n\nRust specific content",
        )?;

        Ok(())
    }

    #[test]
    fn test_load_component_success() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));
        let component = loader.load("rust_standards", ComponentType::Core).unwrap();

        assert_eq!(component.name, "rust_standards");
        assert_eq!(component.component_type, ComponentType::Core);
        assert!(component.content.contains("Rust Coding Standards"));
    }

    #[test]
    fn test_load_component_not_found() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));
        let result = loader.load("nonexistent", ComponentType::Core);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error::ComponentNotFound { .. }
        ));
    }

    #[test]
    fn test_load_all_components_in_category() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));
        let components = loader.load_all(ComponentType::Core).unwrap();

        assert_eq!(components.len(), 2);

        let names: Vec<_> = components.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"rust_standards"));
        assert!(names.contains(&"git_conventions"));
    }

    #[test]
    fn test_component_caching() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));

        assert_eq!(loader.cache_size(), 0);

        // First load - reads from disk
        let component1 = loader.load("rust_standards", ComponentType::Core).unwrap();
        assert_eq!(loader.cache_size(), 1);

        // Second load - reads from cache
        let component2 = loader.load("rust_standards", ComponentType::Core).unwrap();
        assert_eq!(loader.cache_size(), 1);

        assert_eq!(component1, component2);
    }

    #[test]
    fn test_clear_cache() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));

        loader.load("rust_standards", ComponentType::Core).unwrap();
        assert_eq!(loader.cache_size(), 1);

        loader.clear_cache();
        assert_eq!(loader.cache_size(), 0);
    }

    #[test]
    fn test_list_components() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));
        let names = loader.list(ComponentType::Core).unwrap();

        assert_eq!(names.len(), 2);
        assert!(names.contains(&"git_conventions".to_string()));
        assert!(names.contains(&"rust_standards".to_string()));
    }

    #[test]
    fn test_list_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));
        let names = loader.list(ComponentType::Core).unwrap();

        assert_eq!(names.len(), 0);
    }

    #[test]
    fn test_default_loader() {
        let loader = ComponentLoader::default();
        assert_eq!(loader.cache_size(), 0);
    }

    #[test]
    fn test_load_multiple_types() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_components(temp_dir.path()).unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));

        let core_component = loader.load("rust_standards", ComponentType::Core).unwrap();
        let lang_component = loader.load("rust", ComponentType::Languages).unwrap();

        assert_eq!(core_component.component_type, ComponentType::Core);
        assert_eq!(lang_component.component_type, ComponentType::Languages);
        assert_eq!(loader.cache_size(), 2);
    }
}
