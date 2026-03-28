# Wiki Format Specification

## Document Format

### File Conventions

- **Filename**: English title, spaces allowed, `.md` extension
  - `Artificial Intelligence.md`
  - `Apple (Fruit).md`
  - `Apple (Company).md`
- **Location**: all concept documents live in `concepts/`
- **Encoding**: UTF-8, LF line endings

### YAML Frontmatter — Required Fields

Every document wistra generates **must** contain these fields:

```yaml
---
title: "Artificial Intelligence"
aliases:
  - AI
  - 인공지능
tags:
  - computer-science/ai
  - science/cognitive
status: published
language: ko
created: 2026-03-26
---
```

| Field | Required | Description |
|-------|----------|-------------|
| `title` | ✅ | Canonical English title |
| `aliases` | ✅ | Alternate names; includes native-language equivalents |
| `tags` | ✅ | Hierarchical, slash-delimited |
| `status` | ✅ | `stub` / `published` / `disambiguation` |
| `language` | ✅ | ISO 639-1 code of the document body |
| `created` | ✅ | ISO 8601 date |

### Status Types

- `stub`: Placeholder document, pending generation
- `published`: Fully written document
- `disambiguation`: Hub document for ambiguous titles
- `meta`: wistra-managed meta documents (tag-index, stubs, etc.)

### Tag System

Tags are:
- **Hierarchical**: slash-delimited, maximum 3 levels deep
- **Lowercase, hyphenated**: `computer-science/ai`
- **Noun phrases only**: `programming/languages`

### Document Body Structure

wistra does not enforce a rigid template, but uses:

- H2 sections: Overview, Background/History, Key Concepts, See Also
- `[[wikilinks]]` for cross-references
- LaTeX for math: `$inline$`, `$$block$$`
- Fenced code blocks with language tags
- Blockquotes for citations
- Obsidian callouts: `> [!note]`, `> [!warning]`
