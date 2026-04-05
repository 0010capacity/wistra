use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Global configuration stored in ~/.wistra/config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Default wiki path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_path: Option<PathBuf>,
    /// User's preferred language (ISO 639-1)
    pub language: String,
    /// Daily concept generation count
    #[serde(default = "default_daily_count")]
    pub daily_count: usize,
    /// Interest domains
    pub interests: Vec<String>,
}

fn default_daily_count() -> usize {
    5
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            wiki_path: None,
            language: "en".to_string(),
            daily_count: 5,
            interests: vec![],
        }
    }
}

impl GlobalConfig {
    /// Load from ~/.wistra/config.toml
    pub fn load() -> Result<Option<Self>> {
        let path = super::global_config_path()?;
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)
            .context("Failed to read global config file")?;

        let config: GlobalConfig = toml::from_str(&content)
            .context("Failed to parse global config")?;

        Ok(Some(config))
    }

    /// Save to ~/.wistra/config.toml
    pub fn save(&self) -> Result<()> {
        super::ensure_global_config_dir()?;

        let path = super::global_config_path()?;
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&path, content)
            .context("Failed to write config file")?;

        Ok(())
    }
}

/// Wiki-local configuration stored in <wiki>/.wistra/config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiConfig {
    /// Wiki root path (concepts/, meta/ directories are here)
    #[serde(skip)]
    pub root_path: PathBuf,
    /// Wiki display name (e.g., "My Knowledge Base")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Short description of the wiki
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl WikiConfig {
    /// Get wiki display name, falling back to "Wistra"
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Wistra")
    }

    /// Load from <wiki>/.wistra/config.toml
    pub fn load(wiki_path: &PathBuf) -> Result<Self> {
        let config_path = wiki_path.join(".wistra").join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .context("Failed to read wiki config file")?;

            let mut config: WikiConfig = toml::from_str(&content)
                .context("Failed to parse wiki config")?;
            config.root_path = wiki_path.clone();
            return Ok(config);
        }

        Ok(WikiConfig {
            root_path: wiki_path.clone(),
            name: None,
            description: None,
        })
    }

    /// Save to <wiki>/.wistra/config.toml
    pub fn save(&self) -> Result<()> {
        let config_path = self.wistra_dir().join("config.toml");
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create .wistra directory")?;
        }
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize wiki config")?;
        std::fs::write(&config_path, content)
            .context("Failed to write wiki config file")?;
        Ok(())
    }

    /// Get concepts directory path
    pub fn concepts_dir(&self) -> PathBuf {
        self.root_path.join("concepts")
    }

    /// Get meta directory path
    pub fn meta_dir(&self) -> PathBuf {
        self.root_path.join("meta")
    }

    /// Get .wistra directory path
    pub fn wistra_dir(&self) -> PathBuf {
        self.root_path.join(".wistra")
    }

    /// Get state.json path
    pub fn state_path(&self) -> PathBuf {
        self.wistra_dir().join("state.json")
    }

    /// Ensure wiki directory structure exists
    pub fn ensure_structure(&self) -> Result<()> {
        std::fs::create_dir_all(self.concepts_dir())
            .context("Failed to create concepts directory")?;
        std::fs::create_dir_all(self.meta_dir())
            .context("Failed to create meta directory")?;
        std::fs::create_dir_all(self.wistra_dir())
            .context("Failed to create .wistra directory")?;
        std::fs::create_dir_all(self.wistra_dir().join("logs"))
            .context("Failed to create logs directory")?;
        Ok(())
    }
}

/// Available interest domains
pub const INTEREST_DOMAINS: &[(&str, &str)] = &[
    // STEM
    ("science", "Science"),
    ("mathematics", "Mathematics"),
    ("computer-science", "Computer Science"),
    ("programming", "Programming"),
    ("technology", "Technology"),
    // Humanities & Social Sciences
    ("history", "History"),
    ("philosophy", "Philosophy"),
    ("psychology", "Psychology"),
    ("economics", "Economics"),
    ("politics", "Politics"),
    ("law", "Law"),
    ("geography", "Geography"),
    ("language-linguistics", "Language & Linguistics"),
    // Culture & Arts
    ("culture", "Culture"),
    ("subculture", "Subculture"),
    ("current-affairs", "Current Affairs"),
    ("arts", "Arts"),
    ("design", "Design"),
    // Applied & Professional
    ("business-management", "Business & Management"),
    ("education", "Education"),
    ("health-medicine", "Health & Medicine"),
    ("environment", "Environment"),
    // Personal & Recreation
    ("self-improvement", "Self-improvement"),
    ("sports-recreation", "Sports & Recreation"),
    ("religion-mythology", "Religion & Mythology"),
];

/// Available languages
pub const LANGUAGES: &[(&str, &str)] = &[
    ("ko", "한국어"),
    ("en", "English"),
];
