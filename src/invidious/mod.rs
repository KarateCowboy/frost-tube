pub mod types;

use crate::services::{Video, VideoService};

#[derive(Debug, Clone)]
pub struct InvidiousClient {
    base_url: String,
    client: reqwest::Client,
}

impl InvidiousClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::builder()
                .user_agent("FrostTube/0.1")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }
}

impl Default for InvidiousClient {
    fn default() -> Self {
        Self::new("https://yewtu.be")
    }
}

impl VideoService for InvidiousClient {
    async fn search(&self, query: &str) -> Result<Vec<Video>, String> {
        let response = self.client
            .get(format!("{}/api/v1/search", self.base_url))
            .query(&[("q", query)])
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Search failed (HTTP {status}): {body}"));
        }

        let results: Vec<types::SearchResult> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {e}"))?;

        Ok(results.into_iter().filter_map(|r| match r {
            types::SearchResult::Video { title, id } => Some(Video { title, id }),
            _ => None,
        }).collect())
    }
}
