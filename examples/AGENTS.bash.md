# AGENTS.md

This file provides critical guidance for AI agents working on this project.
**READ THE CRITICAL RULES FIRST** - they represent the most commonly violated
requirements.

## 🎯 CRITICAL RULES - READ FIRST

These are the **TOP 10 MOST VIOLATED RULES**. You MUST follow these before any
other instructions:

### 1. ⚠️ MARKDOWN FILE NAMING (MOST CRITICAL)

**RULE**: ALL markdown files MUST use `snake_case` lowercase (except
`README.md`)

- ✅ CORRECT: `getting_started.md`, `api_reference.md`, `setup_guide.md`
- ❌ WRONG: `GettingStarted.md`, `getting-started.md`, `SETUP.md`
- **EXCEPTION**: Only `README.md` uses uppercase

**YOU MUST**: Verify filename format BEFORE creating any `.md` file.

### 2. ⚠️ FENCED CODE BLOCKS (SECOND MOST CRITICAL)

**RULE**: EVERY fenced code block MUST specify a language identifier.

- ✅ CORRECT: ` ```python `, ` ```bash `, ` ```text `
- ❌ WRONG: ` ``` ` (bare backticks without language)
- **REQUIRED**: This enforces markdownlint rule MD040

**YOU MUST**: Add language identifier to ALL code blocks, use `text` when
unsure.

### 3. ⚠️ YAML FILE EXTENSIONS

**RULE**: ALL YAML files MUST use `.yaml` extension, NEVER `.yml`

- ✅ CORRECT: `config.yaml`, `settings.yaml`
- ❌ WRONG: `config.yml`, `settings.yml`

### 4. ⚠️ NO EMOJIS IN CODE OR DOCUMENTATION

**RULE**: NEVER use emojis in documentation files, code comments, or headings

- ✅ CORRECT: `## Getting Started`
- ❌ WRONG: `## 🚀 Getting Started`
- **NOTE**: Emojis are ONLY acceptable in AGENTS.md for emphasis

### 5. ⚠️ COMMIT MESSAGE FORMAT

**RULE**: ALL commits MUST follow Conventional Commits format

```text
<type>(<scope>): <description>
```

- ✅ CORRECT: `feat(template): add rust template`, `docs(readme): update usage instructions`
- ❌ WRONG: `Added template`, `Update docs`, `feat: add template` (missing scope)

**COMMIT TYPES**:

- `feat` - New feature or template
- `fix` - Bug fix
- `docs` - Documentation changes
- `refactor` - Code/template restructuring
- `chore` - Maintenance tasks

### 6. ⚠️ SHELL SCRIPT CONVENTIONS

**RULE**: Shell scripts MUST follow bash best practices

```bash
# ✅ CORRECT
#!/usr/bin/env bash
set -euo pipefail

# ❌ WRONG
#!/bin/bash
# (missing error handling)
```

- Always use `#!/usr/bin/env bash` shebang
- Use `set -euo pipefail` for error handling
- Quote variables: `"$variable"` not `$variable`
- Make scripts executable: `chmod +x script.sh`

### 7. ⚠️ TEMPLATE FILE NAMING

**RULE**: Template files MUST follow consistent naming

- ✅ CORRECT: `AGENTS_TEMPLATE.md`, `AGENTS.python.template`, `setup_agents.sh`
- ❌ WRONG: `template.md`, `agents-template.md`, `SetupAgents.sh`

### 8. ⚠️ MARKDOWN LINE LENGTH

**RULE**: Markdown lines SHOULD wrap at 80 characters for readability

- Use prose wrapping for documentation
- **EXCEPTION**: Code blocks, tables, and URLs may exceed limit

---

## 📋 PRE-FLIGHT CHECKLIST

**BEFORE STARTING ANY WORK**, verify:

### Environment Setup

- [ ] Current directory verified: `pwd` (should be in project root)
- [ ] Git status clean or understood: `git status`
- [ ] Branch is correct: `git branch --show-current`

