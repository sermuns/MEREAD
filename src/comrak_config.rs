use color_eyre::eyre::{Result, eyre};
use comrak::{self, options::Plugins, plugins};
use plugins::syntect::SyntectAdapterBuilder;
use std::{io::Cursor, sync::OnceLock};
use syntect::highlighting::ThemeSet;

pub struct ComrakConfig {
    pub options: comrak::Options<'static>,
    pub plugins: Plugins<'static>,
}

pub(crate) static COMRAK_CONFIG: OnceLock<ComrakConfig> = OnceLock::new();

pub fn init_comrak_config(light: bool) -> Result<()> {
    let mut theme_set = ThemeSet::new();

    theme_set.themes.insert(
        "InspiredGitHub".to_string(),
        if light {
            ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!(
                "../themes/light-default.tmTheme"
            )))?
        } else {
            ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("../themes/dark.tmTheme")))?
        },
    );

    let options = comrak::Options {
        render: comrak::options::Render {
            r#unsafe: true,
            ..Default::default()
        },
        extension: comrak::options::Extension {
            alerts: true,
            autolink: true,
            footnotes: true,
            header_id_prefix: Some(String::new()),
            math_code: true,
            math_dollars: true,
            shortcodes: true,
            strikethrough: true,
            table: true,
            tagfilter: true,
            tasklist: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let adapter = Box::leak(Box::new(
        SyntectAdapterBuilder::new().theme_set(theme_set).build(),
    ));
    let plugins = Plugins::builder()
        .render(
            comrak::options::RenderPlugins::builder()
                .codefence_syntax_highlighter(adapter)
                .build(),
        )
        .build();

    COMRAK_CONFIG
        .set(ComrakConfig { options, plugins })
        .map_err(|_| eyre!("COMRAK_CONFIG already initialized"))?;

    Ok(())
}
