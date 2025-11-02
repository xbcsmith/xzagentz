//! Ollama HTTP client implementation
//!
//! This module provides an HTTP client for interacting with Ollama services.
//! It handles model listing, text generation, streaming, and health checks.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = OllamaConfig::default();
//! let client = OllamaClient::new(config)?;
//!
//! // Check health
//! let is_healthy = client.health_check().await?;
//! println!("Ollama is healthy: {}", is_healthy);
//!
//! // List available models
//! let models = client.list_models().await?;
//! println!("Available models: {:?}", models);
//!
//! // Generate text
//! let prompt = "Explain Rust ownership";
//! let response = client.generate("llama3.2:3b", prompt, None).await?;
//! println!("Response: {}", response);
//! # Ok(())
//! # }
//! ```

use super::config::OllamaConfig;
use super::error::OllamaError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Ollama HTTP client
///
/// Provides methods for interacting with an Ollama service via HTTP.
/// Supports model listing, text generation, streaming, and health checks.
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = OllamaConfig::default();
/// let client = OllamaClient::new(config)?;
///
/// let is_healthy = client.health_check().await?;
/// assert!(is_healthy);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct OllamaClient {
    /// Base URL of the Ollama service
    base_url: String,

    /// HTTP client for making requests
    http_client: Client,

    /// Request timeout duration
    timeout: Duration,

    /// Maximum number of retry attempts
    max_retries: u32,
}

/// Request body for generate endpoint
#[derive(Debug, Clone, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GenerateOptions>,
}

/// Options for generation
#[derive(Debug, Clone, Serialize)]
struct GenerateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,
}

/// Response from generate endpoint
#[derive(Debug, Clone, Deserialize)]
struct GenerateResponse {
    #[serde(default)]
    response: String,
    #[serde(default)]
    #[allow(dead_code)]
    done: bool,
    #[serde(default)]
    #[allow(dead_code)]
    model: String,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Name of the model
    pub name: String,
    /// Model size (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// Modified timestamp (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

/// Response from list models endpoint
#[derive(Debug, Clone, Deserialize)]
struct ListModelsResponse {
    #[serde(default)]
    models: Vec<ModelInfo>,
}

impl OllamaClient {
    /// Creates a new Ollama client
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the client
    ///
    /// # Returns
    ///
    /// Returns a configured OllamaClient or an error if configuration is invalid
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    ///
    /// let config = OllamaConfig::default();
    /// let client = OllamaClient::new(config);
    /// assert!(client.is_ok());
    /// ```
    pub fn new(config: OllamaConfig) -> Result<Self, OllamaError> {
        config
            .validate()
            .map_err(|e| OllamaError::config(format!("Invalid configuration: {}", e)))?;

        let http_client = Client::builder()
            .timeout(config.timeout_duration())
            .build()
            .map_err(|e| OllamaError::config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            base_url: config.base_url().to_string(),
            http_client,
            timeout: config.timeout_duration(),
            max_retries: config.max_retries(),
        })
    }

    /// Lists available models
    ///
    /// # Returns
    ///
    /// Returns a list of available models or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The service is unavailable
    /// - The request fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::new(OllamaConfig::default())?;
    /// let models = client.list_models().await?;
    /// println!("Found {} models", models.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, OllamaError> {
        let url = format!("{}/api/tags", self.base_url);

        let response = self.http_client.get(&url).send().await.map_err(|e| {
            if e.is_connect() {
                OllamaError::service_unavailable(format!(
                    "Cannot connect to Ollama at {}: {}",
                    self.base_url, e
                ))
            } else {
                OllamaError::from(e)
            }
        })?;