### Before Creating Files

- [ ] Markdown filename is `snake_case` lowercase (except `README.md`)
- [ ] File will be created in correct directory (e.g., `docs/how_to/`)
- [ ] ALL code blocks will have language identifiers
- [ ] No emojis will be used in content

### Before Committing

- [ ] Shell scripts are executable: `chmod +x *.sh`
- [ ] Markdown linting passes: `markdownlint --config .markdownlint.json *.md`
- [ ] No bare code blocks: `grep -rn '^```$' *.md` (should return nothing)
- [ ] Commit message follows format: `type(scope): description`
- [ ] All changes are intentional: `git diff --staged`

---

## 🏗️ PROJECT OVERVIEW

### Basic Information

- **Project Name**: agents
- **Description**: Repository of AGENTS.md templates and tools to create new AGENTS.md files for new and existing projects
- **Primary Language**: Bash/Shell scripts
- **Type**: Template and tooling repository

### Key Features

- **Multiple Templates**: Pre-built AGENTS.md templates for different project types
- **Python Template**: Comprehensive template for Python projects with PEP 8, type hints, docstrings
- **Rust Template**: Template for Rust projects
- **Markdown Template**: Template focused on documentation projects
- **Git Template**: Template emphasizing git conventions
- **Setup Scripts**: Automation tools for creating AGENTS.md files in projects

### Core Scripts

- `setup_agents.sh` - Setup script to create AGENTS.md files in projects
- `test_template.sh` - Test script for validating templates

### Project Structure

```text
agents/
├── archive/                           # Archived versions and old files
├── docs/                              # Documentation directory
├── projects/                          # Project-specific documentation
│   └── diataxis_vscode_arch.md       # VSCode architecture notes
├── scripts/                           # Utility scripts (currently empty)
├── AGENTS.md                          # Main AGENTS.md file (this file)
├── AGENTS.git.md                      # Git-focused template
├── AGENTS.markdown.md                 # Markdown-focused template
├── AGENTS.md.python.template          # Python project template
├── AGENTS.rust.new.md                 # Rust project template
├── AGENTS_TEMPLATE.md                 # Base template
├── AGENTS_TEMPLATE_INDEX.md           # Template index
├── AGENTS_TEMPLATE_QUICKSTART.md      # Quick start template
├── AGENTS_TEMPLATE_README.md          # README-focused template
├── TEMPLATE_PACKAGE_SUMMARY.md        # Package summary template
├── setup_agents.sh                    # Setup automation script
├── test_template.sh                   # Template testing script
├── .gitignore                         # Git ignore rules
├── .markdownlint.json                 # Markdown linting rules
├── LICENSE                            # Project license
└── README.md                          # Project overview
```

---

## 📝 TEMPLATE DEVELOPMENT GUIDELINES

### Template Structure (MANDATORY)

**YOU MUST follow these conventions** when creating or modifying templates:

| Element        | Convention              | Example                  |
| -------------- | ----------------------- | ------------------------ |
| Template Files | `AGENTS*.md`            | `AGENTS_TEMPLATE.md`     |
| Script Files   | `snake_case.sh`         | `setup_agents.sh`        |
| Project Files  | `snake_case.md`         | `project_overview.md`    |
| Archive Files  | Original name preserved | `archive/old_version.md` |

### Shell Script Best Practices

**REQUIRED STANDARDS**:

1. **Shebang**: Always use `#!/usr/bin/env bash`
2. **Error Handling**: Use `set -euo pipefail` at top of script
3. **Variable Quoting**: Always quote variables: `"$var"` not `$var`
4. **Functions**: Use lowercase with underscores
5. **Comments**: Explain why, not what

