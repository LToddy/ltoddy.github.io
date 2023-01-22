use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use anyhow::Context;
use chrono::prelude::*;

use crate::cli::NewArgs;

pub fn new_post(args: NewArgs) -> anyhow::Result<()> {
    let current_dir = current_dir().with_context(|| "unable to get current directory")?;
    let NewArgs { title } = args;

    let date = Local::now().format("%Y-%m-%d");
    let posts_dir = current_dir.join("posts");

    let mut filename =
        posts_dir.join(format!("{}-{}", date, title.replace(' ', "-").to_lowercase()));
    filename.set_extension("md");

    if filename.exists() {
        log::info!("{} already exists", filename.display());
        return Ok(());
    }

    log::info!("Creating new post: {}", filename.display());
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(filename)
        .with_context(|| "unable to create a new post")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(format!("# {}", title).as_bytes())?;

    Ok(())
}
