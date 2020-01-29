use url::form_urlencoded;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use reqwest::blocking::{Client, RequestBuilder};
use std::time::SystemTime;

fn get_current_timestamp() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub trait Query {
    fn build_request(&self, c: Client) -> RequestBuilder;
}

#[derive(Debug)]
pub struct AuthInfo {
    hound_client_auth: String,
    hound_request_auth: String,
    timestamp: u64,
}

pub fn generate_auth_values(client_id: String, client_key: String, user_id: String, request_id: String, timestamp: u64) -> AuthInfo {
    let decoded_client_key = base64::decode_config(&client_key, base64::URL_SAFE).unwrap();
    let mut mac: Hmac<Sha256> = Hmac::new_varkey(&decoded_client_key).unwrap();
    let data = format!("{};{}{}", user_id, request_id, timestamp.to_string());
    mac.input(data.as_bytes());
    let hmac_result = mac.result();
    let signature = base64::encode_config(&hmac_result.code(), base64::URL_SAFE);
    AuthInfo{
        hound_client_auth: format!("{};{};{}", &client_id, &timestamp, &signature),
        hound_request_auth: format!("{};{}", &user_id, &request_id),
        timestamp: timestamp,
    }
}

#[derive(Debug)]
pub struct TextQuery {
    user_id: String,
    query_id: String,
    pub query: String,  // TODO: remove pub
}

impl TextQuery {
    pub fn new(query_text: String) -> TextQuery {
        TextQuery{query: query_text, user_id: "test_user".to_string(), query_id: "deadbeef".to_string()}
    }
}

impl Query for TextQuery {
    fn build_request(&self, c: Client) -> RequestBuilder {
        let api_url = "https://api.houndify.com/";
        let url: String = form_urlencoded::Serializer::new(format!("{}v1/text?", api_url.to_string()))
            .append_pair("query", &self.query)
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
        rb
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
        let auth_info = generate_auth_values(client_id, client_key, String::from("test_user"), String::from("deadbeef"), 1580278266);
        println!("{:?}", auth_info);
        assert_eq!(auth_info.hound_client_auth, "EqQpJDGt0YozIb8Az6xvvA==;1580278266;Ix3_MpLnyz1jGEV5g-mXxmbfgfZ85rD8-6S6yRTJEag=");
    }
}