```bash
#!/usr/bin/env bash
# ✅ CORRECT - Proper shell script structure
set -euo pipefail

# Function to validate template file
validate_template() {
    local template_file="$1"

    if [[ ! -f "$template_file" ]]; then
        echo "Error: Template file not found: $template_file" >&2
        return 1
    fi

    echo "Template validated: $template_file"
}

# ❌ WRONG - Missing error handling and quotes
validate_template() {
    if [ ! -f $1 ]; then
        echo "Error"
        return 1
    fi
}
```

### Template Content Guidelines

**YOU MUST ensure templates**:

1. Use placeholder text that's easy to find and replace
2. Include clear instructions in comments
3. Follow all markdown conventions (snake_case filenames, code block languages)
4. Provide examples for each major section
5. Include verification commands where applicable

```bash
# ✅ CORRECT - Template with clear placeholders
# Replace PROJECT_NAME with your actual project name
# Replace PROJECT_DESCRIPTION with your project description

## 🏗️ PROJECT OVERVIEW

### Basic Information

- **Project Name**: PROJECT_NAME
- **Description**: PROJECT_DESCRIPTION

# ❌ WRONG - No clear guidance
## PROJECT OVERVIEW
Add your project info here
```

### Testing Templates

**YOU MUST test templates** before committing:

````bash
# Test that template can be read
cat AGENTS_TEMPLATE.md > /dev/null

# Check for markdown issues
markdownlint --config .markdownlint.json AGENTS_TEMPLATE.md

# Verify no bare code blocks
grep -n '^```$' AGENTS_TEMPLATE.md

# Test shell scripts
bash -n setup_agents.sh  # Syntax check
shellcheck setup_agents.sh  # Lint (if available)
````

### Quality Checks

**YOU MUST verify** before committing:

````bash
# Check all markdown files
markdownlint --config .markdownlint.json *.md

# Verify shell scripts are executable
ls -l *.sh

# Check for common issues
grep -rn '^```$' . --include="*.md"  # Bare code blocks
find . -name "*.yml"  # Wrong YAML extension
````

---

## 📚 DOCUMENTATION STYLE GUIDE

### File Creation Decision Tree

**BEFORE creating any documentation file, answer these questions**:

1. **What type of content am I creating?**
   - Learning tutorial → `docs/tutorials/`
   - Task solution → `docs/how_to/`
   - Concept explanation → `docs/explanations/`
   - Technical specification → `docs/reference/`

2. **What should I name the file?**
   - Is it README? → `README.md` (uppercase)
   - Anything else → `lowercase_snake_case.md`

3. **What is the filename?**
   - ✅ All lowercase letters
   - ✅ Underscores between words
   - ❌ No hyphens, no spaces, no camelCase

**EXAMPLE WORKFLOW**:

```text
Task: Document how to set up the development environment

1. Type: Task-oriented guide → docs/how_to/
2. Not README → use snake_case
3. Filename: development_environment_setup.md

Final path: docs/how_to/development_environment_setup.md
```

### Template Categories

**Templates are organized by purpose**:

| Template Type     | Purpose                         | Example File                    |
| ----------------- | ------------------------------- | ------------------------------- |
| Base Templates    | Foundation for all projects     | `AGENTS_TEMPLATE.md`            |
| Language-Specific | Tailored for specific languages | `AGENTS.python.template`        |
| Focus Templates   | Emphasize specific aspects      | `AGENTS.git.md`                 |
| Quick Starts      | Minimal starting points         | `AGENTS_TEMPLATE_QUICKSTART.md` |

**IMPORTANT**: Templates should be self-contained and ready to copy.
</parameter>
</invoke>

### Markdown File Naming (CRITICAL)

**ABSOLUTE RULES - NO EXCEPTIONS**:

```text
✅ CORRECT FILENAMES:
- README.md                          (only README is uppercase)
- getting_started.md                 (snake_case)
- api_reference.md                   (snake_case)
- database_migration_guide.md        (snake_case)

❌ WRONG FILENAMES:
- GettingStarted.md                  (camelCase)
- getting-started.md                 (kebab-case)
- Getting_Started.md                 (mixed case)
- GETTING_STARTED.md                 (all uppercase)
- api-reference.md                   (kebab-case)
- ApiReference.md                    (PascalCase)
```

