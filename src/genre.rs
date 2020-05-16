use reqwest::Client;

use crate::base::{AnimeInfo, MALTypeItem, MangaInfo};
use crate::client::BASE_URL;
use crate::search::enums::{AnimeGenre, MangaGenre};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_animes_with_genre(genre: AnimeGenre, page: u16, http_clt: &Client) -> Result<GenreAnimeResult> {
    let url = format!("{}/genre/anime/{}/{}", BASE_URL, genre as u8, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let animes: GenreAnimeResult = serde_json::from_str(&body)?;

    Ok(animes)
}

pub(crate) async fn find_mangas_with_genre(genre: MangaGenre, page: u16, http_clt: &Client) -> Result<GenreMangaResult> {
    let url = format!("{}/genre/manga/{}/{}", BASE_URL, genre as u8, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let mangas: GenreMangaResult = serde_json::from_str(&body)?;

    Ok(mangas)
}

jikan_response_entity!(
    pub struct GenreAnimeResult {
        pub mal_url: MALTypeItem,
        pub item_count: u32,
        #[serde(rename = "anime")]
        pub animes: Vec<AnimeInfo>,
    }
);

jikan_response_entity!(
    pub struct GenreMangaResult {
        pub mal_url: MALTypeItem,
        pub item_count: u32,
        #[serde(rename = "manga")]
        pub mangas: Vec<MangaInfo>,
    }
);
