use url::Url;
use {Transport};
use {Action, Resource, Album, Artist, Track};

const API_URL: &'static str = "https://api.jamendo.com/v3.0";

/// The Client is used to create actions
pub struct Client {
    transport: Transport,
}

impl Client {

    pub fn new(client_id: &str) -> Client {
        Client {
            transport: Transport::new(Url::parse(API_URL).unwrap(), client_id),
        }
    }

    pub fn get_albums(&self) -> Action<Album> {
        Action::new(&self.transport, Resource::GetAlbums)
    }

    pub fn get_users_albums(&self) -> Action<Album> {
        Action::new(&self.transport, Resource::GetUsersAlbums)
    }

    pub fn get_artists(&self) -> Action<Artist> {
        Action::new(&self.transport, Resource::GetArtists)
    }

    pub fn get_users_artists(&self) -> Action<Artist> {
        Action::new(&self.transport, Resource::GetUsersArtists)
    }

    pub fn get_tracks(&self) -> Action<Track> {
        Action::new(&self.transport, Resource::GetTracks)
    }

    pub fn get_users_tracks(&self) -> Action<Track> {
        Action::new(&self.transport, Resource::GetUsersTracks)
    }

}
