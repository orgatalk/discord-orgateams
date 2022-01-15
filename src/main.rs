/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
mod assembly;
mod cli;
mod config;
mod discord_api;
mod io;
mod model;
mod templating;

fn main() -> Result<()> {
    let args = cli::parse_args();

    let writer: Box<dyn Write> = io::get_writer(args.output_filename)?;

    match args.command {
        cli::Command::Export => {
            let config = config::load_config(&args.config_filename)?;
            export_json(&config.discord, writer)?;
        }
        cli::Command::Render {
            input_filename,
            output_format,
        } => render_roles(input_filename, output_format, writer)?,
    }

    Ok(())
}

fn export_json(config: &config::DiscordConfig, writer: impl Write) -> Result<()> {
    let api_client = discord_api::Client::new(&config.bot_token);
    let guild_members = api_client.get_guild_members(&config.guild_id)?;
    let guild_roles = api_client.get_guild_roles(&config.guild_id)?;

    let roles = assembly::assemble_roles(&guild_members, &guild_roles, &config.roles_excluded);

    model::write_roles(roles, writer)?;

    Ok(())
}

fn render_roles(
    input_filename: PathBuf,
    output_format: cli::OutputFormat,
    writer: impl Write,
) -> Result<()> {
    let roles = model::read_roles(input_filename)?;

    match output_format {
        cli::OutputFormat::Html => templating::render_html(roles, writer)?,
        cli::OutputFormat::Text => templating::render_text(roles, writer)?,
    }

    Ok(())
}
