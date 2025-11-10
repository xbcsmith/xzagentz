// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Configuration for Ollama infrastructure
//!
//! This module defines configuration structures for Ollama HTTP client
//! and plan generation settings.
//!
//! # Configuration
//!
//! - [`OllamaConfig`]: Main configuration for Ollama client
//!
//! # Examples
//!
//! ```rust
//! use xzagentz::infrastructure::ollama::OllamaConfig;
//!
//! let config = OllamaConfig::default();
//! assert_eq!(config.base_url(), "http://localhost:11434");
//! assert_eq!(config.default_model(), "llama3.2:3b");
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for Ollama client
///
/// This structure contains all configuration options for connecting to
/// and interacting with an Ollama service.
///
/// # Examples
///
/// ```rust
/// use xzagentz::infrastructure::ollama::OllamaConfig;
///
/// // Use default configuration
/// let config = OllamaConfig::default();
///
/// // Create custom configuration
/// let config = OllamaConfig::new()
///     .with_base_url("http://my-ollama:11434")
///     .with_default_model("mistral")
///     .with_timeout_seconds(60)
///     .with_max_retries(5);
///
/// assert_eq!(config.default_model(), "mistral");
/// assert_eq!(config.max_retries(), 5);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// Base URL of the Ollama service
    base_url: String,

    /// Default model to use for generation
    default_model: String,

    /// Request timeout in seconds
    timeout_seconds: u64,

    /// Maximum number of retry attempts
    max_retries: u32,

    /// Temperature for generation (0.0 to 1.0)
    #[serde(default = "default_temperature")]
    temperature: f32,

    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    max_tokens: usize,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> usize {
    8192
}

impl OllamaConfig {
    /// Creates a new configuration with default values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new();
    /// assert_eq!(config.base_url(), "http://localhost:11434");
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new()
    ///     .with_base_url("http://custom:11434");
    ///
    /// assert_eq!(config.base_url(), "http://custom:11434");
    /// ```
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sets the default model (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new()
    ///     .with_default_model("mistral");
    ///
    /// assert_eq!(config.default_model(), "mistral");
    /// ```
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    /// Sets the timeout in seconds (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new()
    ///     .with_timeout_seconds(60);
    ///
    /// assert_eq!(config.timeout_seconds(), 60);
    /// ```
    pub fn with_timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Sets the maximum retry attempts (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new()
    ///     .with_max_retries(5);
    ///
    /// assert_eq!(config.max_retries(), 5);
    /// ```
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Sets the temperature (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new()
    ///     .with_temperature(0.8);
    ///
    /// assert_eq!(config.temperature(), 0.8);
    /// ```
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }

    /// Sets the maximum tokens (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::new()
    ///     .with_max_tokens(4096);
    ///
    /// assert_eq!(config.max_tokens(), 4096);
    /// ```
    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = tokens;
        self
    }

    /// Returns the base URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.base_url(), "http://localhost:11434");
    /// ```
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the default model
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.default_model(), "llama3.2:3b");
    /// ```
    pub fn default_model(&self) -> &str {
        &self.default_model
    }

    /// Returns the timeout in seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.timeout_seconds(), 120);
    /// ```
    pub fn timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }

    /// Returns the timeout as a Duration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    /// use std::time::Duration;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.timeout_duration(), Duration::from_secs(120));
    /// ```
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }

    /// Returns the maximum retry attempts
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.max_retries(), 3);
    /// ```
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Returns the temperature
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.temperature(), 0.7);
    /// ```
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    /// Returns the maximum tokens
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.max_tokens(), 8192);
    /// ```
    pub fn max_tokens(&self) -> usize {
        self.max_tokens
    }

    /// Validates the configuration
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if valid, or an error message if invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert!(config.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.base_url.is_empty() {
            return Err("Base URL cannot be empty".to_string());
        }

        if self.default_model.is_empty() {
            return Err("Default model cannot be empty".to_string());
        }

        if self.timeout_seconds == 0 {
            return Err("Timeout must be greater than 0".to_string());
        }

        if !(0.0..=1.0).contains(&self.temperature) {
            return Err("Temperature must be between 0.0 and 1.0".to_string());
        }

        if self.max_tokens == 0 {
            return Err("Max tokens must be greater than 0".to_string());
        }

        Ok(())
    }
}

