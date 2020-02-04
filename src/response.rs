use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HoundServerResponse {
    format: String,
    format_version: String,
    status: String,
    error_message: Option<String>,
    num_to_return: Option<i64>,
    all_results: Option<Vec<Map<String, Value>>>,
    disambiguation: Option<Map<String, Value>>,
    results_are_final: Option<Vec<Value>>,
    domain_usage: Option<Vec<Value>>,
    build_info: Option<Map<String, Value>>,
    #[serde(rename = "QueryID")]
    query_id: String,
    server_generated_id: Option<String>,
    audio_length: Option<f64>,
    real_speech_time: Option<f64>,
    cpu_speech_time: Option<f64>,
    real_time: Option<f64>,
    cpu_time: Option<f64>,
    local_or_remote: Option<String>,
    local_or_remote_reason: Option<String>,
}

#[cfg(test)]
mod response_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_hound_server_response_parse() {
        let file = std::fs::File::open("tests/sample_result.json").unwrap();
        let hsr: HoundServerResponse = serde_json::from_reader(file).unwrap();
        println!("{:#?}", hsr);
        assert_eq!(1, 1);
    }
}
