//! Integration tests for size validation system
//!
//! These tests verify the size validation functionality works correctly
//! across different component types, categories, and tiers.

use std::fs;
use std::path::PathBuf;
use xzagentz::validator::size::{SizeLimits, SizeReport, SizeValidation, SizeValidator};

/// Helper function to get test component path
fn component_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("components")
        .join(filename)
}

/// Helper function to create test content with specific line count
fn create_test_content(lines: usize) -> String {
    (0..lines)
        .map(|i| format!("Line {} content", i))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_size_validator_with_default_limits() {
    let validator = SizeValidator::default();
    let limits = validator.limits();

    assert_eq!(limits.core_max, 500);
    assert_eq!(limits.general_max, 800);
    assert_eq!(limits.language_max, 600);
    assert_eq!(limits.tool_essential_max, 300);
    assert_eq!(limits.tool_comprehensive_max, 800);
    assert_eq!(limits.total_max, 10000);
}

#[test]
fn test_size_validator_with_custom_limits() {
    let custom_limits = SizeLimits {
        core_max: 400,
        general_max: 700,
        language_max: 500,
        tool_essential_max: 250,
        tool_comprehensive_max: 700,
        total_max: 8000,
        warn_threshold: 0.75,
    };

    let validator = SizeValidator::new(custom_limits);
    let limits = validator.limits();

    assert_eq!(limits.core_max, 400);
    assert_eq!(limits.warn_threshold, 0.75);
}

#[test]
fn test_validate_content_within_core_limit() {
    let validator = SizeValidator::default();
    let content = create_test_content(400);

    let result = validator.validate_content("core", None, &content);
    assert!(result.is_ok());

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert_eq!(validation.actual_lines, 400);
    assert_eq!(validation.max_lines, 500);
}

#[test]
fn test_validate_content_exceeds_core_limit() {
    let validator = SizeValidator::default();
    let content = create_test_content(600);

    let result = validator.validate_content("core", None, &content);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.to_string().contains("Size limit exceeded"));
    assert!(error.to_string().contains("600"));
    assert!(error.to_string().contains("500"));
}

#[test]
fn test_validate_content_warning_threshold() {
    let validator = SizeValidator::default();
    let content = create_test_content(450);

    let result = validator.validate_content("core", None, &content);
    assert!(result.is_ok());

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert!(validation.is_warning);
    assert_eq!(validation.warn_lines, 400);
}

#[test]
fn test_validate_tool_essential_tier() {
    let validator = SizeValidator::default();
    let content = create_test_content(250);

    let result = validator.validate_content("tools", Some("essential"), &content);
    assert!(result.is_ok());

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert_eq!(validation.max_lines, 300);
    assert_eq!(validation.tier, Some("essential".to_string()));
}

#[test]
fn test_validate_tool_comprehensive_tier() {
    let validator = SizeValidator::default();
    let content = create_test_content(700);

    let result = validator.validate_content("tools", Some("comprehensive"), &content);
    assert!(result.is_ok());

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert_eq!(validation.max_lines, 800);
    assert_eq!(validation.tier, Some("comprehensive".to_string()));
}

#[test]
fn test_validate_tool_essential_exceeds_limit() {
    let validator = SizeValidator::default();
    let content = create_test_content(350);

    let result = validator.validate_content("tools", Some("essential"), &content);
    assert!(result.is_err());
}

#[test]
fn test_validate_general_category() {
    let validator = SizeValidator::default();
    let content = create_test_content(750);

    let result = validator.validate_content("general", None, &content);
    assert!(result.is_ok());

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert_eq!(validation.max_lines, 800);
}

#[test]
fn test_validate_language_category() {
    let validator = SizeValidator::default();
    let content = create_test_content(550);

    let result = validator.validate_content("languages", None, &content);
    assert!(result.is_ok());

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert_eq!(validation.max_lines, 600);
}

#[test]
fn test_validate_total_within_limit() {
    let validator = SizeValidator::default();
    let result = validator.validate_total(5000);
    assert!(result.is_ok());
}

#[test]
fn test_validate_total_exceeds_limit() {
    let validator = SizeValidator::default();
    let result = validator.validate_total(12000);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.to_string().contains("12000"));
    assert!(error.to_string().contains("10000"));
}

#[test]
fn test_count_lines_ignores_blanks() {
    let validator = SizeValidator::default();

    let content = "Line 1\n\nLine 2\n   \nLine 3\n\n\n";
    assert_eq!(validator.count_lines(content), 3);
}

#[test]
fn test_count_lines_with_whitespace_only() {
    let validator = SizeValidator::default();

    let content = "   \n  \t  \n\n   \n";
    assert_eq!(validator.count_lines(content), 0);
}

#[test]
fn test_size_validation_status_messages() {
    let val_ok = SizeValidation::new("test".to_string(), "core".to_string(), None, 200, 500, 400);
    let message = val_ok.status_message();
    assert!(message.contains("OK"));
    assert!(message.contains("200 / 500"));

    let val_warning =
        SizeValidation::new("test".to_string(), "core".to_string(), None, 450, 500, 400);
    let message = val_warning.status_message();
    assert!(message.contains("WARNING"));
    assert!(message.contains("approaching limit"));

    let val_error =
        SizeValidation::new("test".to_string(), "core".to_string(), None, 600, 500, 400);
    let message = val_error.status_message();
    assert!(message.contains("ERROR"));
    assert!(message.contains("exceeds limit"));
}

