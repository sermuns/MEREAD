use color_eyre::eyre::{Context, OptionExt, ensure};
use std::{fs, path::Path};

use crate::{assets::EmbeddedAssets, render::render_markdown};

pub fn export(
    markdown_file_path: &Path,
    export_dir: &Path,
    force: bool,
    light_mode: bool,
) -> color_eyre::Result<()> {
    ensure!(
        force || !export_dir.exists(),
        "export directory {:?} already exists and --force was not supplied",
        export_dir
    );

    fs::create_dir_all(export_dir).context("failed to create export directory")?;

    let markdown_content = fs::read_to_string(markdown_file_path)
        .context("failed to read markdown file for export")?;

    let rendered_html = render_markdown(&markdown_content, markdown_file_path, light_mode)?;

    fs::write(export_dir.join("index.html"), rendered_html).context("failed to write HTML")?;

    for path in EmbeddedAssets::iter() {
        fs::write(
            export_dir.join(path.as_ref()),
            EmbeddedAssets::get(&path)
                .ok_or_eyre("failed getting asset")?
                .data,
        )?;
    }
    println!("Exported to {}", export_dir.display());
    Ok(())
}
