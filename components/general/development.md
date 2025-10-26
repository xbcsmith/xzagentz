---
component:
  name: development_workflow
  category: general
  version: 2.0.0
  description: Development workflow and environment setup standards
  languages:
    - rust
    - python
    - golang
    - typescript
    - bash
  sections:
    - id: principles
      language_specific: false
      required: true
    - id: environment_setup
      language_specific: true
      required: true
    - id: development_workflow
      language_specific: true
      required: true
    - id: dependency_management
      language_specific: true
      required: true
    - id: quality_checks
      language_specific: true
      required: true
    - id: debugging
      language_specific: true
      required: true
---

# Development Workflow

## Core Principles

Follow these universal development practices:

1. **Work in Small Iterations**: Commit working code frequently
2. **Test Early and Often**: Write tests alongside code
3. **Maintain Clean History**: Use meaningful commit messages
4. **Keep Builds Green**: Never commit broken code
5. **Review Before Merge**: All code undergoes review
6. **Automate Repetition**: Use tools to enforce standards
7. **Document as You Go**: Update docs with code changes

---

## Environment Setup

Set up your development environment with required and recommended tools.

<!-- LANG:rust -->

### Rust Environment

**Required Tools:**

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add components
rustup component add clippy rustfmt
```

**Recommended Tools:**

```bash
# Development helpers
cargo install cargo-watch      # Auto-rebuild on changes
cargo install cargo-edit       # Manage dependencies
cargo install cargo-audit      # Security scanning
cargo install cargo-outdated   # Dependency updates
```

**IDE Setup:**

- VS Code: Install rust-analyzer extension
- RustRover: Native Rust support
- Vim/Neovim: Install rust.vim and coc-rust-analyzer

<!-- /LANG -->

<!-- LANG:python -->

### Python Environment

**Required Tools:**

```bash
# Install Python (use pyenv for version management)
pyenv install 3.11.0
pyenv global 3.11.0

# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
.venv\Scripts\activate     # Windows
```

**Recommended Tools:**

```bash
# Development helpers
pip install black          # Code formatting
pip install pylint         # Linting
pip install mypy           # Type checking
pip install pytest         # Testing
pip install pytest-cov     # Coverage
pip install ipdb           # Debugging
```

**IDE Setup:**

- VS Code: Install Python extension
- PyCharm: Native Python support
- Vim/Neovim: Install python-mode and coc-pyright

<!-- /LANG -->

<!-- LANG:golang -->

### Go Environment

**Required Tools:**

```bash
# Install Go
wget https://go.dev/dl/go1.21.0.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.21.0.linux-amd64.tar.gz

# Add to PATH
export PATH=$PATH:/usr/local/go/bin
export GOPATH=$HOME/go
export PATH=$PATH:$GOPATH/bin
```

**Recommended Tools:**

```bash
# Development helpers
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
go install golang.org/x/tools/cmd/goimports@latest
go install github.com/go-delve/delve/cmd/dlv@latest
go install gotest.tools/gotestsum@latest
```

**IDE Setup:**

- VS Code: Install Go extension
- GoLand: Native Go support
- Vim/Neovim: Install vim-go

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Environment

**Required Tools:**

```bash
# Install Node.js (use nvm for version management)
nvm install 18
nvm use 18

# Install package manager
npm install -g pnpm  # or yarn
```

**Recommended Tools:**

```bash
# Development helpers
pnpm add -D typescript         # TypeScript compiler
pnpm add -D @types/node        # Node.js types
pnpm add -D eslint             # Linting
pnpm add -D prettier           # Formatting
pnpm add -D vitest             # Testing
pnpm add -D tsx                # TypeScript runner
```

**IDE Setup:**

- VS Code: Install ESLint and Prettier extensions
- WebStorm: Native TypeScript support
- Vim/Neovim: Install coc-tsserver

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Environment

**Required Tools:**

```bash
# Bash is typically pre-installed
# Verify version (4.0+ recommended)
bash --version

# Install ShellCheck
apt-get install shellcheck  # Debian/Ubuntu
brew install shellcheck     # macOS
```

**Recommended Tools:**

```bash
# Development helpers
apt-get install shunit2    # Unit testing
brew install bats-core     # Testing framework
pip install bashate        # Style checking

