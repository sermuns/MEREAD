use askama::Template;
use axum::body::Bytes;
use color_eyre::{Result, eyre::Context, eyre::OptionExt};
use comrak::create_formatter;
use comrak::html::ChildRendering;
use comrak::nodes::NodeValue;
use comrak::{Arena, parse_document};
use math_core::LatexToMathML;
use math_core::MathCoreConfig;
use math_core::MathDisplay;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

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

static MATHML_CONVERTER: LazyLock<LatexToMathML> = LazyLock::new(|| {
    let config = MathCoreConfig::default();
    LatexToMathML::new(config).unwrap()
});

create_formatter!(CustomFormatter, {
    NodeValue::Strong => |context, entering| {
        context.write_str(if entering { "<b>" } else { "</b>" })?;
    },
    NodeValue::Math(ref node_math) => |context, entering| {
        if !entering {
            return Ok(ChildRendering::Skip);
        }
        let display_mode = if node_math.display_math { MathDisplay::Block } else { MathDisplay::Inline };
        let mathml = MATHML_CONVERTER.convert_with_local_counter(&node_math.literal, display_mode).unwrap();
        context.write_str(&mathml)?;
    }
});

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

    let arena = Arena::new();
    let doc = parse_document(&arena, markdown_content, &comrak_config.options);

    let mut formatted = String::new();
    CustomFormatter::format_document_with_plugins(
        doc,
        &comrak_config.options,
        &mut formatted,
        &comrak_config.plugins,
    )?;

    let rendered = HtmlTemplate {
        title,
        contents: &formatted,
        light,
    }
    .render()?;

    Ok(rendered.into())
}
