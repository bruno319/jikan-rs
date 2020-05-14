use reqwest::Client;

use crate::base::MALRoleItem;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_characters(mal_id: &u32, http_clt: &Client) -> Result<Vec<MALRoleItem>> {
    let url = format!("{}/manga/{}/characters", BASE_URL, mal_id);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;

    Ok(response.characters)
}

jikan_response_entity!(
    struct Response {
        characters: Vec<MALRoleItem>,
    }
);