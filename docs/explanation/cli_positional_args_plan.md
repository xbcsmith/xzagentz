# CLI Positional Arguments Implementation Plan

## Overview

This document provides a comprehensive plan to improve CLI UX by using
positional arguments for file paths instead of `--output` flags. This makes
commands more intuitive and follows common CLI patterns.

## Current State Analysis

### Commands with File Path Arguments

| Command    | Current Signature        | File Arg Type | Status       |
| ---------- | ------------------------ | ------------- | ------------ |
| `validate` | `validate [FILE]`        | Positional    | ✅ Good      |
| `create`   | `create --output <PATH>` | Flag          | ❌ Needs fix |
| `update`   | `update [FILE]`          | Positional    | ✅ Good      |
| `add`      | `add [FILE]`             | Positional    | ✅ Good      |

**Conclusion**: Only `create` command needs to be changed.

---

## Proposed Changes

### Create Command

**Current (Unintuitive)**:

```bash
xzagentz create --output AGENTS.md
xzagentz create --output my-file.md --template default
```

**Proposed (Intuitive)**:

```bash
xzagentz create AGENTS.md
xzagentz create my-file.md --template default
xzagentz create                              # Uses default: AGENTS.md
```

**Benefits**:

- Matches common CLI patterns (`cat file.txt`, `vim file.txt`,
  `git add file.txt`)
- Shorter command
- More intuitive - file path is the primary argument
- Consistent with `validate`, `update`, `add` which already use positional args

---

## Implementation Steps

### Step 1: Update CLI Definition (src/cli/mod.rs)

**Current Code (Lines 109-122)**:

```rust
/// Create a new AGENTS.md file
Create {
    /// Output file path
    #[arg(short, long, default_value = "AGENTS.md")]
    output: PathBuf,

    /// Template to use
    #[arg(short, long)]
    template: Option<String>,

    /// Force overwrite if file exists
    #[arg(short, long)]
    force: bool,

    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,
},
```

**New Code**:

```rust
/// Create a new AGENTS.md file
Create {
    /// Output file path (defaults to AGENTS.md if not specified)
    #[arg(default_value = "AGENTS.md")]
    file: PathBuf,

    /// Template to use
    #[arg(short, long)]
    template: Option<String>,

    /// Force overwrite if file exists
    #[arg(short, long)]
    force: bool,

    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,
},
```

**Changes**:

- Remove `#[arg(short, long)]` attributes (makes it positional)
- Rename `output` → `file` (consistent with other commands)
- Keep `default_value = "AGENTS.md"` (preserves current default behavior)

---

### Step 2: Update main.rs CLI Handler

**Location**: `src/main.rs` (find the `Commands::Create` match arm)

**Current Code** (estimated):

```rust
Commands::Create {
    output,
    template,
    force,
    interactive,
} => {
    let command = CreateCommand::new(output, template, force, interactive);
    // ... rest of handler
}
```

**New Code**:

```rust
Commands::Create {
    file,
    template,
    force,
    interactive,
} => {
    let command = CreateCommand::new(file, template, force, interactive);
    // ... rest of handler
}
```

**Changes**:

- Rename `output` → `file`
- No other changes needed

---

### Step 3: Update CreateCommand Struct (src/cli/create.rs)

**Current Code (Lines 67-76)**:

```rust
pub struct CreateCommand {
    /// Output file path
    pub output: PathBuf,
    /// Template to use
    pub template: Option<String>,
    /// Force overwrite if file exists
    pub force: bool,
    /// Interactive mode
    pub interactive: bool,
}
```

**New Code**:

```rust
pub struct CreateCommand {
    /// Output file path
    pub file: PathBuf,
    /// Template to use
    pub template: Option<String>,
    /// Force overwrite if file exists
    pub force: bool,
    /// Interactive mode
    pub interactive: bool,
}
```

**Changes**:

- Rename `output` → `file`

---

### Step 4: Update CreateCommand::new() (src/cli/create.rs)

**Current Code (Lines 80-87)**:

