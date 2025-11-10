// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Comprehensive unit tests for architecture domain models and validation
//!
//! This module provides extensive test coverage for the architecture domain layer,
//! including model creation, validation, serialization, and edge cases.

use chrono::Utc;
use std::collections::HashMap;
use xzagentz::domain::architecture::{
    ArchitectureDocument, ArchitectureMetadata, ArchitecturePattern, ArchitectureTemplate,
    ArchitectureValidator, AvailabilityDesign, ComplexityLevel, Component, ComponentTemplate,
    DeploymentArchitecture, DeploymentStrategy, GenerationOptions, InfrastructureComponent,
    Integration, IntegrationType, Interface, Layer, Overview, QualityAttribute, ScalingStrategy,
    TemplateInfo, TemplateStructure, ValidationError,
};

// Architecture Document Tests

#[test]
fn test_architecture_document_creation_with_all_fields() {
    let metadata = ArchitectureMetadata {
        title: "Test Architecture".to_string(),
        version: "1.0.0".to_string(),
        generated_at: Utc::now(),
        model_used: "test-model".to_string(),
        pattern: ArchitecturePattern::Layered,
        authors: vec!["Test Author".to_string()],
    };

    let overview = Overview {
        description: "Test description".to_string(),
        business_goals: vec!["Goal 1".to_string(), "Goal 2".to_string()],
        constraints: vec!["Constraint 1".to_string()],
        assumptions: vec!["Assumption 1".to_string()],
    };

    let layer = Layer {
        name: "Test Layer".to_string(),
        description: "Layer description".to_string(),
        responsibilities: vec!["Responsibility 1".to_string()],
        components: vec!["component-1".to_string()],
        dependencies: vec!["Other Layer".to_string()],
    };

    let component = Component {
        id: "component-1".to_string(),
        name: "Test Component".to_string(),
        description: "Component description".to_string(),
        layer: "Test Layer".to_string(),
        responsibilities: vec!["Do something".to_string()],
        interfaces: vec![],
        dependencies: vec![],
        technology_stack: vec!["Rust".to_string()],
    };

    let document = ArchitectureDocument {
        metadata: metadata.clone(),
        overview: overview.clone(),
        layers: vec![layer.clone()],
        components: vec![component.clone()],
        integrations: vec![],
        deployment: None,
        quality_attributes: vec![],
    };

    assert_eq!(document.metadata.title, "Test Architecture");
    assert_eq!(document.metadata.version, "1.0.0");
    assert_eq!(document.overview.description, "Test description");
    assert_eq!(document.layers.len(), 1);
    assert_eq!(document.components.len(), 1);
}

#[test]
fn test_architecture_document_minimal() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Minimal".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Monolithic,
            authors: vec![],
        },
        overview: Overview {
            description: "Minimal description".to_string(),
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

    assert_eq!(document.metadata.title, "Minimal");
    assert!(document.metadata.model_used.is_empty());
    assert!(document.layers.is_empty());
    assert!(document.components.is_empty());
}

// Architecture Pattern Tests

#[test]
fn test_architecture_pattern_variants() {
    let patterns = vec![
        ArchitecturePattern::Monolithic,
        ArchitecturePattern::Microservices,
        ArchitecturePattern::EventDriven,
        ArchitecturePattern::Layered,
        ArchitecturePattern::Hexagonal,
        ArchitecturePattern::CQRS,
        ArchitecturePattern::Serverless,
        ArchitecturePattern::Custom("MyPattern".to_string()),
    ];

    assert_eq!(patterns.len(), 8);

    for pattern in patterns {
        let _clone = pattern.clone();
    }
}

#[test]
fn test_architecture_pattern_custom() {
    let custom = ArchitecturePattern::Custom("Domain-Driven Design".to_string());
    match custom {
        ArchitecturePattern::Custom(name) => {
            assert_eq!(name, "Domain-Driven Design");
        }
        _ => panic!("Expected Custom pattern"),
    }
}

