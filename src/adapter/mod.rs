pub mod claude;

use anyhow::Result;
use async_trait::async_trait;
use crate::types::{Document, WikiIndex};

/// Context for concept generation
#[derive(Debug, Clone)]
pub struct GenerationContext {
    pub concept_name: String,
    pub concepts_dir: std::path::PathBuf,
    pub wiki_dir: std::path::PathBuf,
    pub related_docs: Vec<String>,
    pub wiki_index: WikiIndex,
    pub language: String,
    pub tag_index: String,
}

/// Context for disambiguation resolution
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DisambigContext {
    pub title: String,
    pub wiki_dir: std::path::PathBuf,
    pub context_a: Vec<String>,
    pub context_b: Vec<String>,
    pub wiki_index: WikiIndex,
    pub language: String,
}

/// Context for concept suggestion
#[derive(Debug, Clone)]
pub struct SuggestionContext {
    pub wiki_dir: std::path::PathBuf,
    pub wiki_index: WikiIndex,
    pub interests: Vec<String>,
    pub language: String,
    pub tag_index: String,
}

/// Result of concept suggestion
#[derive(Debug, Clone)]
pub struct SuggestedConcept {
    pub title: String,
    pub reason: String,
    pub related_existing: Vec<String>,
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
#[allow(dead_code)]
pub struct LinkUpdate {
    pub source_file: String,
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

    /// Suggest a new concept based on interests and existing wiki
    async fn suggest_concept(&self, ctx: SuggestionContext) -> Result<SuggestedConcept>;
}
