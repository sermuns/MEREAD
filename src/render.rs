use anyhow::{Context, Result};
use askama::Template;
use std::fs;
use std::path::{Path, PathBuf};

use crate::comrak_config::COMRAK_CONFIG;

pub struct RenderedMarkdown {
    pub content: String,
    pub path: PathBuf,
}

impl RenderedMarkdown {
    pub fn new(path: &Path, light: bool) -> Result<Self> {
        let markdown_content = fs::read_to_string(path).context("Failed to read markdown file")?;

        let rendered_markdown = render_markdown(
            &markdown_content,
            path.file_name().unwrap().to_str().unwrap(),
            light,
        )?;

        Ok(Self {
            content: rendered_markdown,
            path: path.to_path_buf(),
        })
    }

    pub fn rebuild(&mut self, light: bool) -> Result<()> {
        let markdown_content =
            fs::read_to_string(&self.path).context("Failed to read markdown file")?;

        self.content = render_markdown(
            &markdown_content,
            self.path.file_name().unwrap().to_str().unwrap(),
            light,
        )?;

        Ok(())
    }
}

#[derive(Template)]
#[template(path = "template.html")]
pub struct HtmlTemplate<'a> {
    title: &'a str,
    contents: &'a str,
    light: bool,
}

pub fn render_markdown(
    markdown_content: &str,
    title: &str,
    light: bool,
) -> Result<String, askama::Error> {
    let comrak_config = COMRAK_CONFIG.get().unwrap();

    let rendered_markdown = comrak::markdown_to_html_with_plugins(
        markdown_content,
        &comrak_config.options,
        &comrak_config.plugins,
    );

    HtmlTemplate {
        title,
        contents: &rendered_markdown,
        light,
    }
    .render()
}
