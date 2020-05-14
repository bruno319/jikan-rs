use std::collections::HashMap;

use reqwest::Client;

use crate::base::Resource;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn retrieve_api_status(http_clt: &Client) -> Result<ApiStatus> {
    let url = format!("{}/meta/status", BASE_URL);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let api_status: ApiStatus = serde_json::from_str(&body)?;

    Ok(api_status)
}

pub(crate) async fn retrieve_request_info(about: InfoAbout, period: Period, offset: u32, http_clt: &Client) -> Result<HashMap<String, u16>> {
    let url = format!("{}/meta/requests/{}/{}/{}", BASE_URL, about.uri(), period.uri(), offset);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let request_info: HashMap<String, u16> = serde_json::from_str(&body)?;

    Ok(request_info)
}

#[derive(Deserialize, Debug)]
pub struct ApiStatus {
    pub cached_requests: u32,
    pub requests_today: u32,
    pub requests_this_week: u32,
    pub requests_this_month: u32,
    pub connected_clients: String,
    pub total_connections_received: String,
}

#[derive(Resource)]
pub enum InfoAbout {
    Anime,
    Manga,
    Character,
    Person,
    Search,
    Top,
    Schedule,
    Season,
}

#[derive(Resource)]
pub enum Period {
    Today,
    Weekly,
    Monthly,
}