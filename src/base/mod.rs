
#[derive(Deserialize, Debug)]
pub struct MALItem {
    pub mal_id: i32,
    #[serde(rename = "type")]
    pub content_type: String,
    pub name: String,
    pub url: String,
}