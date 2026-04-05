/// HTML templates for the serve command

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
const ICON_ARROW_RIGHT: &str = icon!("arrow-right", r#"<line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>"#);
const ICON_LAYOUT_GRID: &str = icon!("layout-grid", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>"#);
const ICON_LIST: &str = icon!("list", r#"<line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/>"#);
const ICON_X: &str = icon!("x", r#"<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>"#);

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Build a navigation link with optional active state.
pub fn nav_link(href: &str, label: &str, icon: &str, active: bool) -> String {
    let active_class = if active { " nav-link-active" } else { "" };
    format!(
        r#"<a href="{}" class="nav-link{}">{}<span>{}</span></a>"#,
        href,
        active_class,
        icon,
        label
    )
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
) -> String {
    // Helper: nav link helper to avoid repetition
    fn nav_item(href: &str, label: &str, icon: &str, active_nav: &str, current: &str) -> String {
        let active = if current == active_nav { " nav-link-active" } else { "" };
        format!(
            r#"<a href="{}" class="nav-link{}">{}<span>{}</span></a>"#,
            href, active, icon, label
        )
    }

    format!(
        r#"<!DOCTYPE html>
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
        /* ── Lists on list pages ── */
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
        /* ── Search results ── */
        .search-results {{ padding: 1rem 0; }}
        .search-results h2 {{ margin-bottom: 0.75rem; font-size: 1.125rem; }}
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
    // Close all
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
            h.id = h.textContent.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-');
        }}
        observer.observe(h);
    }});

    // Copy IDs to TOC links
    tocLinks.forEach(a => {{
        const href = a.getAttribute('href');
        if (href && href.startsWith('#')) {{
            const id = href.slice(1);
            const target = document.getElementById(id);
            if (!target) {{
                a.style.display = 'none';
            }}
        }}
    }});
}})();
</script>

<!-- ── Footer ── -->
<footer>
    Powered by <a href="https://github.com/wistra/wistra">Wistra</a>
</footer>

</body>
</html>"#,
        title = title,
        // Nav links
        nav_home = nav_item("/", "Home", ICON_HOME, active_nav, "home"),
        nav_all = nav_item("/all", "All Pages", ICON_FILE_TEXT, active_nav, "all"),
        nav_tags = nav_item("/tags", "Tags", ICON_TAG, active_nav, "tags"),
        nav_graph = nav_item("/graph", "Graph", ICON_LAYOUT_GRID, active_nav, "graph"),
        // Active flags for mobile nav
        home_active = if active_nav == "home" { "active" } else { "" },
        // Content panels
        sidebar_html = sidebar_html,
        outline_html = outline_html,
        main_content = main_content,
    )
}

// ---------------------------------------------------------------------------
// Page templates — updated to use new base_template signature
// ---------------------------------------------------------------------------

/// Home page with recent documents and stats
pub fn home_template(documents: &[DocumentInfo]) -> String {
    let recent: String = documents
        .iter()
        .take(10)
        .map(|doc| {
            format!(
                r#"<li><a href="/page/{}">{}</a><div class="doc-list-meta">{} · {}</div></li>"#,
                urlencoding::encode(&doc.title),
                doc.title,
                doc.status,
                doc.created
            )
        })
        .collect();

    let main = format!(
        r#"<article>
            <h1>Welcome to Wistra</h1>
            <p>Browse your personal knowledge base.</p>
            <h2>Recent Documents</h2>
            <ul class="doc-list">{}</ul>
        </article>"#,
        recent
    );

    base_template("Home", "home", "", "", &main)
}

