//! Markdown architecture writer implementation
//!
//! This module provides an implementation of the `ArchitectureWriter` trait
//! that outputs architecture documents as formatted Markdown files.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::{
//!     ArchitectureWriter, ArchitectureDocument, ArchitectureMetadata,
//!     Overview, ArchitecturePattern,
//! };
//! use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
//! use chrono::Utc;
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let writer = MarkdownArchitectureWriter::new();
//!
//! let document = ArchitectureDocument {
//!     metadata: ArchitectureMetadata {
//!         title: "My System".to_string(),
//!         version: "1.0.0".to_string(),
//!         generated_at: Utc::now(),
//!         model_used: "llama3.2:3b".to_string(),
//!         pattern: ArchitecturePattern::Microservices,
//!         authors: vec!["Team".to_string()],
//!     },
//!     overview: Overview {
//!         description: "System description".to_string(),
//!         business_goals: vec![],
//!         constraints: vec![],
//!         assumptions: vec![],
//!     },
//!     layers: vec![],
//!     components: vec![],
//!     integrations: vec![],
//!     deployment: None,
//!     quality_attributes: vec![],
//! };
//!
//! writer.write(&document, Path::new("architecture.md")).await?;
//! # Ok(())
//! # }
//! ```

use crate::domain::architecture::{ArchitectureDocument, ArchitectureWriter};
use async_trait::async_trait;
use std::path::Path;
use thiserror::Error;
use tokio::fs;

/// Markdown architecture writer
///
/// Formats architecture documents as Markdown files with clear structure
/// and formatting.
///
/// # Examples
///
/// ```rust
/// use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
///
/// let writer = MarkdownArchitectureWriter::new();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MarkdownArchitectureWriter;

impl MarkdownArchitectureWriter {
    /// Create a new Markdown architecture writer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::writers::MarkdownArchitectureWriter;
    ///
    /// let writer = MarkdownArchitectureWriter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Format metadata section
    fn format_metadata(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        output.push_str(&format!("# {}\n\n", document.metadata.title));
        output.push_str("## Metadata\n\n");
        output.push_str(&format!("- **Version**: {}\n", document.metadata.version));
        output.push_str(&format!(
            "- **Generated**: {}\n",
            document
                .metadata
                .generated_at
                .format("%Y-%m-%d %H:%M:%S UTC")
        ));
        output.push_str(&format!("- **Model**: {}\n", document.metadata.model_used));
        output.push_str(&format!("- **Pattern**: {}\n", document.metadata.pattern));
        output.push_str(&format!(
            "- **Authors**: {}\n\n",
            document.metadata.authors.join(", ")
        ));

        output
    }

    /// Format overview section
    fn format_overview(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        output.push_str("## Overview\n\n");
        output.push_str(&format!("{}\n\n", document.overview.description));

        if !document.overview.business_goals.is_empty() {
            output.push_str("### Business Goals\n\n");
            for goal in &document.overview.business_goals {
                output.push_str(&format!("- {}\n", goal));
            }
            output.push('\n');
        }

        if !document.overview.constraints.is_empty() {
            output.push_str("### Constraints\n\n");
            for constraint in &document.overview.constraints {
                output.push_str(&format!("- {}\n", constraint));
            }
            output.push('\n');
        }

        if !document.overview.assumptions.is_empty() {
            output.push_str("### Assumptions\n\n");
            for assumption in &document.overview.assumptions {
                output.push_str(&format!("- {}\n", assumption));
            }
            output.push('\n');
        }

        output
    }

