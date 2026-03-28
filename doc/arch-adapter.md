# Adapter Architecture

## Overview

The adapter layer provides an abstraction for AI providers. This allows wistra to support multiple AI backends while maintaining a consistent interface.

## WikiAdapter Trait

```rust
#[async_trait]
pub trait WikiAdapter: Send + Sync {
    async fn generate_concept(&self, ctx: GenerationContext) -> Result<Document>;
    async fn resolve_disambiguation(&self, ctx: DisambigContext) -> Result<DisambigResult>;
    async fn suggest_concept(&self, ctx: SuggestionContext) -> Result<SuggestedConcept>;
}
```

## Context Types

### GenerationContext
Used for generating a new concept document.
- `concept_name`: Target concept name
- `related_docs`: Existing documents to link from
- `wiki_index`: Compressed wiki state for AI context
- `language`: Document body language
- `tag_index`: Tag hierarchy string

### DisambigContext
Used for resolving ambiguous titles.
- `title`: Ambiguous title
- `context_a`, `context_b`: Context from linking documents
- `wiki_index`: Wiki state
- `language`: Document body language

### SuggestionContext
Used for suggesting new concepts based on interests.
- `wiki_index`: Wiki state
- `interests`: User's interest domains
- `language`: Document body language
- `tag_index`: Tag hierarchy

## Claude CLI Adapter

Current implementation uses Claude Code CLI installed on the user's machine:

- **CLI**: `claude -p <prompt> --output-format text`
- **Installation**: Claude Code CLI must be installed and authenticated
- **API key**: Not required (CLI handles authentication)

### CLI Installation

```bash
# Install Claude Code CLI
npm install -g @anthropic-ai/claude-code

# Authenticate
claude auth
```

## Future Adapters

The trait design allows for future adapters:
- Claude API (HTTP)
- GPT (OpenAI)
- Gemini (Google)
- Local LLM (Ollama, llama.cpp)
