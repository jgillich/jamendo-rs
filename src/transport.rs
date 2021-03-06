use std::io::Read;
use hyper;
use url;
use Response;
use serde_json;
use Error;
use ErrorKind;
use serde::de::Deserialize;

#[derive(Debug)]
#[doc(hidden)]
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

    pub fn get<T: Deserialize>(&self, path: &str, query_pairs: Vec<(&str, String)>) -> Result<Response<T>, Error> {
        let url = self.make_url(path, query_pairs);
        let mut res = try!(self.hyper_client.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let response: Response<T> = try!(serde_json::from_str(&body));

        if response.headers.code == 0 {
            Ok(response)
        } else {
            Err(Error::from(ErrorKind::Api((response.headers.code, response.headers.error_message))))
        }
    }

    fn make_url(&self, path: &str, mut query_pairs: Vec<(&str, String)>) -> url::Url {
        let mut url = self.base_url.clone();
        let path = format!("{}/{}", url.path(), path);

        url.set_path(&path);

        query_pairs.push(("client_id", self.client_id.clone()));
        query_pairs.push(("format", "json".to_string()));

        url.query_pairs_mut().extend_pairs(query_pairs);

        url
    }
}