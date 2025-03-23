use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageQueue {
    pub subscription_name: String,
    pub chat_id: i64,
    pub message_id: Option<i64>,
    pub payload: MessageType,
    pub hash: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "title")]
    pub text: String,
    pub id: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobData {
    pub id: String,
    pub job_id: i64,
    pub state: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageData {
    pub id: String,
    pub message_id: i64,
    pub creation_time: i64,
    pub state: String,
    pub message_state_text: Option<String>,
    pub text: String,
    pub author: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    #[default]
    RefetchChannel,
    MessageCancelled,
    JobUpdated(JobData),
    ChatTitleUpdated(Title),
    MessageAdded(MessageData),
    Raw(Value),
}

fn deserialize_str<'de, D>(deserializer: D) -> anyhow::Result<Vec<MessageQueue>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut messages = vec![];

    let s: Vec<Value> = Deserialize::deserialize(deserializer)?;
    for message in s.iter().filter_map(|v| v.as_str()) {
        let data = serde_json::from_str::<Value>(message).map_err(D::Error::custom)?;
        if let Some(message_type) = data.get("message_type").and_then(|v| v.as_str()) {
            if message_type == "refetchChannel" {
                messages.clear();
                messages.push(MessageQueue {
                    payload: MessageType::RefetchChannel,
                    ..Default::default()
                });
                return Ok(messages);
            }

            if let Some(payload) = data.get("payload") {
                let chat_id = payload["unique_id"]
                    .as_str()
                    .and_then(|v| v.split(":").last())
                    .map(|v| v.parse::<i64>().unwrap_or(-1))
                    .ok_or(D::Error::custom("Expected value 'unique_id', found null."))?;
                let subscription_name =
                    payload["subscription_name"]
                        .as_str()
                        .ok_or(D::Error::custom(
                            "Expected value 'subscription_name', found null.",
                        ))?;
                let payload_data = payload
                    .get("data")
                    .and_then(|v| v.get(subscription_name))
                    .ok_or(D::Error::custom(format!(
                        "Expected value 'data:{}', found null.",
                        subscription_name
                    )))?;
                // skip any message from user
                if payload_data
                    .get("author")
                    .and_then(|v| v.as_str())
                    .map(|v| v == "human")
                    .unwrap_or(false)
                {
                    continue;
                }

                let message_id = payload_data.get("messageId").and_then(|v| v.as_i64());

                let payload_string =
                    serde_json::to_string(payload_data).map_err(D::Error::custom)?;
                let hash = format!("{:x}", md5::compute(payload_string));

                let payload = if subscription_name.starts_with("chat") {
                    let data = serde_json::from_value::<Title>(payload_data.clone())
                        .map_err(D::Error::custom)?;
                    MessageType::ChatTitleUpdated(data)
                } else if subscription_name.starts_with("job") {
                    let data = serde_json::from_value::<JobData>(payload_data.clone())
                        .map_err(D::Error::custom)?;
                    MessageType::JobUpdated(data)
                } else if subscription_name == "messageAdded" {
                    let data = serde_json::from_value::<MessageData>(payload_data.clone())
                        .map_err(D::Error::custom)?;
                    MessageType::MessageAdded(data)
                } else if subscription_name == "messageCancelled" {
                    MessageType::MessageCancelled
                } else {
                    MessageType::Raw(payload_data.clone())
                };

                messages.push(MessageQueue {
                    subscription_name: subscription_name.to_string(),
                    chat_id,
                    message_id,
                    payload,
                    hash,
                });
            }
        }
    }

    Ok(messages)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnMessage {
    #[serde(deserialize_with = "deserialize_str")]
    pub messages: Vec<MessageQueue>,
    pub min_seq: i64,
}
