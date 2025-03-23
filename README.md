<div align="center">
  
# Poe API Rust
</div>

**Poe API Rust** is a simple, lightweight, and efficient API wrapper for Poe.com. This Rust-based project provides a clean interface to interact with the Poe API, making it easy to manage chat conversations, bot interactions, and user profiles.

## Table of Contents

- [Overview](#overview)
- [How to get your token](#how-to-get-your-token)
  - [Getting `p-b` and `p-lat` cookies (required)](#step-1-retrieve-p-b-and-p-lat-cookies-required)
  - [Getting fromkey (optional)](#step-2-retrieve-fromkey-optional)
- [Connecting to the API](#connecting-to-the-api)
- [Documentation](#documentation)
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

---

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
> The **poe-api-wrapper** automatically retrieves the `formkey` for you. If it fails, follow the steps below to obtain the token manually.

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

--- 

## Connecting to the API
```rust
use poe_api::{api::PoeApi, models::Token};

let api = PoeApi::new(Token {
    p_b: "P-B", // required
    p_lat: "P-LAT", // required
    formkey: Some("fromkey"), // optional
}).await?;
```

---

## Documentation

#### Send Message

**Description:**  
Sends a new message to a specified chat conversation. Supports both text and media messages.

**Parameters:**
- `chat_id` (String): Identifier for the target chat.
- `message` (String): Message content.

**Example:**
```rust
match send_message(String::from("chat_id_4567"), String::from("Hello, world!")) {
    Ok(_) => println!("Message sent successfully."),
    Err(e) => eprintln!("Error: {}", e),
}
```
---

