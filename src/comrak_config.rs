use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{self, options::Plugins, plugins};
use once_cell::sync::OnceCell;
use plugins::syntect::SyntectAdapter;
use plugins::syntect::SyntectAdapterBuilder;
use std::io::Cursor;
use syntect::highlighting::ThemeSet;

pub struct ComrakConfig {
    pub options: comrak::Options<'static>,
    pub plugins: Plugins<'static>,
}

pub(crate) static COMRAK_CONFIG: OnceCell<ComrakConfig> = OnceCell::new();
static SYNTECT_ADAPTER: OnceCell<SyntectAdapter> = OnceCell::new();

pub fn init_comrak_config(light: bool) -> Result<()> {
    let mut theme_set = ThemeSet::new();

    // Funny code.. Maybe this is too cursed..
    theme_set.themes.insert(
        true.to_string(),
        ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("light-default.tmTheme")))?,
    );
    theme_set.themes.insert(
        false.to_string(),
        ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("dark.tmTheme")))?,
    );

    SYNTECT_ADAPTER
        .set(
            SyntectAdapterBuilder::new()
                .theme_set(theme_set)
                .theme(&light.to_string())
                .build(),
        )
        .map_err(|_| eyre!("SYNTECT_ADAPTER already initialized"))?;

    let options = comrak::Options {
        render: comrak::options::Render {
            r#unsafe: true,
            ..Default::default()
        },
        extension: comrak::options::Extension {
            header_ids: Some("".to_string()),
            table: true,
            strikethrough: true,
            autolink: true,
            tagfilter: true,
            footnotes: true,
            shortcodes: true,
            math_code: true,
            math_dollars: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let plugins = Plugins {
        render: comrak::options::RenderPlugins {
            codefence_syntax_highlighter: SYNTECT_ADAPTER
                .get()
                .map(|a| a as &dyn SyntaxHighlighterAdapter),
            ..Default::default()
        },
    };

    COMRAK_CONFIG
        .set(ComrakConfig { options, plugins })
        .map_err(|_| eyre!("COMRAK_CONFIG already initialized"))?;

    Ok(())
}

