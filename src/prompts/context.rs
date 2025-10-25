//! Context extraction from plans for prompt generation
//!
//! This module provides functionality to extract and build prompt context
//! from architecture plans and implementation plans, including filtering
//! relevant AGENTS.md rules based on section content.

use crate::error::Result;
use crate::plans::structures::{ArchitecturePlan, ImplementationPlan, Phase, PlanSection};
use crate::prompts::template::{DeliverableFile, PromptContext, Rule, TestExample, TestFile};

/// Context extractor for building prompt contexts
pub struct ContextExtractor {
    /// Include full architecture in context
    include_full_architecture: bool,
    /// Include all AGENTS.md rules or only relevant ones
    include_all_rules: bool,
}

impl ContextExtractor {
    /// Create a new context extractor
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::prompts::context::ContextExtractor;
    ///
    /// let extractor = ContextExtractor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            include_full_architecture: false,
            include_all_rules: false,
        }
    }

    /// Create context extractor with full architecture
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::prompts::context::ContextExtractor;
    ///
    /// let extractor = ContextExtractor::with_full_architecture();
    /// ```
    pub fn with_full_architecture() -> Self {
        Self {
            include_full_architecture: true,
            include_all_rules: false,
        }
    }

    /// Create context extractor with all rules
    pub fn with_all_rules() -> Self {
        Self {
            include_full_architecture: false,
            include_all_rules: true,
        }
    }

    /// Extract context from implementation plan section
    ///
    /// # Arguments
    ///
    /// * `impl_plan` - Implementation plan
    /// * `arch_plan` - Architecture plan (optional)
    /// * `phase` - Current phase
    /// * `section` - Current section
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::context::ContextExtractor;
    /// use xzagentz::plans::parser::PlanParser;
    ///
    /// let parser = PlanParser::new();
    /// let plan_content = std::fs::read_to_string("plan.md").unwrap();
    /// let impl_plan = parser.parse_implementation_plan(&plan_content).unwrap();
    /// let phase = &impl_plan.phases[0];
    /// let section = &phase.sections[0];
    ///
    /// let extractor = ContextExtractor::new();
    /// let context = extractor.extract_context(&impl_plan, None, phase, section).unwrap();
    /// ```
    pub fn extract_context(
        &self,
        impl_plan: &ImplementationPlan,
        arch_plan: Option<&ArchitecturePlan>,
        phase: &Phase,
        section: &PlanSection,
    ) -> Result<PromptContext> {
        // Build context with struct literal initialization
        let mut context = PromptContext {
            project_name: impl_plan.metadata.name.clone(),
            project_description: impl_plan.metadata.description.clone().unwrap_or_default(),
            phase_number: phase.number,
            phase_duration: phase.duration.clone().unwrap_or_default(),
            phase_goal: phase.goal.clone(),
            section_number: section.number.clone(),
            section_name: section.title.clone(),
            section_purpose: self.extract_section_purpose(section),
            tasks: section.tasks.clone(),
            deliverable_files: self.extract_deliverables(section),
            test_files: self.extract_test_files(section),
            documentation_file: self.generate_documentation_filename(phase, section),
            acceptance_criteria: section.acceptance_criteria.clone(),
            test_examples: self.extract_test_examples(section),
            ..Default::default()
        };

        // Architecture context (if available)
        if let Some(arch) = arch_plan {
            context.architecture_overview = if self.include_full_architecture {
                arch.overview.clone()
            } else {
                self.extract_relevant_architecture(arch, section)
            };

            context.related_components = self.extract_related_components(arch, section);
            context.dependencies = self.extract_dependencies(phase, section);
        }

        // AGENTS.md rules
        context.relevant_rules = if self.include_all_rules {
            self.get_all_rules()
        } else {
            self.extract_relevant_rules(section)
        };

        // Metadata
        context.version = env!("CARGO_PKG_VERSION").to_string();
        context.timestamp = chrono::Utc::now().to_rfc3339();

        Ok(context)
    }

    /// Extract section purpose from tasks
    fn extract_section_purpose(&self, section: &PlanSection) -> String {
        section
            .tasks
            .first()
            .cloned()
            .unwrap_or_else(|| format!("Implement {}", section.title))
    }

    /// Extract deliverable files from section
    fn extract_deliverables(&self, section: &PlanSection) -> Vec<DeliverableFile> {
        section
            .deliverables
            .iter()
            .map(|d| {
                let (path, description) = if d.contains(':') {
                    let parts: Vec<&str> = d.splitn(2, ':').collect();
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    (d.clone(), self.infer_file_description(d))
                };

                DeliverableFile { path, description }
            })
            .collect()
    }

    /// Infer file description from path
    fn infer_file_description(&self, path: &str) -> String {
        if path.contains("test") {
            "Test implementation".to_string()
        } else if path.ends_with(".md") {
            "Documentation file".to_string()
        } else if path.ends_with(".rs") {
            "Rust implementation file".to_string()
        } else if path.ends_with(".toml") {
            "Configuration file".to_string()
        } else {
            "Implementation file".to_string()
        }
    }

    /// Extract test files from section
    fn extract_test_files(&self, section: &PlanSection) -> Vec<TestFile> {
        section
            .tests
            .iter()
            .map(|t| {
                let path = if t.starts_with("fn ") || t.starts_with("test_") {
                    format!("tests/{}.rs", sanitize_test_name(t))
                } else {
                    t.clone()
                };

                let description = if t.starts_with("fn test_") {
                    format!("Test: {}", t.strip_prefix("fn test_").unwrap_or(t))
                } else if t.starts_with("test_") {
                    format!("Test: {}", t.strip_prefix("test_").unwrap_or(t))
                } else {
                    format!("Test function: {}", t)
                };

                TestFile { path, description }
            })
            .collect()
    }

    /// Generate documentation filename
    fn generate_documentation_filename(&self, phase: &Phase, section: &PlanSection) -> String {
        let section_slug = section.number.replace(['.', ' '], "_").to_lowercase();

        format!("phase{}_{}_implementation.md", phase.number, section_slug)
    }

    /// Extract test examples from section
    fn extract_test_examples(&self, section: &PlanSection) -> Vec<TestExample> {
        section
            .tests
            .iter()
            .map(|t| {
                let name = sanitize_test_name(t);
                let description = if t.contains("test_") {
                    format!("Test {}", t.replace("test_", "").replace('_', " "))
                } else {
                    format!("Test for {}", t)
                };

                TestExample { name, description }
            })
            .collect()
    }

    /// Extract relevant architecture sections
    fn extract_relevant_architecture(
        &self,
        arch_plan: &ArchitecturePlan,
        _section: &PlanSection,
    ) -> String {
        // For now, return overview. Could be enhanced to extract only relevant parts.
        arch_plan.overview.clone()
    }

    /// Extract related components from architecture
    fn extract_related_components(
        &self,
        arch_plan: &ArchitecturePlan,
        _section: &PlanSection,
    ) -> Vec<String> {
        // Return all components for now. Could be enhanced with smart filtering.
        arch_plan.components.clone()
    }

    /// Extract dependencies for section
    fn extract_dependencies(&self, phase: &Phase, _section: &PlanSection) -> Vec<String> {
        // Map phase dependencies to strings
        phase
            .dependencies
            .iter()
            .map(|dep| format!("Phase {}", dep))
            .collect()
    }

    /// Extract relevant AGENTS.md rules based on section content
    fn extract_relevant_rules(&self, section: &PlanSection) -> Vec<Rule> {
        let mut rules = Vec::new();

        // Always include core rules
        rules.extend(self.get_core_rules());

        // Add language-specific rules if section involves code
        if self.section_involves_code(section) {
            rules.extend(self.get_code_rules());
        }

        // Add documentation rules if section involves docs
        if self.section_involves_documentation(section) {
            rules.extend(self.get_documentation_rules());
        }

        // Add configuration rules if section involves YAML/TOML
        if self.section_involves_configuration(section) {
            rules.extend(self.get_configuration_rules());
        }

        rules
    }

    /// Get all AGENTS.md rules
    fn get_all_rules(&self) -> Vec<Rule> {
        let mut rules = self.get_core_rules();
        rules.extend(self.get_code_rules());
        rules.extend(self.get_documentation_rules());
        rules.extend(self.get_configuration_rules());
        rules.extend(self.get_git_rules());
        rules
    }

    /// Get core AGENTS.md rules
    fn get_core_rules(&self) -> Vec<Rule> {
        vec![
            Rule {
                title: "File Extensions".to_string(),
                content: "Use .yaml (not .yml), .md (not .markdown)".to_string(),
            },
            Rule {
                title: "Markdown File Naming".to_string(),
                content: "Use lowercase_with_underscores.md (exception: README.md)".to_string(),
            },
            Rule {
                title: "No Emojis".to_string(),
                content: "No emojis in code, documentation, or commits".to_string(),
            },
        ]
    }

    /// Get code-specific rules
    fn get_code_rules(&self) -> Vec<Rule> {
        vec![
            Rule {
                title: "Error Handling".to_string(),
                content: "Use Result<T, E>, no unwrap() without justification".to_string(),
            },
            Rule {
                title: "Documentation".to_string(),
                content: "All public items must have /// doc comments with examples".to_string(),
            },
            Rule {
                title: "Testing".to_string(),
                content: "Test success, failure, and edge cases. >80% coverage required"
                    .to_string(),
            },
        ]
    }

    /// Get documentation-specific rules
    fn get_documentation_rules(&self) -> Vec<Rule> {
        vec![
            Rule {
                title: "Code Blocks".to_string(),
                content: "All code blocks must specify language identifier".to_string(),
            },
            Rule {
                title: "Documentation Structure".to_string(),
                content: "Include: Overview, Components, Implementation Details, Testing, Examples"
                    .to_string(),
            },
        ]
    }

    /// Get configuration-specific rules
    fn get_configuration_rules(&self) -> Vec<Rule> {
        vec![Rule {
            title: "YAML Files".to_string(),
            content: "Always use .yaml extension, never .yml".to_string(),
        }]
    }

    /// Get git-specific rules
    fn get_git_rules(&self) -> Vec<Rule> {
        vec![
            Rule {
                title: "Branch Naming".to_string(),
                content: "Use pr-{jira-issue} format (lowercase)".to_string(),
            },
            Rule {
                title: "Commit Messages".to_string(),
                content: "Format: <type>(<scope>): <description> (JIRA-ISSUE)".to_string(),
            },
        ]
    }

    /// Check if section involves code implementation
    fn section_involves_code(&self, section: &PlanSection) -> bool {
        let text = format!(
            "{} {} {}",
            section.title.to_lowercase(),
            section.content.to_lowercase(),
            section.tasks.join(" ").to_lowercase()
        );

        text.contains("implement")
            || text.contains("code")
            || text.contains("function")
            || text.contains("struct")
            || text.contains("module")
            || section.deliverables.iter().any(|d| d.ends_with(".rs"))
    }

    /// Check if section involves documentation
    fn section_involves_documentation(&self, section: &PlanSection) -> bool {
        let text = format!(
            "{} {} {}",
            section.title.to_lowercase(),
            section.content.to_lowercase(),
            section.tasks.join(" ").to_lowercase()
        );

        text.contains("documentation")
            || text.contains("doc")
            || text.contains("readme")
            || section.deliverables.iter().any(|d| d.ends_with(".md"))
    }

    /// Check if section involves configuration files
    fn section_involves_configuration(&self, section: &PlanSection) -> bool {
        section
            .deliverables
            .iter()
            .any(|d| d.ends_with(".yaml") || d.ends_with(".toml") || d.ends_with(".yml"))
    }
}

