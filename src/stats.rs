use reqwest::Client;

use crate::base::Resource;
use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_stats(mal_id: SourceType, http_clt: &Client) -> Result<Stats> {
    let url = format!("{}{}/stats", BASE_URL, mal_id.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let stats = match mal_id {
        SourceType::Anime(_) => Stats::Anime(serde_json::from_str(&body)?),
        SourceType::Manga(_) => Stats::Manga(serde_json::from_str(&body)?),
        _ => return Err(Box::from("There is no stats for this type source")),
    };

    Ok(stats)
}

pub enum Stats {
    Anime(AnimeStats),
    Manga(MangaStats),
}

jikan_response_entity!(
    pub struct AnimeStats {
        pub watching: u32,
        pub completed: u32,
        pub on_hold: u32,
        pub dropped: u32,
        pub plan_to_watch: u32,
        pub total: u32,
        pub scores: Score,
    }
);

jikan_response_entity!(
    pub struct MangaStats {
        pub reading: u32,
        pub completed: u32,
        pub on_hold: u32,
        pub dropped: u32,
        pub plan_to_read: u32,
        pub total: u32,
        pub scores: Score,
    }
);

#[derive(Deserialize, Debug)]
pub struct Score {
    #[serde(rename = "1")]
    pub one: ScoreStats,
    #[serde(rename = "2")]
    pub two: ScoreStats,
    #[serde(rename = "3")]
    pub three: ScoreStats,
    #[serde(rename = "4")]
    pub four: ScoreStats,
    #[serde(rename = "5")]
    pub five: ScoreStats,
    #[serde(rename = "6")]
    pub six: ScoreStats,
    #[serde(rename = "7")]
    pub seven: ScoreStats,
    #[serde(rename = "8")]
    pub eight: ScoreStats,
    #[serde(rename = "9")]
    pub nine: ScoreStats,
    #[serde(rename = "10")]
    pub ten: ScoreStats,
}

#[derive(Deserialize, Debug)]
pub struct ScoreStats {
    pub votes: u32,
    pub percentage: f32,
}