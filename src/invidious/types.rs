#[derive(Debug, Deserialize)]
pub struct Video {
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    // add fields as needed
}