impl Default for OllamaConfig {
    /// Creates default configuration
    ///
    /// # Defaults
    ///
    /// - base_url: "http://localhost:11434"
    /// - default_model: "llama3.2:3b"
    /// - timeout_seconds: 120
    /// - max_retries: 3
    /// - temperature: 0.7
    /// - max_tokens: 8192
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xzagentz::infrastructure::ollama::OllamaConfig;
    ///
    /// let config = OllamaConfig::default();
    /// assert_eq!(config.base_url(), "http://localhost:11434");
    /// assert_eq!(config.default_model(), "llama3.2:3b");
    /// assert_eq!(config.timeout_seconds(), 120);
    /// assert_eq!(config.max_retries(), 3);
    /// ```
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            default_model: "llama3.2:3b".to_string(),
            timeout_seconds: 120,
            max_retries: 3,
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = OllamaConfig::default();
        assert_eq!(config.base_url(), "http://localhost:11434");
        assert_eq!(config.default_model(), "llama3.2:3b");
        assert_eq!(config.timeout_seconds(), 120);
        assert_eq!(config.max_retries(), 3);
        assert_eq!(config.temperature(), 0.7);
        assert_eq!(config.max_tokens(), 8192);
    }

    #[test]
    fn test_new_config() {
        let config = OllamaConfig::new();
        assert_eq!(config, OllamaConfig::default());
    }

    #[test]
    fn test_with_base_url() {
        let config = OllamaConfig::new().with_base_url("http://custom:11434");
        assert_eq!(config.base_url(), "http://custom:11434");
    }

    #[test]
    fn test_with_default_model() {
        let config = OllamaConfig::new().with_default_model("mistral");
        assert_eq!(config.default_model(), "mistral");
    }

    #[test]
    fn test_with_timeout_seconds() {
        let config = OllamaConfig::new().with_timeout_seconds(60);
        assert_eq!(config.timeout_seconds(), 60);
    }

    #[test]
    fn test_with_max_retries() {
        let config = OllamaConfig::new().with_max_retries(5);
        assert_eq!(config.max_retries(), 5);
    }

    #[test]
    fn test_with_temperature() {
        let config = OllamaConfig::new().with_temperature(0.8);
        assert_eq!(config.temperature(), 0.8);
    }

    #[test]
    fn test_with_temperature_clamping() {
        let config1 = OllamaConfig::new().with_temperature(1.5);
        assert_eq!(config1.temperature(), 1.0);

        let config2 = OllamaConfig::new().with_temperature(-0.5);
        assert_eq!(config2.temperature(), 0.0);
    }

    #[test]
    fn test_with_max_tokens() {
        let config = OllamaConfig::new().with_max_tokens(4096);
        assert_eq!(config.max_tokens(), 4096);
    }

    #[test]
    fn test_timeout_duration() {
        let config = OllamaConfig::new().with_timeout_seconds(30);
        assert_eq!(config.timeout_duration(), Duration::from_secs(30));
    }

    #[test]
    fn test_builder_pattern() {
        let config = OllamaConfig::new()
            .with_base_url("http://test:11434")
            .with_default_model("codellama")
            .with_timeout_seconds(90)
            .with_max_retries(5)
            .with_temperature(0.9)
            .with_max_tokens(16384);

        assert_eq!(config.base_url(), "http://test:11434");
        assert_eq!(config.default_model(), "codellama");
        assert_eq!(config.timeout_seconds(), 90);
        assert_eq!(config.max_retries(), 5);
        assert_eq!(config.temperature(), 0.9);
        assert_eq!(config.max_tokens(), 16384);
    }

    #[test]
    fn test_validate_valid_config() {
        let config = OllamaConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_base_url() {
        let config = OllamaConfig::new().with_base_url("");
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Base URL"));
    }

    #[test]
    fn test_validate_empty_model() {
        let config = OllamaConfig::new().with_default_model("");
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("model"));
    }

    #[test]
    fn test_validate_zero_timeout() {
        let config = OllamaConfig::new().with_timeout_seconds(0);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Timeout"));
    }

    #[test]
    fn test_validate_zero_max_tokens() {
        let config = OllamaConfig::new().with_max_tokens(0);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Max tokens"));
    }

    #[test]
    fn test_clone() {
        let config1 = OllamaConfig::new().with_default_model("mistral");
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_debug() {
        let config = OllamaConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("OllamaConfig"));
    }

    #[test]
    fn test_serialization() {
        let config = OllamaConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("llama3.2:3b"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "base_url": "http://localhost:11434",
            "default_model": "llama3.2:3b",
            "timeout_seconds": 120,
            "max_retries": 3,
            "temperature": 0.7,
            "max_tokens": 8192
        }"#;
        let config: OllamaConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_model(), "llama3.2:3b");
    }

    #[test]
    fn test_equality() {
        let config1 = OllamaConfig::default();
        let config2 = OllamaConfig::default();
        assert_eq!(config1, config2);

        let config3 = OllamaConfig::new().with_default_model("mistral");
        assert_ne!(config1, config3);
    }
}