    /// Format layers section
    fn format_layers(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        if document.layers.is_empty() {
            return output;
        }

        output.push_str("## Architecture Layers\n\n");

        for layer in &document.layers {
            output.push_str(&format!("### {}\n\n", layer.name));
            output.push_str(&format!("{}\n\n", layer.description));

            if !layer.responsibilities.is_empty() {
                output.push_str("**Responsibilities:**\n\n");
                for responsibility in &layer.responsibilities {
                    output.push_str(&format!("- {}\n", responsibility));
                }
                output.push('\n');
            }

            if !layer.components.is_empty() {
                output.push_str("**Components:**\n\n");
                for component in &layer.components {
                    output.push_str(&format!("- {}\n", component));
                }
                output.push('\n');
            }

            if !layer.dependencies.is_empty() {
                output.push_str("**Dependencies:**\n\n");
                for dep in &layer.dependencies {
                    output.push_str(&format!("- {}\n", dep));
                }
                output.push('\n');
            }
        }

        output
    }

    /// Format components section
    fn format_components(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        if document.components.is_empty() {
            return output;
        }

        output.push_str("## Components\n\n");

        for component in &document.components {
            output.push_str(&format!("### {}\n\n", component.name));
            output.push_str(&format!("**ID**: `{}`\n\n", component.id));
            output.push_str(&format!("**Layer**: {}\n\n", component.layer));
            output.push_str(&format!("{}\n\n", component.description));

            if !component.responsibilities.is_empty() {
                output.push_str("**Responsibilities:**\n\n");
                for responsibility in &component.responsibilities {
                    output.push_str(&format!("- {}\n", responsibility));
                }
                output.push('\n');
            }

            if !component.interfaces.is_empty() {
                output.push_str("**Interfaces:**\n\n");
                for interface in &component.interfaces {
                    output.push_str(&format!(
                        "- **{}** ({})\n",
                        interface.name, interface.protocol
                    ));
                    if !interface.endpoints.is_empty() {
                        output.push_str("  - Endpoints:\n");
                        for endpoint in &interface.endpoints {
                            output.push_str(&format!("    - {}\n", endpoint));
                        }
                    }
                    if !interface.data_formats.is_empty() {
                        output.push_str("  - Data Formats:\n");
                        for format in &interface.data_formats {
                            output.push_str(&format!("    - {}\n", format));
                        }
                    }
                }
                output.push('\n');
            }

            if !component.dependencies.is_empty() {
                output.push_str("**Dependencies:**\n\n");
                for dep in &component.dependencies {
                    output.push_str(&format!("- {}\n", dep));
                }
                output.push('\n');
            }

            if !component.technology_stack.is_empty() {
                output.push_str("**Technology Stack:**\n\n");
                for tech in &component.technology_stack {
                    output.push_str(&format!("- {}\n", tech));
                }
                output.push('\n');
            }
        }

        output
    }

    /// Format integrations section
    fn format_integrations(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        if document.integrations.is_empty() {
            return output;
        }

        output.push_str("## Integrations\n\n");

        for integration in &document.integrations {
            output.push_str(&format!(
                "### {} → {}\n\n",
                integration.from_component, integration.to_component
            ));
            output.push_str(&format!("**Type**: {:?}\n\n", integration.integration_type));
            output.push_str(&format!("{}\n\n", integration.description));

            if !integration.protocols.is_empty() {
                output.push_str("**Protocols:**\n\n");
                for protocol in &integration.protocols {
                    output.push_str(&format!("- {}\n", protocol));
                }
                output.push('\n');
            }
        }

        output
    }

    /// Format deployment section
    fn format_deployment(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        if let Some(deployment) = &document.deployment {
            output.push_str("## Deployment Architecture\n\n");
            output.push_str(&format!("**Strategy**: {:?}\n\n", deployment.strategy));
            output.push_str(&format!("**Scaling**: {:?}\n\n", deployment.scaling));

            if !deployment.infrastructure.is_empty() {
                output.push_str("### Infrastructure Components\n\n");
                for infra in &deployment.infrastructure {
                    output.push_str(&format!("#### {}\n\n", infra.name));
                    output.push_str(&format!("**Type**: {}\n\n", infra.component_type));
                    output.push_str(&format!("{}\n\n", infra.purpose));

                    if !infra.configuration.is_empty() {
                        output.push_str("**Configuration:**\n\n");
                        for (key, value) in &infra.configuration {
                            output.push_str(&format!("- {}: {}\n", key, value));
                        }
                        output.push('\n');
                    }
                }
            }

            output.push_str("### Availability Design\n\n");
            output.push_str(&format!(
                "- **Redundancy**: {}\n",
                deployment.availability.redundancy
            ));
            output.push_str(&format!(
                "- **Failover**: {}\n",
                deployment.availability.failover
            ));
            output.push_str(&format!(
                "- **Disaster Recovery**: {}\n\n",
                deployment.availability.disaster_recovery
            ));
        }

        output
    }

