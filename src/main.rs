use anyhow::{Context, Result};
use askama::Template;
use axum::{
    Router,
    http::{StatusCode, header},
    response::{
        Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::get,
};
use clap::Parser;
use futures::{Stream, StreamExt};
use once_cell::sync::Lazy;
use rust_embed::Embed;
use std::convert::Infallible;
use std::fs;
use std::time::Duration;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use time::OffsetDateTime;
use tokio::net::TcpListener;
use tokio::sync::{RwLock, broadcast};
use tokio_stream::wrappers::BroadcastStream;
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to markdown file or directory containing README.md
    #[arg(default_value = ".")]
    path: PathBuf,

    /// If supplied, will export the markdown file to HTML in the specified directory
    #[arg(long, short)]
    export_dir: Option<PathBuf>,

    /// Whether to overwrite the export directory if it exists
    #[arg(long, short)]
    force: bool,

    /// Address to bind the server to
    #[arg(long, short, default_value = "localhost:3000")]
    address: String,

    /// Whether to open the browser on serve
    #[arg(long, short)]
    open: bool,
}

struct RenderedMarkdown {
    content: String,
    path: PathBuf,
}

impl RenderedMarkdown {
    fn new(path: &Path) -> Result<Self> {
        let markdown_content = fs::read_to_string(path).context("Failed to read markdown file")?;

        let rendered_markdown = render_markdown(
            &markdown_content,
            path.file_name().unwrap().to_str().unwrap(),
        )?;

        Ok(Self {
            content: rendered_markdown,
            path: path.to_path_buf(),
        })
    }

    fn rebuild(&mut self) -> Result<()> {
        let markdown_content =
            fs::read_to_string(&self.path).context("Failed to read markdown file")?;

        self.content = render_markdown(
            &markdown_content,
            self.path.file_name().unwrap().to_str().unwrap(),
        )?;

        Ok(())
    }
}

use axum::{
    extract::{Request, State},
    response::IntoResponse,
};
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

const SIMPLE_TIME_FORMAT: &[BorrowedFormatItem<'_>] =
    format_description!("[hour]:[minute]:[second]");

static RELOAD_TX: Lazy<broadcast::Sender<String>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(100);
    tx
});

async fn reload_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = BroadcastStream::new(RELOAD_TX.subscribe()).map(|_| {
        Result::<Event, Infallible>::Ok(
            Event::default()
                .retry(Duration::from_millis(250))
                .data("reload"),
        )
    });
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-ping"),
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let root_path = Arc::new(args.path);

    let markdown_file_path = if root_path.is_dir() {
        root_path.join("README.md")
    } else {
        root_path.to_path_buf()
    };

    if let Some(export_dir) = &args.export_dir {
        anyhow::ensure!(
            args.force || !export_dir.exists(),
            "Export directory already exists: {}, and --force was not supplied",
            export_dir.display()
        );

        let markdown_content = fs::read_to_string(&markdown_file_path)
            .context("Failed to read markdown file for export")?;

        let rendered_html = render_markdown(
            &markdown_content,
            markdown_file_path.file_name().unwrap().to_str().unwrap(), // FIXME: horrible unwrap chain..
        )?;

        fs::create_dir_all(export_dir).context("Failed to create export directory")?;

        fs::write(export_dir.join("index.html"), rendered_html).context("Failed to write HTML")?;

        for path in EmbeddedAssets::iter() {
            let path_ref = path.as_ref();
            fs::write(
                export_dir.join(path_ref),
                EmbeddedAssets::get(path_ref).unwrap().data,
            )?;
        }
        println!("Exported to {}", export_dir.display());
        return Ok(());
    }

    let state = Arc::new(RwLock::new(RenderedMarkdown::new(&markdown_file_path)?));

    use notify::EventKind::{Create, Modify, Remove};
    use notify_debouncer_full::{DebounceEventResult, new_debouncer};
    let mut debouncer = new_debouncer(Duration::from_millis(250), None, {
        let state = Arc::clone(&state);
        let rt = tokio::runtime::Handle::current();
        move |result: DebounceEventResult| {
            if let Ok(events) = &result {
                if events
                    .iter()
                    .any(|e| matches!(e.event.kind, Create(_) | Modify(_) | Remove(_)))
                {
                    let now = OffsetDateTime::now_local()
                        .unwrap_or(OffsetDateTime::now_utc())
                        .time()
                        .format(SIMPLE_TIME_FORMAT)
                        .unwrap_or("?".to_string());

                    println!("[{now}] Detected change, rebuilding...");

                    let state = Arc::clone(&state);

                    rt.spawn(async move {
                        match state.write().await.rebuild() {
                            Ok(_) => {
                                let _ = RELOAD_TX.send("reload".to_string());
                            }
                            Err(e) => {
                                eprintln!("[{now}] Error during rebuild: {e}");
                            }
                        }
                    });
                }
            }
        }
    })
    .context("Failed to set up file watcher!")?;

    debouncer
        .watch(root_path.as_path(), notify::RecursiveMode::Recursive)
        .with_context(|| format!("Failed to watch path: {}", root_path.display()))?;

    state.write().await.rebuild()?;

    let app = Router::new()
        .route("/", get(serve))
        .fallback_service(
            ServeDir::new(markdown_file_path.with_file_name("")).fallback(get(assets_handler)),
        )
        .with_state(state)
        .layer(axum::middleware::from_fn(append_livereload_script))
        .route("/~~~meread-reload", get(reload_handler));

    if args.open {
        let _ = open::that(format!("http://{}", &args.address));
    }

    println!("Serving {} on http://{}", root_path.display(), args.address);

    let listener = TcpListener::bind(&args.address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

static LIVERELOAD_SCRIPT_BYTES: &[u8] = br#"<script>
    new EventSource('/~~~meread-reload').onmessage = (e) => {
        if (e.data === 'reload') window.location.reload()
    };
</script>"#;

use axum::body;
use axum::middleware::Next;
async fn append_livereload_script(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    if response.status() != StatusCode::OK {
        return response;
    }

    let (mut parts, body) = response.into_parts();

    match parts.headers.get(hyper::header::CONTENT_TYPE) {
        Some(content_type) if content_type.to_str().unwrap_or("").contains("text/html") => {}
        _ => {
            // dont mess with non-html
            return Response::from_parts(parts, body);
        }
    }

    let body_bytes = body::to_bytes(body, usize::MAX).await.unwrap();

    let mut modified_body_bytes =
        Vec::with_capacity(body_bytes.len() + LIVERELOAD_SCRIPT_BYTES.len());
    modified_body_bytes.extend_from_slice(&body_bytes);
    modified_body_bytes.extend_from_slice(LIVERELOAD_SCRIPT_BYTES);

    parts.headers.remove(hyper::header::CONTENT_LENGTH);

    Response::from_parts(parts, body::Body::from(modified_body_bytes))
}

#[derive(Template)]
#[template(path = "template.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    contents: &'a str,
}

