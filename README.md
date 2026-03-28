# wistra

AI-powered personal wiki builder. Scans a knowledge graph, fills stub concepts, resolves disambiguation, and keeps everything connected — entirely from the command line.

## Installation

```bash
cargo install wistra
```

## Quick Start

```bash
# Initial setup (configure API key, wiki path, interests)
wistra onboard

# Grow your wiki with AI-generated content
wistra run

# Scan and see wiki statistics
wistra scan

# Start a local web server to browse your wiki
wistra serve
```

## Commands

### Core

| Command | Description |
|---------|-------------|
| `wistra onboard` | Run the setup wizard |
| `wistra run` | Grow the wiki with AI-generated concepts |
| `wistra scan` | Scan wiki and print detailed report |
| `wistra status` | Print compact status summary |
| `wistra serve` | Start HTTP server to browse wiki |

### Document Management

| Command | Description |
|---------|-------------|
| `wistra rename <old> <new>` | Rename a document and update all links |
| `wistra merge <source> <target>` | Merge two documents |
| `wistra delete <title>` | Delete a document and clean up links |
| `wistra import <path>` | Import external markdown files |

### Analysis

| Command | Description |
|---------|-------------|
| `wistra backlinks <title>` | Show documents linking to a document |
| `wistra orphans` | Find documents with no incoming links |
| `wistra search <query>` | Full-text search across documents |
| `wistra graph <title>` | Show document connection graph |
| `wistra stats [type]` | Extended statistics (basic, trends, tags, links) |
| `wistra dedup` | Detect duplicate or similar documents |
| `wistra clean` | Detect and fix wiki technical debt |

### Tags

| Command | Description |
|---------|-------------|
| `wistra tags list` | List all tags with document counts |
| `wistra tags rename <old> <new>` | Rename a tag across all documents |
| `wistra tags merge <source> <target>` | Merge tags |
| `wistra tags orphans` | Find unused tags |

### Configuration

| Command | Description |
|---------|-------------|
| `wistra config` | Modify configuration |
| `wistra interests` | Modify interest domains |

## Wiki Format

Wistra uses standard Markdown with YAML frontmatter:

```markdown
---
title: Concept Name
aliases: ["Alternative Name"]
tags: ["category/subcategory"]
status: published
created: 2024-01-15
---

# Concept Name

Content goes here. Link to other concepts with [[Wikilinks]].

You can also use [[Target|display text]].
```

### Status Values

- `published` — Complete document
- `stub` — Placeholder waiting for content
- `disambiguation` — Disambiguation page
- `meta` — Index/meta document

## Configuration

Global configuration is stored at `~/.wistra/config.toml`:

```toml
wiki_path = "~/wiki"
language = "en"
interests = ["computer-science", "philosophy"]
```

Per-wiki configuration is stored at `<wiki>/.wistra/config.toml`.

## Requirements

- Rust 1.70+
- Claude Code CLI (for AI-powered features)

## License

MIT