# Install shfmt for formatting
go install mvdan.cc/sh/v3/cmd/shfmt@latest
```

**IDE Setup:**

- VS Code: Install ShellCheck extension
- Any editor with shellcheck integration
- Vim/Neovim: Install vim-shell-lint

<!-- /LANG -->

---

## Development Workflow

Follow the standard development workflow for making changes.

<!-- LANG:rust -->

### Rust Workflow

```bash
# 1. Pull latest changes
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b pr-proj-1234

# 3. Develop with fast feedback
cargo watch -x check -x test

# 4. Make changes
vim src/main.rs

# 5. Run quality checks
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

# 6. Commit changes
git add -A
git commit -m "feat(module): add feature (PROJ-1234)"

# 7. Push to remote
git push origin pr-proj-1234
```

**Quick Commands:**

```bash
# Fast check (no binary)
cargo check

# Build optimized
cargo build --release

# Run with args
cargo run -- --arg value

# Clean build artifacts
cargo clean
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Workflow

```bash
# 1. Pull latest changes
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b pr-proj-1234

# 3. Activate virtual environment
source .venv/bin/activate

# 4. Install dependencies
pip install -e ".[dev]"

# 5. Develop with fast feedback
pytest-watch

# 6. Make changes
vim src/mypackage/module.py

# 7. Run quality checks
black .
pylint src/
mypy src/
pytest --cov

# 8. Commit changes
git add -A
git commit -m "feat(module): add feature (PROJ-1234)"

# 9. Push to remote
git push origin pr-proj-1234
```

**Quick Commands:**

```bash
# Run script
python -m mypackage.main

# Install package locally
pip install -e .

# Update dependencies
pip install --upgrade -r requirements.txt

# Freeze dependencies
pip freeze > requirements.txt
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Workflow

```bash
# 1. Pull latest changes
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b pr-proj-1234

# 3. Initialize module (if new)
go mod init github.com/user/project

# 4. Develop with fast feedback
gotestsum --watch

# 5. Make changes
vim main.go

# 6. Run quality checks
gofmt -w .
goimports -w .
golangci-lint run
go test ./... -cover

# 7. Commit changes
git add -A
git commit -m "feat(module): add feature (PROJ-1234)"

# 8. Push to remote
git push origin pr-proj-1234
```

**Quick Commands:**

```bash
# Run program
go run main.go

# Build binary
go build -o myapp

# Install binary
go install

# Clean cache
go clean -cache -modcache
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Workflow

```bash
# 1. Pull latest changes
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b pr-proj-1234

# 3. Install dependencies
pnpm install

# 4. Develop with fast feedback
pnpm run test:watch

# 5. Make changes
vim src/index.ts

# 6. Run quality checks
pnpm run format
pnpm run lint
pnpm run type-check
pnpm run test

# 7. Commit changes
git add -A
git commit -m "feat(module): add feature (PROJ-1234)"

# 8. Push to remote
git push origin pr-proj-1234
```

**Quick Commands:**

```bash
# Run TypeScript directly
pnpm tsx src/index.ts

# Build project
pnpm run build

# Type checking
pnpm tsc --noEmit

# Clean build
rm -rf dist node_modules
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Workflow

```bash
# 1. Pull latest changes
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b pr-proj-1234

# 3. Make executable
chmod +x script.sh

# 4. Develop with fast feedback
while true; do
    clear
    shellcheck script.sh
    ./tests/run_tests.sh
    sleep 2
done

# 5. Make changes
vim script.sh

# 6. Run quality checks
shellcheck script.sh
shfmt -w script.sh
./tests/run_tests.sh

# 7. Commit changes
git add -A
git commit -m "feat(script): add feature (PROJ-1234)"

# 8. Push to remote
git push origin pr-proj-1234
```

**Quick Commands:**

```bash
# Check syntax without executing
bash -n script.sh

# Debug script
bash -x script.sh

# Profile execution
time bash script.sh

# Check for portability
checkbashisms script.sh
```

<!-- /LANG -->

---

## Dependency Management

Manage project dependencies effectively.

<!-- LANG:rust -->

### Rust Dependencies

```bash
# Add dependency
cargo add serde --features derive

# Add dev dependency
cargo add --dev tempfile

# Add build dependency
cargo add --build cc

# Update dependencies
cargo update

# Check for outdated packages
cargo outdated

# Audit for security issues
cargo audit
```

**Cargo.toml Version Syntax:**

```toml
[dependencies]
serde = "1.0"           # Compatible updates
serde = "=1.0.150"      # Exact version
serde = ">=1.0, <2.0"   # Range
serde = "*"             # Any (avoid in production)
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Dependencies

