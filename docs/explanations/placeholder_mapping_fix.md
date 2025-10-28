# Placeholder Mapping Fix

## Overview

This document describes the fix for the "Placeholder not found" error that occurred when using the `xzagentz create` command with components that contain template placeholders.

## Problem

When users ran `xzagentz create --interactive --output my_custom.md`, they encountered an error:

```text
Error: Placeholder not found: Placeholder 'VERSION' not found
```

This error also occurred for other placeholders like `PROJECT_NAME`, `PRIMARY_LANGUAGE`, `PROJECT_TYPE`, `UPDATED_AT`, and `PROJECT_DESCRIPTION`.

## Root Cause

### Component Templates Use Uppercase Placeholders

Component files (e.g., `components/core/header.md`) contain uppercase placeholder references:

```markdown
# {{PROJECT_NAME}} - AI Agent Development Guidelines

**Version**: {{VERSION}}
**Last Updated**: {{UPDATED_AT}}
**Project Type**: {{PROJECT_TYPE}}
**Primary Language**: {{PRIMARY_LANGUAGE}}

## Project Overview

{{PROJECT_DESCRIPTION}}
```

### Metadata Mapping Used Lowercase Keys

The `ProjectMetadata::to_placeholder_map()` method was creating a `HashMap` with lowercase keys:

```rust
// OLD CODE (BROKEN)
pub fn to_placeholder_map(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("project_name".to_string(), self.project_name.clone());
    map.insert("language".to_string(), self.language.clone());
    map.insert("version".to_string(), self.version.clone());
    // ...
}
```

When the template renderer tried to replace `{{VERSION}}`, it looked for the key `"VERSION"` in the map, but only `"version"` existed, causing the error.

## Solution

### Add Both Lowercase and Uppercase Variants

Updated `ProjectMetadata::to_placeholder_map()` to include both lowercase and uppercase keys:

```rust
// NEW CODE (FIXED)
pub fn to_placeholder_map(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Lowercase variants (for legacy compatibility)
    map.insert("project_name".to_string(), self.project_name.clone());
    map.insert("language".to_string(), self.language.clone());
    map.insert("project_type".to_string(), self.project_type.clone());
    map.insert("version".to_string(), self.version.clone());

    // Uppercase variants (used in component templates)
    map.insert("PROJECT_NAME".to_string(), self.project_name.clone());
    map.insert("PRIMARY_LANGUAGE".to_string(), self.language.clone());
    map.insert("PROJECT_TYPE".to_string(), self.project_type.clone());
    map.insert("VERSION".to_string(), self.version.clone());

    // Description with auto-generated default
    let default_description = format!("A {} project written in {}",
        self.project_type, self.language);
    let description = self.description.as_ref().unwrap_or(&default_description);
    map.insert("description".to_string(), description.clone());
    map.insert("PROJECT_DESCRIPTION".to_string(), description.clone());

    // Author (optional)
    if let Some(author) = &self.author {
        map.insert("author".to_string(), author.clone());
        map.insert("AUTHOR".to_string(), author.clone());
    }

    // Dates
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let datetime = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    map.insert("date".to_string(), date.clone());
    map.insert("UPDATED_AT".to_string(), date);
    map.insert("CREATED_AT".to_string(), datetime);

    map
}
```

### Enhanced Features

1. **Default Description**: If user doesn't provide a description, auto-generates one:
   ```text
   "A library project written in rust"
   ```

2. **Multiple Date Formats**:
   - `{{UPDATED_AT}}` - Date only (YYYY-MM-DD)
   - `{{CREATED_AT}}` - Full datetime (YYYY-MM-DD HH:MM:SS)

3. **Backward Compatibility**: Kept lowercase keys for any existing templates that might use them

## Supported Placeholders

After this fix, the following placeholders are fully supported:

| Placeholder | Example Value | Description |
|-------------|---------------|-------------|
| `{{PROJECT_NAME}}` | `my-rust-project` | Project name (preserves case) |
| `{{PRIMARY_LANGUAGE}}` | `rust` | Programming language |
| `{{PROJECT_TYPE}}` | `library` | Type of project |
| `{{VERSION}}` | `0.1.0` | Project version |
| `{{PROJECT_DESCRIPTION}}` | `High-performance data library` | Project description |
| `{{AUTHOR}}` | `Jane Developer` | Author name (if provided) |
| `{{UPDATED_AT}}` | `2024-01-15` | Current date |
| `{{CREATED_AT}}` | `2024-01-15 14:30:00` | Current datetime |

