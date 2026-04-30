use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use clap::Parser;
use color_eyre::{
    Result,
    eyre::{Context, ContextCompat},
};
use notify::EventKind::{Create, Modify, Remove};
use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use std::{path::PathBuf, sync::Arc, time::Duration};
use time::{OffsetDateTime, format_description::BorrowedFormatItem, macros::format_description};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::services::ServeDir;

mod assets;
mod comrak_config;
mod export;
mod reload;
mod render;

use crate::{
    assets::assets_handler,
    comrak_config::init_comrak_config,
    export::export,
    reload::{RELOAD_TX, append_livereload_script, reload_handler},
    render::RenderedMarkdown,
};

const SIMPLE_TIME_FORMAT: &[BorrowedFormatItem<'_>] =
    format_description!("[hour]:[minute]:[second]");

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
    light_mode: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .display_location_section(cfg!(debug_assertions))
        .install()?;

    let args = Args::parse();

    let markdown_file_path = if args.path.is_dir() {
        args.path.join("README.md")
    } else {
        args.path
    };

    init_comrak_config(args.light_mode)?;

    if let Some(export_dir) = &args.export_dir {
        export(&markdown_file_path, export_dir, args.force, args.light_mode)?;
        return Ok(());
    }

    let state = Arc::new(RwLock::new(RenderedMarkdown::new(
        &markdown_file_path,
        args.light_mode,
    )?));

    let mut debouncer = new_debouncer(Duration::from_millis(250), None, {
        let rt = tokio::runtime::Handle::current();
        let state = state.clone();
        move |result: DebounceEventResult| {
            if let Ok(events) = result
                && events
                    .iter()
                    .any(|e| matches!(e.kind, Create(_) | Modify(_) | Remove(_)))
            {
                let now = OffsetDateTime::now_local()
                    .unwrap_or(OffsetDateTime::now_utc())
                    .time()
                    .format(SIMPLE_TIME_FORMAT)
                    .unwrap_or("?".to_string());

                println!("[{now}] detected change, rebuilding...");

                let state = state.clone();
                rt.spawn(async move {
                    if let Err(e) = state.write().await.rebuild(args.light_mode) {
                        eprintln!("[{now}] error during rebuild: {e}");
                    } else {
                        let _ = RELOAD_TX.send("reload".to_string());
                    }
                });
            }
        }
    })
    .context("failed to set up file watcher")?;

    let parent_dir = markdown_file_path
        .parent()
        .context("trying to watch file in root / or something??")?;

    debouncer
        .watch(parent_dir, notify::RecursiveMode::Recursive)
        .with_context(|| format!("failed to watch path: {}", markdown_file_path.display()))?;

    state.write().await.rebuild(args.light_mode)?;

    let app = Router::new()
        .route("/", get(serve))
        .fallback_service(ServeDir::new(parent_dir).fallback(get(assets_handler)))
        .with_state(state)
        .layer(axum::middleware::from_fn(append_livereload_script))
        .route("/~~~meread-reload", get(reload_handler));

    if args.open {
        open::that(format!("http://{}", &args.address)).ok();
    }

    println!(
        "serving {} on http://{}",
        markdown_file_path.display(),
        args.address
    );

    let listener = TcpListener::bind(&args.address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve(
    State(rendered_markdown): State<Arc<RwLock<RenderedMarkdown>>>,
) -> impl IntoResponse {
    Html(rendered_markdown.read().await.content.clone())
}
