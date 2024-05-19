/*
 * Copyright 2022-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::io::Write;
use std::sync::OnceLock;
use tera::{Context, Tera};

use crate::model::Role;

fn get_tera() -> &'static Tera {
    static TERA: OnceLock<Tera> = OnceLock::new();
    TERA.get_or_init(|| {
        let mut tera = Tera::default();
        tera.add_raw_template("index.html", include_str!("templates/index.html"))
            .unwrap();
        tera.add_raw_template("index.txt", include_str!("templates/index.txt"))
            .unwrap();
        tera
    })
}

/// Render roles as HTML representation.
pub(crate) fn render_html(
    title: String,
    subtitle: Option<String>,
    roles: Vec<Role>,
    generated_at: DateTime<Utc>,
    writer: impl Write,
) -> Result<()> {
    let mut context = Context::new();
    context.insert("roles", &roles);
    context.insert("title", &title);
    context.insert("subtitle", &subtitle);
    context.insert("generated_at", &generated_at);

    render("index.html", &context, writer)?;

    Ok(())
}

/// Render roles as text representation.
pub(crate) fn render_text(roles: Vec<Role>, writer: impl Write) -> Result<()> {
    let mut context = Context::new();
    context.insert("roles", &roles);

    render("index.txt", &context, writer)?;

    Ok(())
}

fn render(template_name: &str, context: &Context, writer: impl Write) -> Result<()> {
    get_tera().render_to(template_name, &context, writer)?;

    Ok(())
}
