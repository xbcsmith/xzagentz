//! Core domain models for software architecture documentation
//!
//! This module defines the complete domain model for representing software architecture
//! documents, including metadata, layers, components, integrations, and deployment details.
//!
//! # Architecture
//!
//! The domain models are organized hierarchically:
//!
//! - [`ArchitectureDocument`]: Top-level container for complete architecture
//! - [`Layer`]: Architectural layers (e.g., API, Application, Domain)
//! - [`Component`]: Individual components within layers
//! - [`Integration`]: Connections between components
//! - [`DeploymentArchitecture`]: Infrastructure and deployment strategy
//! - [`QualityAttribute`]: Non-functional requirements and tactics
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::domain::architecture::{
//!     ArchitectureDocument, ArchitectureMetadata, ArchitecturePattern,
//!     Overview, Layer, Component,
//! };
//! use chrono::Utc;
//!
//! // Create architecture metadata
//! let metadata = ArchitectureMetadata {
//!     title: "E-commerce Platform".to_string(),
//!     version: "1.0.0".to_string(),
//!     generated_at: Utc::now(),
//!     model_used: "llama3.2:3b".to_string(),
//!     pattern: ArchitecturePattern::Microservices,
//!     authors: vec!["Architecture Team".to_string()],
//! };
//!
//! // Create overview
//! let overview = Overview {
//!     description: "Scalable e-commerce platform".to_string(),
//!     business_goals: vec!["Handle 10k requests/sec".to_string()],
//!     constraints: vec!["Budget: $100k/year".to_string()],
//!     assumptions: vec!["AWS deployment".to_string()],
//! };
//!
//! // Create a simple architecture document
//! let document = ArchitectureDocument {
//!     metadata,
//!     overview,
//!     layers: vec![],
//!     components: vec![],
//!     integrations: vec![],
//!     deployment: None,
//!     quality_attributes: vec![],
//! };
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Complete software architecture document
///
/// Represents a full architecture specification including all layers, components,
/// integrations, deployment strategy, and quality attributes.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{ArchitectureDocument, ArchitectureMetadata, Overview, ArchitecturePattern};
/// use chrono::Utc;
///
/// let doc = ArchitectureDocument {
///     metadata: ArchitectureMetadata {
///         title: "My System".to_string(),
///         version: "1.0.0".to_string(),
///         generated_at: Utc::now(),
///         model_used: "llama3.2".to_string(),
///         pattern: ArchitecturePattern::Layered,
///         authors: vec!["Team".to_string()],
///     },
///     overview: Overview {
///         description: "System overview".to_string(),
///         business_goals: vec![],
///         constraints: vec![],
///         assumptions: vec![],
///     },
///     layers: vec![],
///     components: vec![],
///     integrations: vec![],
///     deployment: None,
///     quality_attributes: vec![],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureDocument {
    /// Document metadata (title, version, generation info)
    pub metadata: ArchitectureMetadata,
    /// High-level overview and context
    pub overview: Overview,
    /// Architectural layers (e.g., API, Application, Domain)
    pub layers: Vec<Layer>,
    /// All components across all layers
    pub components: Vec<Component>,
    /// Integrations between components
    pub integrations: Vec<Integration>,
    /// Deployment architecture and infrastructure
    pub deployment: Option<DeploymentArchitecture>,
    /// Quality attributes and tactics
    pub quality_attributes: Vec<QualityAttribute>,
}

/// Metadata about the architecture document
///
/// Contains generation details, versioning, and authorship information.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{ArchitectureMetadata, ArchitecturePattern};
/// use chrono::Utc;
///
/// let metadata = ArchitectureMetadata {
///     title: "Payment Service Architecture".to_string(),
///     version: "2.1.0".to_string(),
///     generated_at: Utc::now(),
///     model_used: "llama3.2:3b".to_string(),
///     pattern: ArchitecturePattern::Microservices,
///     authors: vec!["Alice".to_string(), "Bob".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureMetadata {
    /// Document title
    pub title: String,
    /// Semantic version (e.g., "1.0.0")
    pub version: String,
    /// Timestamp when document was generated
    pub generated_at: DateTime<Utc>,
    /// LLM model used for generation (e.g., "llama3.2:3b")
    pub model_used: String,
    /// Primary architecture pattern
    pub pattern: ArchitecturePattern,
    /// Document authors
    pub authors: Vec<String>,
}