    /// Format quality attributes section
    fn format_quality_attributes(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        if document.quality_attributes.is_empty() {
            return output;
        }

        output.push_str("## Quality Attributes\n\n");

        for quality in &document.quality_attributes {
            output.push_str(&format!("### {}\n\n", quality.name));
            output.push_str(&format!("{}\n\n", quality.description));

            if !quality.tactics.is_empty() {
                output.push_str("**Tactics:**\n\n");
                for tactic in &quality.tactics {
                    output.push_str(&format!("- {}\n", tactic));
                }
                output.push('\n');
            }

            if !quality.metrics.is_empty() {
                output.push_str("**Metrics:**\n\n");
                for metric in &quality.metrics {
                    output.push_str(&format!("- {}\n", metric));
                }
                output.push('\n');
            }
        }

        output
    }
}

impl Default for MarkdownArchitectureWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ArchitectureWriter for MarkdownArchitectureWriter {
    type Error = WriterError;

    async fn write(&self, document: &ArchitectureDocument, path: &Path) -> Result<(), Self::Error> {
        let content = self.format(document);

        fs::write(path, content)
            .await
            .map_err(|e| WriterError::IoError(e.to_string()))?;

        Ok(())
    }

    fn format(&self, document: &ArchitectureDocument) -> String {
        let mut output = String::new();

        output.push_str(&self.format_metadata(document));
        output.push_str(&self.format_overview(document));
        output.push_str(&self.format_layers(document));
        output.push_str(&self.format_components(document));
        output.push_str(&self.format_integrations(document));
        output.push_str(&self.format_deployment(document));
        output.push_str(&self.format_quality_attributes(document));

        output
    }
}