```bash
# Install package
pip install requests

# Install with version
pip install requests==2.31.0

# Install from requirements
pip install -r requirements.txt

# Update package
pip install --upgrade requests

# Show installed packages
pip list

# Show dependency tree
pip install pipdeptree
pipdeptree
```

**requirements.txt Version Syntax:**

```txt
requests==2.31.0      # Exact version
requests>=2.30.0      # Minimum version
requests~=2.31.0      # Compatible release
requests>=2.30,<3.0   # Range
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Dependencies

```bash
# Add dependency (auto on import)
go get github.com/user/package

# Add specific version
go get github.com/user/package@v1.2.3

# Update dependency
go get -u github.com/user/package

# Update all dependencies
go get -u ./...

# Tidy modules (remove unused)
go mod tidy

# Verify dependencies
go mod verify

# Show dependency graph
go mod graph
```

**go.mod Version Syntax:**

```go
require (
    github.com/user/package v1.2.3      // Specific version
    github.com/user/other v1.2.0        // Minimum version
)

replace github.com/user/package => ../local/package  // Local replace
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Dependencies

```bash
# Add dependency
pnpm add lodash

# Add dev dependency
pnpm add -D @types/lodash

# Add specific version
pnpm add lodash@4.17.21

# Update dependency
pnpm update lodash

# Update all dependencies
pnpm update

# Remove unused dependencies
pnpm prune

# List outdated packages
pnpm outdated
```

**package.json Version Syntax:**

```json
{
  "dependencies": {
    "lodash": "^4.17.21", // Compatible updates
    "react": "~18.2.0", // Patch updates only
    "express": "4.18.2", // Exact version
    "axios": ">=1.0.0 <2.0.0" // Range
  }
}
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Dependencies

```bash
# Check for required commands
command -v git >/dev/null 2>&1 || {
    echo "git is required but not installed"
    exit 1
}

# Install system packages (Debian/Ubuntu)
apt-get update
apt-get install -y curl jq

# Install system packages (macOS)
brew install curl jq

# Document dependencies in README
# Required: bash 4.0+, curl, jq
# Optional: shellcheck, shfmt
```

**Version Checking:**

```bash
# Check bash version
if [ "${BASH_VERSINFO[0]}" -lt 4 ]; then
    echo "Bash 4.0+ required"
    exit 1
fi

# Check command version
version=$(curl --version | head -n1)
echo "Using: $version"
```

<!-- /LANG -->

---

## Quality Checks

Run quality checks before committing code.

<!-- LANG:rust -->

### Rust Quality Checks

```bash
# Format code
cargo fmt --all

# Check compilation
cargo check --all-targets --all-features

# Lint with zero warnings
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Generate documentation
cargo doc --no-deps

# All checks in one command
cargo fmt --all && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features
```

**CI Configuration (.github/workflows/ci.yml):**

```yaml
- name: Format check
  run: cargo fmt --all -- --check
- name: Clippy
  run: cargo clippy --all-targets --all-features -- -D warnings
- name: Test
  run: cargo test --all-features
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Quality Checks

```bash
# Format code
black .

# Sort imports
isort .

# Lint code
pylint src/

# Type check
mypy src/

# Run tests
pytest --cov --cov-report=html

# All checks in one command
black . && \
isort . && \
pylint src/ && \
mypy src/ && \
pytest --cov
```

**Configuration (pyproject.toml):**

```toml
[tool.black]
line-length = 100

[tool.pylint.messages_control]
disable = ["C0111"]

[tool.mypy]
strict = true

[tool.pytest.ini_options]
testpaths = ["tests"]
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Quality Checks

```bash
# Format code
gofmt -w .
goimports -w .

# Lint code
golangci-lint run

# Run tests
go test ./...

# Run tests with coverage
go test -cover ./...

# Vet code
go vet ./...

# All checks in one command
gofmt -w . && \
goimports -w . && \
golangci-lint run && \
go test -cover ./...
```

**Configuration (.golangci.yml):**

```yaml
linters:
  enable:
    - gofmt
    - goimports
    - govet
    - errcheck
    - staticcheck
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Quality Checks

```bash
# Format code
pnpm run format

# Lint code
pnpm run lint

# Type check
pnpm tsc --noEmit

# Run tests
pnpm test

# All checks in one command
pnpm run format && \
pnpm run lint && \
pnpm tsc --noEmit && \
pnpm test
```

**Scripts (package.json):**

