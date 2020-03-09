extern crate jikan_rs;
extern crate rand;

use jikan_rs::client::JikanClient;
use rand::Rng;
use jikan_rs::character::TypeSource;

#[tokio::main]
async fn main() {
    let id: u32 = rand::thread_rng().gen_range(1,30000);
    println!("ID {}", id);

    let jikancl = JikanClient::new();

    let anime = jikancl.find_anime("21").await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", anime);
    //
    // let characters = anime.get_characters().await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let characters = jikancl.find_characters_from(TypeSource::Anime(anime.mal_id.to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);
    //
    // let characters = jikancl.find_characters_from(TypeSource::Manga("1".to_string())).await.unwrap();
    // println!("Response {:#?} \n-------------------------------------------", characters);

    let episodes = anime.get_episodes().await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", episodes);

    let episodes = jikancl.find_episodes("21").await.unwrap();
    println!("Response {:#?} \n-------------------------------------------", episodes.len());

}