// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! File-based template repository implementation
//!
//! This module provides a file-based implementation of the `TemplateRepository` trait
//! that stores architecture templates as YAML files in a directory.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::TemplateRepository;
//! use xzagentz::infrastructure::templates::FileTemplateRepository;
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let templates_dir = PathBuf::from("templates/architecture");
//! let repo = FileTemplateRepository::new(templates_dir);
//!
//! // Initialize with default templates
//! repo.initialize().await?;
//!
//! // List available templates
//! let templates = repo.list_templates().await?;
//! for template in templates {
//!     println!("{}: {}", template.name, template.description);
//! }
//!
//! // Load a specific template
//! let template = repo.load_template("microservices").await?;
//! println!("Loaded template: {}", template.info.name);
//! # Ok(())
//! # }
//! ```

use crate::domain::architecture::{
    ArchitecturePattern, ArchitectureTemplate, ComponentTemplate, TemplateInfo, TemplateRepository,
    TemplateStructure,
};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;

/// File-based template repository
///
/// Stores architecture templates as YAML files in a directory.
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::infrastructure::templates::FileTemplateRepository;
/// use std::path::PathBuf;
///
/// let repo = FileTemplateRepository::new(PathBuf::from("templates/architecture"));
/// ```
#[derive(Debug, Clone)]
pub struct FileTemplateRepository {
    /// Directory containing template files
    templates_dir: PathBuf,
}

impl FileTemplateRepository {
    /// Create a new file-based template repository
    ///
    /// # Arguments
    ///
    /// * `templates_dir` - Directory path where templates are stored
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// use std::path::PathBuf;
    ///
    /// let repo = FileTemplateRepository::new(PathBuf::from("templates/architecture"));
    /// ```
    pub fn new(templates_dir: PathBuf) -> Self {
        Self { templates_dir }
    }

