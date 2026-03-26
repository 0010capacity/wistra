use crate::types::{Document, Link, Status};
use anyhow::{Context, Result};
use regex::Regex;
use serde_yaml::Value;
use std::path::Path;

/// Parse a markdown file and extract the document
pub fn parse_document(path: &Path) -> Result<Option<Document>> {
    let content = std::fs::read_to_string(path)?;

    // Extract frontmatter between --- markers
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Parse frontmatter fields
    let title = get_string(&frontmatter, "title")
        .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string());

    let aliases = get_string_list(&frontmatter, "aliases");
    let tags = get_string_list(&frontmatter, "tags");

    let language = get_string(&frontmatter, "language")
        .unwrap_or_else(|| "en".to_string());

    let created = get_string(&frontmatter, "created")
        .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Local::now().naive_local().date());

    let status_str = get_string(&frontmatter, "status")
        .context("Missing status field")?;

    let status = parse_status(&status_str)?;

    let relates = Some(get_string_list(&frontmatter, "relates"));
    let disambig = get_string(&frontmatter, "disambig");

    Ok(Some(Document {
        title,
        aliases,
        tags,
        status,
        language,
        created,
        relates,
        disambig,
        body,
    }))
}

/// Get a string field from YAML value
fn get_string(value: &Value, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(|s| s.to_string())
}

/// Get a list of strings from a YAML value
fn get_string_list(value: &Value, key: &str) -> Vec<String> {
    let seq = match value.get(key).and_then(|v| v.as_sequence()) {
        Some(s) => s,
        None => return Vec::new(),
    };
    seq.iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect()
}

/// Parse YAML frontmatter into a nested structure
fn extract_frontmatter(content: &str) -> Result<(Value, String)> {
    let content = content.trim_start();

    if !content.starts_with("---") {
        anyhow::bail!("No frontmatter found");
    }

    let rest = &content[3..];
    let end = rest.find("---")
        .context("Frontmatter not properly closed")?;

    let yaml_content = &rest[..end];
    let body = rest[end + 3..].to_string();

    let value: Value = serde_yaml::from_str(yaml_content)
        .context("Failed to parse YAML frontmatter")?;

    Ok((value, body))
}

/// Parse status string to Status enum
fn parse_status(s: &str) -> Result<Status> {
    match s.to_lowercase().as_str() {
        "stub" => Ok(Status::Stub),
        "published" => Ok(Status::Published),
        "disambiguation" => Ok(Status::Disambiguation),
        "meta" => Ok(Status::Meta),
        _ => anyhow::bail!("Invalid status: {}", s),
    }
}

/// Extract all wikilinks from document body
pub fn extract_links(body: &str, source_path: &Path) -> Vec<Link> {
    let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    let source_file = source_path.file_name().unwrap().to_string_lossy().to_string();

    re.captures_iter(body)
        .filter_map(|cap| {
            let raw = cap.get(1)?.as_str();
            Link::parse(raw, source_file.clone())
        })
        .collect()
}

/// Extract the first sentence as summary
pub fn extract_summary(body: &str) -> String {
    // Remove markdown syntax and get first meaningful sentence
    let body = body.trim();

    // Skip common prefixes
    let body = body.trim_start_matches("# ")
        .trim_start_matches("## ")
        .trim_start_matches("> [!note]\n")
        .trim_start_matches("<!-- stub:");

    // Find the first sentence (ends with . ! ? followed by space or newline, or end of string)
    let re = Regex::new(r"^[^.!?\n]*[.!?\n]").unwrap();
    if let Some(cap) = re.captures(body) {
        let sentence = cap.get(0).unwrap().as_str();
        // Clean up markdown syntax
        sentence
            .replace("#", "")
            .replace("*", "")
            .replace("_", "")
            .trim()
            .to_string()
    } else {
        // Fallback: first 100 chars
        body.chars().take(100).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let body = "This links to [[Apple]] and [[Banana|Yellow Fruit]] and [[Carrot#Section]].";
        let links = extract_links(body, Path::new("test.md"));

        assert_eq!(links.len(), 3);
        assert_eq!(links[0].target, "Apple");
        assert_eq!(links[1].target, "Banana");
        assert_eq!(links[1].display, Some("Yellow Fruit".to_string()));
        assert_eq!(links[2].target, "Carrot");
        assert_eq!(links[2].section, Some("Section".to_string()));
    }

    #[test]
    fn test_extract_summary() {
        let body = "This is the first sentence. This is the second.";
        let summary = extract_summary(body);
        assert!(summary.starts_with("This is the first sentence"));
    }
}
