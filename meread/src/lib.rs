use std::{
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
    time::Duration,
};

use bus::{Bus, BusReader};
use rouille::{Response, ResponseBody, try_or_400, websocket};

use crate::{
    assets::EmbeddedAsset,
    comrak_config::ComrakConfig,
    render::{RawMarkdown, RenderedMarkdown},
};
pub mod assets;
pub mod comrak_config;
pub mod export;
pub mod render;

pub fn serve_and_rebuild_on_receive(
    markdown_content_receiver: Receiver<RawMarkdown>,
    light_mode: bool,
    comrak_config: ComrakConfig,
    address: &str,
    open: bool,
) -> color_eyre::Result<()> {
    let initial_markdown = markdown_content_receiver.recv().unwrap();
    let rendered_markdown = Arc::new(Mutex::new(RenderedMarkdown::new(
        initial_markdown,
        light_mode,
        comrak_config,
    )?));

    let reload_bus = Arc::new(Mutex::new(Bus::new(1)));
    std::thread::spawn({
        let reload_bus = Arc::clone(&reload_bus);
        let rendered_markdown = Arc::clone(&rendered_markdown);
        move || {
            for RawMarkdown { mut content, .. } in &markdown_content_receiver {
                // debounce
                while let Ok(RawMarkdown { content: newer, .. }) =
                    markdown_content_receiver.recv_timeout(Duration::from_millis(50))
                {
                    content = newer;
                }

                rendered_markdown.lock().unwrap().rebuild(&content).unwrap();
                reload_bus.lock().unwrap().broadcast(());
            }
        }
    });

    if open {
        open::that(format!("http://{}", address)).ok();
    }

    #[cfg(feature = "stdout")]
    println!(
        "serving {} on http://{}",
        rendered_markdown.lock().unwrap().file_name,
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

            if path.is_empty() || path == rendered_markdown.file_name {
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
