/// HTML templates for the serve command

use crate::serve::renderer::{Heading, truncate_utf8};

// ---------------------------------------------------------------------------
// Lucide SVG icon constants
// ---------------------------------------------------------------------------

macro_rules! icon {
    ($name:expr, $svg:literal) => {
        concat!(
            r#"<svg class="icon" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none">"#,
            $svg,
            "</svg>"
        )
    };
}

const ICON_BOOK_OPEN: &str = icon!("book-open", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/>"#);
const ICON_HOME: &str = icon!("home", r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/>"#);
const ICON_FILE_TEXT: &str = icon!("file-text", r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><line x1="10" y1="9" x2="8" y2="9"/>"#);
const ICON_TAG: &str = icon!("tag", r#"<path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/>"#);
const ICON_SEARCH: &str = icon!("search", r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#);
const ICON_MOON: &str = icon!("moon", r#"<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>"#);
const ICON_LAYOUT_GRID: &str = icon!("layout-grid", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>"#);
const ICON_LIST: &str = icon!("list", r#"<line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/>"#);
const ICON_ARROW_RIGHT: &str = icon!("arrow-right", r#"<line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>"#);
const ICON_STAR: &str = icon!("star", r#"<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>"#);
const ICON_FILE_PLUS: &str = icon!("file-plus", r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="12" y1="18" x2="12" y2="12"/><line x1="9" y1="15" x2="15" y2="15"/>"#);
const ICON_CIRCLE: &str = icon!("circle", r#"<circle cx="12" cy="12" r="10"/>"#);

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Truncate label with ellipsis if too long
fn truncate_label(s: &str, max_len: usize) -> String {
    truncate_utf8(s, max_len)
}

/// Wrap a tag string in a badge span.
pub fn tag_badge(tag: &str) -> String {
    format!(r#"<span class="tag">{}</span>"#, tag)
}

/// Wrap a status string in a badge span.
pub fn status_badge(status: &str) -> String {
    let cls = match status.to_lowercase().as_str() {
        "published" => "badge badge-published",
        "stub" => "badge badge-stub",
        "disambiguation" => "badge badge-disambiguation",
        _ => "badge",
    };
    format!(r#"<span class="{}">{}</span>"#, cls, status)
}

// ---------------------------------------------------------------------------
// Base template — 3-panel layout
// ---------------------------------------------------------------------------

/// Base HTML template with Warm Stone theme and 3-panel layout.
///
/// Layout:
///   [topbar] — full width, dark bg
///   [sidebar 220px] | [main flex-1 max-720px] | [outline 200px]
/// Mobile: sidebar/outline become fixed bottom sheets behind a toggle.
pub fn base_template(
    title: &str,
    active_nav: &str,
    sidebar_html: &str,
    outline_html: &str,
    main_content: &str,
    build_timestamp: Option<&str>,
) -> String {
    fn nav_item(href: &str, label: &str, icon: &str, active_nav: &str, current: &str) -> String {
        let active = if current == active_nav { " nav-link-active" } else { "" };
        format!(
            r#"<a href="{}" class="nav-link{}">{}<span>{}</span></a>"#,
            href, active, icon, label
        )
    }

    format!(
        r##"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} — Wistra</title>
    <style>
        :root {{
            --bg: #fafaf9;
            --fg: #1c1917;
            --muted: #78716c;
            --accent: #b45309;
            --accent-hover: #92400e;
            --accent-light: #fef3c7;
            --border: #e7e5e4;
            --card: #ffffff;
            --topbar-bg: #1c1917;
            --topbar-fg: #fafaf9;
            --topbar-muted: #a8a29e;
            --sidebar-bg: #fafaf9;
            --outline-bg: #fafaf9;
            --sheet-bg: #ffffff;
            --shadow: rgba(0,0,0,0.08);
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg: #1c1917;
                --fg: #fafaf9;
                --muted: #a8a29e;
                --accent: #d97706;
                --accent-hover: #fbbf24;
                --accent-light: #451a03;
                --border: #292524;
                --card: #292524;
                --topbar-bg: #0c0a09;
                --topbar-fg: #fafaf9;
                --topbar-muted: #78716c;
                --sidebar-bg: #1c1917;
                --outline-bg: #1c1917;
                --sheet-bg: #292524;
                --shadow: rgba(0,0,0,0.3);
            }}
        }}
        *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
        html {{ scroll-behavior: smooth; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: var(--fg);
            background: var(--bg);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        /* ── Topbar ── */
        .topbar {{
            position: sticky;
            top: 0;
            z-index: 50;
            width: 100%;
            height: 56px;
            background: var(--topbar-bg);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 1.5rem;
            gap: 1rem;
            border-bottom: 1px solid rgba(255,255,255,0.06);
        }}
        .topbar-logo {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            text-decoration: none;
            flex-shrink: 0;
        }}
        .topbar-logo a {{
            color: var(--topbar-fg);
            font-size: 1.125rem;
            font-weight: 700;
            letter-spacing: -0.01em;
            text-decoration: none;
        }}
        .topbar-logo a:hover {{ color: var(--accent); }}
        .topbar-icon {{ color: var(--accent); display: flex; align-items: center; }}
        .topbar-icon .icon {{ width: 22px; height: 22px; }}
        .search-wrap {{
            flex: 1;
            max-width: 400px;
        }}
        .search-wrap form {{
            display: flex;
            gap: 0.375rem;
        }}
        .search-wrap input {{
            flex: 1;
            height: 34px;
            padding: 0 0.75rem;
            border-radius: 6px;
            border: 1px solid rgba(255,255,255,0.12);
            background: rgba(255,255,255,0.08);
            color: var(--topbar-fg);
            font-size: 0.875rem;
        }}
        .search-wrap input::placeholder {{ color: var(--topbar-muted); }}
        .search-wrap input:focus {{ outline: none; border-color: var(--accent); }}
        .topbar-actions {{
            display: flex;
            align-items: center;
            gap: 0.25rem;
            flex-shrink: 0;
        }}
        .icon-btn {{
            width: 34px;
            height: 34px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 6px;
            border: none;
            background: transparent;
            color: var(--topbar-muted);
            cursor: pointer;
            transition: background 0.15s, color 0.15s;
        }}
        .icon-btn:hover {{ background: rgba(255,255,255,0.08); color: var(--topbar-fg); }}
        .icon-btn .icon {{ width: 18px; height: 18px; }}
        /* ── 3-panel layout ── */
        .layout {{ display: grid; grid-template-columns: 220px 1fr 200px; flex: 1; }}
        /* ── Sidebar ── */
        .sidebar {{
            background: var(--sidebar-bg);
            border-right: 1px solid var(--border);
            padding: 1.25rem 0;
            position: sticky;
            top: 56px;
            height: calc(100vh - 56px);
            overflow-y: auto;
            font-size: 0.875rem;
        }}
        .sidebar-section {{ padding: 0 0.75rem; margin-bottom: 1.5rem; }}
        .sidebar-title {{
            font-size: 0.6875rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.06em;
            color: var(--muted);
            padding: 0 0.5rem;
            margin-bottom: 0.375rem;
        }}
        .nav-link {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.4375rem 0.5rem;
            border-radius: 6px;
            color: var(--muted);
            text-decoration: none;
            transition: background 0.12s, color 0.12s;
            margin-bottom: 2px;
        }}
        .nav-link:hover {{ background: var(--accent-light); color: var(--accent); }}
        .nav-link-active {{ background: var(--accent-light) !important; color: var(--accent) !important; font-weight: 600; }}
        .nav-link .icon {{ width: 15px; height: 15px; flex-shrink: 0; }}
        .nav-link span {{ line-height: 1; }}
        /* ── Main content ── */
        .main {{
            min-width: 0;
            padding: 2rem 2.5rem;
        }}
        /* ── Article ── */
        article {{
            max-width: 720px;
            margin: 0 auto;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 2rem 2.5rem;
        }}
        article h1 {{
            font-size: 1.875rem;
            font-weight: 700;
            letter-spacing: -0.02em;
            line-height: 1.2;
            margin-bottom: 0.5rem;
        }}
        .doc-meta {{
            color: var(--muted);
            font-size: 0.8125rem;
            margin-bottom: 1.5rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            gap: 0.25rem 0.5rem;
        }}
        .badge {{
            display: inline-flex;
            align-items: center;
            padding: 0.125rem 0.5rem;
            border-radius: 999px;
            font-size: 0.75rem;
            font-weight: 500;
            background: var(--accent-light);
            color: var(--accent);
        }}
        .badge-published {{ background: #dcfce7; color: #15803d; }}
        .badge-stub {{ background: #fef9c3; color: #854d0e; }}
        .badge-disambiguation {{ background: #e0f2fe; color: #0369a1; }}
        .tag {{
            display: inline-flex;
            align-items: center;
            gap: 0.25rem;
            padding: 0.125rem 0.5rem;
            border-radius: 999px;
            font-size: 0.75rem;
            background: var(--border);
            color: var(--muted);
        }}
        /* ── Outline (TOC) ── */
        .outline {{
            background: var(--outline-bg);
            border-left: 1px solid var(--border);
            padding: 1.25rem 0.75rem;
            position: sticky;
            top: 56px;
            height: calc(100vh - 56px);
            overflow-y: auto;
            font-size: 0.8125rem;
        }}
        .outline-title {{
            font-size: 0.6875rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.06em;
            color: var(--muted);
            margin-bottom: 0.5rem;
            padding: 0 0.25rem;
        }}
        .outline-list {{ list-style: none; }}
        .outline-list li {{ margin-bottom: 2px; }}
        .outline-list a {{
            display: block;
            padding: 0.25rem 0.375rem;
            border-radius: 4px;
            color: var(--muted);
            text-decoration: none;
            transition: background 0.12s, color 0.12s;
            border-left: 2px solid transparent;
        }}
        .outline-list a:hover {{ background: var(--accent-light); color: var(--accent); }}
        .outline-list .toc-h2 {{ padding-left: 0.75rem; }}
        .outline-list .toc-h3 {{ padding-left: 1.5rem; font-size: 0.75rem; }}
        .outline-list a.toc-active {{
            border-left-color: var(--accent);
            color: var(--accent);
            background: var(--accent-light);
            font-weight: 500;
        }}
        /* ── Article prose ── */
        article h2 {{
            font-size: 1.375rem;
            font-weight: 600;
            margin: 2rem 0 0.75rem;
            letter-spacing: -0.01em;
            color: var(--fg);
        }}
        article h3 {{
            font-size: 1.125rem;
            font-weight: 600;
            margin: 1.5rem 0 0.5rem;
        }}
        article h4, article h5, article h6 {{
            font-size: 1rem;
            font-weight: 600;
            margin: 1rem 0 0.375rem;
        }}
        article p {{ margin: 0.875rem 0; }}
        article ul, article ol {{ margin: 0.875rem 0; padding-left: 1.5rem; }}
        article li {{ margin: 0.25rem 0; }}
        article code {{
            background: var(--accent-light);
            color: var(--accent);
            padding: 0.125rem 0.375rem;
            border-radius: 4px;
            font-family: 'SF Mono', 'Fira Code', 'Fira Mono', Menlo, Consolas, monospace;
            font-size: 0.875em;
        }}
        article pre {{
            background: var(--card);
            border: 1px solid var(--border);
            padding: 1rem 1.25rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1rem 0;
        }}
        article pre code {{
            background: none;
            color: var(--fg);
            padding: 0;
            border-radius: 0;
            font-size: 0.875rem;
            line-height: 1.6;
        }}
        article blockquote {{
            border-left: 3px solid var(--accent);
            padding: 0.25rem 0 0.25rem 1rem;
            margin: 1rem 0;
            color: var(--muted);
            font-style: italic;
        }}
        article a {{ color: var(--accent); text-decoration: none; }}
        article a:hover {{ text-decoration: underline; }}
        article table {{ width: 100%; border-collapse: collapse; margin: 1rem 0; }}
        article th, article td {{ border: 1px solid var(--border); padding: 0.5rem 0.75rem; text-align: left; }}
        article th {{ background: var(--accent-light); font-weight: 600; }}
        article img {{ max-width: 100%; height: auto; border-radius: 6px; }}
        article hr {{ border: none; border-top: 1px solid var(--border); margin: 2rem 0; }}
        /* ── Home page stats ── */
        .stats-row {{
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 1rem;
            margin-bottom: 2rem;
        }}
        .stat-card {{
            background: var(--accent-light);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1rem;
            text-align: center;
        }}
        .stat-card .stat-num {{
            font-size: 1.75rem;
            font-weight: 700;
            color: var(--accent);
            line-height: 1;
        }}
        .stat-card .stat-label {{
            font-size: 0.75rem;
            color: var(--muted);
            margin-top: 0.25rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        /* ── Recent docs list ── */
        .doc-list {{ list-style: none; padding: 0; }}
        .doc-list li {{
            padding: 0.75rem 0;
            border-bottom: 1px solid var(--border);
        }}
        .doc-list li:last-child {{ border-bottom: none; }}
        .doc-list a {{ color: var(--accent); font-weight: 500; font-size: 1rem; }}
        .doc-list .doc-list-meta {{
            color: var(--muted);
            font-size: 0.8125rem;
            margin-top: 0.125rem;
        }}
        /* ── Document cards (grid/list view) ── */
        .cards-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
            gap: 1rem;
            margin-top: 1rem;
        }}
        .cards-list {{
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            margin-top: 1rem;
        }}
        .doc-card {{
            background: var(--bg);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1rem;
            text-decoration: none;
            display: block;
            transition: border-color 0.12s, box-shadow 0.12s;
        }}
        .doc-card:hover {{
            border-color: var(--accent);
            box-shadow: 0 2px 8px var(--shadow);
        }}
        .doc-card-title {{
            font-weight: 600;
            font-size: 0.9375rem;
            color: var(--fg);
            margin-bottom: 0.25rem;
        }}
        .doc-card-meta {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            flex-wrap: wrap;
            margin-top: 0.375rem;
        }}
        .doc-card-summary {{
            color: var(--muted);
            font-size: 0.8125rem;
            margin-top: 0.375rem;
            line-height: 1.5;
        }}
        .cards-list .doc-card {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 1rem;
            padding: 0.75rem 1rem;
        }}
        .cards-list .doc-card .doc-card-title {{ margin-bottom: 0; }}
        .cards-list .doc-card .doc-card-summary {{ display: none; }}
        /* ── Filter bar ── */
        .filter-bar {{
            display: flex;
            align-items: center;
            gap: 0.75rem;
            margin-bottom: 1rem;
            flex-wrap: wrap;
        }}
        .filter-bar select, .filter-bar input {{
            height: 36px;
            padding: 0 0.75rem;
            border-radius: 6px;
            border: 1px solid var(--border);
            background: var(--card);
            color: var(--fg);
            font-size: 0.875rem;
        }}
        .filter-bar select:focus, .filter-bar input:focus {{
            outline: none;
            border-color: var(--accent);
        }}
        .filter-bar .filter-label {{
            color: var(--muted);
            font-size: 0.8125rem;
        }}
        .view-toggle {{
            margin-left: auto;
            display: flex;
            gap: 0.25rem;
        }}
        .view-toggle button {{
            width: 36px;
            height: 36px;
            border: 1px solid var(--border);
            background: var(--card);
            border-radius: 6px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            color: var(--muted);
            transition: background 0.12s, color 0.12s;
        }}
        .view-toggle button:hover {{ background: var(--accent-light); color: var(--accent); }}
        .view-toggle button.active {{ background: var(--accent-light); color: var(--accent); border-color: var(--accent); }}
        .view-toggle button .icon {{ width: 16px; height: 16px; }}
        /* ── Callout / summary box ── */
        .callout {{
            background: var(--accent-light);
            border-left: 3px solid var(--accent);
            padding: 0.75rem 1rem;
            border-radius: 0 6px 6px 0;
            margin-bottom: 1.5rem;
            color: var(--muted);
            font-size: 0.9375rem;
        }}
        /* ── Backlinks ── */
        .backlinks {{
            margin-top: 2.5rem;
            padding-top: 1.5rem;
            border-top: 1px solid var(--border);
        }}
        .backlinks h3 {{
            font-size: 0.875rem;
            color: var(--muted);
            margin-bottom: 0.5rem;
        }}
        .backlinks-list {{ list-style: none; padding: 0; display: flex; flex-wrap: wrap; gap: 0.5rem; }}
        .backlinks-list li::after {{ content: none; }}
        .backlinks-list a {{
            background: var(--accent-light);
            color: var(--accent);
            padding: 0.125rem 0.625rem;
            border-radius: 999px;
            font-size: 0.8125rem;
            font-weight: 500;
        }}
        /* ── Tag cloud ── */
        .tag-cloud {{
            display: flex;
            flex-wrap: wrap;
            gap: 0.5rem;
            margin-top: 1rem;
        }}
        .tag-cloud a {{
            display: inline-flex;
            align-items: center;
            padding: 0.25rem 0.75rem;
            border-radius: 999px;
            border: 1px solid var(--border);
            color: var(--muted);
            font-size: 0.875rem;
            text-decoration: none;
            transition: background 0.12s, color 0.12s, border-color 0.12s;
        }}
        .tag-cloud a:hover {{ background: var(--accent-light); color: var(--accent); border-color: var(--accent); }}
        /* ── Quick links section ── */
        .quick-links {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
            gap: 0.75rem;
            margin-top: 1.5rem;
        }}
        .quick-link {{
            display: flex;
            align-items: center;
            gap: 0.625rem;
            padding: 0.75rem 1rem;
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            text-decoration: none;
            font-size: 0.875rem;
            font-weight: 500;
            transition: background 0.12s, border-color 0.12s;
        }}
        .quick-link:hover {{ background: var(--accent-light); border-color: var(--accent); color: var(--accent); }}
        .quick-link .icon {{ width: 16px; height: 16px; flex-shrink: 0; color: var(--accent); }}
        /* ── Search results ── */
        .search-bar-top {{
            display: flex;
            gap: 0.5rem;
            margin-bottom: 1.5rem;
        }}
        .search-bar-top input {{
            flex: 1;
            height: 44px;
            padding: 0 1rem;
            border-radius: 8px;
            border: 1px solid var(--border);
            background: var(--card);
            color: var(--fg);
            font-size: 1rem;
        }}
        .search-bar-top input:focus {{ outline: none; border-color: var(--accent); }}
        .search-bar-top button {{
            height: 44px;
            padding: 0 1.25rem;
            border-radius: 8px;
            border: none;
            background: var(--accent);
            color: #fff;
            font-size: 0.9375rem;
            font-weight: 600;
            cursor: pointer;
        }}
        .search-bar-top button:hover {{ background: var(--accent-hover); }}
        .result-count {{
            color: var(--muted);
            font-size: 0.875rem;
            margin-bottom: 1rem;
        }}
        /* ── Footer ── */
        footer {{
            text-align: center;
            padding: 1.5rem;
            color: var(--muted);
            font-size: 0.8125rem;
            border-top: 1px solid var(--border);
        }}
        footer a {{ color: var(--accent); }}
        /* ── Mobile ── */
        @media (max-width: 767px) {{
            .layout {{
                display: flex;
                flex-direction: column;
            }}
            .sidebar, .outline {{
                display: none;
                position: fixed;
                left: 0;
                right: 0;
                z-index: 40;
                height: 70vh;
                border-radius: 16px 16px 0 0;
                box-shadow: 0 -4px 24px var(--shadow);
                border-right: none;
                border-left: none;
            }}
            .sidebar.open {{ bottom: 0; display: block; }}
            .outline.open {{ top: auto; bottom: 0; display: block; }}
            .main {{ padding: 1rem; }}
            article {{ padding: 1.25rem; border-radius: 8px; }}
            article h1 {{ font-size: 1.5rem; }}
            .stats-row {{ grid-template-columns: repeat(2, 1fr); }}
            .cards-grid {{ grid-template-columns: 1fr; }}
            /* mobile nav bar */
            .mobile-nav {{
                display: flex;
                position: fixed;
                bottom: 0;
                left: 0;
                right: 0;
                z-index: 50;
                background: var(--topbar-bg);
                border-top: 1px solid rgba(255,255,255,0.08);
                height: 56px;
                align-items: center;
                justify-content: space-around;
                padding: 0 1rem;
            }}
            .mobile-nav-btn {{
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 2px;
                background: none;
                border: none;
                cursor: pointer;
                color: var(--topbar-muted);
                font-size: 0.6875rem;
                padding: 4px 8px;
                border-radius: 6px;
                transition: color 0.15s;
                text-decoration: none;
            }}
            .mobile-nav-btn:hover, .mobile-nav-btn.active {{ color: var(--accent); }}
            .mobile-nav-btn .icon {{ width: 20px; height: 20px; }}
            .mobile-nav-btn span {{ line-height: 1; }}
            .mobile-nav-btn.nav-home {{ margin-right: auto; }}
            /* backdrop */
            .backdrop {{
                display: none;
                position: fixed;
                inset: 0;
                z-index: 35;
                background: rgba(0,0,0,0.5);
            }}
            .backdrop.show {{ display: block; }}
        }}
        @media (min-width: 768px) {{
            .mobile-nav, .backdrop {{ display: none !important; }}
        }}
    </style>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js"
        onload="renderMathInElement(document.body, {{
            delimiters: [
                {{left: '$$', right: '$$', display: true}},
                {{left: '$', right: '$', display: false}},
                {{left: '\\\\(', right: '\\\\)', display: false}},
                {{left: '\\\\[', right: '\\\\]', display: true}}
            ],
            throwOnError: false
        }});"></script>
</head>
<body>

<!-- ── Topbar ── -->
<header class="topbar">
    <div class="topbar-logo">
        <span class="topbar-icon">{ICON_BOOK_OPEN}</span>
        <a href="/">Wistra</a>
    </div>
    <div class="search-wrap">
        <form action="/search" method="get">
            <input type="text" name="q" placeholder="Search..." autocomplete="off">
        </form>
    </div>
    <div class="topbar-actions">
        <button class="icon-btn" title="Toggle dark mode" onclick="document.documentElement.classList.toggle('dark')">
            <span class="icon">{ICON_MOON}</span>
        </button>
    </div>
</header>

<!-- ── 3-panel layout ── -->
<div class="layout">

    <!-- Sidebar -->
    <aside class="sidebar" id="sidebar">
        <div class="sidebar-section">
            <div class="sidebar-title">Navigate</div>
            {nav_home}
            {nav_all}
            {nav_tags}
            {nav_graph}
        </div>
        {sidebar_html}
    </aside>

    <!-- Main content -->
    <main class="main">
        {main_content}
    </main>

    <!-- Outline / TOC -->
    <aside class="outline" id="outline">
        {outline_html}
    </aside>

</div>

<!-- ── Mobile bottom sheet backdrop ── -->
<div class="backdrop" id="backdrop" onclick="closeSheets()"></div>

<!-- ── Mobile bottom navigation ── -->
<nav class="mobile-nav">
    <a href="/" class="mobile-nav-btn nav-home {home_active}">
        <span class="icon">{ICON_HOME}</span>
        <span>Home</span>
    </a>
    <button class="mobile-nav-btn" onclick="toggleSheet('sidebar', this)" id="sidebar-btn">
        <span class="icon">{ICON_LIST}</span>
        <span>Menu</span>
    </button>
    <button class="mobile-nav-btn" onclick="toggleSheet('outline', this)" id="outline-btn">
        <span class="icon">{ICON_LAYOUT_GRID}</span>
        <span>Contents</span>
    </button>
</nav>

<!-- ── Inline JavaScript ── -->
<script>
// Sheet toggling
function toggleSheet(id, btn) {{
    const sheet = document.getElementById(id);
    const backdrop = document.getElementById('backdrop');
    const isOpen = sheet.classList.contains('open');
    sheet.classList.remove('open');
    document.querySelectorAll('.mobile-nav-btn').forEach(b => b.classList.remove('active'));
    if (!isOpen) {{
        sheet.classList.add('open');
        backdrop.classList.add('show');
        btn.classList.add('active');
    }} else {{
        backdrop.classList.remove('show');
    }}
}}

function closeSheets() {{
    document.getElementById('sidebar').classList.remove('open');
    document.getElementById('outline').classList.remove('open');
    document.getElementById('backdrop').classList.remove('show');
    document.querySelectorAll('.mobile-nav-btn').forEach(b => b.classList.remove('active'));
}}

// TOC scroll spy
(function() {{
    const headings = document.querySelectorAll('article h2, article h3');
    if (!headings.length) return;

    const tocLinks = document.querySelectorAll('.outline-list a');
    if (!tocLinks.length) return;

    const observer = new IntersectionObserver((entries) => {{
        entries.forEach(entry => {{
            if (entry.isIntersecting) {{
                tocLinks.forEach(a => a.classList.remove('toc-active'));
                const id = entry.target.id;
                const active = document.querySelector('.outline-list a[href="#' + id + '"]');
                if (active) active.classList.add('toc-active');
            }}
        }});
    }}, {{ rootMargin: '-56px 0px -70% 0px' }});

    headings.forEach(h => {{
        if (!h.id) {{
            h.id = h.textContent.trim().toLowerCase().replace(/[^a-z0-9가-힣]+/g, '-');
        }}
        observer.observe(h);
    }});

    tocLinks.forEach(a => {{
        const href = a.getAttribute('href');
        if (href && href.startsWith('#')) {{
            const id = href.slice(1);
            const target = document.getElementById(id);
            if (!target) {{ a.style.display = 'none'; }}
        }}
    }});
}})();
</script>

<!-- ── Footer ── -->
<footer>
    {}
    <span style="margin-left:1rem;">Powered by <a href="https://github.com/wistra/wistra">Wistra</a></span>
</footer>

</body>
</html>"##,
        build_info = if let Some(ts) = build_timestamp {
            format!("<span style=\"color:var(--muted);\">Build: {}</span>", ts)
        } else {
            String::new()
        },
        title = title,
        nav_home = nav_item("/", "Home", ICON_HOME, active_nav, "home"),
        nav_all = nav_item("/all", "All Pages", ICON_FILE_TEXT, active_nav, "all"),
        nav_tags = nav_item("/tags", "Tags", ICON_TAG, active_nav, "tags"),
        nav_graph = nav_item("/graph", "Graph", ICON_LAYOUT_GRID, active_nav, "graph"),
        home_active = if active_nav == "home" { "active" } else { "" },
        sidebar_html = sidebar_html,
        outline_html = outline_html,
        main_content = main_content,
    )
}

// ---------------------------------------------------------------------------
// Page templates
// ---------------------------------------------------------------------------

// Types from mod.rs — re-exported here for convenience within templates
use crate::serve::{DocumentInfo, SearchResultInfo};

/// Home page with stats, recent docs, and quick links.
pub fn home_template(
    recent: &[&DocumentInfo],
    random: Option<&DocumentInfo>,
    total: usize,
    published: usize,
    stubs: usize,
    tag_count: usize,
    build_timestamp: Option<&str>,
) -> String {
    // ── Sidebar: recent docs ──
    let recent_sidebar: String = recent
        .iter()
        .take(8)
        .map(|doc| {
            format!(
                r#"<a href="/page/{}" class="nav-link"><span>{}</span></a>"#,
                urlencoding::encode(&doc.title),
                truncate_label(&doc.title, 28)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let sidebar_html = if recent_sidebar.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="sidebar-section">
                <div class="sidebar-title">Recent</div>
                {}
            </div>"#,
            recent_sidebar
        )
    };

    // ── Main: stats row ──
    let stats = format!(
        r#"<div class="stats-row">
            <div class="stat-card">
                <div class="stat-num">{}</div>
                <div class="stat-label">Total Docs</div>
            </div>
            <div class="stat-card">
                <div class="stat-num">{}</div>
                <div class="stat-label">Published</div>
            </div>
            <div class="stat-card">
                <div class="stat-num">{}</div>
                <div class="stat-label">Stubs</div>
            </div>
            <div class="stat-card">
                <div class="stat-num">{}</div>
                <div class="stat-label">Tags</div>
            </div>
        </div>"#,
        total, published, stubs, tag_count
    );

    // ── Recent docs list ──
    let recent_list: String = recent
        .iter()
        .map(|doc| {
            let tags_html: String = doc.tags.iter().take(3).map(|t| tag_badge(t)).collect::<Vec<_>>().join(" ");
            format!(
                r#"<li>
                    <a href="/page/{}">{}</a>
                    <div class="doc-list-meta">
                        {} &middot; {} {}
                    </div>
                </li>"#,
                urlencoding::encode(&doc.title),
                doc.title,
                status_badge(&doc.status),
                doc.created,
                tags_html
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // ── Random suggestion card ──
    let random_card = if let Some(doc) = random {
        format!(
            r#"<div class="callout" style="margin-top:1.5rem;">
                <strong>Suggested:</strong>
                <a href="/page/{}">{}</a>
                {}
            </div>"#,
            urlencoding::encode(&doc.title),
            doc.title,
            tag_badge(&doc.status)
        )
    } else {
        String::new()
    };

    // ── Quick links ──
    let quick_links = format!(
        r#"<div class="quick-links">
            <a href="/all" class="quick-link">{ICON_FILE_TEXT}<span>Browse All</span></a>
            <a href="/tags" class="quick-link">{ICON_TAG}<span>Tags</span></a>
            <a href="/graph" class="quick-link">{ICON_LAYOUT_GRID}<span>Knowledge Graph</span></a>
            <a href="/search?q=" class="quick-link">{ICON_SEARCH}<span>Search</span></a>
        </div>"#
    );

    let main_content = format!(
        r#"<article>
            <h1>Welcome to Your Wiki</h1>
            <p>Browse your personal knowledge base. {} documents available.</p>
            {}
            <h2 style="margin-top:2rem;">Recent Documents</h2>
            <ul class="doc-list">{}</ul>
            {}
            {}
        </article>"#,
        total,
        stats,
        recent_list,
        random_card,
        quick_links
    );

    base_template("Home", "home", &sidebar_html, "", &main_content, build_timestamp)
}

/// Single document page with TOC, metadata, and backlinks.
pub fn page_template(
    doc: &DocumentInfo,
    html_body: &str,
    headings: &[Heading],
) -> String {
    // ── Tags HTML ──
    let tags_html: String = doc.tags.iter().map(|t| tag_badge(t)).collect::<Vec<_>>().join(" ");

    // ── Aliases ──
    let aliases_html = if doc.aliases.is_empty() {
        String::new()
    } else {
        format!(r#"<span style="color:var(--muted);font-size:0.8125rem;">aka {}</span>"#, doc.aliases.join(", "))
    };

    // ── Callout (summary) ──
    let callout = if doc.summary.is_empty() {
        String::new()
    } else {
        format!(r#"<div class="callout">{}</div>"#, doc.summary)
    };

    // ── Backlinks ──
    let backlinks_html = if doc.backlinks.is_empty() {
        String::new()
    } else {
        let links: String = doc.backlinks
            .iter()
            .map(|t| {
                let normalized = t.replace(' ', "-");
                format!(r#"<li><a href="/page/{}">{}</a></li>"#, urlencoding::encode(&normalized), t)
            })
            .collect();
        format!(
            r##"<div class="backlinks">
                <h3>Backlinks ({})</h3>
                <ul class="backlinks-list">{}</ul>
            </div>"##,
            doc.backlinks.len(),
            links
        )
    };

    // ── Main content ──
    let main_content = format!(
        r#"<article>
            <h1>{}</h1>
            <div class="doc-meta">
                {} {}
                <span>{}</span>
                <br>
                <span>{}</span>
            </div>
            {}
            {}
            {}
        </article>"#,
        doc.title,
        status_badge(&doc.status),
        doc.created,
        aliases_html,
        tags_html,
        callout,
        html_body,
        backlinks_html
    );

    // ── Sidebar: nav links to same tag pages ──
    let sidebar_tags: String = doc.tags
        .iter()
        .map(|t| {
            format!(
                r#"<a href="/tag/{}" class="nav-link">{}<span>{}</span></a>"#,
                urlencoding::encode(t),
                ICON_TAG,
                t
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let sidebar_html = if doc.tags.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="sidebar-section">
                <div class="sidebar-title">Tags</div>
                {}
            </div>"#,
            sidebar_tags
        )
    };

    // ── Outline: TOC from headings ──
    let outline_items: String = headings
        .iter()
        .map(|h| {
            let cls = if h.level == 3 { "toc-h3" } else { "toc-h2" };
            format!(
                r#"<li><a href="{}{}" class="{}">{}</a></li>"#,
                "#", h.id, cls, h.text
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let outline_html = if headings.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="outline-title">On This Page</div>
<ul class="outline-list">{}</ul>"#,
            outline_items
        )
    };

    base_template(&doc.title, "home", &sidebar_html, &outline_html, &main_content, None)
}

/// All pages listing with filter bar and grid/list toggle.
pub fn all_pages_template(
    docs: &[DocumentInfo],
    view: &str,
    status_filter: Option<&str>,
    _tag_filter: Option<&str>,
    q_filter: Option<&str>,
) -> String {
    let is_grid = view == "grid";

    // ── Filter bar ──
    let filter_bar = format!(
        r#"<div class="filter-bar">
            <span class="filter-label">Filter:</span>
            <select onchange="updateQueryParam('status', this.value)">
                <option value="">All Status</option>
                <option value="published" {}>Published</option>
                <option value="stub" {}>Stub</option>
                <option value="disambiguation" {}>Disambiguation</option>
            </select>
            <select onchange="updateQueryParam('tag', this.value)">
                <option value="">All Tags</option>
            </select>
            <input type="text" id="search-input" placeholder="Search title..."
                value="{}"
                onkeydown="if(event.key==='Enter'){{updateQueryParam('q',document.getElementById('search-input').value)}}">
            <div class="view-toggle">
                <button class="active" onclick="setView('grid', this)" title="Grid view">
                    <span class="icon">{}</span>
                </button>
                <button onclick="setView('list', this)" title="List view">
                    <span class="icon">{}</span>
                </button>
            </div>
        </div>
        <script>
        function updateQueryParam(key, value) {{
            const url = new URL(window.location.href);
            if (value) url.searchParams.set(key, value);
            else url.searchParams.delete(key);
            window.location.href = url.toString();
        }}
        function setView(v, btn) {{
            updateQueryParam('view', v);
        }}
        // Sync active state on load
        (function() {{
            const isGrid = '{}' === 'grid';
            const btns = document.querySelectorAll('.view-toggle button');
            btns.forEach(b => b.classList.remove('active'));
            if (isGrid) btns[0].classList.add('active');
            else btns[1].classList.add('active');
        }})();
        </script>"#,
        if status_filter == Some("published") { "selected" } else { "" },
        if status_filter == Some("stub") { "selected" } else { "" },
        if status_filter == Some("disambiguation") { "selected" } else { "" },
        q_filter.unwrap_or(""),
        ICON_LAYOUT_GRID,
        ICON_LIST,
        view
    );

    // ── Document cards ──
    let card_view_class = if is_grid { "cards-grid" } else { "cards-list" };
    let cards: String = docs
        .iter()
        .map(|d| {
            let tags_html: String = d.tags.iter().take(3).map(|t| tag_badge(t)).collect::<Vec<_>>().join(" ");
            format!(
                r#"<a href="/page/{}" class="doc-card">
                    <div class="doc-card-title">{}</div>
                    <div class="doc-card-meta">
                        {} <span style="color:var(--muted);font-size:0.8125rem;">{}</span>
                        {}
                    </div>
                    <div class="doc-card-summary">{}</div>
                </a>"#,
                urlencoding::encode(&d.title),
                d.title,
                status_badge(&d.status),
                d.created,
                tags_html,
                truncate_label(&d.summary, 100)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let doc_count_note = format!(r#"<p style="color:var(--muted);font-size:0.875rem;margin-bottom:0.5rem;">{} documents</p>"#, docs.len());

    let main_content = format!(
        r#"<article>
            <h1>All Pages</h1>
            {}
            {}
            <div class="{}">{}</div>
        </article>"#,
        filter_bar,
        doc_count_note,
        card_view_class,
        cards
    );

    // Sidebar: recent docs
    let sidebar_recent: String = docs.iter().take(8).map(|d| {
        format!(
            r#"<a href="/page/{}" class="nav-link"><span>{}</span></a>"#,
            urlencoding::encode(&d.title),
            truncate_label(&d.title, 28)
        )
    }).collect::<Vec<_>>().join("\n");
    let sidebar_html = if sidebar_recent.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="sidebar-section">
                <div class="sidebar-title">Recent</div>
                {}
            </div>"#,
            sidebar_recent
        )
    };

    base_template("All Pages", "all", &sidebar_html, "", &main_content, None)
}

/// Search results page.
pub fn search_results_template(query: &str, results: &[SearchResultInfo]) -> String {
    let search_bar = format!(
        r#"<div class="search-bar-top">
            <input type="text" id="q-input" value="{}" placeholder="Search documents..." autofocus>
            <button onclick="window.location.href='/search?q='+encodeURIComponent(document.getElementById('q-input').value)">
                Search
            </button>
        </div>"#,
        query
    );

    let result_count = if results.is_empty() {
        r#"<p style="color:var(--muted);font-size:0.9375rem;">No results found. Try different keywords.</p>"#.to_string()
    } else {
        format!(
            r#"<p class="result-count">{} result{} for "{}"</p>"#,
            results.len(),
            if results.len() == 1 { "" } else { "s" },
            query
        )
    };

    let match_badge = |mt: &str| -> String {
        let (cls, label) = match mt {
            "title" => ("badge-published", "Title"),
            "content" => ("badge-stub", "Content"),
            "tag" => ("badge-disambiguation", "Tag"),
            "alias" => ("", "Alias"),
            _ => ("", mt),
        };
        if cls.is_empty() {
            format!(r#"<span>{}</span>"#, label)
        } else {
            format!(r#"<span class="badge {}">{}</span>"#, cls, label)
        }
    };

    let cards: String = results
        .iter()
        .map(|r| {
            let tags_html: String = r.tags.iter().take(3).map(|t| tag_badge(t)).collect::<Vec<_>>().join(" ");
            format!(
                r#"<a href="/page/{}" class="doc-card">
                    <div class="doc-card-title">{}</div>
                    <div class="doc-card-meta">
                        {} {}
                    </div>
                    <div class="doc-card-summary">{}</div>
                </a>"#,
                urlencoding::encode(&r.title),
                r.title,
                match_badge(&r.match_type),
                tags_html,
                truncate_label(&r.snippet, 150)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let main_content = format!(
        r#"<article>
            <h1>Search</h1>
            {}
            {}
            <div class="cards-grid">{}</div>
        </article>"#,
        search_bar,
        result_count,
        cards
    );

    base_template(&format!("Search: {}", query), "home", "", "", &main_content, None)
}

/// Tags overview page with tag cloud.
pub fn tags_template(tags: &[(String, usize)]) -> String {
    if tags.is_empty() {
        let main = r#"<article>
            <h1>Tags</h1>
            <p>No tags found.</p>
        </article>"#.to_string();
        return base_template("Tags", "tags", "", "", &main, None);
    }

    // Compute font size range based on doc count
    let max_count = tags.iter().map(|(_, c)| *c).max().unwrap_or(1);
    let tag_cloud: String = tags
        .iter()
        .map(|(tag, count)| {
            // Scale font size from 0.8125rem (min) to 1.25rem (max)
            let ratio = (*count as f64) / (max_count as f64);
            let size = 0.8125 + (ratio * 0.4375);
            format!(
                r#"<a href="/tag/{}" style="font-size:{}rem;">{} <span style="font-size:0.75rem;opacity:0.7;">({})</span></a>"#,
                urlencoding::encode(tag),
                size,
                tag,
                count
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let main_content = format!(
        r#"<article>
            <h1>Tags</h1>
            <p>Browse {} unique tags across your wiki.</p>
            <div class="tag-cloud">{}</div>
        </article>"#,
        tags.len(),
        tag_cloud
    );

    base_template("Tags", "tags", "", "", &main_content, None)
}

/// Documents filtered by a specific tag.
pub fn tag_page_template(tag: &str, docs: &[DocumentInfo]) -> String {
    let cards: String = docs
        .iter()
        .map(|d| {
            let tags_html: String = d.tags.iter().take(3).map(|t| tag_badge(t)).collect::<Vec<_>>().join(" ");
            format!(
                r#"<a href="/page/{}" class="doc-card">
                    <div class="doc-card-title">{}</div>
                    <div class="doc-card-meta">
                        {} <span style="color:var(--muted);font-size:0.8125rem;">{}</span>
                        {}
                    </div>
                    <div class="doc-card-summary">{}</div>
                </a>"#,
                urlencoding::encode(&d.title),
                d.title,
                status_badge(&d.status),
                d.created,
                tags_html,
                truncate_label(&d.summary, 100)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let count_note = format!(r#"<p style="color:var(--muted);font-size:0.875rem;margin-bottom:1rem;">{} document{}</p>"#,
        docs.len(),
        if docs.len() == 1 { "" } else { "s" }
    );

    let main_content = format!(
        r#"<article>
            <h1>Tag: {}</h1>
            {}
            <div class="cards-grid">{}</div>
        </article>"#,
        tag,
        count_note,
        cards
    );

    // Sidebar: nav + recent docs
    let sidebar_html = format!(
        r#"<div class="sidebar-section">
            <div class="sidebar-title">Tags</div>
            <a href="/tags" class="nav-link">{}<span>All Tags</span></a>
        </div>"#,
        ICON_TAG
    );

    base_template(&format!("Tag: {}", tag), "tags", &sidebar_html, "", &main_content, None)
}

/// Graph page with interactive network visualization.
pub fn graph_template(documents: &[DocumentInfo], links: &[(String, String)]) -> String {
    let nodes_json: String = documents
        .iter()
        .map(|doc| {
            format!(
                r#"{{"id": "{}", "label": "{}", "title": "{}"}}"#,
                doc.title.replace('"', "\\\"").replace('\'', "\\'"),
                truncate_label(&doc.title, 20),
                doc.title.replace('"', "\\\"")
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let edges_json: String = links
        .iter()
        .map(|(from, to)| {
            format!(
                r#"{{"from": "{}", "to": "{}"}}"#,
                from.replace('"', "\\\""),
                to.replace('"', "\\\"")
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    // Full-width graph: override layout via inline style wrapper
    let main_content = format!(
        r#"<div id="graph-wrap" style="padding:1.5rem;max-width:100%;">
            <h1 style="margin-bottom:0.5rem;">Knowledge Graph</h1>
            <p style="color:var(--muted);font-size:0.9375rem;margin-bottom:1.5rem;">
                Visualizing connections between {} documents. Click a node to open the page.
            </p>
            <div id="graph" style="width:100%;height:calc(100vh - 200px);min-height:400px;border-radius:10px;overflow:hidden;"></div>
        </div>
        <script src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
        <script>
        (function() {{
            var isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            var nodeColor = isDark ? {{ background: '#d97706', border: '#fbbf24' }} : {{ background: '#b45309', border: '#92400e' }};
            var edgeColor = isDark ? 'rgba(248,250,252,0.15)' : 'rgba(28,25,23,0.15)';
            var textColor = isDark ? '#fafaf9' : '#1c1917';

            var container = document.getElementById('graph');
            var nodes = new vis.DataSet([{nodes}]);
            var edges = new vis.DataSet([{edges}]);
            var data = {{ nodes: nodes, edges: edges }};
            var options = {{
                nodes: {{
                    shape: 'dot',
                    size: 18,
                    font: {{ size: 13, color: textColor, face: 'system-ui,sans-serif' }},
                    borderWidth: 2,
                    color: nodeColor,
                    shadow: {{ enabled: isDark, size: 8, x: 0, y: 2, color: 'rgba(0,0,0,0.4)' }}
                }},
                edges: {{
                    width: 1.5,
                    color: {{ color: edgeColor, opacity: 0.7 }},
                    arrows: {{ to: {{ enabled: true, scaleFactor: 0.5 }} }},
                    smooth: {{ type: 'continuous' }}
                }},
                physics: {{
                    stabilization: {{ iterations: 200 }},
                    barnesHut: {{
                        gravitationalConstant: -3000,
                        springConstant: 0.05,
                        springLength: 120
                    }}
                }},
                interaction: {{
                    hover: true,
                    tooltipDelay: 200,
                    zoomView: true,
                    dragView: true
                }}
            }};
            var network = new vis.Network(container, data, options);

            network.on('click', function(params) {{
                if (params.nodes.length > 0) {{
                    window.location.href = '/page/' + encodeURIComponent(params.nodes[0]);
                }}
            }});
            network.on('hoverNode', function() {{ container.style.cursor = 'pointer'; }});
            network.on('blurNode', function() {{ container.style.cursor = 'default'; }});

            // Update colors on dark mode toggle
            var observer = new MutationObserver(function(mutations) {{
                mutations.forEach(function(m) {{
                    if (m.attributeName === 'class') {{
                        var dark = document.documentElement.classList.contains('dark');
                        var nc = dark ? {{ background: '#d97706', border: '#fbbf24' }} : {{ background: '#b45309', border: '#92400e' }};
                        var ec = dark ? 'rgba(248,250,252,0.15)' : 'rgba(28,25,23,0.15)';
                        var tc = dark ? '#fafaf9' : '#1c1917';
                        nodes.forEach(function(n) {{ nodes.update({{ id: n.id, font: {{ size: 13, color: tc }}, color: nc }}); }});
                    }}
                }});
            }});
            observer.observe(document.documentElement, {{ attributes: true }});
        }})();
        </script>"#,
        documents.len(),
        nodes = nodes_json,
        edges = edges_json
    );

    // Override layout: hide sidebar/outline on graph page
    // Pass empty sidebar/outline; the graph page is full-width
    base_template("Knowledge Graph", "graph", "", "", &main_content, None)
}

/// 404 not found page.
pub fn not_found_template(title: &str) -> String {
    let main_content = format!(
        r#"<article style="text-align:center;padding:3rem 2rem;">
            <h1 style="font-size:4rem;margin-bottom:1rem;opacity:0.3;">404</h1>
            <h1 style="margin-bottom:0.75rem;">Page Not Found</h1>
            <p style="color:var(--muted);margin-bottom:1.5rem;">
                The document "<strong>{}</strong>" does not exist or has been moved.
            </p>
            <a href="/" style="
                display:inline-flex;align-items:center;gap:0.5rem;
                padding:0.625rem 1.25rem;border-radius:8px;
                background:var(--accent);color:#fff;font-weight:600;
                text-decoration:none;
            ">
                Return Home
            </a>
            <div style="margin-top:2rem;">
                <p style="color:var(--muted);font-size:0.875rem;margin-bottom:0.5rem;">Try searching instead:</p>
                <div class="search-bar-top" style="max-width:400px;margin:0 auto;">
                    <input type="text" id="q-input" placeholder="Search documents..."
                        onkeydown="if(event.key==='Enter')window.location.href='/search?q='+encodeURIComponent(this.value)">
                    <button onclick="var q=document.getElementById('q-input').value;window.location.href='/search?q='+encodeURIComponent(q)">
                        Search
                    </button>
                </div>
            </div>
        </article>"#,
        title
    );

    base_template("Not Found", "home", "", "", &main_content, None)
}
