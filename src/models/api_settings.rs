use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSettings {
    pub tchannel_data: TChannelData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TChannelData {
    pub min_seq: String,
    pub channel: String,
    pub channel_hash: String,
    pub box_name: String,
    pub base_host: String,
    pub target_url: String,
    pub enable_websocket: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MySettings {
    pub uid: i64,
    pub default_bot: DefaultBot,
    pub message_point_info: MessagePointInfo,
    pub primary_phone_number: Option<String>,
    pub primary_email: Option<String>,
    pub confirmed_emails: Vec<String>,
    pub has_active_subscription: bool,
    #[serde(rename = "enableGTMEventSending")]
    pub enable_gtm_event_sending: bool,
    pub viewer_country_code: String,
    pub global_context_optimization_status: bool,
    pub enable_global_context_optimization: bool,
    pub has_unread_message: bool,
}

impl MySettings {
    pub fn message_point_balance(&self) -> i64 {
        self.message_point_info.message_point_balance
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBot {
    pub display_name: String,
    pub bot_id: i64,
    pub id: String,
}

// Deserializer
fn timestamp_ms<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();
    Ok(datetime)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagePointInfo {
    #[serde(deserialize_with = "timestamp_ms")]
    pub message_point_reset_time: DateTime<Utc>,
    pub message_point_balance: i64,
    pub total_message_point_allotment: i64,
    pub all_chat_default_point_price_threshold_per_message: i64,
}
