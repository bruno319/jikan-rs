extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::Jikan;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref JIKAN: Jikan = Jikan::new();
}

#[tokio::test]
async fn should_get_manga_characters() {
    thread::sleep(Duration::from_secs(3));
    let characters = JIKAN.find_manga(1)
        .await.unwrap()
        .get_characters()
        .await.unwrap();
    assert!(!characters.is_empty());
}

#[tokio::test]
async fn should_get_manga_news() {
    thread::sleep(Duration::from_secs(3));
    let news = JIKAN.find_manga(1)
        .await.unwrap()
        .get_news()
        .await.unwrap();
    assert!(!news.is_empty());
}

#[tokio::test]
async fn should_get_manga_pictures() {
    thread::sleep(Duration::from_secs(3));
    let pictures = JIKAN.find_manga(1)
        .await.unwrap()
        .get_pictures()
        .await.unwrap();
    assert!(!pictures.is_empty());
}

#[tokio::test]
async fn should_get_manga_stats() {
    thread::sleep(Duration::from_secs(3));
    let stats = JIKAN.find_manga(1)
        .await.unwrap()
        .get_stats()
        .await.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_get_manga_forum() {
    thread::sleep(Duration::from_secs(3));
    let topics = JIKAN.find_manga(1)
        .await.unwrap()
        .get_forum()
        .await.unwrap();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn should_get_more_manga_info() {
    thread::sleep(Duration::from_secs(3));
    let more_info = JIKAN.find_manga(2)
        .await.unwrap()
        .get_more_info()
        .await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_get_manga_reviews() {
    thread::sleep(Duration::from_secs(3));
    let reviews = JIKAN.find_manga(1)
        .await.unwrap()
        .get_reviews(&1)
        .await.unwrap();
    assert!(reviews.len() > 0 && reviews.len() <= 20);
}

#[tokio::test]
async fn should_get_manga_recommendations() {
    thread::sleep(Duration::from_secs(3));
    let recommendations = JIKAN.find_manga(1)
        .await.unwrap()
        .get_recommendations()
        .await.unwrap();
    assert!(!recommendations.is_empty());
}

#[tokio::test]
async fn should_get_manga_user_updates() {
    thread::sleep(Duration::from_secs(3));
    let user_updates = JIKAN.find_manga(1)
        .await.unwrap()
        .get_user_updates(&1)
        .await.unwrap();
    assert!(!user_updates.is_empty());
}
