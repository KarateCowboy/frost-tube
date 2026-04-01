use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SearchResult {
    #[serde(rename = "video")]
    Video {
        title: String,
        #[serde(rename = "videoId")]
        id: String,
    },
    #[serde(other)]
    Other,
}
