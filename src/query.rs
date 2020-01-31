use url::form_urlencoded;

pub trait Query {
    fn get_url(&self, api_url: &str) -> String;
}

#[derive(Debug)]
pub struct TextQuery<'a> {
    pub(crate) query: &'a str,
    pub(crate) user_id: &'a str,
}

impl <'a> TextQuery<'a> {
    pub fn new(query: &'a str, user_id: &'a str) -> TextQuery <'a> {
        TextQuery {
            query,
            user_id,
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
