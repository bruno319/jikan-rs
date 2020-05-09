extern crate jikan_rs;

use jikan_rs::base::SourceType;
use jikan_rs::client::Jikan;
use jikan_rs::reviews::Reviews;
use jikan_rs::schedule::ScheduleOn;
use jikan_rs::search::enums::{AnimeGenre, MangaGenre};
use jikan_rs::season::Season;
use jikan_rs::stats::Stats;
use jikan_rs::top::{Top, TopAnimeSubtype, TopMangaSubtype, TopResult};
use jikan_rs::user_updates::UserUpdates;
use jikan_rs::meta::{InfoAbout, Period};
use std::thread;
use std::time::Duration;

#[tokio::test]
async fn should_find_an_anime() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let anime = jikan.find_anime(1).await.unwrap();
    assert_eq!(anime.mal_id, 1);
    assert_eq!(anime.title, "Cowboy Bebop");
}

#[tokio::test]
async fn should_find_a_manga() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let manga = jikan.find_manga(1).await.unwrap();
    assert_eq!(manga.mal_id, 1);
    assert_eq!(manga.title, "Monster");
}

#[tokio::test]
async fn should_find_a_person() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let person = jikan.find_person(1).await.unwrap();
    assert_eq!(person.mal_id, 1);
    assert_eq!(person.name, "Tomokazu Seki");
}

#[tokio::test]
async fn should_find_a_character() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let character = jikan.find_character(1).await.unwrap();
    assert_eq!(character.mal_id, 1);
    assert_eq!(character.name, "Spike Spiegel");
}

#[tokio::test]
async fn should_find_anime_characters() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let characters_staff = jikan.find_anime_characters(1).await.unwrap();
    assert!(characters_staff.characters.len() > 0);
    assert!(characters_staff.staff.len() > 0);
}

#[tokio::test]
async fn should_find_manga_characters() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let characters = jikan.find_manga_characters(1).await.unwrap();
    assert!(characters.len() > 0);
}

#[tokio::test]
async fn should_find_anime_episodes_info() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let episodes = jikan.find_episodes(1).await.unwrap();
    assert_eq!(episodes.len(), 26);
}

#[tokio::test]
async fn should_find_anime_news() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let news = jikan.find_news(SourceType::Anime(1)).await.unwrap();
    assert!(news.len() > 0);
}

#[tokio::test]
async fn should_find_manga_news() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let news = jikan.find_news(SourceType::Manga(1)).await.unwrap();
    assert!(news.len() > 0);
}

#[tokio::test]
async fn should_find_anime_pictures() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let pictures = jikan.find_pictures(SourceType::Anime(1)).await.unwrap();
    assert!(pictures.len() > 0);
}

#[tokio::test]
async fn should_find_manga_pictures() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let pictures = jikan.find_pictures(SourceType::Manga(1)).await.unwrap();
    assert!(pictures.len() > 0);
}

#[tokio::test]
async fn should_find_anime_videos() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let videos = jikan.find_videos(1).await.unwrap();
    assert!(videos.promo.len() > 0);
    assert!(videos.episodes.len() > 0);
}

#[tokio::test]
async fn should_find_anime_stats() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let stats = jikan.find_stats(SourceType::Anime(1)).await.unwrap();
    let stats = match stats {
        Stats::Anime(stats) => Some(stats),
        _ => None
    }.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_find_manga_stats() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let stats = jikan.find_stats(SourceType::Manga(1)).await.unwrap();
    let stats = match stats {
        Stats::Manga(stats) => Some(stats),
        _ => None
    }.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_find_anime_forum() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let topics = jikan.find_forum(SourceType::Anime(1)).await.unwrap();
    assert!(topics.len() > 0);
}

#[tokio::test]
async fn should_find_manga_forum() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let topics = jikan.find_forum(SourceType::Manga(1)).await.unwrap();
    assert!(topics.len() > 0);
}

#[tokio::test]
async fn should_find_more_anime_info() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let more_info = jikan.find_more_info(SourceType::Anime(1)).await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_find_more_manga_info() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let more_info = jikan.find_more_info(SourceType::Manga(2)).await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_find_anime_reviews() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let reviews = jikan.find_reviews(SourceType::Anime(1), &1).await.unwrap();
    let reviews = match reviews {
        Reviews::Anime(u) => Some(u),
        _ => None,
    }.unwrap();
    assert!(reviews.len() > 0 && reviews.len() <= 20);
}

#[tokio::test]
async fn should_find_manga_reviews() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let reviews = jikan.find_reviews(SourceType::Manga(1), &1).await.unwrap();
    let reviews = match reviews {
        Reviews::Manga(u) => Some(u),
        _ => None,
    }.unwrap();
    assert!(reviews.len() > 0 && reviews.len() <= 20);
}

#[tokio::test]
async fn should_find_anime_recommendations() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let recommendations = jikan.find_recommendations(SourceType::Anime(1)).await.unwrap();
    assert!(recommendations.len() > 0);
}

#[tokio::test]
async fn should_find_manga_recommendations() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let recommendations = jikan.find_recommendations(SourceType::Manga(1)).await.unwrap();
    assert!(recommendations.len() > 0);
}

#[tokio::test]
async fn should_find_anime_user_updates() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let user_updates = jikan.find_user_updates(SourceType::Anime(1), &1).await.unwrap();
    let user_updates = match user_updates {
        UserUpdates::Anime(u) => Some(u),
        _ => None,
    }.unwrap();
    assert!(user_updates.len() > 0);
}

