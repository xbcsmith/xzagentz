//! Application configuration module
//!
//! Provides configuration management for xzagentz application settings,
//! including Ollama integration, planning defaults, and interactive mode preferences.
//!
//! Configuration is loaded from `~/.config/xzagentz/config.yaml` if present,
//! with sensible defaults used when the file does not exist.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Configuration error types
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Failed to read configuration file
    #[error("Failed to read config file: {0}")]
    ReadError(String),

    /// Failed to parse YAML configuration
    #[error("Failed to parse config YAML: {0}")]
    ParseError(String),

    /// Failed to determine config directory
    #[error("Failed to determine config directory")]
    ConfigDirError,

    /// Configuration validation failed
    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}

/// Result type for configuration operations
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Application configuration
///
/// Contains all configuration sections for the xzagentz application.
/// Can be loaded from YAML file or constructed with defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Ollama service configuration
    #[serde(default)]
    pub ollama: OllamaConfig,

    /// Planning operation defaults
    #[serde(default)]
    pub planning: PlanningConfig,

    /// Interactive mode preferences
    #[serde(default)]
    pub interactive: InteractiveConfig,

    /// Architecture generation configuration
    #[serde(default)]
    pub architecture: ArchitectureConfig,
}

/// Ollama service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// Base URL for Ollama service
    pub base_url: String,

    /// Default model to use for generation
    pub default_model: String,

    /// Request timeout in seconds
    pub timeout_seconds: u64,

    /// Maximum number of retry attempts
    pub max_retries: u32,
}

/// Planning operation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningConfig {
    /// Default number of phases to generate
    pub default_phases: Option<usize>,

    /// Default output directory for plans
    pub default_output_dir: String,

    /// Output format (markdown, json, yaml)
    pub output_format: String,
}

/// Interactive mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfig {
    /// Enable colored output
    pub enable_colors: bool,

    /// Show progress indicators
    pub show_progress: bool,

    /// Confirm before saving files
    pub confirm_before_save: bool,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            default_model: "llama3".to_string(),
            timeout_seconds: 300,
            max_retries: 3,
        }
    }
}

impl Default for PlanningConfig {
    fn default() -> Self {
        Self {
            default_phases: Some(7),
            default_output_dir: "docs/plans".to_string(),
            output_format: "markdown".to_string(),
        }
    }
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            enable_colors: true,
            show_progress: true,
            confirm_before_save: true,
        }
    }
}

/// Architecture generation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArchitectureConfig {
    /// Ollama-specific settings for architecture generation
    pub ollama: ArchitectureOllamaConfig,

    /// Default generation settings
    pub generation: GenerationConfig,

    /// Output settings
    pub output: ArchitectureOutputConfig,

    /// Template settings
    pub templates: TemplateConfig,

    /// Interactive mode settings for architecture commands
    pub interactive: ArchitectureInteractiveConfig,
}

/// Ollama configuration specific to architecture generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureOllamaConfig {
    /// Base URL for Ollama service (inherits from global if not set)
    pub base_url: Option<String>,

    /// Default model for architecture generation
    pub default_model: String,

    /// Timeout in seconds for architecture generation requests
    pub timeout_seconds: u64,

    /// Maximum number of retry attempts
    pub max_retries: u32,
}

/// Generation defaults configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Default architecture pattern
    pub default_pattern: String,

    /// Default complexity level
    pub default_complexity: String,

    /// Include deployment architecture by default
    pub include_deployment: bool,

    /// Include quality attributes by default
    pub include_quality_attributes: bool,

    /// Maximum number of components to generate
    pub max_components: usize,
}

/// Architecture output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureOutputConfig {
    /// Default output directory for architecture documents
    pub default_directory: String,

    /// Output format (markdown, json, yaml)
    pub format: String,

    /// Create backup before overwriting
    pub create_backup: bool,
}

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Directory containing architecture templates
    pub directory: String,

    /// Automatically create default templates if missing
    pub auto_create_defaults: bool,
}

/// Interactive mode configuration for architecture commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureInteractiveConfig {
    /// Enable colored output
    pub enable_colors: bool,

    /// Show progress during generation
    pub show_progress: bool,

    /// Confirm before saving
    pub confirm_before_save: bool,

    /// Show token usage statistics
    pub show_token_usage: bool,
}

impl Default for ArchitectureOllamaConfig {
    fn default() -> Self {
        Self {
            base_url: None, // Inherits from global ollama.base_url
            default_model: "llama3".to_string(),
            timeout_seconds: 600, // Longer timeout for architecture generation
            max_retries: 3,
        }
    }
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            default_pattern: "layered".to_string(),
            default_complexity: "moderate".to_string(),
            include_deployment: true,
            include_quality_attributes: true,
            max_components: 20,
        }
    }
}

