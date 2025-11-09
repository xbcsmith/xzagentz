---
component:
  name: markdown_essential
  category: tools
  version: 1.0.0
  tier: essential
  description: Essential Markdown guidelines for documentation
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: file_naming
      language_specific: false
      required: true
    - id: document_structure
      language_specific: false
      required: true
    - id: code_blocks
      language_specific: true
      required: true
    - id: formatting
      language_specific: false
      required: true
---

# Markdown Essential Guidelines

Core Markdown guidelines for consistent documentation.

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
```

---

## Document Structure

### Headers

**Use ATX-style headers (# prefix):**

```text
# H1 - Document Title

## H2 - Major Section

### H3 - Subsection

#### H4 - Minor Subsection
```

**Header Rules:**

- One H1 per document (document title)
- Use sentence case (capitalize first word only)
- No trailing punctuation
- Leave blank line before and after headers
- Don't skip header levels

**Examples:**

```text
✅ CORRECT:
# Project overview

## Installation

### Prerequisites

❌ WRONG:
# Project Overview.          (trailing period)
## installation              (not capitalized)
#### Details                 (skipped H3)
```

---

## Lists

### Unordered Lists

**Use hyphens (-) for consistency:**

```text
- First item
- Second item
  - Nested item
  - Another nested item
- Third item
```

**Rules:**

- Use hyphens, not asterisks or plus signs
- Indent nested lists with 2 spaces
- Leave blank line before and after list

### Ordered Lists

```text
1. First step
2. Second step
   1. Sub-step A
   2. Sub-step B
3. Third step
```

### Task Lists

```text
- [ ] Incomplete task
- [x] Completed task
- [ ] Another task
```

---

## Code Blocks

### Inline Code

**Use single backticks:**

```text
Use the `build` command to compile.

The `ComponentLoader` struct handles file loading.
```

### Fenced Code Blocks

**ALWAYS specify language identifier:**

<!-- LANG:* -->

```text
✅ CORRECT:
```rust
fn main() {
    println!("Hello!");
}
```

❌ WRONG:
```
fn main() {
    println!("Hello!");
}
```
```

<!-- /LANG -->

<!-- LANG:rust -->

**Rust code blocks:**

```rust
// Always specify 'rust' as the language
fn example() -> Result<(), Error> {
    Ok(())
}
```

<!-- /LANG -->

<!-- LANG:python -->

**Python code blocks:**

```python
# Always specify 'python' as the language
def example():
    return "result"
```

<!-- /LANG -->

<!-- LANG:golang -->

**Go code blocks:**

```go
// Always specify 'go' as the language
func example() error {
    return nil
}
```

<!-- /LANG -->

<!-- LANG:typescript -->

**TypeScript code blocks:**

```typescript
// Always specify 'typescript' as the language
function example(): void {
    console.log("example");
}
```

<!-- /LANG -->

<!-- LANG:bash -->

**Bash code blocks:**

```bash
# Always specify 'bash' or 'sh' as the language
echo "example"
```

<!-- /LANG -->

**Common Language Identifiers:**

- `rust` - Rust code
- `python` - Python code
- `go` / `golang` - Go code
- `typescript` / `javascript` - TypeScript/JavaScript
- `bash` / `sh` - Shell commands
- `yaml` - YAML configuration
- `toml` - TOML configuration
- `json` - JSON data
- `text` - Plain text

---

## Links

### Inline Links

```text
See the [documentation](https://example.com/) for details.

Check out [our guide](./guides/setup.md) for setup.
```

### Reference Links

```text
See the [Rust Book][rust-book] for more.

[rust-book]: https://doc.rust-lang.org/book/
```

### Internal Links

```text
See the [Installation](#installation) section below.

Jump to [Chapter 2](./chapter_2.md).
```

---

## Emphasis

### Bold and Italic

```text
Use **bold** for strong emphasis.

Use *italic* for mild emphasis.

Use ***bold italic*** for very strong emphasis.
```

---

## Tables

### Basic Table

```text
| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Row 1    | Data     | More     |
| Row 2    | Data     | More     |
```

### Alignment

```text
| Left      | Center    | Right     |
|:----------|:---------:|----------:|
| Left      | Center    | Right     |
| Data      | Data      | Data      |
```

**Rules:**

- Use pipes (|) for column separators
- Use colons for alignment
- Keep tables simple

---

## No Emojis Rule

**CRITICAL: Never use emojis in markdown files**

```text
❌ WRONG:
# Setup Guide 🚀

✅ CORRECT:
# Setup Guide
```

Use task lists instead:

```text
- [x] Task completed
- [ ] Task pending
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

**3. Explanations** (`docs/explanation/`)
- Understanding-oriented
- Conceptual discussions
- Architecture decisions

**4. Reference** (`docs/reference/`)
- Information-oriented
- Technical specifications
- API documentation

---

## Validation Checklist

Before committing markdown files:

- [ ] Filename uses lowercase_with_underscores.md
- [ ] All code blocks have language identifiers
- [ ] Headers follow proper hierarchy
- [ ] No emojis used
- [ ] Links are valid
- [ ] Tables are properly formatted
- [ ] File ends with newline

---

## Best Practices

### Writing Style

- Use clear, concise language
- Write in present tense
- Use active voice
- Be consistent with terminology
- Define acronyms on first use

### Code Examples

- Always specify language
- Include complete examples
- Add comments for clarity
- Test code examples
