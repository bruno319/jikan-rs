use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::anime;
use crate::anime::Anime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct JikanClient {
    http_client: Client<HttpConnector, Body>,
}

impl JikanClient {
    pub fn new() -> JikanClient {
        JikanClient {
            http_client: Client::new()
        }
    }

    pub async fn find_anime_by_id(&self, id: &str) -> Result<Anime> {
        anime::find_anime_by_id(id, &self.http_client).await
    }
}