struct ComrakConfig {
    options: comrak::Options<'static>,
    plugins: comrak::Plugins<'static>,
}

use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};
static SYNTECT_ADAPTER: Lazy<SyntectAdapter> = Lazy::new(|| {
    use std::io::Cursor;
    use syntect::highlighting::ThemeSet;
    let mut theme_set = ThemeSet::new();

    theme_set.themes.insert(
        "light-default".to_string(),
        ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("light-default.tmTheme")))
            .unwrap(),
    );
    theme_set.themes.insert(
        "dark".to_string(),
        ThemeSet::load_from_reader(&mut Cursor::new(include_bytes!("dark.tmTheme"))).unwrap(),
    );

    SyntectAdapterBuilder::new()
        .theme_set(theme_set)
        .theme("light-default") // TODO: make this controllable
        .build()
});

static COMRAK_CONFIG: Lazy<ComrakConfig> = Lazy::new(|| {
    use comrak::{ExtensionOptions, Plugins, RenderOptions, RenderPlugins};

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
            ..Default::default()
        },
        ..Default::default()
    };

    let plugins = Plugins {
        render: RenderPlugins {
            codefence_syntax_highlighter: Some(&*SYNTECT_ADAPTER),
            ..Default::default()
        },
    };

    ComrakConfig { options, plugins }
});

fn render_markdown(markdown_content: &str, title: &str) -> Result<String, askama::Error> {
    let rendered_markdown = comrak::markdown_to_html_with_plugins(
        markdown_content,
        &COMRAK_CONFIG.options,
        &COMRAK_CONFIG.plugins,
    );

    HtmlTemplate {
        title,
        contents: &rendered_markdown,
    }
    .render()
}

use axum::response::Html;
async fn serve(
    State(rendered_markdown): State<Arc<RwLock<RenderedMarkdown>>>,
) -> impl IntoResponse {
    let content = rendered_markdown.read().await.content.clone();
    Html(content)
}

use axum::http::Uri;
async fn assets_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    EmbeddedAsset(path)
}

#[derive(Embed, Debug)]
#[folder = "assets/"]
struct EmbeddedAssets;

pub struct EmbeddedAsset<T>(pub T);

impl<T> IntoResponse for EmbeddedAsset<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match EmbeddedAssets::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
