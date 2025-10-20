# AGENTS.md

This file provides guidance for AI agents working on this project. It outlines the project's structure, required toolchain, coding conventions, and development workflow. Adhering to these instructions ensures consistency and high-quality code.

## Project overview

**Project name:** pipeline-documentation

**Description:** `pipeline-documentation repository contains all the documentation for the pipeline`

**Architecture:** This project follows the Diataxis Framework architecture pattern.

## Documentation Style Guide

This guide defines the style conventions for documentation in the project. All documentation follows the **Diataxis Framework** for organization and structure.

### Documentation Organization (Diataxis Framework)

All documentation is organized into four categories:

1. **Tutorials** (`docs/tutorials/`) - Learning-oriented, hands-on lessons
2. **How-to Guides** (`docs/how_to/`) - Task-oriented, problem-solving guides
3. **Explanations** (`docs/explanations/`) - Understanding-oriented, conceptual discussion
4. **Reference** (`docs/reference/`) - Information-oriented, technical specifications

#### Structure Example

```text
docs/
├── README.md                    # Documentation overview
├── tutorials/
│   ├── README.md               # Tutorials index
│   └── getting_started.md      # Learning guide
├── how_to/
│   ├── README.md               # How-to index
│   ├── setup.md                # Installation steps
│   └── use_feature.md          # Task guide
├── explanations/
│   ├── README.md               # Explanations index
│   └── architecture.md         # Conceptual discussion
└── reference/
    ├── README.md               # Reference index
    ├── api.md                  # API specifications
    └── style_guide.md          # Standards and conventions
```

### File Naming Conventions

#### Markdown Files

- **README files**: `README.md` (uppercase)
- **All other documentation**: lowercase with underscores
  - ✅ `docs/tutorials/getting_started.md`
  - ✅ `docs/how_to/setup.md`
  - ✅ `docs/reference/api.md`
  - ❌ `docs/GETTING_STARTED.md`
  - ❌ `docs/How-To-Setup.md`

### Markdown Formatting

- **Documentation Framework:** Follow the Diataxis framework for structuring documentation:
  - **Tutorials** - Learning-oriented, teach through hands-on examples
  - **How-to Guides** - Task-oriented, solve specific problems
  - **Explanations** - Understanding-oriented, clarify concepts
  - **Reference** - Information-oriented, technical specifications
- **Category Placement:** Place each document in the appropriate category directory
- **Index Files:** Each category has a README.md index explaining its purpose
- **Doc comments:** Every public function should have doc comments explaining its purpose, arguments, and return values
- **Example code:** Include code examples within your doc comments
- **Internal documentation:** Use `#` comments for internal implementation details
- **MARKDOWN**
    - **Filenames:** ALL MARKDOWN file names should be lower case except for the README.md.
    - **DO NOT USE EMOJIs**
    - **Markdownlint:** Use the rules .markdownlint.json
- **Centralized documentation:** All documentation lives in `docs/` directory with Diataxis structure
- **Documentation updates:** Update documentation as part of the development process. When adding new features, create or modify documentation in the appropriate category
- **DO NOT USE EMOJIS in documentation or code comments**
- **Always specify language for fenced code blocks:**
  ```python
  # Python code here
  ```

### Markdownlint Configuration

The project uses `.markdownlint.json`:

```json
{
  "default": true,
  "line-length": {
    "code_blocks": false,
    "tables": false,
    "headings": false
  },
  "MD026": {
    "punctuation": ".,;:!。，；：！？"
  },
  "MD025": false,
  "MD024": false,
  "MD033": false,
  "MD036": false,
  "MD059": false
}
```

### Validation

#### Before Committing

Lint and format your markdown files:

```bash
markdownlint --fix --config .markdownlint.json docs/your_file.md
prettier --write --parser markdown --prose-wrap always docs/your_file.md
```
