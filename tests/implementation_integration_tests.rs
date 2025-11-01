//! Integration tests for implementation command with mocked Ollama service
//!
//! This module contains integration tests that validate the implementation
//! command workflow with a mocked Ollama service using wiremock.

use std::fs;
use tempfile::TempDir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xzagentz::application::PlanningService;
use xzagentz::domain::planning::{
    ArchitectureDocument, Component, PlanOptions, PlanWriter, Requirement, Section,
};
use xzagentz::infrastructure::fileio::{MarkdownArchitectureParser, MarkdownPlanWriter};
use xzagentz::infrastructure::ollama::{OllamaClient, OllamaConfig, OllamaPlanGenerator};

/// Helper function to create a test architecture document
fn create_test_architecture_document() -> ArchitectureDocument {
    let mut arch = ArchitectureDocument::new("Test Project Architecture");

    arch.add_section(
        Section::new("Overview", 1)
            .with_content("This is a test architecture document for integration testing purposes."),
    );

    arch.add_section(
        Section::new("Requirements", 2)
            .with_content("The system must meet the following requirements:"),
    );

    arch.add_component(
        Component::new("API Layer", "api-layer")
            .with_description("Handles HTTP requests and responses"),
    );

    arch.add_component(
        Component::new("Business Logic", "business-logic")
            .with_description("Core business rules and processing"),
    );

    arch.add_component(
        Component::new("Data Layer", "data-layer")
            .with_description("Database access and persistence"),
    );

    arch.add_requirement(
        Requirement::new("REQ-1", "System must handle 1000 requests per second")
            .with_priority("High"),
    );

    arch.add_requirement(
        Requirement::new("REQ-2", "System must have 99.9% uptime").with_priority("High"),
    );

    arch.add_requirement(
        Requirement::new(
            "REQ-3",
            "System must support multiple authentication methods",
        )
        .with_priority("Medium"),
    );

    arch
}

/// Helper function to create a valid plan JSON response
fn create_valid_plan_json_response() -> String {
    serde_json::json!({
        "title": "Implementation Plan for Test Project",
        "description": "A comprehensive implementation plan generated from the architecture",
        "phases": [
            {
                "id": "phase-1",
                "name": "Foundation Setup",
                "description": "Set up project structure and basic infrastructure",
                "tasks": [
                    {
                        "name": "Initialize project",
                        "description": "Create project structure and dependencies",
                        "acceptance_criteria": [
                            "Project structure created",
                            "Dependencies configured"
                        ],
                        "components": ["api-layer"]
                    }
                ],
                "dependencies": [],
                "estimated_duration": "1 week"
            },
            {
                "id": "phase-2",
                "name": "Core Implementation",
                "description": "Implement core business logic",
                "tasks": [
                    {
                        "name": "Implement business logic",
                        "description": "Core processing implementation",
                        "acceptance_criteria": [
                            "Business rules implemented",
                            "Unit tests passing"
                        ],
                        "components": ["business-logic"]
                    }
                ],
                "dependencies": ["phase-1"],
                "estimated_duration": "2 weeks"
            },
            {
                "id": "phase-3",
                "name": "Data Layer",
                "description": "Implement data persistence",
                "tasks": [
                    {
                        "name": "Set up database",
                        "description": "Configure and implement data layer",
                        "acceptance_criteria": [
                            "Database schema created",
                            "CRUD operations working"
                        ],
                        "components": ["data-layer"]
                    }
                ],
                "dependencies": ["phase-1"],
                "estimated_duration": "1 week"
            }
        ]
    })
    .to_string()
}

