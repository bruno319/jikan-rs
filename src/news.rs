use reqwest::Client;

use crate::base::Resource;
use crate::base::SourceType;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_news(mal_id: SourceType, http_clt: &Client) -> Result<Vec<News>> {
    let url = format!("{}{}/news", BASE_URL, mal_id.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&body)?;

    Ok(response.articles)
}

jikan_response_entity!(
    struct Response {
        articles: Vec<News>,
    }
);

#[derive(Deserialize, Debug)]
pub struct News {
    pub url: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub forum_url: Option<String>,
    pub image_url: Option<String>,
    pub comments: Option<u16>,
    pub intro: Option<String>,
}