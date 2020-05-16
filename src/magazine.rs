use reqwest::Client;

use crate::base::{MALTypeItem, MangaInfo};
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_magazine(id: u32, page: u16, http_clt: &Client) -> Result<Magazine> {
    let url = format!("{}/magazine/{}/{}", BASE_URL, id, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let magazine: Magazine = serde_json::from_str(&body)?;

    Ok(magazine)
}

jikan_response_entity!(
    pub struct Magazine {
        #[serde(rename = "meta")]
        pub data: MALTypeItem,
        #[serde(rename = "manga")]
        pub mangas: Vec<MangaInfo>,
    }
);