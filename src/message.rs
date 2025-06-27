use std::{
    io::{stderr, Write},
    thread::sleep,
    time::Duration,
};

use crate::{
    api::PoeApi,
    chat::{Chat, ChatContext},
    models::{
        message::Message,
        on_message::{MessageQueue, MessageType, OnMessage},
    },
};

#[cfg(feature = "cli")]
use crossterm::{
    execute,
    terminal::{self, ClearType},
};
use futures_util::{FutureExt, SinkExt, Stream, StreamExt};
use tokio_tungstenite::tungstenite::Message as WsMessage;

pub struct MessageContext<'a> {
    api: &'a mut PoeApi,
    chat: Chat,
    user_message: Message,
    bot_message: Message,
    is_completed: bool,
    is_cancelled: bool,
    text: String,
    chat_title: String,
}

impl<'a> MessageContext<'a> {
    pub fn new(
        api: &'a mut PoeApi,
        chat: Chat,
        user_message: Message,
        bot_message: Message,
    ) -> Self {
        Self {
            api,
            chat,
            user_message,
            bot_message,
            is_completed: false,
            is_cancelled: false,
            text: String::new(),
            chat_title: String::new(),
        }
    }

    pub fn title(&self) -> String {
        self.chat.title.clone().unwrap_or(self.chat_title.clone())
    }

    pub async fn text(&mut self) -> String {
        if !self.is_completed && !self.is_cancelled {
            while self.next().await.is_some() {
                // do nothing
            }
        }
        self.text.clone()
    }

    pub fn chat(&mut self) -> ChatContext {
        let chat = self.chat.clone();
        ChatContext::new(self.api, chat)
    }

    pub async fn cancel(&mut self) -> anyhow::Result<bool> {
        self.api.cancel_message(self.chat.chat_id).await
    }

    pub async fn retry(&mut self) -> anyhow::Result<MessageContext> {
        self.api.retry_message(&self.chat.chat_code).await
    }

    pub async fn total_cost_points(&mut self) -> anyhow::Result<i64> {
        self.api
            .get_total_cost_points(&self.bot_message.message_code)
            .await
    }

    pub async fn share(&mut self) -> anyhow::Result<String> {
        self.api
            .get_message_share_url(
                self.chat.chat_id,
                &[self.user_message.message_id, self.bot_message.message_id],
            )
            .await
    }

    pub async fn get_list_preview_app(&mut self) -> anyhow::Result<Vec<String>> {
        self.api
            .get_list_preview_app(self.bot_message.message_id)
            .await
    }

    pub async fn delete_message_context(&mut self) -> anyhow::Result<bool> {
        self.api
            .delete_messages(
                &self.chat.chat_code,
                &[self.bot_message.message_id, self.user_message.message_id],
            )
            .await
    }

    pub async fn delete_user_message(&mut self) -> anyhow::Result<bool> {
        self.api
            .delete_messages(&self.chat.chat_code, &[self.user_message.message_id])
            .await
    }

    pub async fn delete_bot_message(&mut self) -> anyhow::Result<bool> {
        self.api
            .delete_messages(&self.chat.chat_code, &[self.bot_message.message_id])
            .await
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        log::info!("Reconnecting websocket");
        if let Some(writer) = &mut self.api.stream_writer {
            writer.close().await.unwrap_or_default();
        }
        self.api.connect_websocket().await?;
        Ok(())
    }

    async fn handle_websocket_message(&mut self, message: WsMessage) -> anyhow::Result<Option<MessageQueue>> {
        if let WsMessage::Close(_) = message {
            self.reconnect().await?;
            return Ok(None);
        }

        if let WsMessage::Text(message) = message {
            if let Ok(on_message) = serde_json::from_str::<OnMessage>(&message) {
                return self.process_on_message(on_message).await;
            }
        }
        Ok(None)
    }

