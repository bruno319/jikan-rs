use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::TypeSource;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_forum(mal_id: TypeSource, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Topic>> {
    let url = format!("{}{}/forum", BASE_URL, mal_id.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

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