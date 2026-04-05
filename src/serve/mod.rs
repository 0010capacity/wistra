/// HTTP server for browsing the wiki

mod renderer;
mod templates;

use crate::config::{GlobalConfig, WikiConfig};
use crate::scanner;
use crate::scanner::ScanReport;
use crate::types::{Document, Status};
use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

use templates::{
    all_pages_template, graph_template, home_template, not_found_template, page_template,
    search_results_template, tag_page_template, tags_template,
};
use renderer::{extract_headings, extract_summary, render_markdown, truncate_utf8};

/// Start the HTTP server
pub async fn serve(path: &str, host: &str, port: u16, open: bool) -> Result<()> {
    let wiki_path = if path == "." {
        GlobalConfig::load()?
            .and_then(|c| c.wiki_path)
            .context("No default wiki path configured. Run `wistra onboard` first or specify a path.")?
    } else {
        PathBuf::from(shellexpand::tilde(path).to_string())
    };
    let config = WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&config)?;

    let state = WikiState {
        wiki_path: wiki_path.clone(),
        config,
        report: Arc::new(RwLock::new(report)),
    };

    let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse()?;
    let filters = create_filters(state.clone());

    println!("🌐 Serving wiki at http://{}", addr);
    println!("   Path: {}", wiki_path.display());
    println!("   Press Ctrl+C to stop.");

    if open {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(format!("http://{}", addr))
                .spawn()
                .ok();
        }
    }

    warp::serve(filters).run(addr).await;
    Ok(())
}

/// Wiki state shared across requests
#[derive(Clone)]
struct WikiState {
    wiki_path: PathBuf,
    config: WikiConfig,
    report: Arc<RwLock<ScanReport>>,
}

/// Create warp filters
fn create_filters(
    state: WikiState,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let with_state = warp::any().map(move || state.clone());

    // Routes
    let root = warp::path::end().and(with_state.clone()).and_then(handle_home);
    let all_pages = warp::path("all")
        .and(warp::query::<AllPagesQuery>())
        .and(with_state.clone())
        .and_then(handle_all_pages);
    let tags = warp::path("tags")
        .and(with_state.clone())
        .and_then(handle_tags);
    let tag_route = warp::path!("tag" / String)
        .and(with_state.clone())
        .and_then(handle_tag);
    let graph = warp::path("graph")
        .and(with_state.clone())
        .and_then(handle_graph);
    let search = warp::path("search")
        .and(warp::query::<SearchQuery>())
        .and(with_state.clone())
        .and_then(handle_search);
    let page = warp::path!("page" / String)
        .and(with_state.clone())
        .and_then(handle_page);

    root.or(all_pages)
        .or(tags)
        .or(tag_route)
        .or(graph)
        .or(search)
        .or(page)
        .with(warp::log("wistra"))
}

#[derive(Debug, serde::Deserialize)]
struct SearchQuery {
    q: String,
}

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

fn doc_to_info(doc: &Document, report: &ScanReport) -> DocumentInfo {
    let backlinks: Vec<String> = report.link_graph.incoming_links
        .get(&doc.title)
        .map(|links| links.iter().map(|l| l.source_file.trim_end_matches(".md").to_string()).collect())
        .unwrap_or_default();

    let summary = extract_summary(&doc.body, &doc.title);

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

/// Handle home page
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

/// Handle all pages list
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
    let html = templates::all_pages_template(&docs, view, query.status.as_deref(), query.tag.as_deref(), query.q.as_deref());
    Ok(warp::reply::html(html).into_response())
}

/// Handle tags page
async fn handle_tags(state: WikiState) -> Result<impl Reply, Rejection> {
    let report = state.report.read().await;
    let tags = &report.tag_stats.tag_counts;
    let html = templates::tags_template(tags);
    Ok(warp::reply::html(html).into_response())
}

/// Handle documents by tag
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

/// Handle graph page
async fn handle_graph(state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    let docs: Vec<DocumentInfo> = report
        .documents
        .values()
        .filter(|doc| doc.status != Status::Meta)
        .map(|doc| doc_to_info(doc, &report))
        .collect();

    let links: Vec<(String, String)> = report
        .link_graph
        .outgoing_links
        .iter()
        .filter_map(|(from, links)| {
            let from_title = from.trim_end_matches(".md").to_string();
            links.first().map(|link| {
                (
                    from_title.clone(),
                    link.target.trim_end_matches(".md").to_string(),
                )
            })
        })
        .collect();

    let html = graph_template(&docs, &links);
    Ok(warp::reply::html(html).into_response())
}

/// Handle search
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

    results.sort_by_key(|r| match r.match_type.as_str() {
        "title" => 0, "content" => 1, "tag" => 2, _ => 3,
    });

    let html = templates::search_results_template(&query.q, &results);
    Ok(warp::reply::html(html).into_response())
}

/// Handle individual page
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

/// Find document by title or alias
fn find_document<'a>(report: &'a ScanReport, title: &str) -> Option<&'a Document> {
    // Try exact title match first
    for doc in report.documents.values() {
        if doc.title == title {
            return Some(doc);
        }
    }

    // Try URL-decoded title match
    let decoded = urlencoding::decode(title).ok()?.to_string();
    for doc in report.documents.values() {
        if doc.title == decoded {
            return Some(doc);
        }
    }

    // Try matching by alias (case-insensitive)
    for doc in report.documents.values() {
        if doc.aliases.iter().any(|a| {
            let a_lower = a.to_lowercase();
            let title_lower = title.to_lowercase();
            a_lower == title_lower || a_lower == decoded.to_lowercase()
        }) {
            return Some(doc);
        }
    }

    None
}
