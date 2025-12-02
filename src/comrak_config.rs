use comrak::{self, Plugins, plugins};
use once_cell::sync::OnceCell;
use plugins::syntect::SyntectAdapter;

pub struct ComrakConfig {
    pub options: comrak::Options<'static>,
    pub plugins: Plugins<'static>,
}

pub(crate) static COMRAK_CONFIG: OnceCell<ComrakConfig> = OnceCell::new();
static SYNTECT_ADAPTER: OnceCell<SyntectAdapter> = OnceCell::new();

pub fn init_comrak_config(light: bool) {
    use comrak::{ExtensionOptions, RenderOptions, RenderPlugins};
    use plugins::syntect::SyntectAdapterBuilder;

    use std::io::Cursor;
    use syntect::highlighting::ThemeSet;
    let mut theme_set = ThemeSet::new();

    // Funny code.. Maybe this is too cursed..
    theme_set.themes.insert(
        true.to_string(),
        ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("light-default.tmTheme")))
            .unwrap(),
    );
    theme_set.themes.insert(
        false.to_string(),
        ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("dark.tmTheme"))).unwrap(),
    );
    // FIXME: HANDLE THE FUCKING ERRORS
    let _ = SYNTECT_ADAPTER.set(
        SyntectAdapterBuilder::new()
            .theme_set(theme_set)
            .theme(&light.to_string())
            .build(),
    );

    let options = comrak::Options {
        render: RenderOptions {
            unsafe_: true,
            ..Default::default()
        },
        extension: ExtensionOptions {
            header_ids: Some("".to_string()),
            table: true,
            strikethrough: true,
            autolink: true,
            tagfilter: true,
            footnotes: true,
            shortcodes: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let plugins = Plugins {
        render: RenderPlugins {
            codefence_syntax_highlighter: Some(SYNTECT_ADAPTER.get().unwrap()),
            ..Default::default()
        },
    };

    // FIXME: HANDLE THE FUCKING ERRORS
    let _ = COMRAK_CONFIG.set(ComrakConfig { options, plugins });
}
