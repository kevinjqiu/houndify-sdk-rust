use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Map, Value};

/// Reference: https://docs.houndify.com/reference/BuildInfo
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BuildInfo {
    /// This is the user ID that built this server executable.
    user: Option<String>,

    /// This is a human-readable date and time stamp for when the server executable was built.
    date: Option<String>,

    /// This is the name of the machine on which the server executable was built.
    machine: Option<String>,

    /// This is the SVN revision number of the sources from which the server executable was built.
    #[serde(rename = "SVNRevision")]
    svn_revision: Option<String>,

    /// This is the SVN branch of the sources from which the server executable was built.
    #[serde(rename = "SVNBranch")]
    svn_branch: Option<String>,

    /// This is the build system build number of the build that built the server executable, if it was built by an automated build and test system.
    build_number: Option<String>,

    /// If the server executable as built by an automated build and test system that does different kinds of builds, this is the kind of build that built it. Typical examples are "Low Fat" and "Full".
    kind: Option<String>,

    /// This is the kind of executable that generate the result. Typical examples are "release", "debug", "profile", etc.
    variant: Option<String>,
}

/// Reference: https://docs.houndify.com/reference/HoundServer
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HoundServerResponse {
    /// This field is a marker to verify that this is a value in this format.
    pub format: String,

    /// This field specifies which version of this format is being used. The current version is 1.0. If a different version string is here, the format might not match what is documented here.
    pub format_version: String,

    /// This field uses only a fixed, finite number of JSON strings to encode an enumeration.
    pub status: String,

    /// If the "Status" was "Error", then this field will be present to give a human-readable explanation of what went wrong.
    pub error_message: Option<String>,

    /// If the "Status" was "OK", then the server received a proper request and processed it, and this field will be present to specify how many possible results it is returning.
    pub num_to_return: Option<i64>,

    /// If the "Status" was "OK", then the server received a proper request and processed it, and this field will be present to specify the possible results it came up with. The number of elements in the array will be equal to the value of the "NumToReturn" field.
    pub all_results: Option<Vec<Map<String, Value>>>,

    /// If the "Status" was "OK", then the server received a proper request and processed it, and this field will be present to specify information to help the client do disambiguous among the results being returned by the server in the "AllResults" array.
    pub disambiguation: Option<Map<String, Value>>,

    /// This field must have exactly the same number of elements as the "AllResults" field, and the elements of "ResultsAreFinal" correspond to the elements of "AllResults". If the client didn't set "ResultUpdateAllowed" in the RequestInfo, or set it to false, then the values in this array will always be true. If the client set "ResultUpdateAllowed" to true in the RequestInfo, then some or all of these elements may be false. Each one that is false means the corresponding element of "AllResults" is not final and will be updated by a HoundUpdate object. If any element of this array is false, the client should keep the connection open and wait for HoundUpdate objects to come, even while it is displaying the preliminary information to the user.
    pub results_are_final: Option<Vec<Value>>,

    /// This field specifies which domains, if any, were used to generate the responses, and for each that was used to generate the responses, how many credits that domain cost. Note that this aggregates the domain usage and costs for all the responses in AllResults, so if there were multiple results, the total cost for some domains may be higher than the maximum cost of that domain per response.
    pub domain_usage: Option<Vec<Value>>,

    /// This field is used to send information about the details of the build of the server that generated this result. It is not required but highly recommended that each server put as much information here as possible. The data here shouldn't be relied on by the client. It is intended to help in debugging issues. If possible, all the information from this field should be included in bug reports.
    pub build_info: Option<BuildInfo>,

    /// This is a string that can be used to track this query for debugging purposes on the server. Each query is assigned a unique ID string.
    #[serde(rename = "QueryID")]
    pub query_id: String,

    /// This is a deprecated copy of the "QueryID" string. It should not be counted on and it will be removed in the future.
    pub server_generated_id: Option<String>,

    /// This field is only present if the input is speech. It is the length of the audio input in seconds.
    pub audio_length: Option<f64>,

    /// This field is only present if the input is speech. It is the amount of real time, in seconds, spent processing the speech.
    pub real_speech_time: Option<f64>,

    /// This field is only present if the input is speech. It is the amount of CPU time, in seconds, spent processing the speech.
    pub cpu_speech_time: Option<f64>,

    /// This field is the amount of real time, in seconds, spent processing the query.
    pub real_time: Option<f64>,

    /// This field is the amount of CPU time, in seconds, spent processing the query.
    pub cpu_time: Option<f64>,

    /// If the Hound server returning this JSON is a dual server that can either service requests locally or use a remote server to service them, then it can set this field to indicate whether the result it is sending came from its local engine or a remote engine.
    pub local_or_remote: Option<String>,

    /// If the Hound server returning this JSON is a dual server that can either service requests locally or use a remote server to service them, then it can set this field to indicate why it chose the result from its local engine or a remote engine.
    pub local_or_remote_reason: Option<String>,
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
