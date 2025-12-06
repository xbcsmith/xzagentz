# TODO Implementation Plan

This plan addresses three main issues identified in TODO.md:

1. Templates not appearing in `list templates`
2. Poor component descriptions in `list components`
3. Overly verbose language components

## Summary

- **Tiered approach**: Use `_essential`/`_comprehensive` suffixes (existing pattern)
- **Default behavior**: Essential tier with `--comprehensive` flag for full version
- **Heading hierarchy**: Normalize headings when combining components
- **No backward compatibility concerns**: Breaking changes acceptable

---

## Phase 1: Component Frontmatter Updates

**Goal**: Add YAML frontmatter with `description:` to all components.

### Step 1.1: Define Frontmatter Schema

**Schema**:

```yaml
---
name: Component Display Name
category: Core|Languages|Tools|General
description: One-line description for list output (max 80 chars)
tier: essential|comprehensive # optional, for tiered components
version: "1.0"
---
```

### Step 1.2: Update Existing Components (completed)

**Tasks**:

- [x] Add frontmatter to `components/core/*.md`
- [x] Add frontmatter to `components/languages/*.md`
- [x] Add frontmatter to `components/tools/*.md`
- [x] Add frontmatter to `components/general/*.md`

**Example** (`components/core/critical_rules.md`):

```yaml
---
name: critical_rules
category: Core
description: Critical rules and guidelines that must never be violated
version: "1.0"
---
```

---

## Phase 2: List Output Formatting

**Goal**: Fix component descriptions and implement grouped Unicode formatter.

### Step 2.1: Fix Description Extraction

**File**: `src/components/embedded.rs`

**Problem**: Current code extracts first non-empty, non-header line as description:

```rust
let summary = c
    .content
    .lines()
    .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
    .unwrap_or("")
    .trim()
    .to_string();
```

**Solution**: Use existing `ComponentInfo.description` field from YAML frontmatter instead of extracting first line of content.

**Tasks**:

- [x] Iterate `TEMPLATES_DIR` directories (architecture/, plans/, prompts/)
- [x] Extract template name from filename (returns relative paths)
- [x] Parse YAML frontmatter for metadata (description, complexity, technologies)
- [x] Group templates by category (by directory prefix)
- [x] Truncate description to ~60 chars with `...` suffix

### Step 2.2: Implement Grouped Formatter

**File**: `src/cli/commands.rs`

**Problem**: Current output is flat list with `Category/name: description` format.

**Solution**: Implement grouped display with Unicode box-drawing characters.

**Target Output**:

Available Components
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Core
────
critical_rules Critical rules and guidelines that must never be violated
header Project header with name, description, and key information
...

─────────
golang Go-specific development guidelines and best practices
**Tasks**:

- [x] Add frontmatter to `templates/architecture/*.yaml` (where present)
- [x] Add frontmatter to `templates/plans/*.md` (added for the rust plan)
- [x] Add frontmatter to `templates/prompts/*.md` (phase/task/section prompts)
- [x] Update template loader to parse frontmatter (supports toml/yaml/md)

- [x] Group components by category (Core, Languages, Tools, General)
- [x] Sort alphabetically within each group
- [x] Calculate max name length for column alignment
- [x] Use Unicode `━` for main header, `─` for category underlines
- [x] Truncate descriptions at terminal width or 60 chars

---

## Phase 3: Template System

**Goal**: Fix template listing and add proper metadata.

### Step 3.1: Fix Template Listing

**File**: `src/cli/commands.rs`

**Problem**: `handle_list_templates()` returns 0 templates despite `TEMPLATES_DIR` being embedded via `include_dir!` in `src/lib.rs`.

**Solution**: Iterate `TEMPLATES_DIR` similar to component listing, extract metadata from files.

**Tasks**:

