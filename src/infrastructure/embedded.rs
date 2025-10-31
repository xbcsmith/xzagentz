//! Embedded resources module for xzagentz
//!
//! This module provides access to components and templates embedded in the binary
//! at compile time using the `include_dir` crate. It serves as a fallback when
//! filesystem resources are not available.
//!
//! # Architecture
//!
//! Resources are embedded at compile time and can be:
//! - Listed (discover available components/templates)
//! - Read (access content without filesystem)
//! - Extracted (write to filesystem for customization)
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::infrastructure::embedded::EmbeddedResources;
//!
//! let resources = EmbeddedResources::new();
//!
//! // List available components
//! let components = resources.list_components();
//! assert!(!components.is_empty());
//!
//! // Get a specific component
//! if let Some(content) = resources.get_component("core/base.md") {
//!     println!("Component content: {}", content);
//! }
//! ```

use crate::error::{Error, Result};
use include_dir::{include_dir, Dir};
use std::path::Path;

/// Embedded components directory (compile-time)
static COMPONENTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/components");

/// Embedded templates directory (compile-time)
static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Provides access to embedded components and templates
///
/// This struct wraps the embedded directory structures and provides
/// convenient methods for accessing, listing, and extracting resources.
///
/// # Examples
///
/// ```rust
/// use xzagentz::infrastructure::embedded::EmbeddedResources;
///
/// let resources = EmbeddedResources::new();
///
/// // Check if a component exists
/// let has_base = resources.has_component("core/base.md");
/// ```
#[derive(Debug, Clone)]
pub struct EmbeddedResources {
    // This is a zero-sized type wrapper for now
    _private: (),
}

impl EmbeddedResources {
    /// Creates a new EmbeddedResources instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// ```
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Gets the content of an embedded component file
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path to the component file (e.g., "core/base.md")
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` with the file content if found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// if let Some(content) = resources.get_component("core/base.md") {
    ///     assert!(!content.is_empty());
    /// }
    /// ```
    pub fn get_component(&self, path: &str) -> Option<String> {
        COMPONENTS_DIR
            .get_file(path)
            .and_then(|f| f.contents_utf8())
            .map(|s| s.to_string())
    }

    /// Gets the content of an embedded template file
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path to the template file (e.g., "plans/rust_binary.md")
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` with the file content if found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// if let Some(content) = resources.get_template("prompts/task_prompt.md") {
    ///     assert!(!content.is_empty());
    /// }
    /// ```
    pub fn get_template(&self, path: &str) -> Option<String> {
        TEMPLATES_DIR
            .get_file(path)
            .and_then(|f| f.contents_utf8())
            .map(|s| s.to_string())
    }

    /// Checks if a component exists in the embedded resources
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path to the component file
    ///
    /// # Returns
    ///
    /// Returns `true` if the component exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// let exists = resources.has_component("core/base.md");
    /// ```
    pub fn has_component(&self, path: &str) -> bool {
        COMPONENTS_DIR.get_file(path).is_some()
    }

    /// Checks if a template exists in the embedded resources
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path to the template file
    ///
    /// # Returns
    ///
    /// Returns `true` if the template exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// let exists = resources.has_template("prompts/task_prompt.md");
    /// ```
    pub fn has_template(&self, path: &str) -> bool {
        TEMPLATES_DIR.get_file(path).is_some()
    }

    /// Lists all embedded component file paths
    ///
    /// # Returns
    ///
    /// Returns a vector of relative file paths for all embedded components.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// let components = resources.list_components();
    /// assert!(!components.is_empty());
    ///
    /// // Check that paths are relative
    /// for component in &components {
    ///     assert!(!component.starts_with('/'));
    /// }
    /// ```
    pub fn list_components(&self) -> Vec<String> {
        let mut paths = Vec::new();
        collect_file_paths(&COMPONENTS_DIR, "", &mut paths);
        paths
    }

    /// Lists all embedded template file paths
    ///
    /// # Returns
    ///
    /// Returns a vector of relative file paths for all embedded templates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    ///
    /// let resources = EmbeddedResources::new();
    /// let templates = resources.list_templates();
    /// assert!(!templates.is_empty());
    ///
    /// // Check that paths are relative
    /// for template in &templates {
    ///     assert!(!template.starts_with('/'));
    /// }
    /// ```
    pub fn list_templates(&self) -> Vec<String> {
        let mut paths = Vec::new();
        collect_file_paths(&TEMPLATES_DIR, "", &mut paths);
        paths
    }

