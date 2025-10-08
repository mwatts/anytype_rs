---
name: Markdown Import Feature
about: Add markdown file import functionality to the CLI
title: '[Feature] Markdown Import with Frontmatter Support'
labels: enhancement, cli
assignees: ''
---

## Feature Description
Add ability to import markdown files into Anytype using an authenticated CLI session. Users should be able to specify the target space and type, with frontmatter metadata automatically mapped to type properties.

## Motivation
Currently, there's no easy way to import existing markdown files into Anytype via the CLI. This feature would enable:
- Bulk migration of markdown notes from other tools
- Automated workflows for creating Anytype objects from markdown
- Integration with existing markdown-based workflows

## Proposed Solution

### Command Interface
```bash
anytype import markdown <file> --space <space-id> --type <type-key> [options]
```

**Required Arguments:**
- `<file>`: Path to markdown file to import
- `--space <space-id>`: Target space ID
- `--type <type-key>`: Type key for the new object

**Optional Flags:**
- `--dry-run`: Preview the mapping without creating the object
- `--verbose`: Show detailed mapping information

### Implementation Components

#### 1. Dependencies
Add to `Cargo.toml`:
- `gray_matter` - Frontmatter parsing with YAML/JSON/TOML support

#### 2. New CLI Module
Create `src/cli/commands/import.rs` with:
- Argument parsing structures
- File reading logic
- Frontmatter extraction
- Property mapping logic

#### 3. Frontmatter to Property Mapping
The importer should:
1. Fetch the type definition from the specified space
2. Parse frontmatter fields from the markdown file
3. Map frontmatter keys to type property keys (case-insensitive matching)
4. Handle type conversion based on property format:
   - `text` - String values
   - `number` - Numeric values
   - `date` - ISO date strings
   - `checkbox` - Boolean values
   - `select` - String values
   - `multi_select` - Array values
   - `url`, `email`, `phone` - String validation
5. Warn about unmapped frontmatter fields
6. Use `title` frontmatter field for object name (fallback to filename)

#### 4. Object Creation
Use existing `create_object` API with:
- `name`: From frontmatter `title` or filename
- `markdown`: Markdown body content (after frontmatter)
- `properties`: JSON object with mapped frontmatter values

#### 5. Error Handling
Handle common error cases:
- File not found or not readable
- Invalid markdown/frontmatter format
- Space not found
- Type not found in space
- Property type conversion errors
- API authentication/network errors

### Example Usage

**Markdown File (`note.md`):**
```markdown
---
title: My Project Notes
date: 2025-01-15
status: active
tags: [rust, cli, anytype]
priority: 3
published: true
---

# Project Overview

This is my project documentation.

## Features
- Markdown import
- Frontmatter parsing
```

**Import Command:**
```bash
anytype import markdown note.md \
  --space sp_abc123 \
  --type ot_note \
  --verbose
```

**Expected Output:**
```
📄 Reading markdown file: note.md
✓ Parsed frontmatter: 6 fields found
✓ Fetched type definition: ot_note

📋 Property Mapping:
  title → name (object name)
  date → date (Date)
  status → status (Text)
  tags → tags (MultiSelect)
  priority → priority (Number)
  published → published (Checkbox)

✓ Created object in space sp_abc123
  🆔 ID: obj_xyz789
  📝 Name: My Project Notes
  📄 Markdown: 145 characters
  🔑 Properties: 5 mapped
```

### Testing Checklist
- [ ] Parse YAML frontmatter
- [ ] Parse JSON frontmatter (optional)
- [ ] Parse TOML frontmatter (optional)
- [ ] Handle files with no frontmatter
- [ ] Map all standard property formats
- [ ] Handle type conversion errors gracefully
- [ ] Validate file path handling
- [ ] Test with various type definitions
- [ ] Dry-run mode works correctly
- [ ] Error messages are clear and actionable

### Files to Modify/Create

**New Files:**
- `src/cli/commands/import.rs`

**Modified Files:**
- `Cargo.toml` - Add gray_matter dependency
- `src/cli/main.rs` - Add Import command variant
- `src/cli/commands/mod.rs` - Export import module
- `README.md` - Document new command (optional)

### Open Questions
1. Should we support batch import (multiple files)?
   - **Decision:** Start with single file import for MVP
2. How to handle frontmatter fields that don't match any type properties?
   - **Decision:** Warn but continue, optionally store in a generic "metadata" property if available
3. Should we support custom property key mapping (e.g., `author` → `creator`)?
   - **Decision:** Future enhancement, start with direct key matching
4. How to handle arrays/nested objects in frontmatter?
   - **Decision:** Support arrays for multi_select, flatten nested objects with dot notation or warn

### Future Enhancements
- Batch import from directory
- Custom field mapping configuration
- Support for embedded images/files
- Bidirectional sync (export to markdown)
- Template-based imports
- Interactive mode for resolving mapping conflicts

## Additional Context
This feature aligns with the project's goal of providing comprehensive Anytype API coverage. The implementation should follow existing patterns in the codebase (similar to `spaces`, `types`, etc. commands).

## References
- Anytype API Documentation: Objects endpoint
- gray_matter crate: https://crates.io/crates/gray_matter
- Related: Existing `create_object` implementation in `src/api/client/objects.rs`
