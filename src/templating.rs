/*
 * Copyright 2022-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use std::io::Write;
use std::sync::OnceLock;
use tera::{Context, Tera};

use crate::model::Data;

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
pub(crate) fn render_html(data: Data, writer: impl Write) -> Result<()> {
    let mut context = Context::new();
    context.insert("title", &data.title);
    context.insert("subtitle", &data.subtitle);
    context.insert("roles", &data.roles);
    context.insert("user_count", &data.user_count);
    context.insert("generated_at", &data.generated_at);

    render("index.html", &context, writer)?;

    Ok(())
}

/// Render roles as text representation.
pub(crate) fn render_text(data: Data, writer: impl Write) -> Result<()> {
    let mut context = Context::new();
    context.insert("roles", &data.roles);

    render("index.txt", &context, writer)?;

    Ok(())
}

fn render(template_name: &str, context: &Context, writer: impl Write) -> Result<()> {
    get_tera().render_to(template_name, context, writer)?;

    Ok(())
}
