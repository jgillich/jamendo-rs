use serde;
use std::marker::PhantomData;
use {Transport, Error, ErrorKind};

/// Action represents the request to a resource
pub struct Action<'a, T> where T: 'a + serde::de::Deserialize {
    transport: &'a Transport,
    resource: Resource,
    query: Vec<Query>,
    response_type: PhantomData<&'a T>
}

impl<'a, T> Action<'a, T> where T: 'a + serde::de::Deserialize {
    #[doc(hidden)]
    pub fn new(transport: &'a Transport, resource: Resource) -> Self {
        Action {
            transport: transport,
            resource: resource,
            query: Vec::new(),
            response_type: PhantomData,
        }
    }

    pub fn set(mut self, query: Query) -> Self {
        self.query.push(query);
        self
    }

    pub fn user_id(self, v: u32) -> Self { self.set(Query::UserId(v)) }
    pub fn offset(self, v: u32) -> Self { self.set(Query::Offset(v)) }
    pub fn limit(self, v: u32) -> Self { self.set(Query::Limit(v)) }
    pub fn album_id(self, v: u32) -> Self { self.set(Query::AlbumId(v)) }
    pub fn artist_id(self, v: u32) -> Self { self.set(Query::ArtistId(v)) }
    pub fn track_id(self, v: u32) -> Self { self.set(Query::TrackId(v)) }
    pub fn album_name(self, v: &str) -> Self { self.set(Query::AlbumName(v.into())) }
    pub fn artist_name(self, v: &str) -> Self { self.set(Query::ArtistName(v.into())) }
    pub fn track_name(self, v: &str) -> Self { self.set(Query::TrackName(v.into())) }
    pub fn name_search(self, v: &str) -> Self { self.set(Query::NameSearch(v.into())) }
    pub fn featured(self, v: bool) -> Self { self.set(Query::Featured(v)) }

    pub fn run(self) -> Result<Vec<T>, Error> {
        use Resource::*;
        use Query::*;

        let path = match self.resource {
            GetAlbums => "albums",
            GetUsersAlbums => "users/albums",
            GetArtists => "artists",
            GetUsersArtists => "users/artists",
            Resource::GetTracks => "tracks",
            GetUsersTracks => "users/tracks",
        };

        let mut query_pairs: Vec<(&str, String)> = Vec::new();
        for q in self.query.iter() {
            query_pairs.push(match q {
                &UserId(ref v) => match self.resource {
                    GetUsersAlbums | GetUsersArtists | GetUsersTracks => ("user_id", v.to_string()),
                    _ => return Err(Error::Client(ErrorKind::InvalidQuery)),
                },
                &Offset(ref v) => ("offset", v.to_string()),
                &Limit(ref v) => ("limit", v.to_string()),
                &AlbumId(ref v) => match self.resource {
                    GetAlbums => ("id", v.to_string()),
                    _ => ("album_id", v.to_string()),
                },
                &ArtistId(ref v) => match self.resource {
                    GetArtists => ("id", v.to_string()),
                    _ => ("artist_id", v.to_string()),
                },
                &TrackId(ref v) => match self.resource {
                    GetTracks => ("id", v.to_string()),
                    _ => ("track_id", v.to_string()),
                },
                &AlbumName(ref v) => match self.resource {
                    GetAlbums => ("name", v.clone()),
                    _ => ("album_name", v.clone()),
                },
                &ArtistName(ref v) => match self.resource {
                    GetArtists => ("name", v.clone()),
                    _ => ("artist_name", v.clone()),
                },
                &TrackName(ref v) => match self.resource {
                    GetTracks => ("name", v.clone()),
                    _ => ("track_name", v.clone()),
                },
                &NameSearch(ref v) => ("namesearch", v.clone()),
                &Featured(ref v) => ("featured", v.to_string()),
            });
        }

        Ok(self.transport.get(path, query_pairs)?.results)
    }

    pub fn unwrap(self) -> Vec<T> {
        self.run().unwrap()
    }
}

#[doc(hidden)]
pub enum Resource {
    GetAlbums,
    GetUsersAlbums,
    GetArtists,
    GetUsersArtists,
    GetTracks,
    GetUsersTracks,
}

/// A query parameter
pub enum Query {
    UserId(u32),
    Offset(u32),
    Limit(u32),
    AlbumId(u32),
    ArtistId(u32),
    TrackId(u32),
    AlbumName(String),
    ArtistName(String),
    TrackName(String),
    NameSearch(String),
    Featured(bool),
}