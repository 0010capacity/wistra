pub mod parser;
pub mod frontmatter;

use crate::config::WikiConfig;
use crate::scanner::parser::extract_links;
use crate::types::{Document, Status};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Result of an import operation
#[derive(Debug, Clone)]
pub struct ImportResult {
    pub documents_imported: usize,
    pub stubs_created: usize,
    pub disambig_candidates: Vec<DisambigCandidate>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DisambigCandidate {
    pub title: String,
    pub files: Vec<String>,
}

/// Import markdown files from a source path into the wiki
pub fn import_path(
    source: &Path,
    wiki_config: &WikiConfig,
    dry_run: bool,
    json_output: bool,
) -> Result<ImportResult> {
    let source_path = PathBuf::from(source);

    if !source_path.exists() {
        anyhow::bail!("Source path does not exist: {}", source.display());
    }

    let mut result = ImportResult {
        documents_imported: 0,
        stubs_created: 0,
        disambig_candidates: Vec::new(),
        errors: Vec::new(),
    };

    // Collect all markdown files to import
    let files_to_import: Vec<PathBuf> = if source_path.is_file() {
        vec![source_path]
    } else {
        WalkDir::new(&source_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    if files_to_import.is_empty() {
        if !json_output {
            println!("No markdown files found to import.");
        }
        return Ok(result);
    }

    // Scan existing wiki to get current state
    let wiki_report = crate::scanner::scan_wiki(wiki_config)?;
    let existing_titles: Vec<String> = wiki_report.wiki_index.titles()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // Track titles to detect disambiguation candidates
    let mut title_occurrences: HashMap<String, Vec<String>> = HashMap::new();

    // Track all wikilinks to create stubs
    let mut all_missing_links: Vec<String> = Vec::new();

    // Process each file
    let mut documents_to_write: Vec<Document> = Vec::new();

    for file_path in &files_to_import {
        match process_file(file_path, wiki_config, &existing_titles) {
            Ok((doc, missing_links)) => {
                // Check for disambiguation
                title_occurrences
                    .entry(doc.title.clone())
                    .or_insert_with(Vec::new)
                    .push(file_path.to_string_lossy().to_string());

                all_missing_links.extend(missing_links);
                documents_to_write.push(doc);
            }
            Err(e) => {
                result.errors.push(format!(
                    "{}: {}",
                    file_path.display(),
                    e
                ));
            }
        }
    }

    // Find disambiguation candidates (same title in multiple files)
    for (title, files) in title_occurrences {
        if files.len() > 1 {
            result.disambig_candidates.push(DisambigCandidate {
                title,
                files,
            });
        }
    }

    // Find missing targets that need stubs
    let existing_titles_lower: Vec<String> = existing_titles.iter().map(|t| t.to_lowercase()).collect();
    let unique_missing: Vec<String> = all_missing_links
        .into_iter()
        .filter(|link| !existing_titles_lower.contains(&link.to_lowercase()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let stubs_to_create = unique_missing.len();

    // Print summary
    if json_output {
        let output = serde_json::json!({
            "documents_found": files_to_import.len(),
            "documents_to_import": documents_to_write.len(),
            "stubs_to_create": stubs_to_create,
            "disambig_candidates": result.disambig_candidates,
            "errors": result.errors,
            "dry_run": dry_run
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("📥 Import Preview");
        println!("   Files found: {}", files_to_import.len());
        println!("   Documents to import: {}", documents_to_write.len());
        println!("   Stubs to create: {}", stubs_to_create);
        if !result.disambig_candidates.is_empty() {
            println!();
            println!("⚠️  Disambiguation candidates:");
            for candidate in &result.disambig_candidates {
                println!("   - {} (in {} files)", candidate.title, candidate.files.len());
            }
        }
        if !result.errors.is_empty() {
            println!();
            println!("⚠️  Errors:");
            for error in &result.errors {
                println!("   - {}", error);
            }
        }
    }

    if dry_run {
        println!();
        println!("🏃 Dry run - no changes made.");
        result.documents_imported = 0;
        result.stubs_created = 0;
        return Ok(result);
    }

    // Write documents
    let concepts_dir = wiki_config.concepts_dir();
    for doc in &documents_to_write {
        if let Err(e) = crate::writer::DocumentWriter::write(doc, &concepts_dir) {
            result.errors.push(format!("{}: {}", doc.filename(), e));
        } else {
            result.documents_imported += 1;
        }
    }

    // Create stubs for missing links
    let default_language = crate::config::GlobalConfig::load()?
        .map(|c| c.language)
        .unwrap_or_else(|| "en".to_string());

    for link_target in unique_missing {
        let stub = Document::new_stub(link_target.clone(), default_language.clone());
        if let Err(e) = crate::writer::DocumentWriter::write(&stub, &concepts_dir) {
            result.errors.push(format!("{}: {}", stub.filename(), e));
        } else {
            result.stubs_created += 1;
        }
    }

    if !json_output {
        println!();
        println!("✅ Import complete.");
        println!("   {} documents imported", result.documents_imported);
        println!("   {} stubs created", result.stubs_created);
    }

    Ok(result)
}

/// Process a single markdown file
fn process_file(
    path: &Path,
    wiki_config: &WikiConfig,
    existing_titles: &[String],
) -> Result<(Document, Vec<String>)> {
    let content = std::fs::read_to_string(path)?;
    let filename = path.file_stem().unwrap().to_string_lossy().to_string();

    // Try to parse as wistra document first
    if let Ok(Some(doc)) = crate::scanner::parser::parse_document(path) {
        // Valid wistra document, return as-is
        return Ok((doc, Vec::new()));
    }

    // Parse as foreign document (no frontmatter or invalid frontmatter)
    let (frontmatter, body) = parser::extract_frontmatter(&content);

    let title = frontmatter::get_string(&frontmatter, "title")
        .unwrap_or_else(|| filename.clone());

    let aliases = frontmatter::get_string_list(&frontmatter, "aliases")
        .or_else(|| frontmatter::generate_aliases(&body, &title))
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();

    let tags = frontmatter::get_string_list(&frontmatter, "tags")
        .unwrap_or_default();
    let language = frontmatter::get_string(&frontmatter, "language")
        .unwrap_or_else(|| "en".to_string());
    let created = frontmatter::get_string(&frontmatter, "created")
        .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Local::now().naive_local().date());

    let status = frontmatter::get_string(&frontmatter, "status")
        .map(|s| {
            match s.to_lowercase().as_str() {
                "stub" => Status::Stub,
                "published" => Status::Published,
                "disambiguation" => Status::Disambiguation,
                _ => Status::Published,
            }
        })
        .unwrap_or(Status::Published);

    // Extract wikilinks to find missing targets
    let links = extract_links(&body, path);
    let missing_links: Vec<String> = links
        .into_iter()
        .map(|l| l.target)
        .filter(|target| {
            !existing_titles.iter().any(|t| t.eq_ignore_ascii_case(target))
        })
        .collect();

    let doc = Document {
        title,
        aliases,
        tags,
        status,
        language,
        created,
        relates: None,
        disambig: None,
        body,
    };

    Ok((doc, missing_links))
}
