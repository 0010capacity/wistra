# Dedup Command

## Problem
Wiki can accumulate duplicate or very similar documents over time through organic growth or imports.

## Approach
- **Similar titles**: Fuzzy match on titles (ignore case, parens, punctuation)
- **Duplicate aliases**: Find documents sharing exact alias values
- **Content similarity**: Future extension (not in v1)

**Output**: Human-readable report or JSON for scripting

## Acceptance Criteria
- [ ] Detect similar titles (Levenshtein distance or fuzzy match)
- [ ] Detect documents with duplicate aliases
- [ ] Report orphaned stubs that reference same concept
- [ ] Output formats: text (default), JSON
- [ ] Dry-run only (no auto-actions)

## Out of Scope
- Automatic merging (use `merge` command manually)
- Content-based similarity detection (future)
- Auto-fixing
