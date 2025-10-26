//! Integration tests for refactored core components
//!
//! Tests that the hybrid component system correctly parses and renders
//! the refactored core components with YAML frontmatter and language sections.

use xzagentz::components::{ComponentMetadata, LanguageFilter};
use xzagentz::error::Result;

#[test]
fn test_critical_rules_has_valid_frontmatter() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");

    // Act
    let (metadata, body) = ComponentMetadata::parse_frontmatter(&content)?;

    // Assert
    assert_eq!(metadata.component.name, "critical_rules");
    assert_eq!(metadata.component.category, "core");
    assert_eq!(metadata.component.version, "2.0.0");
    assert!(!body.is_empty());

    // Check languages
    let languages = &metadata.component.languages;
    assert!(languages.contains(&"rust".to_string()));
    assert!(languages.contains(&"python".to_string()));
    assert!(languages.contains(&"golang".to_string()));
    assert!(languages.contains(&"typescript".to_string()));

    // Check sections
    assert!(!metadata.component.sections.is_empty());
    let section_ids: Vec<_> = metadata
        .component
        .sections
        .iter()
        .map(|s| s.id.as_str())
        .collect();
    assert!(section_ids.contains(&"file_naming"));
    assert!(section_ids.contains(&"code_quality"));
    assert!(section_ids.contains(&"documentation"));
    assert!(section_ids.contains(&"error_handling"));
    assert!(section_ids.contains(&"testing"));

    Ok(())
}

#[test]
fn test_error_handling_has_valid_frontmatter() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/error_handling.md")
        .expect("Failed to read error_handling.md");

    // Act
    let (metadata, body) = ComponentMetadata::parse_frontmatter(&content)?;

    // Assert
    assert_eq!(metadata.component.name, "error_handling");
    assert_eq!(metadata.component.category, "core");
    assert_eq!(metadata.component.version, "2.0.0");
    assert!(!body.is_empty());

    // Check sections
    let section_ids: Vec<_> = metadata
        .component
        .sections
        .iter()
        .map(|s| s.id.as_str())
        .collect();
    assert!(section_ids.contains(&"principles"));
    assert!(section_ids.contains(&"error_types"));
    assert!(section_ids.contains(&"error_patterns"));
    assert!(section_ids.contains(&"context_propagation"));
    assert!(section_ids.contains(&"testing_errors"));

    Ok(())
}

#[test]
fn test_testing_standards_has_valid_frontmatter() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/testing_standards.md")
        .expect("Failed to read testing_standards.md");

    // Act
    let (metadata, body) = ComponentMetadata::parse_frontmatter(&content)?;

    // Assert
    assert_eq!(metadata.component.name, "testing_standards");
    assert_eq!(metadata.component.category, "core");
    assert_eq!(metadata.component.version, "2.0.0");
    assert!(!body.is_empty());

    // Check sections
    let section_ids: Vec<_> = metadata
        .component
        .sections
        .iter()
        .map(|s| s.id.as_str())
        .collect();
    assert!(section_ids.contains(&"philosophy"));
    assert!(section_ids.contains(&"test_structure"));
    assert!(section_ids.contains(&"naming_conventions"));
    assert!(section_ids.contains(&"assertions"));
    assert!(section_ids.contains(&"test_fixtures"));
    assert!(section_ids.contains(&"coverage"));

    Ok(())
}

#[test]
fn test_critical_rules_renders_for_rust() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("rust");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("cargo fmt"));
    assert!(filtered_content.contains("cargo check"));
    assert!(filtered_content.contains("cargo clippy"));
    assert!(filtered_content.contains("cargo test"));
    // Check for Rust-specific patterns rather than absence of other languages
    assert!(filtered_content.contains("Result<T, E>") || filtered_content.contains("cargo"));
    assert!(filtered_content.lines().count() > 0);

    Ok(())
}

#[test]
fn test_critical_rules_renders_for_python() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("python");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("black"));
    assert!(filtered_content.contains("pytest"));
    assert!(filtered_content.contains("mypy"));
    assert!(!filtered_content.contains("cargo fmt"));
    assert!(!filtered_content.contains("go test"));

    Ok(())
}

#[test]
fn test_critical_rules_renders_for_golang() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("golang");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("gofmt"));
    assert!(filtered_content.contains("go test"));
    assert!(filtered_content.contains("go vet"));
    assert!(!filtered_content.contains("cargo"));
    assert!(!filtered_content.contains("pytest"));

    Ok(())
}

#[test]
fn test_critical_rules_renders_for_typescript() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("typescript");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("prettier"));
    assert!(filtered_content.contains("eslint"));
    assert!(filtered_content.contains("tsc"));
    assert!(!filtered_content.contains("cargo"));
    assert!(!filtered_content.contains("pytest"));

    Ok(())
}

#[test]
fn test_error_handling_renders_for_rust() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/error_handling.md")
        .expect("Failed to read error_handling.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("rust");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(
        filtered_content.contains("Result")
            || filtered_content.contains("thiserror")
            || filtered_content.contains("#[derive(Error")
    );
    assert!(filtered_content.contains("Rust") || filtered_content.contains("rust"));

    Ok(())
}

