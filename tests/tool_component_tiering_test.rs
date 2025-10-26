//! Integration tests for tool component tiering (Phase 4)
//!
//! Tests validate:
//! - Tier metadata parsing (essential/comprehensive)
//! - Size limits per tier (essential ≤300, comprehensive ≤800 lines)
//! - Language-specific rendering per tier
//! - All 8 tiered components exist and parse correctly
//! - Tier selection logic

use std::fs;
use std::path::Path;
use xzagentz::components::metadata::ComponentMetadata;
use xzagentz::components::LanguageFilter;

const TOOLS_DIR: &str = "components/tools";

/// Helper to count non-empty lines in content
fn count_lines(content: &str) -> usize {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

/// Helper to check if file exists
fn component_exists(name: &str) -> bool {
    let path = format!("{}/{}.md", TOOLS_DIR, name);
    Path::new(&path).exists()
}

/// Helper to read component file
fn read_component(name: &str) -> String {
    let path = format!("{}/{}.md", TOOLS_DIR, name);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path))
}

#[test]
fn test_all_tiered_components_exist() {
    let components = vec![
        "git_essential",
        "git_comprehensive",
        "markdown_essential",
        "markdown_comprehensive",
        "docker_essential",
        "docker_comprehensive",
        "kubernetes_essential",
        "kubernetes_comprehensive",
    ];

    for component in components {
        assert!(
            component_exists(component),
            "Component {} does not exist",
            component
        );
    }
}

#[test]
fn test_git_essential_metadata() {
    let content = read_component("git_essential");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse git_essential frontmatter");

    assert_eq!(metadata.component.name, "git_essential");
    assert_eq!(metadata.component.category, "tools");
    assert_eq!(metadata.component.tier, Some("essential".to_string()));
    assert!(metadata.component.languages.contains(&"rust".to_string()));
    assert!(metadata.component.languages.contains(&"python".to_string()));
    assert!(metadata.component.languages.contains(&"golang".to_string()));
}

#[test]
fn test_git_comprehensive_metadata() {
    let content = read_component("git_comprehensive");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse git_comprehensive frontmatter");

    assert_eq!(metadata.component.name, "git_comprehensive");
    assert_eq!(metadata.component.category, "tools");
    assert_eq!(metadata.component.tier, Some("comprehensive".to_string()));
    assert!(metadata.component.languages.len() >= 5);
}

#[test]
fn test_markdown_essential_metadata() {
    let content = read_component("markdown_essential");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse markdown_essential frontmatter");

    assert_eq!(metadata.component.name, "markdown_essential");
    assert_eq!(metadata.component.tier, Some("essential".to_string()));
}

#[test]
fn test_markdown_comprehensive_metadata() {
    let content = read_component("markdown_comprehensive");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse markdown_comprehensive frontmatter");

    assert_eq!(metadata.component.name, "markdown_comprehensive");
    assert_eq!(metadata.component.tier, Some("comprehensive".to_string()));
}

#[test]
fn test_docker_essential_metadata() {
    let content = read_component("docker_essential");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse docker_essential frontmatter");

    assert_eq!(metadata.component.name, "docker_essential");
    assert_eq!(metadata.component.tier, Some("essential".to_string()));
}

#[test]
fn test_docker_comprehensive_metadata() {
    let content = read_component("docker_comprehensive");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse docker_comprehensive frontmatter");

    assert_eq!(metadata.component.name, "docker_comprehensive");
    assert_eq!(metadata.component.tier, Some("comprehensive".to_string()));
}

#[test]
fn test_kubernetes_essential_metadata() {
    let content = read_component("kubernetes_essential");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse kubernetes_essential frontmatter");

    assert_eq!(metadata.component.name, "kubernetes_essential");
    assert_eq!(metadata.component.tier, Some("essential".to_string()));
}

#[test]
fn test_kubernetes_comprehensive_metadata() {
    let content = read_component("kubernetes_comprehensive");
    let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
        .expect("Failed to parse kubernetes_comprehensive frontmatter");

    assert_eq!(metadata.component.name, "kubernetes_comprehensive");
    assert_eq!(metadata.component.tier, Some("comprehensive".to_string()));
}

#[test]
fn test_essential_tier_size_limit_git() {
    let content = read_component("git_essential");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 300,
        "git_essential has {} lines, exceeds 300 line limit for essential tier",
        line_count
    );
}

#[test]
fn test_essential_tier_size_limit_markdown() {
    let content = read_component("markdown_essential");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 300,
        "markdown_essential has {} lines, exceeds 300 line limit for essential tier",
        line_count
    );
}

#[test]
fn test_essential_tier_size_limit_docker() {
    let content = read_component("docker_essential");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 300,
        "docker_essential has {} lines, exceeds 300 line limit for essential tier",
        line_count
    );
}

#[test]
fn test_essential_tier_size_limit_kubernetes() {
    let content = read_component("kubernetes_essential");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 300,
        "kubernetes_essential has {} lines, exceeds 300 line limit for essential tier",
        line_count
    );
}

