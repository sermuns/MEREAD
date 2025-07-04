use axum::{Router, http::StatusCode, response::Html, routing::get};
use clap::Parser;
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
    let document = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Markdown Viewer</title>
        </head>
        <body>
            {}
        </body>
        </html>",
        body_contents
    );
    Ok(Html(document))
}
