//! Prompt template rendering module
//!
//! This module provides functionality for rendering prompt templates with context data.
//! Templates use Handlebars-style placeholders that are replaced with actual values.

use crate::error::Result;
use std::collections::HashMap;

/// Template section types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSection {
    /// Context section with project and phase information
    Context,
    /// Critical AGENTS.md rules relevant to the section
    CriticalRules,
    /// Task description and steps
    Task,
    /// Pre-flight checklist before starting
    PreFlightChecklist,
    /// Implementation checklist during development
    ImplementationChecklist,
    /// Expected deliverables
    Deliverables,
    /// Acceptance criteria for validation
    AcceptanceCriteria,
    /// Post-implementation review steps
    PostImplementationReview,
}

/// Prompt template with content and metadata
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    /// Template name
    pub name: String,
    /// Template version
    pub version: String,
    /// Template content with placeholders
    pub content: String,
    /// Sections included in template
    pub sections: Vec<TemplateSection>,
}

/// Context data for rendering prompts
#[derive(Debug, Clone, Default)]
pub struct PromptContext {
    /// Project metadata
    pub project_name: String,
    pub project_description: String,

    /// Phase information
    pub phase_number: usize,
    pub phase_duration: String,
    pub phase_goal: String,

    /// Section information
    pub section_number: String,
    pub section_name: String,
    pub section_purpose: String,

    /// Tasks to complete
    pub tasks: Vec<String>,

    /// Deliverable files
    pub deliverable_files: Vec<DeliverableFile>,

    /// Test files
    pub test_files: Vec<TestFile>,

    /// Documentation filename
    pub documentation_file: String,

    /// Acceptance criteria
    pub acceptance_criteria: Vec<String>,

    /// Test examples
    pub test_examples: Vec<TestExample>,

    /// Additional rules (optional)
    pub additional_rules: Vec<Rule>,

    /// Related sections (optional)
    pub related_sections: Vec<String>,

    /// Architecture context (optional)
    pub architecture_overview: String,

    /// Related components (optional)
    pub related_components: Vec<String>,

    /// Dependencies (optional)
    pub dependencies: Vec<String>,

    /// Relevant AGENTS.md rules
    pub relevant_rules: Vec<Rule>,

    /// JIRA issue (for commit messages)
    pub jira_issue: String,

    /// Tool version
    pub version: String,

    /// Timestamp
    pub timestamp: String,
}

/// Deliverable file information
#[derive(Debug, Clone)]
pub struct DeliverableFile {
    /// File path
    pub path: String,
    /// File description
    pub description: String,
}

/// Test file information
#[derive(Debug, Clone)]
pub struct TestFile {
    /// File path
    pub path: String,
    /// Test description
    pub description: String,
}

/// Test example
#[derive(Debug, Clone)]
pub struct TestExample {
    /// Test function name
    pub name: String,
    /// Test description
    pub description: String,
}

/// Additional rule
#[derive(Debug, Clone)]
pub struct Rule {
    /// Rule title
    pub title: String,
    /// Rule content
    pub content: String,
}

impl PromptTemplate {
    /// Create a new prompt template
    ///
    /// # Arguments
    ///
    /// * `name` - Template name
    /// * `content` - Template content with placeholders
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::prompts::template::PromptTemplate;
    ///
    /// let template = PromptTemplate::new(
    ///     "section_prompt".to_string(),
    ///     "# Section {{section_number}}".to_string()
    /// );
    /// assert_eq!(template.name, "section_prompt");
    /// ```
    pub fn new(name: String, content: String) -> Self {
        Self {
            name,
            version: "1.0.0".to_string(),
            content,
            sections: vec![
                TemplateSection::Context,
                TemplateSection::CriticalRules,
                TemplateSection::Task,
                TemplateSection::PreFlightChecklist,
                TemplateSection::ImplementationChecklist,
                TemplateSection::Deliverables,
                TemplateSection::AcceptanceCriteria,
                TemplateSection::PostImplementationReview,
            ],
        }
    }

    /// Load template from embedded file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xzagentz::prompts::template::PromptTemplate;
    ///
    /// let template = PromptTemplate::load_embedded("section_prompt").unwrap();
    /// assert_eq!(template.name, "section_prompt");
    /// ```
    pub fn load_embedded(name: &str) -> Result<Self> {
        let content = match name {
            "section_prompt" => {
                include_str!("../../templates/prompts/section_prompt.md").to_string()
            }
            _ => return Err(crate::error::Error::TemplateNotFound(name.to_string())),
        };

        Ok(Self::new(name.to_string(), content))
    }

