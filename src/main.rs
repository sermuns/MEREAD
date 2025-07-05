use anyhow::{Context, Result};
use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};
use axum_embed::ServeEmbed;
use clap::Parser;
use notify::Watcher;
use rust_embed::RustEmbed;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to serve
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Optional: run once and export the rendered markdown to given html path
    #[arg(long, short)]
    export_path: Option<PathBuf>,
}

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(export_path) = &args.export_path {
        let markdown_content =
            fs::read_to_string("README.md").context("Failed to read README.md")?;
        let rendered_html = render_markdown(markdown_content)
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
        .route("/", get(serve_markdown))
        .nest_service("/assets", ServeEmbed::<Assets>::new())
        .fallback_service(ServeDir::new(&args.path))
        .layer(livereload_layer);

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;

    watcher.watch(&args.path, notify::RecursiveMode::Recursive)?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Serving {:?} on http://{}", args.path, addr);
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "template.html")]
struct HtmlTemplate<'a> {
    title: &'a str,
    contents: &'a str,
}

async fn render_markdown(markdown_content: String) -> anyhow::Result<String> {
    let body_contents = markdown::to_html_with_options(
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
        title: "hehe",
        contents: &body_contents,
    }
    .render()
    .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;

    Ok(rendered_html)
}

async fn serve_markdown() -> Result<Html<String>, (StatusCode, String)> {
    let markdown_content = fs::read_to_string("README.md").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read README.md: {}", e),
        )
    })?;

    Ok(Html(render_markdown(markdown_content).await.map_err(
        |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    )?))
}
