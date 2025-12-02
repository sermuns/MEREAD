use axum::{
    http::Uri,
    response::{IntoResponse, Response},
};
use rust_embed::Embed;

#[derive(Embed, Debug)]
#[folder = "assets/"]
pub struct EmbeddedAssets;

struct EmbeddedAsset<T>(T);

impl<T: Into<String>> IntoResponse for EmbeddedAsset<T> {
    fn into_response(self) -> Response {
        use axum::http::{StatusCode, header::CONTENT_TYPE};
        let path = self.0.into();

        match EmbeddedAssets::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
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
