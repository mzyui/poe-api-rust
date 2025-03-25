<div align="center">
  <h1>Poe API Rust <img src="https://psc2.cf2.poecdn.net/favicon.svg" height="35"></h1>
  <em>A simple, lightweight, and efficient API wrapper for Poe.com</em>
</div>

## Table of Contents 📚

- [Overview](#overview-)
- [How to Get Your Token](#how-to-get-your-token-)
  - [Step 1: Retrieve `p-b` and `p-lat` Cookies (Required)](#step-1-retrieve-p-b-and-p-lat-cookies-required)
  - [Step 2: Retrieve `formkey` (Optional)](#step-2-retrieve-formkey-optional)
- [Installation](#installation-)
- [Connecting to the API](#connecting-to-the-api-)
- [Documentation](#documentation-)
  - **Message Handling**
    - [Send Message](#send-message-)
    - [Retry Message](#retry-message-)
    - [Cancel Message](#cancel-message-)
    - [Delete Messages](#delete-messages-)
    - [Get Message Share URL](#get-message-share-url-)
    - [Get Total Cost Points](#get-total-cost-points-)
    - [Get List Preview Apps](#get-list-preview-apps-)
  - **User & Bot Management**
    - [Explore](#explore-)
    - [Get Available Categories](#get-available-categories-)
    - [Get Bot Info](#get-bot-info-)
    - [Get User Info](#get-user-info-)
    - [Follow User](#follow-user-)
    - [Unfollow User](#unfollow-user-)
  - **Chat Setup & Customization**
    - [Set Default Message Point Limit](#set-default-message-point-limit-)
    - [Set Default Bot](#set-default-bot-)
    - [Set Chat Context Optimization](#set-chat-context-optimization-)
    - [Set Chat Title](#set-chat-title-)
  - **Conversation Management**
    - [Purge All Conversations](#purge-all-conversations-)
    - [Delete Chat](#delete-chat-)
    - [Import Chat](#import-chat-)
    - [Chat History](#chat-history-)
    - [Clear Chat Context](#clear-chat-context-)
  - **Miscellaneous**
    - [Get Settings](#get-settings-)
- [License](#license-)

---

## Overview ✨

**Poe API Rust** is a high-performance API designed to manage chat and messaging functionalities on Poe.com. Leveraging the safety and concurrency benefits of Rust, this API wrapper enables you to:

- Explore available users, bots, and AI models.
- Customize chat conversation settings.
- Send, retry, and cancel messages seamlessly.
- Retrieve detailed data about bots and users.
- Optimize chat contexts for enhanced interactions.

This documentation provides a comprehensive guide to all the available functions within the API.

---

## How to Get Your Token 🔑

### Step 1: Retrieve `p-b` and `p-lat` Cookies (Required)

1. **Sign in** at [poe.com](https://poe.com/).
2. **Open Developer Tools:**
   - **Chromium:** Press `F12` or right-click and select **Inspect**, then navigate to **Application** > **Cookies** > **poe.com**.
   - **Firefox:** Press `F12` or right-click and select **Inspect**, then go to **Storage** > **Cookies**.
   - **Safari:** Press `F12` or right-click and select **Inspect**, then access **Storage** > **Cookies**.
3. **Copy** the values of the `p-b` and `p-lat` cookies.

### Step 2: Retrieve `formkey` (Optional)

> **Note:** The **poe-api-rust** automatically retrieves the `formkey` for you. If this fails, follow the steps below to obtain the token manually.

There are two methods to get the `formkey`:

1. **Method 1: Using the Network Tab**
   - Open Developer Tools (`F12` or right-click and select **Inspect**).
   - Navigate to **Network** > **gql_POST** > **Headers**.
   - Copy the value of **Poe-Formkey**.

2. **Method 2: Using the Console**
   - Open Developer Tools (`F12` or right-click and select **Inspect**).
   - Go to the **Console** tab.
   - Type: `allow pasting` and press Enter.
   - Paste the following script:
     ```js
     window.ereNdsRqhp2Rd3LEW()
     ```
   - Copy the resulting output.

---

## Installation 💾

Use Cargo to manage your dependencies and build your project. To integrate the API, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
poe-api = { git = "https://github.com/zevtyardt/poe-api-rust", default-features = false }
```

### Command-Line Interface (CLI)

This library also offers a CLI. Install it using:

```bash
cargo install --git "https://github.com/zevtyardt/poe-api-rust"
```

Then, execute CLI commands using the `poe-cli` command.

---

## Connecting to the API 🔗

Below is a simple example of how to initialize a connection:

```rust
use poe_api::{api::PoeApi, models::Token};

let api = PoeApi::new(Token {
    p_b: "P-B", // required
    p_lat: "P-LAT", // required
    formkey: Some("fromkey"), // optional
}).await?;
```

---

## Documentation 📖

### Send Message ✉️

Send a new message to a specified AI model or bot name. Both text and media messages are supported.

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

// Ask a simple question using the "gemini-2.0-flash" model.
let mut message = api.send_message(SendMessageData {
    bot_handle: "gemini-2.0-flash",
    message: "What is the result of 2x2?",
    ..Default::default()
}).await?;

// Handle streamed output.
while let Some(chunk) = message.next().await {
    // Process the chunk or print it directly.
    chunk.print()?;
}

// For non-streamed output:
let text = message.text().await;
```

**Another Example:**

Sending a message in an existing chat thread including an image referenced locally:

```rust
// Retrieve chat_id from an existing message.
let chat_id = message.chat().inner.chat_id;
let message_data = SendMessageData {
    bot_handle: "gemini-2.0-flash",
    message: "Who is she?",
    chat_id: Some(chat_id),
    files: vec![
        FileInput::Local("my-wife.png")
    ],
};

let mut message = api.send_message(message_data).await?;
// Alternatively:
let mut message = message.chat().send_message(message_data).await?;

println!("{}", message.text().await);
```

**Expected Output:**

```markdown
The anime character in the image is Ritsu Tainaka from the anime series K-On!. She is the self-proclaimed president of the Light Music Club and the drummer of the band Ho-kago Tea Time.

---

Related searches:
+ [anime characters in image](https://www.google.com/search?q=anime+characters+in+image&client=app-vertex-grounding-quora-poe)
+ [anime with characters Ritsu Tainaka](https://www.google.com/search?q=anime+with+characters+Ritsu+Tainaka&client=app-vertex-grounding-quora-poe)
```
</details>

---

### Retry Message 🔄

Reattempt sending or recreating a message that was previously undelivered or inappropriate.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description     |
|-------------|-----------|-----------------|
| `chat_code` | `&str`    | Chat identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_code: &str = "sample";
let mut message = api.retry_message(chat_code).await?;

let message = message.retry().await?;

// Functionality similar to send_message.
```
</details>

---

### Cancel Message ❌

Cancel a message that is in the process of being sent to avoid duplicates or errors.

<details>
<summary><b>Parameters</b></summary>

| Field Name | Data Type | Description     |
|------------|-----------|-----------------|
| `chat_id`  | `i64`     | Chat identifier |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 12345;
api.cancel_message(chat_id).await?;

// Alternately, cancel via the message instance.
message.cancel().await?;
```
</details>

---

### Delete Messages 🗑️

Delete one or more messages from a chat by specifying their message IDs.

<details>
<summary><b>Parameters</b></summary>

| Field Name   | Data Type   | Description                               |
|--------------|-------------|-------------------------------------------|
| `chat_id`    | `i64`       | Chat identifier                           |
| `message_ids`| `Vec<i64>`  | Vector containing message IDs             |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 12345;
let message_ids: Vec<i64> = vec![678910, 11121314];
api.delete_message(chat_id, message_ids).await?;

// Delete specific message types:
message.delete_user_message().await?;
message.delete_bot_message().await?;
// Or clear both contexts:
message.delete_message_context().await?;
```
</details>

---

### Get Message Share URL 🔗

Generate a shareable URL for a specific message, making it easy to share externally.

<details>
<summary><b>Parameters</b></summary>

| Field Name   | Data Type   | Description              |
|--------------|-------------|--------------------------|
| `chat_id`    | `i64`       | Chat identifier          |
| `message_ids`| `Vec<i64>`  | Vector containing IDs    |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 12345;
let message_ids: Vec<i64> = vec![678910, 11121314];
api.get_message_share_url(chat_id, message_ids).await?;

// Alternatively, via the message instance:
message.share().await?;
```
</details>

---

### Get Total Cost Points 💰

Calculate the total cost (in message points) for a specific message. Useful for metering or billing.

<details>
<summary><b>Parameters</b></summary>

| Field Name     | Data Type | Description          |
|----------------|-----------|----------------------|
| `message_code` | `&str`    | Message identifier   |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let message_code: &str = "abcdef";
api.get_total_cost_points(message_code).await?;

// Or via the message instance:
message.total_cost_points().await?;
```
</details>

---

### Get List Preview Apps 📱

Generate a shareable URL for preview apps. Poe.com's "Previews" feature allows you to interact with web applications, such as games, animations, or data visualizations using AI coding models.

<details>
<summary><b>Parameters</b></summary>

| Field Name   | Data Type | Description            |
|--------------|-----------|------------------------|
| `message_id` | `i64`     | Message identifier     |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let message_id: i64 = 12345;
api.get_list_preview_app(message_id).await?;

// Or via the message instance:
message.list_preview_app().await?;
```
</details>

---

### Explore 🔎

Discover available users, bots, and AI models.

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
        Entity::User(user_info) => {
            // Process user data.
        }
        Entity::Bot(bot_info) => {
            // Process bot data.
        }
    }
}
```
</details>

---

### Get Available Categories 📑

Retrieve a list of all available AI model categories so that you can quickly find your interest.

<details>
<summary><b>Example</b></summary>

```rust
let categories = api.get_available_categories().await?;
println!("{:?}", categories);
```

**Expected Output:**

```markdown
["Official", "Reasoning", "Apps", "Search", "Image generation", "Audio and video", "For you", "Popular", "Funny", "Roleplay", "AI", "Utilities", "Programming", "Hobbies", "Learning", "Game Apps", "Featured", "Professional", "Creative writing", "Game Bots", "Advice", "Mind", "Translation", "Text analysis", "New"]
```
</details>

---

### Get Bot Info 🤖

Retrieve detailed information about a specific bot, including configuration, status, and capabilities.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description         |
|-------------|-----------|---------------------|
| `bot_handle`| `&str`    | Bot handle name     |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let bot_handle: &str = "Claude-3.7-Sonnet-Reasoning";
let bot_info = api.get_bot_info(bot_handle).await?;
println!("{:?}", bot_info);
```

**Expected Output:**

```markdown
Some(BotInfo {
    id: "Qm90OjEwMjY=",
    bot_id: 1026,
    handle: "Claude-3.7-Sonnet-Reasoning",
    display_name: "Claude-3.7-Sonnet-Reasoning",
    model: Some("flannel_reasoning"),
    picture_url: Some("https://qph.cf2.poecdn.net/main-thumb-pb-1026-200-fvvsiofehkfrtswcutfmahqytzyfadsp.jpeg"),
    description: "Anthropic's most intelligent model (with reasoning capabilities on by default). Claude 3.7 Sonnet is a hybrid reasoning model, producing near-instant responses or extended, step-by-step thinking. Recommended for complex math or coding problems. Supports a 200k token context window.",
    powered_by: Some("Powered by Anthropic."),
    tags: ["OFFICIAL"],
    display_message_point_price: 123,
    introduction: Some(""),
    is_created_by_poe_user_account: false
})
```
</details>

---

### Get User Info 👤

Fetch profile details for a specific user.

<details>
<summary><b>Parameters</b></summary>

| Field Name   | Data Type | Description       |
|--------------|-----------|-------------------|
| `user_handle`| `&str`    | User handle name  |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let user_handle: &str = "openai";
let user_info = api.get_user_info(user_handle).await?;
println!("{:?}", user_info);
```

**Expected Output:**

```markdown
Some(UserInfo {
    id: "UG9lVXNlcjoyOTEwNDAwODc5",
    uid: 2910400879,
    handle: "openai",
    full_name: "OpenAI",
    follower_count: 2470,
    medium_profile_photo_url: Some("https://qph.cf2.poecdn.net/main-thumb-2910400879-100-wrfgcbmfjrhquvwlxvypitmawpovrxoi.jpeg"),
    profile_photo_url: Some("https://qph.cf2.poecdn.net/main-thumb-2910400879-200-wrfgcbmfjrhquvwlxvypitmawpovrxoi.jpeg")
})
```
</details>

---

### Follow User ➕

Add the specified user to your follow list to track updates and activities.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description       |
|-------------|-----------|-------------------|
| `user_id`   | `i64`     | User identifier   |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let user_id: i64 = 123456;
api.follow_user(user_id).await?;
```
</details>

---

### Unfollow User ➖

Remove the specified user from your follow list.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description       |
|-------------|-----------|-------------------|
| `user_id`   | `i64`     | User identifier   |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let user_id: i64 = 123456;
api.unfollow_user(user_id).await?;
```
</details>

---

### Set Default Message Point Limit 🔢

Set the default threshold for message points per conversation. Useful for enforcing usage policies or managing message sizes.

<details>
<summary><b>Parameters</b></summary>

| Field Name | Data Type | Description                                           |
|------------|-----------|-------------------------------------------------------|
| `limit`    | `usize`   | Maximum number of message points allowed per chat     |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let limit: usize = 420;
api.set_default_message_point_limit(limit).await?;
```
</details>

---

### Set Default Bot 🛠️

Assign a default bot to the chat system when no specific bot is chosen.

<details>
<summary><b>Parameters</b></summary>

| Field Name | Data Type | Description         |
|------------|-----------|---------------------|
| `bot_id`   | `i64`     | Bot identifier      |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let bot_id: i64 = 420;
api.set_default_bot(bot_id).await?;
```
</details>

---

### Set Chat Context Optimization ⚙️

Toggle context optimization for a chat session to improve relevance and performance.

<details>
<summary><b>Parameters</b></summary>

| Field Name | Data Type | Description                                                   |
|------------|-----------|---------------------------------------------------------------|
| `chat_id`  | `i64`     | Chat identifier                                               |
| `enabled`  | `bool`    | Set `true` to enable or `false` to disable optimization       |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 420;
let enabled: bool = false;
api.set_chat_context_optimization(chat_id, enabled).await?;

// Alternatively, via the message's chat instance:
message.chat().set_context_optimization(enabled).await?;
```
</details>

---

### Set Chat Title 🏷️

Update the title of an existing chat conversation.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description            |
|-------------|-----------|------------------------|
| `chat_id`   | `i64`    | Chat identifier         |
| `new_title` | `&str`   | New title for the chat  |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 420;
let new_title: &str = "ayonima";
api.set_new_title(chat_id, new_title).await?;

// Or via the message’s chat instance:
message.chat().set_title(new_title).await?;
```
</details>

---

### Purge All Conversations 🧹

Remove all chat conversations from the system to reset the chat history.

<details>
<summary><b>Example</b></summary>

```rust
api.purge_all_conversations().await?;
```
</details>

---

### Delete Chat 🗑️

Delete a specific chat session using its unique identifier.

<details>
<summary><b>Parameters</b></summary>

| Field Name | Data Type | Description         |
|------------|-----------|---------------------|
| `chat_id`  | `i64`     | Chat identifier     |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 420;
api.delete_chat(chat_id).await?;

// Or via the message's chat instance:
message.chat().delete().await?;
```
</details>

---

### Import Chat 📥

Import chat data from an external source, useful for migrating or restoring conversations.

<details>
<summary><b>Parameters</b></summary>

| Field Name  | Data Type | Description         |
|-------------|-----------|---------------------|
| `chat_code` | `&str`   | Chat identifier      |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_code: &str = "sample";
api.import_chat(chat_code).await?;
```
</details>

---

### Chat History 📜

Retrieve the complete history of chat conversations.

<details>
<summary><b>Example</b></summary>

```rust
while let Some(chat) = api.chat_history() {
    dbg!(chat);
}
```
</details>

---

### Clear Chat Context 🔄

Reset the context of a specific chat conversation by clearing any stored temporary data. This is useful for restarting a conversation without any residual context.

<details>
<summary><b>Parameters</b></summary>

| Field Name | Data Type | Description         |
|------------|-----------|---------------------|
| `chat_id`  | `i64`     | Chat identifier     |
</details>

<details>
<summary><b>Example</b></summary>

```rust
let chat_id: i64 = 123456;
api.clear_chat_context(chat_id).await?;

// Or via the message's chat instance:
message.chat().clear_context().await?;
```
</details>

---

### Get Settings ⚙️

Retrieve your settings including remaining points and additional configuration details.

<details>
<summary><b>Example</b></summary>

```rust
let my_setting = api.get_settings().await?;
println!("{:?}", my_setting);
```

**Expected Output:**

```markdown
MySettings {
    uid: 659168979,
    default_bot: DefaultBot {
        display_name: "Assistant",
        bot_id: 3002,
        id: "Qm90OjMwMDI="
    },
    message_point_info: MessagePointInfo {
        message_point_reset_time: +57199-01-26T05:30:00Z,
        message_point_balance: 1827,
        total_message_point_allotment: 3000,
        all_chat_default_point_price_threshold_per_message: 500
    },
    primary_phone_number: None,
    primary_email: Some("[REDACTED]"),
    confirmed_emails: ["[REDACTED]"],
    has_active_subscription: false,
    enable_gtm_event_sending: true,
    viewer_country_code: "ID",
    global_context_optimization_status: true,
    enable_global_context_optimization: true,
    has_unread_message: false
}
```
</details>

---

## License 📄

```text
MIT License

Copyright (c) 2025 zevtyardt

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
