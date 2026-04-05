# Serve Command

## Problem
Users want to browse their wistra wiki without Obsidian or external tools, and share it publicly on the web.

## Approach

### Local Server
- HTTP server serving rendered HTML
- On-the-fly markdown → HTML conversion
- Wikilink resolution to local HTML links
- Local search index
- Responsive CSS with warm, minimal aesthetic

### Static Export
- Pre-render all pages to static HTML at build time
- Generate hosting configuration (firebase.json, _redirects)
- Optional direct deployment to Cloudflare Pages or Firebase Hosting

**Stack**: Single binary, no external dependencies (pulldown-cmark for markup, warp for HTTP)

## Acceptance Criteria

### Local Server
- [x] Serve on configurable port (default: 15432)
- [x] Render markdown with syntax highlighting
- [x] Resolve wikilinks to HTML pages
- [x] Render frontmatter as metadata
- [x] Basic full-text search
- [x] Responsive design (mobile-friendly)
- [x] --host to bind to specific address
- [x] --open to auto-open browser

### Static Export
- [x] Export all pages to static HTML
- [x] Generate firebase.json for Firebase Hosting
- [x] Generate _redirects for Cloudflare Pages
- [x] --hosting flag for target selection (firebase, cloudflare, both)
- [x] --project flag for project name (auto-derived from wiki name)
- [x] --deploy flag for automatic deployment
- [x] Show deployment URL after export

## Pages

### Local Server Routes
- `/` — Home page with recent documents, stats, random picks
- `/all` — All documents with filtering (status, tag, search) and view modes (grid, list)
- `/tags` — Tag cloud with document counts
- `/tag/<slug>` — Documents filtered by tag
- `/graph` — Interactive knowledge graph visualization
- `/search?q=<query>` — Search results
- `/page/<slug>` — Individual document pages
- `/404` — Not found page

### Static Export Structure
```
dist/
├── index.html          # Home page
├── 404.html            # Not found page
├── all/
│   └── index.html      # All pages
├── tags/
│   └── index.html      # Tag cloud
├── tag/<slug>/
│   └── index.html      # Per-tag pages
├── graph/
│   └── index.html      # Knowledge graph
├── page/<slug>/
│   └── index.html      # Document pages
├── firebase.json       # Firebase Hosting config
└── _redirects          # Cloudflare Pages config
```

## Out of Scope
- Full-text search indexing (simple in-memory)
- Authentication/authorization
- Image/file hosting
- Plugin system
- Live reload
- Multiple wiki support
- User accounts

## Design

### Visual Style
- **Colors**: Warm stone palette (sand, terracotta, warm grays)
- **Typography**: System fonts, clean hierarchy
- **Layout**: Single-column, responsive grid
- **Icons**: Lucide icons for UI elements

### Technical
- Single-threaded async server (tokio + warp)
- Template rendering with String concatenation (no external template engine)
- CSS inlined in HTML for portability
- KaTeX for LaTeX math rendering
- Mermaid.js for graph visualization
