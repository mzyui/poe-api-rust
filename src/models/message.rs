use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::chat::Chat;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageEdgeCreate {
    #[serde(deserialize_with = "deserialize_message")]
    pub message: Option<Message>,
    pub status: String,
    pub status_message: String,
    pub chat: Option<Chat>,
    #[serde(deserialize_with = "deserialize_message")]
    pub bot_message: Option<Message>,
}

fn deserialize_message<'de, D>(deserializer: D) -> anyhow::Result<Option<Message>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: Value = Deserialize::deserialize(deserializer)?;
    if let Some(node) = data.get("node") {
        let message: Message = serde_json::from_value(node.clone()).map_err(D::Error::custom)?;
        return Ok(Some(message));
    }
    Ok(None)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: i64,
    pub creation_time: i64,
    pub id: String,
    pub author_user: Option<User>,
    pub text: String,
    pub state: String,
    pub client_nonce: Option<String>,
    pub author: String,
    pub content_type: String,
    pub source_type: String,
    pub message_state_text: Option<String>,
    #[serde(default)]
    pub message_code: String,
    pub text_length_on_cancellation: i64,
    pub uid: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: i64,
    pub id: String,
    pub handle: String,
    pub profile_photo_url: String,
    pub full_name: String,
}
