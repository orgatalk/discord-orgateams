/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::PathBuf;

/// Obtain a reader to read the file from.
pub(crate) fn get_reader(path: PathBuf) -> Result<Box<dyn Read>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(Box::new(reader))
}

/// Obtain a writer to write the output to.
pub(crate) fn get_writer(filename: Option<PathBuf>) -> Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = if filename.is_some() {
        Box::new(File::create(filename.unwrap())?)
    } else {
        Box::new(io::stdout())
    };
    Ok(writer)
}
