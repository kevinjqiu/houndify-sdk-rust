use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DomainUsage {
    /// This field gives the title of a domain that was used by this query.
    pub domain: String,

    /// This field gives the unique ID of a domain that was used by this query.
    #[serde(rename = "DomainUniqueID")]
    pub domain_unique_id: String,

    /// This field species how many credits the use of this domain cost.
    pub credits_used: f64,
}

/// Reference: https://docs.houndify.com/reference/BuildInfo
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BuildInfo {
    /// This is the user ID that built this server executable.
    pub user: Option<String>,

    /// This is a human-readable date and time stamp for when the server executable was built.
    pub date: Option<String>,

    /// This is the name of the machine on which the server executable was built.
    pub machine: Option<String>,

    /// This is the SVN revision number of the sources from which the server executable was built.
    #[serde(rename = "SVNRevision")]
    pub svn_revision: Option<String>,

    /// This is the SVN branch of the sources from which the server executable was built.
    #[serde(rename = "SVNBranch")]
    pub svn_branch: Option<String>,

    /// This is the build system build number of the build that built the server executable, if it was built by an automated build and test system.
    pub build_number: Option<String>,

    /// If the server executable as built by an automated build and test system that does different kinds of builds, this is the kind of build that built it. Typical examples are "Low Fat" and "Full".
    pub kind: Option<String>,

    /// This is the kind of executable that generate the result. Typical examples are "release", "debug", "profile", etc.
    pub variant: Option<String>,
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DisambiguationChoice {
    /// This gives the transcription that led to the corresponding result in "AllResults". Sometimes, the different results in "AllResults" will come from different transcriptions of the audio and sometimes the different results will come from different parses of the same transcription. It is provided here so it can be presented to the user as one of the things we think he or she said.
    pub transcription: String,
    /// This gives an idea of how confident the server is that this particular interpretation of the query from the user is correct. It combines both information about how likely the server thinks it is the transcription is correct and how likely the server thinks it is that the parse of that transcription is correct.
    pub confidence_score: i64,
    /// This field contains a version of the transcription formatted for easier reading. This is in contrast to the "Transcription" field which only contains the raw words.
    pub formatted_transcription: String,
    /// Under some circumstances, the server will make corrections to the transcription to give what it believes the user meant. This field contains this fixed transcription.
    pub fixed_transcription: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Disambiguation {
    /// This field tells the client how many of the results from "AllResults" should be shown to the user, for the user to choose among. If it is 1, then the server is suggesting not giving the user a choice and instead just showing the first result in "AllResults". If it is greater than one, the server is suggesting showing that many results and asking the user to choose among them. The elements of "AllResults" are ordered in preference from most prefered to least, so if this field has the value 3, the server is suggesting using the first three results from "AllResults".
    pub num_to_show: i64,

    /// This is an array with the number of elements specified by "NumToShow". For each element, it gives information for the corresponding element of "AllResults". This is information about that result that can be used to inform the disambiguous decision about which result to use.
    pub choice_data: DisambiguationChoice,
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
    pub results_are_final: Option<Vec<bool>>,

    /// This field specifies which domains, if any, were used to generate the responses, and for each that was used to generate the responses, how many credits that domain cost. Note that this aggregates the domain usage and costs for all the responses in AllResults, so if there were multiple results, the total cost for some domains may be higher than the maximum cost of that domain per response.
    pub domain_usage: Option<Vec<DomainUsage>>,

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
        assert!(true);
    }
}
