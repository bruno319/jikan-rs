use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use crate::base::MALItem;

const BASE_URL: &str = "http://api.jikan.moe/v3/anime/";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_anime_by_id(id: &str, http_clt: &Client<HttpConnector, Body>) -> Result<Anime> {
    let url = format!("{}{}", BASE_URL, id).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let anime: Anime = serde_json::from_reader(body.reader())?;

    Ok(anime)
}

#[derive(Deserialize, Debug)]
pub struct Anime {
    pub request_hash: String,
    pub request_cached: bool,
    pub request_cache_expiry: i32,
    pub mal_id: i32,
    pub url: String,
    pub image_url: Option<String>,
    pub trailer_url: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub source: String,
    pub episodes: i16,
    pub status: String,
    pub airing: bool,
    pub aired: Aired,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<i32>,
    pub rank: Option<i32>,
    pub popularity: Option<i32>,
    pub members: Option<i32>,
    pub favorites: Option<i32>,
    pub synopsis: String,
    pub background: Option<String>,
    pub premiered: Option<String>,
    pub broadcast: Option<String>,
    pub related: RelatedContent
}

#[derive(Deserialize, Debug)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct RelatedContent {
    #[serde(rename = "Alternative Version", default = "default_content")]
    pub alternative_versions: Vec<MALItem>,
    #[serde(rename = "Alternative Setting", default = "default_content")]
    pub alternative_settings: Vec<MALItem>,
    #[serde(rename = "Adaptation", default = "default_content")]
    pub adaptations: Vec<MALItem>,
    #[serde(rename = "Character", default = "default_content")]
    pub characters: Vec<MALItem>,
    #[serde(rename = "Full story", default = "default_content")]
    pub full_stories: Vec<MALItem>,
    #[serde(rename = "Parent story", default = "default_content")]
    pub parent_stories: Vec<MALItem>,
    #[serde(rename = "Prequel", default = "default_content")]
    pub prequels: Vec<MALItem>,
    #[serde(rename = "Sequel", default = "default_content")]
    pub sequels: Vec<MALItem>,
    #[serde(rename = "Other", default = "default_content")]
    pub others: Vec<MALItem>,
    #[serde(rename = "Side story", default = "default_content")]
    pub side_stories: Vec<MALItem>,
    #[serde(rename = "Spin-off", default = "default_content")]
    pub spin_offs: Vec<MALItem>,
    #[serde(rename = "Summary", default = "default_content")]
    pub summaries: Vec<MALItem>,
}

fn default_content() -> Vec<MALItem> {
    Vec::with_capacity(0)
}