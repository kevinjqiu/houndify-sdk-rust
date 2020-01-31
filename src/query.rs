use url::form_urlencoded;
use serde_json::{Map, Value, Number};
use crate::error::InvalidRequestInfoError;

pub trait Query {
    fn get_url(&self, api_url: &str) -> String;
}

#[derive(Debug)]
pub struct RequestInfo {
    request_info_map: Map<String, Value>,
}

impl RequestInfo {
    pub fn new() -> Self {
        RequestInfo{
            request_info_map: Map::new(),
        }
    }

    /// Set the latitude of the request
    pub fn latitude(&mut self, v: f64) -> Option<InvalidRequestInfoError> {
        if v < -90.0 || v > 90.0 {
            return Some(InvalidRequestInfoError::new("Latitude must between -90 and 90"));
        }
        let n = match Number::from_f64(v) {
            Some(n) => n,
            None => return Some(InvalidRequestInfoError::new("Cannot parse latitude")),
        };
        &self.request_info_map.insert("Latitude".to_string(), Value::Number(n));
        None
    }

    /// Set the longitude of the request
    pub fn longitude(&mut self, v: f64) -> Option<InvalidRequestInfoError> {
        if v < -180.0 || v > 180.0 {
            return Some(InvalidRequestInfoError::new("Longitude must between -180 and 180"));
        }
        let n = match Number::from_f64(v) {
            Some(n) => n,
            None => return Some(InvalidRequestInfoError::new("Cannot parse longitude")),
        };
        &self.request_info_map.insert("Longitude".to_string(), Value::Number(n));
        None
    }

    /// Set arbitrary RequestInfo
    pub fn set(&mut self, k: String, v: Value) -> Option<InvalidRequestInfoError> {
        &self.request_info_map.insert(k, v);
        None
    }
}

#[derive(Debug)]
pub struct TextQuery<'a> {
    pub(crate) query: &'a str,
    pub(crate) user_id: &'a str,
    pub(crate) request_info: RequestInfo,
}

impl <'a> TextQuery<'a> {
    pub fn new(query: &'a str, user_id: &'a str, request_info: RequestInfo) -> TextQuery <'a> {
        TextQuery {
            query,
            user_id,
            request_info,
        }
    }
}

impl Query for TextQuery<'_> {
    fn get_url(&self, api_url: &str) -> String {
        let url: String = form_urlencoded::Serializer::new(format!("{}v1/text?", api_url))
            .append_pair("query", &self.query)
            .finish();
        url
    }
}
