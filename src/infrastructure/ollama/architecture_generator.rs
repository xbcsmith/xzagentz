// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Ollama-based architecture generator implementation
//!
//! This module provides an implementation of the `ArchitectureGenerator` trait
//! that uses Ollama's local LLM service to generate software architecture documents.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::domain::architecture::{
//!     ArchitectureGenerator, GenerationOptions, ComplexityLevel, ArchitecturePattern,
//! };
//! use xzagentz::infrastructure::ollama::{OllamaArchitectureGenerator, OllamaConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = OllamaConfig::default();
//! let generator = OllamaArchitectureGenerator::new(config)?;
//!
//! let requirements = "Build an e-commerce platform with user management, \
//!                     product catalog, shopping cart, and payment processing";
//!
//! let options = GenerationOptions {
//!     pattern: Some(ArchitecturePattern::Microservices),
//!     complexity: ComplexityLevel::Moderate,
//!     include_deployment: true,
//!     include_quality_attributes: true,
//!     max_components: Some(10),
//!     technology_preferences: vec!["Rust".to_string(), "PostgreSQL".to_string()],
//! };
//!
//! let document = generator.generate(requirements, &options).await?;
//! println!("Generated: {}", document.metadata.title);
//! # Ok(())
//! # }
//! ```

use crate::domain::architecture::{
    ArchitectureDocument, ArchitectureGenerator, ArchitectureMetadata, ArchitecturePattern,
    ArchitectureTemplate, ArchitectureValidator, Component, GenerationOptions, Integration,
    IntegrationType, Interface, Layer, Overview,
};
use crate::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaError};
use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use thiserror::Error;

/// Ollama-based architecture generator
///
/// Generates architecture documents using Ollama's local LLM service.
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::infrastructure::ollama::{OllamaArchitectureGenerator, OllamaConfig};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = OllamaConfig::default();
/// let generator = OllamaArchitectureGenerator::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct OllamaArchitectureGenerator {
    /// Ollama client for LLM interactions
    client: OllamaClient,
    /// Default model to use for generation
    default_model: String,
}

