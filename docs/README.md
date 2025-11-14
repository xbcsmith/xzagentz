# xzagentz Documentation

Welcome to the xzagentz documentation. This guide will help you find the information you need quickly.

## Quick Start

New to xzagentz? Start here:

- [Getting Started Tutorial](tutorials/getting_started.md) - Install and create your first component (15 minutes)
- [CLI Commands Reference](reference/cli_commands.md) - Complete command reference
- [Component Format Specification](reference/component_format.md) - YAML structure and requirements

## Documentation Organization

This documentation follows the [Diataxis Framework](https://diataxis.fr/), organizing content into four categories based on your needs:

- **Tutorials**: Learning-oriented lessons for getting started
- **How-To Guides**: Task-oriented recipes for solving specific problems
- **Explanations**: Understanding-oriented discussions of concepts and design
- **Reference**: Information-oriented technical specifications

## Tutorials (Learning-Oriented)

Step-by-step lessons to learn xzagentz concepts through hands-on practice.

- [Getting Started](tutorials/getting_started.md) - Install xzagentz, create your first component, and run validation (15 minutes)
- [Creating Custom Components](tutorials/creating_custom_component.md) - Build a custom component with language-specific sections (30 minutes)
- [Architecture to Production Workflow](tutorials/architecture_to_production_workflow.md) - Complete workflow from architecture design to implementation
- [Generating Implementation Plans](tutorials/generating_implementation_plans.md) - Create structured implementation plans from architecture

## How-To Guides (Task-Oriented)

Practical guides for accomplishing specific tasks and solving common problems.

### Working with Components

- [Author Components](how_to/authoring_components.md) - Create and structure component files
- [Validate Components](how_to/validate_components.md) - Run validation and fix common errors
- [Use Component Tiers](how_to/use_component_tiers.md) - Choose between essential and comprehensive tool tiers
- [Implement Language-Agnostic Components](how_to/implement_language_agnostic_components.md) - Create components that support multiple languages
- [Verify No YAML Frontmatter](how_to/verify_no_yaml_frontmatter.md) - Ensure proper component structure

### Architecture and Planning

- [Generate Architecture](how_to/generate_architecture.md) - Create architecture documents from templates
- [Generate Architecture with LLM](how_to/generate_architecture_with_llm.md) - Use AI assistance for architecture creation
- [Use Interactive Mode](how_to/use_interactive_mode.md) - Work with xzagentz interactively
- [Use Implementation Command](how_to/using_implementation_command.md) - Generate implementation plans

### Configuration and Resources

- [Customize Embedded Resources](how_to/customize_embedded_resources.md) - Override default resources and templates
- [Setup Custom Resources](how_to/setup_custom_resources.md) - Configure custom resource directories
- [Troubleshooting](how_to/troubleshooting.md) - Diagnose and fix common issues

### Migration

- [Migrating to v2](how_to/migrating_to_v2.md) - Upgrade from v1 to v2

## Explanation (Understanding-Oriented)

Conceptual discussions to help you understand how xzagentz works and why it is designed the way it is.

### Core Concepts

- [Architecture Overview](explanation/architecture.md) - System design and component interaction
- [Component System Design](explanation/component_system_design.md) - Why language-agnostic components and design rationale
- [Component System Overview](explanation/component_system_overview.md) - Comprehensive component system overview
- [Tier System](explanation/tier_system.md) - Tool tier philosophy and when to use each tier
- [Architecture Patterns](explanation/architecture_patterns.md) - Supported patterns and selection guidance

### Advanced Topics

- [Template System](explanation/template_system.md) - How templates work and are rendered
- [Prompt System](explanation/prompt_system.md) - Prompt system design and customization
- [Plan Management](explanation/plan_management.md) - Implementation plan structure and management

### Project Documentation

- [Development Standards](explanation/development_standards.md) - Coding standards and conventions
- [Documentation Cleanup Summary](explanation/documentation_cleanup_summary.md) - Documentation reorganization overview
- [Document Cleanup Implementation Plan](explanation/document_cleanup_implementation_plan.md) - Detailed cleanup strategy

## Reference (Information-Oriented)

Technical specifications and reference material for looking up details.

### Command Line Interface

- [CLI Commands](reference/cli_commands.md) - Complete command reference with all subcommands and flags

### Component Specifications

- [Component Format](reference/component_format.md) - Component file format specification
- [Component YAML Schema](reference/component_yaml_schema.md) - Complete YAML schema with validation rules
- [Component Configuration](reference/component_configuration.md) - Component YAML structure details

### Configuration

- [Configuration Reference](reference/configuration_reference.md) - All configuration options and defaults
- [Environment Variables](reference/environment_variables.md) - Configuration via environment variables
- [Project Config System](reference/project_config_system.md) - Project-level configuration system

## Documentation by Feature

Find documentation organized by feature area:

### Component Management

- Tutorial: [Creating Custom Components](tutorials/creating_custom_component.md)
- How-To: [Author Components](how_to/authoring_components.md)
- How-To: [Validate Components](how_to/validate_components.md)
- How-To: [Use Component Tiers](how_to/use_component_tiers.md)
- Explanation: [Component System Design](explanation/component_system_design.md)
- Explanation: [Tier System](explanation/tier_system.md)
- Reference: [Component Format](reference/component_format.md)
- Reference: [Component YAML Schema](reference/component_yaml_schema.md)

### Architecture Generation

- Tutorial: [Architecture to Production Workflow](tutorials/architecture_to_production_workflow.md)
- How-To: [Generate Architecture](how_to/generate_architecture.md)
- How-To: [Generate Architecture with LLM](how_to/generate_architecture_with_llm.md)
- Explanation: [Architecture Overview](explanation/architecture.md)
- Explanation: [Architecture Patterns](explanation/architecture_patterns.md)

### Implementation Planning

- Tutorial: [Generating Implementation Plans](tutorials/generating_implementation_plans.md)
- How-To: [Use Implementation Command](how_to/using_implementation_command.md)
- Explanation: [Plan Management](explanation/plan_management.md)

### Templates and Prompts

- Explanation: [Template System](explanation/template_system.md)
- Explanation: [Prompt System](explanation/prompt_system.md)

### Configuration

- How-To: [Customize Embedded Resources](how_to/customize_embedded_resources.md)
- How-To: [Setup Custom Resources](how_to/setup_custom_resources.md)
- Reference: [Configuration Reference](reference/configuration_reference.md)
- Reference: [Environment Variables](reference/environment_variables.md)

### CLI Usage

- Reference: [CLI Commands](reference/cli_commands.md)
- How-To: [Use Interactive Mode](how_to/use_interactive_mode.md)

## Getting Help

Each documentation page includes:

- Prerequisites clearly listed
- Step-by-step instructions with verification
- Code examples with explanations
- Troubleshooting sections for common issues
- Links to related documentation

If you cannot find what you need:

1. Check the [Troubleshooting Guide](how_to/troubleshooting.md)
2. Review the [Architecture Overview](explanation/architecture.md) for conceptual understanding
3. Consult the [CLI Commands Reference](reference/cli_commands.md) for command details
4. Check the main project [README.md](../README.md)

## Contributing to Documentation

When adding or updating documentation, follow these guidelines:

### Choose the Right Category

Use the Diataxis framework to determine where your documentation belongs:

- **Tutorials**: Teaching through hands-on examples (learning-oriented)
- **How-To Guides**: Solving specific problems (task-oriented)
- **Explanations**: Clarifying concepts and design decisions (understanding-oriented)
- **Reference**: Technical specifications and API details (information-oriented)

See the [Decision Tree](../AGENTS.md#decision-tree-where-to-put-documentation) in AGENTS.md for detailed guidance.

### Follow Naming Conventions

- Use lowercase with underscores: `getting_started.md`
- Use `.md` extension for all Markdown files
- Exception: `README.md` files use uppercase

### Include Standard Sections

Documentation should include:

- Clear title and introduction
- Prerequisites (for how-to guides and tutorials)
- Step-by-step instructions or explanations
- Code examples with language specified
- Troubleshooting section (for how-to guides)
- Links to related documentation

### Follow Documentation Standards

- No emojis anywhere in documentation
- Specify language for all code blocks
- Use descriptive link text
- Test all commands and examples
- Keep line length reasonable for readability

### Detailed Guidelines

See [AGENTS.md](../AGENTS.md) for:

- Complete documentation standards
- File naming conventions
- Code quality requirements
- Git commit message format
- Testing requirements

## External Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Diataxis Framework](https://diataxis.fr/)
- [Markdown Guide](https://www.markdownguide.org/)

## Documentation Metrics

Current documentation coverage:

- Tutorials: 4 guides
- How-To Guides: 13 task-oriented guides
- Explanations: 15 conceptual documents
- Reference: 7 technical specifications

Total: 39 documentation files covering all major features and use cases.
