use reqwest::Client;

use crate::base::Resource;
use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_more_info(mal_id: SourceType, http_clt: &Client) -> Result<Option<String>> {
    let url = format!("{}{}/moreinfo", BASE_URL, mal_id.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;

    Ok(response.moreinfo)
}

jikan_response_entity! (
    struct Response {
        moreinfo: Option<String>,
    }
);