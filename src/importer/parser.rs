/// Parse frontmatter from foreign markdown files
use std::collections::HashMap;

/// Extract frontmatter from markdown content
pub fn extract_frontmatter(content: &str) -> (HashMap<String, String>, String) {
    let content = content.trim();

    if !content.starts_with("---") {
        // No frontmatter
        return (HashMap::new(), content.to_string());
    }

    let rest = &content[3..];
    if let Some(end) = rest.find("---") {
        let yaml_content = &rest[..end];
        let body = rest[end + 3..].trim().to_string();

        let fields = parse_simple_yaml(yaml_content);
        (fields, body)
    } else {
        // Frontmatter not properly closed
        (HashMap::new(), content.to_string())
    }
}

/// Parse simple YAML key-value pairs (no nesting)
pub fn parse_simple_yaml(yaml: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();

    for line in yaml.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let mut value = line[colon_pos + 1..].trim().to_string();

            // Handle quoted strings
            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                value = value[1..value.len() - 1].to_string();
            } else if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
                value = value[1..value.len() - 1].to_string();
            }

            fields.insert(key, value);
        }
    }

    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_with_frontmatter() {
        let content = r#"---
title: Test
tags: [tag1, tag2]
---
# Hello
This is content."#;

        let (fields, body) = extract_frontmatter(content);
        assert_eq!(fields.get("title"), Some(&"Test".to_string()));
        assert!(body.contains("Hello"));
    }

    #[test]
    fn test_extract_without_frontmatter() {
        let content = "# Just a header\n\nSome text.";
        let (fields, body) = extract_frontmatter(content);
        assert!(fields.is_empty());
        assert!(body.contains("Just a header"));
    }
}