/// High-level architecture overview
///
/// Provides context, business drivers, constraints, and assumptions.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::Overview;
///
/// let overview = Overview {
///     description: "Cloud-native microservices platform for order processing".to_string(),
///     business_goals: vec![
///         "Process 100k orders/day".to_string(),
///         "99.9% uptime".to_string(),
///     ],
///     constraints: vec![
///         "Must use AWS".to_string(),
///         "Budget: $50k/month".to_string(),
///     ],
///     assumptions: vec![
///         "Peak traffic is 10x average".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Overview {
    /// System description
    pub description: String,
    /// Business goals and drivers
    pub business_goals: Vec<String>,
    /// Technical and business constraints
    pub constraints: Vec<String>,
    /// Design assumptions
    pub assumptions: Vec<String>,
}

/// Architectural layer in a layered design
///
/// Represents a horizontal layer (e.g., API, Application, Domain, Infrastructure).
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::Layer;
///
/// let layer = Layer {
///     name: "API Layer".to_string(),
///     description: "REST and GraphQL endpoints".to_string(),
///     responsibilities: vec![
///         "Request validation".to_string(),
///         "Authentication".to_string(),
///     ],
///     components: vec!["rest-api".to_string(), "graphql-api".to_string()],
///     dependencies: vec!["application-layer".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Layer {
    /// Layer name (e.g., "API Layer")
    pub name: String,
    /// Layer description and purpose
    pub description: String,
    /// Key responsibilities
    pub responsibilities: Vec<String>,
    /// Component IDs in this layer
    pub components: Vec<String>,
    /// IDs of layers this depends on
    pub dependencies: Vec<String>,
}

/// Individual software component
///
/// Represents a deployable unit, service, or module within the architecture.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{Component, Interface};
///
/// let component = Component {
///     id: "payment-service".to_string(),
///     name: "Payment Service".to_string(),
///     description: "Handles payment processing via Stripe".to_string(),
///     layer: "application".to_string(),
///     responsibilities: vec![
///         "Process payments".to_string(),
///         "Refund management".to_string(),
///     ],
///     interfaces: vec![
///         Interface {
///             name: "Payment API".to_string(),
///             protocol: "REST".to_string(),
///             endpoints: vec!["/api/payments".to_string()],
///             data_formats: vec!["application/json".to_string()],
///         },
///     ],
///     dependencies: vec!["database".to_string(), "stripe-api".to_string()],
///     technology_stack: vec!["Rust".to_string(), "Actix-Web".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Component {
    /// Unique component identifier (kebab-case)
    pub id: String,
    /// Human-readable component name
    pub name: String,
    /// Component description and purpose
    pub description: String,
    /// Layer this component belongs to
    pub layer: String,
    /// Component responsibilities
    pub responsibilities: Vec<String>,
    /// External interfaces exposed by this component
    pub interfaces: Vec<Interface>,
    /// IDs of components/systems this depends on
    pub dependencies: Vec<String>,
    /// Technology stack (languages, frameworks, libraries)
    pub technology_stack: Vec<String>,
}

/// External interface exposed by a component
///
/// Defines how other components interact with this component.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::Interface;
///
/// let interface = Interface {
///     name: "User API".to_string(),
///     protocol: "gRPC".to_string(),
///     endpoints: vec![
///         "CreateUser".to_string(),
///         "GetUser".to_string(),
///     ],
///     data_formats: vec!["protobuf".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Interface {
    /// Interface name
    pub name: String,
    /// Communication protocol (REST, gRPC, GraphQL, etc.)
    pub protocol: String,
    /// Available endpoints or methods
    pub endpoints: Vec<String>,
    /// Data formats supported (JSON, XML, protobuf, etc.)
    pub data_formats: Vec<String>,
}

/// Integration between components
///
/// Represents a connection or data flow between two components.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{Integration, IntegrationType};
///
/// let integration = Integration {
///     from_component: "order-service".to_string(),
///     to_component: "payment-service".to_string(),
///     integration_type: IntegrationType::Synchronous,
///     description: "Order service calls payment service to process payment".to_string(),
///     protocols: vec!["HTTPS".to_string(), "REST".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Integration {
    /// Source component ID
    pub from_component: String,
    /// Target component ID
    pub to_component: String,
    /// Type of integration pattern
    pub integration_type: IntegrationType,
    /// Integration description
    pub description: String,
    /// Protocols used (HTTP, AMQP, gRPC, etc.)
    pub protocols: Vec<String>,
}

