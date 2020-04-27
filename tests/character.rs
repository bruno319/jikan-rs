extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::JikanClient;

lazy_static! {
    static ref JIKAN_CL: JikanClient = JikanClient::new();
}

#[tokio::test]
async fn should_get_character_pictures() {
    let pictures = JIKAN_CL.find_character(1)
        .await.unwrap()
        .get_pictures()
        .await.unwrap();
    assert!(!pictures.is_empty());
}