    /// Extracts all embedded components to the specified directory
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Directory where components will be extracted
    ///
    /// # Returns
    ///
    /// Returns `Ok(usize)` with the number of files extracted, or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The target directory cannot be created
    /// - Any file cannot be written
    /// - File permissions prevent writing
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    /// use std::path::PathBuf;
    /// use tempfile::tempdir;
    ///
    /// let resources = EmbeddedResources::new();
    /// let temp_dir = tempdir().unwrap();
    /// let target = temp_dir.path().join("components");
    ///
    /// let count = resources.extract_components_to(&target).unwrap();
    /// assert!(count > 0);
    /// ```
    pub fn extract_components_to(&self, target_dir: &Path) -> Result<usize> {
        extract_dir(&COMPONENTS_DIR, target_dir, "component")
    }

    /// Extracts all embedded templates to the specified directory
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Directory where templates will be extracted
    ///
    /// # Returns
    ///
    /// Returns `Ok(usize)` with the number of files extracted, or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The target directory cannot be created
    /// - Any file cannot be written
    /// - File permissions prevent writing
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    /// use std::path::PathBuf;
    /// use tempfile::tempdir;
    ///
    /// let resources = EmbeddedResources::new();
    /// let temp_dir = tempdir().unwrap();
    /// let target = temp_dir.path().join("templates");
    ///
    /// let count = resources.extract_templates_to(&target).unwrap();
    /// assert!(count > 0);
    /// ```
    pub fn extract_templates_to(&self, target_dir: &Path) -> Result<usize> {
        extract_dir(&TEMPLATES_DIR, target_dir, "template")
    }

    /// Extracts both components and templates to their respective subdirectories
    ///
    /// This is a convenience method that extracts components to `target_dir/components`
    /// and templates to `target_dir/templates`.
    ///
    /// # Arguments
    ///
    /// * `target_dir` - Base directory (e.g., `~/.config/xzagentz`)
    ///
    /// # Returns
    ///
    /// Returns `Ok((components_count, templates_count))` with the number of files
    /// extracted for each type, or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if extraction of either type fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::embedded::EmbeddedResources;
    /// use tempfile::tempdir;
    ///
    /// let resources = EmbeddedResources::new();
    /// let temp_dir = tempdir().unwrap();
    ///
    /// let (comp_count, temp_count) = resources.extract_all_to(temp_dir.path()).unwrap();
    /// assert!(comp_count > 0);
    /// assert!(temp_count > 0);
    /// ```
    pub fn extract_all_to(&self, target_dir: &Path) -> Result<(usize, usize)> {
        let components_dir = target_dir.join("components");
        let templates_dir = target_dir.join("templates");

        let components_count = self.extract_components_to(&components_dir)?;
        let templates_count = self.extract_templates_to(&templates_dir)?;

        Ok((components_count, templates_count))
    }
}

impl Default for EmbeddedResources {
    fn default() -> Self {
        Self::new()
    }
}

/// Recursively collects file paths from an embedded directory
///
/// # Arguments
///
/// * `dir` - The embedded directory to traverse
/// * `prefix` - Current path prefix for building relative paths
/// * `paths` - Vector to accumulate file paths
fn collect_file_paths(dir: &Dir, prefix: &str, paths: &mut Vec<String>) {
    for entry in dir.entries() {
        // Get just the file/directory name, not the full path
        let name = entry
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let entry_path = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", prefix, name)
        };

        match entry {
            include_dir::DirEntry::File(_) => {
                paths.push(entry_path);
            }
            include_dir::DirEntry::Dir(subdir) => {
                collect_file_paths(subdir, &entry_path, paths);
            }
        }
    }
}

/// Extracts an embedded directory to the filesystem
///
/// # Arguments
///
/// * `dir` - The embedded directory to extract
/// * `target_dir` - Target filesystem directory
/// * `resource_type` - Type name for error messages ("component" or "template")
///
/// # Returns
///
/// Returns the number of files extracted, or an error.
///
/// # Errors
///
/// Returns an error if directory creation or file writing fails.
fn extract_dir(dir: &Dir, target_dir: &Path, resource_type: &str) -> Result<usize> {
    // Create the target directory
    std::fs::create_dir_all(target_dir).map_err(|e| Error::FileIo {
        path: target_dir.to_path_buf(),
        source: e,
    })?;

    let mut count = 0;

    // Extract all entries recursively
    extract_entries(dir, target_dir, resource_type, &mut count)?;

    Ok(count)
}