// Complexity Level Tests

#[test]
fn test_complexity_level_variants() {
    let levels = [
        ComplexityLevel::Simple,
        ComplexityLevel::Moderate,
        ComplexityLevel::Complex,
        ComplexityLevel::Enterprise,
    ];

    assert_eq!(levels.len(), 4);
}

#[test]
fn test_complexity_level_copy() {
    let level = ComplexityLevel::Moderate;
    let copied = level;

    match (level, copied) {
        (ComplexityLevel::Moderate, ComplexityLevel::Moderate) => {}
        _ => panic!("Copy failed"),
    }
}

// Component Tests

#[test]
fn test_component_with_interfaces() {
    let interface = Interface {
        name: "REST API".to_string(),
        protocol: "HTTP/REST".to_string(),
        endpoints: vec!["/api/users".to_string(), "/api/orders".to_string()],
        data_formats: vec!["JSON".to_string()],
    };

    let component = Component {
        id: "api-gateway".to_string(),
        name: "API Gateway".to_string(),
        description: "Entry point for all client requests".to_string(),
        layer: "API Layer".to_string(),
        responsibilities: vec!["Route requests".to_string(), "Authenticate".to_string()],
        interfaces: vec![interface.clone()],
        dependencies: vec!["auth-service".to_string()],
        technology_stack: vec!["Rust".to_string(), "Actix".to_string()],
    };

    assert_eq!(component.interfaces.len(), 1);
    assert_eq!(component.interfaces[0].name, "REST API");
    assert_eq!(component.interfaces[0].endpoints.len(), 2);
    assert_eq!(component.dependencies.len(), 1);
}

#[test]
fn test_component_no_dependencies() {
    let component = Component {
        id: "util".to_string(),
        name: "Utilities".to_string(),
        description: "Helper functions".to_string(),
        layer: "Infrastructure".to_string(),
        responsibilities: vec!["Provide utilities".to_string()],
        interfaces: vec![],
        dependencies: vec![],
        technology_stack: vec!["Rust".to_string()],
    };

    assert!(component.dependencies.is_empty());
    assert!(component.interfaces.is_empty());
}

// Integration Tests

#[test]
fn test_integration_synchronous() {
    let integration = Integration {
        from_component: "web-ui".to_string(),
        to_component: "api-gateway".to_string(),
        integration_type: IntegrationType::Synchronous,
        description: "Direct HTTP calls".to_string(),
        protocols: vec!["HTTP".to_string()],
    };

    assert_eq!(integration.from_component, "web-ui");
    assert_eq!(integration.to_component, "api-gateway");
    match integration.integration_type {
        IntegrationType::Synchronous => {}
        _ => panic!("Expected Synchronous integration"),
    }
}

#[test]
fn test_integration_asynchronous() {
    let integration = Integration {
        from_component: "order-service".to_string(),
        to_component: "notification-service".to_string(),
        integration_type: IntegrationType::Asynchronous,
        description: "Message queue communication".to_string(),
        protocols: vec!["AMQP".to_string(), "RabbitMQ".to_string()],
    };

    match integration.integration_type {
        IntegrationType::Asynchronous => {}
        _ => panic!("Expected Asynchronous integration"),
    }
    assert_eq!(integration.protocols.len(), 2);
}

#[test]
fn test_integration_event_driven() {
    let integration = Integration {
        from_component: "payment-service".to_string(),
        to_component: "event-bus".to_string(),
        integration_type: IntegrationType::EventDriven,
        description: "Publish payment events".to_string(),
        protocols: vec!["Kafka".to_string()],
    };

    match integration.integration_type {
        IntegrationType::EventDriven => {}
        _ => panic!("Expected EventDriven integration"),
    }
}

