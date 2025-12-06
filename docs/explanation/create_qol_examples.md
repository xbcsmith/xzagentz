# Create command — QoL examples

## Overview

Quick examples demonstrating the new CLI QoL features added: `--tier`, `--dry-run`, `--diff`, `list templates` filters.

### Default (essential) vs comprehensive

Default behavior uses compact language components (essential tier):

```
xzagentz create AGENTS.md --template rust-binary
```

Larger, verbose language sections are selected with `--tier comprehensive`:

```
xzagentz create AGENTS.md --template rust-binary --tier comprehensive
```

### Dry-run

Preview which component files will be selected and show short, normalized excerpts without writing the file:

```
xzagentz create --template rust-binary --dry-run
```

Example dry-run excerpt output:

```
Dry-run: components selected (no file written):
  - Quick Reference (Core) -> file: quick_reference
    # Quick Reference
    Short guidance
    --

  - Rust Essential (Languages) -> file: rust_essential
    ## Rust Essential
    Important rules
    --
```

### Diff preview

See a tiny unified diff against an existing target file (first few hunks) with `--diff`:

```
xzagentz create AGENTS.md --template rust-binary --diff
```

### list templates filtering

Filter by metadata key/value and technology keywords:

```
xzagentz list templates --detailed --filter complexity=advanced --tech Rust
```