**VERIFICATION COMMAND**:

```bash
# Check if filename is valid (returns nothing if valid)
basename your_file.md | grep -v '^[a-z_]*\.md$\|^README\.md$'
```

### Fenced Code Blocks (CRITICAL - MD040)

**ABSOLUTE RULE**: EVERY code block MUST have a language identifier.

**COMMON LANGUAGE IDENTIFIERS**:

```text
python      - Python code
bash        - Shell commands (also: sh)
sql         - SQL queries
json        - JSON data
yaml        - YAML configuration
toml        - TOML configuration
text        - Plain text (use when no other language fits)
markdown    - Markdown content
diff        - Diff/patch output
typescript  - TypeScript code
javascript  - JavaScript code
```

**EXAMPLES**:

````markdown
✅ CORRECT:

```python
def hello():
    print("Hello")
```

```bash
pip install -e .
```

```text
Plain text output or content
```

❌ WRONG:

```
def hello():
    print("Hello")
```

```
pip install -e .
```
````

**WHEN IN DOUBT**: Use `text` as the language identifier.

**VERIFICATION COMMAND**:

````bash
# Find code blocks missing language identifiers
grep -n '^```$' your_file.md
# Should return nothing if all blocks have languages
````

### Emoji Usage (CRITICAL)

**RULE**: NO EMOJIS in documentation or code comments.

```markdown
✅ CORRECT:

## Getting Started

## Installation Steps

❌ WRONG:

## 🚀 Getting Started

## 📦 Installation Steps
```

**EXCEPTION**: Emojis are ONLY allowed in `AGENTS.md` for visual emphasis.

### YAML File Extensions (CRITICAL)

**RULE**: ALL YAML files MUST use `.yaml` extension.

```text
✅ CORRECT:
- config.yaml
- settings.yaml
- .markdownlint.yaml

❌ WRONG:
- config.yml
- settings.yml
- .markdownlint.yml
```

### Markdownlint Compliance

**YOU MUST ensure documentation passes markdownlint**:

```bash
# Lint a file
markdownlint --config .markdownlint.json docs/your_file.md

# Auto-fix issues
markdownlint --fix --config .markdownlint.json docs/your_file.md

# Format with prettier
prettier --write --parser markdown --prose-wrap always docs/your_file.md
```

**KEY RULES FROM `.markdownlint.json`**:

- MD040 (fenced-code-language): **ENABLED** - All code blocks need language
- Line length: Relaxed for code blocks, tables, headings
- MD025 (single h1): Disabled
- MD033 (inline HTML): Disabled
- MD036 (emphasis style): Disabled

### Documentation Pre-Commit Checklist

**BEFORE committing ANY documentation file**:

- [ ] Filename is `snake_case` lowercase (or `README.md`)
- [ ] File is in correct Diataxis category directory
- [ ] ALL fenced code blocks have language identifiers
- [ ] NO emojis in content or headings
- [ ] YAML files use `.yaml` extension
- [ ] Links use relative paths (not absolute)
- [ ] Run: `markdownlint --fix --config .markdownlint.json <file>`
- [ ] Run: `grep -n '^```$' <file>` (should return nothing)

---

## 🔀 GIT CONVENTIONS

### Branch Naming (MANDATORY)

**RULE**: Use descriptive branch names with prefix

```bash
✅ CORRECT:
git checkout -b feature/rust-template
git checkout -b fix/markdown-linting
git checkout -b docs/update-readme

❌ WRONG:
git checkout -b FEATURE
git checkout -b my-branch
git checkout -b temp
```

**BRANCH PREFIXES**:

- `feature/` - New features or templates
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code restructuring
- `chore/` - Maintenance tasks

### Commit Messages (MANDATORY)

**RULE**: Follow Conventional Commits format

