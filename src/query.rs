use reqwest::header::HeaderMap;
use serde_json::{Map, Number, Value};
use url::form_urlencoded;

pub trait Query {
    fn get_url(&self, api_url: String) -> String;
    fn get_headers(
        &self,
        client_id: String,
        user_id: String,
        timestamp: u64,
    ) -> HeaderMap;
}

#[derive(Debug)]
pub struct AuthInfo {
    hound_client_auth: String,
    hound_request_auth: String,
    timestamp: u64,
}

#[derive(Debug)]
pub struct TextQuery {
    pub query: String, // TODO: remove pub
}

impl TextQuery {
    pub fn new(query_text: String) -> TextQuery {
        TextQuery {
            query: query_text,
        }
    }
}

impl Query for TextQuery {
    fn get_url(&self, api_url: String) -> String {
        let url: String =
            form_urlencoded::Serializer::new(format!("{}v1/text?", api_url.to_string()))
                .append_pair("query", &self.query)
                .finish();
        url
    }

    fn get_headers(
        &self,
        client_id: String,
        user_id: String,
        timestamp: u64,
    ) -> HeaderMap {
        let mut request_info = Map::new();
        request_info.insert(
            "TimeStamp".to_string(),
            Value::Number(Number::from(timestamp)),
        );
        request_info.insert("ClientID".to_string(), Value::String(client_id.to_string()));
        request_info.insert("UserID".to_string(), Value::String(user_id.to_string()));
        request_info.insert("SDK".to_string(), Value::String("rust".to_string()));

        // request_info.insert("Hound-Input-Language-English-Name".to_string(), Value::String("english".to_string()));
        // request_info.insert("Hound-Input-Language-IETF-Tag".to_string(), Value::String("en-CA".to_string()));
        let request_info_json = serde_json::to_string(&request_info).unwrap();

        let mut header_map = HeaderMap::new();
        header_map.insert("Hound-Request-Info", request_info_json.parse().unwrap());
        header_map.insert(
            "Hound-Request-Info-Length",
            request_info_json.len().to_string().parse().unwrap(),
        );

        header_map
    }
}