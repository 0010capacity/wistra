mod types;

pub use types::*;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const GLOBAL_CONFIG_DIR: &str = ".wistra";
const GLOBAL_CONFIG_FILE: &str = "config.toml";

/// Get the global config directory path (~/.wistra)
pub fn global_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(GLOBAL_CONFIG_DIR))
}

/// Get the global config file path (~/.wistra/config.toml)
pub fn global_config_path() -> Result<PathBuf> {
    Ok(global_config_dir()?.join(GLOBAL_CONFIG_FILE))
}

/// Ensure the global config directory exists
pub fn ensure_global_config_dir() -> Result<PathBuf> {
    let dir = global_config_dir()?;
    std::fs::create_dir_all(&dir).context("Failed to create global config directory")?;
    Ok(dir)
}
