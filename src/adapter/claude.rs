use crate::adapter::{DisambigContext, DisambigResult, GenerationContext, SuggestedConcept, SuggestionContext, WikiAdapter};
use crate::types::{Document, Status};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";

/// Claude API adapter
pub struct ClaudeAdapter {
    client: Client,
    api_key: String,
}

impl ClaudeAdapter {
    pub fn new(api_key: String) -> Self {
        ClaudeAdapter {
            client: Client::new(),
            api_key,
        }
    }

    async fn call_api(&self, prompt: &str) -> Result<String> {
        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": "claude-sonnet-4-20250514",
                "max_tokens": 4096,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ]
            }))
            .send()
            .await
            .context("Failed to call Claude API")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Claude API error: {}", error_text));
        }

        let json: serde_json::Value = response.json().await.context("Failed to parse API response")?;

        let content = json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow!("Unexpected API response format"))?
            .to_string();

        Ok(content)
    }
}

#[async_trait]
impl WikiAdapter for ClaudeAdapter {
    async fn generate_concept(&self, ctx: GenerationContext) -> Result<Document> {
        let wiki_index_json = serde_json::to_string_pretty(&ctx.wiki_index.entries)
            .context("Failed to serialize wiki index")?;

        let prompt = format!(
            r#"You are an editor writing documents for a personal knowledge wiki.

[Wiki Context]
Language: {}
Existing documents: {}
Tag hierarchy: {}
Wiki index (title, tags, summary):
{}

[Writing Rules]
1. Return a single complete Markdown document including YAML frontmatter
2. Required frontmatter fields: title (English), aliases, tags, status (published), language, created
3. If language is Korean: write body in Korean, include Korean aliases
4. Tags must fit the existing hierarchy. New tags must extend existing branches
5. Reference related existing documents using [[wikilinks]] naturally in the body
6. Use LaTeX for math ($inline$, $$block$$), fenced code blocks with language tags, blockquotes for citations
7. Use Obsidian callout syntax for notes: > [!note]
8. Length: 300–800 words
9. Do not wrap in code fences. Return raw Markdown only.

[Target Concept]
Name: {}
Related existing documents: {}
"#,
            ctx.language,
            ctx.wiki_index.entries.len(),
            ctx.tag_index,
            wiki_index_json,
            ctx.concept_name,
            ctx.related_docs.join(", ")
        );

        let response = self.call_api(&prompt).await?;

        // Parse the response as a document
        parse_generated_document(&response, &ctx.language)
    }

    async fn resolve_disambiguation(&self, ctx: DisambigContext) -> Result<DisambigResult> {
        let prompt = format!(
            r#"Two documents share the same title and must be separated.

[Document A]
Title: {}
Context from linking documents:
{}

[Document B]
Title: {}
Context from linking documents:
{}

Instructions:
1. Determine appropriate qualifiers for each concept (e.g. "Apple (Fruit)", "Apple (Company)")
2. Decide which existing [[{}]] links belong to A vs B based on context
3. Return JSON only, no other text:
{{
  "concept_a": {{ "new_title": "...", "frontmatter": "...", "body": "..." }},
  "concept_b": {{ "new_title": "...", "frontmatter": "...", "body": "..." }},
  "disambig": {{ "frontmatter": "...", "body": "..." }},
  "link_updates": [ {{ "file": "relative/path.md", "from": "{}", "to": "Apple (Company)" }} ]
}}
"#,
            ctx.title, ctx.context_a.join("\n"),
            ctx.title, ctx.context_b.join("\n"),
            ctx.title, ctx.title
        );

        let response = self.call_api(&prompt).await?;

        // Parse JSON response
        parse_disambig_response(&response)
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
{{
  "title": "Concept Name",
  "reason": "Brief explanation why this concept fits",
  "related_existing": ["Existing Doc 1", "Existing Doc 2"]
}}
"#,
            ctx.language,
            ctx.interests.join(", "),
            ctx.tag_index,
            existing_titles.join(", "),
            wiki_index_json
        );

        let response = self.call_api(&prompt).await?;
        parse_suggestion_response(&response)
    }
}

fn parse_generated_document(content: &str, language: &str) -> Result<Document> {
    // Extract frontmatter between --- markers
    let content = content.trim();

    if !content.starts_with("---") {
        anyhow::bail!("No frontmatter found in generated document");
    }

    let rest = &content[3..];
    let end = rest.find("---")
        .context("Frontmatter not properly closed")?;

    let yaml_content = &rest[..end];
    let body = rest[end + 3..].trim().to_string();

    // Parse YAML fields
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

    Ok(Document {
        title,
        aliases,
        tags,
        status: Status::Published,
        language: language.to_string(),
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

fn parse_disambig_response(content: &str) -> Result<DisambigResult> {
    // Try to extract JSON from the response
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

    let concept_a = parse_disambig_concept(&json["concept_a"])?;
    let concept_b = parse_disambig_concept(&json["concept_b"])?;
    let disambig = parse_disambig_concept(&json["disambig"])?;

    let link_updates = json["link_updates"]
        .as_array()
        .context("Missing link_updates array")?
        .iter()
        .map(|v| {
            Ok(crate::adapter::LinkUpdate {
                source_file: v["file"].as_str().context("Missing file field")?.to_string(),
                from: v["from"].as_str().context("Missing from field")?.to_string(),
                to: v["to"].as_str().context("Missing to field")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(DisambigResult {
        concept_a,
        concept_b,
        disambig,
        link_updates,
    })
}

fn parse_disambig_concept(json: &serde_json::Value) -> Result<crate::adapter::DisambigConcept> {
    Ok(crate::adapter::DisambigConcept {
        new_title: json["new_title"].as_str().context("Missing new_title")?.to_string(),
        frontmatter: json["frontmatter"].as_str().context("Missing frontmatter")?.to_string(),
        body: json["body"].as_str().context("Missing body")?.to_string(),
    })
}

fn parse_suggestion_response(content: &str) -> Result<SuggestedConcept> {
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
        .context("Failed to parse suggestion JSON response")?;

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
