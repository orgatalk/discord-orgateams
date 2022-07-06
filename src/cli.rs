/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Args {
    /// Configuration filename
    #[clap(short = 'c', long = "config")]
    pub config_filename: PathBuf,

    /// Roles input filename (to load roles from JSON instead of
    /// fetching them from Discord)
    #[clap(long = "roles-input")]
    pub roles_input_filename: Option<PathBuf>,

    /// Output filename [default: STDOUT]
    #[clap(short = 'o', long = "output")]
    pub output_filename: Option<PathBuf>,

    /// Output format
    #[clap(short = 'f', long = "format", arg_enum, default_value = "html")]
    pub output_format: OutputFormat,
}

#[derive(clap::ArgEnum, Clone, Debug)]
pub(crate) enum OutputFormat {
    Html,
    Json,
    Text,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
