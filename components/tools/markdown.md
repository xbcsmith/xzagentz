# Markdown Guidelines and Best Practices

This section provides Markdown-specific guidelines for documentation in this project.

---

## File Naming Convention

**MANDATORY RULES:**

- Use lowercase letters ONLY
- Use underscores to separate words
- Use `.md` extension (NOT `.MD` or `.markdown`)
- **Exception**: `README.md` is the ONLY uppercase filename allowed

**Examples:**
```text
✅ CORRECT:
   docs/architecture_overview.md
   docs/how_to_setup.md
   docs/api_reference.md
   README.md (only exception)

❌ WRONG:
   docs/Architecture-Overview.md
   docs/ArchitectureOverview.md
   docs/ARCHITECTURE.md
   docs/architecture.MD
   docs/architecture.markdown
```

---

## Document Structure

### Frontmatter (Optional)

```markdown
---
title: Document Title
date: 2024-01-01
author: Author Name
tags: [tag1, tag2]
---
```

### Headers

**Use ATX-style headers (# prefix):**

```markdown
# H1 - Document Title

## H2 - Major Section

### H3 - Subsection

#### H4 - Minor Subsection

##### H5 - Rarely Used

###### H6 - Rarely Used
```

**Header Rules:**
- One H1 per document (document title)
- Use sentence case (capitalize first word only)
- No trailing punctuation
- Leave blank line before and after headers
- Don't skip header levels

**Examples:**
```markdown
✅ CORRECT:
# Project overview

## Installation

### Prerequisites

❌ WRONG:
# Project Overview.          # Trailing period
## installation              # Should be capitalized
#### Details                 # Skipped H3
##Installation              # No space after ##
```

---

## Lists

### Unordered Lists

**Use hyphens (-) for consistency:**

```markdown
- First item
- Second item
  - Nested item
  - Another nested item
- Third item
```

**List Rules:**
- Use hyphens, not asterisks or plus signs
- Indent nested lists with 2 spaces
- Leave blank line before and after list
- Use consistent spacing

### Ordered Lists

```markdown
1. First step
2. Second step
   1. Sub-step A
   2. Sub-step B
3. Third step
```

### Task Lists

```markdown
- [ ] Incomplete task
- [x] Completed task
- [ ] Another incomplete task
```

---

## Code Blocks

### Inline Code

**Use single backticks:**

```markdown
Use the `cargo build` command to compile the project.

The `ComponentLoader` struct handles file loading.
```

### Fenced Code Blocks

**ALWAYS specify language identifier:**

```markdown
✅ CORRECT:
\`\`\`rust
fn main() {
    println!("Hello, world!");
}
\`\`\`

\`\`\`bash
cargo build --release
\`\`\`

\`\`\`yaml
config:
  enabled: true
\`\`\`

❌ WRONG:
\`\`\`
fn main() {
    println!("Hello");
}
\`\`\`
```

**Common Language Identifiers:**
- `rust` - Rust code
- `bash` / `sh` - Shell commands
- `yaml` - YAML configuration
- `toml` - TOML configuration
- `json` - JSON data
- `text` - Plain text
- `markdown` - Markdown examples

---

## Links

### Inline Links

```markdown
See the [Rust documentation](https://doc.rust-lang.org/) for details.

Check out [our guide](./guides/setup.md) for setup instructions.
```

### Reference Links

```markdown
See the [Rust Book][rust-book] and [Cargo Book][cargo-book].

[rust-book]: https://doc.rust-lang.org/book/
[cargo-book]: https://doc.rust-lang.org/cargo/
```

### Internal Links

```markdown
See the [Installation](#installation) section below.

Jump to [Chapter 2](./chapter_2.md).
```

---

## Emphasis

### Bold and Italic

```markdown
Use **bold** for strong emphasis.

Use *italic* for mild emphasis.

Use ***bold italic*** for very strong emphasis.
```

### Strikethrough

```markdown
~~This text is struck through~~
```

---

## Block Quotes

```markdown
> This is a quote.
> It can span multiple lines.
>
> And multiple paragraphs.
```

**Nested Quotes:**

```markdown
> Outer quote
>
> > Nested quote
> >
> > > Double nested quote
```

---

## Tables

### Basic Table

```markdown
| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Row 1    | Data     | More     |
| Row 2    | Data     | More     |
```

### Alignment

```markdown
| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left         | Center         | Right         |
| Data         | Data           | Data          |
```

**Table Rules:**
- Use pipes (|) for column separators
- Align pipes vertically for readability
- Use colons for alignment
- Keep tables simple (avoid complex nesting)

---

## Horizontal Rules

```markdown
---

Use three or more hyphens, asterisks, or underscores.
Prefer hyphens for consistency.

---
```

---

## Images

### Inline Images

```markdown
![Alt text](path/to/image.png)

![Architecture Diagram](./diagrams/architecture.png)
```

### Images with Links

```markdown
[![Alt text](image.png)](https://link-destination.com)
```

### Image Sizing (GitHub-flavored)

```markdown
<img src="image.png" width="500" alt="Description">
```

---

## Escaping Characters

**Escape special markdown characters:**

```markdown
\* Not italic \*
\# Not a header
\` Not code \`
\[Not a link\]
```

---

## No Emojis Rule

**CRITICAL: Never use emojis in markdown files**

```markdown
❌ WRONG:
# Setup Guide 🚀

## Prerequisites ✅

❌ Still wrong:
- ✓ Item done
- ❌ Item failed

✅ CORRECT:
# Setup Guide

## Prerequisites

Use words:
- [x] Item done
- [ ] Item failed
```

---

## Documentation Organization

### Diataxis Framework

Organize documentation into four categories:

**1. Tutorials** (`docs/tutorials/`)
- Learning-oriented
- Step-by-step lessons
- Hands-on examples

**2. How-To Guides** (`docs/how_to/`)
- Task-oriented
- Problem-solving recipes
- Specific instructions

**3. Explanations** (`docs/explanations/`)
- Understanding-oriented
- Conceptual discussions
- Architecture and design decisions

**4. Reference** (`docs/reference/`)
- Information-oriented
- Technical specifications
- API documentation

---

## Table of Contents

**For long documents, add TOC:**

```markdown
## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)
```

---

## Best Practices

### Writing Style

- Use clear, concise language
- Write in present tense
- Use active voice
- Be consistent with terminology
- Define acronyms on first use

### Document Organization

- Start with overview/introduction
- Group related information
- Use hierarchical structure
- Include examples
- Add troubleshooting section

### Code Examples

- Always specify language
- Include complete, runnable examples
- Add comments for clarity
- Show expected output
- Test code examples

### Maintenance

- Keep documentation up to date
- Date-stamp major updates
- Version control documentation
- Review regularly
- Remove outdated content

---

## Common Patterns

### Prerequisites Section

```markdown
## Prerequisites

Before starting, ensure you have:

- Rust 1.70 or later installed
- Git version control
- Basic knowledge of Rust
- Text editor or IDE
```

### Installation Instructions

```markdown
## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/user/repo.git
   cd repo
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```
```

### Troubleshooting Section

```markdown
## Troubleshooting

### Issue: Build fails with linker error

**Symptoms:**
```text
error: linker `cc` not found
```

**Solution:**
Install build tools:
```bash
sudo apt install build-essential
```
```

---

## Validation Checklist

Before committing markdown files, verify:

- [ ] Filename uses lowercase_with_underscores.md
- [ ] All code blocks have language identifiers
- [ ] Headers follow proper hierarchy
- [ ] No emojis used anywhere
- [ ] Links are valid and working
- [ ] Tables are properly formatted
- [ ] Images have alt text
- [ ] No trailing whitespace
- [ ] File ends with newline

---

## Additional Resources

- **Markdown Guide**: https://www.markdownguide.org/
- **CommonMark Spec**: https://commonmark.org/
- **GitHub Flavored Markdown**: https://github.github.com/gfm/
- **Diataxis Framework**: https://diataxis.fr/
