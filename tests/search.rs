extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::JikanClient;
use jikan_rs::search::enums::{AnimeType, Source, SourceType, Genre};
use jikan_rs::search::results::SearchResultEnum;
use jikan_rs::search::SearchQueryBuilder;
use jikan_rs::search::enums::AnimeGenre::{Shounen, Fantasy};

lazy_static! {
    static ref JIKAN_CL: JikanClient = JikanClient::new();
}

#[tokio::test]
async fn should_search_for_an_anime() {
    let query = SearchQueryBuilder::new(Source::Anime)
        .type_source(SourceType::Anime(AnimeType::TV))
        .page(1)
        .genre(Genre::Anime(vec![Shounen, Fantasy]))
        .first_letter('y')
        .build()
        .expect("Fail on build search query");

    let animes = JIKAN_CL.search(query)
        .await.unwrap();

    let animes = match animes {
        SearchResultEnum::Anime(result) => Some(result),
        _ => None
    }.unwrap();

    println!("{:#?}", animes);
    assert!(!animes.results.is_empty());
}