#[test]
fn test_integration_type_all_variants() {
    let types = [
        IntegrationType::Synchronous,
        IntegrationType::Asynchronous,
        IntegrationType::EventDriven,
        IntegrationType::DataSharing,
        IntegrationType::BatchProcessing,
    ];

    assert_eq!(types.len(), 5);
}

// Deployment Architecture Tests

#[test]
fn test_deployment_architecture_kubernetes() {
    let mut config = HashMap::new();
    config.insert("replicas".to_string(), "3".to_string());
    config.insert("autoscaling".to_string(), "enabled".to_string());

    let infra_component = InfrastructureComponent {
        name: "Kubernetes Cluster".to_string(),
        component_type: "Container Orchestration".to_string(),
        purpose: "Deploy and manage containers".to_string(),
        configuration: config,
    };

    let deployment = DeploymentArchitecture {
        strategy: DeploymentStrategy::Kubernetes,
        infrastructure: vec![infra_component],
        scaling: ScalingStrategy::AutoScaling,
        availability: AvailabilityDesign {
            redundancy: "Multi-zone deployment".to_string(),
            failover: "Automatic pod rescheduling".to_string(),
            disaster_recovery: "Backup to secondary region".to_string(),
        },
    };

    match deployment.strategy {
        DeploymentStrategy::Kubernetes => {}
        _ => panic!("Expected Kubernetes strategy"),
    }

    match deployment.scaling {
        ScalingStrategy::AutoScaling => {}
        _ => panic!("Expected AutoScaling"),
    }

    assert_eq!(deployment.infrastructure.len(), 1);
    assert!(!deployment.availability.redundancy.is_empty());
}

#[test]
fn test_deployment_strategy_all_variants() {
    let strategies = [
        DeploymentStrategy::SingleInstance,
        DeploymentStrategy::LoadBalanced,
        DeploymentStrategy::Containerized,
        DeploymentStrategy::Kubernetes,
        DeploymentStrategy::Serverless,
    ];

    assert_eq!(strategies.len(), 5);
}

#[test]
fn test_scaling_strategy_all_variants() {
    let strategies = [
        ScalingStrategy::Vertical,
        ScalingStrategy::Horizontal,
        ScalingStrategy::AutoScaling,
        ScalingStrategy::Hybrid,
    ];

    assert_eq!(strategies.len(), 4);
}

// Quality Attribute Tests

#[test]
fn test_quality_attribute_performance() {
    let qa = QualityAttribute {
        name: "Performance".to_string(),
        description: "System must handle 10000 requests per second".to_string(),
        tactics: vec![
            "Caching".to_string(),
            "Load balancing".to_string(),
            "Database indexing".to_string(),
        ],
        metrics: vec![
            "Response time < 100ms".to_string(),
            "Throughput > 10k rps".to_string(),
        ],
    };

    assert_eq!(qa.name, "Performance");
    assert_eq!(qa.tactics.len(), 3);
    assert_eq!(qa.metrics.len(), 2);
}

#[test]
fn test_quality_attribute_security() {
    let qa = QualityAttribute {
        name: "Security".to_string(),
        description: "Protect user data and prevent unauthorized access".to_string(),
        tactics: vec![
            "Encryption at rest".to_string(),
            "JWT authentication".to_string(),
            "Rate limiting".to_string(),
            "Input validation".to_string(),
        ],
        metrics: vec![
            "Zero data breaches".to_string(),
            "100% encrypted connections".to_string(),
        ],
    };

    assert_eq!(qa.name, "Security");
    assert_eq!(qa.tactics.len(), 4);
}

// Generation Options Tests

#[test]
fn test_generation_options_default() {
    let options = GenerationOptions {
        pattern: None,
        complexity: ComplexityLevel::Simple,
        include_deployment: false,
        include_quality_attributes: false,
        max_components: None,
        technology_preferences: vec![],
    };

    assert!(options.pattern.is_none());
    assert!(options.max_components.is_none());
    assert!(options.technology_preferences.is_empty());
}

