// SPDX-FileCopyrightText: 2025 Brett Smith <xbcsmith@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Ollama-based plan generator implementation
//!
//! This module implements the PlanGenerator trait using Ollama LLM services.
//! It converts architecture documents into structured implementation plans.
//!
//! # Examples
//!
//! ```rust,no_run
//! use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};
//! use xzagentz::domain::planning::{PlanGenerator, ArchitectureDocument, PlanOptions};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = OllamaConfig::default();
//! let client = OllamaClient::new(config)?;
//! let generator = OllamaPlanGenerator::new(client);
//!
//! let architecture = ArchitectureDocument::new("My Project");
//! let options = PlanOptions::default();
//! let plan = generator.generate_plan(&architecture, &options)?;
//! println!("Generated plan: {}", plan.title());
//! # Ok(())
//! # }
//! ```

use super::client::OllamaClient;
use super::config::OllamaConfig;
use super::error::OllamaError;
use crate::domain::planning::{
    ArchitectureDocument, Phase, PhaseId, Plan, PlanGenerator, PlanMetadata, PlanOptions, Task,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Ollama-based plan generator
///
/// Implements the PlanGenerator trait using Ollama LLM services to convert
/// architecture documents into structured implementation plans.
///
/// # Examples
///
/// ```rust,no_run
/// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = OllamaConfig::default();
/// let client = OllamaClient::new(config)?;
/// let generator = OllamaPlanGenerator::new(client);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct OllamaPlanGenerator {
    /// Ollama HTTP client
    client: OllamaClient,
    /// Configuration
    config: OllamaConfig,
}

/// Structured response from LLM for plan generation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlanGenerationResponse {
    title: String,
    description: String,
    phases: Vec<PhaseResponse>,
}

/// Phase information from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhaseResponse {
    id: String,
    name: String,
    description: String,
    tasks: Vec<TaskResponse>,
    dependencies: Vec<String>,
    estimated_duration: String,
}

/// Task information from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskResponse {
    name: String,
    description: String,
    acceptance_criteria: Vec<String>,
    components: Vec<String>,
}

