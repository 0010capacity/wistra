# ADR-002: Include Serve in Scope

## Status
Accepted

## Context
The original wistra-plan.md explicitly marked "Rendering or serving documents" as out of scope. However, users have requested the ability to browse their wiki without Obsidian.

## Decision
Include `serve` as a first-class command.

## Rationale
- Serve is read-only (no mutations) - low risk
- Single binary keeps distribution simple
- Provides basic browsing without external dependencies
- Lightweight implementation using pulldown-cmark

## Consequences
- Adds one dependency (pulldown-cmark for markdown rendering)
- Increases binary size slightly
- Maintenance burden for HTTP server code
