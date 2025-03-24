use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    sync::Arc,
    time::Duration,
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use rand::Rng;
use reqwest::{
    cookie::Jar,
    header::{self, HeaderMap, HeaderValue},
    multipart, Client, Url,
};
use serde_json::{json, Value};
use tokio::{net::TcpStream, time};
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};

use crate::{
    bot::BotInfo,
    bundles::PoeBundle,
    chat::Chat,
    constants::{
        default_headers, subscriptions_mutation, BASE_URL, BOT_NICKNAME, DEFAULT_CATEGORY_NAME,
    },
    history::ChatHistory,
    message::MessageContext,
    models::{
        api_settings::{ApiSettings, MySettings},
        message::{Message, MessageEdgeCreate},
        on_message::MessageQueue,
        query::QueryHash,
        user::UserInfo,
        SearchData, SendMessageData, Token,
    },
    queries::{RequestData, RequestPath},
    search::SearchResult,
    utils::{generate_file, generate_nonce},
};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type MessageQueueData = HashMap<i64, VecDeque<MessageQueue>>;

pub struct PoeApi {
    jar: Arc<Jar>,
    client: Client,
    default_headers: HeaderMap<HeaderValue>,
    bundle: PoeBundle,

    // data
    pub message_queues: MessageQueueData,

    // WebSocket Data
    pub stream_writer: Option<SplitSink<WsStream, tungstenite::Message>>,
    pub stream_reader: Option<SplitStream<WsStream>>,
}

impl Debug for PoeApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PoeApi {:p}>", self)
    }
}

impl PoeApi {
    pub async fn new(token: Token<'_>) -> anyhow::Result<Self> {
        let mut default_headers = default_headers();
        if let Some(formkey) = token.formkey.as_ref() {
            default_headers.insert("Poe-Formkey", HeaderValue::from_str(formkey)?);
        }

        let url = Url::parse(BASE_URL)?;

        let jar = Arc::new(Jar::default());
        jar.add_cookie_str(&format!("p-b={}", token.p_b), &url);
        jar.add_cookie_str(&format!("p-lat={}", token.p_lat), &url);

        let bundle = PoeBundle::new(&token)?;

        let mut api = Self {
            jar,
            client: Client::new(),
            bundle,
            default_headers,

            // data
            message_queues: HashMap::new(),

            // websocket
            stream_writer: None,
            stream_reader: None,
        };
        api.update_client()?;
        Ok(api)
    }

    fn update_client(&mut self) -> anyhow::Result<()> {
        self.client = Client::builder()
            .default_headers(self.default_headers.clone())
            .cookie_provider(self.jar.clone())
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(())
    }

    /*
     * +-----------+
     * | WebSocket |
     * +-----------+
     */

    pub async fn connect_websocket(&mut self) -> anyhow::Result<()> {
        let response = self
            .client
            .get(format!("{}/api/settings", BASE_URL))
            .send()
            .await?;
        let data = response.json::<ApiSettings>().await?.tchannel_data;

        let mut rng = rand::rng();
        let random_number: u32 = rng.random_range(1..=1_000_000);
        let ws_domain = format!("tch{}", random_number);
        let ws_domain = ws_domain.get(..11).unwrap_or(&ws_domain);

        self.default_headers
            .insert("Poe-Tchannel", HeaderValue::from_str(&data.channel)?);
        self.update_client()?;

        let channel_url = format!(
            "ws://{}.tch.{}/up/{}/updates?min_seq={}&channel={}&hash={}",
            ws_domain, data.base_host, data.box_name, data.min_seq, data.channel, data.channel_hash
        );

        // subscribe to channel
        let data = RequestData {
            query_name: QueryHash::SubscriptionsMutation,
            data: subscriptions_mutation(),
            ..Default::default()
        };
        let response = self.send_request(data).await?;
        if response.get("data").is_none() || response.get("errors").is_some() {
            anyhow::bail!(
                "Failed to subscribe by sending SubscriptionsMutation. Raw response data: {}",
                serde_json::to_string(&response)?
            );
        }

        let (ws_stream, _) = tokio_tungstenite::connect_async(channel_url).await?;
        let (writer, reader) = ws_stream.split();
        self.stream_writer = Some(writer);
        self.stream_reader = Some(reader);
        Ok(())
    }