/// Single document page
pub fn page_template(
    title: &str,
    html_body: &str,
    status: &str,
    tags: &[String],
    created: &str,
    aliases: &[String],
    backlinks: &[String],
) -> String {
    let tags_html: String = tags.iter().map(|t| tag_badge(t)).collect::<Vec<_>>().join(" ");

    let aliases_html = if aliases.is_empty() {
        String::new()
    } else {
        format!(r#"<span>Aliases: {}</span>"#, aliases.join(", "))
    };

    let backlinks_html = if backlinks.is_empty() {
        String::new()
    } else {
        let links: String = backlinks
            .iter()
            .map(|t| format!(r#"<li><a href="/page/{}">{}</a></li>"#, urlencoding::encode(t), t))
            .collect();
        format!(
            r##"<div class="backlinks">
                <h3>Backlinks</h3>
                <ul class="backlinks-list">{}</ul>
            </div>"##,
            links
        )
    };

    let main = format!(
        r#"<article>
            <h1>{}</h1>
            <div class="doc-meta">
                <span class="badge badge-{}">{}</span>
                <span>{}</span>
                {}
                <div style="margin-top:0.375rem;">{}</div>
            </div>
            {}
            {}
        </article>"##,
        title,
        status.to_lowercase(),
        status,
        created,
        aliases_html,
        tags_html,
        html_body,
        backlinks_html
    );

    // Build sidebar: tags list
    let sidebar = if tags.is_empty() {
        String::new()
    } else {
        let tags_section = tags
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
            .join("");
        format!(
            r#"<div class="sidebar-section">
                <div class="sidebar-title">Tags</div>
                {}
            </div>"#,
            tags_section
        )
    };

    // Build outline: from h2/h3 headings in html_body
    let outline = build_outline(html_body);

    base_template(title, "home", &sidebar, &outline, &main)
}

/// Build a table-of-contents outline HTML from rendered article HTML.
fn build_outline(html_body: &str) -> String {
    use std::collections::HashMap;

    // Extract h2 and h3 from the HTML (simple regex-free approach)
    let mut lines = Vec::new();
    let mut in_tag = false;
    let mut current_tag = String::new();
    let mut current_text = String::new();
    let mut level = 0;
    let mut capturing = false;

    for c in html_body.chars() {
        if c == '<' {
            in_tag = true;
            capturing = false;
            if current_tag.is_empty() && !current_text.is_empty() {
                let trimmed = current_text.trim();
                if !trimmed.is_empty() {
                    lines.push((level, trimmed.to_string()));
                }
                current_text.clear();
            }
            current_tag.clear();
        } else if c == '>' {
            in_tag = false;
            let tag = current_tag.trim().to_lowercase();
            if tag.starts_with("h2") || tag.starts_with("h3") {
                let lvl = if tag == "h2" { 2 } else { 3 };
                level = lvl;
                capturing = true;
                current_text.clear();
            }
        } else if in_tag {
            current_tag.push(c);
        } else if capturing {
            current_text.push(c);
        }
    }
    if !current_text.trim().is_empty() {
        lines.push((level, current_text.trim().to_string()));
    }

    if lines.is_empty() {
        return String::new();
    }

    let items: String = lines
        .iter()
        .map(|(lvl, text)| {
            let id = text
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>();
            let cls = if *lvl == 3 { " toc-h3" } else { " toc-h2" };
            format!(
                r#"<li><a href="#{}" class="{}">{}</a></li>"#,
                id, cls, text
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"<div class="outline-title">On This Page</div>
        <ul class="outline-list">{}</ul>"#,
        items
    )
}

/// All pages list
pub fn all_pages_template(documents: &[DocumentInfo]) -> String {
    let list: String = documents
        .iter()
        .map(|doc| {
            format!(
                r#"<li><a href="/page/{}">{}</a><div class="doc-list-meta">{} · {}</div></li>"#,
                urlencoding::encode(&doc.title),
                doc.title,
                doc.status,
                doc.created
            )
        })
        .collect();

    let main = format!(
        r#"<article>
            <h1>All Pages</h1>
            <p>{} documents total.</p>
            <ul class="doc-list">{}</ul>
        </article>"#,
        documents.len(),
        list
    );

    base_template("All Pages", "all", "", "", &main)
}

/// Tags page
pub fn tags_template(tags: &[(String, usize)]) -> String {
    let list: String = tags
        .iter()
        .map(|(tag, count)| {
            format!(
                r#"<li><a href="/tag/{}">{}</a><div class="doc-list-meta">{} documents</div></li>"#,
                urlencoding::encode(tag),
                tag,
                count
            )
        })
        .collect();

    let main = format!(
        r#"<article>
            <h1>Tags</h1>
            <ul class="doc-list">{}</ul>
        </article>"#,
        list
    );

    base_template("Tags", "tags", "", "", &main)
}

/// Documents by tag
pub fn tag_page_template(tag: &str, documents: &[DocumentInfo]) -> String {
    let list: String = documents
        .iter()
        .map(|doc| {
            format!(
                r#"<li><a href="/page/{}">{}</a></li>"#,
                urlencoding::encode(&doc.title),
                doc.title
            )
        })
        .collect();

    let main = format!(
        r#"<article>
            <h1>Tag: {}</h1>
            <p>{} documents</p>
            <ul class="doc-list">{}</ul>
        </article>"#,
        tag,
        documents.len(),
        list
    );

    base_template(&format!("Tag: {}", tag), "tags", "", "", &main)
}

/// Search results
pub fn search_results_template(query: &str, results: &[SearchResult]) -> String {
    let results_html = if results.is_empty() {
        r#"<p>No results found.</p>"#.to_string()
    } else {
        results
            .iter()
            .map(|r| {
                format!(
                    r#"<li><a href="/page/{}">{}</a><div class="doc-list-meta">{}</div></li>"#,
                    urlencoding::encode(&r.title),
                    r.title,
                    r.match_type
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };

    let main = format!(
        r#"<article>
            <h1>Search: "{}"</h1>
            <p>{} results</p>
            <ul class="doc-list">{}</ul>
        </article>"#,
        query,
        results.len(),
        results_html
    );

    base_template(&format!("Search: {}", query), "home", "", "", &main)
}

/// 404 page
pub fn not_found_template(title: &str) -> String {
    let main = format!(
        r#"<article>
            <h1>Not Found</h1>
            <p>The page "{}" does not exist.</p>
            <p><a href="/">Return home</a></p>
        </article>"#,
        title
    );

    base_template("Not Found", "home", "", "", &main)
}

/// Graph page with interactive network visualization
pub fn graph_template(documents: &[DocumentInfo], links: &[(String, String)]) -> String {
    // Build nodes JSON array
    let nodes_json: String = documents
        .iter()
        .map(|doc| {
            format!(
                r#"{{"id": "{}", "label": "{}", "title": "{}"}}"#,
                doc.title.replace('"', "\\\""),
                truncate_label(&doc.title, 20),
                doc.title.replace('"', "\\\"")
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    // Build edges JSON array
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

    let main = format!(
        r#"<article>
            <h1>Knowledge Graph</h1>
            <p>Visualizing connections between {0} documents. Click a node to view the page.</p>
            <div id="graph" style="width:100%;height:500px;background:var(--card);border-radius:8px;margin-top:1rem;border:1px solid var(--border);"></div>
        </article>
        <script src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
        <script>
        (function() {{
            var isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            var nodeColor = isDark ? {{ background: '#d97706', border: '#fbbf24' }} : {{ background: '#b45309', border: '#92400e' }};
            var edgeColor = isDark ? 'rgba(248,250,252,0.2)' : 'rgba(28,25,23,0.2)';
            var textColor = isDark ? '#fafaf9' : '#1c1917';

            var container = document.getElementById('graph');
            var nodes = new vis.DataSet([{1}]);
            var edges = new vis.DataSet([{2}]);
            var data = {{ nodes: nodes, edges: edges }};
            var options = {{
                nodes: {{
                    shape: 'dot',
                    size: 16,
                    font: {{ size: 12, color: textColor }},
                    borderWidth: 2,
                    color: nodeColor
                }},
                edges: {{
                    width: 1,
                    color: {{ color: edgeColor, opacity: 0.6 }},
                    arrows: {{ to: {{ enabled: true, scaleFactor: 0.5 }} }},
                    smooth: {{ type: 'continuous' }}
                }},
                physics: {{
                    stabilization: {{ iterations: 150 }},
                    barnesHut: {{
                        gravitationalConstant: -2000,
                        springConstant: 0.04,
                        springLength: 100
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

            // Re-apply dark mode colors on change
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {{
                var dark = e.matches;
                var nc = dark ? {{ background: '#d97706', border: '#fbbf24' }} : {{ background: '#b45309', border: '#92400e' }};
                var ec = dark ? 'rgba(248,250,252,0.2)' : 'rgba(28,25,23,0.2)';
                var tc = dark ? '#fafaf9' : '#1c1917';
                nodes.forEach(function(node) {{
                    nodes.update({{ id: node.id, font: {{ size: 12, color: tc }}, color: nc }});
                }});
            }});
        }})();
        </script>"#,
        documents.len(),
        nodes_json,
        edges_json
    );

    base_template("Graph", "graph", "", "", &main)
}

/// Truncate label with ellipsis if too long
fn truncate_label(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}

/// Document info for templates
#[derive(Debug, Clone)]
pub struct DocumentInfo {
    pub title: String,
    pub status: String,
    pub created: String,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub match_type: String,
}
