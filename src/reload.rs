use axum::body;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{
        Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::{Stream, StreamExt};
use once_cell::sync::Lazy;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub static RELOAD_TX: Lazy<broadcast::Sender<String>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(100);
    tx
});

pub static LIVERELOAD_SCRIPT_BYTES: &[u8] = br#"<script>
    new EventSource('/~~~meread-reload').onmessage = (e) => {
        if (e.data === 'reload') window.location.reload()
    };
</script>"#;

pub async fn reload_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
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

pub async fn append_livereload_script(request: Request, next: Next) -> Response {
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