        if !response.status().is_success() {
            return Err(OllamaError::http(format!(
                "HTTP error {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let list_response: ListModelsResponse = response.json().await?;
        Ok(list_response.models)
    }

    /// Generates text using the specified model
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to use
    /// * `prompt` - The prompt to generate from
    /// * `options` - Optional generation options (temperature, max_tokens)
    ///
    /// # Returns
    ///
    /// Returns the generated text or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The model is not found
    /// - The service is unavailable
    /// - Generation fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::new(OllamaConfig::default())?;
    /// let response = client.generate("llama3.2:3b", "Hello", None).await?;
    /// println!("Generated: {}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate(
        &self,
        model: &str,
        prompt: &str,
        options: Option<(Option<f32>, Option<usize>)>,
    ) -> Result<String, OllamaError> {
        let url = format!("{}/api/generate", self.base_url);

        let generate_options = options.map(|(temp, max_tokens)| GenerateOptions {
            temperature: temp,
            num_predict: max_tokens.map(|t| t as i32),
        });

        let request_body = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: Some(false),
            options: generate_options,
        };

        let mut attempts = 0;
        let mut last_error = None;

        while attempts <= self.max_retries {
            match self.http_client.post(&url).json(&request_body).send().await {
                Ok(response) => {
                    let status = response.status();

                    if !status.is_success() {
                        let error_text = response.text().await.unwrap_or_default();

                        if status.as_u16() == 404 && error_text.contains("not found") {
                            return Err(OllamaError::model_not_found(model));
                        }

                        // Retry on 5xx server errors
                        if status.is_server_error() {
                            last_error = Some(OllamaError::http(format!(
                                "HTTP error {}: {}",
                                status, error_text
                            )));
                            attempts += 1;

                            if attempts <= self.max_retries {
                                tokio::time::sleep(Duration::from_secs(2u64.pow(attempts - 1)))
                                    .await;
                                continue;
                            }
                            break;
                        }

                        // Don't retry on 4xx client errors
                        return Err(OllamaError::http(format!(
                            "HTTP error {}: {}",
                            status, error_text
                        )));
                    }

                    let generate_response: GenerateResponse = response.json().await?;
                    return Ok(generate_response.response);
                }
                Err(e) => {
                    last_error = Some(OllamaError::from(e));
                    attempts += 1;

                    if attempts <= self.max_retries
                        && last_error.as_ref().is_some_and(|e| e.is_retryable())
                    {
                        tokio::time::sleep(Duration::from_secs(2u64.pow(attempts - 1))).await;
                        continue;
                    }
                    break;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| OllamaError::http("Generation failed")))
    }

    /// Generates text with streaming (placeholder for future implementation)
    ///
    /// # Arguments
    ///
    /// * `model` - Name of the model to use
    /// * `prompt` - The prompt to generate from
    /// * `options` - Optional generation options
    ///
    /// # Returns
    ///
    /// Returns the generated text or an error
    ///
    /// # Errors
    ///
    /// Returns an error if generation fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::new(OllamaConfig::default())?;
    /// let response = client.generate_stream("llama3.2:3b", "Hello", None).await?;
    /// println!("Generated: {}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_stream(
        &self,
        model: &str,
        prompt: &str,
        options: Option<(Option<f32>, Option<usize>)>,
    ) -> Result<String, OllamaError> {
        // For now, delegate to non-streaming version
        // TODO: Implement actual streaming with callback or channel
        self.generate(model, prompt, options).await
    }

    /// Checks if the Ollama service is healthy
    ///
    /// # Returns
    ///
    /// Returns true if the service is healthy, false otherwise
    ///
    /// # Errors
    ///
    /// Returns an error if the health check fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::new(OllamaConfig::default())?;
    /// let is_healthy = client.health_check().await?;
    /// assert!(is_healthy);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health_check(&self) -> Result<bool, OllamaError> {
        let url = format!("{}/api/tags", self.base_url);

        match self.http_client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                if e.is_connect() {
                    Ok(false)
                } else {
                    Err(OllamaError::from(e))
                }
            }
        }
    }

    /// Returns the base URL of the client
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    ///
    /// let client = OllamaClient::new(OllamaConfig::default()).unwrap();
    /// assert_eq!(client.base_url(), "http://localhost:11434");
    /// ```
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the configured timeout
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig};
    /// use std::time::Duration;
    ///
    /// let client = OllamaClient::new(OllamaConfig::default()).unwrap();
    /// assert_eq!(client.timeout(), Duration::from_secs(120));
    /// ```
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_with_valid_config() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation_with_invalid_config() {
        let config = OllamaConfig::new().with_base_url("");
        let client = OllamaClient::new(config);
        assert!(client.is_err());
    }

    #[test]
    fn test_client_creation_with_custom_base_url() {
        let config = OllamaConfig::new()
            .with_base_url("http://custom-host:8080")
            .with_timeout_seconds(60);
        let client = OllamaClient::new(config).unwrap();
        assert_eq!(client.base_url(), "http://custom-host:8080");
        assert_eq!(client.timeout(), Duration::from_secs(60));
    }

    #[test]
    fn test_client_base_url_default() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        assert_eq!(client.base_url(), "http://localhost:11434");
    }

    #[test]
    fn test_client_timeout_default() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(120));
    }

    #[test]
    fn test_client_timeout_custom() {
        let config = OllamaConfig::new().with_timeout_seconds(300);
        let client = OllamaClient::new(config).unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(300));
    }

    #[test]
    fn test_generate_request_serialization_full() {
        let request = GenerateRequest {
            model: "llama3.2:3b".to_string(),
            prompt: "Hello".to_string(),
            stream: Some(false),
            options: Some(GenerateOptions {
                temperature: Some(0.7),
                num_predict: Some(100),
            }),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("llama3.2:3b"));
        assert!(json.contains("Hello"));
        assert!(json.contains("0.7"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_generate_request_serialization_minimal() {
        let request = GenerateRequest {
            model: "llama3.2:3b".to_string(),
            prompt: "Test".to_string(),
            stream: None,
            options: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("llama3.2:3b"));
        assert!(json.contains("Test"));
        assert!(!json.contains("stream"));
        assert!(!json.contains("options"));
    }

    #[test]
    fn test_generate_request_serialization_with_temperature_only() {
        let request = GenerateRequest {
            model: "llama3.2:3b".to_string(),
            prompt: "Test".to_string(),
            stream: Some(false),
            options: Some(GenerateOptions {
                temperature: Some(0.8),
                num_predict: None,
            }),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("0.8"));
    }

    #[test]
    fn test_generate_response_deserialization_complete() {
        let json = r#"{"response":"test","done":true,"model":"llama3.2:3b"}"#;
        let response: GenerateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.response, "test");
        assert!(response.done);
        assert_eq!(response.model, "llama3.2:3b");
    }

    #[test]
    fn test_generate_response_deserialization_partial() {
        let json = r#"{"response":"partial"}"#;
        let response: GenerateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.response, "partial");
        assert!(!response.done);
    }

    #[test]
    fn test_generate_response_deserialization_empty() {
        let json = r#"{"response":"","done":true,"model":""}"#;
        let response: GenerateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.response, "");
        assert!(response.done);
    }

    #[test]
    fn test_list_models_response_deserialization_single() {
        let json = r#"{"models":[{"name":"llama3.2:3b","size":3825819519}]}"#;
        let response: ListModelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.models.len(), 1);
        assert_eq!(response.models[0].name, "llama3.2:3b");
        assert_eq!(response.models[0].size, Some(3825819519));
    }

    #[test]
    fn test_list_models_response_deserialization_multiple() {
        let json =
            r#"{"models":[{"name":"llama3.2:3b","size":1000},{"name":"codellama","size":2000}]}"#;
        let response: ListModelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.models.len(), 2);
        assert_eq!(response.models[0].name, "llama3.2:3b");
        assert_eq!(response.models[1].name, "codellama");
    }

    #[test]
    fn test_list_models_response_deserialization_empty() {
        let json = r#"{"models":[]}"#;
        let response: ListModelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.models.len(), 0);
    }

    #[test]
    fn test_list_models_response_deserialization_with_metadata() {
        let json = r#"{"models":[{"name":"llama3.2:3b","size":1000,"modified_at":"2024-01-01"}]}"#;
        let response: ListModelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.models.len(), 1);
        assert_eq!(
            response.models[0].modified_at,
            Some("2024-01-01".to_string())
        );
    }

    #[test]
    fn test_model_info_serialization_full() {
        let model = ModelInfo {
            name: "llama3.2:3b".to_string(),
            size: Some(1000),
            modified_at: Some("2024-01-01".to_string()),
        };

        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("llama3.2:3b"));
        assert!(json.contains("1000"));
        assert!(json.contains("2024-01-01"));
    }

    #[test]
    fn test_model_info_serialization_minimal() {
        let model = ModelInfo {
            name: "codellama".to_string(),
            size: None,
            modified_at: None,
        };

        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("codellama"));
        assert!(!json.contains("size"));
        assert!(!json.contains("modified_at"));
    }

    #[test]
    fn test_model_info_deserialization() {
        let json = r#"{"name":"llama3.2:3b","size":1000}"#;
        let model: ModelInfo = serde_json::from_str(json).unwrap();
        assert_eq!(model.name, "llama3.2:3b");
        assert_eq!(model.size, Some(1000));
    }

    #[test]
    fn test_client_clone_preserves_config() {
        let config = OllamaConfig::default();
        let client1 = OllamaClient::new(config).unwrap();
        let client2 = client1.clone();
        assert_eq!(client1.base_url(), client2.base_url());
        assert_eq!(client1.timeout(), client2.timeout());
    }

    #[test]
    fn test_client_debug_format() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("OllamaClient"));
        assert!(debug_str.contains("base_url"));
    }

    #[test]
    fn test_generate_options_with_both_parameters() {
        let options = GenerateOptions {
            temperature: Some(0.5),
            num_predict: Some(200),
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains("0.5"));
        assert!(json.contains("200"));
    }

    #[test]
    fn test_generate_options_with_temperature_only() {
        let options = GenerateOptions {
            temperature: Some(0.9),
            num_predict: None,
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains("0.9"));
        assert!(!json.contains("num_predict"));
    }

    #[test]
    fn test_generate_options_with_num_predict_only() {
        let options = GenerateOptions {
            temperature: None,
            num_predict: Some(500),
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains("500"));
        assert!(!json.contains("temperature"));
    }
}
