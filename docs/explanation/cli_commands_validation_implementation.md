# CLI Commands Validation Test Implementation

## Overview

This document describes the implementation of comprehensive functional tests that validate all CLI commands documented in `docs/reference/cli_commands.md`. The test suite ensures that the actual CLI behavior matches the documentation and identifies discrepancies that need to be addressed.

## Components Delivered

- `tests/cli_commands_validation_tests.rs` (1,042 lines) - Comprehensive CLI test suite
- `docs/explanation/cli_commands_validation_implementation.md` (This document)

Total: Approximately 1,100 lines

## Implementation Details

### Test Suite Architecture

The test suite follows Rust testing best practices and uses the following structure:

1. Test helper infrastructure (`TestEnv` struct)
2. Global options tests (help, version, verbose, format, directory flags)
3. Command-specific tests organized by command
4. Error case validation
5. Integration with `assert_cmd` and `predicates` crates

### Test Environment Helper

Created a `TestEnv` struct to manage isolated test environments:

```rust
struct TestEnv {
    temp_dir: TempDir,
    config_dir: PathBuf,
    components_dir: PathBuf,
    templates_dir: PathBuf,
}
```

This helper provides:
- Isolated temporary directories for each test
- Automatic cleanup on test completion
- Configured environment variables for xzagentz paths
- Convenience method for creating CLI commands

### Test Coverage by Command

#### Global Options (11 tests)
- `--help` and `-h` flags
- `--version` and `-V` flags
- `--verbose` and `-v` flags
- `--config-dir`, `--component-dir`, `--template-dir` flags
- `--format` with `json` and `human` values

#### Init Command (10 tests)
- Default initialization
- Custom directories with `--components-dir` and `--templates-dir`
- Short flag variants (`-c`, `-t`)
- Dry run mode with `--dry-run` and `-n`
- Force overwrite with `--force` and `-f`

#### List Command (7 tests)
- `list components` with and without category filter
- `list templates` with and without detailed mode
- Category filtering with `--category` and `-c`
- Detailed output with `--detailed` and `-d`
- JSON format output

#### Validate Command (7 tests)
- Default validation of AGENTS.md
- Detailed reports with `--detailed` and `-d`
- Auto-fix mode with `--fix` and `-f`
- JSON format output
- Custom file validation

#### Create Command (6 tests)
- Default creation
- Custom output with `--output` and `-o`
- Template selection with `--template` and `-t`
- Force overwrite with `--force` and `-f`
- File creation verification

#### Update Command (5 tests)
- Section requirement validation
- Section update with `--section` and `-s`
- Backup control with `--backup` and `-b`
- Error handling for missing sections

#### Add Command (6 tests)
- Component requirement validation
- Component addition with `--component` and `-c`
- Position control with `--position` and `-p`
- Position values: `top`, `bottom`, `beginning`, `end`

#### Prompt Command (4 tests)
- Help display
- List subcommand
- Generate subcommand with template
- Generate with variables

#### Implementation Command (4 tests, marked as ignored)
- Help display
- Input requirement validation
- Input/output file handling
- Phase specification
- Note: Tests require LLM API key

#### Architecture Command (5 tests, marked as ignored)
- Help display
- Project and description parameters
- Architecture pattern specification
- Language specification
- Refinement mode
- Note: Tests require LLM API key

#### Error Handling (3 tests)
- Invalid command detection
- Invalid global option detection
- Invalid format option detection

### Test Results Summary

#### Total Tests: 71
- Passed: 52 (73.2%)
- Failed: 12 (16.9%)
- Ignored: 7 (9.9%)

#### Passing Tests (52)

All tests passed for:
- Global options (help, version, verbose, format, directories)
- List commands (components and templates)
- Validate commands (all variants)
- Create commands (all variants)
- Update commands (all variants)
- Add commands (all variants)
- Architecture and Implementation help display
- Error handling (invalid commands and options)

#### Failed Tests (12)

All failures are in the Init command tests:

1. `test_init_default` - Failed due to existing files in config directory
2. `test_init_dry_run` - Failed due to existing files detection
3. `test_init_dry_run_short` - Failed due to existing files detection
4. `test_init_force` - Failed, force flag not working as expected
5. `test_init_force_short` - Failed, force flag not working as expected
6. `test_init_with_components_dir` - Failed due to existing files
7. `test_init_with_components_dir_short` - Failed due to existing files
8. `test_init_with_templates_dir` - Failed due to existing files
9. `test_init_with_templates_dir_short` - Failed due to existing files
10. `test_prompt_generate_with_template` - Command structure doesn't match docs
11. `test_prompt_generate_with_variables` - Command structure doesn't match docs
12. `test_prompt_list` - Subcommand doesn't exist as documented

#### Ignored Tests (7)

Tests requiring LLM API keys:
- `test_implementation_requires_input`
- `test_implementation_with_input`
- `test_implementation_with_phases`
- `test_architecture_with_project_and_description`
- `test_architecture_with_pattern`
- `test_architecture_with_language`
- `test_architecture_refine`

## Issues Identified

### Issue 1: Init Command Directory Resolution

**Problem**: The `init` command is detecting files in the global config directory rather than using the test-isolated directories set via environment variables.

**Root Cause**: The embedded resources system is extracting to the default system directories instead of respecting the `XZAGENTZ_CONFIG_DIR`, `XZAGENTZ_COMPONENT_DIR`, and `XZAGENTZ_TEMPLATE_DIR` environment variables.

**Error Message**:
```
Error: Found 30 existing files. Use --force to overwrite:
  components/core/critical_rules.md
  components/core/error_handling.md
  [... more files ...]
```

**Impact**: All 9 init command tests fail.

**Fix Required**:
- Update `src/cli/init.rs` to properly resolve directories from environment variables
- Ensure embedded resource extraction respects custom directory parameters
- Fix directory resolution order: CLI args > environment variables > defaults

### Issue 2: Init Command Force Flag

**Problem**: The `--force` flag is not working correctly. Even when specified, the command still fails with "Found X existing files" error.

**Root Cause**: The force flag logic is not being applied before the file existence check, or the check is happening at the wrong level.

**Impact**: Tests `test_init_force` and `test_init_force_short` fail.

**Fix Required**:
- Ensure force flag bypasses existence checks
- Update file extraction logic to overwrite when force is enabled
- Add proper error handling flow for force mode

### Issue 3: Prompt Command Documentation Mismatch

**Problem**: The documentation in `cli_commands.md` describes a different interface than what is actually implemented.

**Documented Interface** (from cli_commands.md):
```bash
xzagentz prompt generate --template <NAME> --output <FILE> --variables <JSON>
xzagentz prompt list
```

**Actual Interface** (from --help output):
```bash
xzagentz prompt generate [OPTIONS]  # Has different options
xzagentz prompt show                # Not documented
xzagentz prompt complete            # Not documented
xzagentz prompt next                # Not documented
xzagentz prompt progress            # Not documented
xzagentz prompt verify              # Not documented
xzagentz prompt reset               # Not documented
```

**Impact**: 3 tests fail due to documentation mismatch.

**Fix Required**: Choose one of the following:
1. Update documentation to match implementation (recommended)
2. Update implementation to match documentation
3. Implement both interfaces for backwards compatibility

### Issue 4: Dry Run Should Not Fail on Existing Files

**Problem**: The `--dry-run` flag still performs validation checks that cause failures, even though it should only preview actions.

**Expected Behavior**: Dry run should show what would be extracted without failing on existing files.

**Actual Behavior**: Dry run fails with file existence errors.

**Impact**: Tests `test_init_dry_run` and `test_init_dry_run_short` fail.

**Fix Required**:
- Implement dry-run mode that skips all validation checks
- Display preview of what would be extracted
- Return success exit code in dry-run mode

## Recommendations

### Priority 1: Fix Init Command Directory Resolution

This is the most critical issue affecting 9 tests. Steps:

