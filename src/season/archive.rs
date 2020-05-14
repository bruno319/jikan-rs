use reqwest::Client;

use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_season_archives(http_clt: &Client) -> Result<Vec<ArchivedSeason>> {
    let url = format!("{}/season/archive", BASE_URL);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;

    Ok(response.archive)
}

jikan_response_entity!(
    struct Response {
        archive: Vec<ArchivedSeason>,
    }
);

#[derive(Deserialize, Debug)]
pub struct ArchivedSeason {
    pub year: u16,
    pub seasons: Vec<String>,
}