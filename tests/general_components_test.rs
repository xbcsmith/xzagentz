//! Integration tests for general components (Phase 3)

use xzagentz::components::{ComponentMetadata, LanguageFilter};
use xzagentz::error::Result;

const TESTING_COMPONENT: &str = "components/general/testing.md";
const DEVELOPMENT_COMPONENT: &str = "components/general/development.md";
const DOCUMENTATION_COMPONENT: &str = "components/general/documentation.md";

#[test]
fn test_testing_component_metadata_parsing() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let (metadata, body) = ComponentMetadata::parse_frontmatter(&content)?;

    assert_eq!(metadata.component.name, "testing_standards");
    assert_eq!(metadata.component.category, "general");
    assert_eq!(metadata.component.version, "2.0.0");
    assert!(metadata.component.languages.contains(&"rust".to_string()));
    assert!(metadata.component.languages.contains(&"python".to_string()));
    assert!(metadata.component.languages.contains(&"golang".to_string()));
    assert!(metadata
        .component
        .languages
        .contains(&"typescript".to_string()));
    assert!(metadata.component.languages.contains(&"bash".to_string()));
    assert!(!body.is_empty());

    Ok(())
}

#[test]
fn test_development_component_metadata_parsing() -> Result<()> {
    let content = std::fs::read_to_string(DEVELOPMENT_COMPONENT)
        .expect("Failed to read development component");

    let (metadata, body) = ComponentMetadata::parse_frontmatter(&content)?;

    assert_eq!(metadata.component.name, "development_workflow");
    assert_eq!(metadata.component.category, "general");
    assert_eq!(metadata.component.version, "2.0.0");
    assert_eq!(metadata.component.languages.len(), 5);
    assert!(!body.is_empty());

    Ok(())
}

#[test]
fn test_documentation_component_metadata_parsing() -> Result<()> {
    let content = std::fs::read_to_string(DOCUMENTATION_COMPONENT)
        .expect("Failed to read documentation component");

    let (metadata, body) = ComponentMetadata::parse_frontmatter(&content)?;

    assert_eq!(metadata.component.name, "documentation_standards");
    assert_eq!(metadata.component.category, "general");
    assert_eq!(metadata.component.version, "2.0.0");
    assert_eq!(metadata.component.languages.len(), 5);
    assert!(!body.is_empty());

    Ok(())
}

#[test]
fn test_testing_component_rust_rendering() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let filter = LanguageFilter::new("rust");
    let rendered = filter.filter_content(&content)?;

    // Should contain Rust-specific content
    assert!(rendered.contains("#[test]"));
    assert!(rendered.contains("assert_eq!"));
    assert!(rendered.contains("cargo tarpaulin"));

    // Should contain agnostic principles
    assert!(rendered.contains("Core Principles"));
    assert!(rendered.contains("Test Behavior, Not Implementation"));

    Ok(())
}

#[test]
fn test_testing_component_python_rendering() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let filter = LanguageFilter::new("python");
    let rendered = filter.filter_content(&content)?;

    // Should contain Python-specific content
    assert!(rendered.contains("pytest"));
    assert!(rendered.contains("def test_"));
    assert!(rendered.contains("import pytest"));
    assert!(rendered.contains("pytest-cov"));

    Ok(())
}

#[test]
fn test_testing_component_golang_rendering() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let filter = LanguageFilter::new("golang");
    let rendered = filter.filter_content(&content)?;

    // Should contain Go-specific content
    assert!(rendered.contains("go test"));
    assert!(rendered.contains("func Test"));
    assert!(rendered.contains("testing.T"));
    assert!(rendered.contains("go test -cover"));

    Ok(())
}

#[test]
fn test_testing_component_typescript_rendering() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let filter = LanguageFilter::new("typescript");
    let rendered = filter.filter_content(&content)?;

    // Should contain TypeScript-specific content
    assert!(rendered.contains("vitest"));
    assert!(rendered.contains("describe"));
    assert!(rendered.contains("expect"));
    assert!(rendered.contains("npx vitest"));

    Ok(())
}

#[test]
fn test_testing_component_bash_rendering() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let filter = LanguageFilter::new("bash");
    let rendered = filter.filter_content(&content)?;

    // Should contain Bash-specific content
    assert!(rendered.contains("shunit2"));
    assert!(rendered.contains("test_"));
    assert!(rendered.contains("assertEquals"));
    assert!(rendered.contains("kcov"));

    Ok(())
}

