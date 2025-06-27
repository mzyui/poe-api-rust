use mime2ext::mime2ext;
use rand::Rng;
use reqwest::Client;
use serde_json::Value;
use tokio::fs;

use crate::{
    constants::default_headers,
    models::{FileData, FileInput},
};

pub fn generate_nonce(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn get_json_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current_value = Some(value);

    for part in parts {
        current_value = current_value.and_then(|v| v.get(part));
    }
    current_value
}

pub async fn generate_file(files: &[FileInput<'_>]) -> anyhow::Result<Vec<FileData>> {
    let client = Client::builder()
        .default_headers(default_headers())
        .build()?;
    let mut outputs = Vec::new();
    for file in files {
        match file {
            FileInput::Url(url) => {
                let response = client.get(*url).send().await?;
                let content = response.bytes().await?;
                let data = content.to_vec();

                let mime_type = tree_magic_mini::from_u8(&data).to_string();
                let extension = mime2ext(&mime_type).unwrap_or("txt");
                let name = format!("{}.{}", generate_nonce(8), extension);
                outputs.push(FileData {
                    data,
                    name,
                    mime_type,
                });
            }
            FileInput::Local(pathbuf) => {
                let data = fs::read(&pathbuf).await?;
                let mime_type = tree_magic_mini::from_u8(&data).to_string();
                let extension = pathbuf
                    .extension()
                    .and_then(|v| v.to_str())
                    .or(mime2ext(&mime_type))
                    .unwrap_or("txt");
                let name = format!("{}.{}", generate_nonce(8), extension);
                outputs.push(FileData {
                    data,
                    name,
                    mime_type,
                });
            }
        }
    }

    Ok(outputs)
}
