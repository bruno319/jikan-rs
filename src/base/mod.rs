pub enum TypeSource {
    Anime(u32),
    Manga(u32),
    Person(u32),
    Character(u32)
}

impl TypeSource {
    pub(crate) fn get_uri(&self) -> String {
        match self {
            TypeSource::Anime(id) => format!("/anime/{}", id),
            TypeSource::Manga(id) => format!("/manga/{}", id),
            TypeSource::Person(id) => format!("/person/{}", id),
            TypeSource::Character(id) => format!("/character/{}", id),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MALItem {
    pub mal_id: u32,
    #[serde(rename = "type")]
    pub content_type: String,
    pub name: String,
    pub url: String,
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
    pub url: String,
    pub image_url: String,
    pub name: String,
    pub language: String,
}

#[derive(Deserialize, Debug)]
pub enum AnimeStatus {
    Watching,
    Completed,
    #[serde(rename = "On-Hold")]
    OnHold,
    Dropped,
    #[serde(rename = "Plan to Watch")]
    PlanToWatch,
}

#[derive(Deserialize, Debug)]
pub enum MangaStatus {
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
    pub alternative_versions: Vec<MALItem>,
    #[serde(rename = "Alternative Setting", default = "default_content")]
    pub alternative_settings: Vec<MALItem>,
    #[serde(rename = "Adaptation", default = "default_content")]
    pub adaptations: Vec<MALItem>,
    #[serde(rename = "Character", default = "default_content")]
    pub characters: Vec<MALItem>,
    #[serde(rename = "Full story", default = "default_content")]
    pub full_stories: Vec<MALItem>,
    #[serde(rename = "Parent story", default = "default_content")]
    pub parent_stories: Vec<MALItem>,
    #[serde(rename = "Prequel", default = "default_content")]
    pub prequels: Vec<MALItem>,
    #[serde(rename = "Sequel", default = "default_content")]
    pub sequels: Vec<MALItem>,
    #[serde(rename = "Other", default = "default_content")]
    pub others: Vec<MALItem>,
    #[serde(rename = "Side story", default = "default_content")]
    pub side_stories: Vec<MALItem>,
    #[serde(rename = "Spin-off", default = "default_content")]
    pub spin_offs: Vec<MALItem>,
    #[serde(rename = "Summary", default = "default_content")]
    pub summaries: Vec<MALItem>,
}

fn default_content() -> Vec<MALItem> {
    Vec::with_capacity(0)
}