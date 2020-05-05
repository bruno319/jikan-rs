use reqwest::Client;

use crate::base::{MALTypeItem, MangaInfo};
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_magazine(id: u32, page: &u16, http_clt: &Client) -> Result<Magazine> {
    let url = format!("{}/magazine/{}/{}", BASE_URL, id, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let magazine: Magazine = serde_json::from_str(&body)?;

    Ok(magazine)
}

#[derive(Deserialize, Debug)]
pub struct Magazine {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    #[serde(rename = "meta")]
    pub data: MALTypeItem,
    #[serde(rename = "manga")]
    pub mangas: Vec<MangaInfo>,
}