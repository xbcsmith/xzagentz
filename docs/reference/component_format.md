# Component Format Reference

This document specifies the format and structure requirements for component files in the xzagentz project.

---

## Overview

Components are reusable markdown files that can be assembled into complete AGENTS.md files. Each component represents a specific section or topic (e.g., Rust guidelines, Git conventions, testing requirements).

---

## File Location

Components MUST be placed in the `components/` directory, organized by category:

```
components/
├── core/              # Essential components (header, critical rules)
├── languages/         # Language-specific guidelines (rust, python, golang)
├── tools/             # Tool-specific guidelines (git, markdown)
└── general/           # General development topics (testing, documentation)
```

---

## File Naming Convention

**MANDATORY RULES:**

- Use lowercase letters ONLY
- Use underscores to separate words
- Use `.md` extension (NOT `.yml` or other)
- Be descriptive and specific

**Examples:**

```text
✅ CORRECT:
   components/core/critical_rules.md
   components/languages/rust.md
   components/tools/git.md
   components/general/testing.md

❌ WRONG:
   components/core/Critical-Rules.md     # Mixed case
   components/core/CriticalRules.md      # CamelCase
   components/languages/Rust.MD          # Wrong extension case
   components/tools/git-guide.md         # Hyphens instead of underscores
```

---

## Component Structure

### Basic Template

```markdown
# Component Title

Brief introduction explaining the purpose of this component.

---

## Section 1

Content for the first major section.

### Subsection 1.1

Detailed content with examples.

### Subsection 1.2

More detailed content.

---

## Section 2

Content for the second major section.

---

## Additional Resources

- Link to relevant documentation
- Link to external resources
```

### Required Elements

1. **Title Header (H1)**: One top-level header describing the component
2. **Introduction**: Brief paragraph explaining purpose
3. **Section Dividers**: Use `---` to separate major sections
4. **Hierarchical Headers**: Use H2 (`##`) for main sections, H3 (`###`) for subsections
5. **Content**: Markdown-formatted content with examples

### Optional Elements

- Table of contents (for long components)
- Code examples with language identifiers
- Tables for reference information
- Blockquotes for notes or warnings
- Lists (ordered or unordered)
- Links to related components or external resources

---

## Code Block Requirements

### Language Identifiers

**ALWAYS specify language in code blocks:**

```markdown
✅ CORRECT:
\`\`\`rust
fn main() {
    println!("Hello!");
}
\`\`\`

\`\`\`bash
cargo build --release
\`\`\`

❌ WRONG:
\`\`\`
fn main() {
    println!("Hello!");
}
\`\`\`
```

### Supported Languages

Common language identifiers to use:

- `rust` - Rust code
- `bash` / `sh` - Shell commands
- `toml` - TOML configuration
- `yaml` - YAML configuration
- `json` - JSON data
- `text` - Plain text
- `python` - Python code
- `go` - Go code
- `markdown` - Markdown examples
- `ini` - INI configuration

---

## Placeholder Support

Components may include placeholders that will be replaced when assembled into AGENTS.md:

### Standard Placeholders

- `{{PROJECT_NAME}}` - Name of the project
- `{{VERSION}}` - Current version number
- `{{UPDATED_AT}}` - Last update timestamp
- `{{PROJECT_TYPE}}` - Type of project (e.g., "Rust CLI")
- `{{PRIMARY_LANGUAGE}}` - Primary programming language
- `{{PROJECT_DESCRIPTION}}` - Project description

### Usage Example

```markdown
# {{PROJECT_NAME}} - AI Agent Development Guidelines

**Version**: {{VERSION}}
**Last Updated**: {{UPDATED_AT}}
**Project Type**: {{PROJECT_TYPE}}
```

When assembled, these will be replaced with actual values:

```markdown
# xzagentz - AI Agent Development Guidelines

**Version**: 0.1.0
**Last Updated**: 2024-01-15
**Project Type**: Rust CLI Tool
```

---

## Formatting Rules

### No Emojis

**CRITICAL**: Components MUST NOT contain emojis

```markdown
❌ WRONG:
# Git Guidelines 🚀
## Prerequisites ✅

✅ CORRECT:
# Git Guidelines
## Prerequisites
```

### Consistent Formatting

- Use ATX-style headers (`#` prefix)
- Use hyphens (`-`) for unordered lists
- Use `1.` for ordered lists
- Leave blank lines before and after headers
- Leave blank lines before and after code blocks
- Use horizontal rules (`---`) to separate major sections

### Example Structure

```markdown
# Component Title

Introduction paragraph.

---

## Major Section

Content here.

### Subsection

More content.

```language
code example
```

---

## Next Section

More content.
```

---

## Content Guidelines

### Writing Style

- Use clear, concise language
- Write in present tense
- Use imperative mood for instructions
- Be specific and actionable
- Include examples for complex concepts

### Examples and Code

- Provide runnable examples when possible
- Show both correct and incorrect patterns
- Include comments in code examples
- Test code examples before committing

### Best Practices Format

Use this format for showing good vs. bad examples:

```markdown
**Examples:**
```text
✅ CORRECT:
   example of correct usage

❌ WRONG:
   example of incorrect usage
```
```

### Rules and Requirements

Use this format for mandatory requirements:

