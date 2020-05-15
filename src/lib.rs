#[macro_use]
extern crate jikan_resource_derive;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;
pub mod client;
mod anime;
mod manga;
mod person;
mod character;
mod search;
mod user;
mod news;
mod pictures;
mod forum;
mod more_info;
mod recommendations;
mod reviews;
mod user_updates;
mod stats;
mod base;
mod season;
mod schedule;
mod top;
mod genre;
mod producer;
mod magazine;
mod club;
mod meta;

pub mod prelude {
    pub use crate::anime::{Aired, Anime};
    pub use crate::anime::characters::{AnimeCharacter, CharactersStaff, StaffMember};
    pub use crate::anime::episodes::EpisodeInfo;
    pub use crate::anime::videos::{EpisodeVideo, PromoVideo, Videos};
    pub use crate::base::{AnimeInfo, AnimeStatusForUser, Date, MALImageItem, MALRoleItem, MALTypeItem,
                          MangaInfo, MangaStatusForUser, RelatedContent, SourceType, VoiceActor};
    pub use crate::character::Character;
    pub use crate::club::{Club, ClubMember};
    pub use crate::forum::{LastPost, Topic};
    pub use crate::genre::{GenreAnimeResult, GenreMangaResult};
    pub use crate::magazine::Magazine;
    pub use crate::manga::{Manga, Published};
    pub use crate::meta::{ApiStatus, InfoAbout, Period};
    pub use crate::news::News;
    pub use crate::person::{AnimeStaffPosition, Person, PublishedManga, VoiceActingRole};
    pub use crate::pictures::Picture;
    pub use crate::producer::Producer;
    pub use crate::recommendations::Recommendation;
    pub use crate::reviews::{AnimeReviewer, AnimeScores, MangaReviewer, MangaScores, Review, Reviews};
    pub use crate::schedule::{Schedule, ScheduleOn};
    pub use crate::search::enums::{AnimeGenre, AnimeStatus, AnimeType, Genres, MangaGenre, MangaStatus,
                                   MangaType, OrderBy, Rating, SearchSource, SearchSourceType, Sort, SourceStatus};
    pub use crate::search::results::{AnimeResult, CharacterResult, MangaResult, PersonResult, SearchResult, SearchResultEnum};
    pub use crate::search::SearchQueryBuilder;
    pub use crate::season::{archive::ArchivedSeason, Season, SeasonResult};
    pub use crate::stats::{AnimeStats, MangaStats, Score, ScoreStats, Stats};
    pub use crate::top::{RankedAnime, RankedCharacter, RankedManga, RankedPerson, Top, TopAnimeSubtype, TopMangaSubtype, TopResult};
    pub use crate::user::enums::{AnimeListQuery, HistorySource, MangaListQuery, OrderAnimeListBy, OrderMangaListBy};
    pub use crate::user::results::{AnimeListEntry, Friend, HistoryItem, MangaListEntry, Profile, UserResultEnum};
    pub use crate::user::UserInfo;
    pub use crate::user_updates::{AnimeUserUpdate, MangaUserUpdate, UserUpdates};
}