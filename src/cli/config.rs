use crate::config::{GlobalConfig, INTEREST_DOMAINS, LANGUAGES};
use anyhow::{Context, Result};
use dialoguer::{Input, Select, MultiSelect, Confirm};

/// Run the config modification wizard
pub fn run_config(onboard: bool) -> Result<()> {
    if onboard {
        return super::onboard::run_onboard();
    }

    // Load existing config
    let mut config = GlobalConfig::load()?
        .context("No config found. Run `wistra onboard` first.")?;

    println!("⚙️  Current configuration:\n");

    loop {
        let options = vec![
            "Wiki path",
            "Language",
            "Daily concept count",
            "Interest domains",
            "Save and exit",
            "Exit without saving",
        ];

        let selection = Select::new()
            .with_prompt("What would you like to change?")
            .items(&options)
            .default(0)
            .interact()
            .context("Failed to select option")?;

        match selection {
            0 => {
                // Wiki path
                let current = config.wiki_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "~/wiki".to_string());

                let wiki_path: String = Input::new()
                    .with_prompt("Wiki path")
                    .default(current)
                    .interact_text()
                    .context("Failed to read wiki path")?;

                let wiki_path = shellexpand::tilde(&wiki_path).to_string();
                config.wiki_path = Some(std::path::PathBuf::from(wiki_path));
            }
            1 => {
                // Language
                let current_idx = LANGUAGES
                    .iter()
                    .position(|(code, _)| *code == config.language)
                    .unwrap_or(0);

                let language_idx = Select::new()
                    .with_prompt("Language")
                    .items(&LANGUAGES.iter().map(|(_, name)| *name).collect::<Vec<_>>())
                    .default(current_idx)
                    .interact()
                    .context("Failed to select language")?;

                config.language = LANGUAGES[language_idx].0.to_string();
            }
            2 => {
                // Daily count
                let daily_count: usize = Input::new()
                    .with_prompt("Daily concept count")
                    .default(config.daily_count)
                    .interact_text()
                    .context("Failed to read daily count")?;

                config.daily_count = daily_count;
            }
            3 => {
                // Interests
                let defaults: Vec<bool> = (0..INTEREST_DOMAINS.len())
                    .map(|idx| config.interests.contains(&INTEREST_DOMAINS[idx].0.to_string()))
                    .collect();

                let interest_items: Vec<&str> = INTEREST_DOMAINS.iter().map(|(_, name)| *name).collect();
                let selected_indices = MultiSelect::new()
                    .with_prompt("Interests (space to select)")
                    .items(&interest_items)
                    .defaults(&defaults)
                    .interact()
                    .context("Failed to select interests")?;

                config.interests = selected_indices
                    .iter()
                    .map(|&idx| INTEREST_DOMAINS[idx].0.to_string())
                    .collect();
            }
            4 => {
                // Save and exit
                config.save()?;
                println!("\n✅ Configuration saved!");
                break;
            }
            5 => {
                // Exit without saving
                if Confirm::new()
                    .with_prompt("Discard changes?")
                    .default(false)
                    .interact()?
                {
                    println!("Exited without saving.");
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
