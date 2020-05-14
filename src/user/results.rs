use crate::base::{MALTypeItem, MALImageItem};

pub enum UserResultEnum {
    Profile(Profile),
    History(Vec<HistoryItem>),
    Friends(Vec<Friend>),
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