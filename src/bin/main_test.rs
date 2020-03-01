extern crate jikan_rs;
extern crate rand;

use jikan_rs::client::JikanClient;
use rand::Rng;

#[tokio::main]
async fn main() {
    let id: u32 = rand::thread_rng().gen_range(1,30000);
    println!("ID {}", id);

    let jikancl = JikanClient::new();
    let anime = jikancl.find_anime_by_id(id.to_string().as_str()).await.unwrap();

    println!("Response {:?}", anime);
}