Legacy lowercase variants are also supported:
- `{{project_name}}`, `{{language}}`, `{{version}}`, `{{description}}`, `{{author}}`, `{{date}}`

## Testing

### Updated Test

The existing test was updated to verify all placeholder variants:

```rust
#[test]
fn test_metadata_to_placeholder_map() {
    let metadata = ProjectMetadata {
        project_name: "test-project".to_string(),
        language: "rust".to_string(),
        project_type: "cli".to_string(),
        description: Some("A test project".to_string()),
        author: Some("Test Author".to_string()),
        version: "1.0.0".to_string(),
    };

    let map = metadata.to_placeholder_map();

    // Verify lowercase variants
    assert_eq!(map.get("project_name").unwrap(), "test-project");
    assert_eq!(map.get("language").unwrap(), "rust");
    assert_eq!(map.get("version").unwrap(), "1.0.0");

    // Verify uppercase variants
    assert_eq!(map.get("PROJECT_NAME").unwrap(), "test-project");
    assert_eq!(map.get("PRIMARY_LANGUAGE").unwrap(), "rust");
    assert_eq!(map.get("PROJECT_TYPE").unwrap(), "cli");
    assert_eq!(map.get("VERSION").unwrap(), "1.0.0");
    assert_eq!(map.get("PROJECT_DESCRIPTION").unwrap(), "A test project");
    assert_eq!(map.get("AUTHOR").unwrap(), "Test Author");

    // Verify date fields exist
    assert!(map.contains_key("date"));
    assert!(map.contains_key("UPDATED_AT"));
    assert!(map.contains_key("CREATED_AT"));
}
```

### Manual Testing

Verified the fix works by running:

```bash
cargo build
./target/debug/xzagentz create --interactive --output test.md
```

User selects components including `header` (which uses all placeholders), and the file is created successfully without errors.

## Files Modified

### `src/cli/create.rs`

**Function modified**: `ProjectMetadata::to_placeholder_map()`
- Added uppercase placeholder variants
- Added default description generation
- Added both date formats
- Lines changed: ~30 lines

**Test updated**: `test_metadata_to_placeholder_map()`
- Verify both lowercase and uppercase variants exist
- Lines changed: ~10 lines

## Quality Verification

All quality checks pass:

```bash
✓ cargo fmt --all                                      # All files formatted
✓ cargo check --all-targets --all-features            # 0 errors
✓ cargo clippy --all-targets --all-features -- -D warnings  # 0 warnings
✓ cargo test --all-features                           # 167 passed, 0 failed
```

## Impact

### Before Fix

Users encountered errors when:
- Using the `header` component
- Creating files with interactive mode
- Any component that used uppercase placeholders

Error message:
```text
Error: Placeholder not found: Placeholder 'VERSION' not found
```

### After Fix

Users can successfully:
- Use all components with placeholders
- Create files in interactive mode
- Have descriptions auto-generated if not provided
- Use both uppercase and lowercase placeholder variants

## Backward Compatibility

This fix is **fully backward compatible**:

1. Existing templates using lowercase placeholders continue to work
2. New templates using uppercase placeholders now work correctly
3. No breaking changes to API or behavior
4. All existing tests pass

## Related Issues

This fix resolves:
- Placeholder not found errors for VERSION
- Placeholder not found errors for PROJECT_NAME
- Placeholder not found errors for PRIMARY_LANGUAGE
- Placeholder not found errors for PROJECT_TYPE
- Placeholder not found errors for PROJECT_DESCRIPTION
- Placeholder not found errors for UPDATED_AT

## Future Improvements

Potential enhancements for placeholder system:

1. **Custom Placeholders**: Allow users to define additional placeholders
2. **Placeholder Validation**: Warn if component uses undefined placeholders
3. **Placeholder Documentation**: Auto-generate list of available placeholders
4. **Placeholder Preview**: Show placeholder values before file creation
5. **Placeholder Transformation**: Support filters like `{{PROJECT_NAME|uppercase}}`

## References

- `src/cli/create.rs` - Implementation file
- `components/core/header.md` - Example component with placeholders
- `docs/explanations/interactive_component_selection_implementation.md` - Related feature documentation
- `docs/how_to/use_interactive_mode.md` - User guide with troubleshooting

---

**Implementation Date**: 2024
**Status**: Complete and tested
**Version**: Fixed in xzagentz v0.1.0+
