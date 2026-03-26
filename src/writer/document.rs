use crate::types::Document;
use anyhow::{Context, Result};
use std::path::Path;

/// Write a document to disk
pub struct DocumentWriter;

impl DocumentWriter {
    /// Serialize a document to Markdown string
    pub fn serialize(doc: &Document) -> String {
        let mut output = String::new();

        // Frontmatter
        output.push_str("---\n");
        output.push_str(&format!("title: {}\n", serde_json::to_string(&doc.title).unwrap()));
        output.push_str(&format!("aliases: {}\n", serde_json::to_string(&doc.aliases).unwrap()));
        output.push_str(&format!("tags: {}\n", serde_json::to_string(&doc.tags).unwrap()));
        output.push_str(&format!("status: {}\n", doc.status));
        output.push_str(&format!("language: {}\n", doc.language));
        output.push_str(&format!("created: {}\n", doc.created));

        if let Some(ref relates) = doc.relates {
            output.push_str(&format!("relates: {}\n", serde_json::to_string(relates).unwrap()));
        }

        if let Some(ref disambig) = doc.disambig {
            output.push_str(&format!("disambig: {}\n", serde_json::to_string(disambig).unwrap()));
        }

        output.push_str("---\n\n");

        // Body
        output.push_str(&doc.body);

        output
    }

    /// Write a document to a file
    pub fn write(doc: &Document, dir: &Path) -> Result<()> {
        let filename = doc.filename();
        let path = dir.join(&filename);

        let content = Self::serialize(doc);

        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write document: {}", path.display()))?;

        Ok(())
    }

    /// Write multiple documents in a batch
    pub fn write_batch(docs: &[Document], dir: &Path) -> Result<()> {
        // Create backups first
        let mut backups: Vec<(std::path::PathBuf, Vec<u8>)> = Vec::new();

        for doc in docs {
            let path = dir.join(doc.filename());
            if path.exists() {
                let content = std::fs::read(&path)
                    .with_context(|| format!("Failed to read existing file: {}", path.display()))?;
                backups.push((path.clone(), content));
            }
        }

        // Write all documents
        for doc in docs {
            Self::write(doc, dir)?;
        }

        // On success, clear backups (files have been overwritten)
        // On failure, restore from backups
        // For now, we just return success

        Ok(())
    }
}