    /// Initialize repository with default templates
    ///
    /// Creates the templates directory if it does not exist and
    /// writes default template files.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error
    ///
    /// # Errors
    ///
    /// Returns error if directory creation or file writing fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::templates::FileTemplateRepository;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let repo = FileTemplateRepository::new(PathBuf::from("templates/architecture"));
    /// repo.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), RepositoryError> {
        fs::create_dir_all(&self.templates_dir)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        self.create_default_templates().await?;
        Ok(())
    }

    /// Create default template files
    async fn create_default_templates(&self) -> Result<(), RepositoryError> {
        let templates = vec![
            self.create_microservices_template(),
            self.create_monolithic_template(),
            self.create_event_driven_template(),
            self.create_layered_template(),
        ];

        for template in templates {
            self.save_template(&template).await?;
        }

        Ok(())
    }

    /// Create microservices template
    fn create_microservices_template(&self) -> ArchitectureTemplate {
        ArchitectureTemplate {
            info: TemplateInfo {
                name: "microservices".to_string(),
                pattern: ArchitecturePattern::Microservices,
                description: "Cloud-native microservices architecture with independent services"
                    .to_string(),
                use_cases: vec![
                    "Scalable web applications".to_string(),
                    "Distributed systems".to_string(),
                    "Multi-team development".to_string(),
                ],
            },
            structure: TemplateStructure {
                layers: vec![
                    "API Gateway".to_string(),
                    "Service Layer".to_string(),
                    "Data Layer".to_string(),
                ],
                integration_patterns: vec![
                    "REST APIs".to_string(),
                    "Message Queue".to_string(),
                    "Event Bus".to_string(),
                ],
                required_components: vec![
                    "api-gateway".to_string(),
                    "service-discovery".to_string(),
                    "message-broker".to_string(),
                ],
            },
            default_components: vec![
                ComponentTemplate {
                    name: "API Gateway".to_string(),
                    layer: "API Gateway".to_string(),
                    role: "Entry point for all client requests, handles routing and authentication"
                        .to_string(),
                    typical_technologies: vec![
                        "Kong".to_string(),
                        "Nginx".to_string(),
                        "Traefik".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Service Discovery".to_string(),
                    layer: "Service Layer".to_string(),
                    role: "Dynamic service registration and discovery".to_string(),
                    typical_technologies: vec![
                        "Consul".to_string(),
                        "Eureka".to_string(),
                        "etcd".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Message Broker".to_string(),
                    layer: "Service Layer".to_string(),
                    role: "Asynchronous communication between services".to_string(),
                    typical_technologies: vec![
                        "RabbitMQ".to_string(),
                        "Kafka".to_string(),
                        "Redis".to_string(),
                    ],
                },
            ],
            customization_points: vec![
                "Add specific microservices based on business domains".to_string(),
                "Configure service mesh for advanced networking".to_string(),
                "Add monitoring and tracing components".to_string(),
            ],
        }
    }

    /// Create monolithic template
    fn create_monolithic_template(&self) -> ArchitectureTemplate {
        ArchitectureTemplate {
            info: TemplateInfo {
                name: "monolithic".to_string(),
                pattern: ArchitecturePattern::Monolithic,
                description: "Traditional single-deployment monolithic architecture".to_string(),
                use_cases: vec![
                    "Small to medium applications".to_string(),
                    "Rapid prototyping".to_string(),
                    "Simple deployment requirements".to_string(),
                ],
            },
            structure: TemplateStructure {
                layers: vec![
                    "Presentation Layer".to_string(),
                    "Business Logic Layer".to_string(),
                    "Data Access Layer".to_string(),
                ],
                integration_patterns: vec!["Direct method calls".to_string()],
                required_components: vec![
                    "web-server".to_string(),
                    "application".to_string(),
                    "database".to_string(),
                ],
            },
            default_components: vec![
                ComponentTemplate {
                    name: "Web Server".to_string(),
                    layer: "Presentation Layer".to_string(),
                    role: "Handles HTTP requests and serves web pages".to_string(),
                    typical_technologies: vec![
                        "Nginx".to_string(),
                        "Apache".to_string(),
                        "IIS".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Application".to_string(),
                    layer: "Business Logic Layer".to_string(),
                    role: "Core application logic and business rules".to_string(),
                    typical_technologies: vec![
                        "Spring Boot".to_string(),
                        "Django".to_string(),
                        "Rails".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Database".to_string(),
                    layer: "Data Access Layer".to_string(),
                    role: "Persistent data storage".to_string(),
                    typical_technologies: vec![
                        "PostgreSQL".to_string(),
                        "MySQL".to_string(),
                        "SQLite".to_string(),
                    ],
                },
            ],
            customization_points: vec![
                "Add caching layer for performance".to_string(),
                "Implement background job processing".to_string(),
                "Add file storage integration".to_string(),
            ],
        }
    }

    /// Create event-driven template
    fn create_event_driven_template(&self) -> ArchitectureTemplate {
        ArchitectureTemplate {
            info: TemplateInfo {
                name: "event-driven".to_string(),
                pattern: ArchitecturePattern::EventDriven,
                description: "Event-driven architecture with publish-subscribe pattern".to_string(),
                use_cases: vec![
                    "Real-time data processing".to_string(),
                    "IoT systems".to_string(),
                    "Complex event processing".to_string(),
                ],
            },
            structure: TemplateStructure {
                layers: vec![
                    "Event Producers".to_string(),
                    "Event Bus".to_string(),
                    "Event Consumers".to_string(),
                ],
                integration_patterns: vec![
                    "Publish-Subscribe".to_string(),
                    "Event Streaming".to_string(),
                ],
                required_components: vec!["event-bus".to_string(), "event-store".to_string()],
            },
            default_components: vec![
                ComponentTemplate {
                    name: "Event Bus".to_string(),
                    layer: "Event Bus".to_string(),
                    role: "Central event routing and delivery".to_string(),
                    typical_technologies: vec![
                        "Apache Kafka".to_string(),
                        "RabbitMQ".to_string(),
                        "AWS EventBridge".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Event Store".to_string(),
                    layer: "Event Bus".to_string(),
                    role: "Persistent event log storage".to_string(),
                    typical_technologies: vec![
                        "EventStoreDB".to_string(),
                        "Kafka".to_string(),
                        "PostgreSQL".to_string(),
                    ],
                },
            ],
            customization_points: vec![
                "Add event producers for different data sources".to_string(),
                "Add event consumers for specific processing needs".to_string(),
                "Implement event replay capabilities".to_string(),
            ],
        }
    }

    /// Create layered template
    fn create_layered_template(&self) -> ArchitectureTemplate {
        ArchitectureTemplate {
            info: TemplateInfo {
                name: "layered".to_string(),
                pattern: ArchitecturePattern::Layered,
                description: "Traditional layered architecture with clear separation of concerns"
                    .to_string(),
                use_cases: vec![
                    "Enterprise applications".to_string(),
                    "Well-structured codebases".to_string(),
                    "Team-based development".to_string(),
                ],
            },
            structure: TemplateStructure {
                layers: vec![
                    "Presentation Layer".to_string(),
                    "Application Layer".to_string(),
                    "Domain Layer".to_string(),
                    "Infrastructure Layer".to_string(),
                ],
                integration_patterns: vec!["Layer-to-layer calls".to_string()],
                required_components: vec![
                    "ui".to_string(),
                    "application-services".to_string(),
                    "domain-model".to_string(),
                    "data-access".to_string(),
                ],
            },
            default_components: vec![
                ComponentTemplate {
                    name: "User Interface".to_string(),
                    layer: "Presentation Layer".to_string(),
                    role: "User interaction and display".to_string(),
                    typical_technologies: vec![
                        "React".to_string(),
                        "Angular".to_string(),
                        "Vue.js".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Application Services".to_string(),
                    layer: "Application Layer".to_string(),
                    role: "Orchestrates business workflows".to_string(),
                    typical_technologies: vec![
                        "REST APIs".to_string(),
                        "GraphQL".to_string(),
                        "gRPC".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Domain Model".to_string(),
                    layer: "Domain Layer".to_string(),
                    role: "Core business logic and entities".to_string(),
                    typical_technologies: vec![
                        "Domain Models".to_string(),
                        "Business Rules".to_string(),
                    ],
                },
                ComponentTemplate {
                    name: "Data Access".to_string(),
                    layer: "Infrastructure Layer".to_string(),
                    role: "Database operations and persistence".to_string(),
                    typical_technologies: vec![
                        "ORM".to_string(),
                        "Repository Pattern".to_string(),
                        "SQL".to_string(),
                    ],
                },
            ],
            customization_points: vec![
                "Add authentication and authorization layer".to_string(),
                "Implement caching strategy".to_string(),
                "Add logging and monitoring".to_string(),
            ],
        }
    }

    /// Load template from file path
    async fn load_template_from_path(
        &self,
        path: &Path,
    ) -> Result<ArchitectureTemplate, RepositoryError> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        let template: ArchitectureTemplate = serde_yaml::from_str(&content)
            .map_err(|e| RepositoryError::ParseError(format!("YAML parse error: {}", e)))?;

        Ok(template)
    }
}

#[async_trait]
impl TemplateRepository for FileTemplateRepository {
    type Error = RepositoryError;

    async fn list_templates(&self) -> Result<Vec<TemplateInfo>, Self::Error> {
        let mut templates = Vec::new();

        let mut entries = fs::read_dir(&self.templates_dir)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                match self.load_template_from_path(&path).await {
                    Ok(template) => templates.push(template.info),
                    Err(_) => continue, // Skip invalid templates
                }
            }
        }

        Ok(templates)
    }

    async fn load_template(&self, name: &str) -> Result<ArchitectureTemplate, Self::Error> {
        let file_path = self.templates_dir.join(format!("{}.yaml", name));

        if !file_path.exists() {
            return Err(RepositoryError::NotFound(name.to_string()));
        }

        self.load_template_from_path(&file_path).await
    }

    async fn save_template(&self, template: &ArchitectureTemplate) -> Result<(), Self::Error> {
        let file_path = self
            .templates_dir
            .join(format!("{}.yaml", template.info.name));

        let yaml = serde_yaml::to_string(template).map_err(|e| {
            RepositoryError::SerializationError(format!("YAML serialization error: {}", e))
        })?;

        fs::write(&file_path, yaml)
            .await
            .map_err(|e| RepositoryError::IoError(e.to_string()))?;

        Ok(())
    }
}

/// Errors that can occur in the template repository
#[derive(Error, Debug)]
pub enum RepositoryError {
    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Template not found
    #[error("Template not found: {0}")]
    NotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_repository() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileTemplateRepository::new(temp_dir.path().to_path_buf());
        assert_eq!(repo.templates_dir, temp_dir.path());
    }

    #[tokio::test]
    async fn test_initialize_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        let repo = FileTemplateRepository::new(templates_dir.clone());

        repo.initialize().await.unwrap();
        assert!(templates_dir.exists());
    }

    #[tokio::test]
    async fn test_initialize_creates_default_templates() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileTemplateRepository::new(temp_dir.path().to_path_buf());

        repo.initialize().await.unwrap();

        let templates = repo.list_templates().await.unwrap();
        assert!(templates.len() >= 4);

        let names: Vec<String> = templates.iter().map(|t| t.name.clone()).collect();
        assert!(names.contains(&"microservices".to_string()));
        assert!(names.contains(&"monolithic".to_string()));
        assert!(names.contains(&"event-driven".to_string()));
        assert!(names.contains(&"layered".to_string()));
    }

    #[tokio::test]
    async fn test_load_template() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileTemplateRepository::new(temp_dir.path().to_path_buf());

        repo.initialize().await.unwrap();

        let template = repo.load_template("microservices").await.unwrap();
        assert_eq!(template.info.name, "microservices");
        assert_eq!(template.info.pattern, ArchitecturePattern::Microservices);
    }

    #[tokio::test]
    async fn test_load_nonexistent_template() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileTemplateRepository::new(temp_dir.path().to_path_buf());

        repo.initialize().await.unwrap();

        let result = repo.load_template("nonexistent").await;
        assert!(matches!(result, Err(RepositoryError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_save_and_load_template() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileTemplateRepository::new(temp_dir.path().to_path_buf());

        repo.initialize().await.unwrap();

        let custom_template = ArchitectureTemplate {
            info: TemplateInfo {
                name: "custom".to_string(),
                pattern: ArchitecturePattern::Custom("Custom".to_string()),
                description: "Custom template".to_string(),
                use_cases: vec!["Testing".to_string()],
            },
            structure: TemplateStructure {
                layers: vec!["Layer1".to_string()],
                integration_patterns: vec!["Pattern1".to_string()],
                required_components: vec!["component1".to_string()],
            },
            default_components: vec![],
            customization_points: vec![],
        };

        repo.save_template(&custom_template).await.unwrap();

        let loaded = repo.load_template("custom").await.unwrap();
        assert_eq!(loaded.info.name, "custom");
        assert_eq!(loaded.info.description, "Custom template");
    }

    #[tokio::test]
    async fn test_microservices_template_structure() {
        let repo = FileTemplateRepository::new(PathBuf::from("test"));
        let template = repo.create_microservices_template();

        assert_eq!(template.info.pattern, ArchitecturePattern::Microservices);
        assert!(!template.structure.layers.is_empty());
        assert!(!template.default_components.is_empty());
    }

    #[tokio::test]
    async fn test_list_templates_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileTemplateRepository::new(temp_dir.path().to_path_buf());

        fs::create_dir_all(temp_dir.path()).await.unwrap();

        let templates = repo.list_templates().await.unwrap();
        assert_eq!(templates.len(), 0);
    }
}
