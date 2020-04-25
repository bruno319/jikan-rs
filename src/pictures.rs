use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::TypeSource;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn find_pictures(mal_id: TypeSource, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Picture>> {
    let url = format!("{}{}/pictures", BASE_URL, mal_id.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

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