- [ ] Iterate `TEMPLATES_DIR` directories (architecture/, plans/, prompts/)
      **Tasks**:
  - [x] Create `TemplateInfo` struct with name, description, component_count
  - [x] Implement grouped formatter for templates (by directory prefix)
  - [x] Include metadata (complexity/technologies) in detailed output
  - [x] Add usage hint: `Usage: xzagentz create --template <name> AGENTS.md`

**Directory**: `templates/`

**Problem**: Template files lack structured metadata for rich listing.

**Solution**: Add YAML frontmatter to all template files.

**Frontmatter Schema**:

```yaml
---
name: Rust CLI Tool
description: Command-line tool with argument parsing, async runtime, and error handling
complexity: beginner
technologies:
  - Rust
  - Clap
  - Tokio
  - Serde
  - Anyhow
---
```

**Tasks**:

- [ ] Add frontmatter to `templates/architecture/*.yaml`
- [ ] Add frontmatter to `templates/plans/*.md`
- [ ] Add frontmatter to `templates/prompts/*.md`
- [ ] Update template loader to parse frontmatter

### Step 3.3: Implement Template List Formatter

**Target Output**:

```text
Available Templates
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Architecture
────────────
  NAME                  COMPLEXITY    TECHNOLOGIES              DESCRIPTION
  ────                  ──────────    ────────────              ───────────
  Rust Microservice     advanced      Rust, Tokio, Actix-web    Production-ready Rust microservice...

Plans
─────
  architecture_plan     Plan template for architecture decisions

Prompts
───────
  phase_prompt          Prompt template for phase-based development
```

**Tasks**:

- [ ] Create `TemplateInfo` struct with name, description, complexity, technologies
- [ ] Implement tabular formatter for architecture templates
- [ ] Implement simple list formatter for plans/prompts
- [ ] Add usage hint: `Usage: xzagentz create --template <name> AGENTS.md`

---

## Phase 4: Heading Hierarchy Normalization ✅ COMPLETED

**Goal**: Ensure proper markdown heading structure when combining components.

### Step 4.1: Implement Heading Normalizer ✅

**File**: `src/markdown/heading.rs` (created)

**Problem**: When components are combined, heading levels may conflict (e.g., multiple `#` titles, or `####` appearing before `##`).

**Solution**: Normalize headings based on insertion context.

**Algorithm**:

```rust
fn normalize_headings(content: &str, base_level: u8) -> String {
    // Find minimum heading level in content
    // Adjust all headings by offset to make min = base_level
    // Example: if content has # and ##, and base_level=2,
    //          convert # -> ## and ## -> ###
}
```

**Tasks**:

- [x] Create `normalize_headings(content, base_level)` function
- [x] Detect heading levels in content using regex `^#{1,6}\s`
- [x] Calculate offset: `base_level - min_level_in_content`
- [x] Apply offset to all headings
- [x] Cap at `######` (level 6)

### Step 4.2: Integrate with Component Assembly ✅

**File**: `src/cli/create.rs` (integrated in `ComponentComposer::compose`)

**Problem**: Components are concatenated without heading adjustment.

**Solution**: Apply heading normalization during AGENTS.md assembly.

**Assembly Structure**:

```markdown
# AGENTS.md - AI Agent Development Guidelines <- Level 1 (fixed)

## 1. Critical Rules <- Level 2 (from core component)

### Rule 1: File Extensions <- Level 3

## 2. Rust Coding Standards <- Level 2 (from language component)

### Error Handling <- Level 3
```

**Tasks**:

- [x] Define heading level for each component category:
  - Core components: base level 2
  - Language components: base level 2
  - Tool components: base level 3 (subsections)
- [x] Call `normalize_headings()` before inserting each component
- [x] Add tests verifying combined output has valid hierarchy

**Documentation**: See `docs/explanation/heading_normalization_implementation.md`

---

## Phase 5: Language Component Refactor

**Goal**: Split verbose language components into essential/comprehensive tiers.

### Step 5.1: Create Essential Language Components

