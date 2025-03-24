<div align="center">
<h1>Poe API Rust <img src="https://psc2.cf2.poecdn.net/favicon.svg" height="35"></h1>

<p><em>A simple, lightweight and efficient API wrapper for Poe.com</em></p>
</div>
  
## Table of Contents

- [Overview](#overview)
- [How to get your token](#how-to-get-your-token)
  - [Getting `p-b` and `p-lat` cookies (required)](#step-1-retrieve-p-b-and-p-lat-cookies-required)
  - [Getting fromkey (optional)](#step-2-retrieve-fromkey-optional)
- [Installation](#installation)
- [Connecting to the API](#connecting-to-the-api)
- [Documentation](#documentation)
  - **Message Handling**
    - [Send Message](#send-message)
    - [Retry Message](#retry-message)
    - [Cancel Message](#cancel-message)
    - [Delete Messages](#delete-messages)
    - [Get Message Share URL](#get-message-share-url)
    - [Get Total Cost Points](#get-total-cost-points)
    - [Get List Preview Apps](#get-list-preview-apps)
  - **User & Bot Management**
    - [Explore](#explore)
    - [Get Available Categories](#get-available-categories)
    - [Get Bot Info](#get-bot-info)
    - [Get User Info](#get-user-info)
    - [Follow User](#follow-user)
    - [Unfollow User](#unfollow-user)
  - **Chat Setup & Customization**
    - [Set Default Message Point Limit](#set-default-message-point-limit)
    - [Set Default Bot](#set-default-bot)
    - [Set Chat Context Optimization](#set-chat-context-optimization)
    - [Set Chat Title](#set-chat-title)
  - **Conversation Management**
    - [Purge All Conversations](#purge-all-conversations)
    - [Delete Chat](#delete-chat)
    - [Import Chat](#import-chat)
    - [Chat History](#chat-history)
    - [Clear Chat Context](#clear-chat-context)
  - **Misc**
    - [Get Settings](#settings)
- [License](#license)

---

## Overview

**Poe API Rust** is an advanced API crafted for managing chat and messaging functionalities on Poe.com. Leveraging Rust's capabilities, this project emphasizes safety, speed, and efficient concurrency. The API enables users to:

- Discover available users, bots and AI models.
- Customize settings for chat conversations.
- Send, retry, and cancel messages seamlessly.
- Access detailed information about bots and users.
- Fine-tune and optimize chat contexts for enhanced interactions.

This documentation offers a comprehensive guide to each function available within the API.

### How to Get Your Token

#### Step 1: Retrieve `p-b` and `p-lat` Cookies (Required)
1. Sign in at [poe.com](https://poe.com/).
2. Open Developer Tools:
   - **Chromium**: Press `F12` or right-click and select **Inspect**, then navigate to **Application** > **Cookies** > **poe.com**.
   - **Firefox**: Press `F12` or right-click and select **Inspect**, then go to **Storage** > **Cookies**.
   - **Safari**: Press `F12` or right-click and select **Inspect**, then access **Storage** > **Cookies**.
3. Copy the values of the `p-b` and `p-lat` cookies.

#### Step 2: Retrieve `formkey` (Optional)
> [!NOTE]
> The **poe-api-rust** automatically retrieves the `formkey` for you. If it fails, follow the steps below to obtain the token manually.

There are two methods to get the `formkey`:

1. **Method 1: Using Network Tab**
   - Open Developer Tools (`F12` or right-click and select **Inspect**).
   - Navigate to **Network** > **gql_POST** > **Headers**.
   - Copy the value of `Poe-Formkey`.

2. **Method 2: Using Console**
   - Open Developer Tools (`F12` or right-click and select **Inspect**).
   - Go to the **Console** tab.
   - Type: `allow pasting` and press Enter.
   - Paste the following script: `window.ereNdsRqhp2Rd3LEW()`.
   - Copy the resulting output.

## Installation

To manage dependencies and build your project, use Cargo. You can integrate the API functions by adding the module file to your Cargo project structure. Here’s a basic setup:

```toml
[dependencies]
poe-api = { git = "https://github.com/zevtyardt/poe-api-rust", default-features = false }
```

### Command-Line Interface

This library also offers a command-line interface. To install it directly, run:

```bash
cargo install --git "https://github.com/zevtyardt/poe-api-rust"
```

You can then execute the CLI using the `poe-cli` command.

## Connecting to the API
```rust
use poe_api::{api::PoeApi, models::Token};

let api = PoeApi::new(Token {
    p_b: "P-B", // required
    p_lat: "P-LAT", // required
    formkey: Some("fromkey"), // optional
}).await?;
```

## Documentation (Work in progress)

### Send Message
Sends a new message to a specified AI model or bot name. Supports both text and media messages.

<details>
<summary><b>Parameters</b></summary>

```rust
pub struct SendMessageData<'a> {
    pub bot_handle: &'a str,
    pub message: &'a str,
    pub chat_id: Option<i64>,
    pub files: Vec<FileInput<'a>>,
}

pub enum FileInput<'a> {
    Url(&'a str),
    Local(PathBuf),
}
```
</details>

<details>
<summary><b>Example</b></summary>

```rust
use poe_api::models::{SendMessageData, FileInput};
use futures_util::StreamExt;

// Ask simple questions using `gemini-2.0-flash` model
let mut message = api.send_message(SendMessageData {
    bot_handle: "gemini-2.0-flash",
    message: "what is the result of 2x2?",
    ..Default::default()
}).await?;

// Streamed output
while let Some(chunk) = message.next().await {
    // Process chunk output or pretty print on terminal directly
    chunk.print()?;
}

// Non-streamed output
let text = message.text().await;
```
**Another Example:** where these anime characters came from?

![Tainaka Ritsu](https://github.com/user-attachments/assets/28a2f066-9612-4f78-ba0a-3cb6b779c7b8)

```rust
// Send message to an existing chat thread
let chat_id = message.chat().inner.chat_id;
let message_data = SendMessageData {
    bot: "gemini-2.0-flash",
    message: "who is she??",
    chat_id: Some(chat_id),
    files: vec![
        FileInput::Local("my-wife.png")
    ],
};

let mut message = api.send_message(message_data).await?;
// or 
let mut message = message.chat().send_message(message_data).await?;

println!("{}", message.text().await);
```
**Output:**
```markdown
The anime character in the image is Ritsu Tainaka from the anime series K-On!. She is the self-proclaimed president of the Light Music Club and the drummer of the band Ho-kago Tea Time.

---

Related searches:
+ [anime characters in image](https://www.google.com/search?q=anime+characters+in+image&client=app-vertex-grounding-quora-poe)
+ [anime with characters Ritsu Tainaka](https://www.google.com/search?q=anime+with+characters+Ritsu+Tainaka&client=app-vertex-grounding-quora-poe)
```
</details>

---

### Retry Message
Attempt to send or recreate a message that was previously undeliverable or inappropriate.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_code` | `&str` | Chat Identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_code: &str = "sample";
let mut message = api.retry_message(chat_code).await?;

let mut message = message.retry().await?;

// Same as #send-message
```
</details>

---

### Cancel Message
Cancels a message that is in the process of being sent, useful to prevent duplicates or errors.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id`   | `i64`     | Chat identifier. |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 12345;
api.cancel_message(chat_id).await?;

message.cancel().await?;
```
</details>

---

### Delete Messages
Deletes one or more messages from a chat based on provided message IDs.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id`   | `i64`     | Chat identifier. |
| `message_ids` | `Vec<i64>`| A vector of message IDs. |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 12345;
let message_ids: Vec<i64> = vec![678910, 11121314];
api.delete_message(chat_id, message_ids).await?;

// delete user message
message.delete_user_message().await?;
// delete bot message
message.delete_bot_message().await?;
// or both messages
message.delete_message_context().await?;
```
</details>

---

### Get Message Share URL 
Generates a shareable URL for a specific message, allowing it to be shared externally.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id`   | `i64`     | Chat identifier. |
| `message_ids` | `Vec<i64>`| A vector of message IDs. |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 12345;
let message_ids: Vec<i64> = vec![678910, 11121314];
api.get_message_share_url(chat_id, message_ids).await?;

// delete user message
message.share().await?;
```
</details>

---

### Get Total Cost Points 
Calculates the total cost (in points) for a specific message, which can be used for metering or billing purposes.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `message_code`   | `&str`     | Message identifier. |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let message_code: &str = "abcdef";
api.get_total_cost_points(message_code).await?;

// or
message.total_cost_points().await?;
```
</details>

---

### Get List Preview Apps
Generates a shareable URL for a preview apps, allowing it to be shared externally.

The "Previews" feature on poe.com allows users to generate and interact with web applications directly, making it possible to create things like games, animations, and data visualizations using AI coding models.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `message_id`   | `i64`     | Message identifier. |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let message_id: i64 = 12345;
api.get_list_preview_app(message_code).await?;

// or
message.list_preview_app().await?;
```
</details>

---

### Explore
Discover available users, bots and AI models. To get a list of valid categories. see the [Get Available Categories](#get-available-categories) section.

<details>
<summary><b>Parameters</b></summary>

```rust
pub enum EntityType {
    User,
    Bot,
}

pub struct SearchData<'a> {
    pub query: Option<&'a str>,
    pub category_name: &'a str,
    pub entity_type: EntityType,
    pub count: usize,
}
```
</details>

<details>
<summary><b>Example</b></summary>

```rust
use poe_api::search::Entity;
use futures_util::StreamExt;

let search_data = SearchData {
    query: Some("deepseek"),
    entity_type: EntityType::Bot,
    ..Default::default()
};
let mut result = api.explore(search_data).await?;

while let Some(entity) = result.next().await {
    match entity {
        Entity::User(_user) => {
            // process user
        }
        Entity::Bot(_bot) => {
            // process bot
        }
    }
}
```

</details>

---

### Get Available Categories 
Fetches all available AI model categories, enabling users to quickly find the type of content they are interested in.

<details>
<summary><b>Example</b></summary>

```rust
let categories = api.get_available_categories().await?;
println!("{:?}", categories);
```
**Output:**
```markdown
["Official", "Reasoning", "Apps", "Search", "Image generation", "Audio and video", "For you", "Popular", "Funny", "Roleplay", "AI", "Utilities", "Programming", "Hobbies", "Learning", "Game Apps", "Featured", "Professional", "Creative writing", "Game Bots", "Advice", "Mind", "Translation", "Text analysis", "New"]
```
</details>

---

### Get Bot Info
Retrieves detailed information about a bot, including its configuration, current status, and capabilities.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `bot_handle`   | `&str`     | Bot handle name |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let bot_handle: &str = "Claude-3.7-Sonnet-Reasoning";
let bot_info = api.get_bot_info(bot_handle).await?;
println!("{:?}", bot_info);
```
**Output:**
```markdown
Some(BotInfo { id: "Qm90OjEwMjY=", bot_id: 1026, handle: "Claude-3.7-Sonnet-Reasoning", display_name: "Claude-3.7-Sonnet-Reasoning", model: Some("flannel_reasoning"), picture_url: Some("https://qph.cf2.poecdn.net/main-thumb-pb-1026-200-fvvsiofehkfrtswcutfmahqytzyfadsp.jpeg"), description: "Anthropic's most intelligent model (with reasoning capabilities on by default). Claude 3.7 Sonnet is a hybrid reasoning model, producing near-instant responses or extended, step-by-step thinking. Recommended for complex math or coding problems. Supports a 200k token context window.", powered_by: Some("Powered by Anthropic."), tags: ["OFFICIAL"], display_message_point_price: 123, introduction: Some(""), is_created_by_poe_user_account: false })
```
</details>

---

### Get User Info
Fetches profile details for a specific user.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `user_handle`   | `&str`     | User handle name |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let user_handle: &str = "openai";
let user_info = api.get_user_info(bot_handle).await?;
println!("{:?}", user_info);
```
**Output:**
```markdown
Some(UserInfo { id: "UG9lVXNlcjoyOTEwNDAwODc5", uid: 2910400879, handle: "openai", full_name: "OpenAI", follower_count: 2470, medium_profile_photo_url: Some("https://qph.cf2.poecdn.net/main-thumb-2910400879-100-wrfgcbmfjrhquvwlxvypitmawpovrxoi.jpeg"), profile_photo_url: Some("https://qph.cf2.poecdn.net/main-thumb-2910400879-200-wrfgcbmfjrhquvwlxvypitmawpovrxoi.jpeg") })
```
</details>

---

### Follow User
Adds the specified user to your follow list. This is useful for tracking updates and activities.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `user_id`   | `i64`     | User identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let user_id: i64 = 123456;
api.follow_user(user_id).await?;
```
</details>

---

### Unfollow User
Removes the specified user from your follow list.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `user_id`   | `i64`     | User identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let user_id: i64 = 123456;
api.unfollow_user(user_id).await?;
```
</details>

---

### Set Default Message Point Limit 
Sets the default limit on message points per conversation. This function helps enforce usage policies or manage message size constraints.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `limit`   | `usize`     | Maximum threshold points per conversation |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let limit: usize = 420;
api.set_default_message_point_limit(limit).await?;
```
</details>

---

### Set Default Bot
Assigns a default bot to the chat system for cases where no specific bot is selected.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `bot_id`   | `i64`     | Bot identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let bot_id: i64 = 420;
api.set_default_bot(bot_id).await?;
```
</details>

---

### Set Chat Context Optimization 
Enables or disables context optimization for a chat session, which can improve relevance and performance.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id` | `i64` | Chat identifier |
| `enabled` | `bool` | Flag to enable (`true`) or disable (`false`) optimization |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 420;
let enabled: bool = false;
api.set_chat_context_optimization(chat_id, enabled).await?;
```
</details>

---

### Set Chat Title 
Updates the title of an existing chat conversation.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id` | `i64` | Chat identifier |
| `new_title` | `&str` | New title for the chat |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 420;
let new_title: &str = "ayonima";
api.set_new_title(chat_id, new_title).await?;
```
</details>

---

### Purge All Conversations 
Removes all chat conversations from the system, effectively resetting the chat history.

<details>
<summary><b>Example</b></summary>

```rust
api.purge_all_conversations().await?;
```
</details>

---

### Delete Chat 
Deletes a specific chat session identified by its unique ID.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id`   | `i64`     | Chat identifier. |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 420;
api.purge_all_conversations(chat_id).await?;
```
</details>

---

### Import Chat 
Imports chat data from an external source. This can be useful for migrating or restoring conversations.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_code` | `&str` | Chat Identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_code: &str = "sample";
api.import_chat(chat_code).await?;
```
</details>

---
### Chat History 
Retrieves the history of chat conversation.

<details>
<summary><b>Example</b></summary>

```rust
while let Some(chat) = api.chat_history() {
    dbg!(chat);
}
```
</details>

