use std::{
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bus::{Bus, BusReader};
use color_eyre::eyre::{Context, ContextCompat};
use jiff::Zoned;
use notify::EventKind;
use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use rouille::{Response, ResponseBody, try_or_400, websocket};

use crate::{assets::EmbeddedAsset, comrak_config::ComrakConfig, render::RenderedMarkdown};

pub mod assets;
pub mod comrak_config;
pub mod export;
pub mod render;

pub fn watch_and_serve(
    markdown_file_path: &Path,
    light_mode: bool,
    comrak_config: ComrakConfig,
    address: &str,
    open: bool,
) -> color_eyre::Result<()> {
    let rendered_markdown = Arc::new(Mutex::new(RenderedMarkdown::new(
        markdown_file_path,
        light_mode,
        comrak_config,
    )?));

    let reload_bus = Arc::new(Mutex::new(Bus::new(1)));

    let mut debouncer = new_debouncer(Duration::from_millis(100), None, {
        let rendered_markdown = Arc::clone(&rendered_markdown);
        let reload_bus = Arc::clone(&reload_bus);
        move |result: DebounceEventResult| {
            if let Ok(events) = result
                && events.iter().any(|e| {
                    matches!(
                        e.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    )
                })
            {
                let now_time = Zoned::now().time();

                println!("[{}] file changed, rebuilding...", now_time);

                rendered_markdown.lock().unwrap().rebuild().unwrap();
                let _ = reload_bus.lock().unwrap().try_broadcast(());
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

    if open {
        open::that(format!("http://{}", address)).ok();
    }

    println!(
        "serving {} on http://{}",
        markdown_file_path.display(),
        address
    );

    rouille::start_server(address, {
        move |request| {
            if request.method() != "GET" {
                return Response {
                    status_code: 405, // method not allowed
                    headers: Default::default(),
                    data: ResponseBody::empty(),
                    upgrade: None,
                };
            }

            let url = request.url();

            if url == "/~~~meread-reload" {
                let (response, websocket) =
                    try_or_400!(websocket::start(request, None as Option<&str>));

                let rendered_markdown = Arc::clone(&rendered_markdown);

                let reload_rx = reload_bus.lock().unwrap().add_rx();

                thread::spawn(move || {
                    let ws = websocket.recv().unwrap();
                    reload_handler_thread(ws, rendered_markdown, reload_rx)
                });

                return response;
            }

            let path = url.strip_prefix("/").unwrap();

            let rendered_markdown = rendered_markdown.lock().unwrap();

            if path.is_empty() || path == &rendered_markdown.path {
                Response::html(rendered_markdown.content.clone())
            } else {
                EmbeddedAsset::create_response(path).unwrap_or(Response::empty_404())
            }
        }
    })
}

fn reload_handler_thread(
    mut ws: websocket::Websocket,
    rendered_markdown: Arc<Mutex<RenderedMarkdown>>,
    reload_rx: BusReader<()>,
) {
    for _ in reload_rx.into_iter() {
        let rendered_markdown = rendered_markdown.lock().unwrap();
        ws.send_text(&rendered_markdown.content).unwrap();
    }
}