#[test]
fn test_comprehensive_tier_size_limit_git() {
    let content = read_component("git_comprehensive");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 800,
        "git_comprehensive has {} lines, exceeds 800 line limit for comprehensive tier",
        line_count
    );
}

#[test]
fn test_comprehensive_tier_size_limit_markdown() {
    let content = read_component("markdown_comprehensive");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 800,
        "markdown_comprehensive has {} lines, exceeds 800 line limit for comprehensive tier",
        line_count
    );
}

#[test]
fn test_comprehensive_tier_size_limit_docker() {
    let content = read_component("docker_comprehensive");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 800,
        "docker_comprehensive has {} lines, exceeds 800 line limit for comprehensive tier",
        line_count
    );
}

#[test]
fn test_comprehensive_tier_size_limit_kubernetes() {
    let content = read_component("kubernetes_comprehensive");
    let line_count = count_lines(&content);

    assert!(
        line_count <= 800,
        "kubernetes_comprehensive has {} lines, exceeds 800 line limit for comprehensive tier",
        line_count
    );
}

#[test]
fn test_git_essential_rust_rendering() {
    let content = read_component("git_essential");
    let filter = LanguageFilter::new("rust");
    let rendered = filter
        .filter_content(&content)
        .expect("Failed to filter content");

    assert!(
        rendered.contains("cargo fmt"),
        "git_essential Rust rendering should contain Rust-specific quality checks"
    );
    assert!(
        rendered.contains("cargo clippy"),
        "git_essential Rust rendering should contain clippy command"
    );
}

#[test]
fn test_git_essential_python_rendering() {
    let content = read_component("git_essential");
    let filter = LanguageFilter::new("python");
    let rendered = filter
        .filter_content(&content)
        .expect("Failed to filter content");

    assert!(
        rendered.contains("black"),
        "git_essential Python rendering should contain black formatter"
    );
    assert!(
        rendered.contains("pytest") || rendered.contains("ruff"),
        "git_essential Python rendering should contain Python tools"
    );
}

#[test]
fn test_markdown_essential_rust_rendering() {
    let content = read_component("markdown_essential");
    let filter = LanguageFilter::new("rust");
    let rendered = filter
        .filter_content(&content)
        .expect("Failed to filter content");

    assert!(
        rendered.contains("rust"),
        "markdown_essential should have Rust code block examples"
    );
}

#[test]
fn test_docker_essential_rust_rendering() {
    let content = read_component("docker_essential");
    let filter = LanguageFilter::new("rust");
    let rendered = filter
        .filter_content(&content)
        .expect("Failed to filter content");

    assert!(
        rendered.contains("FROM rust") || rendered.contains("Rust"),
        "docker_essential Rust rendering should contain Rust-specific Dockerfile patterns"
    );
}

#[test]
fn test_docker_essential_golang_rendering() {
    let content = read_component("docker_essential");
    let filter = LanguageFilter::new("golang");
    let rendered = filter
        .filter_content(&content)
        .expect("Failed to filter content");

    assert!(
        rendered.contains("FROM golang") || rendered.contains("Go"),
        "docker_essential Go rendering should contain Go-specific Dockerfile patterns"
    );
}

#[test]
fn test_kubernetes_essential_rust_rendering() {
    let content = read_component("kubernetes_essential");
    let filter = LanguageFilter::new("rust");
    let rendered = filter
        .filter_content(&content)
        .expect("Failed to filter content");

    assert!(
        rendered.contains("rust") || rendered.contains("Rust"),
        "kubernetes_essential should have Rust-specific deployment examples"
    );
}

#[test]
fn test_tier_validation_accepts_essential() {
    let content = r#"---
component:
  name: test
  category: tools
  version: 1.0.0
  tier: essential
---
Content"#;

    let result = ComponentMetadata::parse_frontmatter(content);
    assert!(result.is_ok(), "Should accept 'essential' tier");
}

#[test]
fn test_tier_validation_accepts_comprehensive() {
    let content = r#"---
component:
  name: test
  category: tools
  version: 1.0.0
  tier: comprehensive
---
Content"#;

    let result = ComponentMetadata::parse_frontmatter(content);
    assert!(result.is_ok(), "Should accept 'comprehensive' tier");
}

#[test]
fn test_tier_validation_rejects_invalid() {
    let content = r#"---
component:
  name: test
  category: tools
  version: 1.0.0
  tier: invalid
---
Content"#;

    let result = ComponentMetadata::parse_frontmatter(content);
    assert!(
        result.is_err(),
        "Should reject invalid tier value 'invalid'"
    );
}

#[test]
fn test_tier_field_optional() {
    let content = r#"---
component:
  name: test
  category: core
  version: 1.0.0
---
Content"#;

    let result = ComponentMetadata::parse_frontmatter(content);
    assert!(result.is_ok(), "Tier should be optional");

    let (metadata, _) = result.unwrap();
    assert_eq!(metadata.component.tier, None);
}

