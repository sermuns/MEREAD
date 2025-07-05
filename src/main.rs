use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};
use clap::Parser;
use include_dir::include_dir;
use notify::Watcher;
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let livereload_layer = LiveReloadLayer::new();
    let reloader = livereload_layer.reloader();

    let app = Router::new()
        .route("/", get(render_markdown))
        .fallback_service(ServeDir::new(
            include_dir!("$CARGO_MANIFEST_DIR/assets").path(),
        ))
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

async fn render_markdown() -> Result<Html<String>, (StatusCode, String)> {
    let body_contents = markdown::to_html_with_options(
        &fs::read_to_string("README.md").map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read README.md: {}", e.to_string()),
            )
        })?,
        &markdown::Options::gfm(),
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render markdown: {}", e.to_string()),
        )
    })?;
    Ok(Html(
        HtmlTemplate {
            title: "hehe",
            contents: &body_contents,
        }
        .render()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", e.to_string()),
            )
        })?,
    ))
}
