mod adapter;
mod cli;
mod config;
mod importer;
mod planner;
mod scanner;
mod serve;
mod types;
mod writer;

use crate::adapter::{SuggestionContext, WikiAdapter};
use crate::scanner::DisambigCandidate;
use crate::types::Link;
use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wistra")]
#[command(about = "AI-powered personal wiki builder", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        None => {
            // Default: show help
            println!("wistra - AI-powered personal wiki builder");
            println!();
            println!("Commands:");
            println!("  onboard     Run the setup wizard");
            println!("  run         Grow the wiki");
            println!("  scan        Scan and report wiki status");
            println!("  config      Modify configuration");
            println!("  status      Print compact status summary");
            println!("  interests   Modify interest domains");
            println!();
            println!("Run `wistra <command> --help` for more information.");
        }
        Some(cli::Commands::Onboard) => {
            cli::onboard::run_onboard()?;
        }
        Some(cli::Commands::Config { onboard }) => {
            cli::config::run_config(onboard)?;
        }
        Some(cli::Commands::Interests) => {
            run_interests()?;
        }
        Some(cli::Commands::Scan { path }) => {
            run_scan(&path)?;
        }
        Some(cli::Commands::Status { path }) => {
            run_status(&path)?;
        }
        Some(cli::Commands::Run { path, count, dry_run, quiet, no_confirm, no_git }) => {
            run_wiki_growth(&path, count, dry_run, quiet, no_confirm, !no_git).await?;
        }
        Some(cli::Commands::Rename { old_title, new_title, path, dry_run }) => {
            run_rename(&path, &old_title, &new_title, dry_run)?;
        }
        Some(cli::Commands::Merge { source, target, path, dry_run }) => {
            run_merge(&path, &source, &target, dry_run)?;
        }
        Some(cli::Commands::Delete { title, path, dry_run, no_confirm }) => {
            run_delete(&path, &title, dry_run, no_confirm)?;
        }
        Some(cli::Commands::Backlinks { title, path, json }) => {
            run_backlinks(&path, &title, json)?;
        }
        Some(cli::Commands::Orphans { path, exclude_stubs, sort, json }) => {
            run_orphans(&path, exclude_stubs, &sort, json)?;
        }
        Some(cli::Commands::Search { query, path, case_sensitive, regex, json }) => {
            run_search(&path, &query, case_sensitive, regex, json)?;
        }
        Some(cli::Commands::Tags { action, path }) => {
            run_tags(&path, action)?;
        }
        Some(cli::Commands::Graph { title, path, depth, incoming, outgoing, json }) => {
            run_graph(&path, &title, depth, incoming, outgoing, json)?;
        }
        Some(cli::Commands::Stats { stat_type, path, json }) => {
            run_stats(&path, &stat_type, json)?;
        }
        Some(cli::Commands::Clean { path, dry_run, fix, json }) => {
            run_clean(&path, dry_run, fix, json)?;
        }
        Some(cli::Commands::Import { source, path, dry_run, json }) => {
            run_import(&source, &path, dry_run, json)?;
        }
        Some(cli::Commands::Serve { path, port, host, open }) => {
            serve::serve(&path, &host, port, open).await?;
        }
        Some(cli::Commands::Dedup { path, threshold, json }) => {
            run_dedup(&path, threshold, json)?;
        }
        Some(cli::Commands::Cron { set, show, remove, install, no_git }) => {
            run_cron(set.as_deref(), show, remove, install, no_git)?;
        }
    }

    Ok(())
}

fn run_import(source: &str, path: &str, dry_run: bool, json: bool) -> Result<()> {
    let source_path = std::path::PathBuf::from(source);
    let wiki_path = PathBuf::from(shellexpand::tilde(path).to_string());
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    importer::import_path(&source_path, &wiki_config, dry_run, json)?;
    Ok(())
}

fn run_interests() -> Result<()> {
    use dialoguer::MultiSelect;
    use crate::config::{GlobalConfig, INTEREST_DOMAINS};

    let mut config = GlobalConfig::load()?
        .context("No config found. Run `wistra onboard` first.")?;

    // Build a Vec<bool> indicating which items are selected
    let defaults: Vec<bool> = (0..INTEREST_DOMAINS.len())
        .map(|idx| config.interests.contains(&INTEREST_DOMAINS[idx].0.to_string()))
        .collect();

    let interest_items: Vec<&str> = INTEREST_DOMAINS.iter().map(|(_, name)| *name).collect();
    let selected_indices = MultiSelect::new()
        .with_prompt("Interests (space to select)")
        .items(&interest_items)
        .defaults(&defaults)
        .interact()
        .context("Failed to select interests")?;

    config.interests = selected_indices
        .iter()
        .map(|&idx| INTEREST_DOMAINS[idx].0.to_string())
        .collect();

    config.save()?;
    println!("✅ Interests updated!");

    Ok(())
}

fn run_scan(path: &str) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    report.print();

    Ok(())
}

fn run_status(path: &str) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    report.print_status();

    Ok(())
}

fn run_rename(path: &str, old_title: &str, new_title: &str, dry_run: bool) -> Result<()> {
    use crate::scanner::parser;

    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let concepts_dir = wiki_config.concepts_dir();

    // Find the document
    let filename = format!("{}.md", old_title);
    let old_path = concepts_dir.join(&filename);

    if !old_path.exists() {
        anyhow::bail!("Document not found: {}", old_title);
    }

    let _doc = parser::parse_document(&old_path)?
        .context("Failed to parse document")?;

    // Preview changes
    let new_filename = format!("{}.md", new_title);
    let new_path = concepts_dir.join(&new_filename);

    println!("📝 Rename: {} → {}", old_title, new_title);
    println!("   File: {} → {}", old_path.display(), new_path.display());

    // Check for conflicts
    if new_path.exists() && old_path != new_path {
        anyhow::bail!("Target file already exists: {}", new_title);
    }

    if dry_run {
        println!("🏃 Dry run - no changes made.");
        return Ok(());
    }

    // Update frontmatter title in content
    let content = std::fs::read_to_string(&old_path)?;
    let new_content = content.replace(
        &format!("title: {}", serde_json::to_string(old_title).unwrap()),
        &format!("title: {}", serde_json::to_string(new_title).unwrap()),
    );

    // Rename the file
    std::fs::write(&old_path, &new_content)?;
    std::fs::rename(&old_path, &new_path)?;

    // Update all links pointing to the old title
    let mut updates = std::collections::HashMap::new();
    updates.insert(old_title.to_string(), new_title.to_string());
    let link_count = writer::Linker::rewrite_links(&wiki_path, &updates)?;

    println!("✅ Renamed successfully.");
    println!("   Links updated: {}", link_count);

    // Re-scan to update meta files
    let report = scanner::scan_wiki(&wiki_config)?;
    scanner::meta::generate_meta_files(&wiki_config, &report)?;

    Ok(())
}

