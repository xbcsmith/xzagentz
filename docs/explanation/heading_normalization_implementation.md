# Heading Normalization Implementation

## Overview

Phase 4 implemented heading normalization to ensure components combine into a valid
AGENTS.md hierarchy without skipping heading levels.

## Components Delivered

- `src/markdown/heading.rs` — public `normalize_headings(content, target_base)` utility
- `src/markdown/mod.rs` — exports `normalize_headings`
- `src/cli/create.rs` — `ComponentComposer::compose` now normalizes headings when composing components
- Tests added/updated for heading normalization

## Implementation Details

- The normalizer finds the minimum heading level present in a component and shifts
  all heading levels upward so the minimum becomes the requested `target_base`.
- Cap at `######` (level 6) so headings never exceed the markdown limit.
- Integration points:
  - `ComponentComposer::compose` sets per-component bases:
    - First component: base level 1
    - Core/Languages/General: base level 2
    - Tools: base level 3
  - Normalization is applied after language filtering and placeholder rendering so the
    result is the final normalized text that gets concatenated.

## Tests

- Unit tests added in `src/markdown/heading.rs` cover no-headings, shift-up and cap-at-6 cases.
- Full project test-suite passed locally after integration.

## Validation Checklist

- `cargo fmt --all` — OK
- `cargo check --all-targets --all-features` — OK
- `cargo clippy --all-targets --all-features -- -D warnings` — OK
- `cargo test --all-features` — OK (all tests pass)

## Next Steps

- Phase 5: Split language components into essential/comprehensive files and add `--comprehensive` CLI flag.
