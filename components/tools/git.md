# Git Guidelines and Workflows

This section provides Git-specific guidelines, workflows, and best practices for version control in this project.

---

## Branch Naming Convention

**MANDATORY FORMAT:**
```text
pr-<feat>-<issue>
```

**Rules:**
- Use lowercase letters
- Separate with hyphens (not underscores)
- No uppercase in branch names

**Examples:**
```text
✅ CORRECT:
   pr-foo-1234
   pr-xzagentz-5678
   pr-proj-9012

❌ WRONG:
   PR-CPIPE-1234        # uppercase
   feature/cpipe-1234   # wrong format
   cpipe-1234           # missing pr- prefix
   pr_cpipe_1234        # underscore instead of hyphen
```

---

## Commit Message Format

**MANDATORY FORMAT:**
```text
<type>(<scope>): <description>

[optional body explaining why change was made]

[optional footer with breaking changes]
```

### Commit Types

**MUST use one of these types:**

- `feat` - New feature (triggers minor version bump)
- `fix` - Bug fix (triggers patch version bump)
- `docs` - Documentation only (no code changes)
- `style` - Code formatting (no logic changes)
- `refactor` - Code restructuring (no behavior changes)
- `perf` - Performance improvements
- `test` - Adding/fixing tests
- `chore` - Build process, dependencies, tools

### Commit Rules

1. Type MUST be lowercase
2. Scope is optional but recommended
3. Description MUST be lowercase
4. Description MUST use imperative mood ("add" not "added")
5. JIRA issue MUST be uppercase in parentheses
6. First line MUST be ≤72 characters (including JIRA issue)
7. Wrap body at 72 characters

### Commit Examples

**Good Examples:**
```text
feat(auth): add JWT token refresh endpoint (CPIPE-1234)

fix(api): handle edge case in event validation (CPIPE-5678)

docs(readme): update installation instructions (XZAGENTZ-9012)

refactor(metrics): simplify prometheus integration (CPIPE-3456)

test(parser): add edge case tests for markdown parsing (PROJ-7890)
```

**With Body:**
```text
feat(tracing): add distributed tracing support (XZAGENTZ-4567)

Implements OpenTelemetry integration with Jaeger exporter.
Adds automatic span creation for all HTTP requests.
Includes configuration for sampling rates and endpoints.

BREAKING CHANGE: Config file now requires tracing section
```

**Bad Examples:**
```text
Added JWT token refresh (CPIPE-1234)              # Wrong mood, no type
feat(auth): Add JWT Token (cpipe-1234)            # Wrong case
feat: add JWT (CPIPE-1234)                        # Missing scope
add jwt refresh (CPIPE-1234)                      # No type
feat(auth): add JWT token refresh feature that allows users to refresh their authentication tokens when they expire (CPIPE-1234)  # Too long
```

---

## Git Workflow

### Starting New Work

```bash
# 1. Ensure main is up to date
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b pr-cpipe-1234

# 3. Make changes and commit
git add -A
git commit -m "feat(module): add feature (CPIPE-1234)"

# 4. Push to remote
git push origin pr-cpipe-1234
```

### During Development

```bash
# Commit changes frequently
git add -A
git commit -m "fix(module): resolve issue (CPIPE-1234)"

# Keep branch up to date with main
git checkout main
git pull origin main
git checkout pr-cpipe-1234
git rebase main

# Force push after rebase (if needed)
git push origin pr-cpipe-1234 --force-with-lease
```

### Before Creating PR

```bash
# 1. Ensure all quality checks pass
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

# 2. Update branch with latest main
git fetch origin
git rebase origin/main

# 3. Review changes
git diff origin/main

# 4. Push final changes
git push origin pr-cpipe-1234
```

---

## Staging Changes

### Selective Staging

```bash
# Stage all changes
git add -A

# Stage specific file
git add src/main.rs

# Stage specific directory
git add src/api/

# Stage parts of file (interactive)
git add -p src/main.rs

# Review staged changes
git diff --staged
```

### Unstaging Changes

```bash
# Unstage specific file
git restore --staged src/main.rs

# Unstage all changes
git restore --staged .
```

---

## Viewing History and Changes

### Log Commands

```bash
# View commit history
git log

# View compact log
git log --oneline

# View log with graph
git log --oneline --graph --all

# View commits by author
git log --author="name"

# View commits for file
git log -- src/main.rs

# View last N commits
git log -n 5
```

### Diff Commands

