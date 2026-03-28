use crate::adapter::{DisambigContext, DisambigResult, GenerationContext, SuggestedConcept, SuggestionContext, WikiAdapter};
use crate::types::{Document, Status};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tokio::time::{Duration, timeout};

/// Claude Code CLI adapter
pub struct ClaudeAdapter {
    /// Path to claude CLI (default: "claude")
    cli_path: String,
    /// Timeout for CLI commands
    timeout_secs: u64,
}

impl ClaudeAdapter {
    pub fn new() -> Self {
        ClaudeAdapter {
            cli_path: "claude".to_string(),
            timeout_secs: 300, // 5 minutes default
        }
    }

    /// Call Claude CLI with a prompt and timeout
    async fn call_cli(&self, prompt: &str, wiki_dir: &Path) -> Result<String> {
        let output = timeout(
            Duration::from_secs(self.timeout_secs),
            Command::new(&self.cli_path)
                .arg("-p")
                .arg(prompt)
                .arg("--output-format")
                .arg("text")
                .arg("--add-dir")
                .arg(wiki_dir)
                .arg("--permission-mode")
                .arg("acceptEdits")
                .output()
        )
        .await
        .context("Claude CLI timed out")?
        .context("Failed to execute claude CLI. Make sure Claude Code is installed.")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Claude CLI error: {}", stderr));
        }

        let response = String::from_utf8(output.stdout)
            .context("Failed to parse CLI output as UTF-8")?;

        Ok(response.trim().to_string())
    }
}

/// Validate that a path is within the wiki directory
fn validate_path(path: &str, wiki_dir: &Path) -> Result<PathBuf> {
    // Resolve the path relative to wiki_dir
    let full_path = wiki_dir.join(path);

    // Canonicalize both paths for comparison
    let full_path = full_path.canonicalize()
        .or_else(|_| {
            // If canonicalize fails, check if parent exists and is within wiki
            let parent = full_path.parent().unwrap_or(wiki_dir);
            if parent.starts_with(wiki_dir) {
                Ok(full_path)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Path outside wiki directory",
                ))
            }
        })?;

    let wiki_dir = wiki_dir.canonicalize()
        .context("Failed to canonicalize wiki directory")?;

    // Ensure the path is within wiki_dir
    if full_path.starts_with(&wiki_dir) {
        Ok(full_path)
    } else {
        anyhow::bail!("Path escape detected: {} is outside {}", path, wiki_dir.display());
    }
}

/// Read a written document with path validation
fn read_document(path: &Path, wiki_dir: &Path) -> Result<Document> {
    // Validate path is within wiki directory
    validate_path(path.to_str().unwrap_or(""), wiki_dir)?;

    if !path.exists() {
        anyhow::bail!("File not found: {}. Claude may have failed to write it.", path.display());
    }

    let content = std::fs::read_to_string(path)
        .context("Failed to read written document")?;
    parse_document_content(&content)
}

impl Default for ClaudeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WikiAdapter for ClaudeAdapter {
    async fn generate_concept(&self, ctx: GenerationContext) -> Result<Document> {
        let file_path = ctx.concepts_dir.join(format!("{}.md", ctx.concept_name));
        let wiki_index_json = serde_json::to_string_pretty(&ctx.wiki_index.entries)
            .context("Failed to serialize wiki index")?;

        let prompt = format!(
            r#"Write a wiki document for concept "{}" and save it to: {}

[Wiki Context]
Language: {}
Existing documents: {}
Tag hierarchy: {}
Wiki index (title, tags, summary):
{}

[Writing Instructions]
1. Create a Markdown file at the path above with YAML frontmatter
2. Required frontmatter:
   - title: English name
   - aliases: List of alternative names (at least 2-3, include non-English names if relevant)
   - tags: List of hierarchical tags using slash notation (e.g., philosophy/epistemology, science/methodology) - at least 3-5 tags
   - status: published
   - language: {}
   - created: YYYY-MM-DD
3. Body: 300–800 words in the specified language
4. Use [[wikilinks]] to link to related docs: {}
5. Use LaTeX for math ($inline$, $$block$$), fenced code blocks, blockquotes for citations
6. Use Obsidian callout syntax for notes: > [!note]
7. Search the web for relevant information to make the article accurate and comprehensive
8. Do NOT wrap output in code fences. Write raw Markdown only.

Example frontmatter:
---
title: "Concept Name"
aliases: [alternative-name, Another Name, foreign-name]
tags: [category/subcategory, related-field, discipline]
status: published
language: ko
created: "today's date"
---

After writing the file, output "OK" only.
"#,
            ctx.concept_name,
            file_path.display(),
            ctx.language,
            ctx.wiki_index.entries.len(),
            ctx.tag_index,
            wiki_index_json,
            ctx.language,
            ctx.related_docs.join(", ")
        );

        let response = self.call_cli(&prompt, &ctx.wiki_dir).await?;

        // Verify file was written
        if !file_path.exists() {
            anyhow::bail!(
                "File not found at {} after CLI completed. Response: {}",
                file_path.display(),
                response
            );
        }

        // Validate and read the file
        read_document(&file_path, &ctx.concepts_dir)
    }

