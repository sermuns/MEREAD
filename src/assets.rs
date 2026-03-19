use axum::{
    http::{StatusCode, Uri, header::CONTENT_TYPE},
    response::IntoResponse,
};
use rust_embed::Embed;

#[derive(Embed, Debug)]
#[folder = "assets/"]
pub struct EmbeddedAssets;

pub async fn assets_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    if let Some(content) = EmbeddedAssets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(CONTENT_TYPE, mime.as_ref())], content.data).into_response()
    } else {
        (StatusCode::NOT_FOUND, "404 Not Found").into_response()
    }
}
