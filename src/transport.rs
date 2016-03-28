use hyper;
use url;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Transport {
    client_id: String,
    base_url: url::Url,
    hyper_client: hyper::Client,
}

impl Transport {
    pub fn new(base_url: url::Url, client_id: &str) -> Transport {
        Transport {
            base_url: base_url,
            client_id: client_id.to_string(),
            hyper_client: hyper::Client::new(),
        }
    }

    pub fn get(&self, path: &str, mut query_pairs: HashMap<String, String>) -> hyper::client::RequestBuilder {
        let url = self.make_url(path, query_pairs);
        self.hyper_client.get(url)
    }

    fn make_url(&self, path: &str, mut query_pairs: HashMap<String, String>) -> url::Url {
        let mut url = self.base_url.clone();

        url.path_mut().unwrap().push(path.to_string());

        query_pairs.insert("client_id".to_string(), self.client_id.clone());
        query_pairs.insert("format".to_string(), "json".to_string());

        url.set_query_from_pairs(query_pairs);

        url
    }
}
