use houndify::*;
use std::fs::File;
use std::io::BufReader;

fn get_client() -> Client {
    let client_id = "EqQpJDGt0YozIb8Az6xvvA==";
    let client_key =
        "jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==";

    Client::new(
        DEFAULT_API_ENDPOINT,
        client_id,
        client_key,
        Some(|| String::from("deadbeef")),
    )
}

#[test]
fn test_text_query() {
    let c = get_client();
    let query = TextQuery::new("what is one plus one?", "kevinq", RequestInfo::new());
    let resp = c.text_query(query);
    match resp {
        Ok(r) => {
            assert_eq!(r.status, "OK");
        },
        Err(e) => {
            println!("Error={}", e);
            assert!(false);
        }
    }
}

#[test]
fn test_voice_query_success() {
    let c = get_client();
    let file = File::open("tests/whattimeisit.wav").unwrap();
    let buf = BufReader::new(file);
    let query = VoiceQuery::new(Box::new(buf), "kevinq", RequestInfo::new());
    let resp = c.voice_query(query);
    match resp {
        Ok(r) => {
            assert_eq!(r.status, "OK");
        },
        Err(e) => {
            println!("Error={}", e);
            assert!(false);
        },
    }
}

#[test]
fn test_voice_query_unsupported_audio_format() {
    let c = get_client();
    let file = File::open("tests/whattimeisit.mp3").unwrap();
    let buf = BufReader::new(file);
    let query = VoiceQuery::new(Box::new(buf), "kevinq", RequestInfo::new());
    let resp = c.voice_query(query);
    match resp {
        Ok(r) => {
            assert_eq!(r.status, "Error");
            match r.error_message {
                Some(m) => println!("ErrorMessage={}...", &m[..50]),
                None => print!("None"),
            }
        },
        Err(e) => {
            println!("Error={}", e);
            assert!(false)
        }
    }
}