    /*
     * +---------------+
     * | MAIN FUNCTION |
     * +---------------+
     */

    pub async fn send_request(&mut self, request_data: RequestData) -> anyhow::Result<Value> {
        // TODO: Handle rate limit

        let formkey = if let Some(formkey) = self.default_headers.get("Poe-Formkey") {
            formkey.to_str()?.to_owned()
        } else {
            self.bundle.get_form_key().await?
        };

        let mut rng = rand::rng();
        if request_data.ratelimit > 0 {
            log::warn!(
                "Waiting queue {}/2 to avoid rate limit",
                request_data.ratelimit
            );
            time::sleep(Duration::from_secs(rng.random_range(2..=3))).await;
        }
        let payload = serde_json::to_string(&request_data.generate_payload())?;
        let mut base_string = payload.clone();
        base_string.push_str(&formkey);
        base_string.push_str("4LxgHM6KpFqokX0Ox");

        self.default_headers
            .entry("Poe-Formkey")
            .or_insert(HeaderValue::from_str(&formkey)?);
        self.update_client()?;

        let tag = format!("{:x}", md5::compute(base_string));

        let mut request = self
            .client
            .post(format!("{}/api/{}", BASE_URL, request_data.path));
        let mut headers = HeaderMap::new();
        if !request_data.files.is_empty() {
            let mut form = multipart::Form::new().text("queryInfo", payload);
            if request_data.knowledge {
                let file = request_data.files.first().unwrap();
                let part = multipart::Part::bytes(file.data.clone())
                    .mime_str(&file.mime_type)?
                    .file_name(file.name.clone());
                form = form.part("file", part);
            } else {
                for (index, file) in request_data.files.into_iter().enumerate() {
                    let part = multipart::Part::bytes(file.data)
                        .mime_str(&file.mime_type)?
                        .file_name(file.name.clone());
                    form = form.part(format!("file{}", index + 1), part);
                }
            }
            request = request.multipart(form)
        } else {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            request = request.body(payload)
        }
        headers.insert("poe-tag-id", HeaderValue::from_str(&tag)?);
        request = request.headers(headers);

        let response = request.send().await?;
        let data = response.json::<Value>().await?;

        let is_success = data
            .get("success")
            .map_or(false, |v| v.as_bool().unwrap_or(false));
        if !is_success || data.get("data").is_none() {
            if let Some(err) = data
                .get("errors")
                .and_then(|e| e.as_array())
                .and_then(|e| e[0].get("message"))
            {
                let msg = Value::as_str(err).unwrap_or("");
                if msg == "Server Error" {
                    anyhow::bail!(
                        "Server Error. Raw response data: {:?}",
                        serde_json::to_string(&data)?
                    );
                } else {
                    anyhow::bail!("{}", msg);
                }
            }
        }

        Ok(data)
    }

    /*
     * +----------------------------+
     * | Chat Setup & Customization |
     * +----------------------------+
     */

