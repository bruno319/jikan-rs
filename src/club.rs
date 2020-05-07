use reqwest::Client;

use crate::base::MALTypeItem;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_club(mal_id: u32, http_clt: &Client) -> Result<Club> {
    let url = format!("{}/club/{}", BASE_URL, mal_id);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let club: Club = serde_json::from_str(&body)?;

    Ok(club)
}

pub(crate) async fn find_club_members(club_id: u32, page: &u32, http_clt: &Client) -> Result<Vec<ClubMember>> {
    let url = format!("{}/club/{}/members/{}", BASE_URL, club_id, page);
    let body = http_clt.get(&url).send()
        .await?
        .text()
        .await?;
    let response: ClubMembersResponse = serde_json::from_str(&body)?;

    Ok(response.members)
}

#[derive(Deserialize, Debug)]
pub struct Club {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub mal_id: u32,
    pub url: String,
    pub image_url: Option<String>,
    pub title: String,
    pub members_count: u32,
    pub pictures_count: u32,
    pub category: Option<String>,
    pub created: Option<String>,
    #[serde(rename = "type")]
    pub club_type: Option<String>,
    pub staff: Vec<MALTypeItem>,
    pub anime_relations: Vec<MALTypeItem>,
    pub manga_relations: Vec<MALTypeItem>,
    pub character_relations: Vec<MALTypeItem>,
}

#[derive(Deserialize, Debug)]
pub struct ClubMembersResponse {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    members: Vec<ClubMember>,
}

#[derive(Deserialize, Debug)]
pub struct ClubMember {
    pub username: String,
    pub url: String,
    pub image_url: String,
}