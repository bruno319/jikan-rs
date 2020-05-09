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
async fn should_get_person_pictures() {
    thread::sleep(Duration::from_secs(3));
    let pictures = JIKAN.find_person(1)
        .await.unwrap()
        .get_pictures()
        .await.unwrap();
    assert!(!pictures.is_empty());
}