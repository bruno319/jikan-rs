use serde::de;

use crate::base::{MALImageItem, MALTypeItem};

pub enum UserResultEnum {
    Profile(Profile),
    History(Vec<HistoryItem>),
    Friends(Vec<Friend>),
    AnimeList(Vec<AnimeListEntry>),
    MangaList(Vec<MangaListEntry>),
}

jikan_response_entity!(
    pub struct Profile {
        pub user_id: u32,
        pub username: String,
        pub url: String,
        pub image_url: Option<String>,
        pub last_online: String,
        pub gender: Option<String>,
        pub birthday: Option<String>,
        pub location: Option<String>,
        pub joined: String,
        pub anime_stats: UserAnimeStats,
        pub manga_stats: UserMangaStats,
        pub favorites: Favorites,
        pub about: Option<String>,
    }
);

#[derive(Deserialize, Debug)]
pub struct UserAnimeStats {
    pub days_watched: f32,
    pub mean_score: f32,
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
    pub total_entries: u32,
    pub rewatched: u32,
    pub episodes_watched: u64,
}

#[derive(Deserialize, Debug)]
pub struct UserMangaStats {
    pub days_read: f32,
    pub mean_score: f32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_read: u32,
    pub total_entries: u32,
    pub reread: u32,
    pub chapters_read: u64,
    pub volumes_read: u64,
}

#[derive(Deserialize, Debug)]
pub struct Favorites {
    pub anime: Vec<MALImageItem>,
    pub manga: Vec<MALImageItem>,
    pub characters: Vec<MALImageItem>,
    pub people: Vec<MALImageItem>,
}

jikan_response_entity!(
    pub struct HistoryResponse {
        pub history: Vec<HistoryItem>,
    }
);

#[derive(Deserialize, Debug)]
pub struct HistoryItem {
    pub meta: MALTypeItem,
    pub increment: u64,
    pub date: String,
}

jikan_response_entity!(
    pub struct FriendResponse {
        pub friends: Vec<Friend>,
    }
);

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub username: String,
    pub url: String,
    pub image_url: Option<String>,
    pub last_online: Option<String>,
    pub friends_since: Option<String>,
}

jikan_response_entity!(
    pub struct AnimeListResponse {
        pub anime: Vec<AnimeListEntry>,
    }
);

#[derive(Deserialize, Debug)]
pub struct AnimeListEntry {
    pub mal_id: u32,
    pub title: String,
    pub url: String,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    #[serde(deserialize_with = "define_status")]
    pub watching_status: bool,
    pub score: u8,
    pub watched_episodes: u32,
    pub total_episodes: u32,
    #[serde(deserialize_with = "define_status")]
    pub airing_status: bool,
    pub has_episode_video: bool,
    pub has_promo_video: bool,
    pub has_video: bool,
    pub is_rewatching: bool,
    pub rating: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub watch_start_date: Option<String>,
    pub watch_end_date: Option<String>,
    pub days: Option<u32>,
    pub priority: Option<String>,
    pub added_to_list: bool,
}

jikan_response_entity!(
    pub struct MangaListResponse {
        pub manga: Vec<MangaListEntry>,
    }
);

#[derive(Deserialize, Debug)]
pub struct MangaListEntry {
    pub mal_id: u32,
    pub title: String,
    pub url: String,
    pub image_url: Option<String>,
    #[serde(rename = "type")]
    pub manga_type: String,
    #[serde(deserialize_with = "define_status")]
    pub reading_status: bool,
    pub score: u8,
    pub read_chapters: u32,
    pub total_chapters: u32,
    pub read_volumes: u32,
    pub total_volumes: u32,
    #[serde(deserialize_with = "define_status")]
    pub publishing_status: bool,
    pub is_rereading: bool,
    pub rating: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub read_start_date: Option<String>,
    pub read_end_date: Option<String>,
    pub days: Option<u32>,
    pub priority: Option<String>,
    pub added_to_list: bool,
}

fn define_status<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: de::Deserializer<'de>,
{
    let status: u8 = de::Deserialize::deserialize(deserializer)?;
    if status == 1 { Ok(true) } else { Ok(false) }
}
