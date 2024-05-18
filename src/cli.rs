/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(about, author, version)]
pub(crate) struct Args {
    /// Configuration filename
    #[arg(short = 'c', long = "config")]
    pub config_filename: PathBuf,

    /// Roles input filename (to load roles from JSON instead of
    /// fetching them from Discord)
    #[arg(long = "roles-input")]
    pub roles_input_filename: Option<PathBuf>,

    /// Output filename [default: STDOUT]
    #[arg(short = 'o', long = "output")]
    pub output_filename: Option<PathBuf>,

    /// Output format
    #[arg(short = 'f', long = "format", value_enum, default_value = "html")]
    pub output_format: OutputFormat,
}

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum OutputFormat {
    Html,
    Json,
    Text,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
