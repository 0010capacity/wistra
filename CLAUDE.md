# Project: wistra

## Overview
AI-powered personal wiki builder. Scans a knowledge graph, fills stub concepts, resolves disambiguation, and keeps everything connected — entirely from the command line.

## Commands
- `cargo build` — Build the project
- `cargo test` — Run tests
- `cargo run -- [command]` — Run wistra CLI

## Stack
- **Language**: Rust 2021
- **CLI**: Clap v4
- **Async**: Tokio
- **Serialization**: Serde (JSON, YAML, TOML)
- **HTTP**: Reqwest
- **UI**: Dialoguer, Indictatif

## Architecture
```
src/
├── adapter/     # WikiAdapter trait + Claude API
├── cli/         # CLI commands (onboard, config)
├── config/      # GlobalConfig, WikiConfig
├── planner/     # Execution planning, slot allocation
├── scanner/     # Wiki parsing, link graph, reports
├── types/       # Document, Link, LinkGraph, Status
└── writer/      # Document serialization, wikilink rewriting
```

## Conventions
- English comments throughout
- Korean user communication (per global CLAUDE.md)
- YAML frontmatter required for all managed documents
- Wikilinks: `[[Target]]` or `[[Target|Display]]`
- Tags: slash-delimited hierarchy (e.g., `computer-science/ai`)

## Watch Out For
- `scanner/graph.rs` is a placeholder — LinkGraph is defined in `types/mod.rs`
- WikiConfig is stored at `<wiki>/.wistra/config.toml`
- GlobalConfig is stored at `~/.wistra/config.toml`
- API key stored using `secrecy` crate to prevent logging exposure