```bash
# View unstaged changes
git diff

# View staged changes
git diff --staged

# View changes in specific file
git diff src/main.rs

# Compare branches
git diff main..pr-cpipe-1234

# Compare commits
git diff abc123..def456
```

---

## Undoing Changes

### Undo Uncommitted Changes

```bash
# Discard changes in specific file
git restore src/main.rs

# Discard all changes
git restore .

# Remove untracked files
git clean -fd

# Remove untracked files (dry run)
git clean -fdn
```

### Undo Commits

```bash
# Undo last commit, keep changes
git reset --soft HEAD~1

# Undo last commit, discard changes
git reset --hard HEAD~1

# Amend last commit
git commit --amend

# Amend without changing message
git commit --amend --no-edit
```

---

## Stashing Changes

### Basic Stashing

```bash
# Stash current changes
git stash

# Stash with message
git stash save "work in progress"

# List stashes
git stash list

# Apply latest stash
git stash apply

# Apply and remove latest stash
git stash pop

# Apply specific stash
git stash apply stash@{1}

# Drop specific stash
git stash drop stash@{1}

# Clear all stashes
git stash clear
```

---

## Rebasing

### Interactive Rebase

```bash
# Rebase last 3 commits
git rebase -i HEAD~3

# Rebase onto main
git rebase -i main
```

**Interactive rebase commands:**
- `pick` - Keep commit as is
- `reword` - Change commit message
- `edit` - Stop to amend commit
- `squash` - Combine with previous commit
- `fixup` - Like squash but discard message
- `drop` - Remove commit

### Resolving Conflicts

```bash
# During rebase, if conflicts occur:

# 1. Fix conflicts in files
# 2. Stage resolved files
git add resolved-file.rs

# 3. Continue rebase
git rebase --continue

# Or abort rebase
git rebase --abort
```

---

## Tags

### Creating Tags

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Create lightweight tag
git tag v1.0.0

# Tag specific commit
git tag -a v1.0.0 abc123 -m "Release 1.0.0"

# Push tag
git push origin v1.0.0

# Push all tags
git push origin --tags
```

### Viewing Tags

```bash
# List all tags
git tag

# List tags matching pattern
git tag -l "v1.*"

# Show tag details
git show v1.0.0
```

---

## .gitignore

### Common Patterns

```gitignore
# Rust
target/
**/*.rs.bk
Cargo.lock

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Project specific
*.log
.env
.env.local

# Build artifacts
dist/
build/
*.so
*.dylib
*.dll

# Documentation build
docs/_build/
site/
```

### Check Ignore Rules

```bash
# Check why file is ignored
git check-ignore -v filename

# Show all ignored files
git status --ignored
```

---

## Best Practices

### Commit Frequency

- Commit logical units of work
- Each commit should be buildable
- Commits should have single purpose
- Commit before switching context

### Commit Messages

- First line: concise summary (≤72 chars)
- Blank line after first line
- Body: explain what and why (not how)
- Reference issue numbers
- Use imperative mood

### Branch Management

- Keep branches short-lived
- Regularly sync with main
- Delete merged branches
- One feature per branch

### Before Committing

- Run tests: `cargo test --all-features`
- Run linter: `cargo clippy -- -D warnings`
- Format code: `cargo fmt --all`
- Review changes: `git diff --staged`

---

## Troubleshooting

### Common Issues

**Detached HEAD state:**
```bash
# Create branch from current state
git checkout -b recovery-branch
```

**Accidentally committed to main:**
```bash
# Move commits to new branch
git branch pr-issue-1234
git reset --hard origin/main
git checkout pr-issue-1234
```

**Merge conflict during rebase:**
```bash
# Fix conflicts, then:
git add resolved-file
git rebase --continue

# Or abort:
git rebase --abort
```

**Pushed wrong commit:**
```bash
# Undo commit locally
git reset --hard HEAD~1

# Force push (use with caution!)
git push origin pr-issue-1234 --force-with-lease
```

---

## Git Aliases

### Useful Aliases

Add to `~/.gitconfig`:

```ini
[alias]
    st = status
    co = checkout
    br = branch
    ci = commit
    unstage = restore --staged
    last = log -1 HEAD
    lg = log --oneline --graph --all --decorate
    amend = commit --amend --no-edit
    undo = reset --soft HEAD~1
```

---

## Additional Resources

- **Git Documentation**: https://git-scm.com/doc
- **Pro Git Book**: https://git-scm.com/book
- **Git Cheat Sheet**: https://training.github.com/downloads/github-git-cheat-sheet/
- **Conventional Commits**: https://www.conventionalcommits.org/
