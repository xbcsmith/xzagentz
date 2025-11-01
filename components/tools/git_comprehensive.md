---
component:
  name: git_comprehensive
  category: tools
  version: 1.0.0
  tier: comprehensive
  description: Comprehensive Git workflows including advanced operations and troubleshooting
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: branch_naming
      language_specific: false
      required: true
    - id: commit_messages
      language_specific: false
      required: true
    - id: advanced_workflow
      language_specific: true
      required: true
    - id: rebasing
      language_specific: false
      required: true
    - id: stashing
      language_specific: false
      required: true
    - id: troubleshooting
      language_specific: false
      required: true
---

# Git Comprehensive Guidelines

Complete Git workflows including advanced operations, troubleshooting, and best practices.

---

## Branch Naming Convention

**MANDATORY FORMAT:**

```text
pr-<feat>-<issue>
```

**Rules:**

- Use lowercase letters
- Separate with hyphens (not underscores)
- Always include JIRA issue number
- No uppercase in branch names

**Examples:**

```text
✅ CORRECT:
   pr-foo-1234
   pr-xzagentz-5678
   pr-proj-9012

❌ WRONG:
   PR-CPIPE-1234        (uppercase)
   feature/cpipe-1234   (wrong format)
   cpipe-1234           (missing pr- prefix)
   pr_cpipe_1234        (underscore instead of hyphen)
```

---

## Commit Message Format

**MANDATORY FORMAT:**

```text
<type>(<scope>): <description> (JIRA-ISSUE)

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
feat(auth): add JWT token refresh feature that allows users to... (CPIPE-1234)  # Too long
```

---

## Complete Git Workflow

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

<!-- /LANG -->

<!-- LANG:rust -->

### Before Creating PR (Rust)

```bash
# Run all quality checks
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

# Update branch with latest main
git fetch origin
git rebase origin/main

# Review changes
git diff origin/main

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

# Update branch with latest main
git fetch origin
git rebase origin/main

# Review changes
git diff origin/main

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

# Update branch with latest main
git fetch origin
git rebase origin/main

# Review changes
git diff origin/main

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

# Update branch with latest main
git fetch origin
git rebase origin/main

# Review changes
git diff origin/main

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

# Update branch with latest main
git fetch origin
git rebase origin/main

# Review changes
git diff origin/main

# Push final changes
git push origin pr-cpipe-1234
```

<!-- /LANG -->

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

# Search commit messages
git log --grep="feature"

# View commits in date range
git log --since="2 weeks ago" --until="yesterday"
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

# Show changed files only
git diff --name-only

# Show stats
git diff --stat
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

# Revert commit (creates new commit)
git revert abc123
```

---

## Stashing Changes

### Basic Stashing

```bash
# Stash current changes
git stash

# Stash with message
git stash save "work in progress"

# Stash including untracked files
git stash -u

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

# Show stash contents
git stash show -p stash@{0}
```

### Advanced Stashing

```bash
# Stash specific files
git stash push -m "message" file1.rs file2.rs

# Create branch from stash
git stash branch new-branch-name stash@{0}

# Stash with keep index (staged changes remain staged)
git stash --keep-index
```

---

## Rebasing

### Interactive Rebase

```bash
# Rebase last 3 commits
git rebase -i HEAD~3

# Rebase onto main
git rebase -i main

# Rebase onto specific commit
git rebase -i abc123
```

**Interactive rebase commands:**

- `pick` - Keep commit as is
- `reword` - Change commit message
- `edit` - Stop to amend commit
- `squash` - Combine with previous commit (keep message)
- `fixup` - Combine with previous commit (discard message)
- `drop` - Remove commit
- `exec` - Run shell command

### Resolving Conflicts

```bash
# During rebase, if conflicts occur:

# 1. Fix conflicts in files

# 2. Stage resolved files
git add resolved-file.rs

# 3. Continue rebase
git rebase --continue

# Or skip current commit
git rebase --skip

# Or abort rebase
git rebase --abort
```

### Rebase Best Practices

- Never rebase public branches
- Use `--force-with-lease` when force pushing
- Test after each rebase
- Keep commits atomic during rebase
- Write clear rebase commit messages

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

### Managing Tags

```bash
# List all tags
git tag

