pub mod priority;
pub mod interest;

use crate::scanner::ScanReport;
use crate::config::GlobalConfig;
use anyhow::Result;

/// Execution plan for a run
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub slots: Vec<PlanSlot>,
}

#[derive(Debug, Clone)]
pub struct PlanSlot {
    pub action: PlanAction,
    pub target: String,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlanAction {
    Disambiguation,
    Stub,
    Random,
}

/// Create an execution plan from a scan report
pub fn create_plan(report: &ScanReport, _config: &GlobalConfig, slot_count: usize) -> Result<ExecutionPlan> {
    let mut slots = Vec::new();

    // Priority 1: Disambiguation resolution (always first)
    for candidate in &report.disambig_candidates {
        slots.push(PlanSlot {
            action: PlanAction::Disambiguation,
            target: candidate.title.clone(),
            details: format!("{} documents share this title", candidate.documents.len()),
        });
    }

    // Priority 2: Stub fill (sorted by inbound link count)
    let remaining = slot_count.saturating_sub(slots.len());
    for candidate in report.stub_candidates.iter().take(remaining) {
        slots.push(PlanSlot {
            action: PlanAction::Stub,
            target: candidate.target.clone(),
            details: format!("{} inbound links", candidate.inbound_count),
        });
    }

    // Priority 3: Interest-based random (not implemented yet)
    // Would need AI to suggest concepts

    Ok(ExecutionPlan { slots })
}

impl ExecutionPlan {
    /// Print the plan
    pub fn print(&self) {
        println!("📋 Execution plan ({} slots):", self.slots.len());
        for (i, slot) in self.slots.iter().enumerate() {
            let action_str = match slot.action {
                PlanAction::Disambiguation => "disambig",
                PlanAction::Stub => "stub",
                PlanAction::Random => "random",
            };
            println!("   [{}] {:8} → {} ({})",
                i + 1, action_str, slot.target, slot.details);
        }
    }
}
