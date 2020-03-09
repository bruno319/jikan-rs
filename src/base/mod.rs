
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