use crate::error::{HoundifyError, InvalidRequestInfoError};
use crate::Result;
use serde_json::{Map, Number, Value};
use url::form_urlencoded;

pub trait Query {
    fn get_url(&self, api_url: &str) -> String;
}

#[derive(Debug, Clone)]
pub struct RequestInfo {
    request_info_map: Map<String, Value>,
}

impl RequestInfo {
    pub fn new() -> Self {
        RequestInfo {
            request_info_map: Map::new(),
        }
    }

    /// Set the latitude of the request
    pub fn latitude(&mut self, v: f64) -> Option<InvalidRequestInfoError> {
        if v < -90.0 || v > 90.0 {
            return Some(InvalidRequestInfoError::new(
                "Latitude must between -90 and 90",
            ));
        }
        let n = match Number::from_f64(v) {
            Some(n) => n,
            None => return Some(InvalidRequestInfoError::new("Cannot parse latitude")),
        };
        &self
            .request_info_map
            .insert("Latitude".to_string(), Value::Number(n));
        None
    }

    /// Set the longitude of the request
    pub fn longitude(&mut self, v: f64) -> Option<InvalidRequestInfoError> {
        if v < -180.0 || v > 180.0 {
            return Some(InvalidRequestInfoError::new(
                "Longitude must between -180 and 180",
            ));
        }
        let n = match Number::from_f64(v) {
            Some(n) => n,
            None => return Some(InvalidRequestInfoError::new("Cannot parse longitude")),
        };
        &self
            .request_info_map
            .insert("Longitude".to_string(), Value::Number(n));
        None
    }

    /// Set timestamp
    pub fn timestamp(&mut self, v: u64) -> Option<InvalidRequestInfoError> {
        &self
            .request_info_map
            .insert("TimeStamp".to_string(), Value::Number(Number::from(v)));
        None
    }

    /// Set ClientID
    pub fn client_id(&mut self, v: &str) -> Option<InvalidRequestInfoError> {
        &self
            .request_info_map
            .insert("ClientID".to_string(), Value::String(v.to_string()));
        None
    }

    /// Set PositionTime
    pub fn position_time(&mut self, v: u64) -> Option<InvalidRequestInfoError> {
        &self
            .request_info_map
            .insert("PositionTime".to_string(), Value::Number(Number::from(v)));
        None

    }

    /// Set arbitrary RequestInfo
    pub fn set(&mut self, k: String, v: Value) -> Option<InvalidRequestInfoError> {
        &self.request_info_map.insert(k, v);
        None
    }

    pub fn serialize(self) -> Result<String> {
        match serde_json::to_string(&self.request_info_map) {
            Ok(j) => Ok(j),
            Err(e) => Err(HoundifyError::new(e.into())),
        }
    }
}

#[derive(Debug)]
pub struct TextQuery<'a> {
    pub(crate) query: &'a str,
    pub(crate) user_id: &'a str,
    pub(crate) request_info: RequestInfo,
}

impl<'a> TextQuery<'a> {
    pub fn new(query: &'a str, user_id: &'a str, mut request_info: RequestInfo) -> TextQuery<'a> {
        request_info.set(
            "SDK".to_string(),
            Value::String("houndify-sdk-rust/1.0".to_string()),
        ); // TODO: get the SDK version from manifest?
        request_info.set("UserID".to_string(), Value::String(user_id.to_string()));
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

pub struct VoiceQuery<'a> {
    pub(crate) audio_stream: Box<dyn std::io::Read + Send>,
    pub(crate) user_id: &'a str,
    pub(crate) request_info: RequestInfo,
}

impl Query for VoiceQuery<'_> {
    fn get_url(&self, api_url: &str) -> String {
        return format!("{}v1/audio", api_url);
    }
}

impl<'a> VoiceQuery<'a> {
    pub fn new(audio_stream: Box<dyn std::io::Read + Send>, user_id: &'a str, mut request_info: RequestInfo) -> Self {
        VoiceQuery {
            audio_stream,
            user_id,
            request_info,
        }
    }
}