#[test]
fn test_generation_options_full() {
    let options = GenerationOptions {
        pattern: Some(ArchitecturePattern::Microservices),
        complexity: ComplexityLevel::Enterprise,
        include_deployment: true,
        include_quality_attributes: true,
        max_components: Some(20),
        technology_preferences: vec![
            "Rust".to_string(),
            "PostgreSQL".to_string(),
            "Kubernetes".to_string(),
        ],
    };

    assert!(options.pattern.is_some());
    assert_eq!(options.max_components, Some(20));
    assert_eq!(options.technology_preferences.len(), 3);
    assert!(options.include_deployment);
    assert!(options.include_quality_attributes);
}

// Template Tests

#[test]
fn test_template_info() {
    let info = TemplateInfo {
        name: "microservices".to_string(),
        pattern: ArchitecturePattern::Microservices,
        description: "Microservices architecture template".to_string(),
        use_cases: vec![
            "Large-scale web applications".to_string(),
            "Cloud-native systems".to_string(),
        ],
    };

    assert_eq!(info.name, "microservices");
    assert_eq!(info.use_cases.len(), 2);
}

#[test]
fn test_template_structure() {
    let structure = TemplateStructure {
        layers: vec![
            "API Gateway".to_string(),
            "Service Layer".to_string(),
            "Data Layer".to_string(),
        ],
        integration_patterns: vec!["REST".to_string(), "Message Queue".to_string()],
        required_components: vec!["auth-service".to_string(), "api-gateway".to_string()],
    };

    assert_eq!(structure.layers.len(), 3);
    assert_eq!(structure.integration_patterns.len(), 2);
    assert_eq!(structure.required_components.len(), 2);
}

#[test]
fn test_component_template() {
    let template = ComponentTemplate {
        name: "API Gateway".to_string(),
        layer: "Presentation".to_string(),
        role: "Entry point for all client requests".to_string(),
        typical_technologies: vec![
            "Kong".to_string(),
            "Nginx".to_string(),
            "Traefik".to_string(),
        ],
    };

    assert_eq!(template.name, "API Gateway");
    assert_eq!(template.typical_technologies.len(), 3);
}

#[test]
fn test_architecture_template_complete() {
    let info = TemplateInfo {
        name: "layered".to_string(),
        pattern: ArchitecturePattern::Layered,
        description: "Traditional layered architecture".to_string(),
        use_cases: vec!["Enterprise applications".to_string()],
    };

    let structure = TemplateStructure {
        layers: vec![
            "Presentation".to_string(),
            "Business Logic".to_string(),
            "Data Access".to_string(),
        ],
        integration_patterns: vec!["Direct method calls".to_string()],
        required_components: vec![
            "ui".to_string(),
            "service".to_string(),
            "repository".to_string(),
        ],
    };

    let component_template = ComponentTemplate {
        name: "Service Layer".to_string(),
        layer: "Business Logic".to_string(),
        role: "Implement business rules".to_string(),
        typical_technologies: vec!["Rust".to_string()],
    };

    let template = ArchitectureTemplate {
        info,
        structure,
        default_components: vec![component_template],
        customization_points: vec![
            "Technology stack".to_string(),
            "Database choice".to_string(),
        ],
    };

    assert_eq!(template.info.name, "layered");
    assert_eq!(template.structure.layers.len(), 3);
    assert_eq!(template.default_components.len(), 1);
    assert_eq!(template.customization_points.len(), 2);
}

// Validation Tests

#[test]
fn test_validation_empty_title_fails() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec![],
        },
        overview: Overview {
            description: "Test".to_string(),
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

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_err());
    match result {
        Err(ValidationError::EmptyTitle) => {}
        _ => panic!("Expected EmptyTitle error"),
    }
}

