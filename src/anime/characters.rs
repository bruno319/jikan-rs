use reqwest::Client;

use crate::base::VoiceActor;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_characters(mal_id: &u32, http_clt: &Client) -> Result<CharactersStaff> {
    let url = format!("{}/anime/{}/characters_staff", BASE_URL, mal_id);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let characters_staff: CharactersStaff = serde_json::from_str(&body)?;

    Ok(characters_staff)
}


#[derive(Deserialize, Debug)]
pub struct CharactersStaff {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub characters: Vec<AnimeCharacter>,
    pub staff: Vec<StaffMember>,
}

#[derive(Deserialize, Debug)]
pub struct AnimeCharacter {
    pub mal_id: u32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub role: String,
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Deserialize, Debug)]
pub struct StaffMember {
    pub mal_id: u32,
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub positions: Vec<String>,
}