    async fn resolve_disambiguation(&self, ctx: DisambigContext) -> Result<DisambigResult> {
        let wiki_dir = &ctx.wiki_dir;
        let concepts_dir = wiki_dir.join("concepts");

        let wiki_index_json = serde_json::to_string_pretty(&ctx.wiki_index.entries)
            .context("Failed to serialize wiki index")?;

        let prompt = format!(
            r#"Resolve disambiguation for title '{}'. Write 3 files in the concepts/ directory:

1. DISAMBIGUATION PAGE: concepts/{}-disambiguation.md
2. CONCEPT A: concepts/ConceptA.md (use appropriate name)
3. CONCEPT B: concepts/ConceptB.md (use appropriate name)

[Wiki Context]
Language: {}
Wiki index (title, tags, summary):
{}

Context from linking documents:
- Group A: {}
- Group B: {}

[Instructions for each file]
- Use YAML frontmatter: title, aliases, tags, status, language, created
- Body: 300–500 words each, relevant content based on context
- Use [[wikilinks]] to link between related documents
- Search web for accurate information

After writing all 3 files, output JSON only:
{{"disambig_path": "concepts/{}-disambiguation.md", "concept_a_path": "concepts/ConceptA.md", "concept_b_path": "concepts/ConceptB.md", "link_updates": []}}
"#,
            ctx.title,
            ctx.title,
            ctx.language,
            wiki_index_json,
            ctx.context_a.join("; "),
            ctx.context_b.join("; "),
            ctx.title
        );

        let response = self.call_cli(&prompt, wiki_dir).await?;
        parse_disambig_response(&response, &concepts_dir)
    }

    async fn suggest_concept(&self, ctx: SuggestionContext) -> Result<SuggestedConcept> {
        let wiki_index_json = serde_json::to_string_pretty(&ctx.wiki_index.entries)
            .context("Failed to serialize wiki index")?;

        let existing_titles: Vec<&str> = ctx.wiki_index.entries.iter()
            .map(|e| e.title.as_str())
            .collect();

        let prompt = format!(
            r#"You are a knowledge graph curator. Suggest a new concept to add to a personal wiki.

[Wiki Context]
Language: {}
User interests: {}
Tag hierarchy: {}
Existing concepts: {}
Wiki index (title, tags, summary):
{}

[Task]
Suggest ONE new concept that:
1. Relates to the user's interests
2. Connects to multiple existing documents via [[wikilinks]]
3. Is NOT already in the existing concepts list
4. Would naturally extend the knowledge graph

Return JSON only, no other text:
{{"title": "Concept Name", "reason": "Brief explanation why this concept fits", "related_existing": ["Existing Doc 1", "Existing Doc 2"]}}
"#,
            ctx.language,
            ctx.interests.join(", "),
            ctx.tag_index,
            existing_titles.join(", "),
            wiki_index_json
        );

        let response = self.call_cli(&prompt, &ctx.wiki_dir).await?;
        parse_suggestion_response(&response)
    }
}

/// Parse a document from file content
fn parse_document_content(content: &str) -> Result<Document> {
    let content = content.trim();

    if !content.starts_with("---") {
        anyhow::bail!("No frontmatter found in generated document");
    }

    let rest = &content[3..];
    let end = rest.find("---")
        .context("Frontmatter not properly closed")?;

    let yaml_content = &rest[..end];
    let body = rest[end + 3..].trim().to_string();

    let fields = parse_yaml_simple(yaml_content)?;

    let title = fields.get("title")
        .cloned()
        .context("Missing title in generated document")?;

    let aliases = fields.get("aliases")
        .and_then(|s| parse_yaml_list(s))
        .unwrap_or_default();

    let tags = fields.get("tags")
        .and_then(|s| parse_yaml_list(s))
        .unwrap_or_default();

    let created = fields.get("created")
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Local::now().naive_local().date());

    let language = fields.get("language")
        .cloned()
        .unwrap_or_else(|| "en".to_string());

    Ok(Document {
        title,
        aliases,
        tags,
        status: Status::Published,
        language,
        created,
        relates: None,
        disambig: None,
        body,
    })
}