**FORMAT**:

```text
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**COMMIT TYPES**:

| Type       | Usage                          |
| ---------- | ------------------------------ |
| `feat`     | New feature or template        |
| `fix`      | Bug fix                        |
| `docs`     | Documentation changes only     |
| `style`    | Formatting (no logic change)   |
| `refactor` | Code/template restructuring    |
| `test`     | Test additions or corrections  |
| `chore`    | Maintenance or tooling changes |

**EXAMPLES**:

```text
✅ CORRECT:
feat(template): add rust project template
fix(script): handle missing file errors in setup script
docs(readme): update usage instructions
refactor(template): simplify python template structure
chore(lint): update markdownlint configuration

❌ WRONG:
Added rust template
feat: add template (missing scope)
FEAT(template): Add template (wrong case)
added template (missing type and format)
```

**COMMIT MESSAGE RULES**:

1. **Type**: REQUIRED - Must be lowercase
2. **Scope**: REQUIRED - Use parentheses, be specific
3. **Description**: REQUIRED - Lowercase, imperative mood, no period
4. **Line Length**: Maximum 72 characters for first line
5. **Imperative Mood**: Use "add" not "added", "fix" not "fixed"

**VERIFICATION**:

```bash
# Valid commit message format
git log --oneline -1 | grep -E '^[a-f0-9]+ (feat|fix|docs|style|refactor|test|chore)\([a-z_]+\): [a-z]'
```

---

## ⚠️ COMMON MISTAKES & SOLUTIONS

This section documents **the most frequent errors** agents make. Review this
BEFORE starting work.

### Mistake 1: Wrong Markdown Filename Format

**ERROR**: Creating files with wrong naming convention.

```text
❌ WRONG:
docs/how_to/GettingStarted.md
docs/tutorials/User-Guide.md
docs/reference/API-Reference.md
```

**SOLUTION**: ALWAYS use `snake_case` lowercase:

```text
✅ CORRECT:
docs/how_to/getting_started.md
docs/tutorials/user_guide.md
docs/reference/api_reference.md
```

**PREVENTION**: Before creating file, verify:

```bash
echo "getting_started.md" | grep -E '^[a-z_]+\.md$'  # Should match
echo "GettingStarted.md" | grep -E '^[a-z_]+\.md$'   # Should not match
```

### Mistake 2: Missing Language in Code Blocks

**ERROR**: Using bare triple backticks without language identifier.

````markdown
❌ WRONG:

```
pip install -e .
```
````

**SOLUTION**: ALWAYS specify language:

````markdown
✅ CORRECT:

```bash
pip install -e .
```
````

**PREVENTION**: After writing docs, run:

````bash
grep -n '^```$' your_file.md  # Should return nothing
````

### Mistake 3: Using .yml Extension

**ERROR**: Creating YAML files with `.yml` extension.

```text
❌ WRONG:
config.yml
settings.yml
.github/workflows/ci.yml
```

**SOLUTION**: ALWAYS use `.yaml`:

```text
✅ CORRECT:
config.yaml
settings.yaml
.github/workflows/ci.yaml
```

### Mistake 4: Shell Scripts Not Executable

**ERROR**: Shell scripts without execute permissions.

```bash
❌ WRONG:
./setup_agents.sh
# bash: ./setup_agents.sh: Permission denied
```

**SOLUTION**: Make scripts executable:

```bash
✅ CORRECT:
chmod +x setup_agents.sh
./setup_agents.sh
```

**PREVENTION**: Always set permissions when creating scripts:

```bash
# Create and make executable in one step
touch new_script.sh
chmod +x new_script.sh
```

### Mistake 5: Missing Error Handling in Scripts

**ERROR**: Shell scripts without proper error handling.

```bash
❌ WRONG:
#!/bin/bash
cp template.md output.md
echo "Done"
```

**SOLUTION**: Use proper error handling:

