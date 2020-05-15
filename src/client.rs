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
use crate::user_updates::UserUpdates;
use crate::user::UserInfo;
use crate::user::results::UserResultEnum;

pub const BASE_URL: &str = "http://api.jikan.moe/v3";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Jikan {
    http_client: reqwest::Client,
}

impl Jikan {
    pub fn new() -> Jikan {
        Jikan {
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

    pub async fn find_episodes(&self, mal_id: u32) -> Result<Vec<EpisodeInfo>> {
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

    pub async fn find_top(&self, top: Top) -> Result<TopResult> {
        top::find_top(top, &self.http_client).await
    }

    pub async fn find_animes_with_genre(&self, genre: AnimeGenre, page: &u16) -> Result<GenreAnimeResult> {
        genre::find_animes_with_genre(genre, page, &self.http_client).await
    }

    pub async fn find_mangas_with_genre(&self, genre: MangaGenre, page: &u16) -> Result<GenreMangaResult> {
        genre::find_mangas_with_genre(genre, page, &self.http_client).await
    }

    pub async fn find_producer(&self, id: u32, page: &u16) -> Result<Producer> {
        producer::find_producer(id, page, &self.http_client).await
    }

    pub async fn find_magazine(&self, id: u32, page: &u16) -> Result<Magazine> {
        magazine::find_magazine(id, page, &self.http_client).await
    }

    pub async fn find_club(&self, mal_id: u32) -> Result<Club> {
        club::find_club(mal_id, &self.http_client).await
    }

    pub async fn find_club_members(&self, club_id: u32, page: &u32) -> Result<Vec<ClubMember>> {
        club::find_club_members(club_id, page, &self.http_client).await
    }

    pub async fn find_user(&self, username: &str, user_info: UserInfo) -> Result<UserResultEnum> {
        user::find_user(username, user_info, &self.http_client).await
    }

    pub async fn search(&self, query_builder: SearchQueryBuilder) -> Result<SearchResultEnum> {
        search::search(query_builder, &self.http_client).await
    }

    pub async fn retrieve_api_status(&self) -> Result<ApiStatus> {
        meta::retrieve_api_status(&self.http_client).await
    }

    pub async fn retrieve_request_info(&self, about: InfoAbout, period: Period, offset: u32) -> Result<HashMap<String, u16>> {
        meta::retrieve_request_info(about, period, offset, &self.http_client).await
    }
}