    /// Render template with context
    ///
    /// # Arguments
    ///
    /// * `context` - Context data for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use xzagentz::prompts::template::{PromptTemplate, PromptContext};
    ///
    /// let template = PromptTemplate::new(
    ///     "test".to_string(),
    ///     "Section {{section_number}}: {{section_name}}".to_string()
    /// );
    ///
    /// let mut context = PromptContext::default();
    /// context.section_number = "1.1".to_string();
    /// context.section_name = "Foundation".to_string();
    ///
    /// let result = template.render(&context).unwrap();
    /// assert_eq!(result, "Section 1.1: Foundation");
    /// ```
    pub fn render(&self, context: &PromptContext) -> Result<String> {
        let mut result = self.content.clone();

        // Build replacement map with owned strings
        let phase_number_str = context.phase_number.to_string();

        let mut replacements = HashMap::new();
        replacements.insert("project_name", context.project_name.as_str());
        replacements.insert("project_description", context.project_description.as_str());
        replacements.insert("phase_number", phase_number_str.as_str());
        replacements.insert("phase_duration", context.phase_duration.as_str());
        replacements.insert("phase_goal", context.phase_goal.as_str());
        replacements.insert("section_number", context.section_number.as_str());
        replacements.insert("section_name", context.section_name.as_str());
        replacements.insert("section_purpose", context.section_purpose.as_str());
        replacements.insert("documentation_file", context.documentation_file.as_str());
        replacements.insert("jira_issue", context.jira_issue.as_str());
        replacements.insert("version", context.version.as_str());
        replacements.insert("timestamp", context.timestamp.as_str());
        replacements.insert(
            "architecture_overview",
            context.architecture_overview.as_str(),
        );

        // Simple placeholder replacement
        for (key, value) in replacements {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        // Handle task list
        result = self.render_list(&result, "tasks", &context.tasks)?;

        // Handle acceptance criteria list
        result = self.render_list(&result, "acceptance_criteria", &context.acceptance_criteria)?;

        // Handle deliverable files
        result = self.render_deliverable_files(&result, &context.deliverable_files)?;

        // Handle test files
        result = self.render_test_files(&result, &context.test_files)?;

        // Handle test examples
        result = self.render_test_examples(&result, &context.test_examples)?;

        // Handle additional rules
        result = self.render_additional_rules(&result, &context.additional_rules)?;

        // Handle related sections
        result = self.render_related_sections(&result, &context.related_sections)?;

        // Handle related components
        result = self.render_list(&result, "related_components", &context.related_components)?;

        // Handle dependencies
        result = self.render_list(&result, "dependencies", &context.dependencies)?;

        // Handle relevant rules
        result = self.render_additional_rules(&result, &context.relevant_rules)?;

        Ok(result)
    }

    /// Render a simple list with iteration
    fn render_list(&self, content: &str, list_name: &str, items: &[String]) -> Result<String> {
        let start_pattern = format!("{{{{#each {}}}}}", list_name);
        let end_pattern = "{{/each}}".to_string();

        if let Some(start_idx) = content.find(&start_pattern) {
            if let Some(end_idx) = content.find(&end_pattern) {
                let before = &content[..start_idx];
                let template = &content[start_idx + start_pattern.len()..end_idx];
                let after = &content[end_idx + end_pattern.len()..];

                let mut rendered_items = String::new();
                for (idx, item) in items.iter().enumerate() {
                    let mut item_content = template.replace("{{this}}", item);
                    item_content = item_content.replace("{{@index}}", &(idx + 1).to_string());
                    rendered_items.push_str(&item_content);
                }

                return Ok(format!("{}{}{}", before, rendered_items, after));
            }
        }

        Ok(content.to_string())
    }

    /// Render deliverable files section
    fn render_deliverable_files(&self, content: &str, files: &[DeliverableFile]) -> Result<String> {
        let start_pattern = "{{#each deliverable_files}}";
        let end_pattern = "{{/each}}";

        if let Some(start_idx) = content.find(start_pattern) {
            if let Some(end_idx) = content.find(end_pattern) {
                let before = &content[..start_idx];
                let template = &content[start_idx + start_pattern.len()..end_idx];
                let after = &content[end_idx + end_pattern.len()..];

                let mut rendered = String::new();
                for file in files {
                    let mut item = template.replace("{{this.path}}", &file.path);
                    item = item.replace("{{this.description}}", &file.description);
                    rendered.push_str(&item);
                }

                return Ok(format!("{}{}{}", before, rendered, after));
            }
        }

        Ok(content.to_string())
    }

    /// Render test files section
    fn render_test_files(&self, content: &str, files: &[TestFile]) -> Result<String> {
        let start_pattern = "{{#each test_files}}";
        let end_pattern = "{{/each}}";

        if let Some(start_idx) = content.find(start_pattern) {
            if let Some(end_idx) = content.find(end_pattern) {
                let before = &content[..start_idx];
                let template = &content[start_idx + start_pattern.len()..end_idx];
                let after = &content[end_idx + end_pattern.len()..];

                let mut rendered = String::new();
                for file in files {
                    let mut item = template.replace("{{this.path}}", &file.path);
                    item = item.replace("{{this.description}}", &file.description);
                    rendered.push_str(&item);
                }

                return Ok(format!("{}{}{}", before, rendered, after));
            }
        }

        Ok(content.to_string())
    }

    /// Render test examples section
    fn render_test_examples(&self, content: &str, examples: &[TestExample]) -> Result<String> {
        let start_pattern = "{{#each test_examples}}";
        let end_pattern = "{{/each}}";

        if let Some(start_idx) = content.find(start_pattern) {
            if let Some(end_idx) = content.find(end_pattern) {
                let before = &content[..start_idx];
                let template = &content[start_idx + start_pattern.len()..end_idx];
                let after = &content[end_idx + end_pattern.len()..];

                let mut rendered = String::new();
                for example in examples {
                    let mut item = template.replace("{{this.name}}", &example.name);
                    item = item.replace("{{this.description}}", &example.description);
                    rendered.push_str(&item);
                }

                return Ok(format!("{}{}{}", before, rendered, after));
            }
        }

        Ok(content.to_string())
    }

    /// Render additional rules section
    fn render_additional_rules(&self, content: &str, rules: &[Rule]) -> Result<String> {
        if rules.is_empty() {
            // Remove the entire conditional block if no rules
            let if_pattern = "{{#if additional_rules}}";
            let endif_pattern = "{{/if}}";

            if let Some(start_idx) = content.find(if_pattern) {
                if let Some(end_idx) = content.find(endif_pattern) {
                    let before = &content[..start_idx];
                    let after = &content[end_idx + endif_pattern.len()..];
                    return Ok(format!("{}{}", before, after));
                }
            }
            return Ok(content.to_string());
        }

        let mut result = content.to_string();

        // Remove the if/endif markers
        result = result.replace("{{#if additional_rules}}", "");
        result = result.replace("{{/if}}", "");

        // Render each rule
        let start_pattern = "{{#each additional_rules}}";
        let end_pattern = "{{/each}}";

        if let Some(start_idx) = result.find(start_pattern) {
            if let Some(end_idx) = result.find(end_pattern) {
                let before = &result[..start_idx];
                let template = &result[start_idx + start_pattern.len()..end_idx];
                let after = &result[end_idx + end_pattern.len()..];

                let mut rendered = String::new();
                for rule in rules {
                    let mut item = template.replace("{{this.title}}", &rule.title);
                    item = item.replace("{{this.content}}", &rule.content);
                    rendered.push_str(&item);
                }

                return Ok(format!("{}{}{}", before, rendered, after));
            }
        }

        Ok(result)
    }

    /// Render related sections
    fn render_related_sections(&self, content: &str, sections: &[String]) -> Result<String> {
        if sections.is_empty() {
            // Remove the entire conditional block
            let if_pattern = "{{#if related_sections}}";
            let endif_pattern = "{{/if}}";

            if let Some(start_idx) = content.find(if_pattern) {
                if let Some(end_idx) = content.find(endif_pattern) {
                    let before = &content[..start_idx];
                    let after = &content[end_idx + endif_pattern.len()..];
                    return Ok(format!("{}{}", before, after));
                }
            }
            return Ok(content.to_string());
        }

        let mut result = content.to_string();

        // Remove the if/endif markers
        result = result.replace("{{#if related_sections}}", "");
        result = result.replace("{{/if}}", "");

        // Render the list
        let start_pattern = "{{#each related_sections}}";
        let end_pattern = "{{/each}}";

        if let Some(start_idx) = result.find(start_pattern) {
            if let Some(end_idx) = result.find(end_pattern) {
                let before = &result[..start_idx];
                let template = &result[start_idx + start_pattern.len()..end_idx];
                let after = &result[end_idx + end_pattern.len()..];

                let mut rendered = String::new();
                for (idx, section) in sections.iter().enumerate() {
                    let mut item = template.replace("{{this}}", section);
                    // Handle unless @last for comma separation
                    if template.contains("{{#unless @last}}") {
                        if idx < sections.len() - 1 {
                            item = item.replace("{{#unless @last}}", "");
                            item = item.replace("{{/unless}}", "");
                        } else {
                            // Remove the unless block content for last item
                            if let Some(unless_start) = item.find("{{#unless @last}}") {
                                if let Some(unless_end) = item.find("{{/unless}}") {
                                    let before_unless = &item[..unless_start];
                                    let after_unless = &item[unless_end + "{{/unless}}".len()..];
                                    item = format!("{}{}", before_unless, after_unless);
                                }
                            }
                        }
                    }
                    rendered.push_str(&item);
                }

                return Ok(format!("{}{}{}", before, rendered, after));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_new() {
        let template = PromptTemplate::new("test".to_string(), "content".to_string());
        assert_eq!(template.name, "test");
        assert_eq!(template.content, "content");
        assert_eq!(template.sections.len(), 8);
    }

    #[test]
    fn test_simple_placeholder_replacement() {
        let template = PromptTemplate::new(
            "test".to_string(),
            "Section {{section_number}}: {{section_name}}".to_string(),
        );

        let context = PromptContext {
            section_number: "1.1".to_string(),
            section_name: "Foundation".to_string(),
            ..Default::default()
        };

        let result = template.render(&context).unwrap();
        assert_eq!(result, "Section 1.1: Foundation");
    }

    #[test]
    fn test_render_task_list() {
        let template = PromptTemplate::new(
            "test".to_string(),
            "{{#each tasks}}\n{{@index}}. {{this}}\n{{/each}}".to_string(),
        );

        let context = PromptContext {
            tasks: vec!["Task 1".to_string(), "Task 2".to_string()],
            ..Default::default()
        };

        let result = template.render(&context).unwrap();
        assert!(result.contains("1. Task 1"));
        assert!(result.contains("2. Task 2"));
    }

    #[test]
    fn test_render_deliverable_files() {
        let template = PromptTemplate::new(
            "test".to_string(),
            "{{#each deliverable_files}}\n- {{this.path}}: {{this.description}}\n{{/each}}"
                .to_string(),
        );

        let context = PromptContext {
            deliverable_files: vec![DeliverableFile {
                path: "src/main.rs".to_string(),
                description: "Main entry point".to_string(),
            }],
            ..Default::default()
        };

        let result = template.render(&context).unwrap();
        assert!(result.contains("src/main.rs"));
        assert!(result.contains("Main entry point"));
    }

    #[test]
    fn test_render_additional_rules_empty() {
        let template = PromptTemplate::new(
            "test".to_string(),
            "Start\n{{#if additional_rules}}\nRules here\n{{/if}}\nEnd".to_string(),
        );

        let context = PromptContext::default();
        let result = template.render(&context).unwrap();
        assert_eq!(result, "Start\n\nEnd");
    }

    #[test]
    fn test_render_additional_rules_with_data() {
        let template = PromptTemplate::new(
            "test".to_string(),
            "{{#if additional_rules}}{{#each additional_rules}}{{this.title}}: {{this.content}}\n{{/each}}{{/if}}".to_string(),
        );

        let context = PromptContext {
            additional_rules: vec![Rule {
                title: "Rule 1".to_string(),
                content: "Content 1".to_string(),
            }],
            ..Default::default()
        };

        let result = template.render(&context).unwrap();
        assert!(result.contains("Rule 1"));
        assert!(result.contains("Content 1"));
    }

    #[test]
    fn test_load_embedded_section_prompt() {
        let result = PromptTemplate::load_embedded("section_prompt");
        assert!(result.is_ok());
        let template = result.unwrap();
        assert_eq!(template.name, "section_prompt");
        assert!(template.content.contains("Implementation Prompt"));
    }

    #[test]
    fn test_load_embedded_unknown_template() {
        let result = PromptTemplate::load_embedded("unknown");
        assert!(result.is_err());
    }
}
