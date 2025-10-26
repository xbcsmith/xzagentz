# Changelog

All notable changes to xzagentz will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2024-12-19

### Added

#### Component System v2
- YAML frontmatter for structured component metadata
- Language-specific section markers for multi-language support
- Category-based component organization (core, general, languages, tools)
- Tier system for tool components (essential vs comprehensive)
- Size enforcement with per-category limits
- Project-wide size limit of 10,000 lines

#### Parser Infrastructure
- YAML frontmatter parser with full schema validation
- Language marker parser for extracting language-specific sections
- Component metadata models with type-safe field validation
- Support for required and optional frontmatter fields
- Validation of section definitions and language lists

#### Validation System
- Programmatic size validation per category and tier
- Warning thresholds at 80% of size limits
- Frontmatter completeness validation
- Language marker balance validation
- Size reporting with detailed statistics
- Batch validation for multiple components

#### Rendering Engine
- Language-specific content rendering
- Universal content fallback mechanism
- Strict language matching with configurable fallback
- Support for language markers in content sections
- Metadata inclusion options

#### CI/CD Integration
- GitHub Actions workflow for component validation
- Automated frontmatter validation on PRs
- Automated marker balance checking
- Size limit enforcement in CI
- Validation report generation and artifact upload
- Pre-commit validation support

#### Documentation
- Complete authoring guide for component creators
- Migration guide from legacy to v2 format
- Configuration reference documentation
- Troubleshooting guide with common issues
- Implementation plan and architecture docs
- Phase summaries for all development phases
- Example components for each category and tier

#### Examples
- Core component example (code review practices)
- Tool essential tier example (Docker basics)
- Tool comprehensive tier example (Docker advanced)
- Reference implementations for all patterns

### Changed

#### Component Format
- Migrated from plain markdown to YAML frontmatter + markdown
- Replaced implicit language sections with explicit markers
- Added structured section definitions
- Introduced version tracking per component
- Added tier field for tool components

#### Size Constraints
- Implemented hard limits per category:
  - core: 500 lines (was unlimited)
  - general: 800 lines (was unlimited)
  - languages: 600 lines (was unlimited)
  - tools (essential): 300 lines (new)
  - tools (comprehensive): 800 lines (new)
- Total project limit: 10,000 lines (new)
- Warning thresholds at 80% of limits (new)

#### Error Handling
- Added `SizeExceeded` error variant with detailed context
- Improved error messages with line numbers and paths
- Added validation context to all parser errors
- Enhanced error propagation with thiserror

#### Testing
- Expanded test coverage to >80% (was ~60%)
- Added size validation integration tests
- Added frontmatter parsing tests
- Added marker validation tests
- Added end-to-end rendering tests
- Total test count: 369 tests (was ~200)

### Fixed

- Component size now correctly excludes YAML frontmatter from line count
- Language markers now properly support all whitespace variations
- Frontmatter parsing handles edge cases (empty arrays, missing fields)
- Validation errors now provide file path context
- Size validation correctly handles tier-based limits for tools

### Deprecated

- Legacy component format without YAML frontmatter (will be removed in v3.0.0)
- Implicit language sections without markers (will be removed in v3.0.0)
- Components without version field (will be required in v3.0.0)

### Removed

- None (v2.0.0 maintains backward compatibility during transition)

### Security

- Added input validation for all component fields
- Sanitized file paths in validation errors
- Limited component size to prevent memory exhaustion
- Validated YAML parsing to prevent injection attacks

### Performance

- Optimized parser to handle large component files efficiently
- Lazy loading of component content where possible
- Efficient line counting algorithm avoiding full file reads
- Batch validation reduces I/O overhead
- Incremental validation skips unchanged components

## [1.0.0] - 2024-11-01

### Added
- Initial release with basic component management
- Simple markdown-based components
- Basic CLI interface
- Component rendering support

## Component System Implementation Phases

### Phase 1: Foundation and Parser (Week 1)
- Implemented YAML frontmatter parser
- Created language marker parser
- Defined component schema and models
- Added basic validation infrastructure

### Phase 2: Core Component Refactoring (Week 2)
- Migrated core components to v2 format
- Added YAML frontmatter to all core components
- Implemented language sections with markers
- Validated all core components

### Phase 3: General Component Refactoring (Week 3)
- Migrated general components to v2 format
- Added language-specific sections
- Updated CI/CD component guidance
- Validated all general components

### Phase 4: Tool Component Tiering (Week 4)
- Implemented tier system (essential/comprehensive)
- Split tool components by complexity
- Created essential variants (300 line limit)
- Created comprehensive variants (800 line limit)
- Validated all tool components

### Phase 5: Size Enforcement and Validation (Week 5)
- Implemented size validator with category limits
- Added warning thresholds at 80%
- Created CI validation workflow
- Generated size validation reports
- Validated entire component library

### Phase 6: Documentation and Release (Week 6)
- Created authoring guide
- Created migration guide
- Created configuration reference
- Created troubleshooting guide
- Created example components
- Updated README with comprehensive information
- Released v2.0.0

## Migration Notes

### From v1.x to v2.0.0

1. Add YAML frontmatter to all components
2. Define component metadata (name, category, version, etc.)
3. Wrap language-specific content in markers
4. Ensure component size within category limits
5. Update references to use new component names
6. Run validation: `cargo test size_validation_test`

See [Migration Guide](docs/how_to/migrating_to_v2.md) for detailed instructions.

## Links

- [Repository](https://github.com/xbcsmith/xzagentz)
- [Issues](https://github.com/xbcsmith/xzagentz/issues)
- [Documentation](docs/)
- [Implementation Plan](docs/explanations/language_agnostic_component_system_implementation_plan.md)

---

## Version History

- **2.0.0** (2024-12-19) - Component System v2 with language-agnostic rendering
- **1.0.0** (2024-11-01) - Initial release

[2.0.0]: https://github.com/xbcsmith/xzagentz/releases/tag/v2.0.0
[1.0.0]: https://github.com/xbcsmith/xzagentz/releases/tag/v1.0.0
