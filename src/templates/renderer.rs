//! Placeholder rendering with case transformations
//!
//! This module provides the `PlaceholderRenderer` for replacing placeholders
//! in templates with actual values, supporting various case transformations.

use std::collections::HashMap;

use crate::{Error, Result};

/// Renders placeholders in templates with value substitution
///
/// The renderer supports:
/// - Simple placeholder replacement: `{{name}}`
/// - Case transformations: `{{name:upper}}`, `{{name:lower}}`, `{{name:snake}}`, `{{name:kebab}}`
/// - Default values: `{{name:default=value}}`
/// - Error handling for missing required placeholders
///
/// # Examples
///
/// ```
/// use xzagentz::templates::PlaceholderRenderer;
/// use std::collections::HashMap;
///
/// let mut values = HashMap::new();
/// values.insert("project_name".to_string(), "MyProject".to_string());
///
/// let renderer = PlaceholderRenderer::new(values);
/// let result = renderer.render("Project: {{project_name}}").unwrap();
/// assert_eq!(result, "Project: MyProject");
/// ```
#[derive(Debug, Clone)]
pub struct PlaceholderRenderer {
    /// Placeholder values
    values: HashMap<String, String>,
}

impl PlaceholderRenderer {
    /// Creates a new placeholder renderer with values
    ///
    /// # Arguments
    ///
    /// * `values` - Map of placeholder names to their values
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    /// use std::collections::HashMap;
    ///
    /// let mut values = HashMap::new();
    /// values.insert("name".to_string(), "test".to_string());
    ///
    /// let renderer = PlaceholderRenderer::new(values);
    /// ```
    pub fn new(values: HashMap<String, String>) -> Self {
        Self { values }
    }

