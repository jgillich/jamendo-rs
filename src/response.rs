#[derive(Serialize, Deserialize, Debug)]
#[doc(hidden)]
pub struct Response<T> {
    pub headers: ResponseHeaders,
    pub results: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
#[doc(hidden)]
pub struct ResponseHeaders {
    pub status: String,
    pub code: u32,
    pub error_message: String,
    pub warnings: String,
    pub results_count: u32,
}

/// A album resource
#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub id: u32,
    pub name: String,
    pub releasedate: String,
    pub artist_id: String,
    pub artist_name: String,
    pub image: String,
    pub zip: String,
    pub shorturl: String,
    pub shareurl: String,
}

/// A artist resource
#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub id: u32,
    pub name: String,
    pub website: String,
    pub joindate: String,
    pub image: String,
    pub shorturl: String,
    pub shareurl: String,
}

/// A track resource
#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub id: u32,
    pub name: String,
    pub duration: u32,
    pub artist_id: String,
    pub artist_name: String,
    pub artist_idstr: String,
    pub album_name: String,
    pub album_id: String,
    pub license_ccurl: String,
    pub position: u32,
    pub releasedate: String,
    pub album_image: String,
    pub audio: String,
    pub audiodownload: String,
    pub prourl: String,
    pub shorturl: String,
    pub shareurl: String,
    pub image: String,
    //pub musicinfo:
}

/// Album returned by users/albums
#[derive(Serialize, Deserialize, Debug)]
pub struct PartialAlbum {
    pub id: u32,
    pub name: String,
    pub releasedate: String,
    pub artist_id: u32,
    pub artist_name: String,
    pub updatedate: String,
    pub image: String,
    //TODO pub relations: Relations
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersAlbums {
    pub name: String,
    pub dispname: String,
    pub id: u32,
    pub lang: String,
    pub creationdate: String,
    pub image: String,
    pub albums: Vec<PartialAlbum>
}

/// Artist returned by users/artists
#[derive(Serialize, Deserialize, Debug)]
pub struct PartialArtist {
    pub id: u32,
    pub name: String,
    pub image: String,
    pub joindate: String,
    pub updatedate: String,
    //TODO pub relations: Relations
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersArtists {
    pub name: String,
    pub dispname: String,
    pub id: u32,
    pub lang: String,
    pub creationdate: String,
    pub image: String,
    pub artists: Vec<PartialArtist>
}

/// Track returned by users/tracks
#[derive(Serialize, Deserialize, Debug)]
pub struct PartialTrack {
    pub id: u32,
    pub name: String,
    pub album_id: u32,
    pub artist_id: u32,
    pub duration: u32,
    pub license_ccurl: String,
    pub updatedate: String,
    pub album_image: String,
    pub image: String,
    pub audio: String,
    pub audiodownload: String,
    pub relations: TrackRelations,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackRelations {
    pub review: u32,
    pub favorite: u32,
    pub like: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersTracks {
    pub name: String,
    pub dispname: String,
    pub id: u32,
    pub lang: String,
    pub creationdate: String,
    pub tracks: Vec<PartialTrack>
}
