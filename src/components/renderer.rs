// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Component renderer with language filtering and size enforcement
//!
//! This module provides functionality for rendering components with language-specific
//! content filtering and size limit enforcement according to the hybrid component system.
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::components::renderer::{ComponentRenderer, RenderConfig};
//!
//! let config = RenderConfig::new("rust");
//! let renderer = ComponentRenderer::new(config);
//!
//! let content = r#"---
//! component:
//!   name: test
//!   category: core
//!   version: 1.0.0
//! ---
//! # Content
//! <!-- LANG:rust -->
//! Rust content
//! <!-- /LANG -->
//! "#;
//!
//! let rendered = renderer.render(content)?;
//! assert!(rendered.contains("Rust content"));
//! # Ok::<(), xzagentz::Error>(())
//! ```

use crate::components::language_filter::LanguageFilter;
use crate::components::metadata::ComponentMetadata;
use crate::error::{Error, Result};

/// Configuration for component rendering
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Target language for rendering
    pub target_language: String,

    /// Whether to include fallback content (LANG:*)
    pub include_fallback: bool,

    /// Whether to include component metadata in output
    pub include_metadata: bool,

    /// Maximum lines per component (0 = no limit)
    pub max_lines_per_component: usize,

    /// Whether to enforce strict size limits
    pub strict_size_limits: bool,
}

impl RenderConfig {
    /// Creates a new RenderConfig with defaults
    ///
    /// # Arguments
    ///
    /// * `target_language` - Language to render for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::RenderConfig;
    ///
    /// let config = RenderConfig::new("rust");
    /// assert_eq!(config.target_language, "rust");
    /// ```
    pub fn new(target_language: impl Into<String>) -> Self {
        Self {
            target_language: target_language.into(),
            include_fallback: true,
            include_metadata: false,
            max_lines_per_component: 0,
            strict_size_limits: false,
        }
    }

    /// Sets whether to include fallback content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::RenderConfig;
    ///
    /// let config = RenderConfig::new("rust").with_fallback(false);
    /// assert!(!config.include_fallback);
    /// ```
    pub fn with_fallback(mut self, include_fallback: bool) -> Self {
        self.include_fallback = include_fallback;
        self
    }

    /// Sets whether to include metadata
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::RenderConfig;
    ///
    /// let config = RenderConfig::new("rust").with_metadata(true);
    /// assert!(config.include_metadata);
    /// ```
    pub fn with_metadata(mut self, include_metadata: bool) -> Self {
        self.include_metadata = include_metadata;
        self
    }

    /// Sets maximum lines per component
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::RenderConfig;
    ///
    /// let config = RenderConfig::new("rust").with_max_lines(150);
    /// assert_eq!(config.max_lines_per_component, 150);
    /// ```
    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines_per_component = max_lines;
        self
    }

    /// Sets whether to enforce strict size limits
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::RenderConfig;
    ///
    /// let config = RenderConfig::new("rust").with_strict_limits(true);
    /// assert!(config.strict_size_limits);
    /// ```
    pub fn with_strict_limits(mut self, strict: bool) -> Self {
        self.strict_size_limits = strict;
        self
    }
}

/// Statistics about rendered content
#[derive(Debug, Clone, PartialEq)]
pub struct RenderStats {
    /// Number of lines in rendered content
    pub line_count: usize,

    /// Number of words in rendered content
    pub word_count: usize,

    /// Number of language sections processed
    pub sections_processed: usize,

    /// Whether metadata was included
    pub has_metadata: bool,
}

impl RenderStats {
    /// Creates RenderStats from rendered content
    fn from_content(content: &str, sections_processed: usize, has_metadata: bool) -> Self {
        Self {
            line_count: content.lines().count(),
            word_count: content.split_whitespace().count(),
            sections_processed,
            has_metadata,
        }
    }
}

/// Component renderer with language filtering
#[derive(Debug, Clone)]
pub struct ComponentRenderer {
    /// Rendering configuration
    config: RenderConfig,
}

