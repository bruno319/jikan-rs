pub enum TypeSource {
    Anime(String),
    Manga(String),
}

impl TypeSource {
    pub(crate) fn get_uri(&self) -> String {
        match self {
            TypeSource::Anime(id) => format!("/anime/{}", id),
            TypeSource::Manga(id) => format!("/manga/{}", id),
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