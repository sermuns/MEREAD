use askama::Template;
use axum::body::Bytes;
use color_eyre::{Result, eyre::Context, eyre::OptionExt};
use std::fs;
use std::path::{Path, PathBuf};

pub struct RenderedMarkdown {
    pub content: Bytes,
    pub path: PathBuf,
}

impl RenderedMarkdown {
    pub fn new(path: &Path, light: bool) -> Result<Self> {
        let markdown_content = fs::read_to_string(path).context("Failed to read markdown file")?;

        let rendered_markdown = render_markdown(&markdown_content, path, light)?;

        Ok(Self {
            content: rendered_markdown,
            path: path.to_path_buf(),
        })
    }

    pub fn rebuild(&mut self, light: bool) -> Result<()> {
        let markdown_content =
            fs::read_to_string(&self.path).context("Failed to read markdown file")?;

        self.content = render_markdown(&markdown_content, &self.path, light)?;

        Ok(())
    }
}

#[derive(Template)]
#[template(path = "template.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    contents: &'a str,
    light: bool,
}

pub fn render_markdown(
    markdown_content: &str,
    markdown_file_path: &Path,
    light: bool,
) -> Result<Bytes> {
    use crate::comrak_config::COMRAK_CONFIG;
    let title = &markdown_file_path
        .file_name()
        .ok_or_eyre("weird path ending in ...")?
        .to_string_lossy();

    let comrak_config = COMRAK_CONFIG.get().ok_or_eyre("failed getting config")?;

    let rendered_markdown = comrak::markdown_to_html_with_plugins(
        markdown_content,
        &comrak_config.options,
        &comrak_config.plugins,
    );

    let rendered = HtmlTemplate {
        title,
        contents: &rendered_markdown,
        light,
    }
    .render()?;

    Ok(rendered.into())
}
