use crate::base::{AnimeStatusForUser, Date, MangaStatusForUser, Resource};
use crate::search::enums::{AnimeStatus, MangaStatus, Sort};
use crate::season::Season;

#[derive(Resource)]
pub enum HistorySource {
    Anime,
    Manga,
    #[rename_uri = ""]
    Both,
}

builder!(
    pub struct AnimeListQuery {
        title: String,
        status: AnimeStatusForUser,
        page: u16,
        sort: Sort,
        order_by: OrderAnimeListBy,
        order_by2: OrderAnimeListBy,
        aired_from: Date,
        aired_to: Date,
        producer: u32,
        season: Season,
        airing_status: AnimeStatus,
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
        title: String,
        status: MangaStatusForUser,
        page: u16,
        sort: Sort,
        order_by: OrderMangaListBy,
        order_by2: OrderMangaListBy,
        published_from: Date,
        published_to: Date,
        magazine: u32,
        publishing_status: MangaStatus,
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

pub(crate) fn build_animelist_query(query_builder: &AnimeListQuery) -> String {
    unimplemented!()
}

pub(crate) fn build_mangalist_query(query_builder: &MangaListQuery) -> String {
    unimplemented!()
}
