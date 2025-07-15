use anyhow::{Context, Result};
use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};
use axum_embed::ServeEmbed;
use clap::Parser;
use notify::Watcher;
use rust_embed::RustEmbed;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

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

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

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
        export_markdown(&markdown_file_path, export_dir, args.force).await?;
        return Ok(());
    }

    let livereload_layer = LiveReloadLayer::new();
    let reloader = livereload_layer.reloader();

    let app = Router::new()
        .route(
            "/",
            get({
                let md = markdown_file_path.clone();
                move || serve_markdown(md.clone())
            }),
        )
        .nest_service("/assets", ServeEmbed::<Assets>::new())
        .fallback_service(ServeDir::new((*root_path).clone()))
        .layer(livereload_layer);

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(root_path.as_ref(), notify::RecursiveMode::Recursive)?;

    if args.open {
        let _ = open::that(format!("http://{}", &args.address));
    }

    println!("Serving {} on http://{}", root_path.display(), args.address);

    let listener = TcpListener::bind(&args.address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn export_markdown(md_path: &Path, export_dir: &Path, force: bool) -> Result<()> {
    let markdown_content = fs::read_to_string(md_path).context("Failed to read markdown file")?;
    let rendered_html = render_markdown(
        &markdown_content,
        md_path.file_name().unwrap().to_str().unwrap(),
    )
    .await
    .context("Failed to render markdown")?;

    anyhow::ensure!(
        force || !export_dir.exists(),
        "Export directory already exists: {}, and --force was not supplied",
        export_dir.display()
    );

    fs::create_dir_all(export_dir).context("Failed to create export directory")?;
    fs::write(export_dir.join("index.html"), rendered_html).context("Failed to write HTML")?;

    let assets_dir = export_dir.join("assets");
    fs::create_dir_all(&assets_dir).context("Failed to create assets directory")?;

    for name in Assets::iter() {
        let content = Assets::get(name.as_ref()).unwrap().data;
        fs::write(assets_dir.join(name.as_ref()), content)?;
    }
    println!("Exported to {}", export_dir.display());
    Ok(())
}

#[derive(Template)]
#[template(path = "template.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    contents: &'a str,
}

async fn render_markdown(markdown_content: &str, title: &str) -> Result<String> {
    let contents = markdown::to_html_with_options(
        markdown_content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )
    .map_err(|e| anyhow::anyhow!("Failed to convert markdown: {e}"))?;

    Ok(HtmlTemplate {
        title,
        contents: &contents,
    }
    .render()?)
}

async fn serve_markdown(md_path: PathBuf) -> Result<Html<String>, (StatusCode, String)> {
    let content = fs::read_to_string(&md_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Read error: {e}"),
        )
    })?;
    render_markdown(&content, md_path.file_name().unwrap().to_str().unwrap())
        .await
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
