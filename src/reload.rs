use axum::body;
use axum::http::StatusCode;
use axum::middleware::ResponseAxumBody;
use axum::{
    extract::Request,
    middleware::Next,
    response::{
        Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::{Stream, StreamExt};
use once_cell::sync::Lazy;
use std::convert::Infallible;
use tokio::sync::broadcast;

pub static RELOAD_TX: Lazy<broadcast::Sender<String>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(100);
    tx
});

static LIVERELOAD_SCRIPT_BYTES: &[u8] = br#"<script>
    new EventSource('/~~~meread-reload').onmessage = (e) => {
        if (e.data === 'reload') window.location.reload()
    };
</script>"#;

pub async fn reload_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use std::time::Duration;
    use tokio_stream::wrappers::BroadcastStream;

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

    let content_type = response
        .headers()
        .get(hyper::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !content_type.starts_with("text/html") {
        return response;
    }

    let (mut parts, body) = response.into_parts();

    let Ok(body_bytes) = body::to_bytes(body, usize::MAX).await else {
        return Response::from_parts(parts, body::Body::empty());
    };

    let mut modified = Vec::with_capacity(body_bytes.len() + LIVERELOAD_SCRIPT_BYTES.len());
    modified.extend_from_slice(&body_bytes);
    modified.extend_from_slice(LIVERELOAD_SCRIPT_BYTES);

    parts.headers.remove(hyper::header::CONTENT_LENGTH);

    Response::from_parts(parts, body::Body::from(modified))
}
