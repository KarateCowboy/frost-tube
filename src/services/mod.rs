#[derive(Debug, Clone, Default)]
pub struct Video {
    pub title: String,
    pub id: String,
}

pub trait VideoService: Send + Sync {
    fn search(&self, query: &str) -> impl std::future::Future<Output = Vec<Video>> + Send;
}