/// Recursively extracts directory entries
///
/// # Arguments
///
/// * `dir` - The embedded directory to extract
/// * `target_base` - Base target directory
/// * `resource_type` - Type name for error messages
/// * `count` - Mutable counter for extracted files
///
/// # Errors
///
/// Returns an error if directory creation or file writing fails.
fn extract_entries(
    dir: &Dir,
    target_base: &Path,
    resource_type: &str,
    count: &mut usize,
) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let file_path = target_base.join(file.path());

                // Ensure parent directory exists
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| Error::FileIo {
                        path: parent.to_path_buf(),
                        source: e,
                    })?;
                }

                // Write file content
                let content = file.contents();
                std::fs::write(&file_path, content).map_err(|e| {
                    Error::file_create_error(
                        file_path.clone(),
                        format!("Failed to write {} file: {}", resource_type, e),
                    )
                })?;

                *count += 1;
            }
            include_dir::DirEntry::Dir(subdir) => {
                let subdir_path = target_base.join(subdir.path());

                // Create subdirectory
                std::fs::create_dir_all(&subdir_path).map_err(|e| Error::FileIo {
                    path: subdir_path.clone(),
                    source: e,
                })?;

                // Recursively extract subdirectory contents
                extract_entries(subdir, target_base, resource_type, count)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_embedded_resources_new() {
        let _resources = EmbeddedResources::new();
        // Test that new() constructs successfully
    }

    #[test]
    fn test_embedded_resources_default() {
        let _resources = EmbeddedResources::default();
        // Test that default() constructs successfully
    }

    #[test]
    fn test_list_components_not_empty() {
        let resources = EmbeddedResources::new();
        let components = resources.list_components();
        assert!(!components.is_empty(), "Should have embedded components");
    }

    #[test]
    fn test_list_templates_not_empty() {
        let resources = EmbeddedResources::new();
        let templates = resources.list_templates();
        assert!(!templates.is_empty(), "Should have embedded templates");
    }

    #[test]
    fn test_list_components_paths_are_relative() {
        let resources = EmbeddedResources::new();
        let components = resources.list_components();

        for component in components {
            assert!(
                !component.starts_with('/'),
                "Component path should be relative: {}",
                component
            );
            assert!(!component.is_empty(), "Component path should not be empty");
        }
    }

    #[test]
    fn test_list_templates_paths_are_relative() {
        let resources = EmbeddedResources::new();
        let templates = resources.list_templates();

        for template in templates {
            assert!(
                !template.starts_with('/'),
                "Template path should be relative: {}",
                template
            );
            assert!(!template.is_empty(), "Template path should not be empty");
        }
    }

    #[test]
    fn test_get_component_returns_content() {
        let resources = EmbeddedResources::new();
        let components = resources.list_components();

        assert!(!components.is_empty(), "Need components for this test");

        // Try to get the first component
        let first_component = &components[0];
        let content = resources.get_component(first_component);

        assert!(
            content.is_some(),
            "Should be able to get component: {}",
            first_component
        );
        assert!(
            !content.unwrap().is_empty(),
            "Component content should not be empty"
        );
    }

    #[test]
    fn test_get_template_returns_content() {
        let resources = EmbeddedResources::new();
        let templates = resources.list_templates();

        assert!(!templates.is_empty(), "Need templates for this test");

        // Try to get the first template
        let first_template = &templates[0];
        let content = resources.get_template(first_template);

        assert!(
            content.is_some(),
            "Should be able to get template: {}",
            first_template
        );
        assert!(
            !content.unwrap().is_empty(),
            "Template content should not be empty"
        );
    }

    #[test]
    fn test_get_component_nonexistent_returns_none() {
        let resources = EmbeddedResources::new();
        let content = resources.get_component("nonexistent/component.md");
        assert!(content.is_none());
    }

    #[test]
    fn test_get_template_nonexistent_returns_none() {
        let resources = EmbeddedResources::new();
        let content = resources.get_template("nonexistent/template.md");
        assert!(content.is_none());
    }

    #[test]
    fn test_has_component() {
        let resources = EmbeddedResources::new();
        let components = resources.list_components();

        assert!(!components.is_empty(), "Need components for this test");

        let first_component = &components[0];
        assert!(
            resources.has_component(first_component),
            "Should find component: {}",
            first_component
        );
        assert!(
            !resources.has_component("nonexistent/component.md"),
            "Should not find nonexistent component"
        );
    }

    #[test]
    fn test_has_template() {
        let resources = EmbeddedResources::new();
        let templates = resources.list_templates();

        assert!(!templates.is_empty(), "Need templates for this test");

        let first_template = &templates[0];
        assert!(
            resources.has_template(first_template),
            "Should find template: {}",
            first_template
        );
        assert!(
            !resources.has_template("nonexistent/template.md"),
            "Should not find nonexistent template"
        );
    }

    #[test]
    fn test_extract_components_to() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir.path().join("components");

        let count = resources
            .extract_components_to(&target)
            .expect("Should extract components");

        assert!(count > 0, "Should extract at least one component");
        assert!(target.exists(), "Target directory should exist");
        assert!(target.is_dir(), "Target should be a directory");

        // Verify some files were created
        let components = resources.list_components();
        let first_component = &components[0];
        let extracted_file = target.join(first_component);
        assert!(
            extracted_file.exists(),
            "Extracted component file should exist: {:?}",
            extracted_file
        );
    }

    #[test]
    fn test_extract_templates_to() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir.path().join("templates");

        let count = resources
            .extract_templates_to(&target)
            .expect("Should extract templates");

        assert!(count > 0, "Should extract at least one template");
        assert!(target.exists(), "Target directory should exist");
        assert!(target.is_dir(), "Target should be a directory");

        // Verify some files were created
        let templates = resources.list_templates();
        let first_template = &templates[0];
        let extracted_file = target.join(first_template);
        assert!(
            extracted_file.exists(),
            "Extracted template file should exist: {:?}",
            extracted_file
        );
    }

    #[test]
    fn test_extract_all_to() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();

        let (comp_count, temp_count) = resources
            .extract_all_to(temp_dir.path())
            .expect("Should extract all resources");

        assert!(comp_count > 0, "Should extract at least one component");
        assert!(temp_count > 0, "Should extract at least one template");

        let components_dir = temp_dir.path().join("components");
        let templates_dir = temp_dir.path().join("templates");

        assert!(components_dir.exists(), "Components directory should exist");
        assert!(templates_dir.exists(), "Templates directory should exist");
    }

    #[test]
    fn test_extract_preserves_directory_structure() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir.path().join("components");

        resources
            .extract_components_to(&target)
            .expect("Should extract components");

        // Check that subdirectories are preserved
        let components = resources.list_components();
        for component_path in components {
            let extracted_path = target.join(&component_path);
            assert!(
                extracted_path.exists(),
                "File should exist at preserved path: {:?}",
                extracted_path
            );

            // Verify content matches
            let embedded_content = resources
                .get_component(&component_path)
                .expect("Should get embedded content");
            let extracted_content =
                std::fs::read_to_string(&extracted_path).expect("Should read extracted file");
            assert_eq!(
                embedded_content, extracted_content,
                "Content should match for: {}",
                component_path
            );
        }
    }

    #[test]
    fn test_extract_creates_parent_directories() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir
            .path()
            .join("deep")
            .join("nested")
            .join("components");

        let count = resources
            .extract_components_to(&target)
            .expect("Should extract components");

        assert!(count > 0, "Should extract components");
        assert!(target.exists(), "Nested target directory should exist");
    }

    #[test]
    fn test_extract_to_existing_directory() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir.path().join("components");

        // Create directory first
        std::fs::create_dir_all(&target).unwrap();

        // Should still work
        let count = resources
            .extract_components_to(&target)
            .expect("Should extract to existing directory");

        assert!(count > 0, "Should extract components");
    }

    #[test]
    fn test_extract_components_and_templates_independent() {
        let resources = EmbeddedResources::new();
        let temp_dir = TempDir::new().unwrap();
        let comp_target = temp_dir.path().join("components");
        let temp_target = temp_dir.path().join("templates");

        let comp_count = resources
            .extract_components_to(&comp_target)
            .expect("Should extract components");
        let temp_count = resources
            .extract_templates_to(&temp_target)
            .expect("Should extract templates");

        assert!(comp_count > 0);
        assert!(temp_count > 0);
        assert!(comp_target.exists());
        assert!(temp_target.exists());

        // Verify no cross-contamination
        assert!(
            !comp_target.join("prompts").exists(),
            "Components dir should not contain templates"
        );
        assert!(
            !temp_target.join("core").exists(),
            "Templates dir should not contain components"
        );
    }

    #[test]
    fn test_collect_file_paths_empty_prefix() {
        let mut paths = Vec::new();
        collect_file_paths(&COMPONENTS_DIR, "", &mut paths);
        assert!(!paths.is_empty());

        for path in paths {
            assert!(!path.starts_with('/'));
            assert!(!path.is_empty());
        }
    }

    #[test]
    fn test_collect_file_paths_with_prefix() {
        let mut paths = Vec::new();
        collect_file_paths(&COMPONENTS_DIR, "prefix", &mut paths);
        assert!(!paths.is_empty());

        for path in paths {
            assert!(
                path.starts_with("prefix/"),
                "Path should have prefix: {}",
                path
            );
        }
    }
}
