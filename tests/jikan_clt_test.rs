extern crate jikan_rs;

use jikan_rs::client::JikanClient;
use jikan_rs::base::TypeSource;
use std::time::Duration;

#[tokio::test]
async fn should_find_an_anime() {
    let jikancl = JikanClient::new();
    let anime = jikancl.find_anime(1).await.unwrap();
    assert_eq!(anime.mal_id, 1);
    assert_eq!(anime.title, "Cowboy Bebop");
}

#[tokio::test]
async fn should_find_anime_characters() {
    let jikancl = JikanClient::new();
    let characters = jikancl.find_characters(TypeSource::Anime(1)).await.unwrap();
    assert!(characters.len() > 0);
}

#[tokio::test]
async fn should_find_manga_characters() {
    let jikancl = JikanClient::new();
    let characters = jikancl.find_characters(TypeSource::Manga(1)).await.unwrap();
    assert!(characters.len() > 0);
}

#[tokio::test]
async fn should_find_anime_episodes_info() {
    let jikancl = JikanClient::new();
    let episodes = jikancl.find_episodes(1).await.unwrap();
    assert_eq!(episodes.len(), 26);
}

#[tokio::test]
async fn should_find_anime_news() {
    let jikancl = JikanClient::new();
    let news = jikancl.find_news(TypeSource::Anime(1)).await.unwrap();
    assert!(news.len() > 0);
}

#[tokio::test]
async fn should_find_manga_news() {
    let jikancl = JikanClient::new();
    let news = jikancl.find_news(TypeSource::Manga(1)).await.unwrap();
    assert!(news.len() > 0);
}

#[tokio::test]
async fn should_find_anime_pictures() {
    let jikancl = JikanClient::new();
    let pictures = jikancl.find_pictures(TypeSource::Anime(1)).await.unwrap();
    assert!(pictures.len() > 0);
}

#[tokio::test]
async fn should_find_manga_pictures() {
    let jikancl = JikanClient::new();
    let pictures = jikancl.find_pictures(TypeSource::Manga(1)).await.unwrap();
    assert!(pictures.len() > 0);
}