#[test]
fn test_validation_empty_description_fails() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Valid Title".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec![],
        },
        overview: Overview {
            description: "".to_string(),
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

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_err());
    match result {
        Err(ValidationError::EmptyDescription) => {}
        _ => panic!("Expected EmptyDescription error"),
    }
}

#[test]
fn test_validation_no_layers_fails() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Valid Title".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec![],
        },
        overview: Overview {
            description: "Valid description".to_string(),
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

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_err());
    match result {
        Err(ValidationError::NoLayers) => {}
        _ => panic!("Expected NoLayers error"),
    }
}

#[test]
fn test_validation_no_components_fails() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Valid Title".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec![],
        },
        overview: Overview {
            description: "Valid description".to_string(),
            business_goals: vec![],
            constraints: vec![],
            assumptions: vec![],
        },
        layers: vec![Layer {
            name: "Test Layer".to_string(),
            description: "Description".to_string(),
            responsibilities: vec![],
            components: vec![],
            dependencies: vec![],
        }],
        components: vec![],
        integrations: vec![],
        deployment: None,
        quality_attributes: vec![],
    };

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_err());
    match result {
        Err(ValidationError::NoComponents) => {}
        _ => panic!("Expected NoComponents error"),
    }
}

#[test]
fn test_validation_empty_component_name_fails() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Valid Title".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec![],
        },
        overview: Overview {
            description: "Valid description".to_string(),
            business_goals: vec![],
            constraints: vec![],
            assumptions: vec![],
        },
        layers: vec![Layer {
            name: "Test Layer".to_string(),
            description: "Description".to_string(),
            responsibilities: vec![],
            components: vec![],
            dependencies: vec![],
        }],
        components: vec![Component {
            id: "comp-1".to_string(),
            name: "".to_string(),
            description: "Description".to_string(),
            layer: "Test Layer".to_string(),
            responsibilities: vec![],
            interfaces: vec![],
            dependencies: vec![],
            technology_stack: vec![],
        }],
        integrations: vec![],
        deployment: None,
        quality_attributes: vec![],
    };

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_err());
    match result {
        Err(ValidationError::EmptyComponentName { .. }) => {}
        _ => panic!("Expected EmptyComponentName error"),
    }
}

#[test]
fn test_validation_invalid_component_reference_fails() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Valid Title".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec![],
        },
        overview: Overview {
            description: "Valid description".to_string(),
            business_goals: vec![],
            constraints: vec![],
            assumptions: vec![],
        },
        layers: vec![Layer {
            name: "Test Layer".to_string(),
            description: "Description".to_string(),
            responsibilities: vec![],
            components: vec![],
            dependencies: vec![],
        }],
        components: vec![Component {
            id: "comp-1".to_string(),
            name: "Component 1".to_string(),
            description: "Description".to_string(),
            layer: "Test Layer".to_string(),
            responsibilities: vec![],
            interfaces: vec![],
            dependencies: vec![],
            technology_stack: vec![],
        }],
        integrations: vec![Integration {
            from_component: "comp-1".to_string(),
            to_component: "nonexistent".to_string(),
            integration_type: IntegrationType::Synchronous,
            description: "Test".to_string(),
            protocols: vec![],
        }],
        deployment: None,
        quality_attributes: vec![],
    };

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_err());
    match result {
        Err(ValidationError::InvalidComponentReference {
            component_id,
            referenced_id,
        }) => {
            assert_eq!(component_id, "integration");
            assert_eq!(referenced_id, "nonexistent");
        }
        _ => panic!("Expected InvalidComponentReference error"),
    }
}