    pub async fn set_default_bot(&mut self, bot_id: i64) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::SettingsDefaultBotSectionMutation,
                data: json!({
                    "botId": bot_id
                }),
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|d| d.get("poeSetDefaultBot"))
            .and_then(|p| p.get("status"))
            .and_then(|v| v.as_str())
            .map(|v| v == "success")
            .unwrap_or(false);
        Ok(is_success)
    }

    pub async fn set_default_message_point_limit(&mut self, limit: usize) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::SettingsDefaultMessagePointLimitModal_SetAllChatDefaultMessagePointPriceThreshold_Mutation,
                data: json!({
                    "priceThresholdInPoints": limit.to_string()
                }),
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|v| v.get("setAllChatDefaultMessagePointPriceThreshold"))
            .map(|v| v.is_object())
            .unwrap_or(false);
        Ok(is_success)
    }

    /*
     * +-------------------------+
     * | Conversation Management |
     * +-------------------------+
     */

    pub fn chat_history(&mut self) -> ChatHistory {
        ChatHistory::new(self)
    }

    pub async fn import_chat(&mut self, chat_code: &str) -> anyhow::Result<Option<Chat>> {
        // get bot name
        let url = format!("{}/s/{}", BASE_URL, chat_code);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;

        if let Some(cap) = BOT_NICKNAME.captures(&html) {
            if let Some(botname) = cap.get(1).map(|v| v.as_str()) {
                let response = self
                    .send_request(RequestData {
                        query_name:
                            QueryHash::ContinueChatCTAButton_continueChatFromPoeShare_Mutation,
                        data: json!({
                            "shareCode": chat_code,
                            "botName": botname,
                            "postId": null
                        }),
                        ..Default::default()
                    })
                    .await?;
                if let Some(data) = response
                    .get("data")
                    .and_then(|d| d.get("continueChatFromPoeShare"))
                {
                    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
                    if let Some(status_message) = data.get("statusMessage").and_then(|v| v.as_str())
                    {
                        if !status_message.is_empty() {
                            anyhow::bail!(format!("{}: {}", status, status_message));
                        }
                    }
                    return Ok(self.chat_history().next().await);
                }
            }
        }
        anyhow::bail!("failed to get bot name from poe webpage.")
    }

    pub async fn set_chat_title(&mut self, chat_id: i64, new_title: &str) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::ChatSettingsModal_ChatSetTitle_Mutation,
                data: json!({
                    "chatId": chat_id,
                    "title": new_title
                }),
                ..Default::default()
            })
            .await?;
        if let Some(data) = response.get("data").and_then(|d| d.get("chatSetTitle")) {
            let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(status_message) = data.get("statusMessage").and_then(|v| v.as_str()) {
                if !status_message.is_empty() {
                    anyhow::bail!(format!("{}: {}", status, status_message));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn set_chat_context_optimization(
        &mut self,
        chat_id: i64,
        enabled: bool,
    ) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::ChatSettingsModal_ChatSetContextOptimization_Mutation,
                data: json!({
                    "chatId": chat_id,
                    "isContextOptimizationOn": enabled
                }),
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|d| d.get("chatSetContextOptimization"))
            .is_some();
        Ok(is_success)
    }

    pub async fn delete_chat(&mut self, chat_id: i64) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::useDeleteChat_deleteChat_Mutation,
                data: json!({
                    "chatId": chat_id
                }),
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|d| d.get("deleteChat"))
            .is_some();
        Ok(is_success)
    }

    pub async fn purge_all_conversations(&mut self) -> bool {
        self.send_request(RequestData {
            query_name:
                QueryHash::SettingsDeleteAllMessagesButton_deleteUserMessagesMutation_Mutation,
            ..Default::default()
        })
        .await
        .is_ok()
    }

    pub async fn clear_chat_context(&mut self, chat_id: i64) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::sendChatBreakMutation,
                data: json!({
                    "chatId": chat_id,
                    "clientNonce": generate_nonce(16)
                }),
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|d| d.get("messageBreakEdgeCreate"))
            .is_some();
        Ok(is_success)
    }

    /*
     * +------------------+
     * | Message Handling |
     * +------------------+
     */

    pub async fn send_message(
        &mut self,
        payload: SendMessageData<'_>,
    ) -> anyhow::Result<MessageContext> {
        let bot = if payload.bot_handle.is_empty() {
            let my_settings = self.settings().await?;
            my_settings.default_bot.display_name
        } else {
            payload.bot_handle.to_string()
        };

        if let Some(bot_info) = self.get_bot_info(&bot).await? {
            let files = generate_file(&payload.files).await?;
            let total_size = files.iter().map(|f| f.data.len()).sum::<usize>();
            if total_size > 350000000 {
                anyhow::bail!("File size too large. Please try again with a smaller file.");
            }
            let attachments = (1..=files.len())
                .map(|i| format!("file{}", i))
                .collect::<Vec<_>>();

            let api_path = if payload.files.is_empty() {
                RequestPath::GqlPost
            } else {
                RequestPath::GqlUploadPost
            };

            self.connect_websocket().await?;
            let mut data = json!({
                    "chatId": null,
                    "bot": bot,
                    "query": payload.message,
                    "shouldFetchChat": true,
                    "source": {
                        "sourceType": "chat_input",
                        "chatInputMetadata": {
                            "useVoiceRecord": false,
                        },
                    },
                    "clientNonce": generate_nonce(16),
                    "sdid": "",
                    "attachments": attachments,
                    "existingMessageAttachmentsIds": [],
                    "messagePointsDisplayPrice": bot_info.display_message_point_price,
            });

            if let Some(chat_id_val) = payload.chat_id {
                if let Some(chat_id) = data.get_mut("chatId") {
                    *chat_id = json!(chat_id_val);
                }
            }

            let response = self
                .send_request(RequestData {
                    path: api_path.clone(),
                    query_name: QueryHash::SendMessageMutation,
                    data,
                    files: files.clone(),
                    ..Default::default()
                })
                .await?;

            if response.get("data").is_none() && response.get("errors").is_some() {
                anyhow::bail!(
                    "Bot {} not found. Make sure the bot exists before creating new chat.",
                    bot
                )
            }
            if let Some(data) = response
                .get("data")
                .and_then(|v| v.get("messageEdgeCreate"))
            {
                let message_data = serde_json::from_value::<MessageEdgeCreate>(data.clone())?;

                if message_data.status == "success" {
                    for file in files.iter() {
                        log::info!("File '{}' uploaded successfully", file.name);
                    }

                    if let (Some(chat), Some(user_message), Some(bot_message)) = (
                        message_data.chat,
                        message_data.message,
                        message_data.bot_message,
                    ) {
                        return Ok(MessageContext::new(self, chat, user_message, bot_message));
                    }
                }
                if !message_data.status_message.is_empty() {
                    anyhow::bail!("{}: {}", message_data.status, message_data.status_message)
                }
            }
        }

        anyhow::bail!(
            "Failed to get bot info for {}. Make sure the bot exists before creating new chat.",
            bot
        )
    }

    pub async fn retry_message(&mut self, chat_code: &str) -> anyhow::Result<MessageContext> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::ChatPageQuery,
                data: json!({
                   "chatCode": chat_code
                }),
                ..Default::default()
            })
            .await?;

        if let Some(data) = response.get("data").and_then(|c| c.get("chatOfCode")) {
            let msg_price = data
                .get("defaultBotObject")
                .and_then(|b| b.get("messagePointLimit"))
                .and_then(|m| m.get("displayMessagePointPrice"))
                .and_then(|v| v.as_i64());
            let edges_map = data
                .get("messagesConnection")
                .and_then(|m| m.get("edges"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|value| {
                            if let Some(node) = value.get("node") {
                                if let Ok(message) = serde_json::from_value::<Message>(node.clone())
                                {
                                    return Some(message);
                                }
                            }
                            None
                        })
                        .collect::<Vec<_>>()
                });

            if let (Some(display_message_point_price), Some(messages)) = (msg_price, edges_map) {
                let [user_message, bot_message] = &messages[messages.len() - 2..] else {
                    panic!("Vector does not have the expected number of elements");
                };

                let chat = serde_json::from_value::<Chat>(data.clone())?;

                self.connect_websocket().await?;
                let response = self
                    .send_request(RequestData {
                        query_name: QueryHash::regenerateMessageMutation,
                        data: json!({
                            "messageId": bot_message.message_id,
                            "messagePointsDisplayPrice": display_message_point_price
                        }),
                        ..Default::default()
                    })
                    .await?;

                if let Some(data) = response
                    .get("data")
                    .and_then(|v| v.get("messageRegenerate"))
                {
                    if let (Some(status), Some(status_message)) = (
                        data.get("status").and_then(|v| v.as_str()),
                        data.get("statusMessage").and_then(|v| v.as_str()),
                    ) {
                        if !status_message.is_empty() {
                            anyhow::bail!("{}: {}", status, status_message)
                        }
                        return Ok(MessageContext::new(
                            self,
                            chat,
                            user_message.clone(),
                            bot_message.clone(),
                        ));
                    }
                }
            }
        }

        anyhow::bail!("Failed to retry message of Thread {chat_code}")
    }

    pub async fn cancel_message(&mut self, chat_id: i64) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::cancelViewerActiveJobs_cancelViewerActiveJobs_Mutation,
                data: json!({
                    "chatId": chat_id
                }),
                ..Default::default()
            })
            .await?;

        let is_success = response
            .get("data")
            .and_then(|d| d.get("cancelViewerActiveJobs"))
            .is_some();
        Ok(is_success)
    }

    pub async fn delete_messages(
        &mut self,
        chat_code: &str,
        message_ids: &[i64],
    ) -> anyhow::Result<bool> {
        let connections = format!(
            "client:{}:__ChatMessagesView_chat_messagesConnection_connection",
            chat_code
        );
        let data = json!({
            "connections": connections,
            "messageIds": message_ids
        });

        let response = self
            .send_request(RequestData {
                query_name:
                    QueryHash::MessageDeleteConfirmationModal_deleteMessageMutation_Mutation,
                data,
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|d| d.get("messagesDelete"))
            .is_some();
        Ok(is_success)
    }

    pub async fn get_total_cost_points(&mut self, message_code: &str) -> anyhow::Result<i64> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::MessageInfoPageQuery,
                data: json!({
                    "messageCode": message_code
                }),
                ..Default::default()
            })
            .await?;
        let point = response
            .get("data")
            .and_then(|d| d.get("messageOfCode"))
            .and_then(|d| d.get("responsibleJob"))
            .and_then(|j| j.get("totalCostPoints"))
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        Ok(point)
    }

    pub async fn get_message_share_url(
        &mut self,
        chat_id: i64,
        message_ids: &[i64],
    ) -> anyhow::Result<String> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::messageSharing_shareMessagesMutation_Mutation,
                data: json!({
                    "chatId": chat_id,
                    "messageIds": message_ids
                }),
                ..Default::default()
            })
            .await?;
        if let Some(share_code) = response
            .get("data")
            .and_then(|d| d.get("messagesShare"))
            .and_then(|m| m.get("shareCode"))
            .and_then(|c| c.as_str())
        {
            let url = format!("{}/s/{}", BASE_URL, share_code);
            return Ok(url);
        }
        anyhow::bail!("An error occurred while sharing the messages")
    }

    pub async fn get_list_preview_app(&mut self, message_id: i64) -> anyhow::Result<Vec<String>> {
        let mut share_urls = vec![];
        loop {
            let response = self
                .send_request(RequestData {
                    query_name: QueryHash::useSharePreviewFromMessage_Mutation,
                    data: json!({
                        "index": share_urls.len(),
                        "messageId": message_id
                    }),
                    ..Default::default()
                })
                .await?;

            if let Some(share_url) = response
                .get("data")
                .and_then(|d| d.get("sharePreviewFromMessage"))
                .and_then(|s| s.get("sharedPreview"))
                .and_then(|s| s.get("shareUrl"))
                .and_then(|v| v.as_str())
            {
                share_urls.push(share_url.to_string());
                continue;
            }
            break;
        }
        Ok(share_urls)
    }

    /*
     * +-----------------------+
     * | User & Bot Management |
     * +-----------------------+
     */

    pub async fn explore<'a>(
        &'a mut self,
        search_data: SearchData<'a>,
    ) -> anyhow::Result<SearchResult<'a>> {
        if search_data.category_name != DEFAULT_CATEGORY_NAME {
            let available_categories = self.get_available_categories().await?;
            if !available_categories.contains(&search_data.category_name.to_string()) {
                anyhow::bail!(
                    "Category {} not found. Make sure the category exists before exploring.",
                    search_data.category_name
                );
            }
        }

        Ok(SearchResult::new(self, search_data))
    }

    pub async fn get_available_categories(&mut self) -> anyhow::Result<Vec<String>> {
        let mut categories = vec![];
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::ExploreBotsIndexPageQuery,
                data: json!({
                    "categoryName": DEFAULT_CATEGORY_NAME
                }),
                ..Default::default()
            })
            .await?;
        if let Some(category_names) = response
            .get("data")
            .and_then(|d| d.get("exploreBotsCategoryObjects"))
            .and_then(|v| v.as_array())
        {
            for category in category_names {
                if let Some(name) = category.get("categoryName").and_then(|v| v.as_str()) {
                    categories.push(name.to_string())
                }
            }
        }

        Ok(categories)
    }

    pub async fn get_bot_info(&mut self, bot_handle: &str) -> anyhow::Result<Option<BotInfo>> {
        let data = RequestData {
            query_name: QueryHash::HandleBotLandingPageQuery,
            data: json!({"botHandle": bot_handle}),
            ..Default::default()
        };

        let response = self.send_request(data).await?;
        if let Some(data) = response.get("data").and_then(|v| v.get("bot")) {
            if !data.is_null() {
                let bot = serde_json::from_value::<BotInfo>(data.clone())?;
                return Ok(Some(bot));
            }
        }

        Ok(None)
    }

    pub async fn get_user_info(&mut self, user_handle: &str) -> anyhow::Result<Option<UserInfo>> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::HandleProfilePageQuery,
                data: json!({
                    "handle": user_handle
                }),
                ..Default::default()
            })
            .await?;
        if let Some(data) = response.get("data").and_then(|v| v.get("user")) {
            if !data.is_null() {
                let user = serde_json::from_value::<UserInfo>(data.clone())?;
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    async fn set_follow_user_state(
        &mut self,
        user_id: i64,
        should_follow: bool,
    ) -> anyhow::Result<bool> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::UserFollowStateButton_poeUserSetFollow_Mutation,
                data: json!({
                    "shouldFollow": should_follow,
                    "targetUid": user_id
                }),
                ..Default::default()
            })
            .await?;
        let is_success = response
            .get("data")
            .and_then(|d| d.get("poeUserSetFollow"))
            .and_then(|p| p.get("status"))
            .and_then(|v| v.as_str())
            .map(|v| v == "success")
            .unwrap_or(false);
        Ok(is_success)
    }

    pub async fn follow_user(&mut self, user_id: i64) -> anyhow::Result<bool> {
        self.set_follow_user_state(user_id, true).await
    }

    pub async fn unfollow_user(&mut self, user_id: i64) -> anyhow::Result<bool> {
        self.set_follow_user_state(user_id, false).await
    }

    /*
     * +------+
     * | Misc |
     * +------+
     */

    pub async fn settings(&mut self) -> anyhow::Result<MySettings> {
        let response = self
            .send_request(RequestData {
                query_name: QueryHash::settingsPageQuery,
                ..Default::default()
            })
            .await?;
        if let Some(data) = response.get("data").and_then(|d| d.get("viewer")) {
            let settings = serde_json::from_value::<MySettings>(data.clone())?;
            return Ok(settings);
        }
        anyhow::bail!("Failed to fetch settings")
    }
}
