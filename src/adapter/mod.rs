pub mod claude;

use anyhow::Result;
use async_trait::async_trait;
use crate::types::{Document, WikiIndex};

/// Context for concept generation
#[derive(Debug, Clone)]
pub struct GenerationContext {
    pub concept_name: String,
    pub related_docs: Vec<String>,
    pub wiki_index: WikiIndex,
    pub language: String,
    pub tag_index: String,
}

/// Context for disambiguation resolution
#[derive(Debug, Clone)]
pub struct DisambigContext {
    pub title: String,
    pub context_a: Vec<String>,
    pub context_b: Vec<String>,
    pub wiki_index: WikiIndex,
    pub language: String,
}

/// Result of disambiguation resolution
#[derive(Debug, Clone)]
pub struct DisambigResult {
    pub concept_a: DisambigConcept,
    pub concept_b: DisambigConcept,
    pub disambig: DisambigConcept,
    pub link_updates: Vec<LinkUpdate>,
}

#[derive(Debug, Clone)]
pub struct DisambigConcept {
    pub new_title: String,
    pub frontmatter: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct LinkUpdate {
    pub file: String,
    pub from: String,
    pub to: String,
}

/// Trait for wiki adapters
#[async_trait]
pub trait WikiAdapter: Send + Sync {
    /// Generate a concept document
    async fn generate_concept(&self, ctx: GenerationContext) -> Result<Document>;

    /// Resolve a disambiguation
    async fn resolve_disambiguation(&self, ctx: DisambigContext) -> Result<DisambigResult>;
}
