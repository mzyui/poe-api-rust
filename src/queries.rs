use std::fmt::Display;

use serde_json::{json, Value};

use crate::models::{query::QueryHash, FileData};

#[derive(Default, Debug)]
pub struct RequestData {
    pub path: RequestPath,
    pub query_name: QueryHash,
    pub data: serde_json::Value,
    pub files: Vec<FileData>,
    pub knowledge: bool,
    pub ratelimit: u64,
}

#[derive(Default, Debug, Clone)]
pub enum RequestPath {
    #[default]
    GqlPost,
    GqlUploadPost,
}

impl Display for RequestPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::GqlPost => write!(f, "gql_POST"),
            Self::GqlUploadPost => write!(f, "gql_upload_POST"),
        }
    }
}

impl RequestData {
    pub fn generate_payload(&self) -> Value {
        let data = if self.data.is_null() {
            json!({})
        } else {
            json!(self.data)
        };

        let payload = json!({
            "queryName": self.query_name,
            "variables": data,
            "extensions": {"hash": self.query_name.get_hash()},
        });
        payload
    }
}
