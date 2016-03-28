use Transport;
use Response;
use Album;
use std::collections::HashMap;
use Error;

pub struct GetAlbums<'a> {
    transport: &'a Transport,
    query_params: HashMap<String, String>,
}

impl<'a> GetAlbums<'a> {
    pub fn new(transport: &'a Transport) -> Self {
        GetAlbums {
            transport: transport,
            query_params: HashMap::new(),
        }
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.query_params.insert("offset".to_string(), offset.to_string());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.query_params.insert("limit".to_string(), limit.to_string());
        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.query_params.insert("id".to_string(), id.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.query_params.insert("name".to_string(), name.to_string());
        self
    }

    pub fn namesearch(mut self, namesearch: &str) -> Self {
        self.query_params.insert("namesearch".to_string(), namesearch.to_string());
        self
    }

    // TODO artist_id
    // TODO artist_name
    // TODO datebetween
    // TODO imagesize
    // TODO audioformat

    pub fn run(self) -> Result<Response<Album>, Error> {
        self.transport.get("albums", self.query_params)
    }

    pub fn unwrap(self) -> Response<Album> {
        self.run().unwrap()
    }
}
