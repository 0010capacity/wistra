use crate::config::GlobalConfig;
use crate::scanner::ScanReport;
use anyhow::Result;

/// Calculate how many slots to allocate to each priority level
#[allow(dead_code)]
pub fn calculate_slot_allocation(
    report: &ScanReport,
    _config: &GlobalConfig,
    total_slots: usize,
) -> SlotAllocation {
    let disambig_count = report.disambig_candidates.len();
    let remaining_after_disambig = total_slots.saturating_sub(disambig_count);

    let stub_count = report.stub_candidates.len().min(remaining_after_disambig);
    let remaining_after_stubs = remaining_after_disambig.saturating_sub(stub_count);

    SlotAllocation {
        disambig_count,
        stub_count,
        random_count: remaining_after_stubs,
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SlotAllocation {
    pub disambig_count: usize,
    pub stub_count: usize,
    pub random_count: usize,
}

/// Sort stub candidates by inbound link count (descending)
#[allow(dead_code)]
pub fn sort_stub_candidates(report: &ScanReport) -> Vec<StubCandidateWithCount> {
    let mut candidates: Vec<StubCandidateWithCount> = report
        .stub_candidates
        .iter()
        .map(|s| StubCandidateWithCount {
            target: s.target.clone(),
            inbound_count: s.inbound_count,
        })
        .collect();

    candidates.sort_by(|a, b| b.inbound_count.cmp(&a.inbound_count));
    candidates
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StubCandidateWithCount {
    pub target: String,
    pub inbound_count: usize,
}

/// Select random concepts based on user interests
#[allow(dead_code)]
pub fn select_random_concepts(
    report: &ScanReport,
    config: &GlobalConfig,
    count: usize,
) -> Result<Vec<RandomConcept>> {
    if count == 0 {
        return Ok(Vec::new());
    }

    // Build a list of candidate concepts based on interests
    // For now, we filter existing documents by interest tags
    let mut candidates: Vec<RandomConcept> = Vec::new();

    for entry in &report.wiki_index.entries {
        // Check if any of the entry's tags match user interests
        let matching_interests: Vec<String> = entry
            .tags
            .iter()
            .filter(|tag| {
                config.interests.iter().any(|interest| {
                    tag.starts_with(interest) || interest == "subculture"
                })
            })
            .cloned()
            .collect();

        if !matching_interests.is_empty() {
            candidates.push(RandomConcept {
                title: entry.title.clone(),
                matched_interests: matching_interests,
                // Lower weight for recently added concepts (would need state tracking)
                weight: 1.0,
            });
        }
    }

    // Sort by weight descending and take top candidates
    candidates.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
    candidates.truncate(count);

    Ok(candidates)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RandomConcept {
    pub title: String,
    pub matched_interests: Vec<String>,
    pub weight: f64,
}
