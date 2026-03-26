use crate::scanner::parser;
use crate::types::Status;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

/// Linker for rewriting wikilinks
pub struct Linker;

impl Linker {
    /// Rewrite wikilinks in all managed documents
    pub fn rewrite_links(
        wiki_dir: &Path,
        updates: &HashMap<String, String>,
    ) -> Result<usize> {
        let mut updated_count = 0;

        // Walk the concepts directory
        let concepts_dir = wiki_dir.join("concepts");
        if !concepts_dir.exists() {
            return Ok(0);
        }

        for entry in walkdir::WalkDir::new(&concepts_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            if let Some(ext) = entry.path().extension() {
                if ext != "md" {
                    continue;
                }

                // Check if this is a wistra-managed document
                if let Ok(Some(doc)) = parser::parse_document(entry.path()) {
                    if doc.status == Status::Meta {
                        continue;
                    }

                    // Rewrite links in this document
                    let content = std::fs::read_to_string(entry.path())?;
                    let new_content = Self::rewrite_content(&content, updates);

                    if content != new_content {
                        std::fs::write(entry.path(), &new_content)?;
                        updated_count += 1;
                    }
                }
            }
        }

        Ok(updated_count)
    }

    /// Rewrite wikilinks in content
    fn rewrite_content(content: &str, updates: &HashMap<String, String>) -> String {
        let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();

        re.replace_all(content, |caps: &regex::Captures| {
            let raw = &caps[1];

            // Parse the link
            let (target, section, display) = Self::parse_link_parts(raw);

            // Check if this target needs to be updated
            if let Some(new_target) = updates.get(&target) {
                // Reconstruct the link with new target
                let mut new_link = format!("[[{}", new_target);
                if let Some(s) = section {
                    new_link.push_str(&format!("#{}", s));
                }
                if let Some(d) = display {
                    new_link.push_str(&format!("|{}", d));
                }
                new_link.push_str("]]");
                new_link
            } else {
                // No change
                caps[0].to_string()
            }
        }).to_string()
    }

    /// Parse link parts: target, section, display
    fn parse_link_parts(raw: &str) -> (String, Option<String>, Option<String>) {
        let raw = raw.trim();

        // Extract display text if present
        let (target_part, display) = if let Some(pipe_pos) = raw.find('|') {
            let (t, d) = raw.split_at(pipe_pos);
            (t, Some(d[1..].to_string()))
        } else {
            (raw, None)
        };

        // Extract section if present
        let (target, section) = if let Some(hash_pos) = target_part.find('#') {
            let (t, s) = target_part.split_at(hash_pos);
            (t, Some(s[1..].to_string()))
        } else {
            (target_part, None)
        };

        (target.to_string(), section, display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_content() {
        let content = "This links to [[Apple]] and [[Banana|Yellow]].";
        let mut updates = HashMap::new();
        updates.insert("Apple".to_string(), "Apple (Company)".to_string());

        let result = Linker::rewrite_content(content, &updates);
        assert!(result.contains("[[Apple (Company)]]"));
        assert!(result.contains("[[Banana|Yellow]]"));
    }

    #[test]
    fn test_rewrite_with_section() {
        let content = "See [[Apple#History]].";
        let mut updates = HashMap::new();
        updates.insert("Apple".to_string(), "Apple (Fruit)".to_string());

        let result = Linker::rewrite_content(content, &updates);
        assert!(result.contains("[[Apple (Fruit)#History]]"));
    }
}
