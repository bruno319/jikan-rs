use hyper::{Client, Body};
use hyper::client::HttpConnector;
use crate::client::BASE_URL;
use bytes::buf::BufExt as _;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_reviews(mal_id: &u32, page: &u16, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Review>> {
    let url = format!("{}/anime/{}/reviews/{}", BASE_URL, mal_id, page).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

    Ok(response.reviews)
}

#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    reviews: Vec<Review>,
}

#[derive(Deserialize, Debug)]
pub struct Review {
    pub mal_id: u32,
    pub url: String,
    pub helpful_count: u32,
    pub date: Option<String>,
    pub reviewer: Reviewer,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Reviewer {
    pub url: Option<String>,
    pub image_url:Option<String>,
    pub username: String,
    pub episodes_seen: u16,
    pub scores: Scores,
}

#[derive(Deserialize, Debug)]
pub struct Scores {
    pub overall: u8,
    pub story: u8,
    pub animation: u8,
    pub sound: u8,
    pub character: u8,
    pub enjoyment: u8,
}