extern crate jikan_rs;
extern crate rand;

use jikan_rs::client::JikanClient;
use rand::Rng;
use jikan_rs::base::TypeSource;
use jikan_rs::stats::Stats;

#[tokio::main]
async fn main() {
    let id: u32 = rand::thread_rng().gen_range(1, 30000);
    println!("ID {}", id);

    let jikancl = JikanClient::new();

    let anime = jikancl.find_anime("1").await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", anime);

    // let characters = anime.get_characters().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let characters = jikancl.find_characters_from(TypeSource::Anime(anime.mal_id.to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let characters = jikancl.find_characters_from(TypeSource::Manga("1".to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let episodes = anime.get_episodes().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", episodes);
    //
    // let episodes = jikancl.find_episodes("21").await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", episodes.len());
    //
    // let news = anime.get_news().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", news);
    //
    // let news = jikancl.find_news(TypeSource::Anime("1".to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", news);
    //
    // let news = jikancl.find_news(TypeSource::Manga("1".to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", news);
    //
    // let pictures = anime.get_pictures().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", pictures);
    //
    // let pictures = jikancl.find_pictures(TypeSource::Anime("1".to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", pictures);
    //
    // let pictures = jikancl.find_pictures(TypeSource::Manga("1".to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", pictures);
    //
    // let videos = anime.get_videos().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", videos);
    //
    // let videos = jikancl.find_videos("1").await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", videos);
    //
    // let stats = anime.get_stats().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", stats);
    //
    // let stats = jikancl.find_stats(TypeSource::Anime("1".to_string())).await.unwrap();
    // let stats = match stats {
    //     Stats::Anime(stats) => Some(stats),
    //     _ => None
    // };
    // println!("Response {:#?} \n-------------------------------------------", stats.unwrap());
    //
    // let stats = jikancl.find_stats(TypeSource::Manga("1".to_string())).await.unwrap();
    // let stats = match stats {
    //     Stats::Manga(stats) => Some(stats),
    //     _ => None
    // };
    // println!("Response {:#?} \n-------------------------------------------", stats.unwrap());

    let topics = anime.get_forum().await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", topics);

    let topics = jikancl.find_forum(TypeSource::Anime("1".to_string())).await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", topics);

    let topics = jikancl.find_forum(TypeSource::Manga("1".to_string())).await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", topics);

}