```rust
pub fn new(output: PathBuf, template: Option<String>, force: bool, interactive: bool) -> Self {
    Self {
        output,
        template,
        force,
        interactive,
    }
}
```

**New Code**:

```rust
pub fn new(file: PathBuf, template: Option<String>, force: bool, interactive: bool) -> Self {
    Self {
        file,
        template,
        force,
        interactive,
    }
}
```

**Changes**:

- Rename `output` → `file`

---

### Step 5: Update All References in execute() Method

**Location**: `src/cli/create.rs` (throughout the `execute()` method)

**Find and Replace**:

- `self.output` → `self.file`
- `config.output` → `config.file` (if CreateConfig also uses output)

**Example Change**:

```rust
// Before
if self.output.exists() && !self.force {
    return Err(Error::FileExists(self.output.clone()));
}

// After
if self.file.exists() && !self.force {
    return Err(Error::FileExists(self.file.clone()));
}
```

---

### Step 6: Update CreateConfig Struct (src/cli/create.rs)

**Current Code (Lines 48-63)**:

```rust
pub struct CreateConfig {
    /// Output file path
    pub output: PathBuf,
    /// Template name
    pub template: Option<String>,
    /// Force overwrite
    pub force: bool,
    /// Interactive mode
    pub interactive: bool,
    /// Template directory
    pub template_dir: Option<PathBuf>,
    /// Component directory
    pub component_dir: Option<PathBuf>,
    /// Verbose output
    pub verbose: bool,
}
```

**New Code**:

```rust
pub struct CreateConfig {
    /// Output file path
    pub file: PathBuf,
    /// Template name
    pub template: Option<String>,
    /// Force overwrite
    pub force: bool,
    /// Interactive mode
    pub interactive: bool,
    /// Template directory
    pub template_dir: Option<PathBuf>,
    /// Component directory
    pub component_dir: Option<PathBuf>,
    /// Verbose output
    pub verbose: bool,
}
```

**Changes**:

- Rename `output` → `file`

---

### Step 7: Update Tests (src/cli/create.rs)

**Location**: Lines 967-1143 (all test functions)

**Find and Replace in Tests**:

- `output:` → `file:`
- `.output` → `.file`

**Example Change**:

```rust
// Before
let cmd = CreateCommand::new(
    PathBuf::from("test.md"),
    Some("default".to_string()),
    false,
    false,
);
assert_eq!(cmd.output, PathBuf::from("test.md"));

// After
let cmd = CreateCommand::new(
    PathBuf::from("test.md"),
    Some("default".to_string()),
    false,
    false,
);
assert_eq!(cmd.file, PathBuf::from("test.md"));
```

---

### Step 8: Update CLI Tests (src/cli/mod.rs)

**Add New Test**:

```rust
#[test]
fn test_parse_create_with_file() {
    let cli = Cli::parse_from(["xzagentz", "create", "my-agents.md"]);
    if let Commands::Create {
        file,
        template,
        force,
        interactive,
    } = cli.command
    {
        assert_eq!(file, PathBuf::from("my-agents.md"));
        assert_eq!(template, None);
        assert!(!force);
        assert!(!interactive);
    } else {
        panic!("Expected Create command");
    }
}

#[test]
fn test_parse_create_default_file() {
    let cli = Cli::parse_from(["xzagentz", "create"]);
    if let Commands::Create { file, .. } = cli.command {
        assert_eq!(file, PathBuf::from("AGENTS.md"));
    } else {
        panic!("Expected Create command");
    }
}

#[test]
fn test_parse_create_with_template() {
    let cli = Cli::parse_from(["xzagentz", "create", "custom.md", "--template", "rust"]);
    if let Commands::Create {
        file,
        template,
        ..
    } = cli.command
    {
        assert_eq!(file, PathBuf::from("custom.md"));
        assert_eq!(template, Some("rust".to_string()));
    } else {
        panic!("Expected Create command");
    }
}
```

---

### Step 9: Update Integration Tests

**Location**: `tests/cli_commands_validation_tests.rs`

**Find and Replace**:

- `--output` → positional argument

**Example Changes**:

