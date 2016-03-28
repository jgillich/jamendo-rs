#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub headers: ResponseHeaders,
    pub results: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseHeaders {
    pub status: String,
    pub code: i32,
    pub error_message: String,
    pub warnings: String,
    pub results_count: i32,
}
