<div align="center">
  
# Poe API Rust
</div>

**Poe API Rust** is a simple, lightweight, and efficient API wrapper for Poe.com. This Rust-based project provides a clean interface to interact with the Poe API, making it easy to manage chat conversations, bot interactions, and user profiles.

## Table of Contents

- [Overview](#overview)
- How to get your token
  - [Getting `p-b` and `p-lat` cookies (required)](#step-1-retrieve-p-b-and-p-lat-cookies-required)
  - [Getting fromkey (optional)](#step-2-retrieve-fromkey-optional)
- [Connecting to the API](#connecting-to-the-api)
- Documentation
  - **Message Handling**
    - [Send Message](#send-message)
    - [Retry Message](#retry-message)
    - [Cancel Message](#cancel-message)
    - [Delete Messages](#delete-messages)
    - [Message Share URL](#message-share-url)
    - [Get Total Cost Points](#total-cost-points)
    - [Get List Preview App](#get-preview-app)
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
  - **User & Bot Management**
    - [Explore](#explore)
    - [Bot Info](#bot-info)
    - [User Info](#user-info)
    - [Follow User](#follow-user)
    - [Unfollow User](#unfollow-user)
  - **Misc**
    - [Get Settings](#settings)
    - [Get Available Categories](#available-categories)
- [**Installation**](#installation)
- [**License**](#license)

---

## Overview

**Poe API Rust** is an advanced API crafted for managing chat and messaging functionalities on Poe.com. Leveraging Rust's capabilities, this project emphasizes safety, speed, and efficient concurrency. The API enables users to:

- Discover available chat rooms and topics.
- Customize settings for chat conversations.
- Send, retry, and cancel messages seamlessly.
- Access detailed information about bots and users.
- Fine-tune and optimize chat contexts for enhanced interactions.

This documentation offers a comprehensive guide to each function available within the API.


### How to Get Your Token

#### Step 1: Retrieve `p-b` and `p-lat` Cookies (Required)
1. Sign in at [Poe](https://poe.com/).
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

## Connecting to the API
```rust
use poe_api::{api::PoeApi, models::Token};

let api = PoeApi::new(Token {
    p_b: "P-B", // required
    p_lat: "P-LAT", // required
    formkey: Some("fromkey"), // optional
}).await?;
```

## Documentation
#### Send Message
Sends a new message to a specified model (default `assistant`). Supports both text and media messages.

<details>
<summary><b>Parameters:</b></summary>

```rust
pub struct SendMessageData<'a> {
    pub bot: &'a str,
    pub message: &'a str,
    pub chat_id: Option<i64>,
    pub files: Vec<FileInput<'a>>,
}

#[derive(Debug)]
pub enum FileInput<'a> {
    Url(&'a str),
    Local(PathBuf),
}
```
</details>

<details>
<summary><b>Example:</b></summary>

```rust
use poe_api::models::{SendMessageData, FileInput};
use futures_util::StreamExt;

// Ask simple questions using `gemini-2.0-flash` model
let mut message = api.send_message(SendMessageData {
    bot: "gemini-2.0-flash",
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
let mut message = api.send_message(SendMessageData {
    bot: "gemini-2.0-flash",
    message: "who is she??",
    chat_id: Some(chat_id),
    files: vec![
        FileInput::Local("my-wife.png")
    ],
}).await?;

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


#### Retry Message
Attempt to send or recreate a message that was previously undeliverable or inappropriate.

<details>
<summary><b>Parameters:</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_code` | `&str` | Identifier of the chat to retry. |
</details>

<details>
<summary><b>Example:</b></summary>

```rust
let chat_code: &str = "sample";

let mut message = api.retry_message(chat_code).await?;
// or 
let mut message = message.retry().await?;

// Same as #send-message
```
</details>

#### Cancel Message
Cancels a message that is in the process of being sent, useful to prevent duplicates or errors.

<details>
<summary><b>Example:</b></summary>

```rust
let chat_id: i64 = 12345;

api.cancel_message(chat_id).await?;
// or 
message.cancel().await?;
```
</details>

#### Delete Messages
Deletes one or more messages from a chat based on provided message IDs.

<details>
<summary><b>Parameters:</b></summary>

| Field Name  | Data Type | Description |
| --- | --- | --- |
| `chat_id`   | `i64`     | Chat identifier. |
| `message_ids` | `Vec<i64>`| A vector of message IDs to delete. |
</details>

<details>
<summary><b>Example:</b></summary>

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