impl ComponentRenderer {
    /// Creates a new ComponentRenderer
    ///
    /// # Arguments
    ///
    /// * `config` - Rendering configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::{ComponentRenderer, RenderConfig};
    ///
    /// let config = RenderConfig::new("rust");
    /// let renderer = ComponentRenderer::new(config);
    /// ```
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    /// Renders component content for the target language
    ///
    /// Processes component content by:
    /// 1. Parsing YAML frontmatter (if present)
    /// 2. Filtering language-specific sections
    /// 3. Enforcing size limits (if configured)
    /// 4. Including metadata (if configured)
    ///
    /// # Arguments
    ///
    /// * `content` - Raw component content
    ///
    /// # Returns
    ///
    /// Returns rendered content string
    ///
    /// # Errors
    ///
    /// Returns `Error::ComponentValidation` if:
    /// - Content exceeds size limits (when strict mode enabled)
    /// - Language section markers are malformed
    /// - YAML frontmatter is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::{ComponentRenderer, RenderConfig};
    ///
    /// let config = RenderConfig::new("rust");
    /// let renderer = ComponentRenderer::new(config);
    ///
    /// let content = "# Test\nContent here";
    /// let rendered = renderer.render(content)?;
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn render(&self, content: &str) -> Result<String> {
        // Check if content has frontmatter
        let (metadata_opt, body) = if ComponentMetadata::has_frontmatter(content) {
            let (metadata, body) = ComponentMetadata::parse_frontmatter(content)?;
            (Some(metadata), body)
        } else {
            (None, content.to_string())
        };

        // Filter language-specific content
        let filter = LanguageFilter::with_fallback(
            &self.config.target_language,
            self.config.include_fallback,
        );
        let filtered_content = filter.filter_content(&body)?;

        // Build final output
        let mut output = String::new();

        // Add metadata if requested
        if self.config.include_metadata {
            if let Some(ref metadata) = metadata_opt {
                output.push_str(&format!(
                    "<!-- Component: {} (v{}) -->\n",
                    metadata.component.name, metadata.component.version
                ));
                if let Some(ref desc) = metadata.component.description {
                    output.push_str(&format!("<!-- {} -->\n", desc));
                }
                output.push('\n');
            }
        }

        output.push_str(&filtered_content);

        // Enforce size limits if configured
        if self.config.max_lines_per_component > 0 {
            let line_count = output.lines().count();
            if line_count > self.config.max_lines_per_component {
                if self.config.strict_size_limits {
                    return Err(Error::ComponentValidation {
                        reason: format!(
                            "Component exceeds size limit: {} lines (max: {})",
                            line_count, self.config.max_lines_per_component
                        ),
                    });
                } else {
                    // Log warning (would use tracing in production)
                    eprintln!(
                        "Warning: Component exceeds size limit: {} lines (max: {})",
                        line_count, self.config.max_lines_per_component
                    );
                }
            }
        }

        Ok(output)
    }

    /// Renders component content and returns statistics
    ///
    /// # Arguments
    ///
    /// * `content` - Raw component content
    ///
    /// # Returns
    ///
    /// Returns tuple of (rendered_content, statistics)
    ///
    /// # Errors
    ///
    /// Returns same errors as `render()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::{ComponentRenderer, RenderConfig};
    ///
    /// let config = RenderConfig::new("rust");
    /// let renderer = ComponentRenderer::new(config);
    ///
    /// let content = "# Test\nContent here";
    /// let (rendered, stats) = renderer.render_with_stats(content)?;
    /// assert_eq!(stats.line_count, 2);
    /// # Ok::<(), xzagentz::Error>(())
    /// ```
    pub fn render_with_stats(&self, content: &str) -> Result<(String, RenderStats)> {
        let rendered = self.render(content)?;

        let filter = LanguageFilter::new(&self.config.target_language);
        let sections_processed = if filter.has_language_sections(content) {
            filter.parse_sections(content)?.len()
        } else {
            0
        };

        let stats =
            RenderStats::from_content(&rendered, sections_processed, self.config.include_metadata);

        Ok((rendered, stats))
    }

