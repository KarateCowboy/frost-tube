pub mod types;

use crate::services::{Video, VideoService};

#[derive(Debug)]
pub struct InvidiousClient {
    base_url: String,
    client: reqwest::Client,
}

impl InvidiousClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }
}

impl VideoService for InvidiousClient {
    async fn search (&self, query: &str) -> Vec<Video> {
        let response: Vec<types::SearchResult> = self.client.get(format!("{}/api/v1/search", self.base_url)).query(&[("q", query)]).send().await.expect("Failed to send request").json().await.expect("Failed to parse response");
        response.into_iter().filter_map(|r| 
            match r {
                types::SearchResult::Video { title, id} => Some(Video {title, id }),
                _ => None,

            }
        ).collect()

    }
}
