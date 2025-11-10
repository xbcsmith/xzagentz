// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Architecture domain module
//!
//! This module contains the core domain models, traits, and validation logic
//! for software architecture documentation and generation.
//!
//! # Overview
//!
//! The architecture domain provides:
//!
//! - **Models**: Complete data structures for representing software architectures
//! - **Traits**: Contracts for architecture generation, template management, and writing
//! - **Validation**: Comprehensive validation rules for architecture documents
//!
//! # Architecture
//!
//! This module follows Domain-Driven Design principles:
//!
//! - **Pure Domain Logic**: No infrastructure dependencies
//! - **Trait-Based Contracts**: Infrastructure implements domain traits
//! - **Rich Domain Models**: Models contain business rules and validation
//! - **Testability**: All domain logic is easily testable
//!
//! # Examples
//!
//! ## Creating an Architecture Document
//!
//! ```rust
//! use xzagentz::domain::architecture::{
//!     ArchitectureDocument, ArchitectureMetadata, ArchitecturePattern,
//!     Overview, Layer, Component,
//! };
//! use chrono::Utc;
//!
//! let document = ArchitectureDocument {
//!     metadata: ArchitectureMetadata {
//!         title: "E-commerce Platform".to_string(),
//!         version: "1.0.0".to_string(),
//!         generated_at: Utc::now(),
//!         model_used: "llama3.2:3b".to_string(),
//!         pattern: ArchitecturePattern::Microservices,
//!         authors: vec!["Architecture Team".to_string()],
//!     },
//!     overview: Overview {
//!         description: "Cloud-native e-commerce platform".to_string(),
//!         business_goals: vec!["Handle 10k orders/day".to_string()],
//!         constraints: vec!["Budget: $50k/month".to_string()],
//!         assumptions: vec!["AWS deployment".to_string()],
//!     },
//!     layers: vec![],
//!     components: vec![],
//!     integrations: vec![],
//!     deployment: None,
//!     quality_attributes: vec![],
//! };
//! ```
//!
//! ## Validating an Architecture Document
//!
//! ```rust
//! use xzagentz::domain::architecture::{
//!     ArchitectureValidator, ArchitectureDocument, ArchitectureMetadata,
//!     Overview, Layer, Component, ArchitecturePattern,
//! };
//! use chrono::Utc;
//!
//! let document = ArchitectureDocument {
//!     metadata: ArchitectureMetadata {
//!         title: "Test System".to_string(),
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
//!     layers: vec![Layer {
//!         name: "API Layer".to_string(),
//!         description: "REST API".to_string(),
//!         responsibilities: vec![],
//!         components: vec!["api".to_string()],
//!         dependencies: vec![],
//!     }],
//!     components: vec![Component {
//!         id: "api".to_string(),
//!         name: "API Component".to_string(),
//!         description: "REST API".to_string(),
//!         layer: "API Layer".to_string(),
//!         responsibilities: vec![],
//!         interfaces: vec![],
//!         dependencies: vec![],
//!         technology_stack: vec![],
//!     }],
//!     integrations: vec![],
//!     deployment: None,
//!     quality_attributes: vec![],
//! };
//!
//! let validator = ArchitectureValidator;
//! assert!(validator.validate(&document).is_ok());
//! ```
//!
//! ## Using Traits for LLM Integration
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::{
//!     ArchitectureGenerator, GenerationOptions, ComplexityLevel,
//!     ArchitecturePattern,
//! };
//!
//! async fn generate_architecture<G: ArchitectureGenerator>(
//!     generator: &G,
//! ) -> Result<(), Box<dyn std::error::Error>> {
//!     let requirements = "Build a scalable e-commerce platform with user management, \
//!                        product catalog, shopping cart, and payment processing";
//!
//!     let options = GenerationOptions {
//!         pattern: Some(ArchitecturePattern::Microservices),
//!         complexity: ComplexityLevel::Moderate,
//!         include_deployment: true,
//!         include_quality_attributes: true,
//!         max_components: Some(10),
//!         technology_preferences: vec!["Rust".to_string(), "PostgreSQL".to_string()],
//!     };
//!
//!     let document = generator.generate(requirements, &options).await?;
//!     println!("Generated: {}", document.metadata.title);
//!     Ok(())
//! }
//! ```

pub mod models;
pub mod traits;
pub mod validation;

// Re-export main types for convenience
pub use models::{
    ArchitectureDocument, ArchitectureMetadata, ArchitecturePattern, AvailabilityDesign, Component,
    DeploymentArchitecture, DeploymentStrategy, InfrastructureComponent, Integration,
    IntegrationType, Interface, Layer, Overview, QualityAttribute, ScalingStrategy,
};

pub use traits::{
    ArchitectureGenerator, ArchitectureTemplate, ArchitectureWriter, ComplexityLevel,
    ComponentTemplate, GenerationOptions, TemplateInfo, TemplateRepository, TemplateStructure,
};

pub use validation::{ArchitectureValidator, ValidationError};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_module_exports() {
        // Test that all main types are accessible
        let _pattern = ArchitecturePattern::Microservices;
        let _complexity = ComplexityLevel::Moderate;
        let _validator = ArchitectureValidator;
    }

    #[test]
    fn test_create_and_validate_document() {
        let document = ArchitectureDocument {
            metadata: ArchitectureMetadata {
                title: "Test Architecture".to_string(),
                version: "1.0.0".to_string(),
                generated_at: Utc::now(),
                model_used: "test".to_string(),
                pattern: ArchitecturePattern::Layered,
                authors: vec!["Test Team".to_string()],
            },
            overview: Overview {
                description: "Test system for validation".to_string(),
                business_goals: vec!["Goal 1".to_string()],
                constraints: vec![],
                assumptions: vec![],
            },
            layers: vec![Layer {
                name: "Test Layer".to_string(),
                description: "A test layer".to_string(),
                responsibilities: vec!["Handle requests".to_string()],
                components: vec!["test-component".to_string()],
                dependencies: vec![],
            }],
            components: vec![Component {
                id: "test-component".to_string(),
                name: "Test Component".to_string(),
                description: "A test component".to_string(),
                layer: "Test Layer".to_string(),
                responsibilities: vec!["Process data".to_string()],
                interfaces: vec![],
                dependencies: vec![],
                technology_stack: vec!["Rust".to_string()],
            }],
            integrations: vec![],
            deployment: None,
            quality_attributes: vec![],
        };

        let validator = ArchitectureValidator;
        assert!(validator.validate(&document).is_ok());
    }

    #[test]
    fn test_generation_options_default() {
        let options = GenerationOptions::default();
        assert_eq!(options.complexity, ComplexityLevel::Moderate);
        assert!(options.include_deployment);
        assert!(options.include_quality_attributes);
    }
}