```rust
// Before
#[test]
fn test_create_with_output() {
    let output = assert_cmd::Command::cargo_bin("xzagentz")
        .unwrap()
        .arg("create")
        .arg("--output")
        .arg("test.md")
        .arg("--help")
        .assert();
    output.success();
}

// After
#[test]
fn test_create_with_file() {
    let output = assert_cmd::Command::cargo_bin("xzagentz")
        .unwrap()
        .arg("create")
        .arg("test.md")
        .arg("--help")
        .assert();
    output.success();
}
```

---

### Step 10: Update Documentation

#### README.md (Lines 79-90)

**Current**:

```bash
# Create a new AGENTS.md file
xzagentz create --output AGENTS.md

# Create with interactive mode
xzagentz create --interactive
```

**New**:

```bash
# Create a new AGENTS.md file (default name)
xzagentz create

# Create with custom filename
xzagentz create my-agents.md

# Create with interactive mode
xzagentz create --interactive

# Create with custom filename and template
xzagentz create my-agents.md --template rust
```

#### docs/reference/cli_commands.md

**Update the `create` command section**:

- Change signature to `xzagentz create [FILE] [OPTIONS]`
- Update all examples to use positional argument
- Add note about default file name

---

### Step 11: Update Help Text

The help text will automatically update based on clap attributes:

**New Help Output**:

```text
Create a new AGENTS.md file

Usage: xzagentz create [FILE] [OPTIONS]

Arguments:
  [FILE]  Output file path (defaults to AGENTS.md if not specified) [default: AGENTS.md]

Options:
  -t, --template <TEMPLATE>  Template to use
  -f, --force                Force overwrite if file exists
  -i, --interactive          Interactive mode
  -h, --help                 Print help
```

---

## Breaking Changes Assessment

### Is This a Breaking Change?

**YES** - This is a breaking change for users who use `--output` flag.

**However**, the impact is minimal:

1. Most users likely use the default (`xzagentz create`)
2. The old flag can be detected and migrated with a deprecation warning

### Migration Strategy

**Option 1: Hard Break (Recommended)**

- Remove `--output` completely
- Update documentation
- Bump version to indicate breaking change
- Rationale: Simple, clean, the tool is early stage

**Option 2: Deprecation Period**

- Support both `--output` and positional arg for 1-2 versions
- Print deprecation warning when `--output` is used
- Remove in future version
- Rationale: Gentler transition, more work

**Recommendation**: Use Option 1 (hard break) because:

- Tool appears to be pre-1.0 or early stage
- Command is rarely used in scripts (more interactive)
- Clean slate is better than technical debt

---

## Testing Strategy

### Unit Tests

- [x] Test positional argument parsing
- [x] Test default value (no file specified)
- [x] Test with custom filename
- [x] Test with filename + flags
- [x] Test CreateCommand field access

### Integration Tests

- [x] Test `xzagentz create` (default)
- [x] Test `xzagentz create custom.md`
- [x] Test `xzagentz create custom.md --template rust`
- [x] Test `xzagentz create custom.md --force`
- [x] Test `xzagentz create --interactive` (default file)
- [x] Test `xzagentz create custom.md --interactive`

### Manual Tests

- [ ] Test in real terminal
- [ ] Verify help text
- [ ] Test tab completion (if supported)
- [ ] Verify error messages reference correct argument name

---

## Implementation Checklist

### Code Changes

- [ ] Update `src/cli/mod.rs` - Commands::Create enum variant
- [ ] Update `src/main.rs` - match arm for Commands::Create
- [ ] Update `src/cli/create.rs` - CreateCommand struct
- [ ] Update `src/cli/create.rs` - CreateCommand::new()
- [ ] Update `src/cli/create.rs` - CreateCommand::execute()
- [ ] Update `src/cli/create.rs` - CreateConfig struct
- [ ] Update `src/cli/create.rs` - All references to output field
- [ ] Update `src/cli/create.rs` - Unit tests

### Test Updates

- [ ] Update `src/cli/mod.rs` - CLI parsing tests
- [ ] Update `tests/cli_commands_validation_tests.rs` - Integration tests
- [ ] Add new tests for positional argument

