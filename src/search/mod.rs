use percent_encoding::utf8_percent_encode;
use reqwest::Client;

use crate::base::{Date, FRAGMENT};
use crate::base::Resource;
use crate::client::BASE_URL;
use crate::search::enums::{Genres, OrderBy, Rating, Sort, SearchSource, SourceStatus, SearchSourceType};
use crate::search::results::SearchResultEnum;

pub mod enums;
pub mod results;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn search(query_builder: SearchQueryBuilder, http_clt: &Client) -> Result<SearchResultEnum> {
    let query = query_builder.build()?;
    let url = format!("{}{}?{}", BASE_URL, query.source.uri(), query.query);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let search_result = match query.source {
        SearchSource::Anime => SearchResultEnum::Anime(serde_json::from_str(&body)?),
        SearchSource::Manga => SearchResultEnum::Manga(serde_json::from_str(&body)?),
        SearchSource::Person => SearchResultEnum::Person(serde_json::from_str(&body)?),
        SearchSource::Character => SearchResultEnum::Character(serde_json::from_str(&body)?),
    };
    Ok(search_result)
}

pub struct SearchQuery {
    source: SearchSource,
    query: String,
}

pub struct SearchQueryBuilder {
    source: SearchSource,
    page: u16,
    name: Option<String>,
    source_type: Option<SearchSourceType>,
    status: Option<SourceStatus>,
    rating: Option<Rating>,
    order_by: Option<OrderBy>,
    sort: Option<Sort>,
    genre: Option<Genres>,
    score: Option<u8>,
    genre_exclude: bool,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<u8>,
    producer: Option<u32>,
    magazine: Option<u32>,
    letter: Option<char>,
}

impl SearchQueryBuilder {
    pub fn new(source: SearchSource) -> SearchQueryBuilder {
        SearchQueryBuilder {
            source,
            page: 1,
            name: None,
            source_type: None,
            status: None,
            rating: None,
            order_by: None,
            sort: None,
            genre: None,
            score: None,
            genre_exclude: false,
            start_date: None,
            end_date: None,
            limit: None,
            producer: None,
            magazine: None,
            letter: None,
        }
    }

    pub fn page(mut self, page: u16) -> SearchQueryBuilder {
        self.page = page;
        self
    }

    pub fn name(mut self, name: &str) -> SearchQueryBuilder {
        self.name = Some(utf8_percent_encode(name, FRAGMENT).to_string());
        self
    }

    pub fn type_source(mut self, source_type: SearchSourceType) -> SearchQueryBuilder {
        self.source_type = Some(source_type);
        self
    }

    pub fn status(mut self, status: SourceStatus) -> SearchQueryBuilder {
        self.status = Some(status);
        self
    }

    pub fn rating(mut self, rating: Rating) -> SearchQueryBuilder {
        self.rating = Some(rating);
        self
    }

    pub fn order_by(mut self, order_by: OrderBy, sort: Sort) -> SearchQueryBuilder {
        self.order_by = Some(order_by);
        self.sort = Some(sort);
        self
    }

    pub fn genre(mut self, genre: Genres) -> SearchQueryBuilder {
        self.genre = Some(genre);
        self
    }

    pub fn genre_exclude(mut self, genre_exclude: bool) -> SearchQueryBuilder {
        self.genre_exclude = genre_exclude;
        self
    }

    pub fn score(mut self, score: u8) -> SearchQueryBuilder {
        self.score = Some(score);
        self
    }

    pub fn start_date(mut self, date: Date) -> SearchQueryBuilder {
        self.start_date = Some(date.to_string());
        self
    }

    pub fn end_date(mut self, date: Date) -> SearchQueryBuilder {
        self.end_date = Some(date.to_string());
        self
    }

    pub fn limit(mut self, limit: u8) -> SearchQueryBuilder {
        self.limit = Some(limit);
        self
    }

    pub fn producer(mut self, mal_id: u32) -> SearchQueryBuilder {
        self.producer = Some(mal_id);
        self
    }

    pub fn magazine(mut self, mal_id: u32) -> SearchQueryBuilder {
        self.magazine = Some(mal_id);
        self
    }

    pub fn first_letter(mut self, letter: char) -> SearchQueryBuilder {
        self.letter = Some(letter);
        self
    }

    pub fn build(self) -> Result<SearchQuery> {
        let mut query = String::from("");
        query = format!("{}page={}", query, self.page);

        if let Some(name) = self.name {
            query = format!("{}&q={}", query, name);
        }

        if let Some(source_type) = self.source_type {
            match source_type {
                SearchSourceType::Anime(anime_type) => query = format!("{}&status={}", query, anime_type.uri()),
                SearchSourceType::Manga(manga_type) => query = format!("{}&status={}", query, manga_type.uri()),
            }
        }

        if let Some(status) = self.status {
            match status {
                SourceStatus::Anime(anime_status) => query = format!("{}&status={}", query, anime_status.uri()),
                SourceStatus::Manga(manga_status) => query = format!("{}&status={}", query, manga_status.uri()),
            }
        }

        if let Some(rating) = self.rating {
            query = format!("{}&{}", query, rating.uri());
        }

        if let Some(order) = self.order_by {
            query = format!("{}&{}&{}", query, order.uri(), self.sort.unwrap().uri());
        }

        if let Some(score) = self.score {
            query = format!("{}&score={}", query, score);
        }

        if let Some(date) = self.start_date {
            query = format!("{}&start_date={}", query, date);
        }

        if let Some(date) = self.end_date {
            query = format!("{}&end_date={}", query, date);
        }

        if let Some(limit) = self.limit {
            query = format!("{}&limit={}", query, limit);
        }

        if let Some(mal_id) = self.producer {
            query = format!("{}&producer={}", query, mal_id);
        }

        if let Some(mal_id) = self.magazine {
            query = format!("{}&magazine={}", query, mal_id);
        }

        if let Some(letter) = self.letter {
            query = format!("{}&letter={}", query, letter);
        }

        if self.genre_exclude {
            query = format!("{}&genre_exclude=1", query);
        }

        if let Some(genre) = self.genre {
            query = format!("{}&genre=", query);
            match genre {
                Genres::Anime(genres) => {
                    let genres = genres.iter()
                        .map(|genre| (*genre as u8).to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    query = format!("{}{}", query, genres);
                }
                Genres::Manga(genres) => {
                    let genres = genres.iter()
                        .map(|genre| (*genre as u8).to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    query = format!("{}{}", query, genres);
                }
            }
        }

        Ok(
            SearchQuery {
                source: self.source,
                query,
            }
        )
    }
}