impl Default for ContextExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Sanitize test name to valid identifier
fn sanitize_test_name(name: &str) -> String {
    name.trim()
        .strip_prefix("fn ")
        .unwrap_or(name)
        .split('(')
        .next()
        .unwrap_or(name)
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_new() {
        let extractor = ContextExtractor::new();
        assert!(!extractor.include_full_architecture);
        assert!(!extractor.include_all_rules);
    }

    #[test]
    fn test_extractor_with_full_architecture() {
        let extractor = ContextExtractor::with_full_architecture();
        assert!(extractor.include_full_architecture);
        assert!(!extractor.include_all_rules);
    }

    #[test]
    fn test_extractor_with_all_rules() {
        let extractor = ContextExtractor::with_all_rules();
        assert!(!extractor.include_full_architecture);
        assert!(extractor.include_all_rules);
    }

    #[test]
    fn test_sanitize_test_name() {
        assert_eq!(sanitize_test_name("fn test_example"), "test_example");
        assert_eq!(sanitize_test_name("test_example()"), "test_example");
        assert_eq!(sanitize_test_name("test_example"), "test_example");
    }

    #[test]
    fn test_extract_section_purpose() {
        let extractor = ContextExtractor::new();
        let section = PlanSection::new("1.1", "Test", "Content").add_task("First task".to_string());

        let purpose = extractor.extract_section_purpose(&section);
        assert_eq!(purpose, "First task");
    }

    #[test]
    fn test_extract_section_purpose_empty_tasks() {
        let extractor = ContextExtractor::new();
        let section = PlanSection::new("1.1", "Test Section", "Content");

        let purpose = extractor.extract_section_purpose(&section);
        assert_eq!(purpose, "Implement Test Section");
    }

    #[test]
    fn test_infer_file_description() {
        let extractor = ContextExtractor::new();

        assert_eq!(
            extractor.infer_file_description("src/main.rs"),
            "Rust implementation file"
        );
        assert_eq!(
            extractor.infer_file_description("docs/guide.md"),
            "Documentation file"
        );
        assert_eq!(
            extractor.infer_file_description("Cargo.toml"),
            "Configuration file"
        );
        assert_eq!(
            extractor.infer_file_description("tests/test.rs"),
            "Test implementation"
        );
    }

    #[test]
    fn test_generate_documentation_filename() {
        let extractor = ContextExtractor::new();
        let phase = Phase::new(1, "Foundation", "Setup");
        let section = PlanSection::new("1.1", "Init", "Content");

        let filename = extractor.generate_documentation_filename(&phase, &section);
        assert_eq!(filename, "phase1_1_1_implementation.md");
    }

    #[test]
    fn test_section_involves_code() {
        let extractor = ContextExtractor::new();

        let section = PlanSection::new("1.1", "Implement Parser", "Content")
            .add_deliverable("src/parser.rs".to_string());

        assert!(extractor.section_involves_code(&section));
    }

    #[test]
    fn test_section_involves_documentation() {
        let extractor = ContextExtractor::new();

        let section = PlanSection::new("1.1", "Write Documentation", "Content")
            .add_deliverable("docs/guide.md".to_string());

        assert!(extractor.section_involves_documentation(&section));
    }

    #[test]
    fn test_section_involves_configuration() {
        let extractor = ContextExtractor::new();

        let section =
            PlanSection::new("1.1", "Config", "Content").add_deliverable("config.yaml".to_string());

        assert!(extractor.section_involves_configuration(&section));
    }

    #[test]
    fn test_get_core_rules() {
        let extractor = ContextExtractor::new();
        let rules = extractor.get_core_rules();

        assert_eq!(rules.len(), 3);
        assert!(rules.iter().any(|r| r.title == "File Extensions"));
        assert!(rules.iter().any(|r| r.title == "Markdown File Naming"));
        assert!(rules.iter().any(|r| r.title == "No Emojis"));
    }

    #[test]
    fn test_get_all_rules() {
        let extractor = ContextExtractor::new();
        let rules = extractor.get_all_rules();

        assert!(rules.len() > 5);
        assert!(rules.iter().any(|r| r.title == "Error Handling"));
        assert!(rules.iter().any(|r| r.title == "Documentation"));
    }

    #[test]
    fn test_extract_deliverables_with_description() {
        let extractor = ContextExtractor::new();
        let section = PlanSection::new("1.1", "Test", "Content")
            .add_deliverable("src/main.rs: Main entry point".to_string());

        let deliverables = extractor.extract_deliverables(&section);
        assert_eq!(deliverables.len(), 1);
        assert_eq!(deliverables[0].path, "src/main.rs");
        assert_eq!(deliverables[0].description, "Main entry point");
    }

    #[test]
    fn test_extract_deliverables_without_description() {
        let extractor = ContextExtractor::new();
        let section =
            PlanSection::new("1.1", "Test", "Content").add_deliverable("src/lib.rs".to_string());

        let deliverables = extractor.extract_deliverables(&section);
        assert_eq!(deliverables.len(), 1);
        assert_eq!(deliverables[0].path, "src/lib.rs");
        assert_eq!(deliverables[0].description, "Rust implementation file");
    }
}