impl Default for ArchitectureOutputConfig {
    fn default() -> Self {
        Self {
            default_directory: "docs/architecture".to_string(),
            format: "markdown".to_string(),
            create_backup: true,
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            directory: "templates/architecture".to_string(),
            auto_create_defaults: true,
        }
    }
}

impl Default for ArchitectureInteractiveConfig {
    fn default() -> Self {
        Self {
            enable_colors: true,
            show_progress: true,
            confirm_before_save: true,
            show_token_usage: false,
        }
    }
}

impl AppConfig {
    /// Loads configuration from the default location
    ///
    /// Attempts to load from `~/.config/xzagentz/config.yaml`.
    /// If the file does not exist, returns default configuration.
    ///
    /// # Returns
    ///
    /// Returns the loaded or default configuration
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the file exists but cannot be read or parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::config::AppConfig;
    ///
    /// let config = AppConfig::load().unwrap();
    /// assert_eq!(config.ollama.default_model, "llama3");
    /// ```
    pub fn load() -> Result<Self> {
        let config_path = Self::default_config_path()?;
        Self::load_from_path(&config_path)
    }

    /// Loads configuration from a specific path
    ///
    /// If the file does not exist, returns default configuration.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// Returns the loaded or default configuration
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the file exists but cannot be read or parsed
    pub fn load_from_path(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents =
            std::fs::read_to_string(path).map_err(|e| ConfigError::ReadError(e.to_string()))?;

        let config: Self =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        config.validate()?;
        Ok(config)
    }

    /// Saves configuration to the default location
    ///
    /// Creates the config directory if it does not exist.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the directory cannot be created or file cannot be written
    pub fn save(&self) -> Result<()> {
        let config_path = Self::default_config_path()?;
        self.save_to_path(&config_path)
    }

