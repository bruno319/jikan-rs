use reqwest::Client;

use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_anime_episodes(mal_id: u32, http_clt: &Client) -> Result<Vec<EpisodeInfo>> {
    let mut page = 1 as u8;

    let response = make_request(mal_id, http_clt, page).await?;

    let mut episodes = response.episodes;
    let mut total_pages = response.episodes_last_page;

    while total_pages > 1 {
        total_pages -= 1;
        page += 1;

        let mut response = make_request(mal_id, http_clt, page).await?;
        episodes.append(&mut response.episodes);
    }

    Ok(episodes)
}

async fn make_request(mal_id: u32, http_clt: &Client, page: u8) -> Result<Response> {
    let url = format!("{}/anime/{}/episodes/{}", BASE_URL, mal_id, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;
    Ok(response)
}

jikan_response_entity!(
    struct Response {
        episodes_last_page: u8,
        episodes: Vec<EpisodeInfo>,
    }
);

#[derive(Deserialize, Debug)]
pub struct EpisodeInfo {
    pub episode_id: i16,
    pub title: Option<String>,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub aired: Option<String>,
    pub filler: bool,
    pub recap: bool,
    pub video_url: Option<String>,
    pub forum_url: Option<String>,
}
