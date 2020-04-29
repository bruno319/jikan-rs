use crate::base::MALTypeItem;

pub enum SearchResultEnum {
    Anime(SearchResult<AnimeResult>),
    Manga(SearchResult<MangaResult>),
    Person(SearchResult<PersonResult>),
    Character(SearchResult<CharacterResult>),
}

#[derive(Deserialize, Debug)]
pub struct SearchResult<T> {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub results: Vec<T>,
    pub last_page: u16,
}

#[derive(Deserialize, Debug)]
pub struct AnimeResult {
    pub mal_id: u32,
    pub url: String,
    pub image_url: Option<String>,
    pub title: String,
    pub airing: bool,
    pub synopsis: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub episodes: Option<u16>,
    pub score: Option<f32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub members: Option<u32>,
    pub rated: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MangaResult {
    pub mal_id: u32,
    pub url: String,
    pub image_url: Option<String>,
    pub title: String,
    pub publishing: bool,
    pub synopsis: Option<String>,
    #[serde(rename = "type")]
    pub manga_type: String,
    pub chapter: Option<u16>,
    pub volumes: Option<u16>,
    pub score: Option<f32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub members: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct PersonResult {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub alternative_names: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CharacterResult {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub alternative_names: Vec<String>,
    pub anime: Vec<MALTypeItem>,
    pub manga: Vec<MALTypeItem>
}