    /// Creates an empty renderer
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    ///
    /// let renderer = PlaceholderRenderer::empty();
    /// ```
    pub fn empty() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Adds or updates a placeholder value
    ///
    /// # Arguments
    ///
    /// * `name` - Placeholder name
    /// * `value` - Placeholder value
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    ///
    /// let mut renderer = PlaceholderRenderer::empty();
    /// renderer.set("project", "myapp");
    /// ```
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }

    /// Gets a placeholder value
    ///
    /// # Arguments
    ///
    /// * `name` - Placeholder name
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    /// use std::collections::HashMap;
    ///
    /// let mut values = HashMap::new();
    /// values.insert("name".to_string(), "test".to_string());
    ///
    /// let renderer = PlaceholderRenderer::new(values);
    /// assert_eq!(renderer.get("name"), Some(&"test".to_string()));
    /// ```
    pub fn get(&self, name: &str) -> Option<&String> {
        self.values.get(name)
    }

    /// Renders a template string by replacing all placeholders
    ///
    /// Supports the following placeholder formats:
    /// - `{{name}}` - Simple replacement
    /// - `{{name:upper}}` - Uppercase transformation
    /// - `{{name:lower}}` - Lowercase transformation
    /// - `{{name:snake}}` - snake_case transformation
    /// - `{{name:kebab}}` - kebab-case transformation
    /// - `{{name:default=value}}` - Default value if not found
    ///
    /// # Arguments
    ///
    /// * `template` - Template string with placeholders
    ///
    /// # Returns
    ///
    /// Returns the rendered string with all placeholders replaced
    ///
    /// # Errors
    ///
    /// Returns `Error::PlaceholderNotFound` if a required placeholder is missing
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    /// use std::collections::HashMap;
    ///
    /// let mut values = HashMap::new();
    /// values.insert("name".to_string(), "MyProject".to_string());
    ///
    /// let renderer = PlaceholderRenderer::new(values);
    /// let result = renderer.render("Welcome to {{name}}!").unwrap();
    /// assert_eq!(result, "Welcome to MyProject!");
    /// ```
    pub fn render(&self, template: &str) -> Result<String> {
        let mut result = template.to_string();
        let mut offset = 0;

        while let Some(start) = result[offset..].find("{{") {
            let start = offset + start;
            if let Some(end) = result[start..].find("}}") {
                let end = start + end;
                let placeholder = &result[start + 2..end];

                let replacement = self.resolve_placeholder(placeholder)?;

                result.replace_range(start..end + 2, &replacement);
                offset = start + replacement.len();
            } else {
                return Err(Error::TemplateParse(format!(
                    "Unclosed placeholder at position {}",
                    start
                )));
            }
        }

        Ok(result)
    }

    /// Resolves a single placeholder with transformations
    ///
    /// # Arguments
    ///
    /// * `placeholder` - Placeholder content (without {{ }})
    ///
    /// # Errors
    ///
    /// Returns `Error::PlaceholderNotFound` if the placeholder value is missing
    fn resolve_placeholder(&self, placeholder: &str) -> Result<String> {
        let placeholder = placeholder.trim();

        if placeholder.is_empty() {
            return Ok(String::new());
        }

        // Check for default value syntax: name:default=value
        if let Some(default_pos) = placeholder.find(":default=") {
            let name = &placeholder[..default_pos];
            let default_value = &placeholder[default_pos + 9..];

            if let Some(value) = self.values.get(name) {
                return Ok(value.clone());
            } else {
                return Ok(default_value.to_string());
            }
        }

        // Check for transformation syntax: name:transform
        if let Some(colon_pos) = placeholder.find(':') {
            let name = &placeholder[..colon_pos];
            let transform = &placeholder[colon_pos + 1..];

            let value = self.values.get(name).ok_or_else(|| {
                Error::PlaceholderNotFound(format!("Placeholder '{}' not found", name))
            })?;

            return Ok(self.apply_transformation(value, transform));
        }

        // Simple placeholder
        let value = self.values.get(placeholder).ok_or_else(|| {
            Error::PlaceholderNotFound(format!("Placeholder '{}' not found", placeholder))
        })?;

        Ok(value.clone())
    }

    /// Applies a case transformation to a value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to transform
    /// * `transform` - The transformation type (upper, lower, snake, kebab)
    fn apply_transformation(&self, value: &str, transform: &str) -> String {
        match transform {
            "upper" => value.to_uppercase(),
            "lower" => value.to_lowercase(),
            "snake" => self.to_snake_case(value),
            "kebab" => self.to_kebab_case(value),
            _ => value.to_string(), // Unknown transformation, return as-is
        }
    }

    /// Converts a string to snake_case
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert
    fn to_snake_case(&self, value: &str) -> String {
        let mut result = String::new();
        let mut prev_lowercase = false;

        for (i, ch) in value.chars().enumerate() {
            if ch.is_uppercase() && i > 0 && prev_lowercase {
                result.push('_');
                result.push(ch.to_lowercase().next().unwrap());
                prev_lowercase = false;
            } else if ch.is_whitespace() || ch == '-' {
                result.push('_');
                prev_lowercase = false;
            } else {
                result.push(ch.to_lowercase().next().unwrap());
                prev_lowercase = ch.is_lowercase();
            }
        }

        result
    }

    /// Converts a string to kebab-case
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert
    fn to_kebab_case(&self, value: &str) -> String {
        let mut result = String::new();
        let mut prev_lowercase = false;

        for (i, ch) in value.chars().enumerate() {
            if ch.is_uppercase() && i > 0 && prev_lowercase {
                result.push('-');
                result.push(ch.to_lowercase().next().unwrap());
                prev_lowercase = false;
            } else if ch.is_whitespace() || ch == '_' {
                result.push('-');
                prev_lowercase = false;
            } else {
                result.push(ch.to_lowercase().next().unwrap());
                prev_lowercase = ch.is_lowercase();
            }
        }

        result
    }

    /// Returns the number of placeholder values
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    /// use std::collections::HashMap;
    ///
    /// let mut values = HashMap::new();
    /// values.insert("a".to_string(), "1".to_string());
    /// values.insert("b".to_string(), "2".to_string());
    ///
    /// let renderer = PlaceholderRenderer::new(values);
    /// assert_eq!(renderer.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if no placeholder values are set
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    ///
    /// let renderer = PlaceholderRenderer::empty();
    /// assert!(renderer.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns all placeholder values
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::templates::PlaceholderRenderer;
    /// use std::collections::HashMap;
    ///
    /// let mut values = HashMap::new();
    /// values.insert("name".to_string(), "test".to_string());
    ///
    /// let renderer = PlaceholderRenderer::new(values.clone());
    /// assert_eq!(renderer.values(), &values);
    /// ```
    pub fn values(&self) -> &HashMap<String, String> {
        &self.values
    }
}

