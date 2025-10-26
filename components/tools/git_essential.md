---
component:
  name: git_essential
  category: tools
  version: 1.0.0
  tier: essential
  description: Essential Git workflows and conventions for daily development
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: branch_naming
      language_specific: false
      required: true
    - id: commit_messages
      language_specific: false
      required: true
    - id: basic_workflow
      language_specific: true
      required: true
    - id: quality_checks
      language_specific: true
      required: true
---

# Git Essential Guidelines

Core Git workflows and conventions for daily development tasks.

---

## Branch Naming Convention

**MANDATORY FORMAT:**

```text
pr-{jira-issue}
```

**Rules:**

- Use lowercase letters
- Separate with hyphens (not underscores)
- Always include JIRA issue number
- No uppercase in branch names

**Examples:**

```text
✅ CORRECT:
   pr-cpipe-1234
   pr-xzagentz-5678

❌ WRONG:
   PR-CPIPE-1234        (uppercase)
   feature/cpipe-1234   (wrong format)
   pr_cpipe_1234        (underscore)
```

---

## Commit Message Format

**MANDATORY FORMAT:**

```text
<type>(<scope>): <description> (JIRA-ISSUE)
```

### Commit Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation only
- `refactor` - Code restructuring
- `test` - Adding/fixing tests
- `chore` - Build/tooling changes

### Rules

1. Type and description MUST be lowercase
2. Scope is optional but recommended
3. Use imperative mood ("add" not "added")
4. JIRA issue MUST be uppercase
5. First line MUST be ≤72 characters

### Examples

```text
✅ CORRECT:
feat(auth): add JWT token refresh endpoint (CPIPE-1234)
fix(api): handle edge case in validation (CPIPE-5678)
docs(readme): update installation steps (XZAGENTZ-9012)

❌ WRONG:
Added JWT token (CPIPE-1234)              (no type, wrong mood)
feat(auth): Add JWT Token (cpipe-1234)    (wrong case)
add jwt refresh (CPIPE-1234)              (no type)
```

---

## Basic Git Workflow

<!-- LANG:* -->

### Starting New Work

```bash
# Update main branch
git checkout main
git pull origin main

# Create feature branch
git checkout -b pr-cpipe-1234

# Make changes and commit
git add -A
git commit -m "feat(module): add feature (CPIPE-1234)"

# Push to remote
git push origin pr-cpipe-1234
```

### Daily Development

```bash
# Commit changes frequently
git add -A
git commit -m "fix(module): resolve issue (CPIPE-1234)"

# Keep branch updated
git fetch origin
git rebase origin/main

# Push changes
git push origin pr-cpipe-1234 --force-with-lease
```

<!-- /LANG -->

<!-- LANG:rust -->

### Before Creating PR (Rust)

```bash
# Run all quality checks
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

# Update branch
git fetch origin
git rebase origin/main

# Push final changes
git push origin pr-cpipe-1234
```

<!-- /LANG -->

<!-- LANG:python -->

### Before Creating PR (Python)

```bash
# Run all quality checks
black .
ruff check .
mypy .
pytest

# Update branch
git fetch origin
git rebase origin/main

# Push final changes
git push origin pr-cpipe-1234
```

<!-- /LANG -->

<!-- LANG:golang -->

### Before Creating PR (Go)

```bash
# Run all quality checks
go fmt ./...
go vet ./...
golangci-lint run
go test ./...

# Update branch
git fetch origin
git rebase origin/main

# Push final changes
git push origin pr-cpipe-1234
```

<!-- /LANG -->

<!-- LANG:typescript -->

### Before Creating PR (TypeScript)

```bash
# Run all quality checks
npm run format
npm run lint
npm run type-check
npm test

# Update branch
git fetch origin
git rebase origin/main

# Push final changes
git push origin pr-cpipe-1234
```

<!-- /LANG -->

<!-- LANG:bash -->

### Before Creating PR (Bash)

```bash
# Run all quality checks
shellcheck **/*.sh
shfmt -w .

# Update branch
git fetch origin
git rebase origin/main

# Push final changes
git push origin pr-cpipe-1234
```

<!-- /LANG -->

---

## Essential Commands

### Viewing Changes

```bash
# View unstaged changes
git diff

# View staged changes
git diff --staged

# View commit history
git log --oneline

# View last commit
git log -1
```

### Staging

```bash
# Stage all changes
git add -A

# Stage specific file
git add src/main.rs

# Unstage file
git restore --staged src/main.rs
```

### Undoing Changes

```bash
# Discard changes in file
git restore src/main.rs

# Discard all changes
git restore .

# Amend last commit
git commit --amend --no-edit
```

---

## Quality Checks Before Commit

<!-- LANG:rust -->

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

<!-- /LANG -->

<!-- LANG:python -->

```bash
black .
ruff check .
pytest
```

<!-- /LANG -->

<!-- LANG:golang -->

```bash
go fmt ./...
go vet ./...
go test ./...
```

<!-- /LANG -->

<!-- LANG:typescript -->

```bash
npm run format
npm run lint
npm test
```

<!-- /LANG -->

<!-- LANG:bash -->

```bash
shellcheck **/*.sh
shfmt -w .
```

<!-- /LANG -->

---

## Best Practices

- Commit logical units of work
- Write clear commit messages
- Keep branches short-lived
- Run tests before committing
- Rebase to keep history clean
- Use `--force-with-lease` not `--force`
