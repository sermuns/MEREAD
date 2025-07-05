use anyhow::{Context, Result};
use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};
use axum_embed::ServeEmbed;
use clap::Parser;
use notify::Watcher;
use rust_embed::RustEmbed;
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to markdown file. If directory, fallback to README.md
    #[arg(default_value = ".")]
    path: PathBuf,

    /// (If supplied) export the rendered markdown to given html path
    #[arg(long, short)]
    export_path: Option<PathBuf>,

    /// Address to bind the server to
    #[arg(long, short, default_value = "localhost:3000")]
    address: String,

    /// Open rendered markdown in default browser
    #[arg(long, short)]
    open: bool,
}

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let markdown_file_path = if args.path.is_dir() {
        args.path.join("README.md")
    } else {
        args.path.clone()
    };

    if let Some(export_path) = &args.export_path {
        let markdown_content =
            fs::read_to_string(&markdown_file_path).context("Failed to read markdown file")?;
        let rendered_html = render_markdown(
            &markdown_content,
            markdown_file_path.file_name().unwrap().to_str().unwrap(),
        )
        .await
        .context("Failed to render markdown")?;

        fs::write(export_path, rendered_html)
            .context(format!("Failed to write to {}", export_path.display()))?;
        println!("Exported rendered markdown to {}", export_path.display());
        return Ok(());
    }

    let livereload_layer = LiveReloadLayer::new();
    let reloader = livereload_layer.reloader();

    let app = Router::new()
        .route("/", get(move || serve_markdown(markdown_file_path.clone())))
        .nest_service("/assets", ServeEmbed::<Assets>::new())
        .fallback_service(ServeDir::new(&args.path))
        .layer(livereload_layer);

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(&args.path, notify::RecursiveMode::Recursive)?;

    if args.open {
        if let Err(e) = open::that(format!("http://{}", &args.address)) {
            eprintln!("Failed to open browser: {}", e);
        }
    }

    println!("Serving {:?} on http://{}", &args.path, &args.address);

    let listener = TcpListener::bind(&args.address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "template.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    contents: &'a str,
}

async fn render_markdown(markdown_content: &str, title: &str) -> anyhow::Result<String> {
    let contents = markdown::to_html_with_options(
        &markdown_content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )
    .map_err(|e| anyhow::anyhow!("Failed to convert markdown to HTML: {}", e))?;

    let rendered_html = HtmlTemplate {
        title: &title,
        contents: &contents,
    }
    .render()
    .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;

    Ok(rendered_html)
}

async fn serve_markdown(markdown_file_path: PathBuf) -> Result<Html<String>, (StatusCode, String)> {
    let markdown_content = fs::read_to_string(&markdown_file_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read README.md: {}", e),
        )
    })?;

    Ok(Html(
        render_markdown(
            &markdown_content,
            markdown_file_path.file_name().unwrap().to_str().unwrap(),
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    ))
}
