/// HTTP server for browsing the wiki

mod renderer;
mod templates;

use crate::config::WikiConfig;
use crate::scanner;
use crate::scanner::ScanReport;
use crate::types::{Document, Status};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::http::StatusCode;
use warp::Filter;
use warp::Reply;

use templates::{
    all_pages_template, graph_template, home_template, not_found_template, page_template,
    search_results_template, tag_page_template, tags_template, DocumentInfo, SearchResult,
};
use renderer::render_markdown;

/// Start the HTTP server
pub async fn serve(path: &str, host: &str, port: u16, open: bool) -> Result<()> {
    let wiki_path = PathBuf::from(shellexpand::tilde(path).to_string());
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

/// Handle home page
async fn handle_home(state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    let docs: Vec<DocumentInfo> = report
        .documents
        .values()
        .filter(|doc| doc.status != Status::Meta)
        .map(|doc| DocumentInfo {
            title: doc.title.clone(),
            status: doc.status.to_string(),
            created: doc.created.format("%Y-%m-%d").to_string(),
        })
        .collect();

    let html = home_template(&docs);
    Ok(warp::reply::html(html).into_response())
}

/// Handle all pages list
async fn handle_all_pages(state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    let mut docs: Vec<DocumentInfo> = report
        .documents
        .values()
        .filter(|doc| doc.status != Status::Meta)
        .map(|doc| DocumentInfo {
            title: doc.title.clone(),
            status: doc.status.to_string(),
            created: doc.created.format("%Y-%m-%d").to_string(),
        })
        .collect();

    docs.sort_by(|a, b| a.title.cmp(&b.title));

    let html = all_pages_template(&docs);
    Ok(warp::reply::html(html).into_response())
}

/// Handle tags page
async fn handle_tags(state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    let mut tags: Vec<(String, usize)> = report
        .tag_stats
        .tag_counts
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    tags.sort_by(|a, b| b.1.cmp(&a.1));

    let html = tags_template(&tags);
    Ok(warp::reply::html(html).into_response())
}

/// Handle documents by tag
async fn handle_tag(tag: String, state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    let docs: Vec<DocumentInfo> = report
        .documents
        .values()
        .filter(|doc| doc.tags.contains(&tag) && doc.status != Status::Meta)
        .map(|doc| DocumentInfo {
            title: doc.title.clone(),
            status: doc.status.to_string(),
            created: doc.created.format("%Y-%m-%d").to_string(),
        })
        .collect();

    let html = tag_page_template(&tag, &docs);
    Ok(warp::reply::html(html).into_response())
}

/// Handle graph page
async fn handle_graph(state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    let docs: Vec<DocumentInfo> = report
        .documents
        .values()
        .filter(|doc| doc.status != Status::Meta)
        .map(|doc| DocumentInfo {
            title: doc.title.clone(),
            status: doc.status.to_string(),
            created: doc.created.format("%Y-%m-%d").to_string(),
        })
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
async fn handle_search(
    query: SearchQuery,
    state: WikiState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;
    let query_lower = query.q.to_lowercase();

    let mut results: Vec<SearchResult> = Vec::new();

    for doc in report.documents.values() {
        if doc.status == Status::Meta {
            continue;
        }

        let match_type = if doc.title.to_lowercase().contains(&query_lower) {
            "title"
        } else if doc.body.to_lowercase().contains(&query_lower) {
            "content"
        } else if doc.tags.iter().any(|t| t.to_lowercase().contains(&query_lower)) {
            "tag"
        } else if doc.aliases.iter().any(|a| a.to_lowercase().contains(&query_lower)) {
            "alias"
        } else {
            continue;
        };

        results.push(SearchResult {
            title: doc.title.clone(),
            match_type: match_type.to_string(),
        });
    }

    // Sort: title matches first, then content, then tags/aliases
    results.sort_by(|a, b| {
        let order = |t: &str| match t {
            "title" => 0,
            "content" => 1,
            "tag" => 2,
            "alias" => 3,
            _ => 4,
        };
        order(&a.match_type).cmp(&order(&b.match_type))
    });

    let html = search_results_template(&query.q, &results);
    Ok(warp::reply::html(html).into_response())
}

/// Handle individual page
async fn handle_page(title: String, state: WikiState) -> Result<impl warp::Reply, warp::Rejection> {
    let report = state.report.read().await;

    // Try to find document by exact title or alias
    let doc = find_document(&report, &title);

    match doc {
        Some(doc) => {
            let html_body = render_markdown(&doc.body);
            let created = doc.created.format("%Y-%m-%d").to_string();

            // Get backlinks
            let backlinks: Vec<String> = report
                .link_graph
                .incoming_links
                .get(&doc.title)
                .map(|links| {
                    links
                        .iter()
                        .map(|l| l.source_file.trim_end_matches(".md").to_string())
                        .collect()
                })
                .unwrap_or_default();

            let html = page_template(
                &doc.title,
                &html_body,
                &doc.status.to_string(),
                &doc.tags,
                &created,
                &doc.aliases,
                &backlinks,
            );

            Ok(warp::reply::html(html).into_response())
        }
        None => {
            let html = not_found_template(&title);
            let mut response = warp::reply::html(html).into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

/// Find document by title or alias
fn find_document<'a>(report: &'a ScanReport, title: &str) -> Option<&'a Document> {
    // First try exact title match
    if let Some(doc) = report.documents.get(title) {
        return Some(doc);
    }

    // Try URL-decoded title
    let decoded = urlencoding::decode(title).ok()?.to_string();
    if let Some(doc) = report.documents.get(&decoded) {
        return Some(doc);
    }

    // Try matching by alias
    for doc in report.documents.values() {
        if doc.aliases.iter().any(|a| {
            a.to_lowercase() == title.to_lowercase()
                || urlencoding::decode(a).map(|d| d.to_string()).unwrap_or_default().to_lowercase() == decoded.to_lowercase()
        }) {
            return Some(doc);
        }
    }

    None
}
