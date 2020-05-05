use reqwest::Client;

use crate::base::{AnimeInfo, MALTypeItem};
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_producer(id: u32, page: &u16, http_clt: &Client) -> Result<Producer> {
    let url = format!("{}/producer/{}/{}", BASE_URL, id, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let producer: Producer = serde_json::from_str(&body)?;

    Ok(producer)
}

#[derive(Deserialize, Debug)]
pub struct Producer {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    #[serde(rename = "meta")]
    pub data: MALTypeItem,
    #[serde(rename = "anime")]
    pub animes: Vec<AnimeInfo>,
}