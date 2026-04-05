use crate::cli::cron::install_cron;
use crate::config::{GlobalConfig, WikiConfig, INTEREST_DOMAINS, LANGUAGES, ensure_global_config_dir};
use anyhow::{Context, Result};
use dialoguer::{Input, Select, MultiSelect};
use std::path::PathBuf;

/// Run the onboarding wizard
pub fn run_onboard() -> Result<()> {
    println!("🚀 Welcome to wistra setup!\n");

    // Step 1: Wiki path
    let default_path = dirs::home_dir()
        .map(|h| h.join("wiki"))
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "~/wiki".to_string());

    let wiki_path: String = Input::new()
        .with_prompt("Wiki path")
        .default(default_path)
        .interact_text()
        .context("Failed to read wiki path")?;

    let wiki_path = shellexpand::tilde(&wiki_path).to_string();
    let wiki_path = PathBuf::from(wiki_path);

    // Step 2: Language
    let language_idx = Select::new()
        .with_prompt("Language")
        .items(&LANGUAGES.iter().map(|(_, name)| *name).collect::<Vec<_>>())
        .default(0)
        .interact()
        .context("Failed to select language")?;

    let language = LANGUAGES[language_idx].0.to_string();

    // Step 3: Daily concept count
    let daily_count: usize = Input::new()
        .with_prompt("Daily concept count")
        .default(5)
        .interact_text()
        .context("Failed to read daily count")?;

    // Step 4: Interest domains
    let interest_items: Vec<&str> = INTEREST_DOMAINS.iter().map(|(_, name)| *name).collect();
    let selected_indices = MultiSelect::new()
        .with_prompt("Interests (space to select)")
        .items(&interest_items)
        .interact()
        .context("Failed to select interests")?;

    let interests: Vec<String> = selected_indices
        .iter()
        .map(|&idx| INTEREST_DOMAINS[idx].0.to_string())
        .collect();

    // Step 5: Cron job setup
    let setup_cron = Select::new()
        .with_prompt("Set up daily cron job?")
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .context("Failed to select cron option")? == 0;

    // Step 6: Cron time (if cron enabled)
    let (cron_hour, cron_minute) = if setup_cron {
        let hour: u8 = Input::new()
            .with_prompt("Hour (0-23)")
            .default(9)
            .interact_text()
            .context("Failed to read cron hour")?;

        let minute: u8 = Input::new()
            .with_prompt("Minute (0-59)")
            .default(0)
            .interact_text()
            .context("Failed to read cron minute")?;

        (hour, minute)
    } else {
        (9, 0)
    };

    // Build config
    let config = GlobalConfig {
        wiki_path: Some(wiki_path.clone()),
        language,
        daily_count,
        interests,
    };

    // Save global config
    ensure_global_config_dir()?;
    config.save()?;

    println!("\n✅ Config saved → ~/.wistra/config.toml");

    // Initialize wiki structure
    let wiki_config = WikiConfig::load(&wiki_path)?;
    wiki_config.ensure_structure()?;

    println!("✅ Directory structure initialized");

    // Install cron job if requested
    if setup_cron {
        println!("\n⏰ Setting up cron job...");
        install_cron(cron_hour, cron_minute, false, false)?;
        println!("✅ Cron job installed!");
    }

    println!("\n🎉 Setup complete! Run `wistra run` to start growing your wiki.");

    Ok(())
}