# List tags matching pattern
git tag -l "v1.*"

# Show tag details
git show v1.0.0

# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin --delete v1.0.0

# Checkout tag
git checkout v1.0.0
```

---

## Advanced Operations

### Cherry-picking

```bash
# Apply specific commit to current branch
git cherry-pick abc123

# Cherry-pick multiple commits
git cherry-pick abc123 def456

# Cherry-pick without committing
git cherry-pick --no-commit abc123

# Continue after resolving conflicts
git cherry-pick --continue

# Abort cherry-pick
git cherry-pick --abort
```

### Bisect (Find Bad Commit)

```bash
# Start bisect
git bisect start

# Mark current as bad
git bisect bad

# Mark known good commit
git bisect good abc123

# Test and mark (automated)
git bisect run ./test-script.sh

# End bisect
git bisect reset
```

### Submodules

```bash
# Add submodule
git submodule add https://github.com/user/repo.git path/to/submodule

# Clone with submodules
git clone --recurse-submodules https://github.com/user/repo.git

# Update submodules
git submodule update --init --recursive

# Update to latest
git submodule update --remote
```

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

**Lost commits:**

```bash
# Find lost commits
git reflog

# Recover commit
git checkout abc123
git checkout -b recovery-branch
```

**Large file accidentally committed:**

```bash
# Remove from last commit
git rm --cached large-file.bin
git commit --amend --no-edit

# Remove from history (use with caution)
git filter-branch --tree-filter 'rm -f large-file.bin' HEAD
```

---

## Git Configuration

### User Configuration

```bash
# Set user name
git config --global user.name "Your Name"

# Set user email
git config --global user.email "your.email@example.com"

# Set default editor
git config --global core.editor "vim"

# View configuration
git config --list

# View specific setting
git config user.name
```

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
    whatchanged = log --oneline --name-only
    aliases = config --get-regexp alias
```

### Repository Configuration

```bash
# Set default branch
git config --global init.defaultBranch main

# Auto-prune on fetch
git config --global fetch.prune true

# Rebase on pull
git config --global pull.rebase true

# Enable rerere (reuse recorded resolution)
git config --global rerere.enabled true
```

---

## .gitignore Best Practices

### Common Patterns

```gitignore
# Rust
target/
**/*.rs.bk
Cargo.lock

# Python
__pycache__/
*.py[cod]
*$py.class
.venv/
venv/

# Go
vendor/
*.exe
*.test

# Node/TypeScript
node_modules/
dist/
*.log

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Environment
.env
.env.local

# Build artifacts
build/
*.so
*.dylib
*.dll
```

### Managing Ignored Files

```bash
# Check why file is ignored
git check-ignore -v filename

# Show all ignored files
git status --ignored

# Force add ignored file
git add -f ignored-file.txt

# Ignore already tracked file
git rm --cached file.txt
echo "file.txt" >> .gitignore
```

---

## Performance Tips

### Optimize Repository

```bash
# Garbage collection
git gc --aggressive --prune=now

# Verify repository integrity
git fsck --full

# Show repository size
git count-objects -vH

# Compress objects
git repack -a -d --depth=250 --window=250
```

### Shallow Clones

```bash
# Clone with limited history
git clone --depth 1 https://github.com/user/repo.git

# Fetch more history
git fetch --depth=100

# Convert to full clone
git fetch --unshallow
```

---

## Best Practices Summary

### Commit Practices

- Commit logical units of work
- Each commit should be buildable
- Write descriptive commit messages
- Reference issue numbers
- Use imperative mood
- Keep commits focused (single purpose)

### Branch Management

- Keep branches short-lived (< 1 week)
- Regularly sync with main
- Delete merged branches promptly
- One feature per branch
- Use descriptive branch names

### Before Committing

- Run all tests
- Run linter/formatter
- Review changes carefully
- Ensure code compiles
- Update documentation

### Collaboration

- Pull before starting work
- Push regularly
- Communicate branch purpose
- Request reviews promptly
- Respond to feedback quickly
