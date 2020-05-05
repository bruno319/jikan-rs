extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::Jikan;
use jikan_rs::search::enums::{AnimeType, Genres, MangaType, Source, SourceType};
use jikan_rs::search::enums::AnimeGenre::{Adventure, Shounen};
use jikan_rs::search::results::SearchResultEnum;
use jikan_rs::search::SearchQueryBuilder;

lazy_static! {
    static ref JIKAN: Jikan = Jikan::new();
}

#[tokio::test]
async fn should_search_for_anime_one_piece() {
    let query = SearchQueryBuilder::new(Source::Anime)
        .name("one p")
        .type_source(SourceType::Anime(AnimeType::TV))
        .genre(Genres::Anime(vec![Shounen, Adventure]))
        .build()
        .expect("Fail on build search query");

    let anime = JIKAN.search(query)
        .await.unwrap();

    let anime = match anime {
        SearchResultEnum::Anime(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(anime.results[0].title, "One Piece");
}

#[tokio::test]
async fn should_search_for_manga_berserk() {
    let query = SearchQueryBuilder::new(Source::Manga)
        .name("berser")
        .type_source(SourceType::Manga(MangaType::Manga))
        .build()
        .expect("Fail on build search query");

    let manga = JIKAN.search(query)
        .await.unwrap();

    let manga = match manga {
        SearchResultEnum::Manga(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(manga.results[0].title, "Berserk");
}

#[tokio::test]
async fn should_search_for_person_masashi_kishimoto() {
    let query = SearchQueryBuilder::new(Source::Person)
        .name("岸本 斉史")
        .build()
        .expect("Fail on build search query");

    let person = JIKAN.search(query)
        .await.unwrap();

    let person = match person {
        SearchResultEnum::Person(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(person.results[0].name, "Masashi Kishimoto");
}

#[tokio::test]
async fn should_search_for_character_lelouch_lamperouge() {
    let query = SearchQueryBuilder::new(Source::Character)
        .name("ルルーシュ")
        .build()
        .expect("Fail on build search query");

    let characters = JIKAN.search(query)
        .await.unwrap();

    let characters = match characters {
        SearchResultEnum::Character(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(characters.results[0].name, "Lamperouge, Lelouch");
}