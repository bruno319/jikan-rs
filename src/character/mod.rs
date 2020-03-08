use hyper::{Client, Body};
use hyper::client::HttpConnector;
use bytes::buf::BufExt;

const BASE_URL: &str = "http://api.jikan.moe/v3";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub enum TypeSource {
    Anime(String),
    Manga(String),
}

impl TypeSource {
    fn get_uri(&self) -> String {
        match self {
            TypeSource::Anime(id) => format!("/anime/{}/characters_staff", id),
            TypeSource::Manga(id) => format!("/manga/{}/characters", id),
        }
    }
}

pub(crate) async fn find_characters_by_id(id: TypeSource, http_clt: &Client<HttpConnector, Body>) -> Result<Vec<Character>> {
    let url = format!("{}{}", BASE_URL, id.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: Response = serde_json::from_reader(body.reader())?;

    Ok(response.characters)
}

#[derive(Deserialize, Debug)]
struct Response {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: i32,
    pub characters: Vec<Character>,
}

#[derive(Deserialize, Debug)]
pub struct Character {
    pub mal_id: i32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Deserialize, Debug)]
pub struct VoiceActor {
    pub mal_id: i32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub language: String,
}