extern crate jikan_rs;

use jikan_rs::client::JikanClient;
use jikan_rs::base::TypeSource;
use jikan_rs::stats::Stats;
use jikan_rs::user_updates::UserUpdates;

#[tokio::main]
async fn main() {
    let jikancl = JikanClient::new();

    let anime = jikancl.find_anime(1).await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", anime);

    // let characters = anime.get_characters().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let characters = jikancl.find_characters(TypeSource::Anime(anime.mal_id.clone())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let characters = jikancl.find_characters(TypeSource::Manga(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let episodes = anime.get_episodes().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", episodes);
    //
    // let episodes = jikancl.find_episodes(21).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", episodes.len());
    //
    // let news = anime.get_news().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", news);
    //
    // let news = jikancl.find_news(TypeSource::Anime(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", news);
    //
    // let news = jikancl.find_news(TypeSource::Manga(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", news);
    //
    // let pictures = anime.get_pictures().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", pictures);
    //
    // let pictures = jikancl.find_pictures(TypeSource::Anime(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", pictures);
    //
    // let pictures = jikancl.find_pictures(TypeSource::Manga(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", pictures);
    //
    // let videos = anime.get_videos().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", videos);
    //
    // let videos = jikancl.find_videos(1).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", videos);
    //
    // let stats = anime.get_stats().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", stats);
    //
    // let stats = jikancl.find_stats(TypeSource::Anime(1)).await.unwrap();
    // let stats = match stats {
    //     Stats::Anime(stats) => Some(stats),
    //     _ => None
    // };
    // println!("Response {:#?} \n-------------------------------------------", stats.unwrap());
    //
    // let stats = jikancl.find_stats(TypeSource::Manga(1)).await.unwrap();
    // let stats = match stats {
    //     Stats::Manga(stats) => Some(stats),
    //     _ => None
    // };
    // println!("Response {:#?} \n-------------------------------------------", stats.unwrap());
    //
    // let topics = anime.get_forum().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", topics);
    //
    // let topics = jikancl.find_forum(TypeSource::Anime(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", topics);
    //
    // let topics = jikancl.find_forum(TypeSource::Manga(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", topics);
    //
    // let more_info = anime.get_more_info().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", more_info);
    //
    // let more_info = jikancl.find_more_info(TypeSource::Anime(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", more_info);
    //
    // let more_info = jikancl.find_more_info(TypeSource::Manga(2)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", more_info);
    //
    // let reviews = anime.get_reviews(&1).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", reviews);
    //
    // let reviews = jikancl.find_reviews(1, &1).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", reviews);
    //
    // let recommendations = anime.get_recommendations().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", recommendations);
    //
    // let recommendations = jikancl.find_recommendations(TypeSource::Anime(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", recommendations);
    //
    // let recommendations = jikancl.find_recommendations(TypeSource::Manga(1)).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", recommendations);
    //
    // let updates = anime.get_user_updates(&1).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", updates);
    //
    // let user_updates = jikancl.find_user_updates(TypeSource::Anime(1), &1).await.unwrap();
    // let user_updates = match user_updates {
    //     UserUpdates::Anime(u) => Some(u),
    //     _ => None
    // };
    // println!("Response {:#?} \n-------------------------------------------", user_updates.unwrap());
    //
    // let user_updates = jikancl.find_user_updates(TypeSource::Manga(1), &1).await.unwrap();
    // let user_updates = match user_updates {
    //     UserUpdates::Manga(u) => Some(u),
    //     _ => None
    // };
    // println!("Response {:#?} \n-------------------------------------------", user_updates.unwrap());
}