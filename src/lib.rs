use url::form_urlencoded;
use std::time::SystemTime;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use serde_json::{Value, Map, Number};
use base64;

#[derive(Debug)]
pub struct TextQuery {
    query: String,
}

impl TextQuery {
    pub fn new(query_text: String) -> TextQuery {
        TextQuery{query: query_text}
    }
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
}

impl Client {
    pub fn new(api_url: String, client_id: String, client_key: String) -> Self {
        Client{ api_url, client_id, client_key }
    }

    pub fn generate_auth_values(&self, user_id: String, request_id: String, timestamp: u64) -> AuthInfo {
        // let decoded_client_key = base64_url::decode(&self.client_key).unwrap();
        let decoded_client_key = base64::decode_config(&self.client_key, base64::URL_SAFE).unwrap();
        let mut mac: Hmac<Sha256> = Hmac::new_varkey(&decoded_client_key).unwrap();
        let data = format!("{};{}{}", user_id, request_id, timestamp.to_string());
        // let mut data = String::new();
        // data.push_str(&user_id);
        // data.push_str(";");
        // data.push_str(&request_id);
        // data.push_str(&timestamp.to_string());
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
        // let client = reqwest::blocking::Client::new();
        let client = reqwest::blocking::Client::builder()
            .http1_title_case_headers()
            .build().unwrap();
        let url: String = form_urlencoded::Serializer::new(format!("{}v1/text?", self.api_url.clone()))
            .append_pair("query", &q.query)
            .finish();

        let now = SystemTime::now();
        let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
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

        // let mut header_map = reqwest::header::HeaderMap::new();
        // header_map.insert("User-Agent", "houndify-rust".parse().unwrap());
        // header_map.insert("Hound-Request-Authentication", auth_info.hound_request_auth.parse().unwrap());
        // header_map.insert("Hound-Client-Authentication", auth_info.hound_client_auth.parse().unwrap());
        // header_map.insert("Hound-Request-Info", request_info_json.parse().unwrap());
        // header_map.insert("Hound-Request-Info-Length", request_info_json.len().parse().unwrap());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_auth_values() {
        let client_id = String::from("EqQpJDGt0YozIb8Az6xvvA==");
        let client_key = String::from("jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==");
        let api_base = String::from("https://api.houndify.com/");

        let c = Client::new(api_base, client_id, client_key);
        let auth_info = &c.generate_auth_values(String::from("test_user"), String::from("deadbeef"), 1580278266);
        println!("{:?}", auth_info);
        assert_eq!(auth_info.hound_client_auth, "EqQpJDGt0YozIb8Az6xvvA==;1580278266;Ix3_MpLnyz1jGEV5g-mXxmbfgfZ85rD8-6S6yRTJEag=");
    }

    #[test]
    fn it_works() {
        let client_id = String::from("EqQpJDGt0YozIb8Az6xvvA==");
        let client_key = String::from("jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==");
        let api_base = String::from("https://api.houndify.com/");

        let c = Client::new(api_base, client_id, client_key);
        let q = TextQuery::new(String::from("what is 1+1?"));
        c.text_query(q);
    }
}
