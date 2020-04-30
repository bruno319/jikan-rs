use reqwest::Client;

use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_reviews(mal_id: SourceType, page: &u16, http_clt: &Client) -> Result<Reviews> {
    let url = format!("{}{}/reviews/{}", BASE_URL, mal_id.get_uri(), page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response = match mal_id {
        SourceType::Anime(_) => {
            let response: ResponseReview<AnimeReviewer> = serde_json::from_str(&body)?;
            Reviews::Anime(response.reviews)
        }
        SourceType::Manga(_) => {
            let response: ResponseReview<MangaReviewer> = serde_json::from_str(&body)?;
            Reviews::Manga(response.reviews)
        }
        _ => return Err(Box::from("There is no reviews for this type source")),
    };
    Ok(response)
}

pub enum Reviews {
    Anime(Vec<Review<AnimeReviewer>>),
    Manga(Vec<Review<MangaReviewer>>),
}

#[derive(Deserialize, Debug)]
pub struct ResponseReview<T> {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub reviews: Vec<Review<T>>,
}

#[derive(Deserialize, Debug)]
pub struct Review<T> {
    pub mal_id: u32,
    pub url: String,
    pub helpful_count: u32,
    pub date: Option<String>,
    pub reviewer: T,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AnimeReviewer {
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub username: String,
    pub episodes_seen: u16,
    pub scores: AnimeScores,
}

#[derive(Deserialize, Debug)]
pub struct AnimeScores {
    pub overall: u8,
    pub story: u8,
    pub animation: u8,
    pub sound: u8,
    pub character: u8,
    pub enjoyment: u8,
}

#[derive(Deserialize, Debug)]
pub struct MangaReviewer {
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub username: String,
    pub chapters_read: u16,
    pub scores: MangaScores,
}

#[derive(Deserialize, Debug)]
pub struct MangaScores {
    pub overall: u8,
    pub story: u8,
    pub art: u8,
    pub character: u8,
    pub enjoyment: u8,
}