1. Review `src/cli/init.rs` and `src/infrastructure/embedded_resources.rs`
2. Ensure environment variable precedence: CLI args > env vars > defaults
3. Update embedded resource extraction to use resolved directories
4. Add unit tests for directory resolution logic

### Priority 2: Fix Init Command Force Flag

Required for proper overwrite functionality:

1. Move force flag check before file existence validation
2. Update extraction logic to honor force flag
3. Add integration test specifically for force overwrite scenario

### Priority 3: Update Prompt Command Documentation

Documentation must match implementation:

1. Review actual prompt command implementation in `src/cli/prompt.rs`
2. Update `docs/reference/cli_commands.md` with correct subcommands
3. Document all subcommands: generate, show, complete, next, progress, verify, reset
4. Update examples to match actual usage

### Priority 4: Implement Proper Dry Run Mode

Enhance user experience:

1. Make dry-run mode purely informational
2. Display extraction preview without validation
3. Show file counts and destination paths
4. Never fail in dry-run mode

## Testing Strategy

### Running Tests

Run all CLI validation tests:
```bash
cargo test --test cli_commands_validation_tests
```

Run specific test:
```bash
cargo test --test cli_commands_validation_tests test_init_default
```

Run with verbose output:
```bash
cargo test --test cli_commands_validation_tests -- --nocapture
```

Run only non-ignored tests:
```bash
cargo test --test cli_commands_validation_tests --tests
```

### After Fixes Are Applied

1. Run full test suite: `cargo test --test cli_commands_validation_tests`
2. Expected: All 64 non-ignored tests should pass (52 already passing + 12 fixed)
3. Verify exit codes match expectations
4. Validate output format (human and JSON modes)
5. Test error messages are user-friendly

## Validation Results

- cargo fmt --all: Passed
- cargo check --all-targets --all-features: Passed
- cargo clippy --all-targets --all-features -D warnings: Passed (0 warnings)
- cargo test --test cli_commands_validation_tests: 52/71 passed (73.2%)

## Code Quality Notes

The test implementation follows AGENTS.md guidelines:

- Proper error handling with Result types
- Descriptive test names following pattern: `test_{command}_{condition}_{expected}`
- No unwrap() calls without justification
- Comprehensive documentation
- Test isolation using temporary directories
- Serial execution for tests that modify global state
- Proper cleanup through RAII pattern (TempDir)

## Usage Examples

### Running Validation Tests

```bash
# Run all CLI validation tests
cargo test --test cli_commands_validation_tests

# Run only passing tests
cargo test --test cli_commands_validation_tests -- --skip test_init --skip test_prompt

# Run with detailed output
RUST_BACKTRACE=1 cargo test --test cli_commands_validation_tests -- --nocapture

# Run only init command tests
cargo test --test cli_commands_validation_tests test_init
```

### Expected Output After Fixes

```
running 71 tests
test test_add_help ... ok
test test_add_requires_component ... ok
test test_add_with_component ... ok
[... all tests ...]
test test_version_flag ... ok
test test_version_flag_short ... ok

test result: ok. 64 passed; 0 failed; 7 ignored; 0 measured; 0 filtered out
```

## References

- CLI Commands Reference: `docs/reference/cli_commands.md`
- Implementation: `tests/cli_commands_validation_tests.rs`
- Init Module: `src/cli/init.rs`
- Prompt Module: `src/cli/prompt.rs`
- Embedded Resources: `src/infrastructure/embedded_resources.rs`
- AGENTS.md Development Guidelines: `AGENTS.md`

## Conclusion

The CLI validation test suite successfully validates 73.2% of documented commands. The remaining 16.9% of failures are concentrated in two areas:

1. Init command directory resolution and force flag handling (9 tests)
2. Prompt command documentation mismatch (3 tests)

All failures have clear root causes and remediation paths. The test suite provides a solid foundation for regression testing and serves as executable documentation of CLI behavior.

Once the identified issues are resolved, the test suite will provide comprehensive validation that the CLI implementation matches its documentation, ensuring a consistent user experience.
