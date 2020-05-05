use reqwest::Client;

use crate::base::{AnimeStatusForUser, MangaStatusForUser, SourceType};
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_user_updates(mal_id: SourceType, page: &u16, http_clt: &Client) -> Result<UserUpdates> {
    let url = format!("{}{}/userupdates/{}", BASE_URL, mal_id.get_uri(), page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let user_updates = match mal_id {
        SourceType::Anime(_) => {
            let user_updates: AnimeUserUpdatesResponse = serde_json::from_str(&body)?;
            UserUpdates::Anime(user_updates.users)
        }
        SourceType::Manga(_) => {
            let user_updates: MangaUserUpdatesResponse = serde_json::from_str(&body)?;
            UserUpdates::Manga(user_updates.users)
        }
        _ => return Err(Box::from("There is no user updates for this type source")),
    };

    Ok(user_updates)
}

pub enum UserUpdates {
    Anime(Vec<AnimeUserUpdate>),
    Manga(Vec<MangaUserUpdate>),
}

#[derive(Deserialize, Debug)]
struct AnimeUserUpdatesResponse {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    users: Vec<AnimeUserUpdate>,
}

#[derive(Deserialize, Debug)]
struct MangaUserUpdatesResponse {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    users: Vec<MangaUserUpdate>,
}

#[derive(Deserialize, Debug)]
pub struct AnimeUserUpdate {
    pub username: String,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub score: Option<u8>,
    pub status: Option<AnimeStatusForUser>,
    pub episodes_seen: Option<u16>,
    pub episodes_total: Option<u16>,
    pub date: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MangaUserUpdate {
    pub username: String,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub score: Option<u8>,
    pub status: Option<MangaStatusForUser>,
    pub volumes_read: Option<u16>,
    pub volumes_total: Option<u16>,
    pub chapters_read: Option<u16>,
    pub chapters_total: Option<u16>,
    pub date: Option<String>,
}