impl Default for PlaceholderRenderer {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_new() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "test".to_string());

        let renderer = PlaceholderRenderer::new(values);
        assert_eq!(renderer.len(), 1);
    }

    #[test]
    fn test_renderer_empty() {
        let renderer = PlaceholderRenderer::empty();
        assert_eq!(renderer.len(), 0);
        assert!(renderer.is_empty());
    }

    #[test]
    fn test_renderer_set_and_get() {
        let mut renderer = PlaceholderRenderer::empty();
        renderer.set("key", "value");

        assert_eq!(renderer.get("key"), Some(&"value".to_string()));
        assert_eq!(renderer.len(), 1);
    }

    #[test]
    fn test_render_simple_placeholder() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "MyProject".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("Welcome to {{name}}!").unwrap();

        assert_eq!(result, "Welcome to MyProject!");
    }

    #[test]
    fn test_render_multiple_placeholders() {
        let mut values = HashMap::new();
        values.insert("project".to_string(), "xzagentz".to_string());
        values.insert("version".to_string(), "1.0.0".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer
            .render("Project: {{project}} v{{version}}")
            .unwrap();

        assert_eq!(result, "Project: xzagentz v1.0.0");
    }

    #[test]
    fn test_render_missing_placeholder() {
        let renderer = PlaceholderRenderer::empty();
        let result = renderer.render("Hello {{name}}!");

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::PlaceholderNotFound(_))));
    }

    #[test]
    fn test_render_empty_placeholder() {
        let renderer = PlaceholderRenderer::empty();
        let result = renderer.render("Hello {{}}!").unwrap();

        assert_eq!(result, "Hello !");
    }

    #[test]
    fn test_render_unclosed_placeholder() {
        let renderer = PlaceholderRenderer::empty();
        let result = renderer.render("Hello {{name!");

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::TemplateParse(_))));
    }

    #[test]
    fn test_render_uppercase_transformation() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "MyProject".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:upper}}").unwrap();

        assert_eq!(result, "MYPROJECT");
    }

    #[test]
    fn test_render_lowercase_transformation() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "MyProject".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:lower}}").unwrap();

        assert_eq!(result, "myproject");
    }

    #[test]
    fn test_render_snake_case_transformation() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "MyProject".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:snake}}").unwrap();

        assert_eq!(result, "my_project");
    }

    #[test]
    fn test_render_kebab_case_transformation() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "MyProject".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:kebab}}").unwrap();

        assert_eq!(result, "my-project");
    }

    #[test]
    fn test_render_snake_case_from_spaces() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "My Project Name".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:snake}}").unwrap();

        assert_eq!(result, "my_project_name");
    }

    #[test]
    fn test_render_kebab_case_from_underscores() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "my_project_name".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:kebab}}").unwrap();

        assert_eq!(result, "my-project-name");
    }

    #[test]
    fn test_render_default_value() {
        let renderer = PlaceholderRenderer::empty();
        let result = renderer.render("Name: {{name:default=Unknown}}").unwrap();

        assert_eq!(result, "Name: Unknown");
    }

    #[test]
    fn test_render_default_value_when_present() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Actual".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("Name: {{name:default=Unknown}}").unwrap();

        assert_eq!(result, "Name: Actual");
    }

    #[test]
    fn test_render_unknown_transformation() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "test".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let result = renderer.render("{{name:unknown}}").unwrap();

        assert_eq!(result, "test"); // Returns as-is for unknown transformation
    }

    #[test]
    fn test_render_complex_template() {
        let mut values = HashMap::new();
        values.insert("project".to_string(), "MyProject".to_string());
        values.insert("author".to_string(), "John Doe".to_string());

        let renderer = PlaceholderRenderer::new(values);
        let template = r#"
# {{project}}

Created by {{author}}

Repository: {{project:kebab}}
Module: {{project:snake}}
Constant: {{project:upper}}
"#;

        let result = renderer.render(template).unwrap();

        assert!(result.contains("# MyProject"));
        assert!(result.contains("Created by John Doe"));
        assert!(result.contains("Repository: my-project"));
        assert!(result.contains("Module: my_project"));
        assert!(result.contains("Constant: MYPROJECT"));
    }

    #[test]
    fn test_render_no_placeholders() {
        let renderer = PlaceholderRenderer::empty();
        let result = renderer.render("Hello, World!").unwrap();

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_renderer_is_empty() {
        let renderer = PlaceholderRenderer::empty();
        assert!(renderer.is_empty());

        let mut renderer2 = PlaceholderRenderer::empty();
        renderer2.set("key", "value");
        assert!(!renderer2.is_empty());
    }

    #[test]
    fn test_renderer_values() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), "1".to_string());
        values.insert("b".to_string(), "2".to_string());

        let renderer = PlaceholderRenderer::new(values.clone());
        assert_eq!(renderer.values(), &values);
    }

    #[test]
    fn test_to_snake_case_variations() {
        let renderer = PlaceholderRenderer::empty();

        assert_eq!(renderer.to_snake_case("MyProject"), "my_project");
        assert_eq!(renderer.to_snake_case("myProject"), "my_project");
        assert_eq!(renderer.to_snake_case("my-project"), "my_project");
        assert_eq!(renderer.to_snake_case("my project"), "my_project");
        assert_eq!(renderer.to_snake_case("MY_PROJECT"), "my_project");
    }

    #[test]
    fn test_to_kebab_case_variations() {
        let renderer = PlaceholderRenderer::empty();

        assert_eq!(renderer.to_kebab_case("MyProject"), "my-project");
        assert_eq!(renderer.to_kebab_case("myProject"), "my-project");
        assert_eq!(renderer.to_kebab_case("my_project"), "my-project");
        assert_eq!(renderer.to_kebab_case("my project"), "my-project");
        assert_eq!(renderer.to_kebab_case("MY-PROJECT"), "my-project");
    }
}
