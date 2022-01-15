/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Args {
    /// Configuration filename
    #[clap(short = 'c', long = "config")]
    pub config_filename: PathBuf,

    /// Output filename [default: STDOUT]
    #[clap(short = 'o', long = "output")]
    pub output_filename: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Export roles and members as JSON
    Export,

    /// Render roles and members from JSON
    Render {
        /// Input filename
        #[clap(short = 'i', long = "input")]
        input_filename: PathBuf,

        /// Output format
        #[clap(short = 'f', long = "format", arg_enum, default_value = "html")]
        output_format: OutputFormat,
    },
}

#[derive(clap::ArgEnum, Clone, Debug)]
pub(crate) enum OutputFormat {
    Html,
    Text,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
