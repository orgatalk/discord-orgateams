/*
 * Copyright 2022-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

use crate::io;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Data {
    pub title: String,
    pub subtitle: Option<String>,
    pub roles: Vec<Role>,
    pub user_count: u16,
    pub generated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct Role {
    pub name: String,
    pub member_names: Vec<String>,
    pub color_hex: String,
}

/// Read roles from JSON file.
pub(crate) fn read_roles(path: PathBuf) -> Result<Vec<Role>> {
    let reader = io::get_reader(path)?;
    let roles = serde_json::from_reader(reader)?;
    Ok(roles)
}

/// Write roles to JSON file.
pub(crate) fn write_roles(roles: Vec<Role>, writer: impl Write) -> Result<()> {
    serde_json::to_writer(writer, &roles)?;
    Ok(())
}
