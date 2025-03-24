pub mod api_settings;
pub mod message;
pub mod on_message;
pub mod query;
pub mod user;

use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug)]
pub struct Token<'a> {
    pub p_b: &'a str,
    pub p_lat: &'a str,
    pub formkey: Option<&'a str>,
}

#[derive(Debug)]
pub enum FileInput<'a> {
    Url(&'a str),
    Local(PathBuf),
}

#[derive(Debug, Clone)]
pub struct FileData {
    pub data: Vec<u8>,
    pub name: String,
    pub mime_type: String,
}

impl FileData {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Default)]
pub struct SendMessageData<'a> {
    pub bot_handle: &'a str,
    pub message: &'a str,
    pub chat_id: Option<i64>,
    pub files: Vec<FileInput<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    User,
    Bot,
}

#[derive(Debug)]
pub struct SearchData<'a> {
    pub query: Option<&'a str>,
    pub category_name: &'a str,
    pub entity_type: EntityType,
    pub count: usize,
}

impl Default for SearchData<'_> {
    fn default() -> Self {
        Self {
            query: None,
            category_name: "defaultCategory",
            entity_type: EntityType::Bot,
            count: 50,
        }
    }
}
