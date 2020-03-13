use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::{TypeSource, AnimeStatus, MangaStatus};
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_user_updates(mal_id: TypeSource, page: &u16, http_clt: &Client<HttpConnector, Body>) -> Result<UserUpdates> {
    let url = format!("{}{}/userupdates/{}", BASE_URL, mal_id.get_uri(), page).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let user_updates = match mal_id {
        TypeSource::Anime(_) => {
            let user_updates: AnimeUserUpdatesResponse = serde_json::from_reader(body.reader())?;
            UserUpdates::Anime(user_updates.users)
        }
        TypeSource::Manga(_) => {
            let user_updates: MangaUserUpdatesResponse = serde_json::from_reader(body.reader())?;
            UserUpdates::Manga(user_updates.users)
        }
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
    pub status: Option<AnimeStatus>,
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
    pub status: Option<MangaStatus>,
    pub volumes_read: Option<u16>,
    pub volumes_total: Option<u16>,
    pub chapters_read: Option<u16>,
    pub chapters_total: Option<u16>,
    pub date: Option<String>,
}