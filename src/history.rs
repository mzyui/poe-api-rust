use std::{collections::VecDeque, task::Poll};

use futures_util::{FutureExt, Stream};
use serde_json::{json, Value};

use crate::{api::PoeApi, chat::Chat, models::query::QueryHash, queries::RequestData, utils::get_json_value};

#[derive(Debug)]
pub struct ChatHistory<'a> {
    api: &'a mut PoeApi,
    cursor: Option<String>,
    results: VecDeque<Chat>,
    is_completed: bool,
}

impl Stream for ChatHistory<'_> {
    type Item = Chat;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut fut = Box::pin(self.next_item());

        loop {
            match fut.poll_unpin(cx) {
                Poll::Ready(Ok(item)) => return Poll::Ready(item),
                Poll::Ready(Err(_)) => return Poll::Ready(None),
                _ => {}
            }
        }
    }
}

impl<'a> ChatHistory<'a> {
    pub fn new(api: &'a mut PoeApi) -> Self {
        Self {
            api,
            cursor: None,
            results: VecDeque::new(),
            is_completed: false,
        }
    }

    async fn next_item(&mut self) -> anyhow::Result<Option<Chat>> {
        while !self.is_completed || !self.results.is_empty() {
            if let Some(bot_info) = self.results.pop_front() {
                return Ok(Some(bot_info));
            }

            let mut data = json!({
                "count": 10,
            });
            if let (Some(cursor), Some(object)) = (self.cursor.clone(), data.as_object_mut()) {
                object.insert("cursor".into(), Value::String(cursor));
            }

            let response = self
                .api
                .send_request(RequestData {
                    query_name: QueryHash::ChatHistoryListPaginationQuery,
                    data,
                    ..Default::default()
                })
                .await?;

            self.is_completed = true;
            if let Some(data) = get_json_value(&response, "data.chats") {
                self.cursor = get_json_value(data, "pageInfo.endCursor")
                    .and_then(|v| v.as_str())
                    .map(|v| v.to_string());
                if let Some(items) = get_json_value(data, "edges").and_then(|v| v.as_array()) {
                    for item in items {
                        if let Some(node) = get_json_value(item, "node") {
                            let chat = serde_json::from_value::<Chat>(node.clone())?;
                            self.results.push_back(chat);
                            self.is_completed = false;
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}
