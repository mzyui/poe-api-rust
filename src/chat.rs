use serde::{Deserialize, Serialize};

use crate::{api::PoeApi, history::ChatHistory, message::MessageContext, models::SendMessageData};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: String,
    pub chat_id: i64,
    pub chat_code: String,
    pub title: Option<String>,
}

pub struct ChatContext<'a> {
    api: &'a mut PoeApi,
    pub inner: Chat,
}

impl<'a> ChatContext<'a> {
    pub fn new(api: &'a mut PoeApi, inner: Chat) -> Self {
        Self { api, inner }
    }

    pub async fn send_message(
        &mut self,
        mut payload: SendMessageData<'_>,
    ) -> anyhow::Result<MessageContext> {
        payload.chat_id = Some(self.inner.chat_id);
        self.api.send_message(payload).await
    }

    pub async fn clear_context(&mut self) -> anyhow::Result<bool> {
        self.api.clear_chat_context(self.inner.chat_id).await
    }

    pub async fn set_title(&mut self, new_title: &str) -> anyhow::Result<bool> {
        if self
            .api
            .set_chat_title(self.inner.chat_id, new_title)
            .await?
        {
            self.inner.title = Some(new_title.to_string());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn set_context_optimization(&mut self, value: bool) -> anyhow::Result<bool> {
        self.api
            .set_chat_context_optimization(self.inner.chat_id, value)
            .await
    }

    pub async fn delete(&mut self) -> anyhow::Result<bool> {
        self.api.delete_chat(self.inner.chat_id).await
    }
}
