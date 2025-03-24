use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub uid: i64,
    #[serde(alias = "nullableHandle", default)]
    pub handle: String,
    pub full_name: String,
    pub follower_count: i64,
    pub medium_profile_photo_url: Option<String>,
    pub profile_photo_url: Option<String>,
}