    /// Saves configuration to a specific path
    ///
    /// Creates parent directories if they do not exist.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where configuration should be saved
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if directories cannot be created or file cannot be written
    pub fn save_to_path(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ConfigError::ReadError(format!("Failed to create config directory: {}", e))
            })?;
        }

        let yaml = serde_yaml::to_string(self)
            .map_err(|e| ConfigError::ParseError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, yaml)
            .map_err(|e| ConfigError::ReadError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    /// Returns the default configuration file path
    ///
    /// # Returns
    ///
    /// Returns `~/.config/xzagentz/config.yaml`
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ConfigDirError` if config directory cannot be determined
    pub fn default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or(ConfigError::ConfigDirError)?;

        Ok(config_dir.join("xzagentz").join("config.yaml"))
    }

    /// Validates configuration values
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ValidationError` if any values are invalid
    fn validate(&self) -> Result<()> {
        if self.ollama.base_url.is_empty() {
            return Err(ConfigError::ValidationError(
                "ollama.base_url cannot be empty".to_string(),
            ));
        }

        if self.ollama.default_model.is_empty() {
            return Err(ConfigError::ValidationError(
                "ollama.default_model cannot be empty".to_string(),
            ));
        }

        if self.ollama.timeout_seconds == 0 {
            return Err(ConfigError::ValidationError(
                "ollama.timeout_seconds must be greater than 0".to_string(),
            ));
        }

        if !["markdown", "json", "yaml"].contains(&self.planning.output_format.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "planning.output_format must be one of: markdown, json, yaml (got: {})",
                self.planning.output_format
            )));
        }

        // Validate architecture configuration
        if self.architecture.ollama.default_model.is_empty() {
            return Err(ConfigError::ValidationError(
                "architecture.ollama.default_model cannot be empty".to_string(),
            ));
        }

        if self.architecture.ollama.timeout_seconds == 0 {
            return Err(ConfigError::ValidationError(
                "architecture.ollama.timeout_seconds must be greater than 0".to_string(),
            ));
        }

        if !["markdown", "json", "yaml"].contains(&self.architecture.output.format.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "architecture.output.format must be one of: markdown, json, yaml (got: {})",
                self.architecture.output.format
            )));
        }

        let valid_patterns = [
            "microservices",
            "monolithic",
            "event-driven",
            "layered",
            "hexagonal",
            "cqrs",
            "serverless",
        ];
        if !valid_patterns.contains(&self.architecture.generation.default_pattern.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "architecture.generation.default_pattern must be one of: {} (got: {})",
                valid_patterns.join(", "),
                self.architecture.generation.default_pattern
            )));
        }

        let valid_complexity = ["simple", "moderate", "complex", "enterprise"];
        if !valid_complexity.contains(&self.architecture.generation.default_complexity.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "architecture.generation.default_complexity must be one of: {} (got: {})",
                valid_complexity.join(", "),
                self.architecture.generation.default_complexity
            )));
        }

        if self.architecture.generation.max_components == 0 {
            return Err(ConfigError::ValidationError(
                "architecture.generation.max_components must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Creates a builder for constructing configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::config::AppConfig;
    ///
    /// let config = AppConfig::builder()
    ///     .ollama_url("http://localhost:11434")
    ///     .model("llama3")
    ///     .build();
    /// ```
    pub fn builder() -> AppConfigBuilder {
        AppConfigBuilder::default()
    }
}

/// Builder for constructing AppConfig with custom values
#[derive(Default)]
pub struct AppConfigBuilder {
    ollama: Option<OllamaConfig>,
    planning: Option<PlanningConfig>,
    interactive: Option<InteractiveConfig>,
    architecture: Option<ArchitectureConfig>,
}

impl AppConfigBuilder {
    /// Sets the Ollama base URL
    pub fn ollama_url(mut self, url: impl Into<String>) -> Self {
        let mut ollama = self.ollama.unwrap_or_default();
        ollama.base_url = url.into();
        self.ollama = Some(ollama);
        self
    }

    /// Sets the default model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        let mut ollama = self.ollama.unwrap_or_default();
        ollama.default_model = model.into();
        self.ollama = Some(ollama);
        self
    }

    /// Sets the timeout in seconds
    pub fn timeout(mut self, seconds: u64) -> Self {
        let mut ollama = self.ollama.unwrap_or_default();
        ollama.timeout_seconds = seconds;
        self.ollama = Some(ollama);
        self
    }

    /// Sets the maximum retry attempts
    pub fn max_retries(mut self, retries: u32) -> Self {
        let mut ollama = self.ollama.unwrap_or_default();
        ollama.max_retries = retries;
        self.ollama = Some(ollama);
        self
    }

    /// Sets the default output directory
    pub fn output_dir(mut self, dir: impl Into<String>) -> Self {
        let mut planning = self.planning.unwrap_or_default();
        planning.default_output_dir = dir.into();
        self.planning = Some(planning);
        self
    }

    /// Sets the output format
    pub fn output_format(mut self, format: impl Into<String>) -> Self {
        let mut planning = self.planning.unwrap_or_default();
        planning.output_format = format.into();
        self.planning = Some(planning);
        self
    }

    /// Sets whether colors are enabled
    pub fn colors(mut self, enabled: bool) -> Self {
        let mut interactive = self.interactive.unwrap_or_default();
        interactive.enable_colors = enabled;
        self.interactive = Some(interactive);
        self
    }

    /// Sets whether progress indicators are shown
    pub fn progress(mut self, enabled: bool) -> Self {
        let mut interactive = self.interactive.unwrap_or_default();
        interactive.show_progress = enabled;
        self.interactive = Some(interactive);
        self
    }

    /// Sets whether to confirm before saving
    pub fn confirm_save(mut self, enabled: bool) -> Self {
        let mut interactive = self.interactive.unwrap_or_default();
        interactive.confirm_before_save = enabled;
        self.interactive = Some(interactive);
        self
    }

    /// Sets the architecture templates directory
    pub fn templates_dir(mut self, dir: impl Into<String>) -> Self {
        let mut architecture = self.architecture.unwrap_or_default();
        architecture.templates.directory = dir.into();
        self.architecture = Some(architecture);
        self
    }

    /// Sets the architecture output directory
    pub fn architecture_output_dir(mut self, dir: impl Into<String>) -> Self {
        let mut architecture = self.architecture.unwrap_or_default();
        architecture.output.default_directory = dir.into();
        self.architecture = Some(architecture);
        self
    }

    /// Sets the default architecture pattern
    pub fn architecture_pattern(mut self, pattern: impl Into<String>) -> Self {
        let mut architecture = self.architecture.unwrap_or_default();
        architecture.generation.default_pattern = pattern.into();
        self.architecture = Some(architecture);
        self
    }

    /// Sets the architecture model
    pub fn architecture_model(mut self, model: impl Into<String>) -> Self {
        let mut architecture = self.architecture.unwrap_or_default();
        architecture.ollama.default_model = model.into();
        self.architecture = Some(architecture);
        self
    }

    /// Builds the AppConfig
    pub fn build(self) -> AppConfig {
        AppConfig {
            ollama: self.ollama.unwrap_or_default(),
            planning: self.planning.unwrap_or_default(),
            interactive: self.interactive.unwrap_or_default(),
            architecture: self.architecture.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.ollama.base_url, "http://localhost:11434");
        assert_eq!(config.ollama.default_model, "llama3");
        assert_eq!(config.ollama.timeout_seconds, 300);
        assert_eq!(config.planning.output_format, "markdown");
        assert!(config.interactive.enable_colors);
    }

    #[test]
    fn test_config_builder() {
        let config = AppConfig::builder()
            .ollama_url("http://example.com:11434")
            .model("custom-model")
            .timeout(600)
            .colors(false)
            .build();

        assert_eq!(config.ollama.base_url, "http://example.com:11434");
        assert_eq!(config.ollama.default_model, "custom-model");
        assert_eq!(config.ollama.timeout_seconds, 600);
        assert!(!config.interactive.enable_colors);
    }

    #[test]
    fn test_validation_empty_url() {
        let mut config = AppConfig::default();
        config.ollama.base_url = String::new();
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::ValidationError(_))));
    }

    #[test]
    fn test_validation_empty_model() {
        let mut config = AppConfig::default();
        config.ollama.default_model = String::new();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_zero_timeout() {
        let mut config = AppConfig::default();
        config.ollama.timeout_seconds = 0;
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_format() {
        let mut config = AppConfig::default();
        config.planning.output_format = "invalid".to_string();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_valid_formats() {
        for format in ["markdown", "json", "yaml"] {
            let mut config = AppConfig::default();
            config.planning.output_format = format.to_string();
            assert!(config.validate().is_ok());
        }
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.yaml");

        let original = AppConfig::builder()
            .ollama_url("http://test:11434")
            .model("test-model")
            .output_dir("test-output")
            .build();

        original.save_to_path(&config_path).unwrap();
        let loaded = AppConfig::load_from_path(&config_path).unwrap();

        assert_eq!(loaded.ollama.base_url, "http://test:11434");
        assert_eq!(loaded.ollama.default_model, "test-model");
        assert_eq!(loaded.planning.default_output_dir, "test-output");
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.yaml");

        let config = AppConfig::load_from_path(&config_path).unwrap();
        assert_eq!(config.ollama.base_url, "http://localhost:11434");
    }

    #[test]
    fn test_yaml_serialization() {
        let config = AppConfig::default();
        let yaml = serde_yaml::to_string(&config).unwrap();

        assert!(yaml.contains("base_url"));
        assert!(yaml.contains("default_model"));
        assert!(yaml.contains("enable_colors"));
    }

    #[test]
    fn test_yaml_deserialization() {
        let yaml = r#"
ollama:
  base_url: "http://custom:11434"
  default_model: "custom"
  timeout_seconds: 600
  max_retries: 5
planning:
  default_phases: 10
  default_output_dir: "custom/dir"
  output_format: "json"
interactive:
  enable_colors: false
  show_progress: false
  confirm_before_save: false
"#;

        let config: AppConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.ollama.base_url, "http://custom:11434");
        assert_eq!(config.planning.output_format, "json");
        assert!(!config.interactive.enable_colors);
    }

    #[test]
    fn test_architecture_config_defaults() {
        let config = ArchitectureConfig::default();
        assert_eq!(config.generation.default_pattern, "layered");
        assert_eq!(config.generation.default_complexity, "moderate");
        assert_eq!(config.templates.directory, "templates/architecture");
        assert!(config.generation.include_deployment);
        assert!(config.output.create_backup);
    }

    #[test]
    fn test_architecture_config_validation() {
        let mut config = AppConfig::default();
        config.architecture.generation.default_pattern = "invalid".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::ValidationError(_))));
    }

    #[test]
    fn test_architecture_config_builder() {
        let config = AppConfig::builder()
            .templates_dir("custom/templates")
            .architecture_output_dir("custom/arch")
            .architecture_pattern("microservices")
            .architecture_model("llama3.2")
            .build();

        assert_eq!(config.architecture.templates.directory, "custom/templates");
        assert_eq!(config.architecture.output.default_directory, "custom/arch");
        assert_eq!(
            config.architecture.generation.default_pattern,
            "microservices"
        );
        assert_eq!(config.architecture.ollama.default_model, "llama3.2");
    }

    #[test]
    fn test_architecture_complexity_validation() {
        let mut config = AppConfig::default();
        config.architecture.generation.default_complexity = "invalid".to_string();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_architecture_max_components_validation() {
        let mut config = AppConfig::default();
        config.architecture.generation.max_components = 0;
        let result = config.validate();
        assert!(result.is_err());
    }
}
