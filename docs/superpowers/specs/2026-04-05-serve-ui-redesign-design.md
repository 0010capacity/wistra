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

### Tag Badge Colors

| Category | Background | Text |
|---|---|---|
| cs/* | `#fef3c7` | `#92400e` |
| math* | `#e0e7ff` | `#3730a3` |
| philosophy* | `#fce7f3` | `#9d174d` |
| history* | `#d1fae5` | `#065f46` |
| physics* | `#fee2e2` | `#991b1b` |
| Default | `#f5f5f4` | `#44403c` |

### Status Badge Colors

| Status | Background | Text |
|---|---|---|
| published | `#d1fae5` | `#065f46` |
| stub | `#fef3c7` | `#92400e` |
| disambiguation | `#e0e7ff` | `#3730a3` |
| meta | `#f5f5f4` | `#78716c` |

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
| 최근 방문  |  본문...                               |  ← 인공지능     |
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
  - Left: "최근 추가" — last 5 documents with date and status badge
  - Right: "랜덤 탐색" — random document suggestion with summary (styled as amber callout card)

**Right panel:**
- 빠른 링크: Stub 목록, 고아 문서, 중복 — with counts and icons
- 활동: Recent daily activity log (date + count)

### All Pages (/all)

- Filter bar: 상태 dropdown, 태그 dropdown, 검색 input
- View toggle: list (table) / grid (cards) — grid is active by default
- Table view: columns = 문서, 상태, 태그, 생성일
- Grid view: cards with title, status, tags, date, summary snippet
- Sortable by title, date, status

### Page View (/page/:title)

- Main: full document rendering (markdown → HTML with wikilinks, KaTeX)
- Left sidebar: unchanged
- Right panel:
  - 목차 (Table of Contents): auto-generated from h2/h3 headings, scroll-spy highlight
  - 백링크: list of linking documents with arrow icon
  - 별칭: aliases if present

### Search (/search?q=...)

- Search bar at top (auto-focused, with query highlighted)
- Result count with match type breakdown
- Results: title (link), status badge, match type badge, content snippet, tags

### Tags (/tags)

- Tag cloud: sized by document count, colored by category
- Click tag → filtered document list

### Tag Page (/tag/:name)

- Documents filtered by tag
- Same list/card layout as All Pages

### Graph (/graph)

- Full-width dark background (#1c1917)
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

## File Changes

### Modified Files

| File | Changes |
|---|---|
| `src/serve/templates.rs` | Rewrite all template functions: base_template (3-panel + responsive + icons), home_template, page_template, all_pages_template, tags_template, tag_page_template, search_results_template, graph_template, not_found_template |
| `src/serve/renderer.rs` | No changes needed (LaTeX protection already works) |
| `src/serve/mod.rs` | Add outline data (TOC headings) to page handler response |

### No New Files

All changes are inline in `templates.rs`. Icons are inline SVG strings, no external assets.

## Out of Scope

- JavaScript framework (React, Vue, etc.) — pure server-rendered HTML
- Client-side routing — all navigation is full page loads
- Authentication
- Image/file upload
- Live reload / WebSocket
- Theme toggle UI (dark/light) — use `prefers-color-scheme` media query only
- Collapsible sidebar (future enhancement)
- Full-text search index (keep current in-memory scan)
