use reqwest::Client;

use crate::base::{MALRoleItem, SourceType, VoiceActor};
use crate::client::BASE_URL;
use crate::pictures;
use crate::pictures::Picture;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_character(mal_id: u32, http_clt: &Client) -> Result<Character> {
    let url = format!("{}/character/{}", BASE_URL, mal_id);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let mut character: Character = serde_json::from_str(&body)?;

    character.client = http_clt.clone();

    Ok(character)
}

jikan_response_entity!(
    pub struct Character {
        #[serde(skip)]
        client: Client,
        pub mal_id: u32,
        pub url: String,
        pub image_url: String,
        pub name: String,
        pub name_kanji: Option<String>,
        pub nicknames: Vec<String>,
        pub about: Option<String>,
        pub member_favorites: u32,
        pub animeography: Vec<MALRoleItem>,
        pub mangaography: Vec<MALRoleItem>,
        pub voice_actors: Vec<VoiceActor>
    }
);

impl Character {
    pub async fn get_pictures(&self) -> Result<Vec<Picture>> {
        pictures::find_pictures(SourceType::Character(self.mal_id), &self.client).await
    }
}