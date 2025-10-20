# xzagentz Documentation

Welcome to the xzagentz event tracking server documentation.

## About xzagentz

xzagentz is a component-based system for managing project-specific AGENTS.md files, creating architecture plans, and implementation plans using Rust.

## Documentation Structure

This documentation follows the [Diataxis Framework](https://diataxis.fr/),
organizing content into four categories based on your needs:

### Tutorials

**Learning-oriented** - Step-by-step lessons to help you learn by doing.

- [Getting Started](tutorials/getting_started.md) - Your first steps with xzagentz
- [Building Your First Event Tracker](tutorials/first_event_tracker.md) - Create
  a simple event tracking application

Start here if you're new to xzagentz and want to learn the basics through hands-on
practice.

### How-to Guides

**Task-oriented** - Practical guides to solve specific problems.

Use these guides when you need to accomplish a specific task or solve a
particular problem.

### Explanations

**Understanding-oriented** - Conceptual discussions to deepen your knowledge.

- [Architecture Overview](explanations/architecture.md) - System design and
  component interaction

Read these to understand the concepts and reasoning behind xzagentz's design.

### Reference

**Information-oriented** - Technical specifications and details.

- [API Reference](reference/api.md) - Complete REST API documentation
- [Configuration Reference](reference/configuration.md) - All configuration
  options
- [Environment Variables](reference/environment_variables.md) - Configuration
  via environment

Consult these when you need to look up specific information or details.

## Getting Help

Each documentation page includes:

- Prerequisites clearly listed
- Step-by-step instructions with verification
- Troubleshooting sections for common issues
- Links to related documentation

If you can't find what you need:

1. Check the relevant category's README for more documents
2. Look at the troubleshooting sections in how-to guides
3. Review the project's main [README.md](../README.md)

## Contributing to Documentation

When adding or updating documentation:

1. **Choose the right category:**

   - Tutorials: Teaching through hands-on examples
   - How-to: Solving specific problems
   - Explanations: Clarifying concepts
   - Reference: Technical specifications

2. **Follow naming conventions:**

   - Use lowercase with underscores: `getting_started.md`
   - Exception: `README.md` files use uppercase

3. **Include standard sections:**

   - Clear title and introduction
   - Prerequisites (for how-to and tutorials)
   - Code examples with explanations
   - Troubleshooting (for how-to guides)
   - Related links

4. **Follow markdown standards:**
   - Use `.markdownlint.json` rules
   - No emojis
   - Specify language for code blocks
   - Test all commands and examples

See [AGENTS.md](../AGENTS.md) for detailed documentation guidelines.

## External Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Redpanda Documentation](https://docs.redpanda.com/)
- [Axum Web Framework](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Diataxis Framework](https://diataxis.fr/)
