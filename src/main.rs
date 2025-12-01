use anyhow::{Context, Result};
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use clap::Parser;
use std::fs;
use std::time::Duration;
use std::{path::PathBuf, sync::Arc};
use time::OffsetDateTime;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

mod assets;
mod comrak_config;
mod reload;
mod render;

use assets::*;
use comrak_config::*;
use reload::*;
use render::*;

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

    /// Render page in light-mode style
    #[arg(long, short)]
    light: bool,
}

use axum::extract::State;
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

const SIMPLE_TIME_FORMAT: &[BorrowedFormatItem<'_>] =
    format_description!("[hour]:[minute]:[second]");

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let root_path = Arc::new(args.path);

    let markdown_file_path = if root_path.is_dir() {
        root_path.join("README.md")
    } else {
        root_path.to_path_buf()
    };

    init_comrak_config(args.light);

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
            args.light,
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

    let state = Arc::new(RwLock::new(RenderedMarkdown::new(
        &markdown_file_path,
        args.light,
    )?));

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
                        match state.write().await.rebuild(args.light) {
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

    state.write().await.rebuild(args.light)?;

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

async fn serve(
    State(rendered_markdown): State<Arc<RwLock<RenderedMarkdown>>>,
) -> impl IntoResponse {
    let content = rendered_markdown.read().await.content.clone();
    Html(content)
}
