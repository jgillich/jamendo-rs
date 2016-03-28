#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub name: String,
    pub website: String,
    pub joindate: String,
    pub image: String,
    pub shorturl: String,
    pub shareurl: String,
}
