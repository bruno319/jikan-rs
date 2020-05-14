use reqwest::Client;

use crate::base::{MALImageItem, SourceType};
use crate::client::BASE_URL;
use crate::pictures;
use crate::pictures::Picture;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_person(mal_id: u32, http_clt: &Client) -> Result<Person> {
    let url = format!("{}/person/{}", BASE_URL, mal_id);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let mut person: Person = serde_json::from_str(&body)?;

    person.client = http_clt.clone();

    Ok(person)
}

jikan_response_entity!(
    pub struct Person {
        #[serde(skip)]
        client: Client,
        pub mal_id: u32,
        pub url: String,
        pub image_url: Option<String>,
        pub website_url: Option<String>,
        pub name: String,
        pub given_name: Option<String>,
        pub family_name: Option<String>,
        pub alternate_names: Vec<String>,
        pub birthday: Option<String>,
        pub member_favorites: Option<u32>,
        pub about: Option<String>,
        pub voice_acting_roles: Vec<VoiceActingRole>,
        pub anime_staff_positions: Vec<AnimeStaffPosition>,
        pub published_manga: Vec<PublishedManga>,
    }
);

#[derive(Deserialize, Debug)]
pub struct VoiceActingRole {
    pub role: String,
    pub anime: MALImageItem,
    pub character: MALImageItem,
}

#[derive(Deserialize, Debug)]
pub struct AnimeStaffPosition {
    pub position: String,
    pub anime: MALImageItem,
}

#[derive(Deserialize, Debug)]
pub struct PublishedManga {
    pub position: String,
    pub manga: MALImageItem,
}

impl Person {
    pub async fn get_pictures(&self) -> Result<Vec<Picture>> {
        pictures::find_pictures(SourceType::Person(self.mal_id), &self.client).await
    }
}