# Serve Command

## Problem
Users want to browse their wistra wiki without Obsidian or external tools.

## Approach
- HTTP server serving rendered HTML
- On-the-fly markdown → HTML conversion
- Wikilink resolution to local HTML links
- Local search index
- Simple, responsive CSS

**Stack**: Single binary, no external dependencies (use pulldown-cmark markup)

## Acceptance Criteria
- [ ] Serve on configurable port (default: 15432)
- [ ] Render markdown with syntax highlighting
- [ ] Resolve wikilinks to HTML pages
- [ ] Render frontmatter as metadata
- [ ] Basic full-text search
- [ ] Responsive design
- [ ] --host to bind to specific address

## Out of Scope
- Full-text search indexing (simple in-memory)
- Authentication/authorization
- Image/file hosting
- Plugin system
- Live reload

## Open Questions
- Use existing crate (pulldown-cmark) or embed simple renderer?
- Single-threaded or multi-threaded server?
