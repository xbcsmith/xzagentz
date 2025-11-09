# How to Verify YAML Frontmatter is Stripped

## Quick Reference

This guide shows how to verify that generated AGENTS.md files do not contain YAML frontmatter from component files.

## Problem

Previously, generated files included YAML frontmatter like this:

```markdown
---
component:
  name: critical_rules
  category: core
  version: 2.0.0
---

# Critical Rules

Content here...
```

This metadata should not appear in the final output.

## Solution

As of the latest version, YAML frontmatter is automatically stripped during component loading. Generated files now start directly with markdown content:

```markdown
# Critical Rules

Content here...
```

## Verification Steps

### Step 1: Generate a Test File

```bash
./target/release/xzagentz create --output test_agents.md
```

### Step 2: Check the First Lines

```bash
head -30 test_agents.md
```

**Expected**: The first line should be a markdown heading (starting with `#`), not YAML frontmatter.

### Step 3: Search for YAML Markers

```bash
grep -n "^component:" test_agents.md
grep -n "^---$" test_agents.md | head -5
```

**Expected**:
- No `component:` lines found
- Any `---` lines should be section separators (surrounded by blank lines), not YAML delimiters

### Step 4: Interactive Mode Test

```bash
./target/release/xzagentz create --interactive --output my_custom.md
```

Follow the prompts and select components. Then verify:

```bash
head -30 my_custom.md
grep "component:" my_custom.md
```

**Expected**: No YAML frontmatter in the output.

## Troubleshooting

### Issue: YAML Frontmatter Still Appears

**Symptoms**: Generated file starts with `---` and contains `component:` metadata.

**Possible Causes**:
1. Using an old binary (rebuild required)
2. Custom component files with non-standard format
3. Component cache contains old content

**Solution**:

```bash
# Rebuild the project
cargo build --release

# Clear any cached state
rm -rf target/debug
rm -rf target/release

# Rebuild and test
cargo build --release
./target/release/xzagentz create --output test.md
```

### Issue: Content is Missing After Frontmatter

**Symptoms**: Generated file is shorter than expected, or sections are missing.

**Possible Causes**:
1. Component file has unclosed frontmatter delimiters
2. Component file is malformed

**Solution**:

Check component files for proper frontmatter format:

```bash
# Find components with frontmatter issues
for f in components/**/*.md; do
    if head -1 "$f" | grep -q "^---$"; then
        # Count --- delimiters
        count=$(grep -c "^---$" "$f")
        if [ "$count" -ne 2 ]; then
            echo "Issue in $f: found $count delimiters (expected 2)"
        fi
    fi
done
```

## Component File Format

Component files should follow this format:

```markdown
---
component:
  name: component_name
  category: core
  version: 1.0.0
  description: Brief description
---

# Component Content Starts Here

This content will appear in generated files.
The YAML frontmatter above will be automatically stripped.
```

**Key Rules**:
1. Frontmatter starts with `---` on its own line
2. Frontmatter ends with `---` on its own line
3. Content starts after the closing `---`
4. Blank lines between frontmatter and content are optional

## Automated Testing

Add this to your CI/CD pipeline:

```bash
#!/bin/bash
# test_no_frontmatter.sh

set -e

echo "Testing YAML frontmatter stripping..."

# Generate test file
./target/release/xzagentz create --output test_output.md

# Check for YAML frontmatter
if grep -q "^component:" test_output.md; then
    echo "❌ FAIL: YAML frontmatter found in output"
    exit 1
fi

if head -1 test_output.md | grep -q "^---$"; then
    echo "❌ FAIL: YAML delimiter found at start of file"
    exit 1
fi

echo "✓ PASS: No YAML frontmatter in output"
rm test_output.md
```

## Related Documentation

- Implementation: `docs/explanation/yaml_frontmatter_stripping.md`
- Component System: `docs/explanation/component_system.md`
- Interactive Mode: `docs/how_to/use_interactive_mode.md`

## Quick Success Checklist

- [ ] Rebuilt binary after upgrade
- [ ] Generated test file starts with markdown heading
- [ ] No `component:` lines in output
- [ ] No YAML delimiters at file start
- [ ] Content is complete and correctly formatted
- [ ] All quality checks pass (`cargo test`)

---

**Last Updated**: 2024
**Status**: Active
