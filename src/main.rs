mod adapter;
mod cli;
mod config;
mod planner;
mod scanner;
mod types;
mod writer;

use crate::adapter::WikiAdapter;
use crate::scanner::DisambigCandidate;
use anyhow::{Context, Result};
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
        Some(cli::Commands::Run { path, count, dry_run, quiet, no_confirm }) => {
            run_wiki_growth(&path, count, dry_run, quiet, no_confirm).await?;
        }
    }

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

async fn run_wiki_growth(
    path: &str,
    count: usize,
    dry_run: bool,
    quiet: bool,
    no_confirm: bool,
) -> Result<()> {
    use dialoguer::Confirm;
    use indicatif::{ProgressBar, ProgressStyle};

    // Load config
    let global_config = config::GlobalConfig::load()?
        .context("No config found. Run `wistra onboard` first.")?;

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

    // Initialize adapter
    let adapter = adapter::claude::ClaudeAdapter::new(global_config.claude_api_key.clone());

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
                // Interest-based random selection not yet implemented via AI
                // Fall back to skipping
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
