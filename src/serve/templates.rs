/// HTML templates for the serve command

/// Base HTML template with responsive design
pub fn base_template(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Wistra</title>
    <style>
        :root {{
            --bg: #fafafa;
            --fg: #1a1a1a;
            --muted: #666;
            --accent: #0066cc;
            --border: #e0e0e0;
            --card: #fff;
            --code-bg: #f5f5f5;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg: #1a1a1a;
                --fg: #f0f0f0;
                --muted: #aaa;
                --accent: #66b3ff;
                --border: #333;
                --card: #252525;
                --code-bg: #2d2d2d;
            }}
        }}
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            line-height: 1.6;
            color: var(--fg);
            background: var(--bg);
        }}
        .container {{
            max-width: 900px;
            margin: 0 auto;
            padding: 1rem;
        }}
        header {{
            border-bottom: 1px solid var(--border);
            padding: 1rem 0;
            margin-bottom: 2rem;
        }}
        header h1 {{
            font-size: 1.5rem;
        }}
        header h1 a {{
            color: var(--fg);
            text-decoration: none;
        }}
        nav {{
            display: flex;
            gap: 1rem;
            margin-top: 0.5rem;
            flex-wrap: wrap;
        }}
        nav a {{
            color: var(--muted);
            text-decoration: none;
        }}
        nav a:hover {{
            color: var(--accent);
        }}
        .search-form {{
            display: flex;
            gap: 0.5rem;
            margin: 1rem 0;
        }}
        .search-form input[type="text"] {{
            flex: 1;
            padding: 0.5rem 1rem;
            border: 1px solid var(--border);
            border-radius: 4px;
            background: var(--card);
            color: var(--fg);
            font-size: 1rem;
        }}
        .search-form button {{
            padding: 0.5rem 1rem;
            background: var(--accent);
            color: #fff;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }}
        .search-form button:hover {{
            opacity: 0.9;
        }}
        main {{
            min-height: calc(100vh - 200px);
        }}
        article {{
            background: var(--card);
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }}
        article h1 {{
            font-size: 2rem;
            margin-bottom: 1rem;
        }}
        .meta {{
            color: var(--muted);
            font-size: 0.875rem;
            margin-bottom: 1.5rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border);
        }}
        .meta span {{
            margin-right: 1rem;
        }}
        .tag {{
            display: inline-block;
            background: var(--border);
            padding: 0.125rem 0.5rem;
            border-radius: 4px;
            font-size: 0.75rem;
            margin-right: 0.25rem;
        }}
        .status {{
            text-transform: capitalize;
        }}
        .status-published {{ color: #28a745; }}
        .status-stub {{ color: #ffc107; }}
        .status-disambiguation {{ color: #17a2b8; }}
        article h2 {{
            font-size: 1.5rem;
            margin: 1.5rem 0 0.75rem;
        }}
        article h3 {{
            font-size: 1.25rem;
            margin: 1.25rem 0 0.5rem;
        }}
        article p {{
            margin: 1rem 0;
        }}
        article ul, article ol {{
            margin: 1rem 0;
            padding-left: 2rem;
        }}
        article li {{
            margin: 0.25rem 0;
        }}
        article code {{
            background: var(--code-bg);
            padding: 0.125rem 0.375rem;
            border-radius: 3px;
            font-family: 'SF Mono', 'Fira Code', monospace;
            font-size: 0.875em;
        }}
        article pre {{
            background: var(--code-bg);
            padding: 1rem;
            border-radius: 6px;
            overflow-x: auto;
            margin: 1rem 0;
        }}
        article pre code {{
            padding: 0;
            background: none;
        }}
        article blockquote {{
            border-left: 4px solid var(--accent);
            padding-left: 1rem;
            margin: 1rem 0;
            color: var(--muted);
        }}
        article a {{
            color: var(--accent);
            text-decoration: none;
        }}
        article a:hover {{
            text-decoration: underline;
        }}
        article table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
        }}
        article th, article td {{
            border: 1px solid var(--border);
            padding: 0.5rem;
            text-align: left;
        }}
        article th {{
            background: var(--code-bg);
        }}
        article img {{
            max-width: 100%;
            height: auto;
        }}
        article hr {{
            border: none;
            border-top: 1px solid var(--border);
            margin: 2rem 0;
        }}
        .doc-list {{
            list-style: none;
            padding: 0;
        }}
        .doc-list li {{
            padding: 0.75rem;
            border-bottom: 1px solid var(--border);
        }}
        .doc-list li:last-child {{
            border-bottom: none;
        }}
        .doc-list a {{
            color: var(--accent);
            font-weight: 500;
        }}
        .doc-list .doc-meta {{
            color: var(--muted);
            font-size: 0.875rem;
            margin-top: 0.25rem;
        }}
        .search-results {{
            background: var(--card);
            padding: 1.5rem;
            border-radius: 8px;
        }}
        .search-results h2 {{
            margin-bottom: 1rem;
        }}
        .backlinks {{
            margin-top: 2rem;
            padding-top: 1rem;
            border-top: 1px solid var(--border);
        }}
        .backlinks h3 {{
            font-size: 1rem;
            color: var(--muted);
            margin-bottom: 0.5rem;
        }}
        .backlinks ul {{
            list-style: none;
            padding: 0;
        }}
        .backlinks li {{
            display: inline;
        }}
        .backlinks li::after {{
            content: ' · ';
            color: var(--muted);
        }}
        .backlinks li:last-child::after {{
            content: '';
        }}
        footer {{
            text-align: center;
            padding: 2rem 0;
            color: var(--muted);
            font-size: 0.875rem;
        }}
        @media (max-width: 600px) {{
            .container {{
                padding: 0.5rem;
            }}
            article {{
                padding: 1rem;
            }}
            article h1 {{
                font-size: 1.5rem;
            }}
            .search-form {{
                flex-direction: column;
            }}
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
    <div class="container">
        <header>
            <h1><a href="/">📚 Wistra</a></h1>
            <nav>
                <a href="/">Home</a>
                <a href="/all">All Pages</a>
                <a href="/tags">Tags</a>
                <a href="/graph">Graph</a>
            </nav>
            <form class="search-form" action="/search" method="get">
                <input type="text" name="q" placeholder="Search wiki...">
                <button type="submit">Search</button>
            </form>
        </header>
        <main>
            {content}
        </main>
        <footer>
            Powered by <a href="https://github.com/user/wistra" style="color: var(--accent);">Wistra</a>
        </footer>
    </div>
</body>
</html>"#,
        title = title,
        content = content
    )
}

/// Home page with recent documents and stats
pub fn home_template(documents: &[DocumentInfo]) -> String {
    let recent: String = documents
        .iter()
        .take(10)
        .map(|doc| {
            format!(
                r##"<li><a href="/page/{}">{}</a><div class="doc-meta">{} · {}</div></li>"##,
                urlencoding::encode(&doc.title),
                doc.title,
                doc.status,
                doc.created
            )
        })
        .collect();

    let content = format!(
        r##"<article>
            <h1>Welcome to Wistra</h1>
            <p>Browse your personal knowledge base.</p>
            <h2>Recent Documents</h2>
            <ul class="doc-list">{}</ul>
        </article>"##,
        recent
    );

    base_template("Home", &content)
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
    let tags_html: String = tags
        .iter()
        .map(|t| format!(r##"<span class="tag">{}</span>"##, t))
        .collect();

    let aliases_html = if aliases.is_empty() {
        String::new()
    } else {
        format!(r##"<span>Aliases: {}</span>"##, aliases.join(", "))
    };

    let backlinks_html = if backlinks.is_empty() {
        String::new()
    } else {
        let links: String = backlinks
            .iter()
            .map(|t| format!(r##"<li><a href="/page/{}">{}</a></li>"##, urlencoding::encode(t), t))
            .collect();
        format!(
            r##"<div class="backlinks">
                <h3>Backlinks</h3>
                <ul>{}</ul>
            </div>"##,
            links
        )
    };

    let content = format!(
        r##"<article>
            <h1>{}</h1>
            <div class="meta">
                <span class="status status-{}">{}</span>
                <span>{}</span>
                {}
                <div style="margin-top: 0.5rem;">{}</div>
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

    base_template(title, &content)
}

/// All pages list
pub fn all_pages_template(documents: &[DocumentInfo]) -> String {
    let list: String = documents
        .iter()
        .map(|doc| {
            format!(
                r##"<li><a href="/page/{}">{}</a><div class="doc-meta">{} · {}</div></li>"##,
                urlencoding::encode(&doc.title),
                doc.title,
                doc.status,
                doc.created
            )
        })
        .collect();

    let content = format!(
        r##"<article>
            <h1>All Pages</h1>
            <p>{} documents total.</p>
            <ul class="doc-list">{}</ul>
        </article>"##,
        documents.len(),
        list
    );

    base_template("All Pages", &content)
}

/// Tags page
pub fn tags_template(tags: &[(String, usize)]) -> String {
    let list: String = tags
        .iter()
        .map(|(tag, count)| {
            format!(
                r##"<li><a href="/tag/{}">{}</a> ({} documents)</li>"##,
                urlencoding::encode(tag),
                tag,
                count
            )
        })
        .collect();

    let content = format!(
        r##"<article>
            <h1>Tags</h1>
            <ul class="doc-list">{}</ul>
        </article>"##,
        list
    );

    base_template("Tags", &content)
}

/// Documents by tag
pub fn tag_page_template(tag: &str, documents: &[DocumentInfo]) -> String {
    let list: String = documents
        .iter()
        .map(|doc| {
            format!(
                r##"<li><a href="/page/{}">{}</a></li>"##,
                urlencoding::encode(&doc.title),
                doc.title
            )
        })
        .collect();

    let content = format!(
        r##"<article>
            <h1>Tag: {}</h1>
            <p>{} documents</p>
            <ul class="doc-list">{}</ul>
        </article>"##,
        tag,
        documents.len(),
        list
    );

    base_template(&format!("Tag: {}", tag), &content)
}

/// Search results
pub fn search_results_template(query: &str, results: &[SearchResult]) -> String {
    let results_html = if results.is_empty() {
        r##"<p>No results found.</p>"##.to_string()
    } else {
        results
            .iter()
            .map(|r| {
                format!(
                    r##"<li><a href="/page/{}">{}</a> <span style="color: var(--muted);">({} match)</span></li>"##,
                    urlencoding::encode(&r.title),
                    r.title,
                    r.match_type
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };

    let content = format!(
        r##"<div class="search-results">
            <h2>Search: "{}"</h2>
            <p>{} results</p>
            <ul class="doc-list">{}</ul>
        </div>"##,
        query,
        results.len(),
        results_html
    );

    base_template(&format!("Search: {}", query), &content)
}

/// 404 page
pub fn not_found_template(title: &str) -> String {
    let content = format!(
        r##"<article>
            <h1>Not Found</h1>
            <p>The page "{}" does not exist.</p>
            <p><a href="/">Return home</a></p>
        </article>"##,
        title
    );
    base_template("Not Found", &content)
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

    let content = format!(
        r##"<article>
            <h1>Knowledge Graph</h1>
            <p>Visualizing connections between {} documents. Click a node to view the page.</p>
            <div id="graph" style="width: 100%; height: 600px; background: var(--card); border-radius: 8px; margin-top: 1rem; border: 1px solid var(--border);"></div>
        </article>
        <script src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
        <script>
            (function() {{
                const container = document.getElementById('graph');
                const nodes = new vis.DataSet([
                    {}
                ]);
                const edges = new vis.DataSet([
                    {}
                ]);
                const data = {{ nodes, edges }};
                const options = {{
                    nodes: {{
                        shape: 'dot',
                        size: 16,
                        font: {{
                            size: 12,
                            color: window.matchMedia('(prefers-color-scheme: dark)').matches ? '#f0f0f0' : '#1a1a1a'
                        }},
                        borderWidth: 2,
                        color: {{
                            background: '#0066cc',
                            border: '#004499',
                            highlight: {{
                                background: '#66b3ff',
                                border: '#0066cc'
                            }}
                        }}
                    }},
                    edges: {{
                        width: 1,
                        color: {{ color: '#cccccc', opacity: 0.6 }},
                        arrows: {{
                            to: {{ enabled: true, scaleFactor: 0.5 }}
                        }},
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
                const network = new vis.Network(container, data, options);

                // Dark mode color update
                window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {{
                    const color = e.matches ? '#f0f0f0' : '#1a1a1a';
                    nodes.forEach((node) => {{
                        nodes.update({{ id: node.id, font: {{ size: 12, color: color }} }});
                    }});
                }});

                // Click to navigate
                network.on('click', function(params) {{
                    if (params.nodes.length > 0) {{
                        const nodeId = params.nodes[0];
                        window.location.href = '/page/' + encodeURIComponent(nodeId);
                    }}
                }});

                // Change cursor on hover
                network.on('hoverNode', function() {{
                    container.style.cursor = 'pointer';
                }});
                network.on('blurNode', function() {{
                    container.style.cursor = 'default';
                }});
            }})();
        </script>"##,
        documents.len(),
        nodes_json,
        edges_json
    );

    base_template("Graph", &content)
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
