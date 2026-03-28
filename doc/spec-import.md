# Import Command

## Problem
Users have existing markdown/Obsidian vaults they want to migrate to wistra.

## Approach
- Scan directory recursively for .md files
- Parse existing frontmatter (preserve if valid, generate if missing)
- Extract wikilinks from body and create stubs for missing targets
- Detect disambiguation candidates (multiple files with same title)
- Dry-run mode to preview changes

**Key decisions:**
- Files without frontmatter are treated as "foreign" but parsed for wikilinks
- Aliases auto-generated from filename and first heading
- Language inferred from frontmatter or default to config

## Acceptance Criteria
- [ ] Import single file or entire directory
- [ ] Preserve existing valid frontmatter
- [ ] Generate frontmatter for files without it
- [ ] Extract wikilinks and create stubs for missing targets
- [ ] Detect disambiguation candidates
- [ ] --dry-run mode shows preview
- [ ] Report import summary

## Out of Scope
- Content rewriting/migration
- Obsidian-specific syntax (callouts, etc.) conversion
- Automatic alias suggestion from content