/// Errors that can occur during writing
#[derive(Error, Debug)]
pub enum WriterError {
    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),

    /// Format error
    #[error("Format error: {0}")]
    FormatError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::architecture::{
        ArchitectureMetadata, ArchitecturePattern, Component, Integration, IntegrationType,
        Interface, Layer, Overview,
    };
    use chrono::Utc;
    use tempfile::NamedTempFile;

    fn create_test_document() -> ArchitectureDocument {
        ArchitectureDocument {
            metadata: ArchitectureMetadata {
                title: "Test Architecture".to_string(),
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                model_used: "test".to_string(),
                pattern: ArchitecturePattern::Layered,
                authors: vec!["Test Team".to_string()],
            },
            overview: Overview {
                description: "Test system".to_string(),
                business_goals: vec!["Goal 1".to_string()],
                constraints: vec!["Constraint 1".to_string()],
                assumptions: vec!["Assumption 1".to_string()],
            },
            layers: vec![Layer {
                name: "API Layer".to_string(),
                description: "REST API".to_string(),
                responsibilities: vec!["Handle requests".to_string()],
                components: vec!["api-component".to_string()],
                dependencies: vec![],
            }],
            components: vec![Component {
                id: "api-component".to_string(),
                name: "API Component".to_string(),
                description: "Handles HTTP requests".to_string(),
                layer: "API Layer".to_string(),
                responsibilities: vec!["Process requests".to_string()],
                interfaces: vec![Interface {
                    name: "REST API".to_string(),
                    protocol: "HTTP".to_string(),
                    endpoints: vec!["/api/test".to_string()],
                    data_formats: vec!["JSON".to_string()],
                }],
                dependencies: vec![],
                technology_stack: vec!["Rust".to_string()],
            }],
            integrations: vec![],
            deployment: None,
            quality_attributes: vec![],
        }
    }

    #[test]
    fn test_new() {
        let writer = MarkdownArchitectureWriter::new();
        assert!(std::mem::size_of_val(&writer) == 0); // Zero-sized type
    }

    #[test]
    fn test_format_metadata() {
        let writer = MarkdownArchitectureWriter::new();
        let document = create_test_document();
        let output = writer.format_metadata(&document);

        assert!(output.contains("# Test Architecture"));
        assert!(output.contains("**Version**: 1.0.0"));
        assert!(output.contains("**Pattern**: Layered"));
        assert!(output.contains("**Authors**: Test Team"));
    }

    #[test]
    fn test_format_overview() {
        let writer = MarkdownArchitectureWriter::new();
        let document = create_test_document();
        let output = writer.format_overview(&document);

        assert!(output.contains("## Overview"));
        assert!(output.contains("Test system"));
        assert!(output.contains("Goal 1"));
        assert!(output.contains("Constraint 1"));
        assert!(output.contains("Assumption 1"));
    }

    #[test]
    fn test_format_layers() {
        let writer = MarkdownArchitectureWriter::new();
        let document = create_test_document();
        let output = writer.format_layers(&document);

        assert!(output.contains("## Architecture Layers"));
        assert!(output.contains("### API Layer"));
        assert!(output.contains("REST API"));
        assert!(output.contains("Handle requests"));
    }

    #[test]
    fn test_format_components() {
        let writer = MarkdownArchitectureWriter::new();
        let document = create_test_document();
        let output = writer.format_components(&document);

        assert!(output.contains("## Components"));
        assert!(output.contains("### API Component"));
        assert!(output.contains("**ID**: `api-component`"));
        assert!(output.contains("**Layer**: API Layer"));
        assert!(output.contains("Rust"));
    }

    #[test]
    fn test_format_integrations() {
        let writer = MarkdownArchitectureWriter::new();
        let mut document = create_test_document();
        document.integrations.push(Integration {
            from_component: "comp1".to_string(),
            to_component: "comp2".to_string(),
            integration_type: IntegrationType::Synchronous,
            description: "Test integration".to_string(),
            protocols: vec!["HTTP".to_string()],
        });

        let output = writer.format_integrations(&document);

        assert!(output.contains("## Integrations"));
        assert!(output.contains("comp1 → comp2"));
        assert!(output.contains("Test integration"));
    }

    #[test]
    fn test_format_complete_document() {
        let writer = MarkdownArchitectureWriter::new();
        let document = create_test_document();
        let output = writer.format(&document);

        assert!(output.contains("# Test Architecture"));
        assert!(output.contains("## Overview"));
        assert!(output.contains("## Architecture Layers"));
        assert!(output.contains("## Components"));
    }

    #[tokio::test]
    async fn test_write_to_file() {
        let writer = MarkdownArchitectureWriter::new();
        let document = create_test_document();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = writer.write(&document, path).await;
        assert!(result.is_ok());

        let content = fs::read_to_string(path).await.unwrap();
        assert!(content.contains("# Test Architecture"));
    }

    #[test]
    fn test_default() {
        let writer = MarkdownArchitectureWriter;
        let document = create_test_document();
        let output = writer.format(&document);
        assert!(output.contains("Test Architecture"));
    }

    #[test]
    fn test_format_empty_sections() {
        let writer = MarkdownArchitectureWriter::new();
        let document = ArchitectureDocument {
            metadata: ArchitectureMetadata {
                title: "Minimal".to_string(),
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                model_used: "test".to_string(),
                pattern: ArchitecturePattern::Layered,
                authors: vec![],
            },
            overview: Overview {
                description: "Minimal system".to_string(),
                business_goals: vec![],
                constraints: vec![],
                assumptions: vec![],
            },
            layers: vec![],
            components: vec![],
            integrations: vec![],
            deployment: None,
            quality_attributes: vec![],
        };

        let output = writer.format(&document);
        assert!(output.contains("# Minimal"));
        assert!(output.contains("Minimal system"));
    }
}
