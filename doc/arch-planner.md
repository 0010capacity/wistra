# Planner Architecture

## Overview

The planner creates execution plans for the `run` command, determining what concepts to generate and in what order.

## Execution Plan

```rust
pub struct ExecutionPlan {
    pub slots: Vec<PlanSlot>,
}

pub struct PlanSlot {
    pub action: PlanAction,
    pub target: String,
    pub details: String,
}

pub enum PlanAction {
    Disambiguation,  // Resolve ambiguous titles first
    Stub,            // Fill stub documents
    Random,          // Interest-based random selection
}
```

## Slot Priority Logic

```
1st priority  Disambiguation resolution   (always first, blocks graph integrity)
2nd priority  Stub fill                  (sorted by inbound link count, descending)
3rd priority  Interest-based random      (fills remaining slots)
```

## Priority Module

`priority.rs` provides:
- `calculate_slot_allocation()`: Distribute slots across priorities
- `sort_stub_candidates()`: Sort stubs by inbound link count
- `select_random_concepts()`: Filter by user interests

## Interest Module

`interest.rs` provides:
- `weighted_random_select()`: Weighted random selection based on interests
- `calculate_interest_weight()`: Weight based on tag matches
- `suggest_concept()`: Find gaps in the knowledge graph

## Configuration

User interests are defined in `GlobalConfig.interests`:
```rust
pub const INTEREST_DOMAINS: &[(&str, &str)] = &[
    ("science", "Science"),
    ("mathematics", "Mathematics"),
    ("programming", "Programming"),
    ("computer-science", "Computer Science"),
    ("history", "History"),
    ("culture", "Culture"),
    ("current-affairs", "Current Affairs"),
    ("subculture", "Subculture"),
    ("economics", "Economics"),
    ("philosophy", "Philosophy"),
];
```
