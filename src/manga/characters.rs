use bytes::buf::BufExt;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_characters(mal_id: &u32, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Character>> {
    let url = format!("{}/manga/{}/characters", BASE_URL, mal_id).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

    Ok(response.characters)
}


#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub characters: Vec<Character>,
}

#[derive(Deserialize, Debug)]
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub role: String,
}