#[tokio::test]
async fn test_implementation_command_end_to_end_with_mocked_ollama() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock the health check endpoint (optional, not called in normal flow)
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [
                {
                    "name": "llama3.2:3b",
                    "size": 3825819519u64,
                    "modified_at": "2024-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    // Mock the generate endpoint
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": create_valid_plan_json_response(),
            "done": true,
            "model": "llama3.2:3b"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Create temporary directory for test files
    let temp_dir = TempDir::new().unwrap();
    let arch_path = temp_dir.path().join("architecture.md");
    let output_path = temp_dir.path().join("implementation_plan.md");

    // Create architecture document
    let arch = create_test_architecture_document();
    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();

    // Write architecture to file
    let arch_content = format!(
        "# {}\n\n{}\n\n## Components\n\n",
        arch.title(),
        arch.sections().first().unwrap().content()
    );
    fs::write(&arch_path, arch_content).unwrap();

    // Configure Ollama client with mock server URL
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("llama3.2:3b")
        .with_timeout_seconds(30);

    let client = OllamaClient::new(config.clone()).unwrap();
    let generator = OllamaPlanGenerator::with_config(client, config);

    // Create planning service
    let service = PlanningService::new(generator, parser, writer);

    // Execute the planning workflow (spawn_blocking because PlanningService is sync)
    let options = PlanOptions::default()
        .with_num_phases(3)
        .with_model("llama3.2:3b");
    let arch_path_clone = arch_path.clone();
    let plan = tokio::task::spawn_blocking(move || {
        service
            .generate_implementation_plan(&arch_path_clone, &options)
            .expect("Failed to generate plan")
    })
    .await
    .unwrap();

    // Verify plan was generated
    assert_eq!(plan.title(), "Implementation Plan for Test Project");
    assert_eq!(plan.phases().len(), 3);
    assert_eq!(plan.phases()[0].name(), "Foundation Setup");
    assert_eq!(plan.phases()[1].name(), "Core Implementation");
    assert_eq!(plan.phases()[2].name(), "Data Layer");

    // Verify dependencies
    assert!(plan.phases()[0].dependencies().is_empty());
    assert_eq!(plan.phases()[1].dependencies().len(), 1);
    assert_eq!(plan.phases()[2].dependencies().len(), 1);

    // Save plan to file
    let writer = MarkdownPlanWriter::new();
    writer
        .write_plan(&plan, &output_path)
        .expect("Failed to save plan");

    // Verify file was created
    assert!(output_path.exists());

    // Verify file content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Implementation Plan for Test Project"));
    assert!(content.contains("Foundation Setup"));
    assert!(content.contains("Core Implementation"));
    assert!(content.contains("Data Layer"));
}

#[tokio::test]
async fn test_implementation_command_with_invalid_architecture_file() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{"name": "llama3.2:3b"}]
        })))
        .mount(&mock_server)
        .await;

    // Create temporary directory
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_path = temp_dir.path().join("nonexistent.md");

    // Configure Ollama client
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("llama3.2:3b");

    let client = OllamaClient::new(config.clone()).unwrap();
    let generator = OllamaPlanGenerator::with_config(client, config);

    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();
    let service = PlanningService::new(generator, parser, writer);

    // Try to generate plan from nonexistent file
    let options = PlanOptions::default();
    let nonexistent_clone = nonexistent_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        service.generate_implementation_plan(&nonexistent_clone, &options)
    })
    .await
    .unwrap();

    // Verify error
    assert!(result.is_err());
    let error_message = format!("{}", result.unwrap_err());
    assert!(
        error_message.contains("Failed to parse")
            || error_message.contains("No such file")
            || error_message.contains("failed to read")
            || error_message.to_lowercase().contains("not found")
    );
}

#[tokio::test]
async fn test_implementation_command_ollama_service_unavailable() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check to return error
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&mock_server)
        .await;

    // Configure Ollama client
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("llama3.2:3b")
        .with_timeout_seconds(5);

    let client = OllamaClient::new(config).unwrap();

    // Try to check health
    let health_result = client.health_check().await;

    // Verify service is considered unhealthy
    assert!(health_result.is_ok());
    assert!(!health_result.unwrap());
}

#[tokio::test]
async fn test_implementation_command_with_model_not_found() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{"name": "llama3.2:3b"}]
        })))
        .mount(&mock_server)
        .await;

    // Mock generate endpoint with 404 for unknown model
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(404).set_body_string("model 'unknown-model' not found"))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Create temporary architecture file
    let temp_dir = TempDir::new().unwrap();
    let arch_path = temp_dir.path().join("architecture.md");
    fs::write(&arch_path, "# Test Architecture\n\nSimple test.").unwrap();

    // Configure Ollama client
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("unknown-model");

    let client = OllamaClient::new(config.clone()).unwrap();
    let generator = OllamaPlanGenerator::with_config(client, config);

    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();
    let service = PlanningService::new(generator, parser, writer);

    // Try to generate plan with unknown model
    let options = PlanOptions::default().with_model("unknown-model");
    let arch_path_clone = arch_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        service.generate_implementation_plan(&arch_path_clone, &options)
    })
    .await
    .unwrap();

    // Verify error
    assert!(result.is_err());
    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("not found") || error_message.contains("Model"));
}

#[tokio::test]
async fn test_implementation_command_with_invalid_json_response() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{"name": "llama3.2:3b"}]
        })))
        .mount(&mock_server)
        .await;

    // Mock generate endpoint with invalid JSON response
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": "This is not valid JSON for a plan: {invalid}",
            "done": true,
            "model": "llama3.2:3b"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Create temporary architecture file
    let temp_dir = TempDir::new().unwrap();
    let arch_path = temp_dir.path().join("architecture.md");
    let arch = create_test_architecture_document();
    fs::write(&arch_path, format!("# {}\n\nTest content.", arch.title())).unwrap();

    // Configure Ollama client
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("llama3.2:3b");

    let client = OllamaClient::new(config.clone()).unwrap();
    let generator = OllamaPlanGenerator::with_config(client, config);

    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();
    let service = PlanningService::new(generator, parser, writer);

    // Try to generate plan with invalid JSON response
    let options = PlanOptions::default();
    let arch_path_clone = arch_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        service.generate_implementation_plan(&arch_path_clone, &options)
    })
    .await
    .unwrap();

    // Verify error
    assert!(result.is_err());
    let error_message = format!("{}", result.unwrap_err());
    assert!(
        error_message.contains("parse")
            || error_message.contains("JSON")
            || error_message.contains("invalid")
    );
}

