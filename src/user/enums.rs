use crate::base::{AnimeStatusForUser, Date, MangaStatusForUser, Resource};
use crate::search::enums::{AnimeStatus, MangaStatus, Sort};

#[derive(Resource)]
pub enum HistorySource {
    Anime,
    Manga,
    #[rename_uri = ""]
    Both,
}

builder!(
    pub struct AnimeListQuery {
        pub(crate) title: String,
        pub(crate) status: AnimeStatusForUser,
        pub(crate) page: u16,
        pub(crate) sort: Sort,
        pub(crate) order_by: OrderAnimeListBy,
        pub(crate) order_by_2: OrderAnimeListBy,
        pub(crate) aired_from: Date,
        pub(crate) aired_to: Date,
        pub(crate) producer: u32,
        pub(crate) airing_status: AnimeStatus,
    }
);

#[derive(Resource, Clone)]
pub enum OrderAnimeListBy {
    Title,
    FinishDate,
    StartDate,
    Score,
    LastUpdated,
    Type,
    Rated,
    Rewatch,
    Priority,
    EpisodesWatched,
    Storage,
    AirStart,
    AirEnd,
    Status,
}

builder!(
    pub struct MangaListQuery {
        pub(crate) title: String,
        pub(crate) status: MangaStatusForUser,
        pub(crate) page: u16,
        pub(crate) sort: Sort,
        pub(crate) order_by: OrderMangaListBy,
        pub(crate) order_by_2: OrderMangaListBy,
        pub(crate) published_from: Date,
        pub(crate) published_to: Date,
        pub(crate) magazine: u32,
        pub(crate) publishing_status: MangaStatus,
    }
);

#[derive(Resource)]
pub enum OrderMangaListBy {
    Title,
    FinishDate,
    StartDate,
    Score,
    LastUpdated,
    Type,
    Priority,
    ChaptersRead,
    VolumesRead,
    PublishStart,
    PublishEnd,
    Status,
}