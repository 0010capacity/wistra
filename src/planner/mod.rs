pub mod priority;
pub mod interest;

use crate::scanner::ScanReport;
use crate::config::GlobalConfig;
use crate::types;
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
    Polish,
}

/// Create an execution plan from a scan report
pub fn create_plan(report: &ScanReport, config: &GlobalConfig, slot_count: usize) -> Result<ExecutionPlan> {
    create_plan_with_polish(report, config, slot_count, 0)
}

/// Create an execution plan with optional polish mode.
/// In polish mode, randomly samples published documents instead of generating random new ones.
pub fn create_plan_with_polish(
    report: &ScanReport,
    config: &GlobalConfig,
    slot_count: usize,
    polish_count: usize,
) -> Result<ExecutionPlan> {
    let mut slots = Vec::new();

    // Calculate slot allocation using priority module
    let allocation = priority::calculate_slot_allocation_with_polish(report, slot_count, polish_count);

    // Priority 1: Disambiguation resolution (always first)
    for candidate in &report.disambig_candidates {
        slots.push(PlanSlot {
            action: PlanAction::Disambiguation,
            target: candidate.title.clone(),
            details: format!("{} documents share this title", candidate.documents.len()),
        });
    }

    // Priority 2: Stub fill (sorted by inbound link count using priority module)
    let sorted_stubs = priority::sort_stub_candidates(report);
    for candidate in sorted_stubs.iter().take(allocation.stub_count) {
        slots.push(PlanSlot {
            action: PlanAction::Stub,
            target: candidate.target.clone(),
            details: format!("{} inbound links", candidate.inbound_count),
        });
    }

    // Priority 3a: Polish mode — randomly sample published documents
    if allocation.polish_count > 0 {
        let published: Vec<&types::Document> = report
            .documents
            .values()
            .filter(|d| d.status == types::Status::Published)
            .collect();

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let selected: Vec<_> = published.choose_multiple(&mut rng, allocation.polish_count).collect();

        for doc in selected {
            slots.push(PlanSlot {
                action: PlanAction::Polish,
                target: doc.title.clone(),
                details: format!("polish existing document"),
            });
        }
    }
    // Priority 3b: Interest-based random (fills remaining slots when not in polish mode)
    else {
        for i in 0..allocation.random_count {
            slots.push(PlanSlot {
                action: PlanAction::Random,
                target: format!("random-suggestion-{}", i + 1),
                details: "AI-suggested based on interests".to_string(),
            });
        }
    }

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
                PlanAction::Polish => "polish",
            };
            println!("   [{}] {:8} → {} ({})",
                i + 1, action_str, slot.target, slot.details);
        }
    }
}
