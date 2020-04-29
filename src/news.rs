use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_news(mal_id: SourceType, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<News>> {
    let url = format!("{}{}/news", BASE_URL, mal_id.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

    Ok(response.articles)
}

#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub articles: Vec<News>,
}

#[derive(Deserialize, Debug)]
pub struct News {
    pub url: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub forum_url: Option<String>,
    pub image_url: Option<String>,
    pub comments: Option<u16>,
    pub intro: Option<String>,
}