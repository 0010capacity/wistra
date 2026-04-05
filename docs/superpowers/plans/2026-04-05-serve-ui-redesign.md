# Serve UI Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Redesign the `wistra serve` web UI from a basic single-column layout to a 3-panel wiki browser with Warm Stone color theme, Lucide icons, and mobile-responsive bottom sheets.

**Architecture:** Server-rendered HTML templates in Rust with inline CSS/JS. Three files modified: `renderer.rs` (heading extraction, summary extraction, UTF-8 truncation), `mod.rs` (new data structs, query param filtering, updated handlers), `templates.rs` (complete template rewrite with 3-panel layout, Lucide SVG icons, responsive CSS).

**Tech Stack:** Rust, warp (HTTP), pulldown-cmark (markdown), KaTeX CDN (math), vis-network CDN (graph)

**Spec:** `docs/superpowers/specs/2026-04-05-serve-ui-redesign-design.md`

---

## File Structure

| File | Responsibility |
|---|---|
| `src/serve/renderer.rs` | `extract_headings()`, `extract_summary()`, `truncate_utf8()`, heading ID injection in `render_markdown()` |
| `src/serve/mod.rs` | `DocumentInfo`, `Heading`, `SearchResultInfo`, `AllPagesQuery` structs; updated handlers with TOC/summary/filtering |
| `src/serve/templates.rs` | Complete rewrite: `base_template` (3-panel + responsive + Lucide icons + inline JS), all page templates |

## Implementation Order

Phase 1: Data layer (renderer.rs) — pure functions, easy to test
Phase 2: Handler layer (mod.rs) — data structs and handler updates
Phase 3: Template layer (templates.rs) — visual rewrite

---

### Task 1: Add `truncate_utf8` utility function

**Files:**
- Modify: `src/serve/renderer.rs`
- Test: `src/serve/renderer.rs` (inline `#[cfg(test)]`)

- [ ] **Step 1: Write the failing test**

Add to the `#[cfg(test)]` module in `src/serve/renderer.rs`:

```rust
#[test]
fn test_truncate_utf8_short() {
    assert_eq!(truncate_utf8("hello", 10), "hello");
}

#[test]
fn test_truncate_utf8_exact() {
    assert_eq!(truncate_utf8("hello", 5), "hello");
}

#[test]
fn test_truncate_utf8_truncate() {
    assert_eq!(truncate_utf8("hello world", 5), "hello...");
}

#[test]
fn test_truncate_utf8_korean() {
    assert_eq!(truncate_utf8("머신러닝알고리즘", 4), "머신러닝...");
}

#[test]
fn test_truncate_utf8_empty() {
    assert_eq!(truncate_utf8("", 10), "");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --lib serve::renderer -- test_truncate`
Expected: FAIL — `truncate_utf8` not found

- [ ] **Step 3: Implement `truncate_utf8`**

Add to `src/serve/renderer.rs` (public function):

```rust
pub fn truncate_utf8(s: &str, max_chars: usize) -> String {
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

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib serve::renderer -- test_truncate`
Expected: All 5 tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/serve/renderer.rs
git commit -m "feat(serve): add UTF-8 safe string truncation utility"
```

---

### Task 2: Add `extract_headings` function

**Files:**
- Modify: `src/serve/renderer.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_extract_headings_basic() {
    let html = r#"<h2>개요</h2><p>text</p><h3>상세</h3><p>more</p><h2>결론</h2>"#;
    let headings = extract_headings(html);
    assert_eq!(headings.len(), 3);
    assert_eq!(headings[0].level, 2);
    assert_eq!(headings[0].text, "개요");
    assert_eq!(headings[1].level, 3);
    assert_eq!(headings[1].text, "상세");
    assert_eq!(headings[2].level, 2);
    assert_eq!(headings[2].text, "결론");
}

#[test]
fn test_extract_headings_empty() {
    let headings = extract_headings("<p>no headings</p>");
    assert!(headings.is_empty());
}