fn run_merge(path: &str, source: &str, target: &str, dry_run: bool) -> Result<()> {
    use crate::scanner::parser;

    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let concepts_dir = wiki_config.concepts_dir();

    // Find source document
    let source_filename = format!("{}.md", source);
    let source_path = concepts_dir.join(&source_filename);

    if !source_path.exists() {
        anyhow::bail!("Source document not found: {}", source);
    }

    // Find target document
    let target_filename = format!("{}.md", target);
    let target_path = concepts_dir.join(&target_filename);

    if !target_path.exists() {
        anyhow::bail!("Target document not found: {}", target);
    }

    let source_doc = parser::parse_document(&source_path)?
        .context("Failed to parse source document")?;
    let mut target_doc = parser::parse_document(&target_path)?
        .context("Failed to parse target document")?;

    println!("🔀 Merge: {} → {}", source, target);
    println!("   Source: {}", source_path.display());
    println!("   Target: {}", target_path.display());

    // Merge tags (combine, dedupe)
    for tag in &source_doc.tags {
        if !target_doc.tags.contains(tag) {
            target_doc.tags.push(tag.clone());
        }
    }

    // Merge aliases (combine, dedupe)
    for alias in &source_doc.aliases {
        if !target_doc.aliases.contains(alias) {
            target_doc.aliases.push(alias.clone());
        }
    }

    println!("   Tags: {} → {}", source_doc.tags.len(), target_doc.tags.len());
    println!("   Aliases: {} → {}", source_doc.aliases.len(), target_doc.aliases.len());

    if dry_run {
        println!("🏃 Dry run - no changes made.");
        return Ok(());
    }

    // Update target document
    writer::DocumentWriter::write(&target_doc, &concepts_dir)?;

    // Delete source document
    std::fs::remove_file(&source_path)?;

    // Update all links from source to target
    let mut updates = std::collections::HashMap::new();
    updates.insert(source.to_string(), target.to_string());
    let link_count = writer::Linker::rewrite_links(&wiki_path, &updates)?;

    println!("✅ Merged successfully.");
    println!("   Links updated: {}", link_count);

    // Re-scan to update meta files
    let report = scanner::scan_wiki(&wiki_config)?;
    scanner::meta::generate_meta_files(&wiki_config, &report)?;

    Ok(())
}