/// Type of integration pattern between components
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::IntegrationType;
///
/// let sync = IntegrationType::Synchronous;
/// let async_type = IntegrationType::Asynchronous;
/// let event = IntegrationType::EventDriven;
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    /// Direct synchronous call (request-response)
    Synchronous,
    /// Asynchronous messaging
    Asynchronous,
    /// Event-driven publish-subscribe
    EventDriven,
    /// Shared database or data store
    DataSharing,
    /// Batch file transfer or ETL
    BatchProcessing,
}

/// Deployment architecture specification
///
/// Describes infrastructure, scaling, and availability strategy.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::{
///     DeploymentArchitecture, DeploymentStrategy, ScalingStrategy,
///     AvailabilityDesign, InfrastructureComponent,
/// };
///
/// let deployment = DeploymentArchitecture {
///     strategy: DeploymentStrategy::Kubernetes,
///     infrastructure: vec![
///         InfrastructureComponent {
///             name: "EKS Cluster".to_string(),
///             component_type: "Container Orchestration".to_string(),
///             purpose: "Run microservices".to_string(),
///             configuration: std::collections::HashMap::new(),
///         },
///     ],
///     scaling: ScalingStrategy::AutoScaling,
///     availability: AvailabilityDesign {
///         redundancy: "Multi-AZ deployment".to_string(),
///         failover: "Automatic pod restart".to_string(),
///         disaster_recovery: "Daily backups to S3".to_string(),
///     },
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeploymentArchitecture {
    /// Primary deployment strategy
    pub strategy: DeploymentStrategy,
    /// Infrastructure components (servers, clusters, services)
    pub infrastructure: Vec<InfrastructureComponent>,
    /// Scaling strategy
    pub scaling: ScalingStrategy,
    /// Availability and resilience design
    pub availability: AvailabilityDesign,
}

/// Infrastructure component (server, cluster, managed service)
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::InfrastructureComponent;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// config.insert("instance_type".to_string(), "t3.large".to_string());
/// config.insert("replicas".to_string(), "3".to_string());
///
/// let component = InfrastructureComponent {
///     name: "Application Servers".to_string(),
///     component_type: "EC2 Auto Scaling Group".to_string(),
///     purpose: "Run application containers".to_string(),
///     configuration: config,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfrastructureComponent {
    /// Component name
    pub name: String,
    /// Type (e.g., "Load Balancer", "Database", "Cache")
    pub component_type: String,
    /// Purpose and role in infrastructure
    pub purpose: String,
    /// Configuration parameters
    pub configuration: HashMap<String, String>,
}

/// Quality attribute (non-functional requirement)
///
/// Describes quality characteristics like performance, security, scalability.
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::QualityAttribute;
///
/// let quality = QualityAttribute {
///     name: "Performance".to_string(),
///     description: "System must handle 10k requests/sec".to_string(),
///     tactics: vec![
///         "Caching".to_string(),
///         "Load balancing".to_string(),
///         "Database indexing".to_string(),
///     ],
///     metrics: vec![
///         "p95 latency < 100ms".to_string(),
///         "p99 latency < 500ms".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityAttribute {
    /// Attribute name (Performance, Security, Scalability, etc.)
    pub name: String,
    /// Description and target
    pub description: String,
    /// Architectural tactics to achieve this quality
    pub tactics: Vec<String>,
    /// Measurable metrics
    pub metrics: Vec<String>,
}

/// Primary architecture pattern
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::ArchitecturePattern;
///
/// let pattern = ArchitecturePattern::Microservices;
/// assert_eq!(pattern.to_string(), "Microservices");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchitecturePattern {
    /// Single deployable unit
    Monolithic,
    /// Independently deployable services
    Microservices,
    /// Event-driven architecture
    EventDriven,
    /// Traditional layered architecture
    Layered,
    /// Hexagonal/ports-and-adapters architecture
    Hexagonal,
    /// Command Query Responsibility Segregation
    CQRS,
    /// Function-as-a-Service architecture
    Serverless,
    /// Custom or hybrid pattern
    Custom(String),
}

impl fmt::Display for ArchitecturePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Monolithic => "Monolithic",
            Self::Microservices => "Microservices",
            Self::EventDriven => "EventDriven",
            Self::Layered => "Layered",
            Self::Hexagonal => "Hexagonal",
            Self::CQRS => "CQRS",
            Self::Serverless => "Serverless",
            Self::Custom(name) => name,
        };
        write!(f, "{}", s)
    }
}

/// Deployment strategy
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::DeploymentStrategy;
///
/// let strategy = DeploymentStrategy::Kubernetes;
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeploymentStrategy {
    /// Single server instance
    SingleInstance,
    /// Multiple instances behind load balancer
    LoadBalanced,
    /// Docker containers
    Containerized,
    /// Kubernetes orchestration
    Kubernetes,
    /// Serverless functions (Lambda, Cloud Functions, etc.)
    Serverless,
}

