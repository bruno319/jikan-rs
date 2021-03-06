use crate::base::Resource;

#[derive(Resource)]
#[uri_prefix = "/search/"]
pub enum SearchSource {
    Anime,
    Manga,
    Person,
    Character,
}

pub enum SearchSourceType {
    Anime(AnimeType),
    Manga(MangaType),
}

#[derive(Resource)]
#[uri_prefix = "type="]
pub enum AnimeType {
    TV,
    OVA,
    Movie,
    Special,
    ONA,
    Music,
}

#[derive(Resource)]
#[uri_prefix = "type="]
pub enum MangaType {
    Manga,
    Novel,
    OneShot,
    Doujin,
    Manhwa,
    Manhua,
}

pub enum SourceStatus {
    Anime(AnimeStatus),
    Manga(MangaStatus),
}

#[derive(Resource, Clone)]
pub enum AnimeStatus {
    Airing,
    #[rename_uri = "complete"]
    Completed,
    ToBeAired,
}

#[derive(Resource)]
pub enum MangaStatus {
    Publishing,
    #[rename_uri = "complete"]
    Completed,
    ToBePublished,
}

#[derive(Resource)]
#[uri_prefix = "rated="]
pub enum Rating {
    G,
    Pg,
    Pg13,
    R17,
    R,
    Rx,
}

#[derive(Resource)]
#[uri_prefix = "order_by="]
pub enum OrderBy {
    Title,
    StartDate,
    EndDate,
    Score,
    Type,
    Members,
    Id,
    Episodes,
    Rating,
    Volumes,
    Chapters,
}

#[derive(Resource, Clone)]
#[uri_prefix = "sort="]
pub enum Sort {
    Ascending,
    Descending,
}

pub enum Genres {
    Anime(Vec<AnimeGenre>),
    Manga(Vec<MangaGenre>),
}

#[derive(Copy, Clone)]
pub enum AnimeGenre {
    Action = 1,
    Adventure = 2,
    Cars = 3,
    Comedy = 4,
    Dementia = 5,
    Demons = 6,
    Mystery = 7,
    Drama = 8,
    Ecchi = 9,
    Fantasy = 10,
    Game = 11,
    Hentai = 12,
    Historical = 13,
    Horror = 14,
    Kids = 15,
    Magic = 16,
    MartialArts = 17,
    Mecha = 18,
    Music = 19,
    Parody = 20,
    Samurai = 21,
    Romance = 22,
    School = 23,
    SciFi = 24,
    Shoujo = 25,
    ShoujoAi = 26,
    Shounen = 27,
    ShounenAi = 28,
    Space = 29,
    Sports = 30,
    SuperPower = 31,
    Vampire = 32,
    Yaoi = 33,
    Yuri = 34,
    Harem = 35,
    SliceOfLife = 36,
    Supernatural = 37,
    Military = 38,
    Police = 39,
    Psychological = 40,
    Thriller = 41,
    Seinen = 42,
    Josei = 43,
}

#[derive(Copy, Clone)]
pub enum MangaGenre {
    Action = 1,
    Adventure = 2,
    Cars = 3,
    Comedy = 4,
    Dementia = 5,
    Demons = 6,
    Mystery = 7,
    Drama = 8,
    Ecchi = 9,
    Fantasy = 10,
    Game = 11,
    Hentai = 12,
    Historical = 13,
    Horror = 14,
    Kids = 15,
    Magic = 16,
    MartialArts = 17,
    Mecha = 18,
    Music = 19,
    Parody = 20,
    Samurai = 21,
    Romance = 22,
    School = 23,
    SciFi = 24,
    Shoujo = 25,
    ShoujoAi = 26,
    Shounen = 27,
    ShounenAi = 28,
    Space = 29,
    Sports = 30,
    SuperPower = 31,
    Vampire = 32,
    Yaoi = 33,
    Yuri = 34,
    Harem = 35,
    SliceOfLife = 36,
    Supernatural = 37,
    Military = 38,
    Police = 39,
    Psychological = 40,
    Seinen = 41,
    Josei = 42,
    Doujinshi = 43,
    GenderBender = 44,
    Thriller = 45,
}