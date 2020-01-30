use crate::query::{Query, QueryOptions, TextQuery};
use crate::error::HoundifyError;
use base64;
use hmac::{Hmac, Mac};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::HeaderMap;
use sha2::Sha256;
use std::time::SystemTime;

type Result<T> = std::result::Result<T, HoundifyError>;

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
}

impl Client {
    pub fn new(api_url: &str, client_id: &str, client_key: &str) -> Self {
        let http_client = reqwest::blocking::Client::builder()
            .http1_title_case_headers() // because houndify API headers are case-sensitive :(
            .build()
            .unwrap();
        Client {
            api_url: api_url.to_string(),
            client_id: client_id.to_string(),
            client_key: client_key.to_string(),
            http_client,
        }
    }

    fn build_auth_headers(&self, user_id: &str, request_id: &str, timestamp: u64) -> HeaderMap {
        let decoded_client_key = base64::decode_config(&self.client_key, base64::URL_SAFE).unwrap();
        let mut mac: Hmac<Sha256> = Hmac::new_varkey(&decoded_client_key).unwrap();
        let data = format!("{};{}{}", user_id, request_id, timestamp.to_string());
        mac.input(data.as_bytes());
        let hmac_result = mac.result();
        let signature = base64::encode_config(&hmac_result.code(), base64::URL_SAFE);
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Hound-Client-Authentication",
            format!("{};{};{}", &self.client_id, &timestamp, &signature)
                .parse()
                .unwrap(),
        );
        header_map.insert(
            "Hound-Request-Authentication",
            format!("{};{}", &user_id, &request_id).parse().unwrap(),
        );
        header_map
    }

    pub fn text_query(&self, q: &str, options: &QueryOptions) -> Result<String> {
        let query = TextQuery::new(q);
        let timestamp = get_current_timestamp();
        println!("Timestamp={}", timestamp);

        let request_id = "deadbeef";
        let mut headers = self.build_auth_headers(&options.user_id, request_id, timestamp);

        let url = query.get_url(&self.api_url);
        for (k, v) in query
            .get_headers(&self.client_id, &options.user_id, timestamp)
            .iter()
        {
            headers.insert(k.clone(), v.clone());
        }
        let req = self.http_client.get(&url).headers(headers);
        println!("Request={:#?}", req);

        let res = match req.send() {
            Ok(r) => {
                println!("Response={:#?}", r);
                r
            },
            Err(e) => {
                return Err(HoundifyError::new(e.into()))
            }
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
        let client = Client::new(&api_base, &client_id, &client_key);
        let auth_headers = client.build_auth_headers("test_user", "deadbeef", 1580278266);
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
