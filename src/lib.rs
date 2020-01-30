pub use crate::client::{Client, Result};
pub use crate::query::QueryOptions;
pub use crate::error::HoundifyError;

mod client;
mod query;
mod error;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_text_query() {
        let client_id = "EqQpJDGt0YozIb8Az6xvvA==";
        let client_key = "jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==";
        let api_base = "https://api.houndify.com/";

        let c = Client::new(api_base, client_id, client_key);
        let mut options = QueryOptions::new();
        &options.user_id("kevinq");
        let resp = c.text_query("what is one plus one?", &options);
        match resp {
            Ok(r) => println!("{}", r),
            Err(e) => println!("Error={}", e),
        }
    }
}