#[test]
fn test_extract_headings_id_from_korean() {
    let html = r#"<h2>주요 유형</h2>"#;
    let headings = extract_headings(html);
    assert_eq!(headings[0].id, "주요-유형");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --lib serve::renderer -- test_extract_headings`
Expected: FAIL

- [ ] **Step 3: Implement `extract_headings`**

Add the `Heading` struct and function to `src/serve/renderer.rs`:

```rust
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

pub fn extract_headings(html: &str) -> Vec<Heading> {
    let h2_re = regex::Regex::new(r"<h2>(.*?)</h2>").unwrap();
    let h3_re = regex::Regex::new(r"<h3>(.*?)</h3>").unwrap();
    let mut headings: Vec<(usize, u8, String)> = Vec::new();

    for cap in h2_re.captures_iter(html) {
        let pos = html.find(&cap[0]).unwrap_or(0);
        headings.push((pos, 2, cap[1].to_string()));
    }
    for cap in h3_re.captures_iter(html) {
        let pos = html.find(&cap[0]).unwrap_or(0);
        headings.push((pos, 3, cap[1].to_string()));
    }
    headings.sort_by_key(|(pos, _, _)| *pos);

    headings.into_iter().map(|(_, level, text)| {
        let id = text.replace(' ', "-");
        Heading { level, id, text }
    }).collect()
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib serve::renderer -- test_extract_headings`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/serve/renderer.rs
git commit -m "feat(serve): add extract_headings for TOC generation"
```

---

### Task 3: Inject heading IDs into rendered HTML

**Files:**
- Modify: `src/serve/renderer.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_render_markdown_injects_heading_ids() {
    let md = "## 개요\n\nSome text\n\n## 주요 유형\n\nMore text";
    let html = render_markdown(md);
    assert!(html.contains(r#"<h2 id="개요">"#), "h2 should have id attribute");
    assert!(html.contains(r#"<h2 id="주요-유형">"#), "h2 id should have dashes for spaces");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --lib serve::renderer -- test_render_markdown_injects_heading_ids`
Expected: FAIL — current output is `<h2>` without id

- [ ] **Step 3: Modify `render_markdown` to inject heading IDs**

After the `html::push_html` call in `render_markdown`, add a post-processing step:

```rust
// After pushing html and restoring latex placeholders:
let h2_re = regex::Regex::new(r"<h2>(.*?)</h2>").unwrap();
let h3_re = regex::Regex::new(r"<h3>(.*?)</h3>").unwrap();

let result = h2_re.replace_all(&html_output, |caps: &regex::Captures| {
    let id = caps[1].replace(' ', "-");
    format!(r#"<h2 id="{}">{}</h2>"#, id, &caps[1])
}).to_string();

let result = h3_re.replace_all(&result, |caps: &regex::Captures| {
    let id = caps[1].replace(' ', "-");
    format!(r#"<h3 id="{}">{}</h3>"#, id, &caps[1])
}).to_string();
```

Note: This goes at the end of `render_markdown`, after LaTeX placeholder restoration, replacing the final return value.

- [ ] **Step 4: Run all renderer tests**

Run: `cargo test --lib serve::renderer`
Expected: All tests PASS (including existing ones — no regression)

- [ ] **Step 5: Commit**

```bash
git add src/serve/renderer.rs
git commit -m "feat(serve): inject heading IDs for TOC anchor links"
```

---

### Task 4: Add `extract_summary` function

**Files:**
- Modify: `src/serve/renderer.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_extract_summary_basic() {
    let md = "# Title\n\nFirst paragraph with some text.\n\nSecond paragraph.";
    let summary = extract_summary(md, "Title");
    assert!(summary.starts_with("First paragraph"));
}

#[test]
fn test_extract_summary_strips_wikilinks() {
    let md = "# Title\n\nSee [[Python]] for details about [[AI|인공지능]].";
    let summary = extract_summary(md, "Title");
    assert!(!summary.contains("[["), "should strip wikilink syntax");
    assert!(summary.contains("Python"), "should keep link text");
}

#[test]
fn test_extract_summary_fallback() {
    let md = "# Title\n\n$$x^2$$\n\n```code```";
    let summary = extract_summary(md, "수학");
    assert!(summary.contains("수학"), "fallback should include title");
}

#[test]
fn test_extract_summary_truncates() {
    let long_text = "a".repeat(300);
    let md = format!("# Title\n\n{}", long_text);
    let summary = extract_summary(&md, "Title");
    assert!(summary.len() < 250, "should truncate to ~200 chars + ellipsis");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --lib serve::renderer -- test_extract_summary`
Expected: FAIL

- [ ] **Step 3: Implement `extract_summary`**

```rust
pub fn extract_summary(content: &str, title: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("$$")
            || trimmed.starts_with("```")
            || trimmed.starts_with('|')
            || trimmed.starts_with('>')
            || trimmed.starts_with("- [")
            || trimmed.starts_with("* [")
        {
            continue;
        }
        // Strip markdown formatting
        let stripped = trimmed
            .replace(|c: char| c == '*' || c == '_', "")
            .replacen("`", "", 10);
        // Strip wikilinks: [[Target]] -> Target, [[Target|Display]] -> Display
        let wikilink_re = regex::Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap();
        let clean = wikilink_re.replace_all(&stripped, |caps: &regex::Captures| {
            if let Some(display) = caps.get(2) {
                display.as_str().to_string()
            } else {
                caps[1].to_string()
            }
        }).to_string();
        return truncate_utf8(&clean, 200);
    }
    format!("{} — 상세 내용을 확인하세요.", title)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib serve::renderer -- test_extract_summary`
Expected: All 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/serve/renderer.rs
git commit -m "feat(serve): add extract_summary for callout and card snippets"
```

---

### Task 5: Add new data structs and update handlers in `mod.rs`

**Files:**
- Modify: `src/serve/mod.rs`

- [ ] **Step 1: Add data structs**

Add after the existing `SearchQuery` struct:

```rust
#[derive(Debug, Clone)]
pub struct DocumentInfo {
    pub title: String,
    pub status: String,
    pub tags: Vec<String>,
    pub created: String,
    pub summary: String,
    pub aliases: Vec<String>,
    pub backlinks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResultInfo {
    pub title: String,
    pub status: String,
    pub tags: Vec<String>,
    pub match_type: String,
    pub snippet: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AllPagesQuery {
    pub status: Option<String>,
    pub tag: Option<String>,
    pub q: Option<String>,
    pub view: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}
```

Note: `Heading` struct is defined in `renderer.rs` (Task 2) and re-used directly. No duplicate `HeadingInfo` needed.

- [ ] **Step 2: Add `doc_to_info` helper function**

```rust
fn doc_to_info(doc: &Document, report: &ScanReport) -> DocumentInfo {
    let backlinks: Vec<String> = report.link_graph.incoming_links
        .get(&doc.title)
        .map(|links| links.iter().map(|l| l.source_file.trim_end_matches(".md").to_string()).collect())
        .unwrap_or_default();

    let summary = renderer::extract_summary(&doc.body, &doc.title);

    DocumentInfo {
        title: doc.title.clone(),
        status: doc.status.to_string(),
        tags: doc.tags.clone(),
        created: doc.created.to_string(),
        summary,
        aliases: doc.aliases.clone(),
        backlinks,
    }
}
```

- [ ] **Step 3: Build and verify compilation**

Run: `cargo build`
Expected: Compiles with errors in `templates.rs` due to changed signatures — this is expected, we'll fix templates next.

Note: If compilation fails in mod.rs itself, fix those errors first before proceeding.

- [ ] **Step 4: Commit**

```bash
git add src/serve/mod.rs
git commit -m "feat(serve): add DocumentInfo, HeadingInfo, SearchResultInfo, AllPagesQuery structs"
```

---

### Task 6: Update `handle_page` to pass TOC and summary

**Files:**
- Modify: `src/serve/mod.rs`

- [ ] **Step 1: Update `handle_page` handler**

Replace the existing `handle_page` body. Key changes:
- Call `extract_headings()` on rendered HTML
- Use `doc_to_info()` for DocumentInfo with summary
- Pass headings to `page_template`

```rust
async fn handle_page(title: String, state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    match find_document(&report, &title) {
        Some(doc) => {
            let html_body = renderer::render_markdown(&doc.body);
            let headings = renderer::extract_headings(&html_body);
            let info = doc_to_info(doc, &report);
            let html = templates::page_template(&info, &html_body, &headings);
            Ok(warp::reply::html(html).into_response())
        }
        None => {
            let html = templates::not_found_template(&title);
            Ok(warp::reply::with_status(html, StatusCode::NOT_FOUND).into_response())
        }
    }
}
```

- [ ] **Step 2: Build to verify**

Run: `cargo build`
Expected: May fail due to template signature mismatch — that's OK, templates are rewritten next.

- [ ] **Step 3: Commit**

```bash
git add src/serve/mod.rs
git commit -m "feat(serve): update handle_page to pass TOC headings and summary"
```

---

### Task 7: Update remaining handlers

**Files:**
- Modify: `src/serve/mod.rs`

- [ ] **Step 1: Update `handle_home`**

```rust
async fn handle_home(state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    let mut docs: Vec<DocumentInfo> = report.documents
        .iter()
        .filter(|(_, d)| d.status != Status::Meta)
        .map(|(_, d)| doc_to_info(d, &report))
        .collect();
    docs.sort_by(|a, b| b.created.cmp(&a.created));
    let recent: Vec<&DocumentInfo> = docs.iter().take(5).collect();
    let random = docs.choose(&mut rand::thread_rng());
    let total = docs.len();
    let published = docs.iter().filter(|d| d.status == "published").count();
    let stubs = docs.iter().filter(|d| d.status == "stub").count();
    let tag_count = report.tag_stats.unique_tags;
    let html = templates::home_template(&recent, random, total, published, stubs, tag_count);
    Ok(warp::reply::html(html).into_response())
}
```

Note: `rand` is already a dependency in `Cargo.toml`. Add `use rand::seq::SliceRandom;` at the top of `mod.rs`.

- [ ] **Step 2: Update `handle_all_pages` with query parameter filtering**

```rust
async fn handle_all_pages(query: AllPagesQuery, state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    let mut docs: Vec<DocumentInfo> = report.documents
        .iter()
        .filter(|(_, d)| d.status != Status::Meta)
        .map(|(_, d)| doc_to_info(d, &report))
        .collect();

    // Filter by status
    if let Some(ref status) = query.status {
        if !status.is_empty() {
            docs.retain(|d| d.status == *status);
        }
    }
    // Filter by tag
    if let Some(ref tag) = query.tag {
        if !tag.is_empty() {
            docs.retain(|d| d.tags.iter().any(|t| t == tag));
        }
    }
    // Filter by search
    if let Some(ref q) = query.q {
        if !q.is_empty() {
            let lower = q.to_lowercase();
            docs.retain(|d| d.title.to_lowercase().contains(&lower));
        }
    }
    // Sort
    let sort_field = query.sort.as_deref().unwrap_or("date");
    let ascending = query.order.as_deref() == Some("asc");
    docs.sort_by(|a, b| {
        let cmp = match sort_field {
            "title" => a.title.cmp(&b.title),
            "status" => a.status.cmp(&b.status),
            _ => b.created.cmp(&a.created),
        };
        if ascending { cmp.reverse() } else { cmp }
    });

    let view = query.view.as_deref().unwrap_or("grid");
    let html = templates::all_pages_template(&docs, view, &query);
    Ok(warp::reply::html(html).into_response())
}
```

Update the route registration for `/all` to include query parameter extraction:

```rust
let all_pages = warp::path("all")
    .and(warp::query::<AllPagesQuery>())
    .and(with_state(state.clone()))
    .and_then(handle_all_pages);
```

- [ ] **Step 3: Update `handle_search` to use `SearchResultInfo`**

```rust
async fn handle_search(query: SearchQuery, state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    let q = query.q.to_lowercase();
    let mut results: Vec<SearchResultInfo> = Vec::new();

    for (_, doc) in &report.documents {
        if doc.status == Status::Meta { continue; }
        let title_lower = doc.title.to_lowercase();
        let body_lower = doc.body.to_lowercase();
        let (match_type, snippet) = if title_lower.contains(&q) {
            ("title".into(), renderer::extract_summary(&doc.body, &doc.title))
        } else if body_lower.contains(&q) {
            // Extract context around match
            let pos = body_lower.find(&q).unwrap();
            let start = pos.saturating_sub(50);
            let end = (pos + q.len() + 50).min(doc.body.len());
            let context = doc.body.get(start..end).unwrap_or("").to_string();
            ("content".into(), renderer::truncate_utf8(&context, 120))
        } else if doc.tags.iter().any(|t| t.to_lowercase().contains(&q)) {
            ("tag".into(), renderer::extract_summary(&doc.body, &doc.title))
        } else if doc.aliases.iter().any(|a| a.to_lowercase().contains(&q)) {
            ("alias".into(), renderer::extract_summary(&doc.body, &doc.title))
        } else {
            continue;
        };
        results.push(SearchResultInfo {
            title: doc.title.clone(),
            status: doc.status.to_string(),
            tags: doc.tags.clone(),
            match_type,
            snippet,
        });
    }

    // Sort: title > content > tag > alias
    results.sort_by_key(|r| match r.match_type.as_str() {
        "title" => 0, "content" => 1, "tag" => 2, _ => 3,
    });

    let html = templates::search_results_template(&query.q, &results);
    Ok(warp::reply::html(html).into_response())
}
```

- [ ] **Step 4: Update `handle_tags` and `handle_tag`**

```rust
async fn handle_tags(state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    let tags = &report.tag_stats.tag_counts;
    let html = templates::tags_template(tags);
    Ok(warp::reply::html(html).into_response())
}

async fn handle_tag(tag: String, state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    let docs: Vec<DocumentInfo> = report.documents
        .iter()
        .filter(|(_, d)| d.status != Status::Meta && d.tags.contains(&tag))
        .map(|(_, d)| doc_to_info(d, &report))
        .collect();
    let html = templates::tag_page_template(&tag, &docs);
    Ok(warp::reply::html(html).into_response())
}
```

- [ ] **Step 5: Build and verify compilation**

Run: `cargo build`
Expected: Fails in `templates.rs` due to changed function signatures — this is expected.

- [ ] **Step 6: Commit**

```bash
git add src/serve/mod.rs
git commit -m "feat(serve): update all handlers with new data structs and filtering"
```

---

### Task 8: Rewrite `base_template` — CSS, layout, icons

**Files:**
- Modify: `src/serve/templates.rs`

This is the largest task. The `base_template` is the foundation for all pages.

- [ ] **Step 1: Define Lucide SVG icon constants**

Add at the top of `templates.rs` (inside the module, before functions):

```rust
macro_rules! icon {
    ($name:expr, $svg:path:expr) => {
        concat!(r#"<svg class="icon" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none">"#, $svg, "</svg>")
    };
}

const ICON_BOOK_OPEN: &str = icon!("book-open", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/>"#);
const ICON_HOME: &str = icon!("home", r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/>"#);
const ICON_FILE_TEXT: &str = icon!("file-text", r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><line x1="10" y1="9" x2="8" y2="9"/>"#);
const ICON_TAG: &str = icon!("tag", r#"<path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/>"#);
const ICON_SHARE: &str = icon!("share-2", r#"<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>"#);
const ICON_SEARCH: &str = icon!("search", r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#);
const ICON_MOON: &str = icon!("moon", r#"<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>"#);
const ICON_ARROW_RIGHT: &str = icon!("arrow-right", r#"<line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>"#);
const ICON_ARROW_LEFT: &str = icon!("arrow-left", r#"<line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/>"#);
const ICON_MENU: &str = icon!("menu", r#"<line x1="3" y1="12" x2="21" y2="12"/><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="18" x2="21" y2="18"/>"#);
const ICON_LIST_ORDERED: &str = icon!("list-ordered", r#"<line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/>"#);
const ICON_LIGHTBULB: &str = icon!("lightbulb", r#"<path d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 1 1 7.072 0l-.548.547A3.374 3.374 0 0 0 14 18.469V19a2 2 0 1 1-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/>"#);
const ICON_X: &str = icon!("x", r#"<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>"#);
const ICON_CLOCK: &str = icon!("clock", r#"<circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>"#);
const ICON_LAYOUT_GRID: &str = icon!("layout-grid", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>"#);
const ICON_LIST: &str = icon!("list", r#"<line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/>"#);
const ICON_FILTER: &str = icon!("filter", r#"<polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>"#);
const ICON_CALENDAR: &str = icon!("calendar", r#"<rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/>"#);
```

- [ ] **Step 2: Rewrite `base_template` with 3-panel layout, Warm Stone CSS, and inline JS**

The new `base_template` must include:
- `<html lang="ko">`
- CSS variables for light/dark mode (see spec Color Tokens)
- 3-panel CSS (sidebar 220px, main flex:1, outline 200px)
- Responsive CSS (`@media` for tablet/mobile)
- Mobile bottom sheet CSS
- Top bar (dark background, logo, search, dark mode indicator)
- Sidebar HTML (nav links, recent docs, tag badges)
- Inline JS for bottom sheet toggle and scroll-spy

This is a large function (~200 lines of `format!` with HTML). Write it as a single coherent function that takes:
- `title: &str`
- `active_nav: &str` ("home", "all", "tags", "graph", or "")
- `sidebar_html: &str` — pre-rendered sidebar content
- `outline_html: &str` — pre-rendered right panel content (empty string if not needed)
- `main_content: &str` — the page-specific content

- [ ] **Step 3: Add helper functions for sidebar and outline rendering**

```rust
fn sidebar_template(active_nav: &str, recent_docs: &[&DocumentInfo], tags: &[(String, usize)]) -> String
fn outline_template(headings: &[HeadingInfo], backlinks: &[String], aliases: &[String]) -> String
```

- [ ] **Step 4: Build and verify CSS renders**

Run: `cargo build`
Expected: Compiles. Then manually test with `cargo run -- serve --port 3456`.

- [ ] **Step 5: Commit**

```bash
git add src/serve/templates.rs
git commit -m "feat(serve): rewrite base_template with 3-panel layout, Warm Stone theme, Lucide icons"
```

---

### Task 9: Rewrite page templates (home, page, all-pages, search, tags, tag-page, graph, not-found)

**Files:**
- Modify: `src/serve/templates.rs`

- [ ] **Step 1: Rewrite `home_template`**

New signature:
```rust
pub fn home_template(recent: &[&DocumentInfo], random: Option<&DocumentInfo>, total: usize, published: usize, stubs: usize, tag_count: usize) -> String
```

Content: Stats row (4 cards), recent docs list, random suggestion callout, right panel with quick links.

- [ ] **Step 2: Rewrite `page_template`**

New signature:
```rust
pub fn page_template(doc: &DocumentInfo, html_body: &str, headings: &[HeadingInfo]) -> String
```

Content: Title, meta bar (status, date, tags), callout summary, rendered body, related docs, outline with TOC/backlinks/aliases.

- [ ] **Step 3: Rewrite `all_pages_template`**

New signature:
```rust
pub fn all_pages_template(docs: &[DocumentInfo], view: &str, query: &AllPagesQuery) -> String
```

Content: Filter bar with dropdowns, grid/list toggle, document cards or table rows.

Note: `AllPagesQuery` is defined in `mod.rs`. In `templates.rs`, reference it as `super::AllPagesQuery`. Alternatively, pass the relevant filter values (status, tag, q, view) as individual `&str` parameters instead of the whole struct to avoid cross-module coupling.

- [ ] **Step 4: Rewrite `search_results_template`**

New signature:
```rust
pub fn search_results_template(query: &str, results: &[SearchResultInfo]) -> String
```

Content: Search bar with query, result count, result cards with title/status/match-type/snippet/tags.

- [ ] **Step 5: Rewrite `tags_template` and `tag_page_template`**

```rust
pub fn tags_template(tags: &[(String, usize)]) -> String
pub fn tag_page_template(tag: &str, docs: &[DocumentInfo]) -> String
```

Content: Tag cloud with sized/colored badges. Tag page with filtered document list.

- [ ] **Step 6: Rewrite `graph_template`**

```rust
pub fn graph_template(documents: &[DocumentInfo], links: &[(String, String)]) -> String
```

Content: Full-width dark canvas, vis-network with Warm Stone colors, legend.

- [ ] **Step 7: Rewrite `not_found_template`**

```rust
pub fn not_found_template(title: &str) -> String
```

Content: 404 page with search suggestion.

- [ ] **Step 8: Build and manually test**

Run: `cargo build && cargo run -- serve --port 3456 --open`
Expected: Browser opens with new UI. Test all pages:
- `/` — Home dashboard
- `/all` — All pages with filters
- `/tags` — Tag cloud
- `/tag/<tag>` — Filtered tag page
- `/graph` — Knowledge graph
- `/search?q=머신러닝` — Search results
- `/page/<title>` — Page view with TOC

- [ ] **Step 9: Commit**

```bash
git add src/serve/templates.rs
git commit -m "feat(serve): rewrite all page templates with Warm Stone design"
```

---

### Task 10: Wire up handlers to new templates and integration test

**Files:**
- Modify: `src/serve/mod.rs`

- [ ] **Step 1: Fix all handler→template call signatures**

Go through each handler and update the template function calls to match the new signatures from Tasks 8-9. Fix all compilation errors.

- [ ] **Step 2: Update route registration for `/all` with query params**

Ensure the route uses `warp::query::<AllPagesQuery>()` for the `/all` endpoint.

- [ ] **Step 3: Build and fix all compilation errors**

Run: `cargo build`
Expected: Clean compilation, zero errors

- [ ] **Step 4: Manual smoke test**

Run: `cargo run -- serve --port 3456 --open`

Test checklist:
- [ ] Home page loads with stats, recent docs, random suggestion
- [ ] All Pages loads with filter bar, grid view default
- [ ] Switch to list view via toggle
- [ ] Tags page shows colored tag cloud
- [ ] Click tag → filtered document list
- [ ] Graph page shows full-width visualization
- [ ] Search returns results with match type badges
- [ ] Page view shows 3-panel layout with TOC, backlinks, aliases
- [ ] Mobile: resize to < 768px, verify bottom sheet toggle works
- [ ] Dark mode: check `prefers-color-scheme: dark` or browser dev tools toggle
- [ ] KaTeX formulas still render correctly
- [ ] Wikilinks still work

- [ ] **Step 5: Commit**

```bash
git add src/serve/mod.rs src/serve/templates.rs
git commit -m "feat(serve): wire up handlers to new templates, complete UI redesign"
```

---

### Task 11: Fix `truncate_label` to use UTF-8 safe truncation

**Files:**
- Modify: `src/serve/templates.rs`

- [ ] **Step 1: Replace `truncate_label` implementation**

The existing `truncate_label` uses byte-based `s.len()` which breaks Korean text. Replace it to use the `truncate_utf8` function from renderer:

```rust
fn truncate_label(s: &str, max_chars: usize) -> String {
    crate::serve::renderer::truncate_utf8(s, max_chars)
}
```

Or if that creates a circular dependency issue, duplicate the logic inline:

```rust
fn truncate_label(s: &str, max_chars: usize) -> String {
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

- [ ] **Step 2: Build and test**

Run: `cargo build && cargo run -- serve --port 3456`
Expected: Graph node labels with Korean text display correctly without mojibake.

- [ ] **Step 3: Commit**

```bash
git add src/serve/templates.rs
git commit -m "fix(serve): use UTF-8 safe truncation for graph node labels"
```

---

### Task 12: Final polish and verification

**Files:**
- Possibly modify: `src/serve/templates.rs`, `src/serve/mod.rs`

- [ ] **Step 1: Verify dark mode colors**

Open browser DevTools → toggle dark mode. Check:
- Background/text contrast
- Tag badge colors
- Status badge colors
- Top bar colors
- Card backgrounds

- [ ] **Step 2: Verify mobile responsiveness**

Use browser DevTools responsive mode at 375px, 768px, 1024px widths.

- [ ] **Step 3: Fix any visual issues found**

- [ ] **Step 4: Final commit**

```bash
git add -A
git commit -m "style(serve): polish responsive layout and dark mode colors"
```