```bash
✅ CORRECT:
#!/usr/bin/env bash
set -euo pipefail

if ! cp template.md output.md; then
    echo "Error: Failed to copy template" >&2
    exit 1
fi
echo "Done"
```

### Mistake 6: Unquoted Variables in Shell Scripts

**ERROR**: Shell variables without quotes.

```bash
❌ WRONG:
file=$1
cat $file  # Breaks with spaces in filename
```

**SOLUTION**: Always quote variables:

```bash
✅ CORRECT:
file="$1"
cat "$file"  # Handles spaces correctly
```

### Mistake 7: Template Placeholders Not Clear

**ERROR**: Templates with unclear or missing placeholders.

```markdown
❌ WRONG:

## Project Overview

Add your project information here.
```

**SOLUTION**: Use clear, findable placeholders:

```markdown
✅ CORRECT:

## Project Overview

- **Project Name**: PROJECT_NAME
- **Description**: PROJECT_DESCRIPTION
- **Version**: PROJECT_VERSION

Replace PROJECT_NAME, PROJECT_DESCRIPTION, and PROJECT_VERSION with actual values.
```

### Mistake 8: Invalid Commit Messages

**ERROR**: Commit messages not following Conventional Commits.

```text
❌ WRONG:
"Added template"
"fix bug"
"feat: add template"  (missing scope)
"FEAT(template): Add template"  (wrong case)
```

**SOLUTION**: Follow exact format:

```text
✅ CORRECT:
feat(template): add rust project template
fix(script): handle missing file errors
docs(readme): update usage instructions
```

### Mistake 9: Files in Wrong Location

**ERROR**: Creating template files in wrong directory.

```text
❌ WRONG:
scripts/AGENTS.md         (templates should be in root)
docs/template.md          (not a docs file)
```

**SOLUTION**: Place files in correct location:

```text
✅ CORRECT:
AGENTS_TEMPLATE.md        (templates in root)
scripts/setup.sh          (scripts in scripts/)
projects/notes.md         (project files in projects/)
```

### Mistake 10: Not Validating Before Commit

**ERROR**: Committing without validation checks.

**SOLUTION**: ALWAYS run before commit:

````bash
✅ CORRECT WORKFLOW:
# Check markdown
markdownlint --config .markdownlint.json *.md

# Check for bare code blocks
grep -rn '^```$' *.md

# Check shell scripts
bash -n *.sh
chmod +x *.sh

# Verify no .yml files
find . -name "*.yml"

# Commit
git add .
git commit -m "feat(template): add new template"
````

---

## 🔍 TROUBLESHOOTING

### Issue: Markdownlint Errors

**Symptoms**:

```bash
markdownlint *.md
# Returns MD040 errors (missing code block language)
```

**Diagnosis**:

````bash
# Find bare code blocks
grep -n '^```$' your_file.md
````

**Solution**:

````bash
# Add language identifier to all code blocks
# Change ``` to ```bash, ```python, ```text, etc.

# Auto-fix what can be fixed
markdownlint --fix --config .markdownlint.json *.md
````

### Issue: Shell Script Syntax Errors

**Symptoms**:

```bash
./setup_agents.sh
# bash: syntax error near unexpected token
```

**Diagnosis**:

```bash
bash -n setup_agents.sh  # Check syntax without running
```

**Solution**:

```bash
# Common issues:
# - Missing quotes around variables
# - Unmatched brackets or parentheses
# - Missing 'then' after 'if'
# - Missing 'do' after 'for' or 'while'

# Fix syntax and test again
bash -n setup_agents.sh
```

### Issue: Template Not Working

**Symptoms**:

```text
Copied template but instructions unclear
Placeholders not obvious
```

**Solution**:

```bash
# Review template for:
# - Clear placeholder names (PROJECT_NAME, etc.)
# - Explicit instructions for replacement
# - Examples provided for each section
# - Comments explaining what to modify

# Test template by using it yourself
cp AGENTS_TEMPLATE.md test_project/AGENTS.md
# Follow instructions to verify they work
```

