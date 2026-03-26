use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Document status enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Stub,
    Published,
    Disambiguation,
    Meta,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Stub => write!(f, "stub"),
            Status::Published => write!(f, "published"),
            Status::Disambiguation => write!(f, "disambiguation"),
            Status::Meta => write!(f, "meta"),
        }
    }
}

/// Represents a wiki document with frontmatter and body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub aliases: Vec<String>,
    pub tags: Vec<String>,
    pub status: Status,
    pub language: String,
    pub created: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disambig: Option<String>,
    pub body: String,
}

impl Document {
    /// Create a new stub document
    pub fn new_stub(title: String, language: String) -> Self {
        Document {
            title: title.clone(),
            aliases: vec![title],
            tags: vec![],
            status: Status::Stub,
            language,
            created: chrono::Local::now().naive_local().date(),
            relates: None,
            disambig: None,
            body: "<!-- stub: created by wistra, pending generation -->".to_string(),
        }
    }

    /// Get the filename for this document
    pub fn filename(&self) -> String {
        format!("{}.md", self.title)
    }
}

/// Represents a wikilink found in document body
#[derive(Debug, Clone)]
pub struct Link {
    /// The raw link target (e.g., "Apple" from [[Apple]])
    pub target: String,
    /// Display text if different from target (e.g., "Display" from [[Apple|Display]])
    pub display: Option<String>,
    /// Section anchor if present (e.g., "section" from [[Apple#section]])
    pub section: Option<String>,
    /// Source file where this link was found
    pub source_file: String,
}

impl Link {
    /// Parse a wikilink from raw text
    pub fn parse(raw: &str, source_file: String) -> Option<Self> {
        // Handle [[target#section|display]] format
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

        let target = target.trim().to_string();
        if target.is_empty() {
            return None;
        }

        Some(Link {
            target,
            display,
            section,
            source_file,
        })
    }

    /// Reconstruct the link as wikilink syntax
    pub fn to_wikilink(&self) -> String {
        let mut result = format!("[[{}", self.target);
        if let Some(ref section) = self.section {
            result.push_str(&format!("#{}", section));
        }
        if let Some(ref display) = self.display {
            result.push_str(&format!("|{}", display));
        }
        result.push_str("]]");
        result
    }
}

/// Compressed wiki index entry for passing to AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiIndexEntry {
    pub title: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub summary: String,
    pub status: Status,
}

/// Wiki index for AI context
#[derive(Debug, Clone, Default)]
pub struct WikiIndex {
    pub entries: Vec<WikiIndexEntry>,
}

impl WikiIndex {
    pub fn new() -> Self {
        WikiIndex { entries: vec![] }
    }

    /// Find entry by title or alias
    pub fn find(&self, title: &str) -> Option<&WikiIndexEntry> {
        self.entries.iter().find(|e| {
            e.title.eq_ignore_ascii_case(title)
                || e.aliases.iter().any(|a| a.eq_ignore_ascii_case(title))
        })
    }

    /// Get all titles
    pub fn titles(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.title.as_str()).collect()
    }
}

/// Represents the link graph of the wiki
#[derive(Debug, Clone, Default)]
pub struct LinkGraph {
    /// All links grouped by source file
    pub outgoing_links: std::collections::HashMap<String, Vec<Link>>,
    /// All links grouped by target
    pub incoming_links: std::collections::HashMap<String, Vec<Link>>,
}

impl LinkGraph {
    pub fn new() -> Self {
        LinkGraph {
            outgoing_links: std::collections::HashMap::new(),
            incoming_links: std::collections::HashMap::new(),
        }
    }

    /// Add a link to the graph
    pub fn add_link(&mut self, link: Link) {
        let source = link.source_file.clone();
        let target = link.target.clone();

        self.outgoing_links
            .entry(source)
            .or_insert_with(Vec::new)
            .push(link.clone());

        self.incoming_links
            .entry(target)
            .or_insert_with(Vec::new)
            .push(link);
    }

    /// Get inbound link count for a target
    pub fn inbound_count(&self, target: &str) -> usize {
        self.incoming_links
            .get(target)
            .map(|links| links.len())
            .unwrap_or(0)
    }
}
