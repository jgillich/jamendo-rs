use url::Url;
use {Transport, Action, Resource, User, Album, Artist, Track, UserAlbums, UserArtists, UserTracks};

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

    pub fn get_users(&self) -> Action<User> {
        Action::new(&self.transport, Resource::GetUsers)
    }

    pub fn get_albums(&self) -> Action<Album> {
        Action::new(&self.transport, Resource::GetAlbums)
    }

    pub fn get_users_albums(&self) -> Action<UserAlbums> {
        Action::new(&self.transport, Resource::GetUsersAlbums)
    }

    pub fn get_artists(&self) -> Action<Artist> {
        Action::new(&self.transport, Resource::GetArtists)
    }

    pub fn get_users_artists(&self) -> Action<UserArtists> {
        Action::new(&self.transport, Resource::GetUsersArtists)
    }

    pub fn get_tracks(&self) -> Action<Track> {
        Action::new(&self.transport, Resource::GetTracks)
    }

    pub fn get_users_tracks(&self) -> Action<UserTracks> {
        Action::new(&self.transport, Resource::GetUsersTracks)
    }

}
