use bytes::buf::BufExt;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_characters(mal_id: &u32, http_clt: &Client<HttpConnector, Body>) -> Result<CharactersStaff> {
    let url = format!("{}/anime/{}/characters_staff", BASE_URL, mal_id).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let characters_staff: CharactersStaff = serde_json::from_reader(body.reader())?;

    Ok(characters_staff)
}


#[derive(Deserialize, Debug)]
pub struct CharactersStaff {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub characters: Vec<Character>,
    pub staff: Vec<StaffMember>,
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

#[derive(Deserialize, Debug)]
pub struct StaffMember {
    pub mal_id: u32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub positions: Vec<String>,
}