#[test]
fn test_git_essential_has_branch_naming() {
    let content = read_component("git_essential");

    assert!(
        content.contains("Branch Naming"),
        "git_essential should have branch naming section"
    );
    assert!(
        content.contains("pr-"),
        "git_essential should document pr- prefix"
    );
}

#[test]
fn test_git_essential_has_commit_format() {
    let content = read_component("git_essential");

    assert!(
        content.contains("Commit Message"),
        "git_essential should have commit message section"
    );
    assert!(
        content.contains("feat") && content.contains("fix"),
        "git_essential should document commit types"
    );
}

#[test]
fn test_markdown_essential_has_file_naming() {
    let content = read_component("markdown_essential");

    assert!(
        content.contains("File Naming"),
        "markdown_essential should have file naming section"
    );
    assert!(
        content.contains("lowercase"),
        "markdown_essential should document lowercase naming"
    );
}

#[test]
fn test_docker_essential_has_basic_commands() {
    let content = read_component("docker_essential");

    assert!(
        content.contains("docker build") || content.contains("Build"),
        "docker_essential should have build commands"
    );
    assert!(
        content.contains("docker run") || content.contains("Run"),
        "docker_essential should have run commands"
    );
}

#[test]
fn test_kubernetes_essential_has_basic_commands() {
    let content = read_component("kubernetes_essential");

    assert!(
        content.contains("kubectl"),
        "kubernetes_essential should have kubectl commands"
    );
    assert!(
        content.contains("Deployment") || content.contains("deployment"),
        "kubernetes_essential should mention deployments"
    );
}

#[test]
fn test_comprehensive_components_larger_than_essential() {
    let git_essential = count_lines(&read_component("git_essential"));
    let git_comprehensive = count_lines(&read_component("git_comprehensive"));

    assert!(
        git_comprehensive > git_essential,
        "git_comprehensive ({} lines) should be larger than git_essential ({} lines)",
        git_comprehensive,
        git_essential
    );

    let markdown_essential = count_lines(&read_component("markdown_essential"));
    let markdown_comprehensive = count_lines(&read_component("markdown_comprehensive"));

    assert!(
        markdown_comprehensive > markdown_essential,
        "markdown_comprehensive should be larger than markdown_essential"
    );
}

#[test]
fn test_all_components_have_language_markers() {
    let components = vec![
        "git_essential",
        "git_comprehensive",
        "markdown_essential",
        "markdown_comprehensive",
        "docker_essential",
        "docker_comprehensive",
        "kubernetes_essential",
        "kubernetes_comprehensive",
    ];

    for component in components {
        let content = read_component(component);

        assert!(
            content.contains("<!-- LANG:"),
            "{} should have language markers",
            component
        );
        assert!(
            content.contains("<!-- /LANG -->"),
            "{} should have closing language markers",
            component
        );
    }
}

#[test]
fn test_all_components_have_valid_versions() {
    let components = vec![
        "git_essential",
        "git_comprehensive",
        "markdown_essential",
        "markdown_comprehensive",
        "docker_essential",
        "docker_comprehensive",
        "kubernetes_essential",
        "kubernetes_comprehensive",
    ];

    for component in components {
        let content = read_component(component);
        let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)
            .unwrap_or_else(|_| panic!("Failed to parse {}", component));

        assert!(
            metadata.component.version.matches('.').count() == 2,
            "{} should have valid semver version",
            component
        );
    }
}

#[test]
fn test_git_comprehensive_has_advanced_topics() {
    let content = read_component("git_comprehensive");

    assert!(
        content.contains("rebase") || content.contains("Rebase"),
        "git_comprehensive should cover rebasing"
    );
    assert!(
        content.contains("stash") || content.contains("Stash"),
        "git_comprehensive should cover stashing"
    );
}

#[test]
fn test_markdown_comprehensive_has_advanced_topics() {
    let content = read_component("markdown_comprehensive");

    assert!(
        content.contains("Table") || content.contains("table"),
        "markdown_comprehensive should have detailed table documentation"
    );
    assert!(
        content.contains("HTML") || content.contains("html"),
        "markdown_comprehensive should discuss HTML in Markdown"
    );
}

#[test]
fn test_docker_comprehensive_has_advanced_topics() {
    let content = read_component("docker_comprehensive");

    assert!(
        content.contains("multi-stage") || content.contains("Multi-Stage"),
        "docker_comprehensive should cover multi-stage builds"
    );
    assert!(
        content.contains("network") || content.contains("Network"),
        "docker_comprehensive should cover networking"
    );
}

#[test]
fn test_kubernetes_comprehensive_has_advanced_topics() {
    let content = read_component("kubernetes_comprehensive");

    assert!(
        content.contains("StatefulSet") || content.contains("statefulset"),
        "kubernetes_comprehensive should cover StatefulSets"
    );
    assert!(
        content.contains("autoscaling") || content.contains("HPA"),
        "kubernetes_comprehensive should cover autoscaling"
    );
}
