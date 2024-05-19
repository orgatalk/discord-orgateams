/*
 * Copyright 2022-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use chrono::Utc;
use std::collections::HashSet;
use std::io::Write;
mod assembly;
mod cli;
mod config;
mod discord_api;
mod io;
mod model;
mod templating;

use crate::config::{Config, DiscordConfig};
use crate::model::Data;

fn main() -> Result<()> {
    let args = cli::parse_args();

    let config = config::load_config(&args.config_filename)?;
    let writer: Box<dyn Write> = io::get_writer(args.output_filename)?;

    let roles = match args.roles_input_filename {
        Some(filename) => model::read_roles(filename)?,
        None => fetch_roles_from_discord(&config.discord)?,
    };

    let data = assemble_data(config, roles);

    match args.output_format {
        cli::OutputFormat::Html => templating::render_html(data, writer)?,
        cli::OutputFormat::Json => model::write_roles(data.roles, writer)?,
        cli::OutputFormat::Text => templating::render_text(data, writer)?,
    };

    Ok(())
}

fn fetch_roles_from_discord(config: &DiscordConfig) -> Result<Vec<model::Role>> {
    let api_client = discord_api::Client::new(&config.bot_token);
    let guild_members = api_client.get_guild_members(&config.guild_id)?;
    let guild_roles = api_client.get_guild_roles(&config.guild_id)?;

    let roles = assembly::assemble_roles(&guild_members, &guild_roles, &config.roles_excluded);
    Ok(roles)
}

fn assemble_data(config: Config, roles: Vec<model::Role>) -> Data {
    Data {
        title: config.title,
        subtitle: config.subtitle,
        roles: roles.clone(),
        user_count: count_users(roles),
        generated_at: Utc::now(),
    }
}

fn count_users(roles: Vec<model::Role>) -> u16 {
    let mut names = HashSet::new();
    for role in &roles {
        for name in &role.member_names {
            names.insert(name);
        }
    }
    names.len() as u16
}
