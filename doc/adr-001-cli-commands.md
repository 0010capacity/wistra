# ADR-001: CLI Command Design

## Status
Accepted

## Context

wistra needed a CLI for managing a personal wiki. We evaluated two approaches:

1. **Minimal commands** (`onboard`, `run`, `scan`) - only core functionality
2. **Comprehensive commands** - include document management and query tools

## Decision

We chose **comprehensive commands** with a flat command structure:

### Rationale

- **Onboarding clarity**: New users need `onboard` as the entry point
- **Daily workflow**: `run` for growth, `scan`/`status` for monitoring
- **Maintenance**: `rename`, `merge`, `delete` for document lifecycle
- **Discovery**: `search`, `backlinks`, `orphans`, `graph` for exploration
- **Organization**: `tags` subcommand for tag management
- **Statistics**: `stats` for insights

## Consequences

### Positive
- Single entry point for all operations
- Familiar command structure for wiki users
- Consistent with tools like Obsidian, Roam, Notion

### Negative
- More commands to implement and maintain
- Larger CLI surface area

## Alternatives Considered

### Subcommands by category
```rust
// Not chosen - adds nesting without benefit
wistra wiki run
wistra doc rename
wistra query search
```

### Separate binaries
```rust
// Not chosen -分散 requires more setup
wistra-run
wistra-scan
wistra-rename
```
