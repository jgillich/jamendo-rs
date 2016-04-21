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