#[test]
fn test_development_component_rust_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DEVELOPMENT_COMPONENT)
        .expect("Failed to read development component");

    let filter = LanguageFilter::new("rust");
    let rendered = filter.filter_content(&content)?;

    // Should contain Rust-specific content
    assert!(rendered.contains("cargo"));
    assert!(rendered.contains("rustup"));
    assert!(rendered.contains("cargo clippy"));
    assert!(rendered.contains("cargo fmt"));

    Ok(())
}

#[test]
fn test_development_component_python_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DEVELOPMENT_COMPONENT)
        .expect("Failed to read development component");

    let filter = LanguageFilter::new("python");
    let rendered = filter.filter_content(&content)?;

    // Should contain Python-specific content
    assert!(rendered.contains("pip"));
    assert!(rendered.contains("venv"));
    assert!(rendered.contains("black"));
    assert!(rendered.contains("pylint"));

    Ok(())
}

#[test]
fn test_development_component_golang_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DEVELOPMENT_COMPONENT)
        .expect("Failed to read development component");

    let filter = LanguageFilter::new("golang");
    let rendered = filter.filter_content(&content)?;

    // Should contain Go-specific content
    assert!(rendered.contains("go get"));
    assert!(rendered.contains("go mod"));
    assert!(rendered.contains("gofmt"));
    assert!(rendered.contains("golangci-lint"));

    Ok(())
}

#[test]
fn test_development_component_typescript_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DEVELOPMENT_COMPONENT)
        .expect("Failed to read development component");

    let filter = LanguageFilter::new("typescript");
    let rendered = filter.filter_content(&content)?;

    // Should contain TypeScript-specific content
    assert!(rendered.contains("pnpm") || rendered.contains("npm"));
    assert!(rendered.contains("typescript"));
    assert!(rendered.contains("eslint"));
    assert!(rendered.contains("prettier"));

    Ok(())
}

#[test]
fn test_development_component_bash_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DEVELOPMENT_COMPONENT)
        .expect("Failed to read development component");

    let filter = LanguageFilter::new("bash");
    let rendered = filter.filter_content(&content)?;

    // Should contain Bash-specific content
    assert!(rendered.contains("shellcheck"));
    assert!(rendered.contains("shfmt"));
    assert!(rendered.contains("bash"));

    Ok(())
}

#[test]
fn test_documentation_component_rust_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DOCUMENTATION_COMPONENT)
        .expect("Failed to read documentation component");

    let filter = LanguageFilter::new("rust");
    let rendered = filter.filter_content(&content)?;

    // Should contain Rust-specific content
    assert!(rendered.contains("///"));
    assert!(rendered.contains("# Examples"));
    assert!(rendered.contains("Doc Comments"));

    Ok(())
}

#[test]
fn test_documentation_component_python_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DOCUMENTATION_COMPONENT)
        .expect("Failed to read documentation component");

    let filter = LanguageFilter::new("python");
    let rendered = filter.filter_content(&content)?;

    // Should contain Python-specific content
    assert!(rendered.contains("\"\"\""));
    assert!(rendered.contains("Args:"));
    assert!(rendered.contains("Returns:"));
    assert!(rendered.contains("doctest"));

    Ok(())
}

#[test]
fn test_documentation_component_golang_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DOCUMENTATION_COMPONENT)
        .expect("Failed to read documentation component");

    let filter = LanguageFilter::new("golang");
    let rendered = filter.filter_content(&content)?;

    // Should contain Go-specific content
    assert!(rendered.contains("// "));
    assert!(rendered.contains("Example:"));

    Ok(())
}

#[test]
fn test_documentation_component_typescript_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DOCUMENTATION_COMPONENT)
        .expect("Failed to read documentation component");

    let filter = LanguageFilter::new("typescript");
    let rendered = filter.filter_content(&content)?;

    // Should contain TypeScript-specific content
    assert!(rendered.contains("/**"));
    assert!(rendered.contains("@param"));
    assert!(rendered.contains("@returns"));

    Ok(())
}

#[test]
fn test_documentation_component_bash_rendering() -> Result<()> {
    let content = std::fs::read_to_string(DOCUMENTATION_COMPONENT)
        .expect("Failed to read documentation component");

    let filter = LanguageFilter::new("bash");
    let rendered = filter.filter_content(&content)?;

    // Should contain Bash-specific content
    assert!(rendered.contains("#######################################"));
    assert!(rendered.contains("Arguments:"));
    assert!(rendered.contains("Returns:"));

    Ok(())
}

#[test]
fn test_general_components_language_sections_closed() -> Result<()> {
    let components = [
        TESTING_COMPONENT,
        DEVELOPMENT_COMPONENT,
        DOCUMENTATION_COMPONENT,
    ];

    for component in &components {
        let content = std::fs::read_to_string(component).expect("Failed to read component");

        // Verify all language sections are properly closed
        for language in &["rust", "python", "golang", "typescript", "bash"] {
            let filter = LanguageFilter::new(*language);
            let result = filter.filter_content(&content);
            assert!(
                result.is_ok(),
                "Language section not properly closed in {}: {}",
                component,
                result.unwrap_err()
            );
        }
    }

    Ok(())
}

