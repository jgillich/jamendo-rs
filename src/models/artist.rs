#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub website: String,
    pub joindate: String,
    pub image: String,
    pub shorturl: String,
    pub shareurl: String,
}