#[test]
fn test_size_report_generation() {
    let validations = vec![
        SizeValidation::new("comp1".to_string(), "core".to_string(), None, 300, 500, 400),
        SizeValidation::new(
            "comp2".to_string(),
            "general".to_string(),
            None,
            700,
            800,
            640,
        ),
        SizeValidation::new(
            "comp3".to_string(),
            "tools".to_string(),
            Some("essential".to_string()),
            200,
            300,
            240,
        ),
    ];

    let report = SizeReport::new(validations, 10000);

    assert_eq!(report.total_lines, 1200);
    assert_eq!(report.error_count, 0);
    assert_eq!(report.warning_count, 1); // comp2 is 700/800 with warn at 640
    assert!(report.is_valid);
}

#[test]
fn test_size_report_with_errors() {
    let validations = vec![
        SizeValidation::new("comp1".to_string(), "core".to_string(), None, 600, 500, 400),
        SizeValidation::new(
            "comp2".to_string(),
            "general".to_string(),
            None,
            900,
            800,
            640,
        ),
    ];

    let report = SizeReport::new(validations, 10000);

    assert_eq!(report.error_count, 2);
    assert!(!report.is_valid);
}

#[test]
fn test_size_report_format_output() {
    let validations = vec![SizeValidation::new(
        "test".to_string(),
        "core".to_string(),
        None,
        250,
        500,
        400,
    )];

    let report = SizeReport::new(validations, 10000);
    let formatted = report.format_report();

    assert!(formatted.contains("Component Size Report"));
    assert!(formatted.contains("Summary"));
    assert!(formatted.contains("Total lines: 250 / 10000"));
    assert!(formatted.contains("Components: 1"));
    assert!(formatted.contains("Errors: 0"));
    assert!(formatted.contains("Status: PASS"));
}

#[test]
fn test_validate_real_git_essential_component() {
    let path = component_path("tools/git_essential.md");

    if !path.exists() {
        eprintln!("Skipping test: {} does not exist", path.display());
        return;
    }

    let validator = SizeValidator::default();
    let result = validator.validate_file(&path, "tools", Some("essential"));

    assert!(
        result.is_ok(),
        "git_essential.md should be within size limits"
    );

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert!(
        validation.actual_lines <= 300,
        "git_essential.md should be <= 300 lines, got {}",
        validation.actual_lines
    );
}

#[test]
fn test_validate_real_git_comprehensive_component() {
    let path = component_path("tools/git_comprehensive.md");

    if !path.exists() {
        eprintln!("Skipping test: {} does not exist", path.display());
        return;
    }

    let validator = SizeValidator::default();
    let result = validator.validate_file(&path, "tools", Some("comprehensive"));

    assert!(
        result.is_ok(),
        "git_comprehensive.md should be within size limits"
    );

    let validation = result.unwrap();
    assert!(validation.is_valid);
    assert!(
        validation.actual_lines <= 800,
        "git_comprehensive.md should be <= 800 lines, got {}",
        validation.actual_lines
    );
}

#[test]
fn test_validate_all_tool_components() {
    let tools_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("components/tools");

    if !tools_dir.exists() {
        eprintln!("Skipping test: {} does not exist", tools_dir.display());
        return;
    }

    let validator = SizeValidator::default();
    let mut validations = Vec::new();

    for entry in fs::read_dir(&tools_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            let tier = if filename.ends_with("_essential") {
                Some("essential")
            } else if filename.ends_with("_comprehensive") {
                Some("comprehensive")
            } else {
                None
            };

            if tier.is_some() {
                let result = validator.validate_file(&path, "tools", tier);

                match result {
                    Ok(validation) => {
                        println!(
                            "{}: {} / {} lines ({:.1}%)",
                            filename,
                            validation.actual_lines,
                            validation.max_lines,
                            validation.usage_ratio * 100.0
                        );
                        validations.push(validation);
                    }
                    Err(e) => {
                        panic!("Validation failed for {}: {}", filename, e);
                    }
                }
            }
        }
    }

    if !validations.is_empty() {
        let report = SizeReport::new(validations, 10000);
        println!("\n{}", report.format_report());

        assert_eq!(
            report.error_count, 0,
            "All tool components should meet size requirements"
        );
    }
}

#[test]
fn test_usage_ratio_calculation() {
    let validation =
        SizeValidation::new("test".to_string(), "core".to_string(), None, 250, 500, 400);

    assert_eq!(validation.usage_ratio, 0.5);

    let validation_full =
        SizeValidation::new("test".to_string(), "core".to_string(), None, 500, 500, 400);

    assert_eq!(validation_full.usage_ratio, 1.0);

    let validation_over =
        SizeValidation::new("test".to_string(), "core".to_string(), None, 600, 500, 400);

    assert_eq!(validation_over.usage_ratio, 1.2);
}

#[test]
fn test_warn_threshold_percentage() {
    let limits = SizeLimits::default();
    assert_eq!(limits.warn_threshold, 0.8);

    assert_eq!(limits.warn_at(500), 400);
    assert_eq!(limits.warn_at(300), 240);
    assert_eq!(limits.warn_at(800), 640);
}

#[test]
fn test_category_tier_display() {
    let val_with_tier = SizeValidation::new(
        "test".to_string(),
        "tools".to_string(),
        Some("essential".to_string()),
        200,
        300,
        240,
    );

    let message = val_with_tier.status_message();
    assert!(message.contains("tools/essential"));

    let val_without_tier =
        SizeValidation::new("test".to_string(), "core".to_string(), None, 200, 500, 400);

    let message = val_without_tier.status_message();
    assert!(message.contains("(core)"));
    assert!(!message.contains("core/"));
}
