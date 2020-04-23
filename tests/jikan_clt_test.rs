extern crate jikan_rs;

use jikan_rs::client::JikanClient;
use jikan_rs::base::TypeSource;
use jikan_rs::stats::Stats;
use jikan_rs::user_updates::UserUpdates;

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

#[tokio::test]
async fn should_find_anime_videos() {
    let jikancl = JikanClient::new();
    let videos = jikancl.find_videos(1).await.unwrap();
    assert!(videos.promo.len() > 0);
    assert!(videos.episodes.len() > 0);
}

#[tokio::test]
async fn should_find_anime_stats() {
    let jikancl = JikanClient::new();
    let stats = jikancl.find_stats(TypeSource::Anime(1)).await.unwrap();
    let stats = match stats {
        Stats::Anime(stats) => Some(stats),
        _ => None
    }.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_find_manga_stats() {
    let jikancl = JikanClient::new();
    let stats = jikancl.find_stats(TypeSource::Manga(1)).await.unwrap();
    let stats = match stats {
        Stats::Manga(stats) => Some(stats),
        _ => None
    }.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_find_anime_forum() {
    let jikancl = JikanClient::new();
    let topics = jikancl.find_forum(TypeSource::Anime(1)).await.unwrap();
    assert!(topics.len() > 0);
}

#[tokio::test]
async fn should_find_manga_forum() {
    let jikancl = JikanClient::new();
    let topics = jikancl.find_forum(TypeSource::Manga(1)).await.unwrap();
    assert!(topics.len() > 0);
}

#[tokio::test]
async fn should_find_more_anime_info() {
    let jikancl = JikanClient::new();
    let more_info = jikancl.find_more_info(TypeSource::Anime(1)).await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_find_more_manga_info() {
    let jikancl = JikanClient::new();
    let more_info = jikancl.find_more_info(TypeSource::Manga(2)).await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_find_anime_reviews() {
    let jikancl = JikanClient::new();
    let reviews = jikancl.find_anime_reviews(1, &1).await.unwrap();
    assert!(reviews.len() > 0 && reviews.len() <= 20);
}

#[tokio::test]
async fn should_find_anime_recommendations() {
    let jikancl = JikanClient::new();
    let recommendations = jikancl.find_recommendations(TypeSource::Anime(1)).await.unwrap();
    assert!(recommendations.len() > 0);
}

#[tokio::test]
async fn should_find_manga_recommendations() {
    let jikancl = JikanClient::new();
    let recommendations = jikancl.find_recommendations(TypeSource::Manga(1)).await.unwrap();
    assert!(recommendations.len() > 0);
}

#[tokio::test]
async fn should_find_anime_user_updates() {
    let jikancl = JikanClient::new();
    let user_updates = jikancl.find_user_updates(TypeSource::Anime(1), &1).await.unwrap();
    let user_updates = match user_updates {
        UserUpdates::Anime(u) => Some(u),
        _ => None,
    }.unwrap();
    assert!(user_updates.len() > 0);
}

#[tokio::test]
async fn should_find_manga_user_updates() {
    let jikancl = JikanClient::new();
    let user_updates = jikancl.find_user_updates(TypeSource::Manga(1), &1).await.unwrap();
    let user_updates = match user_updates {
        UserUpdates::Manga(u) => Some(u),
        _ => None,
    }.unwrap();
    assert!(user_updates.len() > 0);
}