    async fn process_on_message(&mut self, on_message: OnMessage) -> anyhow::Result<Option<MessageQueue>> {
        for message in on_message.messages {
            if message.payload == MessageType::RefetchChannel {
                self.reconnect().await?;
                break;
            } else if message.payload == MessageType::MessageCancelled {
                self.is_cancelled = true;
            } else if let MessageType::ChatTitleUpdated(ref title) = message.payload {
                if self.chat.chat_id == message.chat_id {
                    self.chat_title = title.text.clone();
                }
            } else if let MessageType::JobUpdated(ref job) = message.payload {
                self.is_completed = job.state.starts_with("complete")
                    && self.chat.chat_id == message.chat_id;
            }
            if self.is_completed && self.chat_title.is_empty() {
                if let Some(title) = self.chat.title.clone() {
                    self.chat_title = title;
                }
            }

            self.api
                .message_queues
                .entry(message.chat_id)
                .or_default()
                .push_back(message);
        }
        Ok(None)
    }

    async fn read_message(&mut self) -> anyhow::Result<MessageQueue> {
        while (!self.is_completed && !self.is_cancelled) || self.chat_title.is_empty() {
            // read from cache
            if let Some(message) = self
                .api
                .message_queues
                .get_mut(&self.chat.chat_id)
                .and_then(|m| m.pop_front())
            {
                return Ok(message);
            }

            if let Some(reader) = &mut self.api.stream_reader {
                if let Some(message) = reader.next().await {
                    if let Some(processed_message) = self.handle_websocket_message(message?).await? {
                        return Ok(processed_message);
                    }
                }
            }
        }
        anyhow::bail!("No more messages or stream completed unexpectedly.")
    }

    async fn next_message(&mut self) -> Option<Text> {
        if !self.is_cancelled && !self.is_completed {
            while let Ok(message) = self.read_message().await {
                if let MessageType::MessageAdded(mut m) = message.payload {
                    if !matches!(m.state.as_str(), "complete" | "completed" | "incomplete") {
                        self.is_completed = true;
                        return Some(Text::Error(format!(
                            "{}: {}\n",
                            m.state,
                            m.message_state_text?.trim()
                        )));
                    }
                    if m.state.starts_with("complete") && !m.text.ends_with("\n") {
                        m.text = m.text.trim().to_owned();
                        m.text.push('\n');
                    }
                    if m.text.contains("...") && m.text.ends_with("\n") {
                        m.text = m.text.trim().to_owned();
                    }

                    if m.text.starts_with(&self.text) {
                        let (_, chunk) = m.text.split_at(self.text.len());
                        if !chunk.is_empty() {
                            self.text = m.text.clone();
                            return Some(Text::Chunk(chunk.to_string()));
                        }
                    } else if m.text.len() > self.text.len() {
                        self.text = m.text.clone();
                        return Some(Text::Full(m.text));
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum Text {
    Chunk(String),
    Full(String),
    Error(String),
}

impl Text {
    pub fn print(&self) -> anyhow::Result<()> {
        let mut stderr = stderr();
        #[cfg(not(feature = "cli"))]
        writeln!(stderr, "{:#?}", self)?;

        #[cfg(feature = "cli")]
        match self {
            Self::Chunk(s) => {
                for ch in s.chars() {
                    write!(stderr, "{}", ch)?;
                    stderr.flush()?;
                    sleep(Duration::from_millis(2));
                }
            }
            Self::Full(s) => {
                write!(stderr, "\r")?;
                execute!(stderr, terminal::Clear(ClearType::UntilNewLine))?;
                write!(stderr, "{}", s)?;
                stderr.flush()?;
            }
            Self::Error(s) => {
                write!(stderr, "\r")?;
                execute!(stderr, terminal::Clear(ClearType::UntilNewLine))?;
                write!(stderr, "{}", s)?;
                stderr.flush()?;
            }
        }
        Ok(())
    }
}

impl Stream for MessageContext<'_> {
    type Item = Text;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut future = Box::pin(self.next_message());
        future.poll_unpin(cx)
    }
}
