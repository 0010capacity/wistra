use crate::config::{GlobalConfig, WikiConfig, INTEREST_DOMAINS, LANGUAGES, ensure_global_config_dir};
use anyhow::{Context, Result};
use dialoguer::{Input, Select, MultiSelect, Password};
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

    // Step 3: Adapter selection (only Claude for now)
    let adapter_idx = Select::new()
        .with_prompt("Adapter")
        .items(&["Claude API"])
        .default(0)
        .interact()
        .context("Failed to select adapter")?;

    let _adapter = match adapter_idx {
        0 => "claude",
        _ => "claude",
    };

    // Step 4: Claude API key
    let api_key = Password::new()
        .with_prompt("Claude API key")
        .interact()
        .context("Failed to read API key")?;

    // Step 5: Daily concept count
    let daily_count: usize = Input::new()
        .with_prompt("Daily concept count")
        .default(5)
        .interact_text()
        .context("Failed to read daily count")?;

    // Step 6: Interest domains
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

    // Step 7: Cron job setup
    let setup_cron = Select::new()
        .with_prompt("Set up daily cron job?")
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .context("Failed to select cron option")? == 0;

    // Build config
    let config = GlobalConfig {
        wiki_path: Some(wiki_path.clone()),
        language,
        claude_api_key: api_key,
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

    // Print cron line if requested
    if setup_cron {
        println!("\n📝 Add this line to your crontab (crontab -e):");
        println!("    0 9 * * * wistra run --quiet --no-confirm");
    }

    println!("\n🎉 Setup complete! Run `wistra run` to start growing your wiki.");

    Ok(())
}
