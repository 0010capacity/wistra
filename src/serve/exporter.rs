/// Static site exporter for Firebase Hosting and Cloudflare Pages

use crate::config::WikiConfig;
use crate::scanner;
use crate::scanner::ScanReport;
use crate::serve::renderer::{extract_headings, extract_summary, render_markdown};
use crate::serve::templates::{
    all_pages_template, graph_template, home_template, not_found_template,
    page_template, search_results_template, tag_page_template, tags_template,
};
use crate::serve::{DocumentInfo, SearchResultInfo};
use crate::types::{Document, Status};
use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::path::Path;

/// Hosting target options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HostingTarget {
    Firebase,
    Cloudflare,
    Both,
}

/// Export the wiki as a static site
pub fn export(
    wiki_path: &Path,
    output_dir: &Path,
    target: HostingTarget,
    project_name: &str,
) -> Result<()> {
    // Load wiki config and scan
    let wiki_path = wiki_path.to_path_buf();
    let config = WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&config)?;

    // Create output directory
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    println!("📦 Exporting wiki to {}", output_dir.display());

    // ── Build shared data ──
    let docs: Vec<DocumentInfo> = report
        .documents
        .iter()
        .filter(|(_, d)| d.status != Status::Meta)
        .map(|(_, d)| doc_to_info(d, &report))
        .collect();

    let tags: Vec<(String, usize)> = report.tag_stats.tag_counts.clone();

    // ── Render pages ──
    render_home(&output_dir, &docs, report.counts.total, tags.len())?;
    render_all_pages(&output_dir, &docs)?;
    render_tags(&output_dir, &tags)?;
    render_graph(&output_dir, &docs, &report)?;

    // Individual pages
    for doc in &docs {
        render_page(output_dir, doc, &report)?;
    }

    // Tag pages
    let used_tags: HashSet<&String> = docs.iter().flat_map(|d| d.tags.iter()).collect();
    let used_tags_count = used_tags.len();
    for tag in used_tags {
        let tag_docs: Vec<DocumentInfo> = docs
            .iter()
            .filter(|d| d.tags.contains(tag))
            .cloned()
            .collect();
        render_tag_page(output_dir, tag, &tag_docs)?;
    }

    // 404 page
    render_404(output_dir)?;

    println!(
        "   {} pages, {} tag pages exported",
        docs.len() + 4,
        used_tags_count
    );

    // ── Generate hosting config ──
    match target {
        HostingTarget::Both => {
            render_firebase_config(output_dir)?;
            println!("   firebase.json generated");
            render_cloudflare_redirects(output_dir)?;
            println!("   _redirects generated");
        }
        HostingTarget::Firebase => {
            render_firebase_config(output_dir)?;
            println!("   firebase.json generated");
        }
        HostingTarget::Cloudflare => {
            render_cloudflare_redirects(output_dir)?;
            println!("   _redirects generated");
        }
    }

    println!();
    println!("🌐 Deploy to:");
    match target {
        HostingTarget::Firebase => {
            println!("   Firebase: https://{}.web.app", project_name);
            println!("              firebase deploy --only hosting");
        }
        HostingTarget::Cloudflare => {
            println!("   Cloudflare: https://{}.pages.dev", project_name);
            println!("               wrangler pages deploy {}", output_dir.display());
        }
        HostingTarget::Both => {
            println!("   Firebase:  https://{}.web.app", project_name);
            println!("              firebase deploy --only hosting");
            println!("   Cloudflare: https://{}.pages.dev", project_name);
            println!("               wrangler pages deploy {}", output_dir.display());
        }
    }
    println!("✅ Export complete!");
    Ok(())
}

// ---------------------------------------------------------------------------
// Page renderers
// ---------------------------------------------------------------------------