### Issue: Files Not Being Tracked

**Symptoms**:

```bash
git status
# Shows files you expect to see aren't listed
```

**Diagnosis**:

```bash
cat .gitignore  # Check if files are ignored
git check-ignore -v filename.md  # See why file is ignored
```

**Solution**:

```bash
# If incorrectly ignored, either:
# - Update .gitignore to not ignore the file
# - Force add with: git add -f filename.md
```

---

## 📖 QUICK REFERENCE COMMANDS

### Daily Workflow

````bash
# Start work
cd /path/to/agents
git checkout -b feature/new-template

# During development
markdownlint --config .markdownlint.json *.md  # Lint markdown
bash -n *.sh                                   # Check shell syntax
chmod +x *.sh                                  # Make scripts executable

# Before commit
markdownlint --fix --config .markdownlint.json *.md  # Fix markdown issues
grep -rn '^```$' *.md                                # Check for bare code blocks
find . -name "*.yml"                                 # Check for wrong YAML extension

# Commit
git add .
git commit -m "feat(template): add new project template"
git push origin feature/new-template
````

### File Validation

````bash
# Check markdown filename format
basename file.md | grep -E '^[a-z_]+\.md$|^README\.md$'

# Check for missing code block languages
grep -n '^```$' file.md

# Check for .yml files (should use .yaml)
find . -name "*.yml"

# Lint markdown
markdownlint --fix --config .markdownlint.json docs/file.md
````

### Shell Script Testing

```bash
# Syntax check
bash -n setup_agents.sh

# Make executable
chmod +x setup_agents.sh

# Run with error checking
bash -x setup_agents.sh  # Debug mode

# Test in temporary directory
mkdir -p /tmp/test_agents
cp setup_agents.sh /tmp/test_agents/
cd /tmp/test_agents && bash setup_agents.sh
```

---

## 📝 FINAL CHECKLIST

Before submitting ANY work, verify ALL items:

### Template Changes

- [ ] Template filename follows conventions
- [ ] All code blocks have language identifiers
- [ ] No emojis in template content
- [ ] Placeholders are clearly marked
- [ ] Instructions are clear and complete
- [ ] Template tested by creating from it
- [ ] Markdown linting passes

### Documentation Changes

- [ ] Filename is `snake_case` lowercase (or `README.md`)
- [ ] File in correct Diataxis category
- [ ] ALL code blocks have language identifiers
- [ ] NO emojis anywhere
- [ ] Markdownlint passes
- [ ] Links are relative paths

### Git Changes

- [ ] Branch name: `prefix/descriptive-name`
- [ ] Commit message format: `type(scope): description`
- [ ] Imperative mood in commit description
- [ ] Type and scope are lowercase

### Before Push

- [ ] Shell scripts executable: `ls -l *.sh`
- [ ] Markdown linted: `markdownlint --config .markdownlint.json *.md`
- [ ] No bare code blocks: `grep -rn '^```$' *.md`
- [ ] No .yml files: `find . -name "*.yml"`
- [ ] Changes reviewed: `git diff --staged`

---

## 🎓 LEARNING RESOURCES

### Essential Reading

1. **Conventional Commits**: Commit message format -
   https://www.conventionalcommits.org/
2. **Bash Best Practices**: Shell scripting guide -
   https://google.github.io/styleguide/shellguide.html
3. **Markdown Guide**: Markdown syntax reference - https://www.markdownguide.org/
4. **ShellCheck**: Shell script linting - https://www.shellcheck.net/

### Project-Specific

- Review available templates in root directory for different project types
- Review `.markdownlint.json` for markdown rules
- Review `setup_agents.sh` for automation capabilities
- Study existing templates to understand structure and patterns

---

**REMEMBER**: When in doubt, follow the CRITICAL RULES at the top of this
document. They represent the most commonly violated requirements and should
ALWAYS be your first priority.