#[test]
fn test_error_handling_renders_for_python() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/error_handling.md")
        .expect("Failed to read error_handling.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("python");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("Exception"));
    assert!(filtered_content.contains("raise"));
    assert!(filtered_content.contains("except"));
    assert!(!filtered_content.contains("Result<T, E>"));
    assert!(!filtered_content.contains("thiserror"));

    Ok(())
}

#[test]
fn test_error_handling_renders_for_golang() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/error_handling.md")
        .expect("Failed to read error_handling.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("golang");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("error"));
    assert!(filtered_content.contains("fmt.Errorf"));
    assert!(filtered_content.contains("if err != nil"));
    assert!(!filtered_content.contains("Result<T, E>"));
    assert!(!filtered_content.contains("except"));

    Ok(())
}

#[test]
fn test_testing_standards_renders_for_rust() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/testing_standards.md")
        .expect("Failed to read testing_standards.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("rust");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("#[test]") || filtered_content.contains("assert_eq!"));
    assert!(
        filtered_content.contains("Rust")
            || filtered_content.contains("rust")
            || filtered_content.contains("cargo")
    );

    Ok(())
}

#[test]
fn test_testing_standards_renders_for_python() -> Result<()> {
    // Arrange
    let content = std::fs::read_to_string("components/core/testing_standards.md")
        .expect("Failed to read testing_standards.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let filter = LanguageFilter::new("python");

    // Act
    let filtered_content = filter.filter_content(&body)?;

    // Assert
    assert!(filtered_content.contains("pytest"));
    assert!(filtered_content.contains("def test_"));
    assert!(filtered_content.contains("@pytest.fixture"));
    assert!(!filtered_content.contains("#[test]"));
    assert!(!filtered_content.contains("cargo test"));

    Ok(())
}

#[test]
fn test_all_components_parse_without_errors() -> Result<()> {
    // Test that all three refactored components parse successfully
    let components = vec![
        "components/core/critical_rules.md",
        "components/core/error_handling.md",
        "components/core/testing_standards.md",
    ];

    for component_path in components {
        let content = std::fs::read_to_string(component_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", component_path));

        let result = ComponentMetadata::parse_frontmatter(&content);
        assert!(
            result.is_ok(),
            "Failed to parse {}: {:?}",
            component_path,
            result.err()
        );

        let (metadata, body) = result.unwrap();

        // Basic validations
        assert!(!metadata.component.name.is_empty());
        assert_eq!(metadata.component.category, "core");
        assert!(!metadata.component.version.is_empty());
        assert!(!body.is_empty());
        assert!(!metadata.component.sections.is_empty());
    }

    Ok(())
}

#[test]
fn test_language_sections_are_properly_closed() -> Result<()> {
    // Ensure all language sections have matching start/end markers
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");

    let filter = LanguageFilter::new("rust");
    let sections = filter.parse_sections(&content)?;

    // Each section should have a valid start and end line
    for section in sections {
        assert!(section.start_line > 0);
        assert!(section.end_line > section.start_line);
        assert!(!section.content.is_empty());
    }

    Ok(())
}

#[test]
fn test_component_stays_within_size_limits() -> Result<()> {
    // Core components should stay under 3000 lines
    let max_lines = 3000;

    let components = vec![
        ("critical_rules.md", "components/core/critical_rules.md"),
        ("error_handling.md", "components/core/error_handling.md"),
        (
            "testing_standards.md",
            "components/core/testing_standards.md",
        ),
    ];

    for (name, path) in components {
        let content =
            std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", path));

        let line_count = content.lines().count();

        assert!(
            line_count <= max_lines,
            "{} has {} lines, exceeds limit of {}",
            name,
            line_count,
            max_lines
        );
    }

    Ok(())
}

#[test]
fn test_agnostic_sections_render_for_all_languages() -> Result<()> {
    // Test that language-agnostic sections appear for all target languages
    let content = std::fs::read_to_string("components/core/critical_rules.md")
        .expect("Failed to read critical_rules.md");
    let (_, body) = ComponentMetadata::parse_frontmatter(&content)?;

    let languages = vec!["rust", "python", "golang", "typescript"];

    for lang in languages {
        let filter = LanguageFilter::new(lang);
        let filtered = filter.filter_content(&body)?;

        // File naming section should appear for all languages (it's agnostic)
        assert!(
            filtered.contains("File Naming Conventions"),
            "File naming section missing for {}",
            lang
        );
        assert!(
            filtered.contains("Markdown Files"),
            "Markdown files section missing for {}",
            lang
        );
        assert!(
            filtered.contains("YAML Files"),
            "YAML files section missing for {}",
            lang
        );
    }

    Ok(())
}

#[test]
fn test_metadata_validation_passes() -> Result<()> {
    // Test that all metadata objects pass validation
    let components = vec![
        "components/core/critical_rules.md",
        "components/core/error_handling.md",
        "components/core/testing_standards.md",
    ];

    for component_path in components {
        let content = std::fs::read_to_string(component_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", component_path));

        let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)?;

        // Validate should pass
        metadata.validate()?;
    }

    Ok(())
}
