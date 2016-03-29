use Transport;
use Track;
use std::collections::HashMap;
use Error;


pub struct GetTracks<'a> {
    transport: &'a Transport,
    query: HashMap<String, String>,
}

impl<'a> GetTracks<'a> {
    pub fn new(transport: &'a Transport) -> Self {
        GetTracks {
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

    // TODO type

    pub fn album_id(mut self, album_id: i32) -> Self {
        self.query.insert("album_id".to_string(), album_id.to_string());
        self
    }

    pub fn album_name(mut self, album_name: &str) -> Self {
        self.query.insert("album_name".to_string(), album_name.to_string());
        self
    }

    pub fn artist_id(mut self, artist_id: i32) -> Self {
        self.query.insert("artist_id".to_string(), artist_id.to_string());
        self
    }

    pub fn artist_name(mut self, artist_name: &str) -> Self {
        self.query.insert("artist_name".to_string(), artist_name.to_string());
        self
    }

    // TODO datebetween

    pub fn featured(mut self, featured: bool) -> Self {
        self.query.insert("featured".to_string(), featured.to_string());
        self
    }

    // TODO imagesize
    // TODO audioformat
    // TODO audiodlformat
    // TODO tags
    // TODO fuzzytags
    // TODO acousticelectric
    // TODO vocalinstrumental
    // TODO gender
    // TODO speed
    // TODO lang
    // TODO durationbetween
    // TODO xartist
    // TODO search
    // TODO prolicensing
    // TODO probackground
    // TODO ccsa
    // TODO ccnd
    // TODO ccnc
    // TODO include
    // TODO groupby
    // TODO boost

    pub fn run(self) -> Result<Vec<Track>, Error> {
        Ok(self.transport.get::<Track>("tracks", self.query)?.results)
    }

    pub fn unwrap(self) -> Vec<Track> {
        self.run().unwrap()
    }
}
