use reqwest::Client;

use crate::base::{AnimeInfo, MALTypeItem};
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_producer(id: u32, page: u16, http_clt: &Client) -> Result<Producer> {
    let url = format!("{}/producer/{}/{}", BASE_URL, id, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let producer: Producer = serde_json::from_str(&body)?;

    Ok(producer)
}

jikan_response_entity!(
    pub struct Producer {
        #[serde(rename = "meta")]
        pub data: MALTypeItem,
        #[serde(rename = "anime")]
        pub animes: Vec<AnimeInfo>,
    }
);