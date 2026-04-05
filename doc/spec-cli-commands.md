# CLI Commands Specification

## Core Commands

### onboard
Run the onboarding wizard to set up wistra.
```
wistra onboard
```

### run
Grow the wiki by generating concepts.
```
wistra run [path] [count]
  --dry-run       # Preview without writing
  --quiet         # Minimal output
  --no-confirm    # Skip confirmation prompts
```

### scan
Scan wiki and print detailed report.
```
wistra scan [path]
```

### config
Modify configuration.
```
wistra config [--onboard]
```

### status
Print compact status summary.
```
wistra status [path]
```

### interests
Modify interest domains.
```
wistra interests
```

## Document Management Commands

### rename
Rename a document and update all links.
```
wistra rename <old_title> <new_title> [path] [--dry-run]
```

### merge
Merge source document into target.
```
wistra merge <source> <target> [path] [--dry-run]
```

### delete
Delete a document and clean up links.
```
wistra delete <title> [path] [--dry-run] [--no-confirm]
```

## Query Commands

### backlinks
Show documents that link to a specific document.
```
wistra backlinks <title> [path] [--json]
```

### orphans
Find orphaned documents (no incoming links).
```
wistra orphans [path] [--exclude-stubs] [--sort created|tags] [--json]
```

### search
Full-text search across documents.
```
wistra search <query> [path] [--case-sensitive] [--regex] [--json]
```

### graph
Show document connection graph.
```
wistra graph <title> [path] [--depth N] [--incoming] [--outgoing] [--json]
```

### stats
Extended statistics.
```
wistra stats [basic|trends|tags|links] [path] [--json]
```

## Tag Commands

### tags list
List all tags with document counts.
```
wistra tags list [path]
```

### tags rename
Rename a tag across all documents.
```
wistra tags rename <old_tag> <new_tag> [path] [--dry-run]
```

### tags merge
Merge source tag into target tag.
```
wistra tags merge <source> <target> [path] [--dry-run]
```

### tags orphans
Find unused tags (no documents).
```
wistra tags orphans [path]
```

## Serve & Export Commands

### serve
Start HTTP server to browse wiki locally.
```
wistra serve [path] [--port N] [--host ADDR] [--open]
```

Options:
- `--port, -p` — Port to listen on (default: 15432)
- `--host` — Host address to bind (default: 127.0.0.1)
- `--open, -o` — Open browser automatically

### export
Export wiki as static site for hosting.
```
wistra export [path] [-o DIR] [--hosting TARGET] [--project NAME] [--deploy]
```

Options:
- `--output, -o` — Output directory (default: dist)
- `--hosting` — Hosting target: firebase, cloudflare, or both (default: firebase)
- `--project` — Project name (auto-derived from wiki name if not set)
- `--deploy` — Deploy to hosting immediately after export

Examples:
```bash
# Export for Cloudflare Pages
wistra export --hosting=cloudflare

# Export and deploy to Cloudflare (creates project if needed)
wistra export --hosting=cloudflare --deploy

# Export for Firebase Hosting
wistra export --hosting=firebase --deploy

# Export with custom project name
wistra export --hosting=cloudflare --deploy --project my-wiki
```

## Utilities

### dedup
Detect duplicate or similar documents.
```
wistra dedup [path] [--threshold N] [--json]
```

### clean
Detect and fix wiki technical debt.
```
wistra clean [path] [--dry-run] [--fix] [--json]
```

### import
Import external markdown files.
```
wistra import <source> [path] [--dry-run] [--json]
```

### cron
Manage scheduled runs.
```
wistra cron [--set HH:MM] [--install] [--show] [--remove] [--no-git]
```