impl OllamaArchitectureGenerator {
    /// Create a new Ollama architecture generator
    ///
    /// # Arguments
    ///
    /// * `config` - Ollama configuration
    ///
    /// # Returns
    ///
    /// Returns a new generator instance or an error
    ///
    /// # Errors
    ///
    /// Returns error if client creation fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaArchitectureGenerator, OllamaConfig};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = OllamaConfig::default();
    /// let generator = OllamaArchitectureGenerator::new(config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: OllamaConfig) -> Result<Self, GeneratorError> {
        let default_model = config.default_model().to_string();
        let client = OllamaClient::new(config).map_err(GeneratorError::ClientCreationError)?;
        Ok(Self {
            client,
            default_model,
        })
    }

    /// Build system prompt for architecture generation
    fn build_system_prompt(&self, requirements: &str, options: &GenerationOptions) -> String {
        let mut prompt = String::from(
            "You are an expert software architect. Generate a complete software architecture \
             document in JSON format based on the following requirements.\n\n",
        );

        prompt.push_str("Requirements:\n");
        prompt.push_str(requirements);
        prompt.push_str("\n\n");

        if let Some(pattern) = &options.pattern {
            prompt.push_str(&format!("Architecture Pattern: {}\n", pattern));
        }

        prompt.push_str(&format!("Complexity Level: {:?}\n", options.complexity));

        if !options.technology_preferences.is_empty() {
            prompt.push_str(&format!(
                "Technology Preferences: {}\n",
                options.technology_preferences.join(", ")
            ));
        }

        if let Some(max) = options.max_components {
            prompt.push_str(&format!("Maximum Components: {}\n", max));
        }

        prompt.push('\n');
        prompt.push_str("Generate a JSON response with the following structure:\n");
        prompt.push_str("{\n");
        prompt.push_str("  \"title\": \"Architecture Title\",\n");
        prompt.push_str("  \"overview\": {\n");
        prompt.push_str("    \"description\": \"System description\",\n");
        prompt.push_str("    \"business_goals\": [\"goal1\", \"goal2\"],\n");
        prompt.push_str("    \"constraints\": [\"constraint1\"],\n");
        prompt.push_str("    \"assumptions\": [\"assumption1\"]\n");
        prompt.push_str("  },\n");
        prompt.push_str("  \"layers\": [\n");
        prompt.push_str("    {\n");
        prompt.push_str("      \"name\": \"Layer Name\",\n");
        prompt.push_str("      \"description\": \"Layer description\",\n");
        prompt.push_str("      \"responsibilities\": [\"responsibility1\"],\n");
        prompt.push_str("      \"components\": [\"component-id\"],\n");
        prompt.push_str("      \"dependencies\": [\"other-layer\"]\n");
        prompt.push_str("    }\n");
        prompt.push_str("  ],\n");
        prompt.push_str("  \"components\": [\n");
        prompt.push_str("    {\n");
        prompt.push_str("      \"id\": \"component-id\",\n");
        prompt.push_str("      \"name\": \"Component Name\",\n");
        prompt.push_str("      \"description\": \"Component description\",\n");
        prompt.push_str("      \"layer\": \"Layer Name\",\n");
        prompt.push_str("      \"responsibilities\": [\"responsibility1\"],\n");
        prompt.push_str("      \"interfaces\": [],\n");
        prompt.push_str("      \"dependencies\": [],\n");
        prompt.push_str("      \"technology_stack\": [\"tech1\"]\n");
        prompt.push_str("    }\n");
        prompt.push_str("  ],\n");
        prompt.push_str("  \"integrations\": []\n");
        prompt.push_str("}\n\n");
        prompt.push_str("Respond ONLY with valid JSON. Do not include any explanatory text.\n");

        prompt
    }

    /// Parse LLM response into architecture document
    fn parse_response(
        &self,
        response: &str,
        model: &str,
    ) -> Result<ArchitectureDocument, GeneratorError> {
        // Extract JSON from response (handle code blocks)
        let json_str = self.extract_json(response)?;

        // Parse JSON response
        let arch_response: ArchitectureResponse = serde_json::from_str(&json_str)
            .map_err(|e| GeneratorError::ParseError(format!("JSON parse error: {}", e)))?;

        // Convert to domain model
        let document = self.convert_response_to_document(arch_response, model)?;

        // Validate the document
        let validator = ArchitectureValidator;
        validator
            .validate(&document)
            .map_err(|e| GeneratorError::ValidationError(format!("{}", e)))?;

        Ok(document)
    }

    /// Extract JSON from response text
    fn extract_json(&self, text: &str) -> Result<String, GeneratorError> {
        let trimmed = text.trim();

        // Check if it's already valid JSON
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            return Ok(trimmed.to_string());
        }

        // Try to extract from code blocks
        if let Some(start) = text.find("```json") {
            let content_start = start + 7; // length of "```json"
            if let Some(end_pos) = text[content_start..].find("```") {
                let json_end = content_start + end_pos;
                return Ok(text[content_start..json_end].trim().to_string());
            }
        }

        // Try to extract from generic code blocks
        if let Some(start) = text.find("```") {
            if let Some(end) = text[start + 3..].find("```") {
                let json_start = start + 3;
                let json_end = start + 3 + end;
                let potential_json = text[json_start..json_end].trim();
                if potential_json.starts_with('{') {
                    return Ok(potential_json.to_string());
                }
            }
        }

        // Try to find JSON object in text
        if let Some(start) = text.find('{') {
            if let Some(end) = text.rfind('}') {
                if end > start {
                    return Ok(text[start..=end].to_string());
                }
            }
        }

        Err(GeneratorError::ParseError(
            "Could not extract valid JSON from response".to_string(),
        ))
    }

    /// Convert response to architecture document
    fn convert_response_to_document(
        &self,
        response: ArchitectureResponse,
        model: &str,
    ) -> Result<ArchitectureDocument, GeneratorError> {
        let pattern = ArchitecturePattern::Layered; // Default, could be inferred

        let metadata = ArchitectureMetadata {
            title: response.title,
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            model_used: model.to_string(),
            pattern,
            authors: vec!["Ollama LLM".to_string()],
        };

        let overview = Overview {
            description: response.overview.description,
            business_goals: response.overview.business_goals,
            constraints: response.overview.constraints,
            assumptions: response.overview.assumptions,
        };

        let layers: Vec<Layer> = response
            .layers
            .into_iter()
            .map(|l| Layer {
                name: l.name,
                description: l.description,
                responsibilities: l.responsibilities,
                components: l.components,
                dependencies: l.dependencies,
            })
            .collect();

        let components: Vec<Component> = response
            .components
            .into_iter()
            .map(|c| Component {
                id: c.id,
                name: c.name,
                description: c.description,
                layer: c.layer,
                responsibilities: c.responsibilities,
                interfaces: c
                    .interfaces
                    .into_iter()
                    .map(|i| Interface {
                        name: i.name,
                        protocol: i.protocol,
                        endpoints: i.endpoints,
                        data_formats: i.data_formats,
                    })
                    .collect(),
                dependencies: c.dependencies,
                technology_stack: c.technology_stack,
            })
            .collect();

        let integrations: Vec<Integration> = response
            .integrations
            .into_iter()
            .map(|i| Integration {
                from_component: i.from_component,
                to_component: i.to_component,
                integration_type: match i.integration_type.as_str() {
                    "Asynchronous" => IntegrationType::Asynchronous,
                    "EventDriven" => IntegrationType::EventDriven,
                    "DataSharing" => IntegrationType::DataSharing,
                    "BatchProcessing" => IntegrationType::BatchProcessing,
                    _ => IntegrationType::Synchronous,
                },
                description: i.description,
                protocols: i.protocols,
            })
            .collect();

        Ok(ArchitectureDocument {
            metadata,
            overview,
            layers,
            components,
            integrations,
            deployment: None,
            quality_attributes: vec![],
        })
    }
}

