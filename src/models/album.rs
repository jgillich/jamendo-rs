#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub releasedate: String,
    pub artist_id: String,
    pub artist_name: String,
    pub image: String,
    pub zip: String,
    pub shorturl: String,
    pub shareurl: String,
}
