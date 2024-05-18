/*
 * Copyright 2022-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub discord: DiscordConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DiscordConfig {
    pub bot_token: String,
    pub guild_id: String,
    pub roles_excluded: HashSet<String>,
}

/// Load configuration from TOML file.
pub(crate) fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}
