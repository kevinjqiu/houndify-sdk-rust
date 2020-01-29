use crate::query::{Query, TextQuery};
use base64;
use hmac::{Hmac, Mac};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::HeaderMap;
use sha2::Sha256;
use std::time::SystemTime;

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
    pub fn new(api_url: String, client_id: String, client_key: String) -> Self {
        let http_client = reqwest::blocking::Client::builder()
            .http1_title_case_headers() // because houndify API headers are case-sensitive :(
            .build()
            .unwrap();
        Client {
            api_url,
            client_id,
            client_key,
            http_client,
        }
    }

    pub fn build_auth_headers(
        &self,
        user_id: String,
        request_id: String,
        timestamp: u64,
    ) -> HeaderMap {
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

    pub fn text_query(&self, q: String) {
        let query = TextQuery::new(q);
        let timestamp = get_current_timestamp();
        println!("Timestamp={}", timestamp);

        let user_id = "test_user";
        let request_id = "deadbeef";
        let mut headers =
            self.build_auth_headers(user_id.to_string(), request_id.to_string(), timestamp);

        let url = query.get_url(self.api_url.clone());
        for (k, v) in query
            .get_headers(
                self.client_id.clone(),
                user_id.to_string(),
                request_id.to_string(),
                timestamp,
            )
            .iter()
        {
            headers.insert(k.clone(), v.clone());
        }
        let req = self.http_client.get(&url).headers(headers);
        println!("{:#?}", req);
        let mut res = req.send().unwrap();
        println!("{:#?}", res);
        res.copy_to(&mut std::io::stdout()).unwrap();
    }
}
