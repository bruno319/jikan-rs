use reqwest::Client;

use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_videos(mal_id: u32, http_clt: &Client) -> Result<Videos> {
    let url = format!("{}/anime/{}/videos", BASE_URL, mal_id);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let videos: Videos = serde_json::from_str(&body)?;

    Ok(videos)
}

jikan_response_entity!(
    pub struct Videos {
        pub promo: Vec<PromoVideo>,
        pub episodes: Vec<EpisodeVideo>,
    }
);

#[derive(Deserialize, Debug)]
pub struct PromoVideo {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeVideo {
    pub title: Option<String>,
    pub episode: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}