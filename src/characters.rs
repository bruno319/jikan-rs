use bytes::buf::BufExt;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::TypeSource;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_characters(mal_id: TypeSource, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Character>> {
    let url = match mal_id {
        TypeSource::Anime(_) => format!("{}{}/characters_staff", BASE_URL, mal_id.get_uri()),
        _ => format!("{}{}/characters", BASE_URL, mal_id.get_uri()),
    };
    let res = http_clt.get(url.parse()?).await?;
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
    #[serde(default)]
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Deserialize, Debug)]
pub struct VoiceActor {
    pub mal_id: u32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub language: String,
}