#[test]
fn test_general_components_size_compliance() {
    let components = [
        (TESTING_COMPONENT, "testing_standards"),
        (DEVELOPMENT_COMPONENT, "development_workflow"),
        (DOCUMENTATION_COMPONENT, "documentation_standards"),
    ];

    // General category limit: 2500 lines
    const GENERAL_LIMIT: usize = 2500;

    for (path, name) in &components {
        let content = std::fs::read_to_string(path).expect("Failed to read component");

        let line_count = content.lines().count();

        assert!(
            line_count <= GENERAL_LIMIT,
            "Component {} exceeds general size limit: {} > {}",
            name,
            line_count,
            GENERAL_LIMIT
        );
    }
}

#[test]
fn test_general_components_have_required_sections() -> Result<()> {
    let components = [
        TESTING_COMPONENT,
        DEVELOPMENT_COMPONENT,
        DOCUMENTATION_COMPONENT,
    ];

    for component in &components {
        let content = std::fs::read_to_string(component).expect("Failed to read component");

        let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)?;

        // Check that at least some sections are defined
        assert!(
            !metadata.component.sections.is_empty(),
            "Component {} has no sections defined",
            component
        );

        // Check that required sections exist
        let has_required = metadata.component.sections.iter().any(|s| s.required);
        assert!(
            has_required,
            "Component {} has no required sections",
            component
        );
    }

    Ok(())
}

#[test]
fn test_general_components_version_consistency() -> Result<()> {
    let components = [
        TESTING_COMPONENT,
        DEVELOPMENT_COMPONENT,
        DOCUMENTATION_COMPONENT,
    ];

    for component in &components {
        let content = std::fs::read_to_string(component).expect("Failed to read component");

        let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)?;

        // All Phase 3 components should be version 2.0.0
        assert_eq!(
            metadata.component.version, "2.0.0",
            "Component {} has incorrect version",
            component
        );
    }

    Ok(())
}

#[test]
fn test_general_components_category_consistency() -> Result<()> {
    let components = [
        TESTING_COMPONENT,
        DEVELOPMENT_COMPONENT,
        DOCUMENTATION_COMPONENT,
    ];

    for component in &components {
        let content = std::fs::read_to_string(component).expect("Failed to read component");

        let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)?;

        // All should be in general category
        assert_eq!(
            metadata.component.category, "general",
            "Component {} has incorrect category",
            component
        );
    }

    Ok(())
}

#[test]
fn test_general_components_language_coverage() -> Result<()> {
    let components = [
        TESTING_COMPONENT,
        DEVELOPMENT_COMPONENT,
        DOCUMENTATION_COMPONENT,
    ];

    let expected_languages = vec!["rust", "python", "golang", "typescript", "bash"];

    for component in &components {
        let content = std::fs::read_to_string(component).expect("Failed to read component");

        let (metadata, _) = ComponentMetadata::parse_frontmatter(&content)?;

        // All Phase 3 components should support all 5 languages
        assert_eq!(
            metadata.component.languages.len(),
            5,
            "Component {} doesn't support all languages",
            component
        );

        for lang in &expected_languages {
            assert!(
                metadata.component.languages.contains(&lang.to_string()),
                "Component {} missing language: {}",
                component,
                lang
            );
        }
    }

    Ok(())
}

#[test]
fn test_rendered_output_contains_no_language_markers() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let filter = LanguageFilter::new("rust");
    let rendered = filter.filter_content(&content)?;

    // Rendered output should not contain language markers
    assert!(!rendered.contains("<!-- LANG:"));
    assert!(!rendered.contains("<!-- /LANG -->"));

    Ok(())
}

#[test]
fn test_general_components_agnostic_sections_in_all_renders() -> Result<()> {
    let content =
        std::fs::read_to_string(TESTING_COMPONENT).expect("Failed to read testing component");

    let languages = ["rust", "python", "golang", "typescript", "bash"];

    for lang in &languages {
        let filter = LanguageFilter::new(*lang);
        let rendered = filter.filter_content(&content)?;

        // Core Principles section is agnostic and should appear in all
        assert!(
            rendered.contains("Core Principles"),
            "Language {} missing Core Principles section",
            lang
        );

        assert!(
            rendered.contains("Test Behavior, Not Implementation"),
            "Language {} missing principles content",
            lang
        );
    }

    Ok(())
}
