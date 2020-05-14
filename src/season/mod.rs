use reqwest::Client;

use crate::base::{AnimeInfo, Resource};
use crate::client::BASE_URL;

pub mod archive;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_season(season: Season, http_clt: &Client) -> Result<SeasonResult> {
    let url = format!("{}/season/{}", BASE_URL, season.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let season_result: SeasonResult = serde_json::from_str(&body)?;

    Ok(season_result)
}

#[derive(Clone)]
pub enum Season {
    Winter(u16),
    Fall(u16),
    Summer(u16),
    Spring(u16),
    Later,
}

impl Resource for Season {
    fn uri(&self) -> String {
        match self {
            Season::Winter(year) => format!("{}/winter", year),
            Season::Fall(year) => format!("{}/fall", year),
            Season::Summer(year) => format!("{}/summer", year),
            Season::Spring(year) => format!("{}/spring", year),
            Season::Later => format!("later"),
        }
    }
}

jikan_response_entity!(
    pub struct SeasonResult {
        pub season_name: String,
        pub season_year: Option<u16>,
        #[serde(rename = "anime")]
        pub animes: Vec<AnimeInfo>,
    }
);