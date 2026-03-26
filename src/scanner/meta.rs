use crate::config::WikiConfig;
use crate::scanner::ScanReport;
use anyhow::Result;
use std::path::PathBuf;

/// Generate meta files based on scan report
pub fn generate_meta_files(wiki_config: &WikiConfig, report: &ScanReport) -> Result<()> {
    let meta_dir = wiki_config.meta_dir();
    std::fs::create_dir_all(&meta_dir)?;

    generate_stubs_md(&meta_dir, report)?;
    generate_disambig_queue_md(&meta_dir, report)?;
    generate_tag_index_md(&meta_dir, report)?;

    Ok(())
}

fn generate_stubs_md(meta_dir: &PathBuf, report: &ScanReport) -> Result<()> {
    let mut content = String::new();

    content.push_str("---\n");
    content.push_str("title: Stubs\n");
    content.push_str("status: meta\n");
    content.push_str("---\n\n");

    content.push_str("## Stub Candidates\n\n");
    content.push_str("The following concepts are linked but do not yet exist:\n\n");

    if report.stub_candidates.is_empty() {
        content.push_str("_No stub candidates found._\n");
    } else {
        content.push_str("| Concept | Inbound Links |\n");
        content.push_str("|---------|---------------|\n");
        for stub in &report.stub_candidates {
            content.push_str(&format!("| [[{}]] | {} |\n", stub.target, stub.inbound_count));
        }
    }

    content.push_str("\n## Missing Targets\n\n");
    content.push_str("These are referenced in wikilinks but have no corresponding file:\n\n");

    for stub in &report.stub_candidates {
        content.push_str(&format!("- [[{}]]\n", stub.target));
    }

    if report.stub_candidates.is_empty() {
        content.push_str("_No missing targets._\n");
    }

    std::fs::write(meta_dir.join("stubs.md"), content)?;
    Ok(())
}

fn generate_disambig_queue_md(meta_dir: &PathBuf, report: &ScanReport) -> Result<()> {
    let mut content = String::new();

    content.push_str("---\n");
    content.push_str("title: Disambiguation Queue\n");
    content.push_str("status: meta\n");
    content.push_str("---\n\n");

    content.push_str("## Pending Disambiguation\n\n");
    content.push_str("These titles appear in multiple documents and need disambiguation:\n\n");

    if report.disambig_candidates.is_empty() {
        content.push_str("_No disambiguation needed._\n");
    } else {
        for candidate in &report.disambig_candidates {
            content.push_str(&format!("### {}\n\n", candidate.title));
            content.push_str(&format!("Appears in {} documents:\n\n", candidate.documents.len()));
            for doc in &candidate.documents {
                content.push_str(&format!("- [[{}]]\n", doc.replace(".md", "")));
            }
            content.push_str("\n");
        }
    }

    std::fs::write(meta_dir.join("disambig-queue.md"), content)?;
    Ok(())
}

fn generate_tag_index_md(meta_dir: &PathBuf, report: &ScanReport) -> Result<()> {
    let mut content = String::new();

    content.push_str("---\n");
    content.push_str("title: Tag Index\n");
    content.push_str("status: meta\n");
    content.push_str("---\n\n");

    content.push_str("## Tags\n\n");

    if report.tag_stats.tag_counts.is_empty() {
        content.push_str("_No tags found._\n");
    } else {
        // Group tags by top-level category
        let mut tag_groups: std::collections::HashMap<String, Vec<(String, usize)>> = std::collections::HashMap::new();

        for (tag, count) in &report.tag_stats.tag_counts {
            let category = if let Some(pos) = tag.find('/') {
                &tag[..pos]
            } else {
                tag.as_str()
            };
            tag_groups
                .entry(category.to_string())
                .or_insert_with(Vec::new)
                .push((tag.clone(), *count));
        }

        let mut categories: Vec<&String> = tag_groups.keys().collect();
        categories.sort();

        for category in categories {
            let tags = tag_groups.get(category).unwrap();
            content.push_str(&format!("### {}\n\n", category));

            let mut sorted_tags = tags.clone();
            sorted_tags.sort_by(|a, b| a.0.cmp(&b.0));

            for (tag, count) in sorted_tags {
                content.push_str(&format!("- [[{}]] ({})\n", tag, count));
            }
            content.push_str("\n");
        }
    }

    std::fs::write(meta_dir.join("tag-index.md"), content)?;
    Ok(())
}