#[async_trait]
impl ArchitectureGenerator for OllamaArchitectureGenerator {
    type Error = GeneratorError;

    async fn generate(
        &self,
        requirements: &str,
        options: &GenerationOptions,
    ) -> Result<ArchitectureDocument, Self::Error> {
        let prompt = self.build_system_prompt(requirements, options);

        let response = self
            .client
            .generate(&self.default_model, &prompt, None)
            .await
            .map_err(GeneratorError::NetworkError)?;

        self.parse_response(&response, &self.default_model)
    }

    async fn refine(
        &self,
        document: &ArchitectureDocument,
        feedback: &str,
    ) -> Result<ArchitectureDocument, Self::Error> {
        let current_json = serde_json::to_string_pretty(document)
            .map_err(|e| GeneratorError::SerializationError(e.to_string()))?;

        let prompt = format!(
            "You are an expert software architect. Refine the following architecture document \
             based on the feedback provided.\n\n\
             Current Architecture (JSON):\n{}\n\n\
             Feedback:\n{}\n\n\
             Generate an improved architecture document in JSON format with the same structure. \
             Respond ONLY with valid JSON.",
            current_json, feedback
        );

        let response = self
            .client
            .generate(&self.default_model, &prompt, None)
            .await
            .map_err(GeneratorError::NetworkError)?;

        self.parse_response(&response, &self.default_model)
    }

