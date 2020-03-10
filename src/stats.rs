use hyper::{Client, Body};
use hyper::client::HttpConnector;
use crate::client::BASE_URL;
use bytes::buf::BufExt as _;
use crate::base::TypeSource;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn find_stats(mal_id: TypeSource, http_clt: &Client<HttpConnector, Body>) -> Result<Stats> {
    let url = format!("{}{}/stats", BASE_URL, mal_id.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let stats: Stats = serde_json::from_reader(body.reader())?;

    Ok(stats)
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
    pub total: u32,
    pub scores: Score,
}

#[derive(Deserialize, Debug)]
pub enum Score {

}