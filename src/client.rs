use std::collections::HashMap;

use crate::{anime, character, club, forum, genre, magazine, manga, meta, more_info, news, person, pictures,
            producer, recommendations, reviews, schedule, search, season, stats, top, user, user_updates};
use crate::anime::{Anime, characters::CharactersStaff, episodes::EpisodeInfo, videos::Videos};
use crate::base::{MALRoleItem, SourceType};
use crate::character::Character;
use crate::club::{Club, ClubMember};
use crate::forum::Topic;
use crate::genre::{GenreAnimeResult, GenreMangaResult};
use crate::magazine::Magazine;
use crate::manga::Manga;
use crate::meta::{ApiStatus, InfoAbout, Period};
use crate::news::News;
use crate::person::Person;
use crate::pictures::Picture;
use crate::producer::Producer;
use crate::recommendations::Recommendation;
use crate::reviews::Reviews;
use crate::schedule::{Schedule, ScheduleOn};
use crate::search::{results::SearchResultEnum, SearchQueryBuilder};
use crate::search::enums::{AnimeGenre, MangaGenre};
use crate::season::{Season, SeasonResult};
use crate::season::archive::ArchivedSeason;
use crate::stats::Stats;
use crate::top::{Top, TopResult};
use crate::user::results::UserResultEnum;
use crate::user::UserInfo;
use crate::user_updates::UserUpdates;

pub const BASE_URL: &str = "http://api.jikan.moe/v3";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Default)]
pub struct Jikan {
    http_client: reqwest::Client,
}

impl Jikan {
    /// Constructs a new `Jikan` client.
    ///
    /// # Examples
    ///
    /// ```
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// ```
    pub fn new() -> Jikan {
        Jikan {
            http_client: reqwest::Client::new()
        }
    }

