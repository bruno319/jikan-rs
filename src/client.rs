use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::{anime, characters, news, pictures, stats};
use crate::anime::{Anime, episodes::Episode};
use crate::characters::Character;
use crate::base::TypeSource;
use crate::news::News;
use crate::pictures::Picture;
use crate::anime::videos::Videos;
use crate::stats::Stats;

pub const BASE_URL: &str = "http://api.jikan.moe/v3";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct JikanClient {
    http_client: Client<HttpConnector, Body>,
}

impl JikanClient {
    pub fn new() -> JikanClient {
        JikanClient {
            http_client: Client::new()
        }
    }

    pub async fn find_anime(&self, mal_id: &str) -> Result<Anime> {
        anime::find_anime(mal_id, &self.http_client).await
    }

    pub async fn find_characters_from(&self, mal_id: TypeSource) -> Result<Vec<Character>> {
        characters::find_characters(mal_id, &self.http_client).await
    }

    pub async fn find_episodes(&self, mal_id: &str) -> Result<Vec<Episode>> {
        anime::episodes::find_anime_episodes(&mal_id.to_string(), &self.http_client).await
    }

    pub async fn find_news(&self, mal_id: TypeSource) -> Result<Vec<News>> {
        news::find_news(mal_id, &self.http_client).await
    }

    pub async fn find_pictures(&self, mal_id: TypeSource) -> Result<Vec<Picture>> {
        pictures::find_pictures(mal_id, &self.http_client).await
    }

    pub async fn find_videos(&self, mal_id: &str) -> Result<Videos> {
        anime::videos::find_videos(&mal_id.to_string(), &self.http_client).await
    }

    pub async fn find_stats(&self, mal_id: TypeSource) -> Result<Stats> {
        stats::find_stats(mal_id, &self.http_client).await
    }
}