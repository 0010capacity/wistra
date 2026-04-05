# Serve UI Redesign — Design Spec

## Overview

Redesign the `wistra serve` web UI from a basic single-column layout to a 3-panel wiki browser with Warm Stone color theme, Lucide icons, KaTeX math rendering, and mobile-responsive bottom sheets.

## Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Layout | 3-panel (sidebar + main + outline) | Best for large-scale wiki navigation and long-form reading |
| Color theme | Warm Stone (amber/beige) | Notion/Wikipedia feel — warm, readable, scholarly |
| Icons | Lucide (inline SVG) | Clean, consistent, lightweight — no emoji |
| Font | System font stack | Fast, no CDN dependency, Korean-friendly (Apple SD Gothic / Pretendard / Segoe UI) |
| Math | KaTeX via CDN | Already implemented, preserved as-is |
| Graph | vis-network (existing) | Keep current library, update colors to match theme |
| Mobile | Bottom sheets for panels | 3-panel doesn't fit on mobile; ☰ / list-icon toggle bottom sheets |

## Color Tokens

### Light Mode

| Token | Value | Usage |
|---|---|---|
| `--bg` | `#fafaf9` | Page background |
| `--fg` | `#1c1917` | Primary text |
| `--muted` | `#78716c` | Secondary text, labels |
| `--faint` | `#a8a29e` | Tertiary text, timestamps |
| `--accent` | `#b45309` | Links, interactive elements |
| `--accent-hover` | `#92400e` | Link hover |
| `--accent-light` | `#fef3c7` | Accent background, active nav |
| `--border` | `#e7e5e4` | Borders, dividers |
| `--card` | `#ffffff` | Card/panel background |
| `--surface` | `#f5f5f4` | Subtle background (code, table headers) |
| `--sidebar-bg` | `#fafaf9` | Sidebar background |
| `--topbar-bg` | `#1c1917` | Top bar background |
| `--topbar-fg` | `#fafaf9` | Top bar text |

### Dark Mode (`prefers-color-scheme: dark`)

| Token | Value |
|---|---|
| `--bg` | `#1c1917` |
| `--fg` | `#fafaf9` |
| `--muted` | `#a8a29e` |
| `--faint` | `#78716c` |
| `--accent` | `#d97706` |
| `--accent-hover` | `#f59e0b` |
| `--accent-light` | `#451a03` |
| `--border` | `#292524` |
| `--card` | `#292524` |
| `--surface` | `#1c1917` |
| `--sidebar-bg` | `#1c1917` |
| `--topbar-bg` | `#0c0a09` |
| `--topbar-fg` | `#fafaf9` |

### Tag Badge Colors (Light / Dark)