```json
{
  "scripts": {
    "format": "prettier --write .",
    "lint": "eslint . --ext .ts,.tsx",
    "type-check": "tsc --noEmit",
    "test": "vitest run"
  }
}
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Quality Checks

```bash
# Check syntax
bash -n script.sh

# Lint with shellcheck
shellcheck script.sh

# Format with shfmt
shfmt -w -i 4 script.sh

# Run tests
./tests/run_tests.sh

# All checks in one command
bash -n script.sh && \
shellcheck script.sh && \
shfmt -d -i 4 script.sh && \
./tests/run_tests.sh
```

**shellcheck Configuration (.shellcheckrc):**

```bash
# Disable specific checks
disable=SC2034  # Unused variables
disable=SC1091  # Not following sourced files

# Set shell
shell=bash
```

<!-- /LANG -->

---

## Debugging

Debug code effectively using language-specific tools.

<!-- LANG:rust -->

### Rust Debugging

```bash
# Print debugging
println!("Debug: {:?}", variable);
dbg!(&variable);
eprintln!("Error: {}", error);

# Run with debug output
RUST_LOG=debug cargo run

# Run with backtrace
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run

# Debug with lldb
rust-lldb target/debug/myapp
(lldb) b main.rs:42
(lldb) run
(lldb) p variable
```

**VS Code launch.json:**

```json
{
  "type": "lldb",
  "request": "launch",
  "name": "Debug",
  "cargo": {
    "args": ["build", "--bin=myapp"]
  }
}
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Debugging

```bash
# Print debugging
print(f"Debug: {variable}")
import pprint; pprint.pprint(complex_object)

# Interactive debugging
import ipdb; ipdb.set_trace()

# Run with debugger
python -m pdb script.py

# Debug with ipdb
ipdb script.py
```

**Debugger Commands:**

```
(Pdb) n          # Next line
(Pdb) s          # Step into
(Pdb) c          # Continue
(Pdb) p variable # Print variable
(Pdb) l          # List code
(Pdb) bt         # Backtrace
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Debugging

```bash
# Print debugging
fmt.Printf("Debug: %+v\n", variable)
fmt.Fprintf(os.Stderr, "Error: %v\n", err)

# Debug with delve
dlv debug
(dlv) break main.go:42
(dlv) continue
(dlv) print variable

# Debug test
dlv test -- -test.run TestName

# Attach to running process
dlv attach $(pgrep myapp)
```

**Delve Commands:**

```
(dlv) break (b)   # Set breakpoint
(dlv) continue (c) # Continue execution
(dlv) next (n)    # Next line
(dlv) step (s)    # Step into
(dlv) print (p)   # Print variable
(dlv) stack (bt)  # Stack trace
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript Debugging

```bash
# Console debugging
console.log('Debug:', variable);
console.dir(object, { depth: null });
console.trace('Trace point');

# Debug with node
node --inspect-brk dist/index.js

# Debug with tsx
tsx --inspect-brk src/index.ts

# Debug tests
node --inspect-brk node_modules/.bin/vitest
```

**VS Code launch.json:**

```json
{
  "type": "node",
  "request": "launch",
  "name": "Debug",
  "runtimeExecutable": "tsx",
  "args": ["src/index.ts"],
  "sourceMaps": true
}
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Debugging

```bash
# Print debugging
echo "Debug: $variable"
declare -p variable  # Show variable attributes

# Enable debug mode
set -x  # Print commands
set -v  # Print input lines

# Debug with PS4
export PS4='+(${BASH_SOURCE}:${LINENO}): ${FUNCNAME[0]:+${FUNCNAME[0]}(): }'
set -x

# Run with debug
bash -x script.sh

# Debug specific section
set -x
# Code to debug
set +x

# Check for errors
set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Fail on pipe errors
```

**VSCode bashdb:**

```json
{
  "type": "bashdb",
  "request": "launch",
  "name": "Debug Script",
  "program": "${file}"
}
```

<!-- /LANG -->

---

## Additional Guidelines

### Code Review Checklist

- [ ] Code follows project conventions
- [ ] All quality checks pass
- [ ] Tests added for new functionality
- [ ] Documentation updated
- [ ] No debug code or commented code
- [ ] Commit messages follow format
- [ ] Changes are focused and minimal

### Performance Considerations

- Profile before optimizing
- Use language-specific profiling tools
- Benchmark performance-critical code
- Document performance requirements
- Test with realistic data sizes

### Security Best Practices

- Never commit secrets or credentials
- Use environment variables for config
- Validate all external input
- Keep dependencies updated
- Run security audits regularly