    /// Checks if content would exceed size limits
    ///
    /// # Arguments
    ///
    /// * `content` - Content to check
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if within limits, error if exceeds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::components::renderer::{ComponentRenderer, RenderConfig};
    ///
    /// let config = RenderConfig::new("rust").with_max_lines(10);
    /// let renderer = ComponentRenderer::new(config);
    ///
    /// let short_content = "Line 1\nLine 2";
    /// assert!(renderer.check_size_limits(short_content).is_ok());
    /// ```
    pub fn check_size_limits(&self, content: &str) -> Result<()> {
        if self.config.max_lines_per_component == 0 {
            return Ok(());
        }

        let line_count = content.lines().count();
        if line_count > self.config.max_lines_per_component {
            return Err(Error::ComponentValidation {
                reason: format!(
                    "Content exceeds size limit: {} lines (max: {})",
                    line_count, self.config.max_lines_per_component
                ),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_config_new() {
        let config = RenderConfig::new("rust");
        assert_eq!(config.target_language, "rust");
        assert!(config.include_fallback);
        assert!(!config.include_metadata);
        assert_eq!(config.max_lines_per_component, 0);
        assert!(!config.strict_size_limits);
    }

    #[test]
    fn test_render_config_builder() {
        let config = RenderConfig::new("rust")
            .with_fallback(false)
            .with_metadata(true)
            .with_max_lines(150)
            .with_strict_limits(true);

        assert!(!config.include_fallback);
        assert!(config.include_metadata);
        assert_eq!(config.max_lines_per_component, 150);
        assert!(config.strict_size_limits);
    }

    #[test]
    fn test_render_simple_content() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = "# Test Component\n\nSome content here";
        let result = renderer.render(content).unwrap();

        assert_eq!(result, content);
    }

    #[test]
    fn test_render_with_language_sections() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = r#"# Test

Common content

<!-- LANG:rust -->
Rust-specific content
<!-- /LANG -->

<!-- LANG:python -->
Python-specific content
<!-- /LANG -->

More common content"#;

        let result = renderer.render(content).unwrap();

        assert!(result.contains("Common content"));
        assert!(result.contains("Rust-specific content"));
        assert!(!result.contains("Python-specific content"));
        assert!(result.contains("More common content"));
    }

    #[test]
    fn test_render_with_frontmatter() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = r#"---
component:
  name: test
  category: core
  version: 1.0.0
---
# Content
Test content"#;

        let result = renderer.render(content).unwrap();

        assert!(!result.contains("---"));
        assert!(!result.contains("component:"));
        assert!(result.contains("# Content"));
        assert!(result.contains("Test content"));
    }

    #[test]
    fn test_render_with_metadata_included() {
        let config = RenderConfig::new("rust").with_metadata(true);
        let renderer = ComponentRenderer::new(config);

        let content = r#"---
component:
  name: test_component
  category: core
  version: 2.0.0
  description: Test description
---
# Content"#;

        let result = renderer.render(content).unwrap();

        assert!(result.contains("<!-- Component: test_component (v2.0.0) -->"));
        assert!(result.contains("<!-- Test description -->"));
        assert!(result.contains("# Content"));
    }

    #[test]
    fn test_render_with_size_limit_warning() {
        let config = RenderConfig::new("rust")
            .with_max_lines(3)
            .with_strict_limits(false);
        let renderer = ComponentRenderer::new(config);

        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let result = renderer.render(content);

        // Should succeed but warn
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_with_size_limit_strict() {
        let config = RenderConfig::new("rust")
            .with_max_lines(3)
            .with_strict_limits(true);
        let renderer = ComponentRenderer::new(config);

        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let result = renderer.render(content);

        // Should fail in strict mode
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("exceeds size limit"));
    }

    #[test]
    fn test_render_with_stats() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = r#"# Test
Line 2
Line 3

<!-- LANG:rust -->
Rust content
<!-- /LANG -->"#;

        let (rendered, stats) = renderer.render_with_stats(content).unwrap();

        assert!(!rendered.is_empty());
        assert!(stats.line_count > 0);
        assert!(stats.word_count > 0);
        assert_eq!(stats.sections_processed, 1);
        assert!(!stats.has_metadata);
    }

    #[test]
    fn test_render_stats_from_content() {
        let content = "Line 1\nLine 2\nLine 3";
        let stats = RenderStats::from_content(content, 2, true);

        assert_eq!(stats.line_count, 3);
        assert_eq!(stats.word_count, 6);
        assert_eq!(stats.sections_processed, 2);
        assert!(stats.has_metadata);
    }

    #[test]
    fn test_check_size_limits_pass() {
        let config = RenderConfig::new("rust").with_max_lines(10);
        let renderer = ComponentRenderer::new(config);

        let content = "Line 1\nLine 2\nLine 3";
        assert!(renderer.check_size_limits(content).is_ok());
    }

    #[test]
    fn test_check_size_limits_fail() {
        let config = RenderConfig::new("rust").with_max_lines(2);
        let renderer = ComponentRenderer::new(config);

        let content = "Line 1\nLine 2\nLine 3";
        let result = renderer.check_size_limits(content);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("exceeds size limit"));
    }

    #[test]
    fn test_check_size_limits_no_limit() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = "Line 1\n".repeat(1000);
        assert!(renderer.check_size_limits(&content).is_ok());
    }

    #[test]
    fn test_render_with_fallback_content() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = r#"
<!-- LANG:* -->
Fallback content for all languages
<!-- /LANG -->

<!-- LANG:rust -->
Rust-specific content
<!-- /LANG -->
"#;

        let result = renderer.render(content).unwrap();

        assert!(result.contains("Fallback content"));
        assert!(result.contains("Rust-specific content"));
    }

    #[test]
    fn test_render_without_fallback() {
        let config = RenderConfig::new("rust").with_fallback(false);
        let renderer = ComponentRenderer::new(config);

        let content = r#"
<!-- LANG:* -->
Fallback content
<!-- /LANG -->

<!-- LANG:rust -->
Rust content
<!-- /LANG -->
"#;

        let result = renderer.render(content).unwrap();

        assert!(!result.contains("Fallback content"));
        assert!(result.contains("Rust content"));
    }

    #[test]
    fn test_render_empty_content() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let result = renderer.render("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_no_matching_language() {
        let config = RenderConfig::new("rust");
        let renderer = ComponentRenderer::new(config);

        let content = r#"
Common content

<!-- LANG:python -->
Python only
<!-- /LANG -->

<!-- LANG:golang -->
Go only
<!-- /LANG -->
"#;

        let result = renderer.render(content).unwrap();

        assert!(result.contains("Common content"));
        assert!(!result.contains("Python only"));
        assert!(!result.contains("Go only"));
    }
}