    async fn generate_from_template(
        &self,
        template: &ArchitectureTemplate,
        customization: &str,
    ) -> Result<ArchitectureDocument, Self::Error> {
        let template_json = serde_json::to_string_pretty(template)
            .map_err(|e| GeneratorError::SerializationError(e.to_string()))?;

        let prompt = format!(
            "You are an expert software architect. Generate a complete architecture document \
             based on the following template and customization instructions.\n\n\
             Template (JSON):\n{}\n\n\
             Customization:\n{}\n\n\
             Generate a complete architecture document in JSON format following the structure \
             defined in the template. Respond ONLY with valid JSON in the architecture document format.",
            template_json, customization
        );

        let response = self
            .client
            .generate(&self.default_model, &prompt, None)
            .await
            .map_err(GeneratorError::NetworkError)?;

        self.parse_response(&response, &self.default_model)
    }
}

/// JSON response structures for LLM output parsing
#[derive(Debug, Deserialize)]
struct ArchitectureResponse {
    title: String,
    overview: OverviewResponse,
    layers: Vec<LayerResponse>,
    components: Vec<ComponentResponse>,
    #[serde(default)]
    integrations: Vec<IntegrationResponse>,
}

#[derive(Debug, Deserialize)]
struct OverviewResponse {
    description: String,
    #[serde(default)]
    business_goals: Vec<String>,
    #[serde(default)]
    constraints: Vec<String>,
    #[serde(default)]
    assumptions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LayerResponse {
    name: String,
    description: String,
    #[serde(default)]
    responsibilities: Vec<String>,
    #[serde(default)]
    components: Vec<String>,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ComponentResponse {
    id: String,
    name: String,
    description: String,
    layer: String,
    #[serde(default)]
    responsibilities: Vec<String>,
    #[serde(default)]
    interfaces: Vec<InterfaceResponse>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    technology_stack: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct InterfaceResponse {
    name: String,
    protocol: String,
    #[serde(default)]
    endpoints: Vec<String>,
    #[serde(default)]
    data_formats: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct IntegrationResponse {
    from_component: String,
    to_component: String,
    integration_type: String,
    description: String,
    #[serde(default)]
    protocols: Vec<String>,
}

/// Errors that can occur during architecture generation
#[derive(Error, Debug)]
pub enum GeneratorError {
    /// Error creating Ollama client
    #[error("Failed to create Ollama client: {0}")]
    ClientCreationError(#[source] OllamaError),

    /// Network error communicating with Ollama
    #[error("Network error: {0}")]
    NetworkError(#[source] OllamaError),

    /// Error parsing LLM response
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Error serializing data
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_plain() {
        let generator = create_test_generator();
        let json = r#"{"title": "Test"}"#;
        let result = generator.extract_json(json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json);
    }

    #[test]
    fn test_extract_json_with_code_block() {
        let generator = create_test_generator();
        let text = r#"Here is the JSON:
```json
{"title": "Test"}
```
That's it."#;
        let result = generator.extract_json(text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"title": "Test"}"#);
    }

    #[test]
    fn test_extract_json_from_mixed_content() {
        let generator = create_test_generator();
        let text = r#"Some text before {"title": "Test"} some text after"#;
        let result = generator.extract_json(text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"title": "Test"}"#);
    }

    #[test]
    fn test_build_system_prompt() {
        let generator = create_test_generator();
        let options = GenerationOptions {
            pattern: Some(ArchitecturePattern::Microservices),
            complexity: crate::domain::architecture::ComplexityLevel::Moderate,
            include_deployment: true,
            include_quality_attributes: true,
            max_components: Some(10),
            technology_preferences: vec!["Rust".to_string()],
        };

        let prompt = generator.build_system_prompt("Build an app", &options);
        assert!(prompt.contains("Microservices"));
        assert!(prompt.contains("Build an app"));
        assert!(prompt.contains("Rust"));
        assert!(prompt.contains("Maximum Components: 10"));
    }

    fn create_test_generator() -> OllamaArchitectureGenerator {
        let config = OllamaConfig::default();
        OllamaArchitectureGenerator::new(config).expect("Failed to create generator")
    }
}