    /// Get the anime providing its MAL id.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop
    /// let anime = jikan.find_anime(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_anime(&self, mal_id: u32) -> Result<Anime> {
        anime::find_anime(mal_id, &self.http_client).await
    }

    /// Get the manga providing its MAL id.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Monster
    /// let manga = jikan.find_manga(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_manga(&self, mal_id: u32) -> Result<Manga> {
        manga::find_manga(mal_id, &self.http_client).await
    }

    /// Get the person info providing its MAL id.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Tomokazu Seki
    /// let person = jikan.find_person(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_person(&self, mal_id: u32) -> Result<Person> {
        person::find_person(mal_id, &self.http_client).await
    }

    /// Get the character providing its MAL id.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Spike Spiegel
    /// let character = jikan.find_character(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_character(&self, mal_id: u32) -> Result<Character> {
        character::find_character(mal_id, &self.http_client).await
    }

    /// Get all characters of the anime.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns all characters from Cowboy Bebop
    /// let characters = jikan.find_anime_characters(1)
    ///     .await
    ///     .unwrap();
    /// }
    /// ```
    pub async fn find_anime_characters(&self, mal_id: u32) -> Result<CharactersStaff> {
        anime::characters::find_characters(mal_id, &self.http_client).await
    }

    /// Get all characters of the manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns all characters from Monster
    /// let characters = jikan.find_manga_characters(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_manga_characters(&self, mal_id: u32) -> Result<Vec<MALRoleItem>> {
        manga::characters::find_characters(mal_id, &self.http_client).await
    }

    /// Get information about the anime episodes.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns the list of episodes
    /// let episodes = jikan.find_episodes(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_episodes(&self, mal_id: u32) -> Result<Vec<EpisodeInfo>> {
        anime::episodes::find_anime_episodes(mal_id, &self.http_client).await
    }

    /// Get news of the anime/manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::SourceType;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop news
    /// let news = jikan.find_news(SourceType::Anime(1))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_news(&self, mal_id: SourceType) -> Result<Vec<News>> {
        news::find_news(mal_id, &self.http_client).await
    }

    /// Get picture links of the anime/manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::SourceType;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Monster pictures
    /// let pictures = jikan.find_pictures(SourceType::Manga(1))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_pictures(&self, mal_id: SourceType) -> Result<Vec<Picture>> {
        pictures::find_pictures(mal_id, &self.http_client).await
    }

    /// Get links of promotional videos and episodes of the anime.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop videos
    /// let videos = jikan.find_videos(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_videos(&self, mal_id: u32) -> Result<Videos> {
        anime::videos::find_videos(mal_id, &self.http_client).await
    }

    /// Get statistical information of the anime/manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::{SourceType, Stats};
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop stats
    /// let stats = jikan.find_stats(SourceType::Anime(1))
    ///     .await
    ///     .unwrap();
    ///
    /// if let Stats::Anime(anime_stats) = stats {
    ///     let stats = anime_stats;
    /// }
    /// # }
    /// ```
    pub async fn find_stats(&self, mal_id: SourceType) -> Result<Stats> {
        stats::find_stats(mal_id, &self.http_client).await
    }

    /// Get forum topics of the anime/manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::SourceType;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Monster forum topics
    /// let topics = jikan.find_forum(SourceType::Manga(1))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_forum(&self, mal_id: SourceType) -> Result<Vec<Topic>> {
        forum::find_forum(mal_id, &self.http_client).await
    }

    /// Get more information of the anime/manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::SourceType;
    ///
    /// let jikan = Jikan::new();
    /// // Returns more information about Cowboy Bebop
    /// let info = jikan.find_more_info(SourceType::Anime(1))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_more_info(&self, mal_id: SourceType) -> Result<Option<String>> {
        more_info::find_more_info(mal_id, &self.http_client).await
    }

    /// Get reviews written by users about the anime/manga.
    ///
    /// Only 20 items are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::{SourceType, Reviews};
    ///
    /// let jikan = Jikan::new();
    /// // Returns reviews about Cowboy Bebop
    /// let reviews = jikan.find_reviews(SourceType::Anime(1), 1)
    ///     .await
    ///     .unwrap();
    ///
    /// if let Reviews::Anime(anime_reviews) = reviews {
    ///     let reviews = anime_reviews;
    /// }
    /// # }
    /// ```
    pub async fn find_reviews(&self, mal_id: SourceType, page: u16) -> Result<Reviews> {
        reviews::find_reviews(mal_id, page, &self.http_client).await
    }

    /// Get recommendations made by users for the anime/manga.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::SourceType;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop recommendations
    /// let recommendations = jikan.find_recommendations(SourceType::Anime(1))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_recommendations(&self, mal_id: SourceType) -> Result<Vec<Recommendation>> {
        recommendations::find_recommendations(mal_id, &self.http_client).await
    }

    /// Get latest list updates made by users of the anime/manga.
    ///
    /// Only 75 items are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::{SourceType, UserUpdates};
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop latest updates
    /// let last_updates = jikan.find_user_updates(SourceType::Anime(1), 1)
    ///     .await
    ///     .unwrap();
    ///
    /// if let UserUpdates::Anime(anime_updates) = last_updates {
    ///     let last_updates = anime_updates;
    /// }
    /// # }
    /// ```
    pub async fn find_user_updates(&self, mal_id: SourceType, page: u16) -> Result<UserUpdates> {
        user_updates::find_user_updates(mal_id, page, &self.http_client).await
    }

    /// Get animes from specified season.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::Season;
    ///
    /// let jikan = Jikan::new();
    /// // Returns animes from spring season of the 2020
    /// let season = jikan.find_season(Season::Spring(2020))
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_season(&self, season: Season) -> Result<SeasonResult> {
        season::find_season(season, &self.http_client).await
    }


    /// Get all the years and their respective seasons available.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// let archived_seasons = jikan.find_season_archives()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_season_archives(&self) -> Result<Vec<ArchivedSeason>> {
        season::archive::find_season_archives(&self.http_client).await
    }

    /// Get anime schedule of the week or specified day.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::ScheduleOn;
    ///
    /// let jikan = Jikan::new();
    /// // Returns schedule for the week
    /// let animes_of_the_week = jikan.find_schedule(ScheduleOn::Week)
    ///     .await
    ///     .unwrap();
    /// // Returns schedule for Sunday
    /// let animes_on_sunday = jikan.find_schedule(ScheduleOn::Sunday)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_schedule(&self, schedule_on: ScheduleOn) -> Result<Schedule> {
        schedule::find_schedule(schedule_on, &self.http_client).await
    }

    /// Get top for the specified item.
    ///
    /// Only 50 items are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::{Top, TopAnimeSubtype, TopResult};
    ///
    /// let jikan = Jikan::new();
    /// // Returns the first 50 ranked animes
    /// let top = jikan.find_top(Top::Anime {subtype: TopAnimeSubtype::All, page: 1})
    ///     .await
    ///     .unwrap();
    ///
    /// if let TopResult::Anime(top_anime) = top {
    ///     let top = top_anime;
    /// }
    /// # }
    /// ```
    pub async fn find_top(&self, top: Top) -> Result<TopResult> {
        top::find_top(top, &self.http_client).await
    }

    /// Get animes of the specified genre.
    ///
    /// Only 100 animes are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::AnimeGenre;
    ///
    /// let jikan = Jikan::new();
    /// let adventure_animes = jikan.find_animes_with_genre(AnimeGenre::Adventure, 1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_animes_with_genre(&self, genre: AnimeGenre, page: u16) -> Result<GenreAnimeResult> {
        genre::find_animes_with_genre(genre, page, &self.http_client).await
    }

    /// Get mangas of the specified genre.
    ///
    /// Only 100 mangas are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::MangaGenre;
    ///
    /// let jikan = Jikan::new();
    /// let shoujo_mangas = jikan.find_mangas_with_genre(MangaGenre::Shoujo, 1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_mangas_with_genre(&self, genre: MangaGenre, page: u16) -> Result<GenreMangaResult> {
        genre::find_mangas_with_genre(genre, page, &self.http_client).await
    }

    /// Get the animes of the producer.
    ///
    /// Only 100 animes are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns animes from Studio Pierrot
    /// let producer = jikan.find_producer(1, 1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_producer(&self, id: u32, page: u16) -> Result<Producer> {
        producer::find_producer(id, page, &self.http_client).await
    }

    /// Get the mangas of the magazine.
    ///
    /// Only 100 mangas are shown per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns mangas from Big Comic Original
    /// let magazine = jikan.find_magazine(1, 1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_magazine(&self, id: u32, page: u16) -> Result<Magazine> {
        magazine::find_magazine(id, page, &self.http_client).await
    }

    /// Get the club providing its MAL id.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop club
    /// let club = jikan.find_club(1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_club(&self, mal_id: u32) -> Result<Club> {
        club::find_club(mal_id, &self.http_client).await
    }

    /// Get the club members providing its MAL id.
    ///
    /// Only 35 items are shown per page for members.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// // Returns Cowboy Bebop club members
    /// let club_members = jikan.find_club_members(1, 1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn find_club_members(&self, club_id: u32, page: u32) -> Result<Vec<ClubMember>> {
        club::find_club_members(club_id, page, &self.http_client).await
    }

    /// Get user related data.
    ///
    /// Anime & Manga Lists are paginated. Only 300 items are returned per page.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::{UserInfo, UserResultEnum, AnimeListQuery};
    ///
    /// let jikan = Jikan::new();
    /// // Returns information about user profile
    /// let user_result = jikan.find_user("Bruno319", UserInfo::Profile)
    ///     .await
    ///     .unwrap();
    ///
    /// if let UserResultEnum::Profile(profile) = user_result {
    ///     let user_result = profile;
    /// }
    ///
    /// // Returns the user animelist
    /// let user_result = jikan.find_user("Bruno319", UserInfo::Animelist {query: AnimeListQuery::new()})
    ///     .await
    ///     .unwrap();
    ///
    /// if let UserResultEnum::AnimeList(anime_list) = user_result {
    ///     let user_result = anime_list;
    /// }
    /// # }
    /// ```
    pub async fn find_user(&self, username: &str, user_info: UserInfo) -> Result<UserResultEnum> {
        user::find_user(username, user_info, &self.http_client).await
    }

    /// Search results for the query.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::*;
    ///
    /// let jikan = Jikan::new();
    /// // Create a query to search for all animes in progress
    /// let query = SearchQueryBuilder::new(SearchSource::Anime)
    ///     .page(1)
    ///     .status(SourceStatus::Anime(AnimeStatus::Airing));
    ///
    /// // Search for the animes
    /// let search_result = jikan.search(query)
    ///     .await
    ///     .unwrap();
    ///
    /// if let SearchResultEnum::Anime(anime_result) = search_result {
    ///     let search_result = anime_result;
    /// }
    /// # }
    /// ```
    pub async fn search(&self, query_builder: SearchQueryBuilder) -> Result<SearchResultEnum> {
        search::search(query_builder, &self.http_client).await
    }

    /// Retrieve status on the Jikan REST API.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    ///
    /// let jikan = Jikan::new();
    /// let api_status = jikan.retrieve_api_status()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn retrieve_api_status(&self) -> Result<ApiStatus> {
        meta::retrieve_api_status(&self.http_client).await
    }

    /// Retrieve most requested endpoints for a specific period.
    ///
    /// 1,000 requests are shown per page, you can use the offset to show more.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn run() {
    /// use jikan_rs::client::Jikan;
    /// use jikan_rs::prelude::{InfoAbout, Period};
    ///
    /// let jikan = Jikan::new();
    /// // Retrieve the most requested endpoints on anime domain today
    /// let api_status = jikan.retrieve_request_info(InfoAbout::Anime, Period::Today, 1)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn retrieve_request_info(&self, about: InfoAbout, period: Period, offset: u32) -> Result<HashMap<String, u16>> {
        meta::retrieve_request_info(about, period, offset, &self.http_client).await
    }
}