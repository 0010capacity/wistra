pub mod parser;
pub mod graph;
pub mod report;
pub mod meta;

pub use report::{ScanReport, DisambigCandidate, StubCandidate};

use crate::config::WikiConfig;
use crate::types::{Document, LinkGraph, WikiIndex};
use anyhow::Result;
use walkdir::WalkDir;
use std::collections::HashMap;

/// Scan the wiki and build the link graph
pub fn scan_wiki(wiki_config: &WikiConfig) -> Result<ScanReport> {
    let concepts_dir = wiki_config.concepts_dir();
    let mut documents: HashMap<String, Document> = HashMap::new();
    let mut link_graph = LinkGraph::new();
    let mut wiki_index = WikiIndex::new();

    // Walk the concepts directory
    if concepts_dir.exists() {
        for entry in WalkDir::new(&concepts_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "md" {
                        if let Ok(Some(doc)) = parser::parse_document(entry.path()) {
                            // Extract links from body
                            let links = parser::extract_links(&doc.body, entry.path());
                            for link in links {
                                link_graph.add_link(link);
                            }

                            // Add to wiki index
                            let summary = parser::extract_summary(&doc.body);
                            wiki_index.entries.push(crate::types::WikiIndexEntry {
                                title: doc.title.clone(),
                                tags: doc.tags.clone(),
                                aliases: doc.aliases.clone(),
                                summary,
                                status: doc.status.clone(),
                            });

                            let filename = entry.file_name().to_string_lossy().to_string();
                            documents.insert(filename, doc);
                        }
                    }
                }
            }
        }
    }

    // Build the scan report
    let report = report::build_report(documents, link_graph, wiki_index);
    Ok(report)
}