#[tokio::test]
async fn should_find_manga_user_updates() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let user_updates = jikan.find_user_updates(SourceType::Manga(1), &1).await.unwrap();
    let user_updates = match user_updates {
        UserUpdates::Manga(u) => Some(u),
        _ => None,
    }.unwrap();
    assert!(user_updates.len() > 0);
}

#[tokio::test]
async fn should_find_a_season() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let season = jikan.find_season(Season::Winter(2020)).await.unwrap();
    assert!(season.animes.len() > 0);
}


#[tokio::test]
async fn should_find_animes_with_undefined_season() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let season = jikan.find_season(Season::Later).await.unwrap();
    assert!(season.animes.len() > 0);
}

#[tokio::test]
async fn should_find_season_archives() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let archives = jikan.find_season_archives().await.unwrap();
    assert!(archives.len() > 0);
}

#[tokio::test]
async fn should_find_schedule() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let schedule = jikan.find_schedule(ScheduleOn::All).await.unwrap();
    assert!(schedule.monday.len() > 0);
    assert!(schedule.tuesday.len() > 0);
    assert!(schedule.wednesday.len() > 0);
    assert!(schedule.thursday.len() > 0);
    assert!(schedule.friday.len() > 0);
    assert!(schedule.saturday.len() > 0);
    assert!(schedule.sunday.len() > 0);
    assert!(schedule.other.len() > 0);
    assert!(schedule.unknown.len() > 0);
}

#[tokio::test]
async fn should_find_schedule_for_specific_day() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let schedule = jikan.find_schedule(ScheduleOn::Sunday).await.unwrap();
    assert!(schedule.sunday.len() > 0);
    assert!(schedule.monday.is_empty());
    assert!(schedule.tuesday.is_empty());
    assert!(schedule.wednesday.is_empty());
    assert!(schedule.thursday.is_empty());
    assert!(schedule.friday.is_empty());
    assert!(schedule.saturday.is_empty());
    assert!(schedule.other.is_empty());
    assert!(schedule.unknown.is_empty());
}

#[tokio::test]
async fn should_find_top_anime() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let top_anime = jikan.find_top(Top::Anime { page: 1, subtype: TopAnimeSubtype::All }).await.unwrap();
    let top_anime = match top_anime {
        TopResult::Anime(top_anime) => Some(top_anime),
        _ => None
    }.unwrap();
    assert_eq!(top_anime.len(), 50);
}

#[tokio::test]
async fn should_find_top_manga() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let top_manga = jikan.find_top(Top::Manga { page: 1, subtype: TopMangaSubtype::All }).await.unwrap();
    let top_manga = match top_manga {
        TopResult::Manga(top_manga) => Some(top_manga),
        _ => None
    }.unwrap();
    assert_eq!(top_manga.len(), 50);
}

#[tokio::test]
async fn should_find_top_characters() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let top_character = jikan.find_top(Top::Character(1)).await.unwrap();
    let top_character = match top_character {
        TopResult::Character(top_character) => Some(top_character),
        _ => None
    }.unwrap();
    assert_eq!(top_character.len(), 50);
}

#[tokio::test]
async fn should_find_top_people() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let top_people = jikan.find_top(Top::People(1)).await.unwrap();
    let top_people = match top_people {
        TopResult::People(top_people) => Some(top_people),
        _ => None
    }.unwrap();
    assert_eq!(top_people.len(), 50);
}

#[tokio::test]
async fn should_find_action_animes() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let action_animes = jikan.find_animes_with_genre(AnimeGenre::Action, &1).await.unwrap();
    assert_eq!(action_animes.mal_url.name, "Action Anime");
    assert!(action_animes.animes.len() > 0);
}

#[tokio::test]
async fn should_find_shoujo_mangas() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let shoujo_mangas = jikan.find_mangas_with_genre(MangaGenre::Shoujo, &1).await.unwrap();
    assert_eq!(shoujo_mangas.mal_url.name, "Shoujo Manga");
    assert!(shoujo_mangas.mangas.len() > 0);
}

#[tokio::test]
async fn should_find_a_producer() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let producer = jikan.find_producer(1, &1).await.unwrap();
    assert!(producer.animes.len() > 0);
}

#[tokio::test]
async fn should_find_a_magazine() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let magazine = jikan.find_magazine(1, &1).await.unwrap();
    assert!(magazine.mangas.len() > 0);
}

#[tokio::test]
async fn should_find_a_club() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let club = jikan.find_club(1).await.unwrap();
    assert_eq!(club.mal_id, 1);
}

#[tokio::test]
async fn should_find_club_members() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let members = jikan.find_club_members(1, &1).await.unwrap();
    assert!(members.len() > 0);
}

#[tokio::test]
async fn should_retrieve_api_status() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let status = jikan.retrieve_api_status().await.unwrap();
    assert!(status.requests_this_month > 0);
}

#[tokio::test]
async fn should_retrieve_info_on_this_month_about_anime_endpoint() {
    thread::sleep(Duration::from_secs(3));
    let jikan = Jikan::new();
    let endpoints = jikan.retrieve_request_info(InfoAbout::Anime, Period::Monthly, 1).await.unwrap();
    assert!(endpoints.len() > 0);
}