/// Scaling strategy
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::ScalingStrategy;
///
/// let scaling = ScalingStrategy::AutoScaling;
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScalingStrategy {
    /// Increase resources on existing instances
    Vertical,
    /// Add more instances
    Horizontal,
    /// Automatic scaling based on metrics
    AutoScaling,
    /// Combination of vertical and horizontal
    Hybrid,
}

/// Availability and resilience design
///
/// # Examples
///
/// ```rust
/// use xzagentz::domain::architecture::AvailabilityDesign;
///
/// let availability = AvailabilityDesign {
///     redundancy: "3 availability zones".to_string(),
///     failover: "Automatic with health checks".to_string(),
///     disaster_recovery: "Cross-region replication".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AvailabilityDesign {
    /// Redundancy strategy
    pub redundancy: String,
    /// Failover mechanism
    pub failover: String,
    /// Disaster recovery plan
    pub disaster_recovery: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_document_creation() {
        let doc = create_test_document();
        assert_eq!(doc.metadata.title, "Test Architecture");
        assert_eq!(doc.layers.len(), 1);
        assert_eq!(doc.components.len(), 1);
    }

    #[test]
    fn test_architecture_pattern_to_string() {
        assert_eq!(
            ArchitecturePattern::Microservices.to_string(),
            "Microservices"
        );
        assert_eq!(ArchitecturePattern::Layered.to_string(), "Layered");
        assert_eq!(
            ArchitecturePattern::Custom("Hybrid".to_string()).to_string(),
            "Hybrid"
        );
    }

    #[test]
    fn test_component_with_dependencies() {
        let component = Component {
            id: "service-a".to_string(),
            name: "Service A".to_string(),
            description: "Test service".to_string(),
            layer: "application".to_string(),
            responsibilities: vec!["Process data".to_string()],
            interfaces: vec![],
            dependencies: vec!["service-b".to_string(), "database".to_string()],
            technology_stack: vec!["Rust".to_string()],
        };

        assert_eq!(component.dependencies.len(), 2);
        assert!(component.dependencies.contains(&"database".to_string()));
    }

    #[test]
    fn test_integration_types() {
        let sync = Integration {
            from_component: "a".to_string(),
            to_component: "b".to_string(),
            integration_type: IntegrationType::Synchronous,
            description: "Sync call".to_string(),
            protocols: vec!["HTTP".to_string()],
        };

        assert_eq!(sync.integration_type, IntegrationType::Synchronous);
    }

    #[test]
    fn test_deployment_architecture_creation() {
        let deployment = DeploymentArchitecture {
            strategy: DeploymentStrategy::Kubernetes,
            infrastructure: vec![InfrastructureComponent {
                name: "Cluster".to_string(),
                component_type: "K8s".to_string(),
                purpose: "Orchestration".to_string(),
                configuration: HashMap::new(),
            }],
            scaling: ScalingStrategy::AutoScaling,
            availability: AvailabilityDesign {
                redundancy: "Multi-AZ".to_string(),
                failover: "Auto".to_string(),
                disaster_recovery: "Backups".to_string(),
            },
        };

        assert_eq!(deployment.strategy, DeploymentStrategy::Kubernetes);
        assert_eq!(deployment.infrastructure.len(), 1);
    }

    #[test]
    fn test_quality_attribute_with_metrics() {
        let quality = QualityAttribute {
            name: "Performance".to_string(),
            description: "Fast response".to_string(),
            tactics: vec!["Caching".to_string()],
            metrics: vec!["p95 < 100ms".to_string()],
        };

        assert_eq!(quality.name, "Performance");
        assert_eq!(quality.tactics.len(), 1);
        assert_eq!(quality.metrics.len(), 1);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let doc = create_test_document();
        let json = serde_json::to_string(&doc).expect("Failed to serialize");
        let deserialized: ArchitectureDocument =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(doc, deserialized);
    }

    // Helper function to create test document
    fn create_test_document() -> ArchitectureDocument {
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
                description: "Test system".to_string(),
                business_goals: vec![],
                constraints: vec![],
                assumptions: vec![],
            },
            layers: vec![Layer {
                name: "API".to_string(),
                description: "API layer".to_string(),
                responsibilities: vec![],
                components: vec![],
                dependencies: vec![],
            }],
            components: vec![Component {
                id: "test-component".to_string(),
                name: "Test Component".to_string(),
                description: "Test".to_string(),
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
}
