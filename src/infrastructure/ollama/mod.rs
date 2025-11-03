//! Ollama infrastructure integration
//!
//! This module provides infrastructure components for integrating with Ollama
//! LLM services. It includes HTTP client, configuration, error handling, and
//! plan generation implementations.
//!
//! # Architecture
//!
//! The Ollama integration consists of several components:
//!
//! - [`OllamaClient`]: HTTP client for Ollama API
//! - [`OllamaConfig`]: Configuration for Ollama services
//! - [`OllamaError`]: Error types for Ollama operations
//! - [`OllamaPlanGenerator`]: Implementation of PlanGenerator trait
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};
//! use xzagentz::domain::planning::{PlanGenerator, ArchitectureDocument, PlanOptions};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create client
//! let config = OllamaConfig::default()
//!     .with_default_model("llama3.2:3b")
//!     .with_timeout_seconds(120);
//!
//! let client = OllamaClient::new(config)?;
//!
//! // Check health
//! if !client.health_check().await? {
//!     eprintln!("Ollama service is not available");
//!     return Ok(());
//! }
//!
//! // Create plan generator
//! let generator = OllamaPlanGenerator::new(client);
//!
//! // Generate plan
//! let architecture = ArchitectureDocument::new("My Project");
//! let options = PlanOptions::default().with_num_phases(5);
//! let plan = generator.generate_plan(&architecture, &options)?;
//!
//! println!("Generated plan: {}", plan.title());
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! The Ollama integration can be configured through [`OllamaConfig`]:
//!
//! ```rust
//! use xzagentz::infrastructure::ollama::OllamaConfig;
//!
//! let config = OllamaConfig::new()
//!     .with_base_url("http://localhost:11434")
//!     .with_default_model("mistral")
//!     .with_timeout_seconds(60)
//!     .with_max_retries(3)
//!     .with_temperature(0.7)
//!     .with_max_tokens(8192);
//!
//! assert_eq!(config.default_model(), "mistral");
//! ```
//!
//! # Error Handling
//!
//! All operations return [`OllamaError`] which provides detailed information
//! about failures:
//!
//! ```rust
//! use xzagentz::infrastructure::ollama::OllamaError;
//!
//! fn handle_error(error: OllamaError) {
//!     if error.is_retryable() {
//!         println!("Error is retryable: {}", error);
//!     } else {
//!         eprintln!("Fatal error: {}", error);
//!     }
//! }
//! ```

pub mod architecture_generator;
pub mod client;
pub mod config;
pub mod error;
pub mod plan_generator;

pub use architecture_generator::{GeneratorError, OllamaArchitectureGenerator};
pub use client::{ModelInfo, OllamaClient};
pub use config::OllamaConfig;
pub use error::OllamaError;
pub use plan_generator::OllamaPlanGenerator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all public types are accessible
        let config = OllamaConfig::default();
        assert_eq!(config.base_url(), "http://localhost:11434");

        let error = OllamaError::timeout(30);
        assert!(error.is_retryable());
    }

    #[test]
    fn test_client_creation() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_generator_creation() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let _generator = OllamaPlanGenerator::new(client);
    }
}
