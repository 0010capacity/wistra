use crate::cli::cron::{install_cron, parse_time};
use crate::config::{GlobalConfig, WikiConfig, INTEREST_DOMAINS, LANGUAGES, ensure_global_config_dir};
use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::path::PathBuf;
use std::process::Command;

/// Run the onboarding wizard
pub fn run_onboard() -> Result<()> {
    // Detect re-run
    let existing_config = GlobalConfig::load()?;
    let is_rerun = existing_config.is_some();

    // Step 1: Welcome
    if is_rerun {
        println!("🔄 Updating existing configuration\n");
    } else {
        let version = env!("CARGO_PKG_VERSION");
        println!("🚀 Welcome to wistra setup! v{}", version);
        println!("   AI-powered personal wiki builder\n");
    }

    // Step 2: Wiki Identity
    let existing_wiki_name = existing_config
        .as_ref()
        .and_then(|c| {
            c.wiki_path
                .as_ref()
                .and_then(|wp| WikiConfig::load(wp).ok().and_then(|wc| wc.name))
        })
        .unwrap_or_else(|| "My Wiki".to_string());

    let wiki_name: String = Input::new()
        .with_prompt("Wiki name")
        .default(existing_wiki_name)
        .interact_text()
        .context("Failed to read wiki name")?;

    let existing_description = existing_config
        .as_ref()
        .and_then(|c| {
            c.wiki_path
                .as_ref()
                .and_then(|wp| WikiConfig::load(wp).ok().and_then(|wc| wc.description))
        })
        .unwrap_or_default();

    let wiki_description: String = Input::new()
        .with_prompt("Description (optional, one-liner)")
        .allow_empty(true)
        .default(if is_rerun {
            existing_description
        } else {
            String::new()
        })
        .interact_text()
        .context("Failed to read wiki description")?;

    let description = if wiki_description.trim().is_empty() {
        None
    } else {
        Some(wiki_description)
    };

    // Step 3: Wiki Path
    let default_path = existing_config
        .as_ref()
        .and_then(|c| c.wiki_path.clone())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            dirs::home_dir()
                .map(|h| h.join("wiki").to_string_lossy().to_string())
                .unwrap_or_else(|| "~/wiki".to_string())
        });

    let wiki_path_input: String = Input::new()
        .with_prompt("Wiki path")
        .default(default_path)
        .interact_text()
        .context("Failed to read wiki path")?;

    let wiki_path = PathBuf::from(shellexpand::tilde(&wiki_path_input).to_string());

    // Step 4: Language
    let default_lang = existing_config
        .as_ref()
        .map(|c| c.language.as_str())
        .unwrap_or("en");

    let default_lang_idx = LANGUAGES
        .iter()
        .position(|(code, _)| *code == default_lang)
        .unwrap_or(0);

    let language_items: Vec<&&str> = LANGUAGES.iter().map(|(_, name)| name).collect();
    let language_idx = Select::new()
        .with_prompt("Language")
        .items(&language_items)
        .default(default_lang_idx)
        .interact()
        .context("Failed to select language")?;

    let language = LANGUAGES[language_idx].0.to_string();

    // Step 5: Seed Concepts (skip on re-run)
    let seed_concepts = if is_rerun {
        Vec::new()
    } else {
        let seeds: String = Input::new()
            .with_prompt("Seed concepts (comma-separated, optional)")
            .allow_empty(true)
            .default(String::new())
            .interact_text()
            .context("Failed to read seed concepts")?;

        if seeds.trim().is_empty() {
            Vec::new()
        } else {
            seeds
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    };

    // Step 6: Daily Count & Interests
    let default_count = existing_config
        .as_ref()
        .map(|c| c.daily_count)
        .unwrap_or(5);

    let daily_count: usize = Input::new()
        .with_prompt("Daily concept count (higher = more API tokens)")
        .default(default_count)
        .interact_text()
        .context("Failed to read daily count")?;

    let default_interests = existing_config
        .as_ref()
        .map(|c| c.interests.clone())
        .unwrap_or_default();

    let interest_items: Vec<&str> = INTEREST_DOMAINS
        .iter()
        .map(|(_, name)| *name)
        .collect();

    let defaults: Vec<bool> = (0..INTEREST_DOMAINS.len())
        .map(|idx| default_interests.contains(&INTEREST_DOMAINS[idx].0.to_string()))
        .collect();

    let selected_indices = MultiSelect::new()
        .with_prompt("Interests (space to select)")
        .items(&interest_items)
        .defaults(&defaults)
        .interact()
        .context("Failed to select interests")?;

    let interests: Vec<String> = selected_indices
        .iter()
        .map(|&idx| INTEREST_DOMAINS[idx].0.to_string())
        .collect();

    let interest_names: Vec<&str> = selected_indices
        .iter()
        .map(|&idx| INTEREST_DOMAINS[idx].1)
        .collect();

    // Step 7: Cron Setup
    let cron_mode_items = ["Generate new", "Polish existing", "Both (alternate)"];
    let cron_mode_idx = Select::new()
        .with_prompt("Daily run mode")
        .items(&cron_mode_items)
        .default(0)
        .interact()
        .context("Failed to select cron mode")?;

    let use_polish = cron_mode_idx == 1 || cron_mode_idx == 2;

    let setup_cron = Confirm::new()
        .with_prompt("Install cron job?")
        .default(true)
        .interact()
        .context("Failed to confirm cron setup")?;

    let (cron_hour, cron_minute) = if setup_cron {
        let time_str: String = Input::new()
            .with_prompt("Cron time (HH:MM)")
            .default("09:00".to_string())
            .interact_text()
            .context("Failed to read cron time")?;
        parse_time(&time_str)?
    } else {
        (9u8, 0u8)
    };

    // Step 8: Git Init
    let git_init = Confirm::new()
        .with_prompt("Initialize git repository?")
        .default(true)
        .interact()
        .context("Failed to confirm git init")?;

    // Step 9: Summary & Confirmation
    println!("\n───────────────────── Configuration Summary");
    println!("  Wiki name:       {}", wiki_name);
    if let Some(ref desc) = &description {
        println!("  Description:     {}", desc);
    }
    println!("  Wiki path:       {}", wiki_path.display());
    println!("  Language:        {}", language);
    if seed_concepts.is_empty() {
        println!("  Seed concepts:   (none)");
    } else {
        println!("  Seed concepts:   {}", seed_concepts.join(", "));
    }
    println!("  Daily count:     {}", daily_count);
    println!("  Interests:       {}", interest_names.join(", "));
    println!(
        "  Cron mode:       {}",
        cron_mode_items[cron_mode_idx]
    );
    if setup_cron {
        println!("  Cron time:       {:02}:{:02}", cron_hour, cron_minute);
    }
    println!(
        "  Git init:        {}",
        if git_init { "yes" } else { "no" }
    );
    println!();

    let proceed = Confirm::new()
        .with_prompt("Proceed with this configuration?")
        .default(true)
        .interact()
        .context("Failed to confirm")?;

    if !proceed {
        println!("Cancelled.");
        return Ok(());
    }

    // Step 10: Execute
    println!();

    // Create directory structure
    let wiki_config = WikiConfig {
        root_path: wiki_path.clone(),
        name: Some(wiki_name.clone()),
        description,
    };
    wiki_config.ensure_structure()?;
    println!("✅ Directory structure created");

    // Save WikiConfig
    wiki_config.save()?;
    println!(
        "✅ Wiki config saved → {}/.wistra/config.toml",
        wiki_path.display()
    );

    // Save GlobalConfig
    let config = GlobalConfig {
        wiki_path: Some(wiki_path.clone()),
        language,
        daily_count,
        interests,
    };
    ensure_global_config_dir()?;
    config.save()?;
    println!("✅ Global config saved → ~/.wistra/config.toml");

    // Git init
    if git_init {
        init_git_repo(&wiki_path)?;
    }

    // Cron
    if setup_cron {
        println!("\n⏰ Setting up cron job...");
        install_cron(cron_hour, cron_minute, false, use_polish)?;
        println!("✅ Cron job installed");
    }

    // Seed concepts
    if !seed_concepts.is_empty() {
        println!(
            "\n🌱 Generating {} seed concepts...",
            seed_concepts.len()
        );
        println!("   Run `wistra run` to generate them.");
        println!("   This may take a while depending on your API usage.");
    } else {
        println!("\n📌 Next steps:");
        println!("   Run `wistra run` to start growing your wiki.");
        println!("   Run `wistra serve` to browse your wiki.");
        println!("   Run `wistra export` to export as a static site.");
    }

    Ok(())
}

/// Initialize a git repository in the wiki directory
fn init_git_repo(wiki_path: &PathBuf) -> Result<()> {
    // Check if already a git repo
    let git_dir = wiki_path.join(".git");
    if git_dir.exists() {
        println!("✅ Git repository already exists");
        return Ok(());
    }

    let status = Command::new("git")
        .args(["init"])
        .current_dir(wiki_path)
        .status()
        .context("Failed to run git init")?;

    if !status.success() {
        anyhow::bail!("git init failed");
    }

    // Write .gitignore
    let gitignore = wiki_path.join(".gitignore");
    if !gitignore.exists() {
        std::fs::write(&gitignore, ".wistra/logs/\n")
            .context("Failed to write .gitignore")?;
    }

    println!("✅ Git repository initialized");
    Ok(())
}
