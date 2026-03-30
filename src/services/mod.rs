use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Default)]
pub struct Video {
    pub title: String,
    pub video_id: String,
}

pub trait VideoService: Send + Sync {
    fn search(&self, query: &str) -> Pin<Box<dyn Future<Output = Vec<Video>> + Send + '_>>;
}
