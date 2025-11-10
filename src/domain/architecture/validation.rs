// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Validation logic for architecture documents
//!
//! This module provides validation for architecture documents to ensure they are
//! complete, consistent, and follow best practices.
//!
//! # Validation Rules
//!
//! - Metadata must have non-empty title and description
//! - Document must have at least one layer
//! - Document must have at least one component
//! - All component references must be valid
//! - Component dependencies must reference existing components
//! - Integration endpoints must reference existing components
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::architecture::{
//!     ArchitectureValidator, ArchitectureDocument, ArchitectureMetadata,
//!     Overview, ArchitecturePattern,
//! };
//! use chrono::Utc;
//!
//! let document = ArchitectureDocument {
//!     metadata: ArchitectureMetadata {
//!         title: "Test Architecture".to_string(),
//!         version: "1.0.0".to_string(),
//!         generated_at: Utc::now(),
//!         model_used: "test".to_string(),
//!         pattern: ArchitecturePattern::Layered,
//!         authors: vec!["Team".to_string()],
//!     },
//!     overview: Overview {
//!         description: "A test system".to_string(),
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
//! let validator = ArchitectureValidator;
//! let result = validator.validate(&document);
//! assert!(result.is_err()); // Fails because no layers or components
//! ```

use crate::domain::architecture::models::ArchitectureDocument;
use std::collections::HashSet;
use thiserror::Error;

/// Validator for architecture documents
///
/// Provides comprehensive validation of architecture documents to ensure
/// completeness and consistency.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::ArchitectureValidator;
///
/// let validator = ArchitectureValidator;
/// // Use validator.validate(&document) to validate documents
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ArchitectureValidator;

impl ArchitectureValidator {
    /// Validate a complete architecture document
    ///
    /// # Arguments
    ///
    /// * `document` - Architecture document to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if validation passes, or `ValidationError` if validation fails
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` for various validation failures:
    /// - Empty title or description
    /// - No layers or components
    /// - Invalid component references
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::domain::architecture::{
    ///     ArchitectureValidator, ArchitectureDocument, ArchitectureMetadata,
    ///     Overview, Layer, Component, ArchitecturePattern,
    /// };
    /// use chrono::Utc;
    ///
    /// let document = ArchitectureDocument {
    ///     metadata: ArchitectureMetadata {
    ///         title: "Valid Architecture".to_string(),
    ///         version: "1.0.0".to_string(),
    ///         generated_at: Utc::now(),
    ///         model_used: "test".to_string(),
    ///         pattern: ArchitecturePattern::Layered,
    ///         authors: vec!["Team".to_string()],
    ///     },
    ///     overview: Overview {
    ///         description: "System description".to_string(),
    ///         business_goals: vec![],
    ///         constraints: vec![],
    ///         assumptions: vec![],
    ///     },
    ///     layers: vec![Layer {
    ///         name: "API".to_string(),
    ///         description: "API layer".to_string(),
    ///         responsibilities: vec![],
    ///         components: vec!["api-component".to_string()],
    ///         dependencies: vec![],
    ///     }],
    ///     components: vec![Component {
    ///         id: "api-component".to_string(),
    ///         name: "API Component".to_string(),
    ///         description: "API".to_string(),
    ///         layer: "API".to_string(),
    ///         responsibilities: vec![],
    ///         interfaces: vec![],
    ///         dependencies: vec![],
    ///         technology_stack: vec![],
    ///     }],
    ///     integrations: vec![],
    ///     deployment: None,
    ///     quality_attributes: vec![],
    /// };
    ///
    /// let validator = ArchitectureValidator;
    /// assert!(validator.validate(&document).is_ok());
    /// ```
    pub fn validate(&self, document: &ArchitectureDocument) -> Result<(), ValidationError> {
        self.validate_metadata(&document.metadata)?;
        self.validate_overview(&document.overview)?;
        self.validate_layers(&document.layers)?;
        self.validate_components(&document.components)?;
        self.validate_integrations(document)?;
        Ok(())
    }

    /// Validate metadata section
    fn validate_metadata(
        &self,
        metadata: &crate::domain::architecture::models::ArchitectureMetadata,
    ) -> Result<(), ValidationError> {
        if metadata.title.trim().is_empty() {
            return Err(ValidationError::EmptyTitle);
        }
        Ok(())
    }

    /// Validate overview section
    fn validate_overview(
        &self,
        overview: &crate::domain::architecture::models::Overview,
    ) -> Result<(), ValidationError> {
        if overview.description.trim().is_empty() {
            return Err(ValidationError::EmptyDescription);
        }
        Ok(())
    }

    /// Validate layers section
    fn validate_layers(
        &self,
        layers: &[crate::domain::architecture::models::Layer],
    ) -> Result<(), ValidationError> {
        if layers.is_empty() {
            return Err(ValidationError::NoLayers);
        }
        Ok(())
    }

    /// Validate components section
    fn validate_components(
        &self,
        components: &[crate::domain::architecture::models::Component],
    ) -> Result<(), ValidationError> {
        if components.is_empty() {
            return Err(ValidationError::NoComponents);
        }

        // Collect all component IDs
        let component_ids: HashSet<&String> = components.iter().map(|c| &c.id).collect();

        for component in components {
            if component.name.trim().is_empty() {
                return Err(ValidationError::EmptyComponentName {
                    component_id: component.id.clone(),
                });
            }

            // Validate dependencies reference existing components
            for dep in &component.dependencies {
                if !component_ids.contains(dep) {
                    return Err(ValidationError::InvalidComponentReference {
                        component_id: component.id.clone(),
                        referenced_id: dep.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validate integrations section
    fn validate_integrations(
        &self,
        document: &ArchitectureDocument,
    ) -> Result<(), ValidationError> {
        let component_ids: HashSet<&String> = document.components.iter().map(|c| &c.id).collect();

        for integration in &document.integrations {
            if !component_ids.contains(&integration.from_component) {
                return Err(ValidationError::InvalidComponentReference {
                    component_id: "integration".to_string(),
                    referenced_id: integration.from_component.clone(),
                });
            }

            if !component_ids.contains(&integration.to_component) {
                return Err(ValidationError::InvalidComponentReference {
                    component_id: "integration".to_string(),
                    referenced_id: integration.to_component.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Validation errors for architecture documents
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::ValidationError;
///
/// let error = ValidationError::EmptyTitle;
/// assert_eq!(error.to_string(), "Document title cannot be empty");
/// ```
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Document title is empty
    #[error("Document title cannot be empty")]
    EmptyTitle,

    /// Overview description is empty
    #[error("Overview description cannot be empty")]
    EmptyDescription,

    /// No layers defined
    #[error("Document must have at least one layer")]
    NoLayers,

    /// No components defined
    #[error("Document must have at least one component")]
    NoComponents,

    /// Component has empty name
    #[error("Component '{component_id}' has empty name")]
    EmptyComponentName {
        /// ID of the component with empty name
        component_id: String,
    },

    /// Invalid component reference
    #[error("Component '{component_id}' references non-existent component '{referenced_id}'")]
    InvalidComponentReference {
        /// ID of the component making the reference
        component_id: String,
        /// ID of the referenced component that does not exist
        referenced_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::architecture::models::{
        ArchitectureDocument, ArchitectureMetadata, ArchitecturePattern, Component, Integration,
        IntegrationType, Layer, Overview,
    };
    use chrono::Utc;

    fn create_valid_document() -> ArchitectureDocument {
        ArchitectureDocument {
            metadata: ArchitectureMetadata {
                title: "Test Architecture".to_string(),
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                model_used: "test".to_string(),
                pattern: ArchitecturePattern::Layered,
                authors: vec!["Test".to_string()],
            },
            overview: Overview {
                description: "A valid test system".to_string(),
                business_goals: vec![],
                constraints: vec![],
                assumptions: vec![],
            },
            layers: vec![Layer {
                name: "API".to_string(),
                description: "API layer".to_string(),
                responsibilities: vec![],
                components: vec!["comp1".to_string()],
                dependencies: vec![],
            }],
            components: vec![Component {
                id: "comp1".to_string(),
                name: "Component 1".to_string(),
                description: "Test component".to_string(),
                layer: "api".to_string(),
                responsibilities: vec![],
                interfaces: vec![],
                dependencies: vec![],
                technology_stack: vec![],
            }],
            integrations: vec![],
            deployment: None,
            quality_attributes: vec![],
        }
    }

    #[test]
    fn test_validate_valid_document() {
        let document = create_valid_document();
        let validator = ArchitectureValidator;
        assert!(validator.validate(&document).is_ok());
    }

    #[test]
    fn test_validate_empty_title() {
        let mut document = create_valid_document();
        document.metadata.title = "".to_string();
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(result, Err(ValidationError::EmptyTitle)));
    }

    #[test]
    fn test_validate_empty_description() {
        let mut document = create_valid_document();
        document.overview.description = "   ".to_string();
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(result, Err(ValidationError::EmptyDescription)));
    }

    #[test]
    fn test_validate_no_layers() {
        let mut document = create_valid_document();
        document.layers = vec![];
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(result, Err(ValidationError::NoLayers)));
    }

    #[test]
    fn test_validate_no_components() {
        let mut document = create_valid_document();
        document.components = vec![];
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(result, Err(ValidationError::NoComponents)));
    }

    #[test]
    fn test_validate_empty_component_name() {
        let mut document = create_valid_document();
        document.components[0].name = "".to_string();
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(
            result,
            Err(ValidationError::EmptyComponentName { .. })
        ));
    }

    #[test]
    fn test_validate_invalid_component_dependency() {
        let mut document = create_valid_document();
        document.components[0].dependencies = vec!["nonexistent".to_string()];
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidComponentReference { .. })
        ));
    }

    #[test]
    fn test_validate_valid_component_dependency() {
        let mut document = create_valid_document();
        document.components.push(Component {
            id: "comp2".to_string(),
            name: "Component 2".to_string(),
            description: "Depends on comp1".to_string(),
            layer: "api".to_string(),
            responsibilities: vec![],
            interfaces: vec![],
            dependencies: vec!["comp1".to_string()],
            technology_stack: vec![],
        });
        let validator = ArchitectureValidator;
        assert!(validator.validate(&document).is_ok());
    }

    #[test]
    fn test_validate_invalid_integration_from() {
        let mut document = create_valid_document();
        document.integrations.push(Integration {
            from_component: "nonexistent".to_string(),
            to_component: "comp1".to_string(),
            integration_type: IntegrationType::Synchronous,
            description: "Test".to_string(),
            protocols: vec![],
        });
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidComponentReference { .. })
        ));
    }

    #[test]
    fn test_validate_invalid_integration_to() {
        let mut document = create_valid_document();
        document.integrations.push(Integration {
            from_component: "comp1".to_string(),
            to_component: "nonexistent".to_string(),
            integration_type: IntegrationType::Synchronous,
            description: "Test".to_string(),
            protocols: vec![],
        });
        let validator = ArchitectureValidator;
        let result = validator.validate(&document);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidComponentReference { .. })
        ));
    }

    #[test]
    fn test_validate_valid_integration() {
        let mut document = create_valid_document();
        document.components.push(Component {
            id: "comp2".to_string(),
            name: "Component 2".to_string(),
            description: "Second component".to_string(),
            layer: "api".to_string(),
            responsibilities: vec![],
            interfaces: vec![],
            dependencies: vec![],
            technology_stack: vec![],
        });
        document.integrations.push(Integration {
            from_component: "comp1".to_string(),
            to_component: "comp2".to_string(),
            integration_type: IntegrationType::Synchronous,
            description: "Test integration".to_string(),
            protocols: vec!["HTTP".to_string()],
        });
        let validator = ArchitectureValidator;
        assert!(validator.validate(&document).is_ok());
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::EmptyTitle;
        assert_eq!(error.to_string(), "Document title cannot be empty");

        let error = ValidationError::InvalidComponentReference {
            component_id: "comp1".to_string(),
            referenced_id: "comp2".to_string(),
        };
        assert!(error
            .to_string()
            .contains("references non-existent component"));
    }
}
