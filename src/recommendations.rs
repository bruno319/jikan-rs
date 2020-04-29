use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_recommendations(mal_id: SourceType, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Recommendation>> {
    let url = format!("{}{}/recommendations", BASE_URL, mal_id.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

    Ok(response.recommendations)
}

#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    recommendations: Vec<Recommendation>,
}

#[derive(Deserialize, Debug)]
pub struct Recommendation {
    pub mal_id: u32,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub recommendation_url: Option<String>,
    pub title: String,
    pub recommendation_count: u32,
}