#[test]
fn test_validation_valid_document_succeeds() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Valid Architecture".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "test".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec!["Author".to_string()],
        },
        overview: Overview {
            description: "A valid architecture document".to_string(),
            business_goals: vec!["Goal 1".to_string()],
            constraints: vec!["Constraint 1".to_string()],
            assumptions: vec!["Assumption 1".to_string()],
        },
        layers: vec![Layer {
            name: "Application Layer".to_string(),
            description: "Business logic layer".to_string(),
            responsibilities: vec!["Process requests".to_string()],
            components: vec!["app-service".to_string()],
            dependencies: vec![],
        }],
        components: vec![Component {
            id: "app-service".to_string(),
            name: "Application Service".to_string(),
            description: "Main application service".to_string(),
            layer: "Application Layer".to_string(),
            responsibilities: vec!["Handle business logic".to_string()],
            interfaces: vec![],
            dependencies: vec![],
            technology_stack: vec!["Rust".to_string()],
        }],
        integrations: vec![],
        deployment: None,
        quality_attributes: vec![],
    };

    let validator = ArchitectureValidator;
    let result = validator.validate(&document);

    assert!(result.is_ok());
}

#[test]
fn test_validation_error_display() {
    let errors = vec![
        ValidationError::EmptyTitle,
        ValidationError::EmptyDescription,
        ValidationError::NoLayers,
        ValidationError::NoComponents,
        ValidationError::EmptyComponentName {
            component_id: "comp-1".to_string(),
        },
        ValidationError::InvalidComponentReference {
            component_id: "comp-1".to_string(),
            referenced_id: "comp-2".to_string(),
        },
    ];

    for error in errors {
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }
}

// Edge Cases and Boundary Tests

#[test]
fn test_very_long_title() {
    let long_title = "A".repeat(1000);
    let metadata = ArchitectureMetadata {
        title: long_title.clone(),
        version: "1.0.0".to_string(),
        generated_at: Utc::now(),
        model_used: "".to_string(),
        pattern: ArchitecturePattern::Layered,
        authors: vec![],
    };

    assert_eq!(metadata.title.len(), 1000);
}

#[test]
fn test_many_layers() {
    let layers: Vec<Layer> = (0..100)
        .map(|i| Layer {
            name: format!("Layer {}", i),
            description: format!("Description {}", i),
            responsibilities: vec![],
            components: vec![],
            dependencies: vec![],
        })
        .collect();

    assert_eq!(layers.len(), 100);
}

#[test]
fn test_many_components() {
    let components: Vec<Component> = (0..200)
        .map(|i| Component {
            id: format!("comp-{}", i),
            name: format!("Component {}", i),
            description: format!("Description {}", i),
            layer: "Test".to_string(),
            responsibilities: vec![],
            interfaces: vec![],
            dependencies: vec![],
            technology_stack: vec![],
        })
        .collect();

    assert_eq!(components.len(), 200);
}

#[test]
fn test_circular_dependency_representation() {
    let comp1 = Component {
        id: "comp-1".to_string(),
        name: "Component 1".to_string(),
        description: "First component".to_string(),
        layer: "Test".to_string(),
        responsibilities: vec![],
        interfaces: vec![],
        dependencies: vec!["comp-2".to_string()],
        technology_stack: vec![],
    };

    let comp2 = Component {
        id: "comp-2".to_string(),
        name: "Component 2".to_string(),
        description: "Second component".to_string(),
        layer: "Test".to_string(),
        responsibilities: vec![],
        interfaces: vec![],
        dependencies: vec!["comp-1".to_string()],
        technology_stack: vec![],
    };

    assert!(comp1.dependencies.contains(&"comp-2".to_string()));
    assert!(comp2.dependencies.contains(&"comp-1".to_string()));
}

#[test]
fn test_unicode_in_fields() {
    let document = ArchitectureDocument {
        metadata: ArchitectureMetadata {
            title: "Architecture 架构 🏗️".to_string(),
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: "".to_string(),
            pattern: ArchitecturePattern::Layered,
            authors: vec!["作者".to_string()],
        },
        overview: Overview {
            description: "Description with émojis and spëcial characters".to_string(),
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

    assert!(document.metadata.title.contains("架构"));
    assert!(document.metadata.authors[0].contains("作者"));
}