/// Parse a simple YAML list string like "[item1, item2]"
fn parse_yaml_list(s: &str) -> Option<Vec<String>> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return None;
    }

    let content = &s[1..s.len()-1];
    let items: Vec<String> = content
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Some(items)
}

/// Parse simple YAML fields (no nesting)
fn parse_yaml_simple(yaml: &str) -> Result<std::collections::HashMap<String, String>> {
    let mut fields = std::collections::HashMap::new();

    for line in yaml.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim();

            if value.starts_with('"') && value.ends_with('"') {
                fields.insert(key, value[1..value.len()-1].to_string());
            } else if value.starts_with('\'') && value.ends_with('\'') {
                fields.insert(key, value[1..value.len()-1].to_string());
            } else {
                fields.insert(key, value.to_string());
            }
        }
    }

    Ok(fields)
}

fn parse_disambig_response(content: &str, concepts_dir: &Path) -> Result<DisambigResult> {
    let content = content.trim();

    // Remove markdown code fences if present
    let content = content
        .strip_prefix("```json")
        .unwrap_or(content)
        .strip_prefix("```")
        .unwrap_or(content);
    let content = content
        .strip_suffix("```")
        .unwrap_or(content)
        .trim();

    let json: serde_json::Value = serde_json::from_str(content)
        .context("Failed to parse disambiguation JSON response")?;

    let disambig_path = json["disambig_path"].as_str()
        .context("Missing disambig_path")?;
    let concept_a_path = json["concept_a_path"].as_str()
        .context("Missing concept_a_path")?;
    let concept_b_path = json["concept_b_path"].as_str()
        .context("Missing concept_b_path")?;

    // Read the written files with path validation
    let disambig_doc = read_document(Path::new(disambig_path), concepts_dir)?;
    let concept_a = read_document(Path::new(concept_a_path), concepts_dir)?;
    let concept_b = read_document(Path::new(concept_b_path), concepts_dir)?;

    let link_updates: Vec<crate::adapter::LinkUpdate> = json["link_updates"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    Some(crate::adapter::LinkUpdate {
                        source_file: v["file"].as_str()?.to_string(),
                        from: v["from"].as_str()?.to_string(),
                        to: v["to"].as_str()?.to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(DisambigResult {
        concept_a: crate::adapter::DisambigConcept {
            new_title: concept_a.title.clone(),
            frontmatter: format!("title: \"{}\"\naliases: {:?}\ntags: {:?}\nstatus: published\nlanguage: {}\ncreated: {}",
                concept_a.title, concept_a.aliases, concept_a.tags, concept_a.language, concept_a.created),
            body: concept_a.body,
        },
        concept_b: crate::adapter::DisambigConcept {
            new_title: concept_b.title.clone(),
            frontmatter: format!("title: \"{}\"\naliases: {:?}\ntags: {:?}\nstatus: published\nlanguage: {}\ncreated: {}",
                concept_b.title, concept_b.aliases, concept_b.tags, concept_b.language, concept_b.created),
            body: concept_b.body,
        },
        disambig: crate::adapter::DisambigConcept {
            new_title: disambig_doc.title.clone(),
            frontmatter: format!("title: \"{}\"\naliases: {:?}\ntags: {:?}\nstatus: disambiguation\nlanguage: {}\ncreated: {}",
                disambig_doc.title, disambig_doc.aliases, disambig_doc.tags, disambig_doc.language, disambig_doc.created),
            body: disambig_doc.body,
        },
        link_updates,
    })
}

fn parse_suggestion_response(content: &str) -> Result<SuggestedConcept> {
    let content = content.trim();

    // Remove markdown code fences if present (handle both ```json and ```)
    let content = if content.starts_with("```json") {
        content.strip_prefix("```json").unwrap_or(content)
    } else if content.starts_with("```") {
        content.strip_prefix("```").unwrap_or(content)
    } else {
        content
    };

    // Remove trailing code fence
    let content = content.strip_suffix("```").unwrap_or(content).trim();

    let json: serde_json::Value = serde_json::from_str(content)
        .with_context(|| format!("Failed to parse suggestion JSON response. Got: {}", content))?;

    let related_existing = json["related_existing"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(SuggestedConcept {
        title: json["title"].as_str().context("Missing title")?.to_string(),
        reason: json["reason"].as_str().unwrap_or("").to_string(),
        related_existing,
    })
}
