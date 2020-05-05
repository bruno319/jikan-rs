use reqwest::Client;

use crate::base::MALTypeItem;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_top(top: Top, http_clt: &Client) -> Result<TopResult> {
    let url = format!("{}/top/{}", BASE_URL, top.get_uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let top_result = match top {
        Top::Anime { page: _, subtype: _ } => {
            let response: Response<RankedAnime> = serde_json::from_str(&body)?;
            TopResult::Anime(response.top)
        }
        Top::Manga { page: _, subtype: _ } => {
            let response: Response<RankedManga> = serde_json::from_str(&body)?;
            TopResult::Manga(response.top)
        }
        Top::Character(_) => {
            let response: Response<RankedCharacter> = serde_json::from_str(&body)?;
            TopResult::Character(response.top)
        }
        Top::People(_) => {
            let response: Response<RankedPerson> = serde_json::from_str(&body)?;
            TopResult::People(response.top)
        }
    };

    Ok(top_result)
}

pub enum Top {
    Anime { page: u16, subtype: TopAnimeSubtype },
    Manga { page: u16, subtype: TopMangaSubtype },
    Character(u16),
    People(u16),
}

impl Top {
    pub fn get_uri(&self) -> String {
        match self {
            Top::Anime { page, subtype } => format!("anime/{}/{}", page, subtype.get_uri_subtype()),
            Top::Manga { page, subtype } => format!("manga/{}/{}", page, subtype.get_uri_subtype()),
            Top::Character(page) => format!("characters/{}", page),
            Top::People(page) => format!("people/{}", page),
        }
    }
}

pub enum TopAnimeSubtype {
    All,
    TV,
    Movie,
    OVA,
    Special,
    Airing,
    Upcoming,
    ByPopularity,
    ByFavorite,
}

impl TopAnimeSubtype {
    fn get_uri_subtype(&self) -> &str {
        match self {
            TopAnimeSubtype::All => "",
            TopAnimeSubtype::TV => "tv",
            TopAnimeSubtype::Movie => "movie",
            TopAnimeSubtype::OVA => "ova",
            TopAnimeSubtype::Special => "special",
            TopAnimeSubtype::Airing => "airing",
            TopAnimeSubtype::Upcoming => "upcoming",
            TopAnimeSubtype::ByPopularity => "bypopularity",
            TopAnimeSubtype::ByFavorite => "byfavorite",
        }
    }
}

pub enum TopMangaSubtype {
    All,
    Manga,
    Novels,
    OneShots,
    Doujinshi,
    Manhwa,
    Manhua,
    ByPopularity,
    ByFavorite,
}

impl TopMangaSubtype {
    fn get_uri_subtype(&self) -> &str {
        match self {
            TopMangaSubtype::All => "",
            TopMangaSubtype::Manga => "manga",
            TopMangaSubtype::Novels => "novels",
            TopMangaSubtype::OneShots => "oneshots",
            TopMangaSubtype::Doujinshi => "doujin",
            TopMangaSubtype::Manhua => "manhua",
            TopMangaSubtype::Manhwa => "manhwa",
            TopMangaSubtype::ByPopularity => "bypopularity",
            TopMangaSubtype::ByFavorite => "byfavorite",
        }
    }
}

pub enum TopResult {
    Anime(Vec<RankedAnime>),
    Manga(Vec<RankedManga>),
    Character(Vec<RankedCharacter>),
    People(Vec<RankedPerson>),
}

#[derive(Deserialize, Debug)]
pub struct RankedAnime {
    pub mal_id: u32,
    pub rank: u32,
    pub title: String,
    pub url: String,
    pub image_url: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub episodes: Option<u16>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub members: Option<u32>,
    pub score: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RankedManga {
    pub mal_id: u32,
    pub rank: u32,
    pub title: String,
    pub url: String,
    pub image_url: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub volumes: Option<u16>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub members: Option<u32>,
    pub score: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct RankedCharacter {
    pub mal_id: u32,
    pub rank: u32,
    pub title: String,
    pub name_kanji: Option<String>,
    pub url: String,
    pub favorites: u32,
    pub image_url: Option<String>,
    pub animeography: Vec<MALTypeItem>,
    pub mangaography: Vec<MALTypeItem>,
}

#[derive(Deserialize, Debug)]
pub struct RankedPerson {
    pub mal_id: u32,
    pub rank: u32,
    pub title: String,
    pub name_kanji: Option<String>,
    pub url: String,
    pub favorites: u32,
    pub image_url: Option<String>,
    pub birthday: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    top: Vec<T>,
}