pub use crate::client::Client;

mod client;
mod query;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let client_id = String::from("EqQpJDGt0YozIb8Az6xvvA==");
        let client_key = String::from("jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==");
        let api_base = String::from("https://api.houndify.com/");

        let c = Client::new(api_base, client_id, client_key);
        // let q = query::TextQuery::new(String::from("what is 1+1?"));
        let q = "what is 1 + 1?".to_string();
        c.text_query(q);
    }
}
