use crate::error::HoundifyError;
use crate::query::{Query, TextQuery};
use base64;
use hmac::{Hmac, Mac};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::HeaderMap;
use serde_json::{Map, Number, Value};
use sha2::Sha256;
use std::time::SystemTime;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, HoundifyError>;

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug)]
pub struct Client {
    api_url: String,
    client_id: String,
    client_key: String,
    http_client: HttpClient,
    request_id_generator: fn() -> String,
}

impl Client {
    pub fn new(
        api_url: &str,
        client_id: &str,
        client_key: &str,
        request_id_generator_option: Option<fn() -> String>,
    ) -> Self {
        let http_client = reqwest::blocking::Client::builder()
            .http1_title_case_headers() // because houndify API headers are case-sensitive :(
            .build()
            .unwrap();

        let request_id_generator = match request_id_generator_option {
            Some(f) => f,
            None => || Uuid::new_v4().to_string(),
        };

        Client {
            api_url: api_url.to_string(),
            client_id: client_id.to_string(),
            client_key: client_key.to_string(),
            http_client,
            request_id_generator,
        }
    }

    fn build_auth_headers(
        &self,
        user_id: &str,
        request_id: &str,
        timestamp: u64,
    ) -> std::result::Result<HeaderMap, Box<dyn std::error::Error>> {
        let decoded_client_key = base64::decode_config(&self.client_key, base64::URL_SAFE)?;
        let mut mac: Hmac<Sha256> = Hmac::new_varkey(&decoded_client_key).unwrap();
        let data = format!("{};{}{}", user_id, request_id, timestamp.to_string());
        mac.input(data.as_bytes());
        let hmac_result = mac.result();
        let signature = base64::encode_config(&hmac_result.code(), base64::URL_SAFE);
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Hound-Client-Authentication",
            format!("{};{};{}", &self.client_id, &timestamp, &signature).parse()?,
        );
        header_map.insert(
            "Hound-Request-Authentication",
            format!("{};{}", &user_id, &request_id).parse()?,
        );
        Ok(header_map)
    }

    // fn get_request_info_headers(&self, timestamp: u64, query: TextQuery) -> Result<Map<String, serde_json::value::Value>> {
    //     let mut request_info = Map::new();
    //     request_info.insert("ClientID".to_string(), Value::String(self.client_id.to_string()));
    //     Ok(request_info)
    // }

    pub fn text_query(&self, mut query: TextQuery) -> Result<String> {
        let timestamp = get_current_timestamp();
        // println!("Timestamp={}", timestamp);
        let request_id = (&self.request_id_generator)();

        let mut headers = match self.build_auth_headers(query.user_id, &request_id, timestamp) {
            Ok(h) => h,
            Err(e) => return Err(HoundifyError::new(e.into())),
        };

        let url = query.get_url(&self.api_url);

        &query.request_info.timestamp(timestamp);
        &query.request_info.client_id(&self.client_id);

        let request_info_json = query.request_info.serialize()?;
        let request_info_len = request_info_json.len();
        headers.insert("Houndify-Request-Info", request_info_json.parse().unwrap());
        headers.insert(
            "Houndify-Request-Info-Length",
            request_info_len.to_string().parse().unwrap(),
        );

        let req = self.http_client.get(&url).headers(headers);
        println!("Request={:#?}", req);

        let res = match req.send() {
            Ok(r) => {
                println!("Response={:#?}", r);
                r
            }
            Err(e) => return Err(HoundifyError::new(e.into())),
        };

        match res.text() {
            Ok(res) => Ok(res),
            Err(e) => Err(HoundifyError::new(e.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auth_values() {
        let client_id = String::from("EqQpJDGt0YozIb8Az6xvvA==");
        let client_key = String::from("jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==");
        let api_base = String::from("https://api.houndify.com/");
        let client = Client::new(&api_base, &client_id, &client_key, None);
        let auth_headers = client
            .build_auth_headers("test_user", "deadbeef", 1580278266)
            .unwrap();
        assert_eq!(
            auth_headers.get("Hound-Client-Authentication").unwrap(),
            "EqQpJDGt0YozIb8Az6xvvA==;1580278266;Ix3_MpLnyz1jGEV5g-mXxmbfgfZ85rD8-6S6yRTJEag="
        );
        assert_eq!(
            auth_headers.get("Hound-Request-Authentication").unwrap(),
            "test_user;deadbeef"
        );
    }
}