fn run_delete(path: &str, title: &str, dry_run: bool, no_confirm: bool) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let concepts_dir = wiki_config.concepts_dir();

    // Find the document
    let filename = format!("{}.md", title);
    let doc_path = concepts_dir.join(&filename);

    if !doc_path.exists() {
        anyhow::bail!("Document not found: {}", title);
    }

    // Scan to find linking documents
    let report = scanner::scan_wiki(&wiki_config)?;

    let incoming_links = report.link_graph.incoming_links.get(title);

    println!("🗑️  Delete: {}", title);
    println!("   File: {}", doc_path.display());

    if let Some(links) = incoming_links {
        println!("   ⚠️  {} documents link to this:", links.len());
        for link in links {
            println!("      - [[{}]]", link.source_file.replace(".md", ""));
        }
    } else {
        println!("   No incoming links.");
    }

    if dry_run {
        println!("🏃 Dry run - no changes made.");
        return Ok(());
    }

    if !no_confirm {
        use dialoguer::Confirm;
        let proceed = Confirm::new()
            .with_prompt("Delete this document?")
            .default(false)
            .interact()?;
        if !proceed {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Delete the document
    std::fs::remove_file(&doc_path)?;

    // Update linking documents to remove links
    if let Some(links) = incoming_links {
        let mut updates = std::collections::HashMap::new();
        updates.insert(title.to_string(), title.to_string()); // replace with self = remove effect

        // Actually we want to just rewrite links to plain text, not redirect
        // For now, just note that links remain
        println!("   ⚠️  {} documents still have [[{}]] links", links.len(), title);
    }

    println!("✅ Deleted successfully.");

    // Re-scan to update meta files
    let report = scanner::scan_wiki(&wiki_config)?;
    scanner::meta::generate_meta_files(&wiki_config, &report)?;

    Ok(())
}

/// Run the backlinks command - show documents that link to a specific document
fn run_backlinks(path: &str, title: &str, json: bool) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    // Find incoming links for the target title
    let incoming = report.link_graph.incoming_links.get(title);

    if json {
        let sources: Vec<String> = incoming
            .map(|links| links.iter().map(|l| l.source_file.replace(".md", "")).collect())
            .unwrap_or_default();
        let output = serde_json::json!({
            "title": title,
            "count": sources.len(),
            "sources": sources
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        match incoming {
            Some(links) if !links.is_empty() => {
                println!("{} documents link to [[{}]]:", links.len(), title);
                for link in links {
                    println!("  - [[{}]]", link.source_file.replace(".md", ""));
                }
            }
            _ => {
                println!("No documents link to [[{}]]", title);
            }
        }
    }

    Ok(())
}

/// Run the orphans command - find documents with no incoming links
fn run_orphans(path: &str, exclude_stubs: bool, sort: &str, json: bool) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    // Find documents with no incoming links
    let mut orphans: Vec<(&String, &types::Document)> = report.documents
        .iter()
        .filter(|(_, doc)| {
            // Skip stubs if requested
            if exclude_stubs && doc.status == types::Status::Stub {
                return false;
            }

            // Check if there are no incoming links
            match report.link_graph.incoming_links.get(&doc.title) {
                None => true,
                Some(links) => links.is_empty(),
            }
        })
        .collect();

    // Sort orphans
    match sort {
        "created" => {
            orphans.sort_by(|a, b| b.1.created.cmp(&a.1.created));
        }
        "tags" => {
            orphans.sort_by(|a, b| a.1.tags.len().cmp(&b.1.tags.len()));
        }
        _ => {
            // Default: sort by title
            orphans.sort_by(|a, b| a.1.title.cmp(&b.1.title));
        }
    }

    if json {
        let orphan_list: Vec<serde_json::Value> = orphans
            .iter()
            .map(|(_, doc)| {
                serde_json::json!({
                    "title": doc.title,
                    "status": doc.status.to_string(),
                    "tags": doc.tags,
                    "created": doc.created.to_string()
                })
            })
            .collect();
        let output = serde_json::json!({
            "count": orphan_list.len(),
            "orphans": orphan_list
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        if orphans.is_empty() {
            println!("No orphan documents found.");
        } else {
            println!("{} orphan documents (no incoming links):", orphans.len());
            for (_, doc) in &orphans {
                println!("  - [[{}]] ({})", doc.title, doc.status);
            }
        }
    }

    Ok(())
}

/// Run the search command - full text search across documents
fn run_search(path: &str, query: &str, case_sensitive: bool, use_regex: bool, json: bool) -> Result<()> {
    use regex::RegexBuilder;

    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    let pattern: String = if use_regex {
        query.to_string()
    } else {
        // Escape special regex characters for literal search
        regex::escape(query)
    };

    let regex = RegexBuilder::new(&pattern)
        .case_insensitive(!case_sensitive)
        .build()
        .context("Invalid search pattern")?;

    let mut results: Vec<(String, String, &types::Document)> = Vec::new();

    for (_, doc) in &report.documents {
        let mut match_type = String::new();

        // Check title
        if regex.is_match(&doc.title) {
            match_type = "title".to_string();
        }
        // Check body
        else if regex.is_match(&doc.body) {
            match_type = "body".to_string();
        }
        // Check tags
        else if doc.tags.iter().any(|t| regex.is_match(t)) {
            match_type = "tags".to_string();
        }
        // Check aliases
        else if doc.aliases.iter().any(|a| regex.is_match(a)) {
            match_type = "alias".to_string();
        }

        if !match_type.is_empty() {
            results.push((doc.title.clone(), match_type, doc));
        }
    }

    // Sort results: title matches first, then body, then tags/aliases
    results.sort_by(|a, b| {
        let order = |t: &str| match t {
            "title" => 0,
            "body" => 1,
            "tags" => 2,
            "alias" => 3,
            _ => 4,
        };
        order(&a.1).cmp(&order(&b.1))
    });

    if json {
        let result_list: Vec<serde_json::Value> = results
            .iter()
            .map(|(title, match_type, doc)| {
                serde_json::json!({
                    "title": title,
                    "match": match_type,
                    "status": doc.status.to_string(),
                    "tags": doc.tags
                })
            })
            .collect();
        let output = serde_json::json!({
            "query": query,
            "count": result_list.len(),
            "results": result_list
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        if results.is_empty() {
            println!("No results for \"{}\"", query);
        } else {
            println!("{} results for \"{}\":", results.len(), query);
            for (title, match_type, _) in &results {
                println!("  - [[{}]] ({} match)", title, match_type);
            }
        }
    }

    Ok(())
}

/// Run the tags command - tag management
fn run_tags(path: &str, action: cli::TagAction) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let concepts_dir = wiki_config.concepts_dir();
    let report = scanner::scan_wiki(&wiki_config)?;

    match action {
        cli::TagAction::List => {
            // List all tags with document counts
            let mut tag_counts = report.tag_stats.tag_counts.clone();
            tag_counts.sort_by(|a, b| b.1.cmp(&a.1));

            if tag_counts.is_empty() {
                println!("No tags found.");
            } else {
                println!("{} tags:", tag_counts.len());
                for (tag, count) in tag_counts {
                    println!("  {} ({} documents)", tag, count);
                }
            }
        }
        cli::TagAction::Rename { old_tag, new_tag, dry_run } => {
            println!("🏷️  Rename tag: {} → {}", old_tag, new_tag);

            // Find documents with the old tag
            let mut affected: Vec<&types::Document> = Vec::new();
            for doc in report.documents.values() {
                if doc.tags.contains(&old_tag) {
                    affected.push(doc);
                }
            }

            println!("   {} documents have this tag", affected.len());

            if dry_run {
                println!("🏃 Dry run - no changes made.");
                return Ok(());
            }

            // Update each document
            for doc in affected {
                let filepath = concepts_dir.join(doc.filename());
                let content = std::fs::read_to_string(&filepath)?;
                let new_content = content.replace(&format!("\"{}\"", old_tag), &format!("\"{}\"", new_tag));
                std::fs::write(&filepath, new_content)?;
            }

            println!("✅ Tag renamed successfully.");
        }
        cli::TagAction::Merge { source, target, dry_run } => {
            println!("🏷️  Merge tag: {} → {}", source, target);

            // Find documents with the source tag
            let mut affected: Vec<&types::Document> = Vec::new();
            for doc in report.documents.values() {
                if doc.tags.contains(&source) {
                    affected.push(doc);
                }
            }

            println!("   {} documents have this tag", affected.len());

            if dry_run {
                println!("🏃 Dry run - no changes made.");
                return Ok(());
            }

            // Update each document: remove source, add target if not present
            for doc in affected {
                let filepath = concepts_dir.join(doc.filename());
                let mut content = std::fs::read_to_string(&filepath)?;

                // Remove source tag
                content = content.replace(&format!("\"{}\"", source), &format!("\"{}\"", target));

                // If target doesn't already exist in tags, we need to add it properly
                // For simplicity, just replace source with target in tags list
                std::fs::write(&filepath, content)?;
            }

            println!("✅ Tag merged successfully.");
        }
        cli::TagAction::Orphans => {
            // Find all tags that have no documents
            let mut all_tags: std::collections::HashSet<&String> = std::collections::HashSet::new();
            let mut used_tags: std::collections::HashSet<&String> = std::collections::HashSet::new();

            for (tag, _) in &report.tag_stats.tag_counts {
                all_tags.insert(tag);
                used_tags.insert(tag);
            }

            let orphan_tags: Vec<&&String> = all_tags.difference(&used_tags).collect();

            if orphan_tags.is_empty() {
                println!("No orphan tags found.");
            } else {
                println!("{} orphan tags (not used by any document):", orphan_tags.len());
                for tag in orphan_tags {
                    println!("  - {}", tag);
                }
            }
        }
    }

    Ok(())
}

/// Run the graph command - show document connection graph
fn run_graph(path: &str, title: &str, max_depth: usize, incoming_only: bool, outgoing_only: bool, json: bool) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    // BFS traversal
    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut queue: Vec<(String, usize)> = vec![(title.to_string(), 0)];
    let mut results: Vec<(String, usize)> = Vec::new();

    while let Some((current, depth)) = queue.pop() {
        if depth > max_depth || visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        results.push((current.clone(), depth));

        if depth < max_depth {
            // Get linked documents
            let links: Vec<&Link> = if !incoming_only {
                report.link_graph.outgoing_links
                    .get(&format!("{}.md", current))
                    .map(|l| l.as_slice())
                    .unwrap_or(&[])
                    .iter()
                    .collect()
            } else {
                Vec::new()
            };

            let incoming: Vec<&Link> = if !outgoing_only {
                report.link_graph.incoming_links
                    .get(&current)
                    .map(|l| l.as_slice())
                    .unwrap_or(&[])
                    .iter()
                    .collect()
            } else {
                Vec::new()
            };

            for link in links {
                let target = link.target.replace(".md", "");
                if !visited.contains(&target) {
                    queue.push((target, depth + 1));
                }
            }

            for link in incoming {
                let source = link.source_file.replace(".md", "");
                if !visited.contains(&source) {
                    queue.push((source, depth + 1));
                }
            }
        }
    }

    if json {
        let graph: Vec<serde_json::Value> = results
            .iter()
            .map(|(doc_title, d)| {
                serde_json::json!({
                    "title": doc_title,
                    "depth": d
                })
            })
            .collect();
        let output = serde_json::json!({
            "root": title,
            "depth": max_depth,
            "graph": graph
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("[[{}]]", title);
        for (doc_title, depth) in &results[1..] {
            let indent = "  ".repeat(*depth);
            println!("{}└── [[{}]]", indent, doc_title);
        }
    }

    Ok(())
}

/// Run the stats command - extended statistics
fn run_stats(path: &str, stat_type: &str, json: bool) -> Result<()> {
    use std::collections::HashMap;

    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    match stat_type {
        "basic" => {
            report.print_status();
        }
        "trends" => {
            // Group documents by creation month
            let mut monthly: HashMap<String, usize> = HashMap::new();
            for doc in report.documents.values() {
                let month = format!("{}-{:02}", doc.created.year(), doc.created.month());
                *monthly.entry(month).or_insert(0) += 1;
            }

            let mut months: Vec<(String, usize)> = monthly.into_iter().collect();
            months.sort_by(|a, b| a.0.cmp(&b.0));

            if json {
                let trends: Vec<serde_json::Value> = months
                    .iter()
                    .map(|(month, count)| {
                        serde_json::json!({
                            "month": month,
                            "count": count
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&trends)?);
            } else {
                println!("📈 Document creation trends:");
                for (month, count) in &months {
                    println!("  {}: {} documents", month, count);
                }
            }
        }
        "tags" => {
            let mut tag_docs: HashMap<String, Vec<&String>> = HashMap::new();
            for doc in report.documents.values() {
                for tag in &doc.tags {
                    tag_docs.entry(tag.clone()).or_insert_with(Vec::new).push(&doc.title);
                }
            }

            let mut tags: Vec<(&String, &Vec<&String>)> = tag_docs.iter().collect();
            tags.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

            if json {
                let tag_stats: Vec<serde_json::Value> = tags
                    .iter()
                    .map(|(tag, docs)| {
                        serde_json::json!({
                            "tag": tag,
                            "count": docs.len(),
                            "documents": docs
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&tag_stats)?);
            } else {
                println!("🏷️  Tag distribution:");
                for (tag, docs) in tags {
                    println!("  {} ({} documents): {}", tag, docs.len(), docs.iter().map(|s| s.as_str()).take(3).collect::<Vec<_>>().join(", "));
                }
            }
        }
        "links" => {
            let total_links: usize = report.link_graph.incoming_links.values().map(|v| v.len()).sum();
            let avg_links = if report.counts.total > 0 {
                total_links as f64 / report.counts.total as f64
            } else {
                0.0
            };

            let mut link_counts: Vec<usize> = report.documents.values()
                .map(|doc| {
                    let outgoing = report.link_graph.outgoing_links.get(&doc.filename())
                        .map(|l| l.len())
                        .unwrap_or(0);
                    let incoming = report.link_graph.incoming_links.get(&doc.title)
                        .map(|l| l.len())
                        .unwrap_or(0);
                    outgoing + incoming
                })
                .collect();
            link_counts.sort_by(|a, b| b.cmp(a));

            let max_links = link_counts.first().copied().unwrap_or(0);
            let min_links = link_counts.last().copied().unwrap_or(0);

            if json {
                let link_stats = serde_json::json!({
                    "total_links": total_links,
                    "average_links_per_doc": avg_links,
                    "max_links": max_links,
                    "min_links": min_links
                });
                println!("{}", serde_json::to_string_pretty(&link_stats)?);
            } else {
                println!("🔗 Link statistics:");
                println!("  Total links: {}", total_links);
                println!("  Average links per document: {:.2}", avg_links);
                println!("  Max links: {}", max_links);
                println!("  Min links: {}", min_links);
            }
        }
        _ => {
            anyhow::bail!("Unknown stats type: {}. Use basic, trends, tags, or links.", stat_type);
        }
    }

    Ok(())
}

/// Run the clean command - detect and fix wiki technical debt
fn run_clean(path: &str, dry_run: bool, fix: bool, json: bool) -> Result<()> {
    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let concepts_dir = wiki_config.concepts_dir();
    let report = scanner::scan_wiki(&wiki_config)?;

    // Collect issues
    let broken_links: Vec<&scanner::StubCandidate> = report.stub_candidates.iter().collect();

    // Find orphan tags (tags used in only 1 document - potential typos)
    let orphan_tags: Vec<(&String, usize)> = report.tag_stats.tag_counts
        .iter()
        .filter(|(_, count)| *count == 1)
        .map(|(tag, count)| (tag, *count))
        .collect();

    // Find empty stubs (stub documents with no real content)
    let empty_stubs: Vec<&types::Document> = report.documents
        .values()
        .filter(|doc| {
            if doc.status != types::Status::Stub {
                return false;
            }
            let body = doc.body.trim();
            body.is_empty() || body.starts_with("<!-- stub:")
        })
        .collect();

    // Find orphan documents (no incoming links)
    let orphan_docs: Vec<&types::Document> = report.documents
        .values()
        .filter(|doc| {
            match report.link_graph.incoming_links.get(&doc.title) {
                None => true,
                Some(links) => links.is_empty(),
            }
        })
        .collect();

    // Output results
    if json {
        let output = serde_json::json!({
            "broken_links": broken_links.iter().map(|s| serde_json::json!({
                "target": s.target,
                "inbound_count": s.inbound_count
            })).collect::<Vec<_>>(),
            "orphan_tags": orphan_tags.iter().map(|(tag, count)| serde_json::json!({
                "tag": tag,
                "document_count": count
            })).collect::<Vec<_>>(),
            "empty_stubs": empty_stubs.iter().map(|doc| serde_json::json!({
                "title": doc.title,
                "filename": doc.filename()
            })).collect::<Vec<_>>(),
            "orphan_documents": orphan_docs.iter().map(|doc| serde_json::json!({
                "title": doc.title,
                "status": doc.status.to_string()
            })).collect::<Vec<_>>(),
            "summary": {
                "broken_links_count": broken_links.len(),
                "orphan_tags_count": orphan_tags.len(),
                "empty_stubs_count": empty_stubs.len(),
                "orphan_documents_count": orphan_docs.len()
            }
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("🧹 Wiki Clean Report");
        println!();

        // Broken links
        if broken_links.is_empty() {
            println!("✅ Broken wikilinks: none");
        } else {
            println!("❌ Broken wikilinks: {}", broken_links.len());
            for stub in &broken_links {
                println!("   [[{}]] ({} incoming links)", stub.target, stub.inbound_count);
            }
        }
        println!();

        // Orphan tags
        if orphan_tags.is_empty() {
            println!("✅ Orphan tags: none");
        } else {
            println!("⚠️  Orphan tags (used in only 1 document): {}", orphan_tags.len());
            for (tag, _) in &orphan_tags {
                println!("   {}", tag);
            }
        }
        println!();

        // Empty stubs
        if empty_stubs.is_empty() {
            println!("✅ Empty stubs: none");
        } else {
            println!("⚠️  Empty stubs: {}", empty_stubs.len());
            for doc in &empty_stubs {
                println!("   [[{}]]", doc.title);
            }
        }
        println!();

        // Orphan documents
        if orphan_docs.is_empty() {
            println!("✅ Orphan documents: none");
        } else {
            println!("⚠️  Orphan documents (no incoming links): {}", orphan_docs.len());
            for doc in &orphan_docs {
                println!("   [[{}]] ({})", doc.title, doc.status);
            }
        }
    }

    // Handle --fix
    if fix && !empty_stubs.is_empty() {
        if dry_run {
            println!();
            println!("🏃 Dry run - would delete {} empty stubs", empty_stubs.len());
        } else {
            println!();
            println!("🗑️  Deleting {} empty stubs...", empty_stubs.len());

            for doc in &empty_stubs {
                let filepath = concepts_dir.join(doc.filename());
                if filepath.exists() {
                    std::fs::remove_file(&filepath)?;
                    println!("   Deleted: {}", doc.title);
                }
            }

            // Re-scan to update meta files
            let final_report = scanner::scan_wiki(&wiki_config)?;
            scanner::meta::generate_meta_files(&wiki_config, &final_report)?;

            println!("✅ Clean complete.");
        }
    } else if dry_run && !empty_stubs.is_empty() {
        println!();
        println!("💡 Run with --fix to delete empty stubs");
    }

    Ok(())
}

async fn run_wiki_growth(
    path: &str,
    count: Option<usize>,
    dry_run: bool,
    quiet: bool,
    no_confirm: bool,
    git: bool,
) -> Result<()> {
    use dialoguer::Confirm;
    use indicatif::{ProgressBar, ProgressStyle};

    // Load config
    let global_config = config::GlobalConfig::load()?
        .context("No config found. Run `wistra onboard` first.")?;

    // Use provided count or fall back to config's daily_count
    let count = count.unwrap_or(global_config.daily_count);

    // Resolve wiki path
    let wiki_path = if path == "." {
        global_config.wiki_path.clone()
            .context("No default wiki path configured. Run `wistra config` or specify a path.")?
    } else {
        PathBuf::from(shellexpand::tilde(path).to_string())
    };

    let wiki_config = config::WikiConfig::load(&wiki_path)?;

    if !quiet {
        println!("🔍 Scanning...");
    }

    let report = scanner::scan_wiki(&wiki_config)?;

    if !quiet {
        println!("   {} documents found", report.counts.total);
        println!("   {} stub links detected", report.stub_candidates.len());
        if !report.disambig_candidates.is_empty() {
            let names: Vec<&str> = report.disambig_candidates.iter().map(|d| d.title.as_str()).collect();
            println!("   {} disambiguation candidates: {}", report.disambig_candidates.len(), names.join(", "));
        }
        println!();
    }

    // Create execution plan
    let plan = planner::create_plan(&report, &global_config, count)?;

    if plan.slots.is_empty() {
        if !quiet {
            println!("✅ Wiki is up to date. No work to do.");
        }
        return Ok(());
    }

    // Print plan
    if !quiet {
        plan.print();
        println!();
    }

    // Check for link updates
    let link_updates = count_link_updates(&report);
    if !link_updates.is_empty() && !quiet {
        println!("⚠️  Link updates required:");
        println!("    [[disambiguated titles]] → {} documents will be rewritten", link_updates.len());
        println!();
    }

    // Dry run: print and exit
    if dry_run {
        println!("🏃 Dry run complete. No changes made.");
        return Ok(());
    }

    // Confirm
    if !no_confirm {
        let proceed = Confirm::new()
            .with_prompt("Proceed?")
            .default(true)
            .interact()?;

        if !proceed {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Initialize adapter (Claude Code CLI)
    let adapter = adapter::claude::ClaudeAdapter::new();

    // Progress bar
    let pb = if !quiet {
        let pb = ProgressBar::new(plan.slots.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"));
        Some(pb)
    } else {
        None
    };

    // Process each slot
    let mut generated_docs: Vec<types::Document> = Vec::new();
    let mut link_updates: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for (_i, slot) in plan.slots.iter().enumerate() {
        if let Some(ref pb) = pb {
            pb.set_message(format!("{}", slot.target));
        }

        match slot.action {
            planner::PlanAction::Stub => {
                let ctx = adapter::GenerationContext {
                    concept_name: slot.target.clone(),
                    concepts_dir: wiki_config.concepts_dir(),
                    wiki_dir: wiki_config.root_path.clone(),
                    related_docs: find_related_docs(&report, &slot.target),
                    wiki_index: report.wiki_index.clone(),
                    language: global_config.language.clone(),
                    tag_index: build_tag_index(&report),
                };

                match adapter.generate_concept(ctx).await {
                    Ok(doc) => generated_docs.push(doc),
                    Err(e) => {
                        if !quiet {
                            eprintln!("❌ Failed to generate {}: {}", slot.target, e);
                        }
                    }
                }
            }
            planner::PlanAction::Disambiguation => {
                // Find the disambig candidate
                if let Some(candidate) = report.disambig_candidates.iter().find(|c| c.title == slot.target) {
                    let (context_a, context_b) = build_disambig_contexts(&report, &candidate);

                    let ctx = adapter::DisambigContext {
                        title: slot.target.clone(),
                        wiki_dir: wiki_path.clone(),
                        context_a,
                        context_b,
                        wiki_index: report.wiki_index.clone(),
                        language: global_config.language.clone(),
                    };

                    match adapter.resolve_disambiguation(ctx).await {
                        Ok(result) => {
                            // Add link updates
                            for update in &result.link_updates {
                                link_updates.insert(update.from.clone(), update.to.clone());
                            }

                            // Create disambiguation document
                            if let Ok(disambig_doc) = parse_disambig_doc(&result.disambig, &global_config.language) {
                                generated_docs.push(disambig_doc);
                            }

                            // Create concept A document
                            if let Ok(doc_a) = parse_disambig_doc(&result.concept_a, &global_config.language) {
                                generated_docs.push(doc_a);
                            }

                            // Create concept B document
                            if let Ok(doc_b) = parse_disambig_doc(&result.concept_b, &global_config.language) {
                                generated_docs.push(doc_b);
                            }

                            if !quiet {
                                println!("✅ {} disambiguation resolved", slot.target);
                            }
                        }
                        Err(e) => {
                            if !quiet {
                                eprintln!("❌ Disambiguation failed for {}: {}", slot.target, e);
                            }
                        }
                    }
                }
            }
            planner::PlanAction::Random => {
                // Ask AI to suggest a concept based on interests
                let ctx = SuggestionContext {
                    wiki_dir: wiki_path.clone(),
                    wiki_index: report.wiki_index.clone(),
                    interests: global_config.interests.clone(),
                    language: global_config.language.clone(),
                    tag_index: build_tag_index(&report),
                };

                match adapter.suggest_concept(ctx).await {
                    Ok(suggestion) => {
                        if !quiet {
                            println!("   💡 Suggested: {} ({})", suggestion.title, suggestion.reason);
                        }
                        // Generate the suggested concept
                        let gen_ctx = adapter::GenerationContext {
                            concept_name: suggestion.title.clone(),
                            concepts_dir: wiki_config.concepts_dir(),
                            wiki_dir: wiki_path.clone(),
                            related_docs: suggestion.related_existing.clone(),
                            wiki_index: report.wiki_index.clone(),
                            language: global_config.language.clone(),
                            tag_index: build_tag_index(&report),
                        };

                        match adapter.generate_concept(gen_ctx).await {
                            Ok(doc) => generated_docs.push(doc),
                            Err(e) => {
                                if !quiet {
                                    eprintln!("❌ Failed to generate {}: {}", suggestion.title, e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if !quiet {
                            eprintln!("❌ Failed to suggest concept: {}", e);
                        }
                    }
                }
            }
        }

        if let Some(ref pb) = pb {
            pb.inc(1);
        }
    }

    if let Some(ref pb) = pb {
        pb.finish_with_message("done");
    }

    // Write documents
    if !generated_docs.is_empty() {
        if !quiet {
            println!("\n✍️  Writing {} documents...", generated_docs.len());
        }

        writer::DocumentWriter::write_batch(&generated_docs, &wiki_config.concepts_dir())?;

        // Apply link rewrites from disambiguation if any
        if !link_updates.is_empty() {
            if !quiet {
                println!("\n🔗 Updating links...");
            }
            let updated_count = writer::Linker::rewrite_links(&wiki_path, &link_updates)?;
            if !quiet {
                println!("   {} files updated", updated_count);
            }
        }

        // Re-scan to get updated report
        let final_report = scanner::scan_wiki(&wiki_config)?;

        // Generate meta files
        scanner::meta::generate_meta_files(&wiki_config, &final_report)?;

        // Update state
        update_state(&wiki_config, &final_report, &generated_docs)?;

        if !quiet {
            println!("✅ Done. {} documents added.", generated_docs.len());
        }
    } else if plan.slots.is_empty() {
        // No documents generated but plan was shown - just regenerate meta files
        let final_report = scanner::scan_wiki(&wiki_config)?;
        scanner::meta::generate_meta_files(&wiki_config, &final_report)?;
    }

    // Git commit and push if requested
    if git {
        git_commit_and_push(&wiki_path, &generated_docs, quiet)?;
    }

    Ok(())
}

fn count_link_updates(_report: &scanner::ScanReport) -> Vec<String> {
    // Return files that would need link updates
    // For now, return empty (will be populated when disambiguation is implemented)
    Vec::new()
}

fn build_disambig_contexts(
    report: &scanner::ScanReport,
    candidate: &DisambigCandidate,
) -> (Vec<String>, Vec<String>) {
    // Build context from linking documents for each occurrence
    let mut context_a: Vec<String> = Vec::new();
    let mut context_b: Vec<String> = Vec::new();

    // Group incoming links by source document
    for filename in &candidate.documents {
        if let Some(links) = report.link_graph.outgoing_links.get(filename) {
            for link in links {
                if link.target == candidate.title {
                    let ctx = format!("From [[{}]]: [[{}]]", filename.replace(".md", ""), candidate.title);
                    if context_a.is_empty() {
                        context_a.push(ctx);
                    } else {
                        context_b.push(ctx);
                    }
                }
            }
        }
    }

    (context_a, context_b)
}

fn parse_disambig_doc(
    concept: &adapter::DisambigConcept,
    language: &str,
) -> anyhow::Result<types::Document> {
    use crate::types::Status;

    // Parse the frontmatter from the string
    let frontmatter_lines: Vec<&str> = concept.frontmatter.lines().collect();
    let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for line in frontmatter_lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            fields.insert(key, value);
        }
    }

    let title = fields.get("title")
        .cloned()
        .unwrap_or_else(|| concept.new_title.clone());

    let aliases = fields.get("aliases")
        .map(|s| parse_yaml_list(s))
        .unwrap_or_default();

    let tags = fields.get("tags")
        .map(|s| parse_yaml_list(s))
        .unwrap_or_default();

    let created = fields.get("created")
        .and_then(|s| chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Local::now().naive_local().date());

    let status = fields.get("status")
        .map(|s| s.trim().to_lowercase())
        .and_then(|s| match s.as_str() {
            "published" => Some(Status::Published),
            "stub" => Some(Status::Stub),
            "disambiguation" => Some(Status::Disambiguation),
            "meta" => Some(Status::Meta),
            _ => None,
        })
        .unwrap_or(Status::Published);

    Ok(types::Document {
        title,
        aliases,
        tags,
        status,
        language: language.to_string(),
        created,
        relates: None,
        disambig: None,
        body: concept.body.clone(),
    })
}

fn parse_yaml_list(s: &str) -> Vec<String> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return Vec::new();
    }
    let content = &s[1..s.len()-1];
    content
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn find_related_docs(report: &scanner::ScanReport, target: &str) -> Vec<String> {
    // Find documents that link to this target
    report.link_graph
        .incoming_links
        .get(target)
        .map(|links| {
            links.iter()
                .map(|l| l.source_file.replace(".md", ""))
                .collect()
        })
        .unwrap_or_default()
}

fn build_tag_index(report: &scanner::ScanReport) -> String {
    // Build a simple tag hierarchy string
    let mut tags: Vec<String> = report.tag_stats.tag_counts
        .iter()
        .take(20)
        .map(|(tag, _)| tag.clone())
        .collect();
    tags.sort();
    tags.join(", ")
}

/// State for tracking wiki growth
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct State {
    last_run: chrono::DateTime<chrono::Utc>,
    documents_total: usize,
    last_added: Vec<String>,
    pending_disambig: Vec<String>,
    broken_links: Vec<String>,
}

fn update_state(
    wiki_config: &config::WikiConfig,
    report: &scanner::ScanReport,
    added_docs: &[types::Document],
) -> anyhow::Result<()> {
    let state = State {
        last_run: chrono::Utc::now(),
        documents_total: report.counts.total,
        last_added: added_docs.iter().map(|d| d.title.clone()).collect(),
        pending_disambig: report.disambig_candidates
            .iter()
            .map(|c| c.title.clone())
            .collect(),
        broken_links: report.stub_candidates
            .iter()
            .map(|c| c.target.clone())
            .collect(),
    };

    let json = serde_json::to_string_pretty(&state)?;
    std::fs::write(wiki_config.state_path(), json)?;

    Ok(())
}

/// Run the dedup command - detect duplicate or similar documents
fn run_dedup(path: &str, threshold: f64, json: bool) -> Result<()> {
    use std::collections::HashMap;
    use strsim::normalized_levenshtein;

    let path = shellexpand::tilde(path).to_string();
    let wiki_path = PathBuf::from(path);
    let wiki_config = config::WikiConfig::load(&wiki_path)?;
    let report = scanner::scan_wiki(&wiki_config)?;

    // Collect non-meta documents
    let docs: Vec<&types::Document> = report.documents
        .values()
        .filter(|doc| doc.status != types::Status::Meta)
        .collect();

    // 1. Detect similar titles (fuzzy match)
    let mut similar_titles: Vec<(String, String, f64)> = Vec::new();
    for i in 0..docs.len() {
        for j in (i + 1)..docs.len() {
            let title_a = normalize_for_comparison(&docs[i].title);
            let title_b = normalize_for_comparison(&docs[j].title);

            // Skip if titles are identical (exact duplicates)
            if title_a == title_b {
                continue;
            }

            let score = normalized_levenshtein(&title_a, &title_b);
            if score >= threshold {
                similar_titles.push((
                    docs[i].title.clone(),
                    docs[j].title.clone(),
                    score,
                ));
            }
        }
    }

    // Sort by similarity score (highest first)
    similar_titles.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    // 2. Detect documents with duplicate aliases
    let mut alias_map: HashMap<String, Vec<String>> = HashMap::new();
    for doc in &docs {
        for alias in &doc.aliases {
            let alias_lower = alias.to_lowercase();
            alias_map.entry(alias_lower)
                .or_insert_with(Vec::new)
                .push(doc.title.clone());
        }
    }
    let duplicate_aliases: Vec<(String, Vec<String>)> = alias_map
        .into_iter()
        .filter(|(_, titles)| titles.len() > 1)
        .collect();

    // 3. Report orphaned stubs that reference same concept
    let stub_groups: HashMap<String, Vec<String>> = report.stub_candidates
        .iter()
        .map(|stub| {
            let target_lower = stub.target.to_lowercase();
            (target_lower, vec![stub.target.clone()])
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            acc.entry(key).or_insert_with(Vec::new).extend(val);
            acc
        });

    let orphaned_stubs: Vec<(String, usize)> = stub_groups
        .into_iter()
        .map(|(key, targets)| (key, targets.len()))
        .collect();

    // Output results
    if json {
        let output = serde_json::json!({
            "similar_titles": similar_titles.iter().map(|(a, b, score)| serde_json::json!({
                "title_a": a,
                "title_b": b,
                "similarity": score
            })).collect::<Vec<_>>(),
            "duplicate_aliases": duplicate_aliases.iter().map(|(alias, titles)| serde_json::json!({
                "alias": alias,
                "documents": titles
            })).collect::<Vec<_>>(),
            "orphaned_stubs": orphaned_stubs.iter().map(|(target, count)| serde_json::json!({
                "target": target,
                "stub_count": count
            })).collect::<Vec<_>>(),
            "summary": {
                "similar_titles_count": similar_titles.len(),
                "duplicate_aliases_count": duplicate_aliases.len(),
                "orphaned_stubs_count": orphaned_stubs.len()
            }
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("🔍 Duplicate Detection Report");
        println!();

        // Similar titles
        if similar_titles.is_empty() {
            println!("✅ Similar titles: none");
        } else {
            println!("⚠️  Similar titles (fuzzy match ≥ {:.0}%):", threshold * 100.0);
            for (a, b, score) in &similar_titles {
                println!("   [[{}]] ↔ [[{}]] ({:.0}%)", a, b, score * 100.0);
            }
        }
        println!();

        // Duplicate aliases
        if duplicate_aliases.is_empty() {
            println!("✅ Duplicate aliases: none");
        } else {
            println!("⚠️  Duplicate aliases:");
            for (alias, titles) in &duplicate_aliases {
                println!("   \"{}\" used by: {}", alias, titles.iter().map(|t| format!("[[{}]]", t)).collect::<Vec<_>>().join(", "));
            }
        }
        println!();

        // Orphaned stubs
        if orphaned_stubs.is_empty() {
            println!("✅ Orphaned stubs: none");
        } else {
            println!("⚠️  Orphaned stubs (unresolved links):");
            for (target, count) in &orphaned_stubs {
                println!("   [[{}]] ({} references)", target, count);
            }
        }
    }

    Ok(())
}

/// Normalize a string for comparison (lowercase, alphanumeric only)
fn normalize_for_comparison(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .trim()
        .to_string()
}

/// Run the cron command - manage cron jobs
fn run_cron(set: Option<&str>, show: bool, remove: bool, install: bool, no_git: bool) -> Result<()> {
    // Default: show help if no options
    if set.is_none() && !show && !remove && !install {
        println!("wistra cron - Manage scheduled runs");
        println!();
        println!("Usage:");
        println!("  wistra cron --set 14:30     Set cron time (shows crontab line)");
        println!("  wistra cron --set 14:30 --install  Auto-install to crontab");
        println!("  wistra cron --set 14:30 --no-git   Skip git commit/push");
        println!("  wistra cron --show          Show current crontab line");
        println!("  wistra cron --remove        Remove wistra from crontab");
        return Ok(());
    }

    // Handle --set
    if let Some(time) = set {
        let (hour, minute) = cli::cron::parse_time(time)?;

        if install {
            cli::cron::install_cron(hour, minute, no_git)?;
        } else {
            cli::cron::show_cron(hour, minute, no_git);
        }
        return Ok(());
    }

    // Handle --show
    if show {
        // Get current crontab and find wistra line
        let output = std::process::Command::new("crontab")
            .arg("-l")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();

        let wistra_line = output.lines()
            .find(|line| line.contains("wistra run"));

        if let Some(line) = wistra_line {
            println!("Current cron job:");
            println!("    {}", line);
        } else {
            println!("No wistra cron job found.");
            println!("Run `wistra cron --set <TIME>` to create one.");
        }
        return Ok(());
    }

    // Handle --remove
    if remove {
        cli::cron::remove_cron()?;
    }

    Ok(())
}

/// Commit and push changes to git
fn git_commit_and_push(wiki_path: &PathBuf, generated_docs: &[types::Document], quiet: bool) -> Result<()> {
    use std::process::Command;

    // Check if there are changes to commit
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(wiki_path)
        .output()
        .context("Failed to run git status")?;

    let status_str = String::from_utf8_lossy(&status_output.stdout);
    if status_str.trim().is_empty() {
        if !quiet {
            println!("📭 No changes to commit.");
        }
        return Ok(());
    }

    // Stage all changes
    let add_status = Command::new("git")
        .args(["add", "-A"])
        .current_dir(wiki_path)
        .status()
        .context("Failed to run git add")?;

    if !add_status.success() {
        anyhow::bail!("git add failed");
    }

    // Build commit message
    let commit_msg = if generated_docs.is_empty() {
        "chore: update wiki meta files".to_string()
    } else {
        let titles: Vec<&str> = generated_docs.iter().map(|d| d.title.as_str()).take(5).collect();
        if generated_docs.len() <= 5 {
            format!("docs: add {} - {}", generated_docs.len(), titles.join(", "))
        } else {
            format!("docs: add {} documents including {}", generated_docs.len(), titles.join(", "))
        }
    };

    // Commit
    let commit_status = Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .current_dir(wiki_path)
        .status()
        .context("Failed to run git commit")?;

    if !commit_status.success() {
        anyhow::bail!("git commit failed");
    }

    if !quiet {
        println!("📤 Committed: {}", commit_msg);
    }

    // Check if remote exists
    let remote_output = Command::new("git")
        .args(["remote"])
        .current_dir(wiki_path)
        .output()
        .context("Failed to check git remote")?;

    let remote_str = String::from_utf8_lossy(&remote_output.stdout);
    if remote_str.trim().is_empty() {
        if !quiet {
            println!("   No remote configured, skipping push.");
        }
        return Ok(());
    }

    // Push
    let push_status = Command::new("git")
        .args(["push"])
        .current_dir(wiki_path)
        .status()
        .context("Failed to run git push")?;

    if !push_status.success() {
        anyhow::bail!("git push failed");
    }

    if !quiet {
        println!("✅ Pushed to remote.");
    }

    Ok(())
}
