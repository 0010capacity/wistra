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
