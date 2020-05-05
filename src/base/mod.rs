use std::fmt;

pub enum SourceType {
    Anime(u32),
    Manga(u32),
    Person(u32),
    Character(u32)
}

impl SourceType {
    pub fn get_uri(&self) -> String {
        match self {
            SourceType::Anime(id) => format!("/anime/{}", id),
            SourceType::Manga(id) => format!("/manga/{}", id),
            SourceType::Person(id) => format!("/person/{}", id),
            SourceType::Character(id) => format!("/character/{}", id),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MALTypeItem {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub content_type: String,
}

#[derive(Deserialize, Debug)]
pub struct MALImageItem {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub image_url: String,
}

#[derive(Deserialize, Debug)]
pub struct MALRoleItem {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct VoiceActor {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub language: String,
}

#[derive(Deserialize, Debug)]
pub enum AnimeStatusForUser {
    Watching,
    Completed,
    #[serde(rename = "On-Hold")]
    OnHold,
    Dropped,
    #[serde(rename = "Plan to Watch")]
    PlanToWatch,
}

#[derive(Deserialize, Debug)]
pub enum MangaStatusForUser {
    Reading,
    Completed,
    #[serde(rename = "On-Hold")]
    OnHold,
    Dropped,
    #[serde(rename = "Plan to Read")]
    PlanToRead,
}

#[derive(Deserialize, Debug)]
pub struct RelatedContent {
    #[serde(rename = "Alternative Version", default = "default_content")]
    pub alternative_versions: Vec<MALTypeItem>,
    #[serde(rename = "Alternative Setting", default = "default_content")]
    pub alternative_settings: Vec<MALTypeItem>,
    #[serde(rename = "Adaptation", default = "default_content")]
    pub adaptations: Vec<MALTypeItem>,
    #[serde(rename = "Character", default = "default_content")]
    pub characters: Vec<MALTypeItem>,
    #[serde(rename = "Full story", default = "default_content")]
    pub full_stories: Vec<MALTypeItem>,
    #[serde(rename = "Parent story", default = "default_content")]
    pub parent_stories: Vec<MALTypeItem>,
    #[serde(rename = "Prequel", default = "default_content")]
    pub prequels: Vec<MALTypeItem>,
    #[serde(rename = "Sequel", default = "default_content")]
    pub sequels: Vec<MALTypeItem>,
    #[serde(rename = "Other", default = "default_content")]
    pub others: Vec<MALTypeItem>,
    #[serde(rename = "Side story", default = "default_content")]
    pub side_stories: Vec<MALTypeItem>,
    #[serde(rename = "Spin-off", default = "default_content")]
    pub spin_offs: Vec<MALTypeItem>,
    #[serde(rename = "Summary", default = "default_content")]
    pub summaries: Vec<MALTypeItem>,
}

fn default_content() -> Vec<MALTypeItem> {
    Vec::with_capacity(0)
}

#[derive(Deserialize, Debug)]
pub struct AnimeInfo {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub image_url: Option<String>,
    pub synopsis: String,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub airing_start: Option<String>,
    pub episodes: Option<u16>,
    pub members: Option<u32>,
    pub genres: Vec<MALTypeItem>,
    pub source: String,
    pub producers: Vec<MALTypeItem>,
    pub score: Option<f32>,
    pub licensors: Vec<String>,
    pub r18: bool,
    pub kids: bool,
    #[serde(default)]
    pub continuing: bool,
}

pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date {
            year,
            month,
            day
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}