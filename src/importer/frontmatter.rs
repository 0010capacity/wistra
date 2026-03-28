use std::collections::HashMap;

/// Get a string field from parsed YAML
pub fn get_string(fields: &HashMap<String, String>, key: &str) -> Option<String> {
    fields.get(key).cloned()
}

/// Get a list of strings from a YAML field
pub fn get_string_list(fields: &HashMap<String, String>, key: &str) -> Option<Vec<String>> {
    fields.get(key).map(|s| parse_yaml_list(s))
}

/// Parse a YAML list string like "[item1, item2]"
pub fn parse_yaml_list(s: &str) -> Vec<String> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        // Try to parse as inline list: item1, item2
        return s
            .split(',')
            .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    let content = &s[1..s.len() - 1];
    content
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Generate aliases from content
pub fn generate_aliases(body: &str, title: &str) -> Option<Vec<String>> {
    let mut aliases: Vec<String> = Vec::new();

    // Add the title itself
    aliases.push(title.to_string());

    // Try to extract first heading (usually H1)
    let lines: Vec<&str> = body.lines().collect();
    for line in lines.iter().take(10) {
        let line = line.trim();
        if line.starts_with("# ") {
            let heading = line[2..].trim().to_string();
            if !heading.is_empty() && heading.to_lowercase() != title.to_lowercase() {
                aliases.push(heading);
            }
            break;
        }
    }

    // If we only have the title, don't return aliases
    if aliases.len() <= 1 {
        None
    } else {
        Some(aliases)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml_list_brackets() {
        let input = "[tag1, tag2, tag3]";
        let result = parse_yaml_list(input);
        assert_eq!(result, vec!["tag1", "tag2", "tag3"]);
    }

    #[test]
    fn test_parse_yaml_list_inline() {
        let input = "tag1, tag2, tag3";
        let result = parse_yaml_list(input);
        assert_eq!(result, vec!["tag1", "tag2", "tag3"]);
    }

    #[test]
    fn test_generate_aliases() {
        let body = "# My Document Title\n\nSome content here.";
        let aliases = generate_aliases(body, "My Document");
        assert!(aliases.is_some());
        let aliases = aliases.unwrap();
        assert!(aliases.contains(&"My Document".to_string()));
        assert!(aliases.contains(&"My Document Title".to_string()));
    }
}
