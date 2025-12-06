// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Component loader with caching and filesystem support
//!
//! This module provides the `ComponentLoader` which loads components from
//! the filesystem with support for caching and fallback to embedded resources.
//!
//! # Resource Resolution Hierarchy
//!
//! The loader follows this priority order:
//! 1. Environment variable overrides (XZAGENTZ_COMPONENTS_DIR, XDG_DATA_HOME)
//! 2. User configuration directory (~/.config/xzagentz/components)
//! 3. Embedded resources (compiled into binary)
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::ComponentLoader;
//! use xzagentz::core::ComponentType;
//!
//! let loader = ComponentLoader::new(None);
//! // Load components from default location or embedded resources
//! ```

use crate::components::metadata::ComponentMetadata;
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
        self.load_with_tier(name, component_type, false)
    }

    /// Loads a component by name and type with tier support
    ///
    /// # Arguments
    ///
    /// * `name` - Component name (without .md extension)
    /// * `component_type` - Component category
    /// * `comprehensive` - If true, prefer comprehensive tier, otherwise essential
    ///
    /// # Returns
    ///
    /// Returns the loaded `Component` or an error if not found
    ///
    /// # Resolution Order
    ///
    /// If `comprehensive` is true:
    /// 1. Try `{name}_comprehensive.md`
    /// 2. Fall back to `{name}.md`
    ///
    /// If `comprehensive` is false (default):
    /// 1. Try `{name}_essential.md`
    /// 2. Fall back to `{name}.md`
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentNotFound` if no variant exists
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::components::ComponentLoader;
    /// use xzagentz::core::ComponentType;
    ///
    /// let loader = ComponentLoader::new(None);
    /// // Load essential version
    /// let essential = loader.load_with_tier("rust", ComponentType::Languages, false)?;
    /// // Load comprehensive version
    /// let comprehensive = loader.load_with_tier("rust", ComponentType::Languages, true)?;
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn load_with_tier(
        &self,
        name: &str,
        component_type: ComponentType,
        comprehensive: bool,
    ) -> Result<Component> {
        // Determine tier suffix based on comprehensive flag
        let tier_suffix = if comprehensive {
            "_comprehensive"
        } else {
            "_essential"
        };

        // Try tiered version first
        let tiered_name = format!("{}{}", name, tier_suffix);

        // Check cache for tiered version
        let tiered_cache_key = format!("{}:{}", component_type.as_str(), tiered_name);
        if let Some(cached) = self.cache.borrow().get(&tiered_cache_key) {
            return Ok(cached.clone());
        }

        // Try to load tiered version
        let tiered_path = self.component_path(&tiered_name, component_type);
        if tiered_path.exists() {
            return self.load_from_path(&tiered_name, component_type, &tiered_path);
        }

        // Fall back to base name
        let base_cache_key = format!("{}:{}", component_type.as_str(), name);
        if let Some(cached) = self.cache.borrow().get(&base_cache_key) {
            return Ok(cached.clone());
        }

        let base_path = self.component_path(name, component_type);
        if base_path.exists() {
            return self.load_from_path(name, component_type, &base_path);
        }

        // Neither tiered nor base version found
        Err(Error::component_not_found(format!(
            "{} (tried: {}, {}) (type: {})",
            name,
            tiered_name,
            name,
            component_type.as_str()
        )))
    }

    /// Internal method to load a component from a specific path
    fn load_from_path(
        &self,
        name: &str,
        component_type: ComponentType,
        path: &PathBuf,
    ) -> Result<Component> {
        let cache_key = format!("{}:{}", component_type.as_str(), name);

        // Read file content
        let raw_content = fs::read_to_string(path).map_err(|e| Error::file_io(path.clone(), e))?;

        // Parse YAML frontmatter if present and capture metadata
        let (content, metadata_map) = if ComponentMetadata::has_frontmatter(&raw_content) {
            match ComponentMetadata::parse_frontmatter(&raw_content) {
                Ok((meta, body)) => {
                    let mut map: HashMap<String, String> = HashMap::new();
                    map.insert("name".to_string(), meta.component.name.clone());
                    map.insert("category".to_string(), meta.component.category.clone());
                    map.insert("version".to_string(), meta.component.version.clone());
                    if let Some(ref desc) = meta.component.description {
                        map.insert("description".to_string(), desc.clone());
                    }
                    if let Some(ref tier) = meta.component.tier {
                        map.insert("tier".to_string(), tier.clone());
                    }
                    if !meta.component.languages.is_empty() {
                        map.insert("languages".to_string(), meta.component.languages.join(","));
                    }

                    (body, map)
                }
                Err(_) => (Self::strip_yaml_frontmatter(&raw_content), HashMap::new()),
            }
        } else {
            (Self::strip_yaml_frontmatter(&raw_content), HashMap::new())
        };

        // Create component with metadata map
        let component = Component::with_metadata(name, component_type, content, metadata_map);

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

    /// Strips YAML frontmatter from component content
    ///
    /// Component files may have YAML frontmatter delimited by `---` markers.
    /// This function removes that frontmatter and returns only the markdown content.
    ///
    /// # Arguments
    ///
    /// * `content` - Raw file content that may contain YAML frontmatter
    ///
    /// # Returns
    ///
    /// Returns the content with YAML frontmatter removed, or the original content
    /// if no frontmatter is present.
    ///
    /// # Implementation Note
    ///
    /// This is an internal helper function used during component loading.
    /// The frontmatter stripping happens automatically when loading components.
    fn strip_yaml_frontmatter(content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();

        // Check if content starts with frontmatter delimiter
        if lines.is_empty() || lines[0].trim() != "---" {
            return content.to_string();
        }

        // Find the closing delimiter
        let mut end_index = None;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                end_index = Some(i);
                break;
            }
        }

        // If we found the closing delimiter, skip all lines up to and including it
        if let Some(end) = end_index {
            // Skip frontmatter and any immediately following blank lines
            let content_start = lines
                .iter()
                .enumerate()
                .skip(end + 1)
                .find(|(_, line)| !line.trim().is_empty())
                .map(|(i, _)| i)
                .unwrap_or(end + 1);

            return lines[content_start..].join("\n");
        }

        // No closing delimiter found, return original content
        content.to_string()
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

    #[test]
    fn test_load_with_tier_prefers_essential_then_fallback() {
        let temp_dir = TempDir::new().unwrap();
        // create languages directory and both essential/comprehensive files
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir_all(&languages_dir).unwrap();

        fs::write(
            languages_dir.join("rust_essential.md"),
            "# Rust Essential\n\nEssential content",
        )
        .unwrap();

        fs::write(
            languages_dir.join("rust_comprehensive.md"),
            "# Rust Comprehensive\n\nComprehensive content",
        )
        .unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));

        // default (comprehensive=false) should pick essential
        let comp = loader
            .load_with_tier("rust", ComponentType::Languages, false)
            .unwrap();
        assert!(comp.content.contains("Essential content"));

        // comprehensive=true should pick comprehensive variant
        let comp2 = loader
            .load_with_tier("rust", ComponentType::Languages, true)
            .unwrap();
        assert!(comp2.content.contains("Comprehensive content"));
    }

    #[test]
    fn test_load_with_tier_fallback_to_base() {
        let temp_dir = TempDir::new().unwrap();
        let languages_dir = temp_dir.path().join("languages");
        fs::create_dir_all(&languages_dir).unwrap();

        // only base exists
        fs::write(
            languages_dir.join("python.md"),
            "# Python Base\n\nBase content",
        )
        .unwrap();

        let loader = ComponentLoader::new(Some(temp_dir.path().to_path_buf()));

        // both comprehensive and essential should fall back to base
        let comp = loader
            .load_with_tier("python", ComponentType::Languages, false)
            .unwrap();
        assert!(comp.content.contains("Base content"));

        let comp2 = loader
            .load_with_tier("python", ComponentType::Languages, true)
            .unwrap();
        assert!(comp2.content.contains("Base content"));
    }

    #[test]
    fn test_strip_yaml_frontmatter_with_frontmatter() {
        let content = "---\nkey: value\nname: test\n---\n\n# Heading\n\nContent here";
        let stripped = ComponentLoader::strip_yaml_frontmatter(content);
        assert_eq!(stripped, "# Heading\n\nContent here");
    }

    #[test]
    fn test_strip_yaml_frontmatter_without_frontmatter() {
        let content = "# Heading\n\nContent here";
        let stripped = ComponentLoader::strip_yaml_frontmatter(content);
        assert_eq!(stripped, "# Heading\n\nContent here");
    }

    #[test]
    fn test_strip_yaml_frontmatter_unclosed() {
        let content = "---\nkey: value\n\n# Heading\n\nContent";
        let stripped = ComponentLoader::strip_yaml_frontmatter(content);
        // Unclosed frontmatter should return original content
        assert_eq!(stripped, content);
    }

    #[test]
    fn test_strip_yaml_frontmatter_empty() {
        let content = "";
        let stripped = ComponentLoader::strip_yaml_frontmatter(content);
        assert_eq!(stripped, "");
    }

    #[test]
    fn test_strip_yaml_frontmatter_with_component_metadata() {
        let content = "---\ncomponent:\n  name: critical_rules\n  category: core\n  version: 2.0.0\n---\n\n# Critical Rules\n\nContent";
        let stripped = ComponentLoader::strip_yaml_frontmatter(content);
        assert_eq!(stripped, "# Critical Rules\n\nContent");
        assert!(!stripped.contains("component:"));
        assert!(!stripped.contains("name: critical_rules"));
    }
}
