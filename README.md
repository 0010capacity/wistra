# wistra

AI-powered personal wiki builder. Scans a knowledge graph, fills stub concepts, resolves disambiguation, and keeps everything connected — entirely from the command line.

## Features

- **AI-Powered Growth**: Automatically generates new concepts and expands stubs using Claude
- **Knowledge Graph**: Tracks links between documents with bidirectional backlinks
- **Local Web UI**: Browse your wiki with a beautiful, responsive web interface
- **Static Export**: Deploy your wiki to Firebase Hosting or Cloudflare Pages with one command
- **Full CLI**: Complete toolkit for managing documents, tags, links, and more

## Installation

```bash
cargo install wistra
```

Or download pre-built binaries from [GitHub Releases](https://github.com/0010capacity/wistra/releases).

## Quick Start

```bash
# Initial setup (configure API key, wiki path, interests)
wistra onboard

# Grow your wiki with AI-generated content
wistra run

# Start local web server to browse your wiki
wistra serve

# Export and deploy to the web
wistra export --deploy
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
| `wistra export` | Export wiki as static site |

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
| `wistra cron` | Manage scheduled runs |

## Deployment

Wistra can export your wiki as a static site and deploy it directly to hosting platforms.

### Cloudflare Pages

Deploy to Cloudflare Pages with automatic project creation:

```bash
# Deploy to Cloudflare (creates project if needed)
wistra export --hosting=cloudflare --deploy

# With custom project name
wistra export --hosting=cloudflare --deploy --project my-wiki
```

Requirements:
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/) installed (`npm install -g wrangler`)
- Cloudflare account authenticated (`wrangler login`)

### Firebase Hosting

Export for Firebase Hosting:

```bash
# Generate static files and firebase.json
wistra export --hosting=firebase

# Then deploy manually
firebase deploy --only hosting
```

Or deploy automatically (requires Firebase CLI):

```bash
wistra export --hosting=firebase --deploy
```

Requirements:
- [Firebase CLI](https://firebase.google.com/docs/cli) installed (`npm install -g firebase-tools`)
- Firebase project configured (`firebase init hosting`)

### Both Platforms

Generate configuration for both platforms at once:

```bash
wistra export --hosting=both --deploy
```

### Export Options

```bash
# Full usage
wistra export [PATH] [OPTIONS]

Options:
  -o, --output <DIR>    Output directory (default: dist)
  --hosting <TARGET>    Hosting target: firebase, cloudflare, or both (default: firebase)
  --project <NAME>      Project name (auto-derived from wiki name if not set)
  --deploy              Deploy to hosting immediately after export
```

## Local Web UI

The `wistra serve` command starts a local web server with a beautiful, responsive interface:

```bash
# Start server on default port (15432)
wistra serve

# Custom port and host
wistra serve --port 8080 --host 0.0.0.0

# Auto-open in browser
wistra serve --open
```

Features:
- **Home**: Recent documents, random picks, wiki statistics
- **All Pages**: Grid/list view with filtering by status and tags
- **Tags**: Browse by tag hierarchy
- **Graph**: Visual knowledge graph
- **Search**: Full-text search across all documents
- **Responsive**: Works on desktop and mobile

## Wiki Format

Wistra uses standard Markdown with YAML frontmatter:

```markdown
---
title: Concept Name
aliases: ["Alternative Name", "Synonym"]
tags: ["category/subcategory", "another-tag"]
status: published
created: 2024-01-15
---

# Concept Name

Content goes here. Link to other concepts with [[Wikilinks]].

You can also use [[Target|display text]] for custom link text.
```

### Status Values

- `published` — Complete, finished document
- `stub` — Placeholder waiting for content expansion
- `disambiguation` — Disambiguation page for ambiguous terms
- `meta` — Index or metadata document

### Wikilinks

Connect your documents with double-bracket links:

- `[[Target]]` — Link to another document
- `[[Target|Display Text]]` — Link with custom display text
- `[[category/subcategory/Target]]` — Link with tag path

### Tags

Organize documents with slash-delimited tag hierarchies:

```yaml
tags:
  - computer-science/ai
  - philosophy/mind
  - projects/personal
```

## Configuration

### Global Config

Stored at `~/.wistra/config.toml`:

```toml
wiki_path = "~/wiki"
language = "en"
interests = ["computer-science", "philosophy"]
daily_count = 3
```

### Per-Wiki Config

Stored at `<wiki>/.wistra/config.toml`:

```toml
[scanner]
include_patterns = ["**/*.md"]
exclude_patterns = ["**/node_modules/**"]
```

## Architecture

```
src/
├── adapter/     # WikiAdapter trait + Claude API integration
├── cli/         # CLI commands (onboard, config)
├── config/      # GlobalConfig, WikiConfig
├── planner/     # Execution planning, slot allocation
├── scanner/     # Wiki parsing, link graph, reports
├── serve/       # HTTP server and web UI
│   ├── renderer # Markdown rendering
│   ├── templates # HTML templates
│   └── exporter # Static site export
├── types/       # Document, Link, LinkGraph, Status
└── writer/      # Document serialization, wikilink rewriting
```

## Requirements

- **Rust 1.70+** — For building from source
- **Claude Code CLI** — For AI-powered features (`wistra run`)
- **Node.js** — For Firebase/Wrangler CLI deployment tools

## License

MIT
