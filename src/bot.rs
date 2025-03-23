use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::Value;

fn get_url<'de, D>(deserializer: D) -> anyhow::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    if let Some(value) = value
        .get("url")
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
    {
        return Ok(Some(value));
    }
    Ok(None)
}

fn get_display_message_point_price<'de, D>(deserializer: D) -> anyhow::Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    value
        .get("displayMessagePointPrice")
        .and_then(|v| v.as_i64())
        .ok_or(D::Error::custom(
            "required field: 'displayMessagePointPrice'",
        ))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotInfo {
    pub id: String,
    pub bot_id: i64,
    pub handle: String,
    pub display_name: String,
    pub model: Option<String>,
    #[serde(rename = "picture", deserialize_with = "get_url")]
    pub picture_url: Option<String>,
    pub description: String,
    pub powered_by: Option<String>,
    #[serde(rename = "translatedBotTags")]
    pub tags: Vec<String>,
    #[serde(
        default,
        rename = "messagePointLimit",
        deserialize_with = "get_display_message_point_price"
    )]
    pub display_message_point_price: i64,
    pub introduction: Option<String>,
    pub is_created_by_poe_user_account: bool,
}
