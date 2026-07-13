use comrak::{options::Plugins, plugins};
use plugins::syntect::SyntectAdapterBuilder;
use std::io::Cursor;
use syntect::highlighting::ThemeSet;

pub struct ComrakConfig {
    pub options: comrak::Options<'static>,
    pub plugins: Plugins<'static>,
}

impl ComrakConfig {
    pub fn new(light: bool) -> color_eyre::Result<Self> {
        let mut theme_set = ThemeSet::new();

        theme_set.themes.insert(
            String::new(), // hack
            if light {
                ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!(
                    "../themes/light-default.tmTheme"
                )))?
            } else {
                ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!(
                    "../themes/dark.tmTheme"
                )))?
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
            SyntectAdapterBuilder::new()
                .syntax_set(two_face::syntax::extra_newlines())
                .theme_set(theme_set)
                .theme("")
                .build(),
        ));

        let plugins = Plugins::builder()
            .render(
                comrak::options::RenderPlugins::builder()
                    .codefence_syntax_highlighter(adapter)
                    .build(),
            )
            .build();

        Ok(ComrakConfig { options, plugins })
    }
}