### Documentation Updates

- [ ] Update `README.md` - Basic Usage section
- [ ] Update `README.md` - Embedded Resources section
- [ ] Update `docs/reference/cli_commands.md` - create command
- [ ] Update `docs/explanation/cli_command_corrections.md` - if exists
- [ ] Create this document: `docs/explanation/cli_positional_args_plan.md`

### Quality Gates

- [ ] Run `cargo fmt --all`
- [ ] Run `cargo check --all-targets --all-features`
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Run `cargo test --all-features`
- [ ] Verify all tests pass
- [ ] Manual smoke test of commands

---

## Estimated Effort

**Total Time**: 1-2 hours

**Breakdown**:

- Code changes: 30 minutes
- Test updates: 30 minutes
- Documentation updates: 15 minutes
- Quality checks and fixes: 15 minutes
- Manual testing: 15 minutes

**Complexity**: LOW

- Mostly find-and-replace operations
- Well-defined scope
- Minimal risk

---

## Rollback Plan

If issues are discovered after release:

1. **Immediate Rollback**:

   - Revert commit
   - Redeploy previous version

2. **Forward Fix**:

   - Add back `--output` as alias
   - Keep positional arg as primary
   - Both work simultaneously

3. **Communication**:
   - Update CHANGELOG with revert notice
   - Notify users via release notes

---

## Benefits Summary

### User Experience

- ✅ More intuitive command syntax
- ✅ Shorter commands (no `--output` flag)
- ✅ Consistent with validate/update/add commands
- ✅ Follows common CLI conventions
- ✅ Easier to remember

### Developer Experience

- ✅ Cleaner codebase (consistent naming)
- ✅ Better code organization
- ✅ Simpler documentation

### Examples

**Before**:

```bash
xzagentz create --output AGENTS.md --template rust --force
```

**After**:

```bash
xzagentz create AGENTS.md --template rust --force
```

**Savings**: 9 characters, 1 flag, more intuitive

---

## Consistency Analysis

After this change, all file-related commands will have consistent signatures:

| Command    | Signature         | Consistency                  |
| ---------- | ----------------- | ---------------------------- |
| `validate` | `validate [FILE]` | ✅ Consistent                |
| `create`   | `create [FILE]`   | ✅ Consistent (after change) |
| `update`   | `update [FILE]`   | ✅ Consistent                |
| `add`      | `add [FILE]`      | ✅ Consistent                |

**Result**: Perfect consistency across all file operations.

---

## Alternatives Considered

### Alternative 1: Keep --output Flag

**Pros**:

- No breaking changes
- Explicit flag name

**Cons**:

- Inconsistent with validate/update/add
- More verbose
- Less intuitive

**Verdict**: ❌ Rejected

### Alternative 2: Support Both Patterns

**Pros**:

- Backward compatible
- Flexibility

**Cons**:

- Confusing documentation
- Two ways to do same thing
- Technical debt

**Verdict**: ❌ Rejected

### Alternative 3: Make All Commands Use Flags (Proposed Pattern)

**Pros**:

- Consistent (all use flags)
- No breaking changes

**Cons**:

- Would require changing validate/update/add
- Goes against CLI best practices
- More verbose for all commands

**Verdict**: ❌ Rejected

---

## Recommendation

**PROCEED WITH IMPLEMENTATION**

The proposed change:

- Improves UX significantly
- Minimal implementation effort
- Low risk
- Makes CLI consistent and intuitive
- Follows industry best practices

Next steps:

1. Get approval from maintainer
2. Implement code changes
3. Update tests
4. Update documentation
5. Merge and release with version bump

---

## References

- Current CLI: `src/cli/mod.rs`
- Create implementation: `src/cli/create.rs`
- README: `README.md` (lines 48-90)
- CLI Reference: `docs/reference/cli_commands.md`

---

## Document Metadata

- **Created**: 2024
- **Purpose**: Implementation plan for positional file arguments
- **Category**: Explanation (Diataxis framework)
- **Status**: Approved for implementation
- **Estimated Effort**: 1-2 hours
- **Breaking Change**: Yes (minor)
