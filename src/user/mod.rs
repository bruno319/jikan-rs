use percent_encoding::utf8_percent_encode;
use reqwest::Client;

use crate::base::{FRAGMENT, Resource};
use crate::client::BASE_URL;
use crate::search::enums::Sort;
use crate::user::enums::{AnimeListQuery, HistorySource, MangaListQuery};
use crate::user::results::{AnimeListResponse, FriendResponse, HistoryResponse, MangaListResponse, UserResultEnum};

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
        UserInfo::Animelist { query: _ } => {
            let response: AnimeListResponse = serde_json::from_str(&body)?;
            UserResultEnum::AnimeList(response.anime)
        }
        UserInfo::Mangalist { query: _ } => {
            let response: MangaListResponse = serde_json::from_str(&body)?;
            UserResultEnum::MangaList(response.manga)
        }
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
            UserInfo::Animelist { query } => format!("/animelist{}", build_animelist_query(query)),
            UserInfo::Mangalist { query } => format!("/mangalist{}", build_mangalist_query(query)),
        }
    }
}

fn build_animelist_query(query_builder: &AnimeListQuery) -> String {
    let mut query= String::from("");

    if let Some(status) = &query_builder.status {
        query = format!("/{}", status.uri());
    }

    match &query_builder.page {
        Some(page) => query = format!("{}?page={}", query, page),
        None => query = format!("{}?page=1", query),
    }

    if let Some(title) = &query_builder.title {
        query = format!("{}&q={}", query, utf8_percent_encode(title, FRAGMENT));
    }

    match &query_builder.sort {
        Some(sort) => query = format!("{}&{}", query, sort.uri()),
        None => query = format!("{}&{}", query, Sort::Descending.uri()),
    }

    if let Some(order) = &query_builder.order_by {
        query = format!("{}&order_by={}", query, order.uri());
    }

    if let Some(order) = &query_builder.order_by_2 {
        query = format!("{}&order_by2={}", query, order.uri());
    }

    if let Some(date) = &query_builder.aired_from {
        query = format!("{}&aired_from={}", query, date);
    }

    if let Some(date) = &query_builder.aired_to {
        query = format!("{}&aired_to={}", query, date);
    }

    if let Some(id) = &query_builder.producer {
        query = format!("{}&producer={}", query, id);
    }

    if let Some(status) = &query_builder.airing_status {
        query = format!("{}&airing_status={}", query, status.uri());
    }

    return query;
}

fn build_mangalist_query(query_builder: &MangaListQuery) -> String {
    let mut query= String::from("");

    if let Some(status) = &query_builder.status {
        query = format!("/{}", status.uri());
    }

    match &query_builder.page {
        Some(page) => query = format!("{}?page={}", query, page),
        None => query = format!("{}?page=1", query),
    }

    if let Some(title) = &query_builder.title {
        query = format!("{}&q={}", query, utf8_percent_encode(title, FRAGMENT));
    }

    match &query_builder.sort {
        Some(sort) => query = format!("{}&{}", query, sort.uri()),
        None => query = format!("{}&{}", query, Sort::Descending.uri()),
    }

    if let Some(order) = &query_builder.order_by {
        query = format!("{}&order_by={}", query, order.uri());
    }

    if let Some(order) = &query_builder.order_by_2 {
        query = format!("{}&order_by2={}", query, order.uri());
    }

    if let Some(date) = &query_builder.published_from {
        query = format!("{}&published_from={}", query, date);
    }

    if let Some(date) = &query_builder.published_to {
        query = format!("{}&published_to={}", query, date);
    }

    if let Some(id) = &query_builder.magazine {
        query = format!("{}&magazine={}", query, id);
    }

    if let Some(status) = &query_builder.publishing_status {
        query = format!("{}&publishing_status={}", query, status.uri());
    }

    return query;
}
