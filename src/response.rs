use serde_json::{Map, String, Value}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoundServerResponse {
    format: str,
    format_version: str,
    status: str,
    error_message: Option<str>,
    num_to_return: Option<i64>,
    all_results: Option<Map<String, Value>>,
    disambiguation: Option<Map<String, Value>>,
    results_are_final: Option<Vec<Value>>,
    domain_usage: Option<Vec<Value>>,
    build_info: Option<Map<String, Value>>,
    query_id: str,
    server_generated_id: Option<str>,
    audio_length: Option<f64>,
    real_speech_time: Option<f64>,
    cpu_speech_time: Option<f64>,
    real_time: Option<f64>,
    cpu_time: Option<f64>,
    local_or_remote: Option<str>,
    local_or_remote_reason: Option<str>,
}

#[cfg(test)]
mod response_tests {
    use super::*;

    #[test]
    fn test_hound_server_response_parse() {
        let file = File::open("tests/sample_result.json").unwrap();
        let hsr: HoundServerResponse = serde_json::from_reader(file).unwrap();
        println!("{}", hsr);
        assert_eq!(1, 1);
    }
}
