use crate::types::{Document, LinkGraph, Status, WikiIndex};
use std::collections::HashMap;

/// Scan report containing wiki statistics
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScanReport {
    /// Total document counts by status
    pub counts: DocumentCounts,
    /// Stub candidates: [[target]] with no matching file
    pub stub_candidates: Vec<StubCandidate>,
    /// Disambiguation candidates: titles appearing in multiple documents
    pub disambig_candidates: Vec<DisambigCandidate>,
    /// Broken links: linked document exists but has no frontmatter
    pub broken_links: Vec<String>,
    /// Tag statistics
    pub tag_stats: TagStats,
    /// Wiki index
    pub wiki_index: WikiIndex,
    /// Link graph
    pub link_graph: LinkGraph,
    /// All documents by filename
    pub documents: HashMap<String, Document>,
}

#[derive(Debug, Clone)]
pub struct DocumentCounts {
    pub total: usize,
    pub published: usize,
    pub stubs: usize,
    pub disambiguation: usize,
    pub meta: usize,
}

#[derive(Debug, Clone)]
pub struct StubCandidate {
    pub target: String,
    pub inbound_count: usize,
}

#[derive(Debug, Clone)]
pub struct DisambigCandidate {
    pub title: String,
    pub documents: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TagStats {
    pub unique_tags: usize,
    pub tag_counts: Vec<(String, usize)>,
}

/// Build a scan report from scanned data
pub fn build_report(
    documents: HashMap<String, Document>,
    link_graph: LinkGraph,
    wiki_index: WikiIndex,
) -> ScanReport {
    // Count documents by status
    let mut counts = DocumentCounts {
        total: 0,
        published: 0,
        stubs: 0,
        disambiguation: 0,
        meta: 0,
    };

    for doc in documents.values() {
        counts.total += 1;
        match doc.status {
            Status::Published => counts.published += 1,
            Status::Stub => counts.stubs += 1,
            Status::Disambiguation => counts.disambiguation += 1,
            Status::Meta => counts.meta += 1,
        }
    }

    // Find stub candidates: links to targets without matching documents
    let existing_titles: Vec<&str> = wiki_index.titles();
    let mut stub_candidates: Vec<StubCandidate> = Vec::new();

    for (target, links) in &link_graph.incoming_links {
        // Check if target exists
        let exists = existing_titles.iter().any(|t| t.eq_ignore_ascii_case(target));
        if !exists {
            stub_candidates.push(StubCandidate {
                target: target.clone(),
                inbound_count: links.len(),
            });
        }
    }

    // Sort by inbound count descending
    stub_candidates.sort_by(|a, b| b.inbound_count.cmp(&a.inbound_count));

    // Find disambiguation candidates: titles appearing in multiple documents
    let mut title_occurrences: HashMap<String, Vec<String>> = HashMap::new();
    for (filename, doc) in &documents {
        if doc.status != Status::Disambiguation && doc.status != Status::Meta {
            title_occurrences
                .entry(doc.title.clone())
                .or_insert_with(Vec::new)
                .push(filename.clone());
        }
    }

    let disambig_candidates: Vec<DisambigCandidate> = title_occurrences
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(title, documents)| DisambigCandidate { title, documents })
        .collect();

    // Build tag statistics
    let mut tag_counts_map: HashMap<String, usize> = HashMap::new();
    for doc in documents.values() {
        for tag in &doc.tags {
            *tag_counts_map.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    let mut tag_counts: Vec<(String, usize)> = tag_counts_map.into_iter().collect();
    tag_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let tag_stats = TagStats {
        unique_tags: tag_counts.len(),
        tag_counts,
    };

    // Find broken links (links to files without frontmatter - for now, empty)
    let broken_links = Vec::new();

    ScanReport {
        counts,
        stub_candidates,
        disambig_candidates,
        broken_links,
        tag_stats,
        wiki_index,
        link_graph,
        documents,
    }
}

impl ScanReport {
    /// Print the report
    pub fn print(&self) {
        println!("📊 Wiki Status");
        println!("   Total documents  : {}", self.counts.total);
        println!("   Published        : {}", self.counts.published);
        println!("   Stubs            : {}", self.counts.stubs);
        println!("   Disambiguation   : {}", self.counts.disambiguation);
        println!();

        println!("🏷️  Tags");
        println!("   Unique tags      : {}", self.tag_stats.unique_tags);
        if !self.tag_stats.tag_counts.is_empty() {
            let top: Vec<&(String, usize)> = self.tag_stats.tag_counts.iter().take(3).collect();
            let top_str: Vec<String> = top.iter().map(|(t, c)| format!("{} ({})", t, c)).collect();
            println!("   Most used        : {}", top_str.join(", "));
        }
        println!();

        println!("🔗 Links");
        let total_links: usize = self.link_graph.incoming_links.values().map(|v| v.len()).sum();
        println!("   Total links      : {}", total_links);
        println!("   Stub targets     : {}", self.stub_candidates.len());
        println!();

        if !self.disambig_candidates.is_empty() {
            println!("⚠️  Action required");
            let disambig_names: Vec<&str> = self.disambig_candidates.iter().map(|d| d.title.as_str()).collect();
            println!("   Disambiguation   : {}", disambig_names.join(", "));
        }

        if !self.broken_links.is_empty() {
            println!("   Broken links     : {}", self.broken_links.join(", "));
        }
    }

    /// Print a compact status summary
    pub fn print_status(&self) {
        println!("Documents: {} ({} published, {} stubs, {} disambig)",
            self.counts.total,
            self.counts.published,
            self.counts.stubs,
            self.counts.disambiguation
        );
        println!("Tags: {} | Links: {} | Stubs needed: {}",
            self.tag_stats.unique_tags,
            self.link_graph.incoming_links.values().map(|v| v.len()).sum::<usize>(),
            self.stub_candidates.len()
        );
    }
}