fn render_home(output_dir: &Path, docs: &[DocumentInfo], total: usize, tag_count: usize) -> Result<()> {
    let mut sorted = docs.to_vec();
    sorted.sort_by(|a, b| b.created.cmp(&a.created));
    let recent: Vec<&DocumentInfo> = sorted.iter().take(5).collect();
    let random = sorted.choose(&mut rand::thread_rng());

    let published = docs.iter().filter(|d| d.status == "published").count();
    let stubs = docs.iter().filter(|d| d.status == "stub").count();

    let html = home_template(&recent, random, total, published, stubs, tag_count);
    write_file(output_dir.join("index.html"), &html)
}

fn render_all_pages(output_dir: &Path, docs: &[DocumentInfo]) -> Result<()> {
    let html = all_pages_template(docs, "grid", None, None, None);
    write_file(output_dir.join("all").join("index.html"), &html)
}

fn render_tags(output_dir: &Path, tags: &[(String, usize)]) -> Result<()> {
    let html = tags_template(tags);
    write_file(output_dir.join("tags").join("index.html"), &html)
}

fn render_graph(output_dir: &Path, docs: &[DocumentInfo], report: &ScanReport) -> Result<()> {
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

    let html = graph_template(docs, &links);
    write_file(output_dir.join("graph").join("index.html"), &html)
}

fn render_page(output_dir: &Path, doc: &DocumentInfo, report: &ScanReport) -> Result<()> {
    let html_body = match find_doc_body(&report, &doc.title) {
        Some(body) => render_markdown(&body),
        None => String::new(),
    };

    let headings = extract_headings(&html_body);
    let html = page_template(doc, &html_body, &headings);

    let slug = slugify(&doc.title);
    write_file(output_dir.join("page").join(&slug).join("index.html"), &html)
}

fn render_tag_page(output_dir: &Path, tag: &str, docs: &[DocumentInfo]) -> Result<()> {
    let html = tag_page_template(tag, docs);
    let slug = slugify(tag);
    write_file(output_dir.join("tag").join(&slug).join("index.html"), &html)
}

fn render_404(output_dir: &Path) -> Result<()> {
    let html = not_found_template("Page not found");
    write_file(output_dir.join("404.html"), &html)
}

// ---------------------------------------------------------------------------
// Hosting config generators
// ---------------------------------------------------------------------------

fn render_firebase_config(output_dir: &Path) -> Result<()> {
    let config = r#"{
  "hosting": {
    "public": ".",
    "ignore": [
      "firebase.json",
      "**/.*",
      "**/node_modules/**"
    ],
    "rewrites": [
      {
        "source": "/",
        "destination": "/index.html"
      },
      {
        "source": "/all",
        "destination": "/all/index.html"
      },
      {
        "source": "/tags",
        "destination": "/tags/index.html"
      },
      {
        "source": "/graph",
        "destination": "/graph/index.html"
      },
      {
        "source": "/page/:slug",
        "destination": "/page/:slug/index.html"
      },
      {
        "source": "/tag/:slug",
        "destination": "/tag/:slug/index.html"
      },
      {
        "source": "/search",
        "destination": "/index.html"
      }
    ]
  }
}
"#;
    write_file(output_dir.join("firebase.json"), config)
}

fn render_cloudflare_redirects(output_dir: &Path) -> Result<()> {
    let redirects = r#"# Cloudflare Pages SPA redirects
/all/*    /all/index.html   200
/tags/*   /tags/index.html  200
/graph/*  /graph/index.html 200
/page/*   /page/:slug/index.html   200
/tag/*    /tag/:slug/index.html   200
/search/* /index.html      200
# Fallback for direct page access
/*        /index.html      200
"#;
    write_file(output_dir.join("_redirects"), redirects)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn doc_to_info(doc: &Document, report: &ScanReport) -> DocumentInfo {
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

fn find_doc_body(report: &ScanReport, title: &str) -> Option<String> {
    report
        .documents
        .get(title)
        .map(|d| d.body.clone())
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else if c.is_whitespace() || c == '/' {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn write_file(path: impl AsRef<Path>, content: impl AsRef<str>) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    std::fs::write(path, content.as_ref())
        .with_context(|| format!("Failed to write file: {}", path.display()))?;
    Ok(())
}

