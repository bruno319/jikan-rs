use crate::{anime, character, forum, manga, more_info, news, person, pictures, recommendations, reviews, schedule, search, season, stats, user_updates};
use crate::anime::{Anime, characters::CharactersStaff, episodes::Episode, videos::Videos};
use crate::base::{MALRoleItem, SourceType};
use crate::character::Character;
use crate::forum::Topic;
use crate::manga::Manga;
use crate::news::News;
use crate::person::Person;
use crate::pictures::Picture;
use crate::recommendations::Recommendation;
use crate::reviews::Reviews;
use crate::schedule::{Schedule, ScheduleOn};
use crate::search::{results::SearchResultEnum, SearchQuery};
use crate::season::{Season, SeasonResult};
use crate::season::archive::ArchivedSeason;
use crate::stats::Stats;
use crate::user_updates::UserUpdates;

pub const BASE_URL: &str = "http://api.jikan.moe/v3";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct JikanClient {
    http_client: reqwest::Client,
}

impl JikanClient {
    pub fn new() -> JikanClient {
        JikanClient {
            http_client: reqwest::Client::new()
        }
    }

    pub async fn find_anime(&self, mal_id: u32) -> Result<Anime> {
        anime::find_anime(mal_id, &self.http_client).await
    }

    pub async fn find_manga(&self, mal_id: u32) -> Result<Manga> {
        manga::find_manga(mal_id, &self.http_client).await
    }

    pub async fn find_person(&self, mal_id: u32) -> Result<Person> {
        person::find_person(mal_id, &self.http_client).await
    }

    pub async fn find_character(&self, mal_id: u32) -> Result<Character> {
        character::find_character(mal_id, &self.http_client).await
    }

    pub async fn find_anime_characters(&self, mal_id: u32) -> Result<CharactersStaff> {
        anime::characters::find_characters(&mal_id, &self.http_client).await
    }

    pub async fn find_manga_characters(&self, mal_id: u32) -> Result<Vec<MALRoleItem>> {
        manga::characters::find_characters(&mal_id, &self.http_client).await
    }

    pub async fn find_episodes(&self, mal_id: u32) -> Result<Vec<Episode>> {
        anime::episodes::find_anime_episodes(&mal_id, &self.http_client).await
    }

    pub async fn find_news(&self, mal_id: SourceType) -> Result<Vec<News>> {
        news::find_news(mal_id, &self.http_client).await
    }

    pub async fn find_pictures(&self, mal_id: SourceType) -> Result<Vec<Picture>> {
        pictures::find_pictures(mal_id, &self.http_client).await
    }

    pub async fn find_videos(&self, mal_id: u32) -> Result<Videos> {
        anime::videos::find_videos(&mal_id, &self.http_client).await
    }

    pub async fn find_stats(&self, mal_id: SourceType) -> Result<Stats> {
        stats::find_stats(mal_id, &self.http_client).await
    }

    pub async fn find_forum(&self, mal_id: SourceType) -> Result<Vec<Topic>> {
        forum::find_forum(mal_id, &self.http_client).await
    }

    pub async fn find_more_info(&self, mal_id: SourceType) -> Result<Option<String>> {
        more_info::find_more_info(mal_id, &self.http_client).await
    }

    pub async fn find_reviews(&self, mal_id: SourceType, page: &u16) -> Result<Reviews> {
        reviews::find_reviews(mal_id, page, &self.http_client).await
    }

    pub async fn find_recommendations(&self, mal_id: SourceType) -> Result<Vec<Recommendation>> {
        recommendations::find_recommendations(mal_id, &self.http_client).await
    }

    pub async fn find_user_updates(&self, mal_id: SourceType, page: &u16) -> Result<UserUpdates> {
        user_updates::find_user_updates(mal_id, page, &self.http_client).await
    }

    pub async fn find_season(&self, season: Season) -> Result<SeasonResult> {
        season::find_season(season, &self.http_client).await
    }

    pub async fn find_season_archives(&self) -> Result<Vec<ArchivedSeason>> {
        season::archive::find_season_archives(&self.http_client).await
    }

    pub async fn find_schedule(&self, schedule_on: ScheduleOn) -> Result<Schedule> {
        schedule::find_schedule(schedule_on, &self.http_client).await
    }

    pub async fn search(&self, query: SearchQuery) -> Result<SearchResultEnum> {
        search::search(query, &self.http_client).await
    }
}