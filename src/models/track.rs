#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub duration: i32,
    pub artist_id: String,
    pub artist_name: String,
    pub artist_idstr: String,
    pub album_name: String,
    pub album_id: String,
    pub license_ccurl: String,
    pub position: i32,
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