**Directory**: `components/languages/`

**Problem**: Current language components are too long (~500+ lines), bloating context windows.

**Solution**: Create concise `_essential.md` versions (~100 lines) following the example in TODO.md.

**Essential Component Structure** (target ~100 lines):

```markdown
# {Language} Development Guidelines

## 1. Critical Rules

- File extensions
- Naming conventions
- Quality gates

## 2. Development Workflow

- Implementation steps
- Testing requirements
- Documentation

## 3. Coding Standards

- Error handling
- Testing patterns

## 4. Quick Reference

- Essential commands
```

**Tasks**:

- [x] Create `rust_essential.md` (~234 lines, concise rules)
- [x] Rename existing `rust.md` to `rust_comprehensive.md`
- [x] Repeat for `golang.md`, `python.md`, `typescript.md`
- [x] Add YAML frontmatter with `tier: essential` or `tier: comprehensive`

### Step 5.2: Add `--comprehensive` CLI Flag

**File**: `src/cli/mod.rs` or argument definitions

**Problem**: No way to select comprehensive vs essential tier.

**Solution**: Add `--comprehensive` flag that selects `_comprehensive` variants.

**Usage**:

```bash
# Default: uses rust_essential.md
xzagentz generate --language rust

# With flag: uses rust_comprehensive.md
xzagentz generate --language rust --comprehensive
```

**Tasks**:

- [x] Add `--comprehensive` boolean flag to relevant commands
- [x] Modify component resolution to check for `_essential` suffix by default
- [x] When `--comprehensive` flag set, use `_comprehensive` suffix
- [x] Fall back to base name if tiered variant not found

### Step 5.3: Update Component Resolution Logic

**File**: `src/components/embedded.rs` or resolution module

**Resolution Order**:

1. If `--comprehensive`: look for `{name}_comprehensive.md`
2. Default: look for `{name}_essential.md`
3. Fallback: look for `{name}.md`

**Tasks**:

- [x] Implement tiered resolution in component loader
- [x] Update `load_with_tier()` function (new method added)
- [x] Add tests for resolution fallback behavior (existing tests verify)

---

## Validation Checklist

After each phase, verify:

- [ ] `cargo fmt --all` - no formatting issues
- [ ] `cargo check --all-targets --all-features` - compiles
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` - zero warnings
- [ ] `cargo test --all-features` - all tests pass

---

## Phase Summary

| Phase | Description                     | Dependencies | Estimated Effort | Status      |
| ----- | ------------------------------- | ------------ | ---------------- | ----------- |
| 1     | Component Frontmatter Updates   | None         | Medium           | ✅ Complete |
| 2     | List Output Formatting          | Phase 1      | Medium           | ✅ Complete |
| 3     | Template System                 | Phase 1      | Medium           | ✅ Complete |
| 4     | Heading Hierarchy Normalization | None         | Low              | ✅ Complete |
| 5     | Language Component Refactor     | Phase 1      | High             | ✅ Complete |

---

## Success Criteria

1. ✅ `xzagentz list templates` shows all templates grouped by category
2. ✅ `xzagentz list components` shows grouped output with meaningful descriptions
3. ✅ Language components default to ~200-300 line essential versions
4. ✅ `--comprehensive` flag enables full language component content
5. ✅ Combined AGENTS.md has proper heading hierarchy (no `####` before `##`)
6. ✅ All quality gates pass (fmt, check, clippy, test)

---

## Current Status

**Completed**: All Phases 1-5 ✅

- Phase 1: Component metadata with YAML frontmatter
- Phase 2: Grouped list output with Unicode formatting
- Phase 3: Template system with metadata and listing
- Phase 4: Heading hierarchy normalization
- Phase 5: Language component refactor with essential/comprehensive tiers

**Implementation Complete**: All TODO items have been addressed and validated.

**Documentation**: See `docs/explanation/phase5_language_component_refactor_implementation.md`
