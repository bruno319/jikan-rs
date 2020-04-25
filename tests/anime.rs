extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::JikanClient;

lazy_static! {
    static ref JIKAN_CL: JikanClient = JikanClient::new();
}

#[tokio::test]
async fn should_get_anime_characters() {
    let characters = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_characters()
        .await.unwrap();
    assert!(!characters.is_empty());
}

#[tokio::test]
async fn should_get_anime_episodes() {
    let episodes = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_episodes()
        .await.unwrap();
    assert!(!episodes.is_empty());
}

#[tokio::test]
async fn should_get_anime_news() {
    let news = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_news()
        .await.unwrap();
    assert!(!news.is_empty());
}

#[tokio::test]
async fn should_get_anime_pictures() {
    let pictures = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_pictures()
        .await.unwrap();
    assert!(!pictures.is_empty());
}

#[tokio::test]
async fn should_get_anime_videos() {
    let videos = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_videos()
        .await.unwrap();
    assert!(!videos.episodes.is_empty());
    assert!(!videos.promo.is_empty());
}

#[tokio::test]
async fn should_get_anime_stats() {
    let stats = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_stats()
        .await.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_get_anime_forum() {
    let topics = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_forum()
        .await.unwrap();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn should_get_more_anime_info() {
    let more_info = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_more_info()
        .await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_get_anime_reviews() {
    let reviews = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_reviews(&1)
        .await.unwrap();
    assert!(reviews.len() > 0 && reviews.len() <= 20);
}

#[tokio::test]
async fn should_get_anime_recommendations() {
    let recommendations = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_recommendations()
        .await.unwrap();
    assert!(!recommendations.is_empty());
}

#[tokio::test]
async fn should_get_anime_user_updates() {
    let user_updates = JIKAN_CL.find_anime(1)
        .await.unwrap()
        .get_user_updates(&1)
        .await.unwrap();
    assert!(!user_updates.is_empty());
}
