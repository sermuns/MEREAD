use axum::http::{StatusCode, header::CONTENT_TYPE};
use axum::{
    http::Uri,
    response::{IntoResponse, Response},
};
use rust_embed::Embed;

#[derive(Embed, Debug)]
#[folder = "assets/"]
pub struct EmbeddedAssets;

struct EmbeddedAsset(String);

impl IntoResponse for EmbeddedAsset {
    fn into_response(self) -> Response {
        let path_str = self.0;
        match EmbeddedAssets::get(&path_str) {
            Some(content) => {
                let mime = mime_guess::from_path(&path_str).first_or_octet_stream();
                ([(CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

pub async fn assets_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    EmbeddedAsset(path)
}
