# Clean Command

## Problem
Wiki accumulates technical debt: broken links, orphan tags, empty stubs.

## Approach
- **Broken links**: Wikilinks pointing to non-existent files
- **Orphan tags**: Tags with zero documents
- **Empty stubs**: Stub documents with no body content
- **Orphan docs**: Documents with no incoming links

**Actions**: Report only (default), or --fix to auto-remove

## Acceptance Criteria
- [ ] Detect broken wikilinks (target file missing)
- [ ] Detect orphan tags
- [ ] Detect empty stubs
- [ ] Detect orphan documents (no incoming links)
- [ ] --dry-run shows what would be changed
- [ ] --fix removes orphan tags, empty stubs
- [ ] Report format: human-readable, JSON with --json

## Out of Scope
- Auto-fixing broken links (requires AI to determine correct target)
- Auto-deleting orphan documents (user decision)
- Auto-merging duplicates