#[tokio::test]
async fn test_implementation_command_with_connection_timeout() {
    // Use an invalid URL that will timeout
    let config = OllamaConfig::new()
        .with_base_url("http://192.0.2.1:11434") // TEST-NET-1 (non-routable)
        .with_timeout_seconds(1)
        .with_max_retries(0);

    let client = OllamaClient::new(config).unwrap();

    // Try to list models with timeout
    let result = client.list_models().await;

    // Verify timeout error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_implementation_command_with_retry_logic() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{"name": "llama3.2:3b"}]
        })))
        .mount(&mock_server)
        .await;

    // Mock generate endpoint to fail twice, then succeed on third attempt
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": create_valid_plan_json_response(),
            "done": true,
            "model": "llama3.2:3b"
        })))
        .mount(&mock_server)
        .await;

    // Create temporary architecture file
    let temp_dir = TempDir::new().unwrap();
    let arch_path = temp_dir.path().join("architecture.md");
    let arch = create_test_architecture_document();
    fs::write(&arch_path, format!("# {}\n\nTest content.", arch.title())).unwrap();

    // Configure Ollama client with retries enabled
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("llama3.2:3b")
        .with_max_retries(3)
        .with_timeout_seconds(10);

    let client = OllamaClient::new(config.clone()).unwrap();
    let generator = OllamaPlanGenerator::with_config(client, config);

    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();
    let service = PlanningService::new(generator, parser, writer);

    // Generate plan (should succeed after retries)
    let options = PlanOptions::default().with_num_phases(3);
    let arch_path_clone = arch_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        service.generate_implementation_plan(&arch_path_clone, &options)
    })
    .await
    .unwrap();

    // Verify success after retries
    assert!(result.is_ok());
    let plan = result.unwrap();
    assert_eq!(plan.phases().len(), 3);
}

#[tokio::test]
async fn test_implementation_command_with_custom_options() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock health check
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{"name": "codellama"}]
        })))
        .mount(&mock_server)
        .await;

    // Mock generate endpoint
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": create_valid_plan_json_response(),
            "done": true,
            "model": "codellama"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Create temporary architecture file
    let temp_dir = TempDir::new().unwrap();
    let arch_path = temp_dir.path().join("architecture.md");
    let arch = create_test_architecture_document();
    fs::write(&arch_path, format!("# {}\n\nTest content.", arch.title())).unwrap();

    // Configure Ollama client
    let config = OllamaConfig::new()
        .with_base_url(mock_server.uri())
        .with_default_model("codellama")
        .with_temperature(0.9)
        .with_max_tokens(2000);

    let client = OllamaClient::new(config.clone()).unwrap();
    let generator = OllamaPlanGenerator::with_config(client, config);

    let parser = MarkdownArchitectureParser::new();
    let writer = MarkdownPlanWriter::new();
    let service = PlanningService::new(generator, parser, writer);

    // Generate plan with custom options
    let options = PlanOptions::default()
        .with_num_phases(5)
        .with_model("codellama")
        .with_temperature(0.9)
        .with_max_tokens(2000);

    let arch_path_clone = arch_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        service.generate_implementation_plan(&arch_path_clone, &options)
    })
    .await
    .unwrap();

    // Verify success
    assert!(result.is_ok());
    let plan = result.unwrap();
    assert_eq!(plan.metadata().model_used(), "codellama");
}

#[tokio::test]
async fn test_app_config_integration_with_implementation_command() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock endpoints
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [{"name": "llama3.2:3b"}]
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": create_valid_plan_json_response(),
            "done": true,
            "model": "llama3.2:3b"
        })))
        .mount(&mock_server)
        .await;

    // Create app config
    let app_config = xzagentz::config::app_config::AppConfig::builder()
        .ollama_url(mock_server.uri())
        .model("llama3.2:3b")
        .timeout(30)
        .build();

    // Verify config values
    assert_eq!(app_config.ollama.base_url, mock_server.uri());
    assert_eq!(app_config.ollama.default_model, "llama3.2:3b");
    assert_eq!(app_config.ollama.timeout_seconds, 30);

    // Create Ollama config from app config
    let ollama_config = OllamaConfig::new()
        .with_base_url(&app_config.ollama.base_url)
        .with_default_model(&app_config.ollama.default_model)
        .with_timeout_seconds(app_config.ollama.timeout_seconds);

    let client = OllamaClient::new(ollama_config).unwrap();

    // Verify client configuration
    assert_eq!(client.base_url(), mock_server.uri());

    // Test health check
    let health = client.health_check().await.unwrap();
    assert!(health);
}
