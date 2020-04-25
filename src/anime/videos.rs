use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_videos(mal_id: &u32, http_clt: &Client<HttpConnector, Body>) -> Result<Videos> {
    let url = format!("{}/anime/{}/videos", BASE_URL, mal_id).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let videos: Videos = serde_json::from_reader(body.reader())?;

    Ok(videos)
}

#[derive(Deserialize, Debug)]
pub struct Videos {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<Episode>,
}

#[derive(Deserialize, Debug)]
pub struct PromoVideo {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub title: Option<String>,
    pub episode: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}