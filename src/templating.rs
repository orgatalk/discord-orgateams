/*
 * Copyright 2022 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use lazy_static::lazy_static;
use std::io::Write;
use tera::{Context, Tera};

use crate::model::Role;

lazy_static! {
    static ref TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template("index.html", include_str!("templates/index.html"))
            .unwrap();
        tera.add_raw_template("index.txt", include_str!("templates/index.txt"))
            .unwrap();
        tera
    };
}

/// Render roles as HTML representation.
pub(crate) fn render_html(roles: Vec<Role>, writer: impl Write) -> Result<()> {
    render("index.html", roles, writer)?;
    Ok(())
}

/// Render roles as text representation.
pub(crate) fn render_text(roles: Vec<Role>, writer: impl Write) -> Result<()> {
    render("index.txt", roles, writer)?;
    Ok(())
}

fn render(template_name: &str, roles: Vec<Role>, writer: impl Write) -> Result<()> {
    let mut context = Context::new();
    context.insert("roles", &roles);

    TERA.render_to(template_name, &context, writer)?;

    Ok(())
}
