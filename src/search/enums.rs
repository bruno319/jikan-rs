pub enum Source {
    Anime,
    Manga,
    Person,
    Character,
}

impl Source {
    pub fn get_uri(&self) -> String {
        match self {
            Source::Anime => String::from("search/anime"),
            Source::Manga => String::from("search/manga"),
            Source::Person => String::from("search/person"),
            Source::Character => String::from("search/character")
        }
    }
}

pub enum SourceType {
    Anime(AnimeType),
    Manga(MangaType),
}

pub enum AnimeType {
    TV,
    OVA,
    Movie,
    Special,
    ONA,
    Music,
}

impl AnimeType {
    pub fn get_query(&self) -> String {
        match self {
            AnimeType::TV => String::from("type=tv"),
            AnimeType::OVA => String::from("type=ova"),
            AnimeType::Movie => String::from("type=movie"),
            AnimeType::Special => String::from("type=special"),
            AnimeType::ONA => String::from("type=ona"),
            AnimeType::Music => String::from("type=music"),
        }
    }
}

pub enum MangaType {
    Manga,
    Novel,
    OneShot,
    Doujin,
    Manhwa,
    Manhua,
}

impl MangaType {
    pub fn get_query(&self) -> String {
        match self {
            MangaType::Manga => String::from("type=manga"),
            MangaType::Novel => String::from("type=novel"),
            MangaType::OneShot => String::from("type=oneshot"),
            MangaType::Doujin => String::from("type=doujin"),
            MangaType::Manhwa => String::from("type=manhwa"),
            MangaType::Manhua => String::from("type=manhua"),
        }
    }
}

pub enum SourceStatus {
    Anime(AnimeStatus),
    Manga(MangaStatus),
}

pub enum AnimeStatus {
    Airing,
    Completed,
    Upcoming,
}

impl AnimeStatus {
    pub fn get_query(&self) -> String {
        match self {
            AnimeStatus::Airing => String::from("status=airing"),
            AnimeStatus::Completed => String::from("status=completed"),
            AnimeStatus::Upcoming => String::from("status=upcoming"),
        }
    }
}

pub enum MangaStatus {
    Publishing,
    Completed,
    Upcoming,
}

impl MangaStatus {
    pub fn get_query(&self) -> String {
        match self {
            MangaStatus::Publishing => String::from("status=publishing"),
            MangaStatus::Completed => String::from("status=completed"),
            MangaStatus::Upcoming => String::from("status=upcoming"),
        }
    }
}

pub enum Rating {
    G,
    Pg,
    Pg13,
    R17,
    R,
    Rx,
}

impl Rating {
    pub fn get_query(&self) -> String {
        match self {
            Rating::G => String::from("rated=g"),
            Rating::Pg => String::from("rated=pg"),
            Rating::Pg13 => String::from("rated=pg13"),
            Rating::R17 => String::from("rated=r17"),
            Rating::R => String::from("rated=r"),
            Rating::Rx => String::from("rated=rx"),
        }
    }
}

pub enum OrderBy {
    Title,
    StartDate,
    EndDate,
    Score,
    SourceType,
    Members,
    Id,
    Episodes,
    Rating,
    Volumes,
    Chapters,
}

impl OrderBy {
    pub fn get_query(&self) -> String {
        match self {
            OrderBy::Title => String::from("order_by=title"),
            OrderBy::StartDate => String::from("order_by=start_date"),
            OrderBy::EndDate => String::from("order_by=end_date"),
            OrderBy::Score => String::from("order_by=score"),
            OrderBy::SourceType => String::from("order_by=type"),
            OrderBy::Members => String::from("order_by=members"),
            OrderBy::Id => String::from("order_by=id"),
            OrderBy::Episodes => String::from("order_by=episodes"),
            OrderBy::Rating => String::from("order_by=rating"),
            OrderBy::Volumes => String::from("order_by=volumes"),
            OrderBy::Chapters => String::from("order_by=chapters"),
        }
    }
}

pub enum Sort {
    Ascending,
    Descending,
}

impl Sort {
    pub fn get_query(&self) -> String {
        match self {
            Sort::Ascending => String::from("sort=ascending"),
            Sort::Descending => String::from("sort=descending")
        }
    }
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