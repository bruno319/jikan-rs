use reqwest::Client;

use crate::base::Resource;
use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_pictures(mal_id: SourceType, http_clt: &Client) -> Result<Vec<Picture>> {
    let url = format!("{}{}/pictures", BASE_URL, mal_id.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;

    Ok(response.pictures)
}

#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub pictures: Vec<Picture>,
}

#[derive(Deserialize, Debug)]
pub struct Picture {
    pub large: Option<String>,
    pub small: Option<String>,
}