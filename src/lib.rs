pub use crate::client::Client;

mod client;
mod query;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_auth_values() {
        let client_id = String::from("EqQpJDGt0YozIb8Az6xvvA==");
        let client_key = String::from("jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==");
        let api_base = String::from("https://api.houndify.com/");

        let c = Client::new(api_base, client_id, client_key);
        let auth_info = &c.generate_auth_values(String::from("test_user"), String::from("deadbeef"), 1580278266);
        println!("{:?}", auth_info);
        assert_eq!(auth_info.hound_client_auth, "EqQpJDGt0YozIb8Az6xvvA==;1580278266;Ix3_MpLnyz1jGEV5g-mXxmbfgfZ85rD8-6S6yRTJEag=");
    }

    #[test]
    fn it_works() {
        let client_id = String::from("EqQpJDGt0YozIb8Az6xvvA==");
        let client_key = String::from("jLTVjUOFBSetQtA3l-lGlb75rPVqKmH_JFgOVZjl4BdJqOq7PwUpub8ROcNnXUTssqd6M_7rC8Jn3_FjITouxQ==");
        let api_base = String::from("https://api.houndify.com/");

        let c = Client::new(api_base, client_id, client_key);
        let q = query::TextQuery::new(String::from("what is 1+1?"));
        c.text_query(q);
    }
}
