use reqwest::header::HeaderMap;
use serde_json::{Map, Number, Value};
use url::form_urlencoded;
use std::result::Result;
use std::error::Error;

pub struct QueryOptions {
    pub(crate) user_id: String,
}

impl QueryOptions {
    pub fn new() -> Self {
        QueryOptions {
            user_id: "test_user".to_string(),
        }
    }

    pub fn user_id<'a>(&'a mut self, user_id: &str) -> &'a mut Self {
        self.user_id = user_id.to_string();
        self
    }
}

pub trait Query {
    fn get_url(&self, api_url: &str) -> String;
    fn get_headers(&self, client_id: &str, user_id: &str, timestamp: u64) -> Result<HeaderMap, Box<dyn Error>>;
}

#[derive(Debug)]
pub(crate) struct TextQuery {
    query: String,
}

impl TextQuery {
    pub fn new(query_text: &str) -> TextQuery {
        TextQuery {
            query: query_text.to_string(),
        }
    }
}

impl Query for TextQuery {
    fn get_url(&self, api_url: &str) -> String {
        let url: String = form_urlencoded::Serializer::new(format!("{}v1/text?", api_url))
            .append_pair("query", &self.query)
            .finish();
        url
    }

    fn get_headers(&self, client_id: &str, user_id: &str, timestamp: u64) -> std::result::Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut request_info = Map::new();
        request_info.insert(
            "TimeStamp".to_string(),
            Value::Number(Number::from(timestamp)),
        );
        request_info.insert("ClientID".to_string(), Value::String(client_id.to_string()));
        request_info.insert("UserID".to_string(), Value::String(user_id.to_string()));
        request_info.insert(
            "SDK".to_string(),
            Value::String("houndify-sdk-rust/1.0".to_string()),
        );

        // request_info.insert("Hound-Input-Language-English-Name".to_string(), Value::String("english".to_string()));
        // request_info.insert("Hound-Input-Language-IETF-Tag".to_string(), Value::String("en-CA".to_string()));
        let request_info_json = serde_json::to_string(&request_info)?;

        let mut header_map = HeaderMap::new();
        header_map.insert("Hound-Request-Info", request_info_json.parse()?);
        header_map.insert(
            "Hound-Request-Info-Length",
            request_info_json.len().to_string().parse()?,
        );

        Ok(header_map)
    }
}
