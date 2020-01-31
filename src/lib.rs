pub use crate::client::{Client, Result};
pub use crate::error::{HoundifyError, InvalidRequestInfoError};
pub use crate::query::{RequestInfo, TextQuery};

mod client;
mod error;
mod query;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_text_query() {
        let client_id = "EqQpJDGt0YozIb8Az6xvvA==";
        let client_key = "jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==";
        let api_base = "https://api.houndify.com/";

        let c = Client::new(
            api_base,
            client_id,
            client_key,
            Some(|| String::from("deadbeef")),
        );
        let query = TextQuery::new("what is one plus one?", "kevinq", RequestInfo::new());
        let resp = c.text_query(query);
        match resp {
            Ok(r) => println!("{}", r),
            Err(e) => println!("Error={}", e),
        }
    }
}