```markdown
**YOU MUST:**
- Requirement 1
- Requirement 2

**NEVER:**
- Anti-pattern 1
- Anti-pattern 2
```

---

## Component Categories

### Core Components

**Purpose**: Essential project information and critical rules

**Examples**:
- `header.md` - Project header and overview
- `critical_rules.md` - Non-negotiable requirements
- `learning_resources.md` - Educational materials

**Characteristics**:
- Required for all AGENTS.md files
- Should be comprehensive
- Contains project-wide rules

### Language Components

**Purpose**: Programming language-specific guidelines

**Examples**:
- `rust.md` - Rust coding standards
- `python.md` - Python best practices
- `golang.md` - Go conventions

**Characteristics**:
- Language-specific patterns
- Toolchain requirements
- Common idioms and anti-patterns

### Tool Components

**Purpose**: Tool and technology-specific guides

**Examples**:
- `git.md` - Git workflow and conventions
- `markdown.md` - Markdown formatting rules

**Characteristics**:
- Tool-specific workflows
- Configuration examples
- Common commands

### General Components

**Purpose**: Cross-cutting development topics

**Examples**:
- `development.md` - General dev practices
- `testing.md` - Testing strategies
- `documentation.md` - Documentation standards

**Characteristics**:
- Language-agnostic
- Process and workflow focused
- Broadly applicable

---

## Component Metadata

Components may optionally include YAML frontmatter for metadata:

```markdown
---
title: Component Title
category: core
required: true
order: 10
tags: [rust, testing]
version: 1.0.0
---

# Component Title

Content starts here...
```

### Metadata Fields

- `title`: Display name of component
- `category`: Category (core, languages, tools, general)
- `required`: Whether component is required in AGENTS.md
- `order`: Sort order when assembling (lower numbers first)
- `tags`: Keywords for categorization
- `version`: Component version number

---

## Validation Rules

### File Structure

Components will be validated for:

- Correct file naming (lowercase, underscores, `.md`)
- Proper markdown structure
- Language identifiers in code blocks
- No emojis in content
- Valid placeholder syntax
- Proper header hierarchy

### Content Requirements

- At least one H1 header
- No more than one H1 header
- Headers in proper hierarchy (don't skip levels)
- Code blocks have language identifiers
- Links are well-formed
- No trailing whitespace

---

## Assembly Process

When components are assembled into AGENTS.md:

1. Components are loaded from specified categories
2. Placeholders are replaced with actual values
3. Components are concatenated in order
4. Final validation is performed
5. Output is written to AGENTS.md

### Assembly Order

Default assembly order by category:

1. `core/header.md`
2. `core/critical_rules.md`
3. Language components (as specified)
4. Tool components (as specified)
5. General components (as specified)
6. `core/learning_resources.md`

---

## Examples

### Minimal Component

```markdown
# Testing Guidelines

This component provides testing best practices.

---

## Unit Testing

Write unit tests for all public functions.

```rust
#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}
```

---

## Additional Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
```

### Full Component with Metadata

```markdown
---
title: Rust Language Guidelines
category: languages
required: true
order: 20
tags: [rust, coding-standards]
version: 1.0.0
---

# Rust Language Guidelines

Comprehensive Rust coding standards and best practices for {{PROJECT_NAME}}.

---

## Code Style

Use rustfmt for all code:

```bash
cargo fmt --all
```

---

## Error Handling

Always use Result for recoverable errors:

```rust
pub fn load_file(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e))
}
```

---

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)
```

---

## Best Practices

### Component Design

- **Single Responsibility**: Each component covers one topic
- **Self-Contained**: Component makes sense on its own
- **Reusable**: Can be used in different project contexts
- **Versioned**: Track changes to component content
- **Tested**: Validate component structure and content

### Maintenance

- Keep components up to date with project changes
- Version components when making significant changes
- Review and update examples regularly
- Remove outdated content
- Test code examples before committing

### Documentation

- Include clear introduction
- Provide context for rules and guidelines
- Show examples of both good and bad patterns
- Link to external resources
- Explain the "why" not just the "what"

---

## Troubleshooting

### Common Issues

**Issue**: Component not loading

- Check file naming convention
- Verify file is in correct directory
- Check for YAML frontmatter errors

**Issue**: Placeholders not replaced

- Verify placeholder syntax: `{{NAME}}`
- Check placeholder is defined in configuration
- Ensure no extra spaces in placeholder

**Issue**: Validation failing

- Check all code blocks have language identifiers
- Verify no emojis in content
- Check header hierarchy is correct
- Ensure no trailing whitespace

---

## Validation Checklist

Before committing a component, verify:

- [ ] Filename uses lowercase_with_underscores.md
- [ ] File is in correct category directory
- [ ] Has exactly one H1 header
- [ ] Headers follow proper hierarchy
- [ ] All code blocks have language identifiers
- [ ] No emojis anywhere in content
- [ ] Placeholders use correct syntax
- [ ] Examples are tested and working
- [ ] Links are valid
- [ ] No trailing whitespace
- [ ] File ends with newline

---

## Additional Resources

- [Markdown Guide](https://www.markdownguide.org/)
- [CommonMark Spec](https://commonmark.org/)
- [Diataxis Framework](https://diataxis.fr/)
