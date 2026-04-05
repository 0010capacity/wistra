/// Markdown to HTML renderer with wikilink and LaTeX support

use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

/// Render markdown to HTML with wikilink resolution
pub fn render_markdown(markdown: &str) -> String {
    // Protect LaTeX from markdown processing, then restore after rendering
    let (processed, placeholders) = protect_latex(markdown);
    let processed = convert_wikilinks(&processed);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&processed, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Restore LaTeX placeholders
    for (placeholder, original) in &placeholders {
        html_output = html_output.replace(placeholder, original);
    }

    html_output
}

/// Replace LaTeX expressions with placeholders to survive markdown processing.
/// Display math ($$...$$) is replaced first, then inline ($...$).
fn protect_latex(text: &str) -> (String, Vec<(String, String)>) {
    let mut result = text.to_string();
    let mut placeholders: Vec<(String, String)> = Vec::new();
    let mut counter = 0;

    // Protect display math: $$...$$
    let display_re = Regex::new(r"\$\$([\s\S]*?)\$\$").unwrap();
    result = display_re
        .replace_all(&result, |caps: &regex::Captures| {
            let key = format!("%%LATEX_DISPLAY_{}%%", counter);
            counter += 1;
            placeholders.push((key.clone(), format!("$${}$$", &caps[1])));
            key
        })
        .to_string();

    // Protect inline math: $...$
    // After display math is replaced, remaining $ are inline.
    // Use simple pattern: $ (non-$ content) $
    let inline_re = Regex::new(r"\$([^$\n]+?)\$").unwrap();
    result = inline_re
        .replace_all(&result, |caps: &regex::Captures| {
            let key = format!("%%LATEX_INLINE_{}%%", counter);
            counter += 1;
            placeholders.push((key.clone(), format!("${}$", &caps[1])));
            key
        })
        .to_string();

    (result, placeholders)
}

/// Truncate a UTF-8 string to a maximum number of characters, appending "..." if truncated.
pub fn truncate_utf8(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let boundary = s.char_indices()
            .nth(max_chars)
            .map(|(i, _)| i)
            .unwrap_or(s.len());
        format!("{}...", &s[..boundary])
    }
}

/// Heading extracted from HTML for TOC generation
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Extract h2 and h3 headings from HTML in document order.
/// Handles both bare headings and headings with HTML attributes (e.g. <h2 id="foo">).
pub fn extract_headings(html: &str) -> Vec<Heading> {
    let h2_re = regex::Regex::new(r"<h2(?:\s[^>]*)?>(.*?)</h2>").unwrap();
    let h3_re = regex::Regex::new(r"<h3(?:\s[^>]*)?>(.*?)</h3>").unwrap();
    let mut headings: Vec<(usize, u8, String)> = Vec::new();

    for cap in h2_re.captures_iter(html) {
        let pos = cap.get(0).map(|m| m.start()).unwrap_or(0);
        headings.push((pos, 2, cap[1].to_string()));
    }
    for cap in h3_re.captures_iter(html) {
        let pos = cap.get(0).map(|m| m.start()).unwrap_or(0);
        headings.push((pos, 3, cap[1].to_string()));
    }
    headings.sort_by_key(|(pos, _, _)| *pos);

    headings
        .into_iter()
        .map(|(_, level, text)| {
            let id = text.replace(' ', "-");
            Heading { level, id, text }
        })
        .collect()
}

/// Convert [[wikilinks]] to HTML links
fn convert_wikilinks(text: &str) -> String {
    let re = Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap();

    re.replace_all(text, |caps: &regex::Captures| {
        let target = caps.get(1).map_or("", |m| m.as_str());
        let display = caps.get(2).map_or(target, |m| m.as_str());
        let encoded = urlencoding::encode(target);
        format!(r#"<a href="/page/{}">{}</a>"#, encoded, display)
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n\nThis is **bold** and *italic*.";
        let html = render_markdown(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_wikilink_simple() {
        let md = "See [[Python]] for more.";
        let html = render_markdown(md);
        assert!(html.contains("/page/Python"));
    }

    #[test]
    fn test_wikilink_with_display() {
        let md = "Check out [[Python|Programming Language]].";
        let html = render_markdown(md);
        assert!(html.contains("/page/Python"));
        assert!(html.contains("Programming Language"));
    }

    #[test]
    fn test_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let html = render_markdown(md);
        assert!(html.contains("<pre>"));
        assert!(html.contains("<code"));
    }

    #[test]
    fn test_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(md);
        assert!(html.contains("<table>"));
    }

    #[test]
    fn test_inline_latex_preserved() {
        let md = "The formula $E = mc^2$ is famous.";
        let html = render_markdown(md);
        assert!(html.contains("$E = mc^2$"));
    }

    #[test]
    fn test_display_latex_preserved() {
        let md = "Block math:\n\n$$\\int_0^1 x dx$$\n\nDone.";
        let html = render_markdown(md);
        assert!(html.contains("$$"));
    }

    #[test]
    fn test_truncate_utf8_short() {
        assert_eq!(truncate_utf8("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_utf8_exact() {
        assert_eq!(truncate_utf8("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_utf8_truncate() {
        assert_eq!(truncate_utf8("hello world", 5), "hello...");
    }

    #[test]
    fn test_truncate_utf8_korean() {
        assert_eq!(truncate_utf8("머신러닝알고리즘", 4), "머신러닝...");
    }

    #[test]
    fn test_truncate_utf8_empty() {
        assert_eq!(truncate_utf8("", 10), "");
    }

    #[test]
    fn test_extract_headings_basic() {
        let html = r#"<h2>개요</h2><p>text</p><h3>상세</h3><p>more</p><h2>결론</h2>"#;
        let headings = extract_headings(html);
        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].level, 2);
        assert_eq!(headings[0].text, "개요");
        assert_eq!(headings[1].level, 3);
        assert_eq!(headings[1].text, "상세");
        assert_eq!(headings[2].level, 2);
        assert_eq!(headings[2].text, "결론");
    }

    #[test]
    fn test_extract_headings_empty() {
        let headings = extract_headings("<p>no headings</p>");
        assert!(headings.is_empty());
    }

    #[test]
    fn test_extract_headings_id_from_korean() {
        let html = r#"<h2>주요 유형</h2>"#;
        let headings = extract_headings(html);
        assert_eq!(headings[0].id, "주요-유형");
    }

    #[test]
    fn test_extract_headings_with_attributes() {
        // Headings with id attributes (as produced by pulldown-cmark after Task 3)
        let html = r#"<h2 id="개요">개요</h2><p>text</p><h3 id="상세">상세</h3>"#;
        let headings = extract_headings(html);
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].text, "개요");
        assert_eq!(headings[0].level, 2);
        assert_eq!(headings[1].text, "상세");
        assert_eq!(headings[1].level, 3);
    }
}
