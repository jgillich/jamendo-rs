use Transport;
use Artist;
use std::collections::HashMap;
use Error;


pub struct GetArtists<'a> {
    transport: &'a Transport,
    query: HashMap<String, String>,
}

impl<'a> GetArtists<'a> {
    pub fn new(transport: &'a Transport) -> Self {
        GetArtists {
            transport: transport,
            query: HashMap::new(),
        }
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.query.insert("offset".to_string(), offset.to_string());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.query.insert("limit".to_string(), limit.to_string());
        self
    }

    // TODO order

    pub fn id(mut self, id: i32) -> Self {
        self.query.insert("id".to_string(), id.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.query.insert("name".to_string(), name.to_string());
        self
    }

    pub fn namesearch(mut self, namesearch: &str) -> Self {
        self.query.insert("namesearch".to_string(), namesearch.to_string());
        self
    }

    pub fn hasimage(mut self, hasimage: bool) -> Self {
        self.query.insert("hasimage".to_string(), hasimage.to_string());
        self
    }

    pub fn run(self) -> Result<Vec<Artist>, Error> {
        Ok(self.transport.get::<Artist>("artists", self.query)?.results)
    }

    pub fn unwrap(self) -> Vec<Artist> {
        self.run().unwrap()
    }
}
