use url::form_urlencoded;
use std::time::SystemTime;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use serde_json::{Value, Map, Number};
use base64;
use crate::query::TextQuery;
use reqwest::blocking::Client as HttpClient;

fn get_current_timestamp() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

#[derive(Debug)]
pub struct AuthInfo {
    hound_client_auth: String,
    hound_request_auth: String,
    timestamp: u64,
}

#[derive(Debug)]
pub struct Client {
    api_url: String,
    client_id: String,
    client_key: String,
    http_client: HttpClient,
}

impl Client {
    pub fn new(api_url: String, client_id: String, client_key: String) -> Self {
        let http_client = reqwest::blocking::Client::builder()
            .http1_title_case_headers()  // because houndify API headers are case-sensitive :(
            .build().unwrap();
        Client{ api_url, client_id, client_key, http_client }
    }

    pub fn generate_auth_values(&self, user_id: String, request_id: String, timestamp: u64) -> AuthInfo {
        let decoded_client_key = base64::decode_config(&self.client_key, base64::URL_SAFE).unwrap();
        let mut mac: Hmac<Sha256> = Hmac::new_varkey(&decoded_client_key).unwrap();
        let data = format!("{};{}{}", user_id, request_id, timestamp.to_string());
        mac.input(data.as_bytes());
        let hmac_result = mac.result();
        let signature = base64::encode_config(&hmac_result.code(), base64::URL_SAFE);
        AuthInfo{
            hound_client_auth: format!("{};{};{}", &self.client_id, &timestamp, &signature),
            hound_request_auth: format!("{};{}", &user_id, &request_id),
            timestamp: timestamp,
        }
    }

    pub fn text_query(&self, q: TextQuery) {
        let url: String = form_urlencoded::Serializer::new(format!("{}v1/text?", self.api_url))
            .append_pair("query", &q.query)
            .finish();

        let timestamp = get_current_timestamp();
        println!("Timestamp={}", timestamp);

        let user_id = String::from("test_user");
        let request_id = String::from("deadbeef");
        let auth_info = &self.generate_auth_values(user_id.to_string(), request_id.to_string(), timestamp);
        println!("AuthInfo={:?}", auth_info);

        let mut request_info = Map::new();
        request_info.insert("TimeStamp".to_string(), Value::Number(Number::from(auth_info.timestamp)));
        request_info.insert("ClientID".to_string(), Value::String(self.client_id.to_string()));
        request_info.insert("UserID".to_string(), Value::String(user_id.to_string()));
        request_info.insert("SDK".to_string(), Value::String("rust".to_string()));

        // request_info.insert("Hound-Input-Language-English-Name".to_string(), Value::String("english".to_string()));
        // request_info.insert("Hound-Input-Language-IETF-Tag".to_string(), Value::String("en-CA".to_string()));
        let request_info_json = serde_json::to_string(&request_info).unwrap();

        let req = client
            .get(&url)
            .header("User-Agent", "houndify-rust")
            .header("Hound-Request-Authentication", &auth_info.hound_request_auth)
            .header("Hound-Client-Authentication", &auth_info.hound_client_auth)
            .header("Hound-Request-Info", &request_info_json)
            .header("Hound-Request-Info-Length", request_info_json.len());
        println!("{:#?}", req);
        let mut res = req.send().unwrap();
        println!("{:#?}", res);
        res.copy_to(&mut std::io::stdout()).unwrap();
    }
}