impl OllamaPlanGenerator {
    /// Creates a new Ollama plan generator
    ///
    /// # Arguments
    ///
    /// * `client` - Configured Ollama HTTP client
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = OllamaConfig::default();
    /// let client = OllamaClient::new(config)?;
    /// let generator = OllamaPlanGenerator::new(client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: OllamaClient) -> Self {
        let config = OllamaConfig::default();
        Self { client, config }
    }

    /// Creates a new generator with custom configuration
    ///
    /// # Arguments
    ///
    /// * `client` - Configured Ollama HTTP client
    /// * `config` - Ollama configuration
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = OllamaConfig::default().with_temperature(0.8);
    /// let client = OllamaClient::new(config.clone())?;
    /// let generator = OllamaPlanGenerator::with_config(client, config);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_config(client: OllamaClient, config: OllamaConfig) -> Self {
        Self { client, config }
    }

    /// Builds the prompt for plan generation
    ///
    /// Creates a structured prompt that guides the LLM to generate
    /// a well-formed implementation plan from the architecture document.
    fn build_prompt(&self, architecture: &ArchitectureDocument, options: &PlanOptions) -> String {
        let num_phases = options.num_phases().unwrap_or(5);

        let requirements = architecture
            .requirements()
            .iter()
            .map(|r| {
                if let Some(priority) = r.priority() {
                    format!("- {} (Priority: {})", r.description(), priority)
                } else {
                    format!("- {}", r.description())
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let components = architecture
            .components()
            .iter()
            .map(|c| {
                format!(
                    "- {}: {}",
                    c.name(),
                    c.description().unwrap_or("No description")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"You are an expert software architect and project planner. Your task is to create a detailed implementation plan based on the provided architecture document.

# Architecture Document

**Title**: {}

**Description**: {}

## Requirements
{}

## Components
{}

# Task

Create a detailed implementation plan with exactly {} phases. Each phase should:
1. Have a clear name and description
2. List specific tasks with acceptance criteria
3. Define dependencies on previous phases
4. Estimate duration

# Output Format

Respond with a JSON object matching this structure:

{{
  "title": "Implementation Plan for [Project Name]",
  "description": "Brief overview of the implementation approach",
  "phases": [
    {{
      "id": "phase-1",
      "name": "Phase name",
      "description": "What this phase accomplishes",
      "tasks": [
        {{
          "name": "Task name",
          "description": "Task description",
          "acceptance_criteria": ["Criterion 1", "Criterion 2"],
          "components": ["Component 1", "Component 2"]
        }}
      ],
      "dependencies": [],
      "estimated_duration": "X days/weeks"
    }}
  ]
}}

Ensure the plan is:
- Practical and implementable
- Properly sequenced (later phases can depend on earlier ones)
- Comprehensive (covers all requirements and components)
- Well-structured (each phase is cohesive)

Generate the implementation plan now."#,
            architecture.title(),
            architecture
                .sections()
                .first()
                .map(|s| s.content())
                .unwrap_or(""),
            requirements,
            components,
            num_phases
        )
    }

    /// Parses the LLM response into a Plan
    ///
    /// Attempts to extract JSON from the response and convert it into
    /// domain Plan objects with proper validation.
    fn parse_response(
        &self,
        response: &str,
        architecture: &ArchitectureDocument,
        model_used: &str,
    ) -> Result<Plan, OllamaError> {
        // Try to extract JSON from response (LLMs sometimes add extra text)
        let json_str = self.extract_json(response)?;

        // Parse JSON into structured response
        let plan_response: PlanGenerationResponse = serde_json::from_str(&json_str)
            .map_err(|e| OllamaError::generation(format!("Failed to parse plan JSON: {}", e)))?;

        // Convert to domain objects
        let phases = plan_response
            .phases
            .into_iter()
            .map(|p| self.convert_phase(p))
            .collect::<Result<Vec<_>, _>>()?;

        let metadata = PlanMetadata::with_version(
            Utc::now(),
            model_used.to_string(),
            PathBuf::from(architecture.title()),
            "1.0",
        );

        let plan = Plan::new(
            plan_response.title,
            plan_response.description,
            phases,
            metadata,
        );

        // Validate the plan
        plan.validate()
            .map_err(|e| OllamaError::generation(format!("Generated invalid plan: {:?}", e)))?;

        Ok(plan)
    }

    /// Extracts JSON from LLM response
    ///
    /// LLMs sometimes wrap JSON in markdown code blocks or add extra text.
    /// This method attempts to extract just the JSON portion.
    fn extract_json(&self, response: &str) -> Result<String, OllamaError> {
        // Try to find JSON in code blocks first
        if let Some(start) = response.find("```json") {
            let search_start = start + 7; // length of "```json"
            let after_marker = &response[search_start..];
            if let Some(end_pos) = after_marker.find("```") {
                return Ok(after_marker[..end_pos].trim().to_string());
            }
        }

        // Try to find plain JSON (starts with { and ends with })
        if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                if end > start {
                    return Ok(response[start..=end].trim().to_string());
                }
            }
        }

        // If no JSON found, return error
        Err(OllamaError::invalid_response(
            "No valid JSON found in LLM response",
        ))
    }

    /// Converts a PhaseResponse to a domain Phase
    fn convert_phase(&self, phase: PhaseResponse) -> Result<Phase, OllamaError> {
        let phase_id = PhaseId::from(phase.id.as_str());

        let tasks = phase
            .tasks
            .into_iter()
            .map(|t| self.convert_task(t))
            .collect::<Result<Vec<_>, _>>()?;

        let dependencies = phase
            .dependencies
            .into_iter()
            .map(|d| PhaseId::from(d.as_str()))
            .collect::<Vec<_>>();

        Ok(Phase::new(phase_id, phase.name, phase.description)
            .with_tasks(tasks)
            .with_dependencies(dependencies)
            .with_estimated_duration(phase.estimated_duration))
    }

    /// Converts a TaskResponse to a domain Task
    fn convert_task(&self, task: TaskResponse) -> Result<Task, OllamaError> {
        Ok(Task::new(task.name, task.description)
            .with_acceptance_criteria(task.acceptance_criteria)
            .with_components(task.components))
    }
}

impl PlanGenerator for OllamaPlanGenerator {
    type Error = OllamaError;

    fn generate_plan(
        &self,
        architecture: &ArchitectureDocument,
        options: &PlanOptions,
    ) -> Result<Plan, Self::Error> {
        // This needs to be async but the trait is sync
        // We'll use tokio::runtime::Handle to bridge sync/async
        let runtime = tokio::runtime::Handle::try_current().or_else(|_| {
            tokio::runtime::Runtime::new()
                .map(|rt| rt.handle().clone())
                .map_err(|e| OllamaError::generation(format!("Failed to create runtime: {}", e)))
        })?;

        runtime.block_on(async {
            let model = options
                .model()
                .unwrap_or(self.config.default_model())
                .to_string();

            let prompt = self.build_prompt(architecture, options);

            let temperature = options.temperature().or(Some(self.config.temperature()));
            let max_tokens = options.max_tokens().or(Some(self.config.max_tokens()));

            let response = self
                .client
                .generate(&model, &prompt, Some((temperature, max_tokens)))
                .await?;

            self.parse_response(&response, architecture, &model)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::planning::{Component, Requirement, Section};

    fn create_test_architecture() -> ArchitectureDocument {
        let mut arch = ArchitectureDocument::new("Test Project");
        arch.add_section(Section::new("Overview", 1).with_content("Test architecture"));
        arch.add_component(
            Component::new("Component1", "component-1").with_description("Test component"),
        );
        arch.add_requirement(Requirement::new("REQ-1", "Test requirement").with_priority("High"));
        arch
    }

    #[test]
    fn test_build_prompt() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config.clone()).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        let architecture = create_test_architecture();
        let options = PlanOptions::default().with_num_phases(3);

        let prompt = generator.build_prompt(&architecture, &options);

        assert!(prompt.contains("Test Project"));
        assert!(prompt.contains("exactly 3 phases"));
        assert!(prompt.contains("Component1"));
        assert!(prompt.contains("Test requirement"));
    }

    #[test]
    fn test_extract_json_from_code_block() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        let response = r#"Here is the plan:
```json
{"title": "Test"}
```
That's it!"#;

        let json = generator.extract_json(response).unwrap();
        assert_eq!(json, r#"{"title": "Test"}"#);
    }

    #[test]
    fn test_extract_json_plain() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        let response = r#"Some text {"title": "Test"} more text"#;

        let json = generator.extract_json(response).unwrap();
        assert_eq!(json, r#"{"title": "Test"}"#);
    }

    #[test]
    fn test_extract_json_no_json() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        let response = "No JSON here at all";

        let result = generator.extract_json(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_task() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        let task_response = TaskResponse {
            name: "Test Task".to_string(),
            description: "Test description".to_string(),
            acceptance_criteria: vec!["Criterion 1".to_string()],
            components: vec!["Component 1".to_string()],
        };

        let task = generator.convert_task(task_response).unwrap();
        assert_eq!(task.name(), "Test Task");
        assert_eq!(task.description(), "Test description");
    }

    #[test]
    fn test_convert_phase() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        let phase_response = PhaseResponse {
            id: "phase-1".to_string(),
            name: "Test Phase".to_string(),
            description: "Test description".to_string(),
            tasks: vec![TaskResponse {
                name: "Task 1".to_string(),
                description: "Task desc".to_string(),
                acceptance_criteria: vec![],
                components: vec![],
            }],
            dependencies: vec![],
            estimated_duration: "1 week".to_string(),
        };

        let phase = generator.convert_phase(phase_response).unwrap();
        assert_eq!(phase.name(), "Test Phase");
        assert_eq!(phase.tasks().len(), 1);
    }

    #[test]
    fn test_generator_creation() {
        let config = OllamaConfig::default();
        let client = OllamaClient::new(config).unwrap();
        let generator = OllamaPlanGenerator::new(client);

        assert!(format!("{:?}", generator).contains("OllamaPlanGenerator"));
    }

    #[test]
    fn test_generator_with_config() {
        let config = OllamaConfig::default().with_temperature(0.9);
        let client = OllamaClient::new(config.clone()).unwrap();
        let generator = OllamaPlanGenerator::with_config(client, config);

        assert!(format!("{:?}", generator).contains("OllamaPlanGenerator"));
    }
}