| Category | Background (Light) | Text (Light) | Background (Dark) | Text (Dark) |
|---|---|---|---|---|
| cs/* | `#fef3c7` | `#92400e` | `#451a03` | `#fbbf24` |
| math* | `#e0e7ff` | `#3730a3` | `#1e1b4b` | `#a5b4fc` |
| philosophy* | `#fce7f3` | `#9d174d` | `#500724` | `#f9a8d4` |
| history* | `#d1fae5` | `#065f46` | `#022c22` | `#6ee7b7` |
| physics* | `#fee2e2` | `#991b1b` | `#450a0a` | `#fca5a5` |
| Default | `#f5f5f4` | `#44403c` | `#292524` | `#d6d3d1` |

### Status Badge Colors (Light / Dark)

| Status | Background (Light) | Text (Light) | Background (Dark) | Text (Dark) |
|---|---|---|---|---|
| published | `#d1fae5` | `#065f46` | `#022c22` | `#6ee7b7` |
| stub | `#fef3c7` | `#92400e` | `#451a03` | `#fbbf24` |
| disambiguation | `#e0e7ff` | `#3730a3` | `#1e1b4b` | `#a5b4fc` |
| meta | `#f5f5f4` | `#78716c` | `#292524` | `#d6d3d1` |

## HTML Foundation

The `<html>` tag uses `lang="ko"`. All text truncation (summaries, snippets) uses Rust's `char` boundary-aware truncation to avoid splitting multi-byte UTF-8 characters.

## Layout

### Desktop (>= 1024px)

```
+------------------------------------------------------------------+
| [book] Wistra  |  [search icon] 검색...              [moon icon] |
+----------+---------------------------------------+---------------+
|          |                                       |               |
| SIDEBAR  |          MAIN CONTENT                 |    OUTLINE    |
| 220px    |          flex:1                       |    200px      |
|          |                                       |               |
| 탐색      |  h1: 머신러닝                          | 목차           |
|  홈       |  status | date | tags                 |  개요          |
|  전체     |                                       |  주요 유형      |
|  태그     |  [callout] 한줄 요약                    |  관련 문서      |
|  그래프   |                                       |               |
|          |  ## 개요                               | 백링크          |
| 최근 생성  |  본문...                               |  ← 인공지능     |
|  머신러닝  |                                       |  ← 딥러닝       |
|  신경망   |  ## 주요 유형                           |               |
|          |  [card grid]                           | 별칭            |
| 태그      |                                       |  ML, 기계학습   |
|  cs/ai    |  $$수식$$                              |               |
|  math     |                                       |               |
|          |  관련 문서                              |               |
+----------+---------------------------------------+---------------+
```

- Sidebar: fixed position, scrollable independently
- Main: scrollable, max-width 720px centered
- Outline: fixed position, scrollable independently
- Main has horizontal scroll boundary; sidebar/outline stay visible

### Graph Page Layout

The `/graph` page uses a **full-width layout** — sidebar and outline panels are hidden. The dark canvas fills the entire width below the top bar for maximum visualization space.

### Tablet (768px–1023px)

- Outline panel hidden by default
- Toggle button in top bar to show outline as overlay
- Sidebar remains visible (narrower: 180px)

### Mobile (< 768px)

- Both sidebar and outline hidden
- Top bar shows two toggle buttons: sidebar (☰) and outline (list icon)
- Each opens as a **bottom sheet** (slides up from bottom, 70% viewport height)
- Bottom sheet has drag handle, tap outside to dismiss
- Main content fills full width

## Pages

### Home (/)

**Main content:**
- Stats row: 4 cards (전체 문서, Published, Stub, 태그) with counts
- Two columns below:
  - Left: "최근 추가" — last 5 documents sorted by created date descending, with date and status badge
  - Right: "랜덤 탐색" — random document suggestion with summary (styled as amber callout card)

**Right panel:**
- 빠른 링크: Stub 목록, 고아 문서, 중복 — with counts and icons
- 활동: Recent daily activity log (date + count)

**Sidebar:** The "최근 방문" section is actually "최근 생성" — documents sorted by `created` date descending (server-side). No localStorage or cookie-based visit tracking.

### All Pages (/all)

- Filter bar: 상태 dropdown, 태그 dropdown, 검색 input
- **Filtering is server-side** — selecting a status or tag reloads the page with query parameters (`?status=stub&tag=math&q=검색어`). No client-side JavaScript filtering.
- View toggle: list (table) / grid (cards) — grid is active by default. Toggle via query param `?view=grid|list`.
- Table view: columns = 문서, 상태, 태그, 생성일
- Grid view: cards with title, status, tags, date, summary snippet
- Sortable by title, date, status (via query param `?sort=title|date|status&order=asc|desc`)

### Page View (/page/:title)

- Main: full document rendering (markdown → HTML with wikilinks, KaTeX)
- Left sidebar: unchanged
- Right panel:
  - 목차 (Table of Contents): auto-generated from h2/h3 headings via `extract_headings()` (see Data Flow section), scroll-spy highlight
  - 백링크: list of linking documents with arrow icon
  - 별칭: aliases if present

### Search (/search?q=...)

- Search bar at top (auto-focused, with query highlighted)
- Result count with match type breakdown
- Results: title (link), status badge, match type badge, content snippet (UTF-8 safe truncation at 120 chars), tags

### Tags (/tags)

- Tag cloud: sized by document count, colored by category
- Click tag → filtered document list

### Tag Page (/tag/:name)

- Documents filtered by tag
- Same list/card layout as All Pages

### Graph (/graph)

- **Full-width layout** — no sidebar, no outline panel. Dark canvas fills entire width below top bar.
- Background: `#1c1917`
- vis-network with Warm Stone themed colors:
  - Nodes: amber (#d97706) for current, stone (#292524) for connected
  - Edges: muted stone (#44403c)
- Legend in bottom-right corner
- Click node → navigate to page

## Icon Set (Lucide)

All icons are inline SVG from Lucide. No external CDN or font files.

| Element | Icon (Lucide name) |
|---|---|
| Logo | `book-open` (stroke: accent color) |
| Home | `home` |
| All pages | `file-text` |
| Tags | `tag` |
| Graph | `share-2` |
| Search | `search` |
| Dark mode | `moon` / `sun` |
| Recent | `clock` |
| Link (outgoing) | `arrow-right` |
| Link (back) | `arrow-left` |
| Filter | `filter` |
| List view | `list` |
| Grid view | `layout-grid` |
| Menu (mobile) | `menu` |
| Outline (mobile) | `list-ordered` |
| External link | `external-link` |
| Info/tip | `lightbulb` |
| Warning | `alert-triangle` |
| Close | `x` |

## Typography

```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Pretendard',
             'Apple SD Gothic Neo', Roboto, Oxygen, Ubuntu, sans-serif;
```

| Element | Size | Weight | Color |
|---|---|---|---|
| Page title (h1) | 28px | 700 | `--fg` |
| Section heading (h2) | 20px | 600 | `--fg` |
| Subsection (h3) | 16px | 600 | `--fg` |
| Body | 15px | 400 | `--fg` (light) / `--muted` (dark secondary) |
| Nav items | 13px | 400 | `--fg` |
| Section labels | 10px | 600 | `--muted`, uppercase, letter-spacing: 0.05em |
| Tags/badges | 10-11px | 400 | per-badge colors |
| Timestamps | 12px | 400 | `--faint` |

## Component Specs

### Top Bar
- Height: 44px
- Background: `--topbar-bg`
- Content: logo + name, search input, dark mode toggle
- Search input: `background: --surface`, `color: --faint`, `border-radius: 6px`
- Sticky on scroll

### Sidebar
- Width: 220px (desktop), 180px (tablet)
- Sections: 탐색, 최근 방문, 태그
- Active nav item: `background: --accent-light`, `color: --accent-hover`
- Section labels: uppercase, small, muted color
- Border-right: `1px solid --border`
- Scrollable independently (`overflow-y: auto`, `position: sticky`, `top: 44px`, `height: calc(100vh - 44px)`)

### Outline Panel
- Width: 200px (desktop only)
- Sections: 목차, 백링크, 별칭
- TOC items: `border-left: 2px solid transparent`, active: `border-color: --accent`
- Border-left: `1px solid --border`
- Scrollable independently (same sticky behavior as sidebar)

### Callout (한줄 요약)
- Data source: auto-extracted from first non-empty paragraph of document content (stripped of markdown formatting, truncated to 200 chars). Falls back to document title + " — 상세 내용을 확인하세요." if first paragraph is empty.
- Background: `#fffbeb` (light) / `#451a03` (dark)
- Border-left: `3px solid --accent`
- Padding: 12px 16px
- Border-radius: 0 6px 6px 0
- Icon: `lightbulb`

### Card Grid
- Grid: `repeat(auto-fit, minmax(200px, 1fr))`
- Card: `background: --card`, `border: 1px solid --border`, `border-radius: 8px`, `padding: 12px`
- Hover: subtle shadow

### Mobile Bottom Sheet
- Height: 70vh
- Background: `--bg`
- Border-top: `2px solid --accent`
- Border-radius: 16px 16px 0 0
- Drag handle: 30px wide, 3px height, `background: --faint`, centered, 10px from top
- Backdrop: semi-transparent overlay, tap to dismiss

## Data Flow

### Data Structures

The following structs are passed from handlers to templates. Add these fields to existing types in `src/serve/mod.rs`:

```rust
// Extended document info for templates (used in home, all-pages, search, page)
struct DocumentInfo {
    title: String,
    status: String,
    tags: Vec<String>,
    created: String,        // YYYY-MM-DD
    summary: String,        // First paragraph, stripped, max 200 chars (UTF-8 safe)
    aliases: Vec<String>,
    backlinks: Vec<String>,
}

// Heading extracted for TOC
struct Heading {
    level: u8,              // 2 or 3
    id: String,             // Slug for anchor: "개요" → "개요"
    text: String,           // Heading text
}

// Search result with match metadata
struct SearchResultInfo {
    doc: DocumentInfo,
    match_type: String,     // "title" | "content" | "tag" | "alias"
    snippet: String,        // Context around match, max 120 chars (UTF-8 safe)
}
```

### UTF-8 Safe Truncation

All text truncation (summary, snippet) must be UTF-8 boundary safe. Use the existing `char_indices()` pattern:

```rust
fn truncate_utf8(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let boundary = s.char_indices()
            .nth(max_chars)
            .map(|(i, _)| i)
            .unwrap_or(s.len());
        format!("{}...", &s[..boundary])
    }
}
```

### TOC Generation

`extract_headings()` function added to `renderer.rs`:

```rust
pub fn extract_headings(html: &str) -> Vec<Heading>
```

- Parses rendered HTML for `<h2>` and `<h3>` tags
- Generates slug ID from heading text (Korean text used as-is, whitespace replaced with `-`)
- Adds `id` attribute to heading tags in the rendered HTML
- Returns ordered list of `Heading` structs

### Callout Summary Extraction

`extract_summary()` function added to `renderer.rs`:

```rust
pub fn extract_summary(content: &str, title: &str) -> String
```

- Takes raw markdown content and document title
- Extracts first non-empty paragraph (ignoring headings, math blocks, code blocks)
- Strips markdown formatting (wikilinks, bold, italic)
- Truncates to 200 chars using `truncate_utf8()`
- Falls back to `"{title} — 상세 내용을 확인하세요."` if no paragraph found

### Graph Node Labels

vis-network node labels use `truncate_utf8()` with max 8 chars. Korean text counts as single characters. Example: "머신러닝" = 4 chars (fits), "조건부 확률" = 6 chars (fits), "비선형동역학이론" = 8 chars (fits), "비선형동역학이론기초" = 10 chars → truncated to "비선형동역학이론...".

### Filtering (All Pages)

All filtering on `/all` page is **server-side** via query parameters:

| Parameter | Values | Default |
|---|---|---|
| `status` | `published`, `stub`, `disambiguation`, `meta`, (empty = all) | all |
| `tag` | any tag name, (empty = all) | all |
| `q` | search string, matches title | (empty) |
| `view` | `grid`, `list` | `grid` |
| `sort` | `title`, `date`, `status` | `date` |
| `order` | `asc`, `desc` | `desc` |

Navigation generates full page reloads with updated query parameters. No client-side JavaScript filtering.

### Recent Documents ("최근 추가")

The sidebar section "최근 방문" is actually "최근 생성" (recently created). It shows the 5 most recently created documents sorted by `created` date descending. This is **server-side only** — no localStorage or client-side tracking.

## JavaScript Boundary

The UI is server-rendered HTML, but minimal inline `<script>` is allowed for three specific interactions:

### 1. Mobile Bottom Sheet Toggle (~30 lines)

```javascript
function toggleSheet(id) {
    var sheet = document.getElementById(id);
    var backdrop = document.getElementById(id + '-backdrop');
    sheet.classList.toggle('active');
    backdrop.classList.toggle('active');
}
```

- Two bottom sheets: `sidebar-sheet` and `outline-sheet`
- Toggle buttons in mobile top bar call `toggleSheet()`
- Backdrop tap dismisses the sheet
- CSS classes handle visibility via `transform: translateY(100%)` → `translateY(0)`

### 2. Scroll-Spy for TOC (~20 lines)

```javascript
// Only on page view. IntersectionObserver highlights active heading in outline.
var observer = new IntersectionObserver(function(entries) { ... });
document.querySelectorAll('h2, h3').forEach(function(h) { observer.observe(h); });
```

- Observes h2/h3 elements with IDs
- Updates `border-left` color on active TOC item in outline panel
- Only initialized on `/page/:title` views

### 3. All Pages View Toggle (~10 lines)

```javascript
function updateQueryParam(key, value) {
    var url = new URL(window.location.href);
    url.searchParams.set(key, value);
    return url.toString();
}

function setView(view) {
    window.location.href = updateQueryParam('view', view);
}
```

- `updateQueryParam()` is a utility used by `setView()` and filter dropdowns
- Simply redirects with updated query parameter
- Server renders the appropriate layout

### Constraints

- All JS is inline in `base_template`, wrapped in a single `<script>` tag
- No external JS files (except KaTeX CDN and vis-network CDN)
- No client-side state management
- No fetch() / AJAX calls

## File Changes

### Modified Files

| File | Changes |
|---|---|
| `src/serve/templates.rs` | Rewrite all template functions: base_template (3-panel + responsive + icons + inline JS + `<html lang="ko">`), home_template, page_template, all_pages_template, tags_template, tag_page_template, search_results_template, graph_template (full-width layout), not_found_template |
| `src/serve/renderer.rs` | Add `extract_headings(html) -> Vec<Heading>` and `extract_summary(content) -> String` and `truncate_utf8(s, max_chars) -> String`. Modify `render_markdown()` to inject `id` attributes on h2/h3 tags. |
| `src/serve/mod.rs` | Add `DocumentInfo`, `Heading`, `SearchResultInfo` structs. Add `AllPagesQuery` struct for parsed query parameters (`status: Option<String>`, `tag: Option<String>`, `q: Option<String>`, `view: String`, `sort: String`, `order: String`). Update page handler to pass TOC data and summary. Update all-pages handler to parse query params and filter. Update home handler to pass recent docs with summaries. |

### No New Files

All changes are inline in existing files. Icons are inline SVG strings, no external assets.

## Out of Scope

- JavaScript framework (React, Vue, etc.) — pure server-rendered HTML
- Client-side routing — all navigation is full page loads
- Authentication
- Image/file upload
- Live reload / WebSocket
- Theme toggle UI (dark/light) — use `prefers-color-scheme` media query only
- Collapsible sidebar (future enhancement)
- Full-text search index (keep current in-memory scan)
- Client-side JavaScript framework or complex JS — only minimal inline scripts as defined in JavaScript Boundary
- localStorage or client-side state — all data is server-rendered
