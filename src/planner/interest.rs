use crate::config::GlobalConfig;
use crate::types::WikiIndex;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Build a weighted random selection of concepts based on user interests
pub fn weighted_random_select(
    wiki_index: &WikiIndex,
    config: &GlobalConfig,
    count: usize,
    recently_generated: &[String],
) -> Vec<String> {
    if config.interests.is_empty() || count == 0 {
        return Vec::new();
    }

    // Build candidate pool from wiki index
    let mut candidates: Vec<(&str, f64)> = Vec::new();

    for entry in &wiki_index.entries {
        // Skip recently generated concepts
        if recently_generated.iter().any(|r| r.eq_ignore_ascii_case(&entry.title)) {
            continue;
        }

        // Calculate weight based on interest match
        let weight = calculate_interest_weight(&entry.tags, &config.interests);

        if weight > 0.0 {
            candidates.push((entry.title.as_str(), weight));
        }
    }

    if candidates.is_empty() {
        return Vec::new();
    }

    // Weighted random selection without replacement
    let mut rng = thread_rng();
    let mut selected: Vec<String> = Vec::new();
    let mut available: Vec<(&str, f64)> = candidates.clone();

    for _ in 0..count.min(available.len()) {
        if let Some(choice) = weighted_choice(&mut available, &mut rng) {
            selected.push(choice.0.to_string());
            // Remove to avoid double selection
            available.retain(|(title, _)| !title.eq_ignore_ascii_case(choice.0));
        }
    }

    selected
}

/// Calculate weight based on how many interest domains match
fn calculate_interest_weight(tags: &[String], interests: &[String]) -> f64 {
    let matches: usize = tags
        .iter()
        .filter(|tag| {
            interests.iter().any(|interest| {
                tag.starts_with(interest) ||
                // Handle subculture special case
                (*interest == "subculture" && !tag.is_empty())
            })
        })
        .count();

    if matches == 0 {
        0.0
    } else {
        // Base weight + bonus for multiple matches
        1.0 + (matches as f64 * 0.5)
    }
}

/// Choose a random item based on weights
fn weighted_choice<'a, R: rand::Rng>(
    items: &mut Vec<(&'a str, f64)>,
    rng: &mut R,
) -> Option<(&'a str, f64)> {
    if items.is_empty() {
        return None;
    }

    let total_weight: f64 = items.iter().map(|(_, w)| w).sum();

    if total_weight <= 0.0 {
        // Fall back to uniform random
        items.shuffle(rng);
        return items.first().map(|(t, w)| (*t, *w));
    }

    let mut rng_val = rng.gen::<f64>() * total_weight;

    for (title, weight) in items.iter() {
        rng_val -= *weight;
        if rng_val <= 0.0 {
            return Some((*title, *weight));
        }
    }

    // Fallback to last item
    items.last().map(|(t, w)| (*t, *w))
}

/// Suggest a new concept that extends the knowledge graph
pub fn suggest_concept(
    wiki_index: &WikiIndex,
    config: &GlobalConfig,
    _tag_hierarchy: &str,
) -> Option<String> {
    // Use the existing wiki index to find gaps
    // Look for tags with few documents that could be expanded

    let mut tag_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

    for entry in &wiki_index.entries {
        for tag in &entry.tags {
            let top_level = tag.split('/').next().unwrap_or(tag.as_str());
            *tag_counts.entry(top_level).or_insert(0) += 1;
        }
    }

    // Find interest tags with low document count (potential for expansion)
    let mut expansion_candidates: Vec<(&str, usize)> = config
        .interests
        .iter()
        .filter_map(|interest| {
            let count = tag_counts.get(interest.as_str()).copied().unwrap_or(0);
            if count > 0 && count < 10 {
                Some((interest.as_str(), count))
            } else {
                None
            }
        })
        .collect();

    // Sort by count ascending (least documented = most room for expansion)
    expansion_candidates.sort_by_key(|(_, count)| *count);

    expansion_candidates.first().map(|(name, _)| {
        format!("Suggest a {} concept that connects to existing documents", name)
    })
}
