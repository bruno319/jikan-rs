use reqwest::Client;

use crate::base::Resource;
use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_forum(mal_id: SourceType, http_clt: &Client) -> Result<Vec<Topic>> {
    let url = format!("{}{}/forum", BASE_URL, mal_id.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;

    Ok(response.topics)
}

#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub topics: Vec<Topic>,
}

#[derive(Deserialize, Debug)]
pub struct Topic {
    pub topic_id: u32,
    pub url: Option<String>,
    pub title: Option<String>,
    pub date_posted: Option<String>,
    pub author_name: Option<String>,
    pub replies: u32,
    pub last_post: Option<LastPost>,
}

#[derive(Deserialize, Debug)]
pub struct LastPost {
    pub url: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub date_posted: Option<String>,
}