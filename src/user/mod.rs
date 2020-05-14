use reqwest::Client;

use crate::base::Resource;
use crate::client::BASE_URL;
use crate::user::enums::{AnimeListQuery, build_animelist_query, build_mangalist_query, HistorySource, MangaListQuery};
use crate::user::results::{FriendResponse, HistoryResponse, UserResultEnum};

pub mod enums;
pub mod results;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_user(username: &str, user_info: UserInfo, http_clt: &Client) -> Result<UserResultEnum> {
    let url = format!("{}/user/{}{}", BASE_URL, username, user_info.uri());
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let user_result = match user_info {
        UserInfo::Profile => UserResultEnum::Profile(serde_json::from_str(&body)?),
        UserInfo::History { source: _ } => {
            let response: HistoryResponse = serde_json::from_str(&body)?;
            UserResultEnum::History(response.history)
        }
        UserInfo::Friends { page: _ } => {
            let response: FriendResponse = serde_json::from_str(&body)?;
            UserResultEnum::Friends(response.friends)
        }
        UserInfo::Animelist { query: _ } => unimplemented!(),
        UserInfo::Mangalist { query: _ } => unimplemented!(),
    };
    Ok(user_result)
}

pub enum UserInfo {
    Profile,
    History { source: HistorySource },
    Friends { page: u16 },
    Animelist { query: AnimeListQuery },
    Mangalist { query: MangaListQuery },
}

impl Resource for UserInfo {
    fn uri(&self) -> String {
        match self {
            UserInfo::Profile => String::from("/profile"),
            UserInfo::History { source } => format!("/history/{}", source.uri()),
            UserInfo::Friends { page } => format!("/friends/{}", page),
            UserInfo::Animelist { query } => format!("/animelist/{}", build_animelist_query(query)),
            UserInfo::Mangalist { query } => format!("/mangalist/{}", build_mangalist_query(query)),
        }
    }
}