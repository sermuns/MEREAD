use rouille::Response;
use rust_embed::Embed;

#[derive(Embed, Debug)]
#[folder = "assets/"]
pub struct EmbeddedAsset;

impl EmbeddedAsset {
    pub fn create_response(path: &str) -> Option<Response> {
        Self::get(path).map(|content| {
            let content_type = mime_guess::from_path(path).first_or_octet_stream();
            Response::from_data(content_type.to_string(), content.data)
        })
    }
}
