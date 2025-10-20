# AGENTS.md

This file provides guidance for AI agents working on this project. It outlines
the project's structure, required toolchain, coding conventions, and development
workflow. Adhering to these instructions ensures consistency and high-quality
code.

## Project overview

**Project name:** project-name

**Description:**
`project-name does X, Y, and Z to help users achieve A, B, and C.`

**Architecture:** This project follows the architecture pattern:
`src/project-name/` contains the main module with submodules for different
functionalities.

**Python Module:** `src/project-name` is main module

## Git Conventions

### Branch Naming

- **PR branches**: Use lowercase format `pr-<username>-<issue>`
  - Example: `pr-xbcsmith-1234`
  - Example: `pr-xbcsmith-5678`
- **Keep branch names lowercase** - No uppercase letters or camelCase
- **Include GitHub issue** - Always reference the issue being worked on when available

### Commit Messages

All commits **MUST** follow the
[Conventional Commits](https://www.conventionalcommits.org/) specification.

#### Format

```text
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

#### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space,
  formatting)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools

#### Commit Examples

```text
feat(auth): add user authentication module

fix(main): updated foo to handle edge cases

docs(readme): update installation instructions

refactor(api): simplify error handling logic

test(utils): add unit tests for string parser

#### Rules

- **Type is required** - Must be one of the standard types
- **Scope is optional** - Use parentheses if included
- **Description is required** - Short summary in lowercase
- **Keep the first line under 72 characters**
- **Use imperative mood** - "add" not "added", "fix" not "fixed"
