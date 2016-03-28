use url::Url;
use Transport;
use actions;

const API_URL: &'static str = "https://api.jamendo.com/v3.0";

pub struct Client {
    transport: Transport,
}

impl Client {
    pub fn new(client_id: &str) -> Client {
        Client {
            transport: Transport::new(Url::parse(API_URL).unwrap(), client_id),
        }
    }

    pub fn get_albums(&self) -> actions::GetAlbums {
        actions::GetAlbums::new(&self.transport)
    }

    pub fn get_tracks(&self) -> actions::GetTracks {
        actions::GetTracks::new(&self.transport)
    }
}
